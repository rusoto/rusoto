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
use rusoto_core::region;
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoFuture};

use rusoto_core::credential::{CredentialsError, ProvideAwsCredentials};
use rusoto_core::request::HttpDispatchError;

use rusoto_core::signature::SignedRequest;
use serde_json;
use serde_json::from_slice;
use serde_json::Value as SerdeJsonValue;
/// <p>An address book with attributes.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
pub struct ApproveSkillRequest {
    /// <p>The unique identifier of the skill.</p>
    #[serde(rename = "SkillId")]
    pub skill_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ApproveSkillResponse {}

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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
pub struct AssociateSkillGroupWithRoomResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AssociateSkillWithSkillGroupRequest {
    /// <p>The ARN of the skill group to associate the skill to. Required.</p>
    #[serde(rename = "SkillGroupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skill_group_arn: Option<String>,
    /// <p>The unique identifier of the skill.</p>
    #[serde(rename = "SkillId")]
    pub skill_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AssociateSkillWithSkillGroupResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AssociateSkillWithUsersRequest {
    /// <p>The ARN of the organization.</p>
    #[serde(rename = "OrganizationArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_arn: Option<String>,
    /// <p>The private skill ID you want to make available to enrolled users.&gt;</p>
    #[serde(rename = "SkillId")]
    pub skill_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AssociateSkillWithUsersResponse {}

/// <p>Usage report with specified parameters.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct BusinessReport {
    /// <p>The time of report delivery.</p>
    #[serde(rename = "DeliveryTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_time: Option<f64>,
    /// <p>The download link where a user can download the report.</p>
    #[serde(rename = "DownloadUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_url: Option<String>,
    /// <p>The failure code.</p>
    #[serde(rename = "FailureCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_code: Option<String>,
    /// <p>The S3 location of the output reports.</p>
    #[serde(rename = "S3Location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_location: Option<BusinessReportS3Location>,
    /// <p>The status of the report generation execution (RUNNING, SUCCEEDED, or FAILED).</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>The content range of the report.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BusinessReportContentRange {
    /// <p>The interval of the content range.</p>
    #[serde(rename = "Interval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
}

/// <p>The recurrence of the reports.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BusinessReportRecurrence {
    /// <p>The start date.</p>
    #[serde(rename = "StartDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
}

/// <p>The S3 location of the output reports.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct BusinessReportS3Location {
    /// <p>The S3 bucket name of the output reports.</p>
    #[serde(rename = "BucketName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_name: Option<String>,
    /// <p>The path of the business report.</p>
    #[serde(rename = "Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

/// <p>The schedule of the usage report.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct BusinessReportSchedule {
    /// <p>The content range of the reports.</p>
    #[serde(rename = "ContentRange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_range: Option<BusinessReportContentRange>,
    /// <p>The format of the generated report (individual CSV files or zipped files of individual files).</p>
    #[serde(rename = "Format")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// <p>The details of the last business report delivery for a specified time interval.</p>
    #[serde(rename = "LastBusinessReport")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_business_report: Option<BusinessReport>,
    /// <p>The recurrence of the reports.</p>
    #[serde(rename = "Recurrence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurrence: Option<BusinessReportRecurrence>,
    /// <p>The S3 bucket name of the output reports.</p>
    #[serde(rename = "S3BucketName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket_name: Option<String>,
    /// <p>The S3 key where the report is delivered.</p>
    #[serde(rename = "S3KeyPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_key_prefix: Option<String>,
    /// <p>The ARN of the business report schedule.</p>
    #[serde(rename = "ScheduleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_arn: Option<String>,
    /// <p>The name identifier of the schedule.</p>
    #[serde(rename = "ScheduleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_name: Option<String>,
}

/// <p>The skill store category that is shown. Alexa skills are assigned a specific skill category during creation, such as News, Social, and Sports.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Category {
    /// <p>The ID of the skill store category.</p>
    #[serde(rename = "CategoryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_id: Option<i64>,
    /// <p>The name of the skill store category.</p>
    #[serde(rename = "CategoryName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_name: Option<String>,
}

/// <p>The default conference provider that is used if no other scheduled meetings are detected.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConferencePreference {
    /// <p>The ARN of the default conference provider.</p>
    #[serde(rename = "DefaultConferenceProviderArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_conference_provider_arn: Option<String>,
}

/// <p>An entity that provides a conferencing solution. Alexa for Business acts as the voice interface and mediator that connects users to their preferred conference provider. Examples of conference providers include Amazon Chime, Zoom, Cisco, and Polycom. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ConferenceProvider {
    /// <p>The ARN of the newly created conference provider.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The IP endpoint and protocol for calling.</p>
    #[serde(rename = "IPDialIn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_dial_in: Option<IPDialIn>,
    /// <p>The meeting settings for the conference provider.</p>
    #[serde(rename = "MeetingSetting")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meeting_setting: Option<MeetingSetting>,
    /// <p>The name of the conference provider.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The information for PSTN conferencing.</p>
    #[serde(rename = "PSTNDialIn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pstn_dial_in: Option<PSTNDialIn>,
    /// <p>The type of conference providers.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>A contact with attributes.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Contact {
    /// <p>The ARN of the contact.</p>
    #[serde(rename = "ContactArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_arn: Option<String>,
    /// <p>The name of the contact to display on the console.</p>
    #[serde(rename = "DisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// <p>The first name of the contact, used to call the contact on the device.</p>
    #[serde(rename = "FirstName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// <p>The last name of the contact, used to call the contact on the device.</p>
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
#[cfg_attr(test, derive(Serialize))]
pub struct ContactData {
    /// <p>The ARN of the contact.</p>
    #[serde(rename = "ContactArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_arn: Option<String>,
    /// <p>The name of the contact to display on the console.</p>
    #[serde(rename = "DisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// <p>The first name of the contact, used to call the contact on the device.</p>
    #[serde(rename = "FirstName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// <p>The last name of the contact, used to call the contact on the device.</p>
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
#[cfg_attr(test, derive(Serialize))]
pub struct CreateAddressBookResponse {
    /// <p>The ARN of the newly created address book.</p>
    #[serde(rename = "AddressBookArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_book_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateBusinessReportScheduleRequest {
    /// <p>The client request token.</p>
    #[serde(rename = "ClientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>The content range of the reports.</p>
    #[serde(rename = "ContentRange")]
    pub content_range: BusinessReportContentRange,
    /// <p>The format of the generated report (individual CSV files or zipped files of individual files).</p>
    #[serde(rename = "Format")]
    pub format: String,
    /// <p>The recurrence of the reports.</p>
    #[serde(rename = "Recurrence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurrence: Option<BusinessReportRecurrence>,
    /// <p>The S3 bucket name of the output reports.</p>
    #[serde(rename = "S3BucketName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket_name: Option<String>,
    /// <p>The S3 key where the report is delivered.</p>
    #[serde(rename = "S3KeyPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_key_prefix: Option<String>,
    /// <p>The name identifier of the schedule.</p>
    #[serde(rename = "ScheduleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateBusinessReportScheduleResponse {
    /// <p>The ARN of the business report schedule.</p>
    #[serde(rename = "ScheduleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateConferenceProviderRequest {
    /// <p>The request token of the client.</p>
    #[serde(rename = "ClientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>The name of the conference provider.</p>
    #[serde(rename = "ConferenceProviderName")]
    pub conference_provider_name: String,
    /// <p>Represents a type within a list of predefined types.</p>
    #[serde(rename = "ConferenceProviderType")]
    pub conference_provider_type: String,
    /// <p>The IP endpoint and protocol for calling.</p>
    #[serde(rename = "IPDialIn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_dial_in: Option<IPDialIn>,
    /// <p>The meeting settings for the conference provider.</p>
    #[serde(rename = "MeetingSetting")]
    pub meeting_setting: MeetingSetting,
    /// <p>The information for PSTN conferencing.</p>
    #[serde(rename = "PSTNDialIn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pstn_dial_in: Option<PSTNDialIn>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateConferenceProviderResponse {
    /// <p>The ARN of the newly-created conference provider.</p>
    #[serde(rename = "ConferenceProviderArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conference_provider_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateContactRequest {
    /// <p>A unique, user-specified identifier for this request that ensures idempotency.</p>
    #[serde(rename = "ClientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>The name of the contact to display on the console.</p>
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
    /// <p>The phone number of the contact in E.164 format.</p>
    #[serde(rename = "PhoneNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteAddressBookResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteBusinessReportScheduleRequest {
    /// <p>The ARN of the business report schedule.</p>
    #[serde(rename = "ScheduleArn")]
    pub schedule_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteBusinessReportScheduleResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteConferenceProviderRequest {
    /// <p>The ARN of the conference provider.</p>
    #[serde(rename = "ConferenceProviderArn")]
    pub conference_provider_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteConferenceProviderResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteContactRequest {
    /// <p>The ARN of the contact to delete.</p>
    #[serde(rename = "ContactArn")]
    pub contact_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteContactResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteDeviceRequest {
    /// <p>The ARN of the device for which to request details.</p>
    #[serde(rename = "DeviceArn")]
    pub device_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteDeviceResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteProfileRequest {
    /// <p>The ARN of the room profile to delete. Required.</p>
    #[serde(rename = "ProfileArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteProfileResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteRoomRequest {
    /// <p>The ARN of the room to delete. Required.</p>
    #[serde(rename = "RoomArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteRoomSkillParameterResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteSkillAuthorizationRequest {
    /// <p>The room that the skill is authorized for.</p>
    #[serde(rename = "RoomArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room_arn: Option<String>,
    /// <p>The unique identifier of a skill.</p>
    #[serde(rename = "SkillId")]
    pub skill_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteSkillAuthorizationResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteSkillGroupRequest {
    /// <p>The ARN of the skill group to delete. Required.</p>
    #[serde(rename = "SkillGroupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skill_group_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteUserResponse {}

/// <p>The details about the developer that published the skill.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeveloperInfo {
    /// <p>The name of the developer.</p>
    #[serde(rename = "DeveloperName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub developer_name: Option<String>,
    /// <p>The email of the developer.</p>
    #[serde(rename = "Email")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// <p>The URL of the privacy policy.</p>
    #[serde(rename = "PrivacyPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy_policy: Option<String>,
    /// <p>The website of the developer.</p>
    #[serde(rename = "Url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// <p>A device with attributes.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
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
    /// <p>The status of a device. If the status is not READY, check the DeviceStatusInfo value for details.</p>
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
#[cfg_attr(test, derive(Serialize))]
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

/// <p>The list of device events.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeviceEvent {
    /// <p>The time (in epoch) when the event occurred. </p>
    #[serde(rename = "Timestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<f64>,
    /// <p>The type of device event.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p>The value of the event.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>Details of a device’s status.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeviceStatusDetail {
    /// <p>The device status detail code.</p>
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}

/// <p>Detailed information about a device's status.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeviceStatusInfo {
    /// <p>The latest available information about the connection status of a device. </p>
    #[serde(rename = "ConnectionStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_status: Option<String>,
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
#[cfg_attr(test, derive(Serialize))]
pub struct DisassociateContactFromAddressBookResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DisassociateDeviceFromRoomRequest {
    /// <p>The ARN of the device to disassociate from a room. Required.</p>
    #[serde(rename = "DeviceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DisassociateDeviceFromRoomResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DisassociateSkillFromSkillGroupRequest {
    /// <p>The unique identifier of a skill. Required.</p>
    #[serde(rename = "SkillGroupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skill_group_arn: Option<String>,
    /// <p>The ARN of a skill group to associate to a skill.</p>
    #[serde(rename = "SkillId")]
    pub skill_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DisassociateSkillFromSkillGroupResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DisassociateSkillFromUsersRequest {
    /// <p>The ARN of the organization.</p>
    #[serde(rename = "OrganizationArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_arn: Option<String>,
    /// <p> The private skill ID you want to make unavailable for enrolled users.</p>
    #[serde(rename = "SkillId")]
    pub skill_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DisassociateSkillFromUsersResponse {}

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
#[cfg_attr(test, derive(Serialize))]
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
pub struct ForgetSmartHomeAppliancesRequest {
    /// <p>The room that the appliances are associated with.</p>
    #[serde(rename = "RoomArn")]
    pub room_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ForgetSmartHomeAppliancesResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetAddressBookRequest {
    /// <p>The ARN of the address book for which to request details.</p>
    #[serde(rename = "AddressBookArn")]
    pub address_book_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetAddressBookResponse {
    /// <p>The details of the requested address book.</p>
    #[serde(rename = "AddressBook")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_book: Option<AddressBook>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetConferencePreferenceRequest {}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetConferencePreferenceResponse {
    /// <p>The conference preference.</p>
    #[serde(rename = "Preference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<ConferencePreference>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetConferenceProviderRequest {
    /// <p>The ARN of the newly created conference provider.</p>
    #[serde(rename = "ConferenceProviderArn")]
    pub conference_provider_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetConferenceProviderResponse {
    /// <p>The conference provider.</p>
    #[serde(rename = "ConferenceProvider")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conference_provider: Option<ConferenceProvider>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetContactRequest {
    /// <p>The ARN of the contact for which to request details.</p>
    #[serde(rename = "ContactArn")]
    pub contact_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
pub struct GetSkillGroupResponse {
    /// <p>The details of the skill group requested. Required.</p>
    #[serde(rename = "SkillGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skill_group: Option<SkillGroup>,
}

/// <p>The IP endpoint and protocol for calling.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IPDialIn {
    /// <p>The protocol, including SIP, SIPS, and H323.</p>
    #[serde(rename = "CommsProtocol")]
    pub comms_protocol: String,
    /// <p>The IP address.</p>
    #[serde(rename = "Endpoint")]
    pub endpoint: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListBusinessReportSchedulesRequest {
    /// <p>The maximum number of schedules listed in the call.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token used to list the remaining schedules from the previous API call.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListBusinessReportSchedulesResponse {
    /// <p>The schedule of the reports.</p>
    #[serde(rename = "BusinessReportSchedules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_report_schedules: Option<Vec<BusinessReportSchedule>>,
    /// <p>The token used to list the remaining schedules from the previous API call.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListConferenceProvidersRequest {
    /// <p>The maximum number of conference providers to be returned, per paginated calls.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The tokens used for pagination.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListConferenceProvidersResponse {
    /// <p>The conference providers.</p>
    #[serde(rename = "ConferenceProviders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conference_providers: Option<Vec<ConferenceProvider>>,
    /// <p>The tokens used for pagination.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListDeviceEventsRequest {
    /// <p>The ARN of a device.</p>
    #[serde(rename = "DeviceArn")]
    pub device_arn: String,
    /// <p>The event type to filter device events. If EventType isn't specified, this returns a list of all device events in reverse chronological order. If EventType is specified, this returns a list of device events for that EventType in reverse chronological order. </p>
    #[serde(rename = "EventType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
    /// <p>The maximum number of results to include in the response. The default value is 50. If more results exist than the specified MaxResults value, a token is included in the response so that the remaining results can be retrieved. </p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>An optional token returned from a prior request. Use this token for pagination of results from this action. If this parameter is specified, the response only includes results beyond the token, up to the value specified by MaxResults. When the end of results is reached, the response has a value of null.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListDeviceEventsResponse {
    /// <p>The device events requested for the device ARN.</p>
    #[serde(rename = "DeviceEvents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_events: Option<Vec<DeviceEvent>>,
    /// <p>The token returned to indicate that there is more data available.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListSkillsRequest {
    /// <p>Whether the skill is enabled under the user's account, or if it requires linking to be used.</p>
    #[serde(rename = "EnablementType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enablement_type: Option<String>,
    /// <p>The maximum number of results to include in the response. If more results exist than the specified <code>MaxResults</code> value, a token is included in the response so that the remaining results can be retrieved.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>An optional token returned from a prior request. Use this token for pagination of results from this action. If this parameter is specified, the response includes only results beyond the token, up to the value specified by <code>MaxResults</code>.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The ARN of the skill group for which to list enabled skills.</p>
    #[serde(rename = "SkillGroupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skill_group_arn: Option<String>,
    /// <p>Whether the skill is publicly available or is a private skill.</p>
    #[serde(rename = "SkillType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skill_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
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
pub struct ListSkillsStoreCategoriesRequest {
    /// <p>The maximum number of categories returned, per paginated calls.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The tokens used for pagination.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListSkillsStoreCategoriesResponse {
    /// <p>The list of categories.</p>
    #[serde(rename = "CategoryList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_list: Option<Vec<Category>>,
    /// <p>The tokens used for pagination.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListSkillsStoreSkillsByCategoryRequest {
    /// <p>The category ID for which the skills are being retrieved from the skill store.</p>
    #[serde(rename = "CategoryId")]
    pub category_id: i64,
    /// <p>The maximum number of skills returned per paginated calls.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The tokens used for pagination.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListSkillsStoreSkillsByCategoryResponse {
    /// <p>The tokens used for pagination.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The skill store skills.</p>
    #[serde(rename = "SkillsStoreSkills")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skills_store_skills: Option<Vec<SkillsStoreSkill>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListSmartHomeAppliancesRequest {
    /// <p>The maximum number of appliances to be returned, per paginated calls.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The tokens used for pagination.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The room that the appliances are associated with.</p>
    #[serde(rename = "RoomArn")]
    pub room_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListSmartHomeAppliancesResponse {
    /// <p>The tokens used for pagination.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The smart home appliances.</p>
    #[serde(rename = "SmartHomeAppliances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_home_appliances: Option<Vec<SmartHomeAppliance>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListTagsRequest {
    /// <p>The ARN of the specified resource for which to list tags.</p>
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
#[cfg_attr(test, derive(Serialize))]
pub struct ListTagsResponse {
    /// <p>The token returned to indicate that there is more data available.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The tags requested for the specified resource.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// <p><p>The values that indicate whether a pin is always required (YES), never required (NO), or OPTIONAL.</p> <ul> <li> <p>If YES, Alexa will always ask for a meeting pin.</p> </li> <li> <p>If NO, Alexa will never ask for a meeting pin.</p> </li> <li> <p>If OPTIONAL, Alexa will ask if you have a meeting pin and if the customer responds with yes, it will ask for the meeting pin.</p> </li> </ul></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MeetingSetting {
    /// <p>The values that indicate whether the pin is always required.</p>
    #[serde(rename = "RequirePin")]
    pub require_pin: String,
}

/// <p>The information for public switched telephone network (PSTN) conferencing.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PSTNDialIn {
    /// <p>The zip code.</p>
    #[serde(rename = "CountryCode")]
    pub country_code: String,
    /// <p>The delay duration before Alexa enters the conference ID with dual-tone multi-frequency (DTMF). Each number on the dial pad corresponds to a DTMF tone, which is how we send data over the telephone network.</p>
    #[serde(rename = "OneClickIdDelay")]
    pub one_click_id_delay: String,
    /// <p>The delay duration before Alexa enters the conference pin with dual-tone multi-frequency (DTMF). Each number on the dial pad corresponds to a DTMF tone, which is how we send data over the telephone network.</p>
    #[serde(rename = "OneClickPinDelay")]
    pub one_click_pin_delay: String,
    /// <p>The phone number to call to join the conference.</p>
    #[serde(rename = "PhoneNumber")]
    pub phone_number: String,
}

/// <p>A room profile with attributes.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Profile {
    /// <p>The address of a room profile.</p>
    #[serde(rename = "Address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// <p>The ARN of the address book.</p>
    #[serde(rename = "AddressBookArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_book_arn: Option<String>,
    /// <p>The distance unit of a room profile.</p>
    #[serde(rename = "DistanceUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distance_unit: Option<String>,
    /// <p>Retrieves if the profile is default or not.</p>
    #[serde(rename = "IsDefault")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
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
#[cfg_attr(test, derive(Serialize))]
pub struct ProfileData {
    /// <p>The address of a room profile.</p>
    #[serde(rename = "Address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// <p>The distance unit of a room profile.</p>
    #[serde(rename = "DistanceUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distance_unit: Option<String>,
    /// <p>Retrieves if the profile data is default or not.</p>
    #[serde(rename = "IsDefault")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
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
pub struct PutConferencePreferenceRequest {
    /// <p>The conference preference of a specific conference provider.</p>
    #[serde(rename = "ConferencePreference")]
    pub conference_preference: ConferencePreference,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct PutConferencePreferenceResponse {}

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
#[cfg_attr(test, derive(Serialize))]
pub struct PutRoomSkillParameterResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutSkillAuthorizationRequest {
    /// <p>The authorization result specific to OAUTH code grant output. "Code” must be populated in the AuthorizationResult map to establish the authorization.</p>
    #[serde(rename = "AuthorizationResult")]
    pub authorization_result: ::std::collections::HashMap<String, String>,
    /// <p>The room that the skill is authorized for.</p>
    #[serde(rename = "RoomArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room_arn: Option<String>,
    /// <p>The unique identifier of a skill.</p>
    #[serde(rename = "SkillId")]
    pub skill_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct PutSkillAuthorizationResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RegisterAVSDeviceRequest {
    /// <p>The device type ID for your AVS device generated by Amazon when the OEM creates a new product on Amazon's Developer Console.</p>
    #[serde(rename = "AmazonId")]
    pub amazon_id: String,
    /// <p>The client ID of the OEM used for code-based linking authorization on an AVS device.</p>
    #[serde(rename = "ClientId")]
    pub client_id: String,
    /// <p>The key generated by the OEM that uniquely identifies a specified instance of your AVS device.</p>
    #[serde(rename = "DeviceSerialNumber")]
    pub device_serial_number: String,
    /// <p>The product ID used to identify your AVS device during authorization.</p>
    #[serde(rename = "ProductId")]
    pub product_id: String,
    /// <p>The code that is obtained after your AVS device has made a POST request to LWA as a part of the Device Authorization Request component of the OAuth code-based linking specification.</p>
    #[serde(rename = "UserCode")]
    pub user_code: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct RegisterAVSDeviceResponse {
    /// <p>The ARN of the device.</p>
    #[serde(rename = "DeviceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RejectSkillRequest {
    /// <p>The unique identifier of the skill.</p>
    #[serde(rename = "SkillId")]
    pub skill_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct RejectSkillResponse {}

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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
pub struct RevokeInvitationResponse {}

/// <p>A room with attributes.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
    /// <p>The filters to use to list a specified set of devices. Supported filter keys are DeviceName, DeviceStatus, DeviceStatusDetailCode, RoomName, DeviceType, DeviceSerialNumber, UnassociatedOnly, and ConnectionStatus (ONLINE and OFFLINE).</p>
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
    /// <p>The sort order to use in listing the specified set of devices. Supported sort keys are DeviceName, DeviceStatus, RoomName, DeviceType, DeviceSerialNumber, and ConnectionStatus.</p>
    #[serde(rename = "SortCriteria")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_criteria: Option<Vec<Sort>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
pub struct SendInvitationResponse {}

/// <p>Granular information about the skill.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct SkillDetails {
    /// <p>The details about what the skill supports organized as bullet points.</p>
    #[serde(rename = "BulletPoints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bullet_points: Option<Vec<String>>,
    /// <p>The details about the developer that published the skill.</p>
    #[serde(rename = "DeveloperInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub developer_info: Option<DeveloperInfo>,
    /// <p>The URL of the end user license agreement.</p>
    #[serde(rename = "EndUserLicenseAgreement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_user_license_agreement: Option<String>,
    /// <p>The generic keywords associated with the skill that can be used to find a skill.</p>
    #[serde(rename = "GenericKeywords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generic_keywords: Option<Vec<String>>,
    /// <p>The phrase used to trigger the skill.</p>
    #[serde(rename = "InvocationPhrase")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invocation_phrase: Option<String>,
    /// <p>The updates added in bullet points.</p>
    #[serde(rename = "NewInThisVersionBulletPoints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_in_this_version_bullet_points: Option<Vec<String>>,
    /// <p>The description of the product.</p>
    #[serde(rename = "ProductDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_description: Option<String>,
    /// <p>The date when the skill was released.</p>
    #[serde(rename = "ReleaseDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_date: Option<String>,
    /// <p>The list of reviews for the skill, including Key and Value pair.</p>
    #[serde(rename = "Reviews")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reviews: Option<::std::collections::HashMap<String, String>>,
    /// <p>The types of skills.</p>
    #[serde(rename = "SkillTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skill_types: Option<Vec<String>>,
}

/// <p>A skill group with attributes.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
pub struct SkillSummary {
    /// <p>Whether the skill is enabled under the user's account, or if it requires linking to be used.</p>
    #[serde(rename = "EnablementType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enablement_type: Option<String>,
    /// <p>The ARN of the skill summary.</p>
    #[serde(rename = "SkillId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skill_id: Option<String>,
    /// <p>The name of the skill.</p>
    #[serde(rename = "SkillName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skill_name: Option<String>,
    /// <p>Whether the skill is publicly available or is a private skill.</p>
    #[serde(rename = "SkillType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skill_type: Option<String>,
    /// <p>Linking support for a skill.</p>
    #[serde(rename = "SupportsLinking")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_linking: Option<bool>,
}

/// <p>The detailed information about an Alexa skill.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct SkillsStoreSkill {
    /// <p>The URL where the skill icon resides.</p>
    #[serde(rename = "IconUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    /// <p>Sample utterances that interact with the skill.</p>
    #[serde(rename = "SampleUtterances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_utterances: Option<Vec<String>>,
    /// <p>Short description about the skill.</p>
    #[serde(rename = "ShortDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_description: Option<String>,
    /// <p>Information about the skill.</p>
    #[serde(rename = "SkillDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skill_details: Option<SkillDetails>,
    /// <p>The ARN of the skill.</p>
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

/// <p>A smart home appliance that can connect to a central system. Any domestic device can be a smart appliance. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct SmartHomeAppliance {
    /// <p>The description of the smart home appliance.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The friendly name of the smart home appliance.</p>
    #[serde(rename = "FriendlyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    /// <p>The name of the manufacturer of the smart home appliance.</p>
    #[serde(rename = "ManufacturerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manufacturer_name: Option<String>,
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
#[cfg_attr(test, derive(Serialize))]
pub struct StartDeviceSyncResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StartSmartHomeApplianceDiscoveryRequest {
    /// <p>The room where smart home appliance discovery was initiated.</p>
    #[serde(rename = "RoomArn")]
    pub room_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct StartSmartHomeApplianceDiscoveryResponse {}

/// <p>A key-value pair that can be associated with a resource. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    /// <p>The key of a tag. Tag keys are case-sensitive. </p>
    #[serde(rename = "Key")]
    pub key: String,
    /// <p>The value of a tag. Tag values are case-sensitive and can be null.</p>
    #[serde(rename = "Value")]
    pub value: String,
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateAddressBookResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateBusinessReportScheduleRequest {
    /// <p>The format of the generated report (individual CSV files or zipped files of individual files).</p>
    #[serde(rename = "Format")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// <p>The recurrence of the reports.</p>
    #[serde(rename = "Recurrence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurrence: Option<BusinessReportRecurrence>,
    /// <p>The S3 location of the output reports.</p>
    #[serde(rename = "S3BucketName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket_name: Option<String>,
    /// <p>The S3 key where the report is delivered.</p>
    #[serde(rename = "S3KeyPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_key_prefix: Option<String>,
    /// <p>The ARN of the business report schedule.</p>
    #[serde(rename = "ScheduleArn")]
    pub schedule_arn: String,
    /// <p>The name identifier of the schedule.</p>
    #[serde(rename = "ScheduleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateBusinessReportScheduleResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateConferenceProviderRequest {
    /// <p>The ARN of the conference provider.</p>
    #[serde(rename = "ConferenceProviderArn")]
    pub conference_provider_arn: String,
    /// <p>The type of the conference provider.</p>
    #[serde(rename = "ConferenceProviderType")]
    pub conference_provider_type: String,
    /// <p>The IP endpoint and protocol for calling.</p>
    #[serde(rename = "IPDialIn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_dial_in: Option<IPDialIn>,
    /// <p>The meeting settings for the conference provider.</p>
    #[serde(rename = "MeetingSetting")]
    pub meeting_setting: MeetingSetting,
    /// <p>The information for PSTN conferencing.</p>
    #[serde(rename = "PSTNDialIn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pstn_dial_in: Option<PSTNDialIn>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateConferenceProviderResponse {}

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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
    /// <p>Sets the profile as default if selected. If this is missing, no update is done to the default status.</p>
    #[serde(rename = "IsDefault")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateSkillGroupResponse {}

/// <p>Information related to a user.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
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

/// Errors returned by ApproveSkill
#[derive(Debug, PartialEq)]
pub enum ApproveSkillError {
    /// <p>Concurrent modification of resources. HTTP Status Code: 400.</p>
    ConcurrentModification(String),
    /// <p>You are performing an action that would put you beyond your account's limits.</p>
    LimitExceeded(String),
    /// <p>The resource is not found.</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl ApproveSkillError {
    pub fn from_response(res: BufferedHttpResponse) -> ApproveSkillError {
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
                    return ApproveSkillError::ConcurrentModification(String::from(error_message));
                }
                "LimitExceededException" => {
                    return ApproveSkillError::LimitExceeded(String::from(error_message));
                }
                "NotFoundException" => {
                    return ApproveSkillError::NotFound(String::from(error_message));
                }
                "ValidationException" => {
                    return ApproveSkillError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return ApproveSkillError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ApproveSkillError {
    fn from(err: serde_json::error::Error) -> ApproveSkillError {
        ApproveSkillError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ApproveSkillError {
    fn from(err: CredentialsError) -> ApproveSkillError {
        ApproveSkillError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ApproveSkillError {
    fn from(err: HttpDispatchError) -> ApproveSkillError {
        ApproveSkillError::HttpDispatch(err)
    }
}
impl From<io::Error> for ApproveSkillError {
    fn from(err: io::Error) -> ApproveSkillError {
        ApproveSkillError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ApproveSkillError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ApproveSkillError {
    fn description(&self) -> &str {
        match *self {
            ApproveSkillError::ConcurrentModification(ref cause) => cause,
            ApproveSkillError::LimitExceeded(ref cause) => cause,
            ApproveSkillError::NotFound(ref cause) => cause,
            ApproveSkillError::Validation(ref cause) => cause,
            ApproveSkillError::Credentials(ref err) => err.description(),
            ApproveSkillError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ApproveSkillError::ParseError(ref cause) => cause,
            ApproveSkillError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by AssociateContactWithAddressBook
#[derive(Debug, PartialEq)]
pub enum AssociateContactWithAddressBookError {
    /// <p>You are performing an action that would put you beyond your account's limits.</p>
    LimitExceeded(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl AssociateContactWithAddressBookError {
    pub fn from_response(res: BufferedHttpResponse) -> AssociateContactWithAddressBookError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "LimitExceededException" => {
                    return AssociateContactWithAddressBookError::LimitExceeded(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return AssociateContactWithAddressBookError::Validation(
                        error_message.to_string(),
                    );
                }
                _ => {}
            }
        }
        return AssociateContactWithAddressBookError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for AssociateContactWithAddressBookError {
    fn from(err: serde_json::error::Error) -> AssociateContactWithAddressBookError {
        AssociateContactWithAddressBookError::ParseError(err.description().to_string())
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
            AssociateContactWithAddressBookError::LimitExceeded(ref cause) => cause,
            AssociateContactWithAddressBookError::Validation(ref cause) => cause,
            AssociateContactWithAddressBookError::Credentials(ref err) => err.description(),
            AssociateContactWithAddressBookError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AssociateContactWithAddressBookError::ParseError(ref cause) => cause,
            AssociateContactWithAddressBookError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by AssociateDeviceWithRoom
#[derive(Debug, PartialEq)]
pub enum AssociateDeviceWithRoomError {
    /// <p>Concurrent modification of resources. HTTP Status Code: 400.</p>
    ConcurrentModification(String),
    /// <p>The request failed because this device is no longer registered and therefore no longer managed by this account.</p>
    DeviceNotRegistered(String),
    /// <p>You are performing an action that would put you beyond your account's limits.</p>
    LimitExceeded(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl AssociateDeviceWithRoomError {
    pub fn from_response(res: BufferedHttpResponse) -> AssociateDeviceWithRoomError {
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
                    return AssociateDeviceWithRoomError::ConcurrentModification(String::from(
                        error_message,
                    ));
                }
                "DeviceNotRegisteredException" => {
                    return AssociateDeviceWithRoomError::DeviceNotRegistered(String::from(
                        error_message,
                    ));
                }
                "LimitExceededException" => {
                    return AssociateDeviceWithRoomError::LimitExceeded(String::from(error_message));
                }
                "ValidationException" => {
                    return AssociateDeviceWithRoomError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return AssociateDeviceWithRoomError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for AssociateDeviceWithRoomError {
    fn from(err: serde_json::error::Error) -> AssociateDeviceWithRoomError {
        AssociateDeviceWithRoomError::ParseError(err.description().to_string())
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
            AssociateDeviceWithRoomError::ConcurrentModification(ref cause) => cause,
            AssociateDeviceWithRoomError::DeviceNotRegistered(ref cause) => cause,
            AssociateDeviceWithRoomError::LimitExceeded(ref cause) => cause,
            AssociateDeviceWithRoomError::Validation(ref cause) => cause,
            AssociateDeviceWithRoomError::Credentials(ref err) => err.description(),
            AssociateDeviceWithRoomError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AssociateDeviceWithRoomError::ParseError(ref cause) => cause,
            AssociateDeviceWithRoomError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by AssociateSkillGroupWithRoom
#[derive(Debug, PartialEq)]
pub enum AssociateSkillGroupWithRoomError {
    /// <p>Concurrent modification of resources. HTTP Status Code: 400.</p>
    ConcurrentModification(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl AssociateSkillGroupWithRoomError {
    pub fn from_response(res: BufferedHttpResponse) -> AssociateSkillGroupWithRoomError {
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
                    return AssociateSkillGroupWithRoomError::ConcurrentModification(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return AssociateSkillGroupWithRoomError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return AssociateSkillGroupWithRoomError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for AssociateSkillGroupWithRoomError {
    fn from(err: serde_json::error::Error) -> AssociateSkillGroupWithRoomError {
        AssociateSkillGroupWithRoomError::ParseError(err.description().to_string())
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
            AssociateSkillGroupWithRoomError::ConcurrentModification(ref cause) => cause,
            AssociateSkillGroupWithRoomError::Validation(ref cause) => cause,
            AssociateSkillGroupWithRoomError::Credentials(ref err) => err.description(),
            AssociateSkillGroupWithRoomError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AssociateSkillGroupWithRoomError::ParseError(ref cause) => cause,
            AssociateSkillGroupWithRoomError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by AssociateSkillWithSkillGroup
#[derive(Debug, PartialEq)]
pub enum AssociateSkillWithSkillGroupError {
    /// <p>Concurrent modification of resources. HTTP Status Code: 400.</p>
    ConcurrentModification(String),
    /// <p>The resource is not found.</p>
    NotFound(String),
    /// <p>The skill must be linked to a third-party account.</p>
    SkillNotLinked(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl AssociateSkillWithSkillGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> AssociateSkillWithSkillGroupError {
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
                    return AssociateSkillWithSkillGroupError::ConcurrentModification(String::from(
                        error_message,
                    ));
                }
                "NotFoundException" => {
                    return AssociateSkillWithSkillGroupError::NotFound(String::from(error_message));
                }
                "SkillNotLinkedException" => {
                    return AssociateSkillWithSkillGroupError::SkillNotLinked(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return AssociateSkillWithSkillGroupError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return AssociateSkillWithSkillGroupError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for AssociateSkillWithSkillGroupError {
    fn from(err: serde_json::error::Error) -> AssociateSkillWithSkillGroupError {
        AssociateSkillWithSkillGroupError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for AssociateSkillWithSkillGroupError {
    fn from(err: CredentialsError) -> AssociateSkillWithSkillGroupError {
        AssociateSkillWithSkillGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AssociateSkillWithSkillGroupError {
    fn from(err: HttpDispatchError) -> AssociateSkillWithSkillGroupError {
        AssociateSkillWithSkillGroupError::HttpDispatch(err)
    }
}
impl From<io::Error> for AssociateSkillWithSkillGroupError {
    fn from(err: io::Error) -> AssociateSkillWithSkillGroupError {
        AssociateSkillWithSkillGroupError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AssociateSkillWithSkillGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AssociateSkillWithSkillGroupError {
    fn description(&self) -> &str {
        match *self {
            AssociateSkillWithSkillGroupError::ConcurrentModification(ref cause) => cause,
            AssociateSkillWithSkillGroupError::NotFound(ref cause) => cause,
            AssociateSkillWithSkillGroupError::SkillNotLinked(ref cause) => cause,
            AssociateSkillWithSkillGroupError::Validation(ref cause) => cause,
            AssociateSkillWithSkillGroupError::Credentials(ref err) => err.description(),
            AssociateSkillWithSkillGroupError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AssociateSkillWithSkillGroupError::ParseError(ref cause) => cause,
            AssociateSkillWithSkillGroupError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by AssociateSkillWithUsers
#[derive(Debug, PartialEq)]
pub enum AssociateSkillWithUsersError {
    /// <p>Concurrent modification of resources. HTTP Status Code: 400.</p>
    ConcurrentModification(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl AssociateSkillWithUsersError {
    pub fn from_response(res: BufferedHttpResponse) -> AssociateSkillWithUsersError {
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
                    return AssociateSkillWithUsersError::ConcurrentModification(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return AssociateSkillWithUsersError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return AssociateSkillWithUsersError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for AssociateSkillWithUsersError {
    fn from(err: serde_json::error::Error) -> AssociateSkillWithUsersError {
        AssociateSkillWithUsersError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for AssociateSkillWithUsersError {
    fn from(err: CredentialsError) -> AssociateSkillWithUsersError {
        AssociateSkillWithUsersError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AssociateSkillWithUsersError {
    fn from(err: HttpDispatchError) -> AssociateSkillWithUsersError {
        AssociateSkillWithUsersError::HttpDispatch(err)
    }
}
impl From<io::Error> for AssociateSkillWithUsersError {
    fn from(err: io::Error) -> AssociateSkillWithUsersError {
        AssociateSkillWithUsersError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AssociateSkillWithUsersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AssociateSkillWithUsersError {
    fn description(&self) -> &str {
        match *self {
            AssociateSkillWithUsersError::ConcurrentModification(ref cause) => cause,
            AssociateSkillWithUsersError::Validation(ref cause) => cause,
            AssociateSkillWithUsersError::Credentials(ref err) => err.description(),
            AssociateSkillWithUsersError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AssociateSkillWithUsersError::ParseError(ref cause) => cause,
            AssociateSkillWithUsersError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by CreateAddressBook
#[derive(Debug, PartialEq)]
pub enum CreateAddressBookError {
    /// <p>The resource being created already exists.</p>
    AlreadyExists(String),
    /// <p>You are performing an action that would put you beyond your account's limits.</p>
    LimitExceeded(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl CreateAddressBookError {
    pub fn from_response(res: BufferedHttpResponse) -> CreateAddressBookError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AlreadyExistsException" => {
                    return CreateAddressBookError::AlreadyExists(String::from(error_message));
                }
                "LimitExceededException" => {
                    return CreateAddressBookError::LimitExceeded(String::from(error_message));
                }
                "ValidationException" => {
                    return CreateAddressBookError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return CreateAddressBookError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreateAddressBookError {
    fn from(err: serde_json::error::Error) -> CreateAddressBookError {
        CreateAddressBookError::ParseError(err.description().to_string())
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
            CreateAddressBookError::ParseError(ref cause) => cause,
            CreateAddressBookError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by CreateBusinessReportSchedule
#[derive(Debug, PartialEq)]
pub enum CreateBusinessReportScheduleError {
    /// <p>The resource being created already exists.</p>
    AlreadyExists(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl CreateBusinessReportScheduleError {
    pub fn from_response(res: BufferedHttpResponse) -> CreateBusinessReportScheduleError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AlreadyExistsException" => {
                    return CreateBusinessReportScheduleError::AlreadyExists(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return CreateBusinessReportScheduleError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return CreateBusinessReportScheduleError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreateBusinessReportScheduleError {
    fn from(err: serde_json::error::Error) -> CreateBusinessReportScheduleError {
        CreateBusinessReportScheduleError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateBusinessReportScheduleError {
    fn from(err: CredentialsError) -> CreateBusinessReportScheduleError {
        CreateBusinessReportScheduleError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateBusinessReportScheduleError {
    fn from(err: HttpDispatchError) -> CreateBusinessReportScheduleError {
        CreateBusinessReportScheduleError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateBusinessReportScheduleError {
    fn from(err: io::Error) -> CreateBusinessReportScheduleError {
        CreateBusinessReportScheduleError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateBusinessReportScheduleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateBusinessReportScheduleError {
    fn description(&self) -> &str {
        match *self {
            CreateBusinessReportScheduleError::AlreadyExists(ref cause) => cause,
            CreateBusinessReportScheduleError::Validation(ref cause) => cause,
            CreateBusinessReportScheduleError::Credentials(ref err) => err.description(),
            CreateBusinessReportScheduleError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateBusinessReportScheduleError::ParseError(ref cause) => cause,
            CreateBusinessReportScheduleError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by CreateConferenceProvider
#[derive(Debug, PartialEq)]
pub enum CreateConferenceProviderError {
    /// <p>The resource being created already exists.</p>
    AlreadyExists(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl CreateConferenceProviderError {
    pub fn from_response(res: BufferedHttpResponse) -> CreateConferenceProviderError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AlreadyExistsException" => {
                    return CreateConferenceProviderError::AlreadyExists(String::from(error_message));
                }
                "ValidationException" => {
                    return CreateConferenceProviderError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return CreateConferenceProviderError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreateConferenceProviderError {
    fn from(err: serde_json::error::Error) -> CreateConferenceProviderError {
        CreateConferenceProviderError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateConferenceProviderError {
    fn from(err: CredentialsError) -> CreateConferenceProviderError {
        CreateConferenceProviderError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateConferenceProviderError {
    fn from(err: HttpDispatchError) -> CreateConferenceProviderError {
        CreateConferenceProviderError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateConferenceProviderError {
    fn from(err: io::Error) -> CreateConferenceProviderError {
        CreateConferenceProviderError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateConferenceProviderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateConferenceProviderError {
    fn description(&self) -> &str {
        match *self {
            CreateConferenceProviderError::AlreadyExists(ref cause) => cause,
            CreateConferenceProviderError::Validation(ref cause) => cause,
            CreateConferenceProviderError::Credentials(ref err) => err.description(),
            CreateConferenceProviderError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateConferenceProviderError::ParseError(ref cause) => cause,
            CreateConferenceProviderError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by CreateContact
#[derive(Debug, PartialEq)]
pub enum CreateContactError {
    /// <p>The resource being created already exists.</p>
    AlreadyExists(String),
    /// <p>You are performing an action that would put you beyond your account's limits.</p>
    LimitExceeded(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl CreateContactError {
    pub fn from_response(res: BufferedHttpResponse) -> CreateContactError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AlreadyExistsException" => {
                    return CreateContactError::AlreadyExists(String::from(error_message));
                }
                "LimitExceededException" => {
                    return CreateContactError::LimitExceeded(String::from(error_message));
                }
                "ValidationException" => {
                    return CreateContactError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return CreateContactError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreateContactError {
    fn from(err: serde_json::error::Error) -> CreateContactError {
        CreateContactError::ParseError(err.description().to_string())
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
            CreateContactError::ParseError(ref cause) => cause,
            CreateContactError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by CreateProfile
#[derive(Debug, PartialEq)]
pub enum CreateProfileError {
    /// <p>The resource being created already exists.</p>
    AlreadyExists(String),
    /// <p>Concurrent modification of resources. HTTP Status Code: 400.</p>
    ConcurrentModification(String),
    /// <p>You are performing an action that would put you beyond your account's limits.</p>
    LimitExceeded(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl CreateProfileError {
    pub fn from_response(res: BufferedHttpResponse) -> CreateProfileError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AlreadyExistsException" => {
                    return CreateProfileError::AlreadyExists(String::from(error_message));
                }
                "ConcurrentModificationException" => {
                    return CreateProfileError::ConcurrentModification(String::from(error_message));
                }
                "LimitExceededException" => {
                    return CreateProfileError::LimitExceeded(String::from(error_message));
                }
                "ValidationException" => {
                    return CreateProfileError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return CreateProfileError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreateProfileError {
    fn from(err: serde_json::error::Error) -> CreateProfileError {
        CreateProfileError::ParseError(err.description().to_string())
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
            CreateProfileError::ConcurrentModification(ref cause) => cause,
            CreateProfileError::LimitExceeded(ref cause) => cause,
            CreateProfileError::Validation(ref cause) => cause,
            CreateProfileError::Credentials(ref err) => err.description(),
            CreateProfileError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateProfileError::ParseError(ref cause) => cause,
            CreateProfileError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by CreateRoom
#[derive(Debug, PartialEq)]
pub enum CreateRoomError {
    /// <p>The resource being created already exists.</p>
    AlreadyExists(String),
    /// <p>You are performing an action that would put you beyond your account's limits.</p>
    LimitExceeded(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl CreateRoomError {
    pub fn from_response(res: BufferedHttpResponse) -> CreateRoomError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AlreadyExistsException" => {
                    return CreateRoomError::AlreadyExists(String::from(error_message));
                }
                "LimitExceededException" => {
                    return CreateRoomError::LimitExceeded(String::from(error_message));
                }
                "ValidationException" => {
                    return CreateRoomError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return CreateRoomError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreateRoomError {
    fn from(err: serde_json::error::Error) -> CreateRoomError {
        CreateRoomError::ParseError(err.description().to_string())
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
            CreateRoomError::ParseError(ref cause) => cause,
            CreateRoomError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by CreateSkillGroup
#[derive(Debug, PartialEq)]
pub enum CreateSkillGroupError {
    /// <p>The resource being created already exists.</p>
    AlreadyExists(String),
    /// <p>Concurrent modification of resources. HTTP Status Code: 400.</p>
    ConcurrentModification(String),
    /// <p>You are performing an action that would put you beyond your account's limits.</p>
    LimitExceeded(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl CreateSkillGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> CreateSkillGroupError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AlreadyExistsException" => {
                    return CreateSkillGroupError::AlreadyExists(String::from(error_message));
                }
                "ConcurrentModificationException" => {
                    return CreateSkillGroupError::ConcurrentModification(String::from(
                        error_message,
                    ));
                }
                "LimitExceededException" => {
                    return CreateSkillGroupError::LimitExceeded(String::from(error_message));
                }
                "ValidationException" => {
                    return CreateSkillGroupError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return CreateSkillGroupError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreateSkillGroupError {
    fn from(err: serde_json::error::Error) -> CreateSkillGroupError {
        CreateSkillGroupError::ParseError(err.description().to_string())
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
            CreateSkillGroupError::ConcurrentModification(ref cause) => cause,
            CreateSkillGroupError::LimitExceeded(ref cause) => cause,
            CreateSkillGroupError::Validation(ref cause) => cause,
            CreateSkillGroupError::Credentials(ref err) => err.description(),
            CreateSkillGroupError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateSkillGroupError::ParseError(ref cause) => cause,
            CreateSkillGroupError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by CreateUser
#[derive(Debug, PartialEq)]
pub enum CreateUserError {
    /// <p>Concurrent modification of resources. HTTP Status Code: 400.</p>
    ConcurrentModification(String),
    /// <p>You are performing an action that would put you beyond your account's limits.</p>
    LimitExceeded(String),
    /// <p>The resource in the request is already in use.</p>
    ResourceInUse(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl CreateUserError {
    pub fn from_response(res: BufferedHttpResponse) -> CreateUserError {
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
                    return CreateUserError::ConcurrentModification(String::from(error_message));
                }
                "LimitExceededException" => {
                    return CreateUserError::LimitExceeded(String::from(error_message));
                }
                "ResourceInUseException" => {
                    return CreateUserError::ResourceInUse(String::from(error_message));
                }
                "ValidationException" => {
                    return CreateUserError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return CreateUserError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreateUserError {
    fn from(err: serde_json::error::Error) -> CreateUserError {
        CreateUserError::ParseError(err.description().to_string())
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
            CreateUserError::ConcurrentModification(ref cause) => cause,
            CreateUserError::LimitExceeded(ref cause) => cause,
            CreateUserError::ResourceInUse(ref cause) => cause,
            CreateUserError::Validation(ref cause) => cause,
            CreateUserError::Credentials(ref err) => err.description(),
            CreateUserError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateUserError::ParseError(ref cause) => cause,
            CreateUserError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteAddressBook
#[derive(Debug, PartialEq)]
pub enum DeleteAddressBookError {
    /// <p>Concurrent modification of resources. HTTP Status Code: 400.</p>
    ConcurrentModification(String),
    /// <p>The resource is not found.</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DeleteAddressBookError {
    pub fn from_response(res: BufferedHttpResponse) -> DeleteAddressBookError {
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
                    return DeleteAddressBookError::ConcurrentModification(String::from(
                        error_message,
                    ));
                }
                "NotFoundException" => {
                    return DeleteAddressBookError::NotFound(String::from(error_message));
                }
                "ValidationException" => {
                    return DeleteAddressBookError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return DeleteAddressBookError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteAddressBookError {
    fn from(err: serde_json::error::Error) -> DeleteAddressBookError {
        DeleteAddressBookError::ParseError(err.description().to_string())
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
            DeleteAddressBookError::ConcurrentModification(ref cause) => cause,
            DeleteAddressBookError::NotFound(ref cause) => cause,
            DeleteAddressBookError::Validation(ref cause) => cause,
            DeleteAddressBookError::Credentials(ref err) => err.description(),
            DeleteAddressBookError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteAddressBookError::ParseError(ref cause) => cause,
            DeleteAddressBookError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteBusinessReportSchedule
#[derive(Debug, PartialEq)]
pub enum DeleteBusinessReportScheduleError {
    /// <p>Concurrent modification of resources. HTTP Status Code: 400.</p>
    ConcurrentModification(String),
    /// <p>The resource is not found.</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DeleteBusinessReportScheduleError {
    pub fn from_response(res: BufferedHttpResponse) -> DeleteBusinessReportScheduleError {
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
                    return DeleteBusinessReportScheduleError::ConcurrentModification(String::from(
                        error_message,
                    ));
                }
                "NotFoundException" => {
                    return DeleteBusinessReportScheduleError::NotFound(String::from(error_message));
                }
                "ValidationException" => {
                    return DeleteBusinessReportScheduleError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return DeleteBusinessReportScheduleError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteBusinessReportScheduleError {
    fn from(err: serde_json::error::Error) -> DeleteBusinessReportScheduleError {
        DeleteBusinessReportScheduleError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteBusinessReportScheduleError {
    fn from(err: CredentialsError) -> DeleteBusinessReportScheduleError {
        DeleteBusinessReportScheduleError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteBusinessReportScheduleError {
    fn from(err: HttpDispatchError) -> DeleteBusinessReportScheduleError {
        DeleteBusinessReportScheduleError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteBusinessReportScheduleError {
    fn from(err: io::Error) -> DeleteBusinessReportScheduleError {
        DeleteBusinessReportScheduleError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteBusinessReportScheduleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteBusinessReportScheduleError {
    fn description(&self) -> &str {
        match *self {
            DeleteBusinessReportScheduleError::ConcurrentModification(ref cause) => cause,
            DeleteBusinessReportScheduleError::NotFound(ref cause) => cause,
            DeleteBusinessReportScheduleError::Validation(ref cause) => cause,
            DeleteBusinessReportScheduleError::Credentials(ref err) => err.description(),
            DeleteBusinessReportScheduleError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteBusinessReportScheduleError::ParseError(ref cause) => cause,
            DeleteBusinessReportScheduleError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteConferenceProvider
#[derive(Debug, PartialEq)]
pub enum DeleteConferenceProviderError {
    /// <p>The resource is not found.</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DeleteConferenceProviderError {
    pub fn from_response(res: BufferedHttpResponse) -> DeleteConferenceProviderError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "NotFoundException" => {
                    return DeleteConferenceProviderError::NotFound(String::from(error_message));
                }
                "ValidationException" => {
                    return DeleteConferenceProviderError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return DeleteConferenceProviderError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteConferenceProviderError {
    fn from(err: serde_json::error::Error) -> DeleteConferenceProviderError {
        DeleteConferenceProviderError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteConferenceProviderError {
    fn from(err: CredentialsError) -> DeleteConferenceProviderError {
        DeleteConferenceProviderError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteConferenceProviderError {
    fn from(err: HttpDispatchError) -> DeleteConferenceProviderError {
        DeleteConferenceProviderError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteConferenceProviderError {
    fn from(err: io::Error) -> DeleteConferenceProviderError {
        DeleteConferenceProviderError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteConferenceProviderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteConferenceProviderError {
    fn description(&self) -> &str {
        match *self {
            DeleteConferenceProviderError::NotFound(ref cause) => cause,
            DeleteConferenceProviderError::Validation(ref cause) => cause,
            DeleteConferenceProviderError::Credentials(ref err) => err.description(),
            DeleteConferenceProviderError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteConferenceProviderError::ParseError(ref cause) => cause,
            DeleteConferenceProviderError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteContact
#[derive(Debug, PartialEq)]
pub enum DeleteContactError {
    /// <p>Concurrent modification of resources. HTTP Status Code: 400.</p>
    ConcurrentModification(String),
    /// <p>The resource is not found.</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DeleteContactError {
    pub fn from_response(res: BufferedHttpResponse) -> DeleteContactError {
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
                    return DeleteContactError::ConcurrentModification(String::from(error_message));
                }
                "NotFoundException" => {
                    return DeleteContactError::NotFound(String::from(error_message));
                }
                "ValidationException" => {
                    return DeleteContactError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return DeleteContactError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteContactError {
    fn from(err: serde_json::error::Error) -> DeleteContactError {
        DeleteContactError::ParseError(err.description().to_string())
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
            DeleteContactError::ConcurrentModification(ref cause) => cause,
            DeleteContactError::NotFound(ref cause) => cause,
            DeleteContactError::Validation(ref cause) => cause,
            DeleteContactError::Credentials(ref err) => err.description(),
            DeleteContactError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteContactError::ParseError(ref cause) => cause,
            DeleteContactError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteDevice
#[derive(Debug, PartialEq)]
pub enum DeleteDeviceError {
    /// <p>Concurrent modification of resources. HTTP Status Code: 400.</p>
    ConcurrentModification(String),
    /// <p>The Certificate Authority can't issue or revoke a certificate.</p>
    InvalidCertificateAuthority(String),
    /// <p>The resource is not found.</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DeleteDeviceError {
    pub fn from_response(res: BufferedHttpResponse) -> DeleteDeviceError {
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
                    return DeleteDeviceError::ConcurrentModification(String::from(error_message));
                }
                "InvalidCertificateAuthorityException" => {
                    return DeleteDeviceError::InvalidCertificateAuthority(String::from(
                        error_message,
                    ));
                }
                "NotFoundException" => {
                    return DeleteDeviceError::NotFound(String::from(error_message));
                }
                "ValidationException" => {
                    return DeleteDeviceError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return DeleteDeviceError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteDeviceError {
    fn from(err: serde_json::error::Error) -> DeleteDeviceError {
        DeleteDeviceError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteDeviceError {
    fn from(err: CredentialsError) -> DeleteDeviceError {
        DeleteDeviceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteDeviceError {
    fn from(err: HttpDispatchError) -> DeleteDeviceError {
        DeleteDeviceError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteDeviceError {
    fn from(err: io::Error) -> DeleteDeviceError {
        DeleteDeviceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteDeviceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteDeviceError {
    fn description(&self) -> &str {
        match *self {
            DeleteDeviceError::ConcurrentModification(ref cause) => cause,
            DeleteDeviceError::InvalidCertificateAuthority(ref cause) => cause,
            DeleteDeviceError::NotFound(ref cause) => cause,
            DeleteDeviceError::Validation(ref cause) => cause,
            DeleteDeviceError::Credentials(ref err) => err.description(),
            DeleteDeviceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteDeviceError::ParseError(ref cause) => cause,
            DeleteDeviceError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteProfile
#[derive(Debug, PartialEq)]
pub enum DeleteProfileError {
    /// <p>Concurrent modification of resources. HTTP Status Code: 400.</p>
    ConcurrentModification(String),
    /// <p>The resource is not found.</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DeleteProfileError {
    pub fn from_response(res: BufferedHttpResponse) -> DeleteProfileError {
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
                    return DeleteProfileError::ConcurrentModification(String::from(error_message));
                }
                "NotFoundException" => {
                    return DeleteProfileError::NotFound(String::from(error_message));
                }
                "ValidationException" => {
                    return DeleteProfileError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return DeleteProfileError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteProfileError {
    fn from(err: serde_json::error::Error) -> DeleteProfileError {
        DeleteProfileError::ParseError(err.description().to_string())
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
            DeleteProfileError::ConcurrentModification(ref cause) => cause,
            DeleteProfileError::NotFound(ref cause) => cause,
            DeleteProfileError::Validation(ref cause) => cause,
            DeleteProfileError::Credentials(ref err) => err.description(),
            DeleteProfileError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteProfileError::ParseError(ref cause) => cause,
            DeleteProfileError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteRoom
#[derive(Debug, PartialEq)]
pub enum DeleteRoomError {
    /// <p>Concurrent modification of resources. HTTP Status Code: 400.</p>
    ConcurrentModification(String),
    /// <p>The resource is not found.</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DeleteRoomError {
    pub fn from_response(res: BufferedHttpResponse) -> DeleteRoomError {
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
                    return DeleteRoomError::ConcurrentModification(String::from(error_message));
                }
                "NotFoundException" => {
                    return DeleteRoomError::NotFound(String::from(error_message));
                }
                "ValidationException" => {
                    return DeleteRoomError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return DeleteRoomError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteRoomError {
    fn from(err: serde_json::error::Error) -> DeleteRoomError {
        DeleteRoomError::ParseError(err.description().to_string())
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
            DeleteRoomError::ConcurrentModification(ref cause) => cause,
            DeleteRoomError::NotFound(ref cause) => cause,
            DeleteRoomError::Validation(ref cause) => cause,
            DeleteRoomError::Credentials(ref err) => err.description(),
            DeleteRoomError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteRoomError::ParseError(ref cause) => cause,
            DeleteRoomError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteRoomSkillParameter
#[derive(Debug, PartialEq)]
pub enum DeleteRoomSkillParameterError {
    /// <p>Concurrent modification of resources. HTTP Status Code: 400.</p>
    ConcurrentModification(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DeleteRoomSkillParameterError {
    pub fn from_response(res: BufferedHttpResponse) -> DeleteRoomSkillParameterError {
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
                    return DeleteRoomSkillParameterError::ConcurrentModification(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return DeleteRoomSkillParameterError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return DeleteRoomSkillParameterError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteRoomSkillParameterError {
    fn from(err: serde_json::error::Error) -> DeleteRoomSkillParameterError {
        DeleteRoomSkillParameterError::ParseError(err.description().to_string())
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
            DeleteRoomSkillParameterError::ConcurrentModification(ref cause) => cause,
            DeleteRoomSkillParameterError::Validation(ref cause) => cause,
            DeleteRoomSkillParameterError::Credentials(ref err) => err.description(),
            DeleteRoomSkillParameterError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteRoomSkillParameterError::ParseError(ref cause) => cause,
            DeleteRoomSkillParameterError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteSkillAuthorization
#[derive(Debug, PartialEq)]
pub enum DeleteSkillAuthorizationError {
    /// <p>Concurrent modification of resources. HTTP Status Code: 400.</p>
    ConcurrentModification(String),
    /// <p>The resource is not found.</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DeleteSkillAuthorizationError {
    pub fn from_response(res: BufferedHttpResponse) -> DeleteSkillAuthorizationError {
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
                    return DeleteSkillAuthorizationError::ConcurrentModification(String::from(
                        error_message,
                    ));
                }
                "NotFoundException" => {
                    return DeleteSkillAuthorizationError::NotFound(String::from(error_message));
                }
                "ValidationException" => {
                    return DeleteSkillAuthorizationError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return DeleteSkillAuthorizationError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteSkillAuthorizationError {
    fn from(err: serde_json::error::Error) -> DeleteSkillAuthorizationError {
        DeleteSkillAuthorizationError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteSkillAuthorizationError {
    fn from(err: CredentialsError) -> DeleteSkillAuthorizationError {
        DeleteSkillAuthorizationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteSkillAuthorizationError {
    fn from(err: HttpDispatchError) -> DeleteSkillAuthorizationError {
        DeleteSkillAuthorizationError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteSkillAuthorizationError {
    fn from(err: io::Error) -> DeleteSkillAuthorizationError {
        DeleteSkillAuthorizationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteSkillAuthorizationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteSkillAuthorizationError {
    fn description(&self) -> &str {
        match *self {
            DeleteSkillAuthorizationError::ConcurrentModification(ref cause) => cause,
            DeleteSkillAuthorizationError::NotFound(ref cause) => cause,
            DeleteSkillAuthorizationError::Validation(ref cause) => cause,
            DeleteSkillAuthorizationError::Credentials(ref err) => err.description(),
            DeleteSkillAuthorizationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteSkillAuthorizationError::ParseError(ref cause) => cause,
            DeleteSkillAuthorizationError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteSkillGroup
#[derive(Debug, PartialEq)]
pub enum DeleteSkillGroupError {
    /// <p>Concurrent modification of resources. HTTP Status Code: 400.</p>
    ConcurrentModification(String),
    /// <p>The resource is not found.</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DeleteSkillGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> DeleteSkillGroupError {
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
                    return DeleteSkillGroupError::ConcurrentModification(String::from(
                        error_message,
                    ));
                }
                "NotFoundException" => {
                    return DeleteSkillGroupError::NotFound(String::from(error_message));
                }
                "ValidationException" => {
                    return DeleteSkillGroupError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return DeleteSkillGroupError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteSkillGroupError {
    fn from(err: serde_json::error::Error) -> DeleteSkillGroupError {
        DeleteSkillGroupError::ParseError(err.description().to_string())
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
            DeleteSkillGroupError::ConcurrentModification(ref cause) => cause,
            DeleteSkillGroupError::NotFound(ref cause) => cause,
            DeleteSkillGroupError::Validation(ref cause) => cause,
            DeleteSkillGroupError::Credentials(ref err) => err.description(),
            DeleteSkillGroupError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteSkillGroupError::ParseError(ref cause) => cause,
            DeleteSkillGroupError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteUser
#[derive(Debug, PartialEq)]
pub enum DeleteUserError {
    /// <p>Concurrent modification of resources. HTTP Status Code: 400.</p>
    ConcurrentModification(String),
    /// <p>The resource is not found.</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DeleteUserError {
    pub fn from_response(res: BufferedHttpResponse) -> DeleteUserError {
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
                    return DeleteUserError::ConcurrentModification(String::from(error_message));
                }
                "NotFoundException" => {
                    return DeleteUserError::NotFound(String::from(error_message));
                }
                "ValidationException" => {
                    return DeleteUserError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return DeleteUserError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteUserError {
    fn from(err: serde_json::error::Error) -> DeleteUserError {
        DeleteUserError::ParseError(err.description().to_string())
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
            DeleteUserError::ConcurrentModification(ref cause) => cause,
            DeleteUserError::NotFound(ref cause) => cause,
            DeleteUserError::Validation(ref cause) => cause,
            DeleteUserError::Credentials(ref err) => err.description(),
            DeleteUserError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteUserError::ParseError(ref cause) => cause,
            DeleteUserError::Unknown(_) => "unknown error",
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
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DisassociateContactFromAddressBookError {
    pub fn from_response(res: BufferedHttpResponse) -> DisassociateContactFromAddressBookError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ValidationException" => {
                    return DisassociateContactFromAddressBookError::Validation(
                        error_message.to_string(),
                    );
                }
                _ => {}
            }
        }
        return DisassociateContactFromAddressBookError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DisassociateContactFromAddressBookError {
    fn from(err: serde_json::error::Error) -> DisassociateContactFromAddressBookError {
        DisassociateContactFromAddressBookError::ParseError(err.description().to_string())
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
            DisassociateContactFromAddressBookError::ParseError(ref cause) => cause,
            DisassociateContactFromAddressBookError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DisassociateDeviceFromRoom
#[derive(Debug, PartialEq)]
pub enum DisassociateDeviceFromRoomError {
    /// <p>Concurrent modification of resources. HTTP Status Code: 400.</p>
    ConcurrentModification(String),
    /// <p>The request failed because this device is no longer registered and therefore no longer managed by this account.</p>
    DeviceNotRegistered(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DisassociateDeviceFromRoomError {
    pub fn from_response(res: BufferedHttpResponse) -> DisassociateDeviceFromRoomError {
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
                    return DisassociateDeviceFromRoomError::ConcurrentModification(String::from(
                        error_message,
                    ));
                }
                "DeviceNotRegisteredException" => {
                    return DisassociateDeviceFromRoomError::DeviceNotRegistered(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return DisassociateDeviceFromRoomError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return DisassociateDeviceFromRoomError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DisassociateDeviceFromRoomError {
    fn from(err: serde_json::error::Error) -> DisassociateDeviceFromRoomError {
        DisassociateDeviceFromRoomError::ParseError(err.description().to_string())
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
            DisassociateDeviceFromRoomError::ConcurrentModification(ref cause) => cause,
            DisassociateDeviceFromRoomError::DeviceNotRegistered(ref cause) => cause,
            DisassociateDeviceFromRoomError::Validation(ref cause) => cause,
            DisassociateDeviceFromRoomError::Credentials(ref err) => err.description(),
            DisassociateDeviceFromRoomError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DisassociateDeviceFromRoomError::ParseError(ref cause) => cause,
            DisassociateDeviceFromRoomError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DisassociateSkillFromSkillGroup
#[derive(Debug, PartialEq)]
pub enum DisassociateSkillFromSkillGroupError {
    /// <p>Concurrent modification of resources. HTTP Status Code: 400.</p>
    ConcurrentModification(String),
    /// <p>The resource is not found.</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DisassociateSkillFromSkillGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> DisassociateSkillFromSkillGroupError {
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
                    return DisassociateSkillFromSkillGroupError::ConcurrentModification(
                        String::from(error_message),
                    );
                }
                "NotFoundException" => {
                    return DisassociateSkillFromSkillGroupError::NotFound(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return DisassociateSkillFromSkillGroupError::Validation(
                        error_message.to_string(),
                    );
                }
                _ => {}
            }
        }
        return DisassociateSkillFromSkillGroupError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DisassociateSkillFromSkillGroupError {
    fn from(err: serde_json::error::Error) -> DisassociateSkillFromSkillGroupError {
        DisassociateSkillFromSkillGroupError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DisassociateSkillFromSkillGroupError {
    fn from(err: CredentialsError) -> DisassociateSkillFromSkillGroupError {
        DisassociateSkillFromSkillGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DisassociateSkillFromSkillGroupError {
    fn from(err: HttpDispatchError) -> DisassociateSkillFromSkillGroupError {
        DisassociateSkillFromSkillGroupError::HttpDispatch(err)
    }
}
impl From<io::Error> for DisassociateSkillFromSkillGroupError {
    fn from(err: io::Error) -> DisassociateSkillFromSkillGroupError {
        DisassociateSkillFromSkillGroupError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DisassociateSkillFromSkillGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DisassociateSkillFromSkillGroupError {
    fn description(&self) -> &str {
        match *self {
            DisassociateSkillFromSkillGroupError::ConcurrentModification(ref cause) => cause,
            DisassociateSkillFromSkillGroupError::NotFound(ref cause) => cause,
            DisassociateSkillFromSkillGroupError::Validation(ref cause) => cause,
            DisassociateSkillFromSkillGroupError::Credentials(ref err) => err.description(),
            DisassociateSkillFromSkillGroupError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DisassociateSkillFromSkillGroupError::ParseError(ref cause) => cause,
            DisassociateSkillFromSkillGroupError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DisassociateSkillFromUsers
#[derive(Debug, PartialEq)]
pub enum DisassociateSkillFromUsersError {
    /// <p>Concurrent modification of resources. HTTP Status Code: 400.</p>
    ConcurrentModification(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DisassociateSkillFromUsersError {
    pub fn from_response(res: BufferedHttpResponse) -> DisassociateSkillFromUsersError {
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
                    return DisassociateSkillFromUsersError::ConcurrentModification(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return DisassociateSkillFromUsersError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return DisassociateSkillFromUsersError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DisassociateSkillFromUsersError {
    fn from(err: serde_json::error::Error) -> DisassociateSkillFromUsersError {
        DisassociateSkillFromUsersError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DisassociateSkillFromUsersError {
    fn from(err: CredentialsError) -> DisassociateSkillFromUsersError {
        DisassociateSkillFromUsersError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DisassociateSkillFromUsersError {
    fn from(err: HttpDispatchError) -> DisassociateSkillFromUsersError {
        DisassociateSkillFromUsersError::HttpDispatch(err)
    }
}
impl From<io::Error> for DisassociateSkillFromUsersError {
    fn from(err: io::Error) -> DisassociateSkillFromUsersError {
        DisassociateSkillFromUsersError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DisassociateSkillFromUsersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DisassociateSkillFromUsersError {
    fn description(&self) -> &str {
        match *self {
            DisassociateSkillFromUsersError::ConcurrentModification(ref cause) => cause,
            DisassociateSkillFromUsersError::Validation(ref cause) => cause,
            DisassociateSkillFromUsersError::Credentials(ref err) => err.description(),
            DisassociateSkillFromUsersError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DisassociateSkillFromUsersError::ParseError(ref cause) => cause,
            DisassociateSkillFromUsersError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DisassociateSkillGroupFromRoom
#[derive(Debug, PartialEq)]
pub enum DisassociateSkillGroupFromRoomError {
    /// <p>Concurrent modification of resources. HTTP Status Code: 400.</p>
    ConcurrentModification(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DisassociateSkillGroupFromRoomError {
    pub fn from_response(res: BufferedHttpResponse) -> DisassociateSkillGroupFromRoomError {
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
                    return DisassociateSkillGroupFromRoomError::ConcurrentModification(
                        String::from(error_message),
                    );
                }
                "ValidationException" => {
                    return DisassociateSkillGroupFromRoomError::Validation(
                        error_message.to_string(),
                    );
                }
                _ => {}
            }
        }
        return DisassociateSkillGroupFromRoomError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DisassociateSkillGroupFromRoomError {
    fn from(err: serde_json::error::Error) -> DisassociateSkillGroupFromRoomError {
        DisassociateSkillGroupFromRoomError::ParseError(err.description().to_string())
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
            DisassociateSkillGroupFromRoomError::ConcurrentModification(ref cause) => cause,
            DisassociateSkillGroupFromRoomError::Validation(ref cause) => cause,
            DisassociateSkillGroupFromRoomError::Credentials(ref err) => err.description(),
            DisassociateSkillGroupFromRoomError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DisassociateSkillGroupFromRoomError::ParseError(ref cause) => cause,
            DisassociateSkillGroupFromRoomError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ForgetSmartHomeAppliances
#[derive(Debug, PartialEq)]
pub enum ForgetSmartHomeAppliancesError {
    /// <p>The resource is not found.</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl ForgetSmartHomeAppliancesError {
    pub fn from_response(res: BufferedHttpResponse) -> ForgetSmartHomeAppliancesError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "NotFoundException" => {
                    return ForgetSmartHomeAppliancesError::NotFound(String::from(error_message));
                }
                "ValidationException" => {
                    return ForgetSmartHomeAppliancesError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return ForgetSmartHomeAppliancesError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ForgetSmartHomeAppliancesError {
    fn from(err: serde_json::error::Error) -> ForgetSmartHomeAppliancesError {
        ForgetSmartHomeAppliancesError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ForgetSmartHomeAppliancesError {
    fn from(err: CredentialsError) -> ForgetSmartHomeAppliancesError {
        ForgetSmartHomeAppliancesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ForgetSmartHomeAppliancesError {
    fn from(err: HttpDispatchError) -> ForgetSmartHomeAppliancesError {
        ForgetSmartHomeAppliancesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ForgetSmartHomeAppliancesError {
    fn from(err: io::Error) -> ForgetSmartHomeAppliancesError {
        ForgetSmartHomeAppliancesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ForgetSmartHomeAppliancesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ForgetSmartHomeAppliancesError {
    fn description(&self) -> &str {
        match *self {
            ForgetSmartHomeAppliancesError::NotFound(ref cause) => cause,
            ForgetSmartHomeAppliancesError::Validation(ref cause) => cause,
            ForgetSmartHomeAppliancesError::Credentials(ref err) => err.description(),
            ForgetSmartHomeAppliancesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ForgetSmartHomeAppliancesError::ParseError(ref cause) => cause,
            ForgetSmartHomeAppliancesError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetAddressBook
#[derive(Debug, PartialEq)]
pub enum GetAddressBookError {
    /// <p>The resource is not found.</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetAddressBookError {
    pub fn from_response(res: BufferedHttpResponse) -> GetAddressBookError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "NotFoundException" => {
                    return GetAddressBookError::NotFound(String::from(error_message));
                }
                "ValidationException" => {
                    return GetAddressBookError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return GetAddressBookError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetAddressBookError {
    fn from(err: serde_json::error::Error) -> GetAddressBookError {
        GetAddressBookError::ParseError(err.description().to_string())
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
            GetAddressBookError::ParseError(ref cause) => cause,
            GetAddressBookError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetConferencePreference
#[derive(Debug, PartialEq)]
pub enum GetConferencePreferenceError {
    /// <p>The resource is not found.</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetConferencePreferenceError {
    pub fn from_response(res: BufferedHttpResponse) -> GetConferencePreferenceError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "NotFoundException" => {
                    return GetConferencePreferenceError::NotFound(String::from(error_message));
                }
                "ValidationException" => {
                    return GetConferencePreferenceError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return GetConferencePreferenceError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetConferencePreferenceError {
    fn from(err: serde_json::error::Error) -> GetConferencePreferenceError {
        GetConferencePreferenceError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetConferencePreferenceError {
    fn from(err: CredentialsError) -> GetConferencePreferenceError {
        GetConferencePreferenceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetConferencePreferenceError {
    fn from(err: HttpDispatchError) -> GetConferencePreferenceError {
        GetConferencePreferenceError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetConferencePreferenceError {
    fn from(err: io::Error) -> GetConferencePreferenceError {
        GetConferencePreferenceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetConferencePreferenceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetConferencePreferenceError {
    fn description(&self) -> &str {
        match *self {
            GetConferencePreferenceError::NotFound(ref cause) => cause,
            GetConferencePreferenceError::Validation(ref cause) => cause,
            GetConferencePreferenceError::Credentials(ref err) => err.description(),
            GetConferencePreferenceError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetConferencePreferenceError::ParseError(ref cause) => cause,
            GetConferencePreferenceError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetConferenceProvider
#[derive(Debug, PartialEq)]
pub enum GetConferenceProviderError {
    /// <p>The resource is not found.</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetConferenceProviderError {
    pub fn from_response(res: BufferedHttpResponse) -> GetConferenceProviderError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "NotFoundException" => {
                    return GetConferenceProviderError::NotFound(String::from(error_message));
                }
                "ValidationException" => {
                    return GetConferenceProviderError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return GetConferenceProviderError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetConferenceProviderError {
    fn from(err: serde_json::error::Error) -> GetConferenceProviderError {
        GetConferenceProviderError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetConferenceProviderError {
    fn from(err: CredentialsError) -> GetConferenceProviderError {
        GetConferenceProviderError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetConferenceProviderError {
    fn from(err: HttpDispatchError) -> GetConferenceProviderError {
        GetConferenceProviderError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetConferenceProviderError {
    fn from(err: io::Error) -> GetConferenceProviderError {
        GetConferenceProviderError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetConferenceProviderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetConferenceProviderError {
    fn description(&self) -> &str {
        match *self {
            GetConferenceProviderError::NotFound(ref cause) => cause,
            GetConferenceProviderError::Validation(ref cause) => cause,
            GetConferenceProviderError::Credentials(ref err) => err.description(),
            GetConferenceProviderError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetConferenceProviderError::ParseError(ref cause) => cause,
            GetConferenceProviderError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetContact
#[derive(Debug, PartialEq)]
pub enum GetContactError {
    /// <p>The resource is not found.</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetContactError {
    pub fn from_response(res: BufferedHttpResponse) -> GetContactError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "NotFoundException" => {
                    return GetContactError::NotFound(String::from(error_message));
                }
                "ValidationException" => {
                    return GetContactError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return GetContactError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetContactError {
    fn from(err: serde_json::error::Error) -> GetContactError {
        GetContactError::ParseError(err.description().to_string())
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
            GetContactError::ParseError(ref cause) => cause,
            GetContactError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetDevice
#[derive(Debug, PartialEq)]
pub enum GetDeviceError {
    /// <p>The resource is not found.</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetDeviceError {
    pub fn from_response(res: BufferedHttpResponse) -> GetDeviceError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "NotFoundException" => return GetDeviceError::NotFound(String::from(error_message)),
                "ValidationException" => {
                    return GetDeviceError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return GetDeviceError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetDeviceError {
    fn from(err: serde_json::error::Error) -> GetDeviceError {
        GetDeviceError::ParseError(err.description().to_string())
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
            GetDeviceError::ParseError(ref cause) => cause,
            GetDeviceError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetProfile
#[derive(Debug, PartialEq)]
pub enum GetProfileError {
    /// <p>The resource is not found.</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetProfileError {
    pub fn from_response(res: BufferedHttpResponse) -> GetProfileError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "NotFoundException" => {
                    return GetProfileError::NotFound(String::from(error_message));
                }
                "ValidationException" => {
                    return GetProfileError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return GetProfileError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetProfileError {
    fn from(err: serde_json::error::Error) -> GetProfileError {
        GetProfileError::ParseError(err.description().to_string())
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
            GetProfileError::ParseError(ref cause) => cause,
            GetProfileError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetRoom
#[derive(Debug, PartialEq)]
pub enum GetRoomError {
    /// <p>The resource is not found.</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetRoomError {
    pub fn from_response(res: BufferedHttpResponse) -> GetRoomError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "NotFoundException" => return GetRoomError::NotFound(String::from(error_message)),
                "ValidationException" => return GetRoomError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return GetRoomError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetRoomError {
    fn from(err: serde_json::error::Error) -> GetRoomError {
        GetRoomError::ParseError(err.description().to_string())
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
            GetRoomError::ParseError(ref cause) => cause,
            GetRoomError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetRoomSkillParameter
#[derive(Debug, PartialEq)]
pub enum GetRoomSkillParameterError {
    /// <p>The resource is not found.</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetRoomSkillParameterError {
    pub fn from_response(res: BufferedHttpResponse) -> GetRoomSkillParameterError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "NotFoundException" => {
                    return GetRoomSkillParameterError::NotFound(String::from(error_message));
                }
                "ValidationException" => {
                    return GetRoomSkillParameterError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return GetRoomSkillParameterError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetRoomSkillParameterError {
    fn from(err: serde_json::error::Error) -> GetRoomSkillParameterError {
        GetRoomSkillParameterError::ParseError(err.description().to_string())
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
            GetRoomSkillParameterError::ParseError(ref cause) => cause,
            GetRoomSkillParameterError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetSkillGroup
#[derive(Debug, PartialEq)]
pub enum GetSkillGroupError {
    /// <p>The resource is not found.</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetSkillGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> GetSkillGroupError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "NotFoundException" => {
                    return GetSkillGroupError::NotFound(String::from(error_message));
                }
                "ValidationException" => {
                    return GetSkillGroupError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return GetSkillGroupError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetSkillGroupError {
    fn from(err: serde_json::error::Error) -> GetSkillGroupError {
        GetSkillGroupError::ParseError(err.description().to_string())
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
            GetSkillGroupError::ParseError(ref cause) => cause,
            GetSkillGroupError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListBusinessReportSchedules
#[derive(Debug, PartialEq)]
pub enum ListBusinessReportSchedulesError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl ListBusinessReportSchedulesError {
    pub fn from_response(res: BufferedHttpResponse) -> ListBusinessReportSchedulesError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ValidationException" => {
                    return ListBusinessReportSchedulesError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return ListBusinessReportSchedulesError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListBusinessReportSchedulesError {
    fn from(err: serde_json::error::Error) -> ListBusinessReportSchedulesError {
        ListBusinessReportSchedulesError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListBusinessReportSchedulesError {
    fn from(err: CredentialsError) -> ListBusinessReportSchedulesError {
        ListBusinessReportSchedulesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListBusinessReportSchedulesError {
    fn from(err: HttpDispatchError) -> ListBusinessReportSchedulesError {
        ListBusinessReportSchedulesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListBusinessReportSchedulesError {
    fn from(err: io::Error) -> ListBusinessReportSchedulesError {
        ListBusinessReportSchedulesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListBusinessReportSchedulesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListBusinessReportSchedulesError {
    fn description(&self) -> &str {
        match *self {
            ListBusinessReportSchedulesError::Validation(ref cause) => cause,
            ListBusinessReportSchedulesError::Credentials(ref err) => err.description(),
            ListBusinessReportSchedulesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListBusinessReportSchedulesError::ParseError(ref cause) => cause,
            ListBusinessReportSchedulesError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListConferenceProviders
#[derive(Debug, PartialEq)]
pub enum ListConferenceProvidersError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl ListConferenceProvidersError {
    pub fn from_response(res: BufferedHttpResponse) -> ListConferenceProvidersError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ValidationException" => {
                    return ListConferenceProvidersError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return ListConferenceProvidersError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListConferenceProvidersError {
    fn from(err: serde_json::error::Error) -> ListConferenceProvidersError {
        ListConferenceProvidersError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListConferenceProvidersError {
    fn from(err: CredentialsError) -> ListConferenceProvidersError {
        ListConferenceProvidersError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListConferenceProvidersError {
    fn from(err: HttpDispatchError) -> ListConferenceProvidersError {
        ListConferenceProvidersError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListConferenceProvidersError {
    fn from(err: io::Error) -> ListConferenceProvidersError {
        ListConferenceProvidersError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListConferenceProvidersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListConferenceProvidersError {
    fn description(&self) -> &str {
        match *self {
            ListConferenceProvidersError::Validation(ref cause) => cause,
            ListConferenceProvidersError::Credentials(ref err) => err.description(),
            ListConferenceProvidersError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListConferenceProvidersError::ParseError(ref cause) => cause,
            ListConferenceProvidersError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListDeviceEvents
#[derive(Debug, PartialEq)]
pub enum ListDeviceEventsError {
    /// <p>The resource is not found.</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl ListDeviceEventsError {
    pub fn from_response(res: BufferedHttpResponse) -> ListDeviceEventsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "NotFoundException" => {
                    return ListDeviceEventsError::NotFound(String::from(error_message));
                }
                "ValidationException" => {
                    return ListDeviceEventsError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return ListDeviceEventsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListDeviceEventsError {
    fn from(err: serde_json::error::Error) -> ListDeviceEventsError {
        ListDeviceEventsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListDeviceEventsError {
    fn from(err: CredentialsError) -> ListDeviceEventsError {
        ListDeviceEventsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListDeviceEventsError {
    fn from(err: HttpDispatchError) -> ListDeviceEventsError {
        ListDeviceEventsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListDeviceEventsError {
    fn from(err: io::Error) -> ListDeviceEventsError {
        ListDeviceEventsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListDeviceEventsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListDeviceEventsError {
    fn description(&self) -> &str {
        match *self {
            ListDeviceEventsError::NotFound(ref cause) => cause,
            ListDeviceEventsError::Validation(ref cause) => cause,
            ListDeviceEventsError::Credentials(ref err) => err.description(),
            ListDeviceEventsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListDeviceEventsError::ParseError(ref cause) => cause,
            ListDeviceEventsError::Unknown(_) => "unknown error",
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
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl ListSkillsError {
    pub fn from_response(res: BufferedHttpResponse) -> ListSkillsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ValidationException" => {
                    return ListSkillsError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return ListSkillsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListSkillsError {
    fn from(err: serde_json::error::Error) -> ListSkillsError {
        ListSkillsError::ParseError(err.description().to_string())
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
            ListSkillsError::ParseError(ref cause) => cause,
            ListSkillsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListSkillsStoreCategories
#[derive(Debug, PartialEq)]
pub enum ListSkillsStoreCategoriesError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl ListSkillsStoreCategoriesError {
    pub fn from_response(res: BufferedHttpResponse) -> ListSkillsStoreCategoriesError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ValidationException" => {
                    return ListSkillsStoreCategoriesError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return ListSkillsStoreCategoriesError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListSkillsStoreCategoriesError {
    fn from(err: serde_json::error::Error) -> ListSkillsStoreCategoriesError {
        ListSkillsStoreCategoriesError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListSkillsStoreCategoriesError {
    fn from(err: CredentialsError) -> ListSkillsStoreCategoriesError {
        ListSkillsStoreCategoriesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListSkillsStoreCategoriesError {
    fn from(err: HttpDispatchError) -> ListSkillsStoreCategoriesError {
        ListSkillsStoreCategoriesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListSkillsStoreCategoriesError {
    fn from(err: io::Error) -> ListSkillsStoreCategoriesError {
        ListSkillsStoreCategoriesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListSkillsStoreCategoriesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListSkillsStoreCategoriesError {
    fn description(&self) -> &str {
        match *self {
            ListSkillsStoreCategoriesError::Validation(ref cause) => cause,
            ListSkillsStoreCategoriesError::Credentials(ref err) => err.description(),
            ListSkillsStoreCategoriesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListSkillsStoreCategoriesError::ParseError(ref cause) => cause,
            ListSkillsStoreCategoriesError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListSkillsStoreSkillsByCategory
#[derive(Debug, PartialEq)]
pub enum ListSkillsStoreSkillsByCategoryError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl ListSkillsStoreSkillsByCategoryError {
    pub fn from_response(res: BufferedHttpResponse) -> ListSkillsStoreSkillsByCategoryError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ValidationException" => {
                    return ListSkillsStoreSkillsByCategoryError::Validation(
                        error_message.to_string(),
                    );
                }
                _ => {}
            }
        }
        return ListSkillsStoreSkillsByCategoryError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListSkillsStoreSkillsByCategoryError {
    fn from(err: serde_json::error::Error) -> ListSkillsStoreSkillsByCategoryError {
        ListSkillsStoreSkillsByCategoryError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListSkillsStoreSkillsByCategoryError {
    fn from(err: CredentialsError) -> ListSkillsStoreSkillsByCategoryError {
        ListSkillsStoreSkillsByCategoryError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListSkillsStoreSkillsByCategoryError {
    fn from(err: HttpDispatchError) -> ListSkillsStoreSkillsByCategoryError {
        ListSkillsStoreSkillsByCategoryError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListSkillsStoreSkillsByCategoryError {
    fn from(err: io::Error) -> ListSkillsStoreSkillsByCategoryError {
        ListSkillsStoreSkillsByCategoryError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListSkillsStoreSkillsByCategoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListSkillsStoreSkillsByCategoryError {
    fn description(&self) -> &str {
        match *self {
            ListSkillsStoreSkillsByCategoryError::Validation(ref cause) => cause,
            ListSkillsStoreSkillsByCategoryError::Credentials(ref err) => err.description(),
            ListSkillsStoreSkillsByCategoryError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListSkillsStoreSkillsByCategoryError::ParseError(ref cause) => cause,
            ListSkillsStoreSkillsByCategoryError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListSmartHomeAppliances
#[derive(Debug, PartialEq)]
pub enum ListSmartHomeAppliancesError {
    /// <p>The resource is not found.</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl ListSmartHomeAppliancesError {
    pub fn from_response(res: BufferedHttpResponse) -> ListSmartHomeAppliancesError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "NotFoundException" => {
                    return ListSmartHomeAppliancesError::NotFound(String::from(error_message));
                }
                "ValidationException" => {
                    return ListSmartHomeAppliancesError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return ListSmartHomeAppliancesError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListSmartHomeAppliancesError {
    fn from(err: serde_json::error::Error) -> ListSmartHomeAppliancesError {
        ListSmartHomeAppliancesError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListSmartHomeAppliancesError {
    fn from(err: CredentialsError) -> ListSmartHomeAppliancesError {
        ListSmartHomeAppliancesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListSmartHomeAppliancesError {
    fn from(err: HttpDispatchError) -> ListSmartHomeAppliancesError {
        ListSmartHomeAppliancesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListSmartHomeAppliancesError {
    fn from(err: io::Error) -> ListSmartHomeAppliancesError {
        ListSmartHomeAppliancesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListSmartHomeAppliancesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListSmartHomeAppliancesError {
    fn description(&self) -> &str {
        match *self {
            ListSmartHomeAppliancesError::NotFound(ref cause) => cause,
            ListSmartHomeAppliancesError::Validation(ref cause) => cause,
            ListSmartHomeAppliancesError::Credentials(ref err) => err.description(),
            ListSmartHomeAppliancesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListSmartHomeAppliancesError::ParseError(ref cause) => cause,
            ListSmartHomeAppliancesError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListTags
#[derive(Debug, PartialEq)]
pub enum ListTagsError {
    /// <p>The resource is not found.</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl ListTagsError {
    pub fn from_response(res: BufferedHttpResponse) -> ListTagsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "NotFoundException" => return ListTagsError::NotFound(String::from(error_message)),
                "ValidationException" => {
                    return ListTagsError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return ListTagsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListTagsError {
    fn from(err: serde_json::error::Error) -> ListTagsError {
        ListTagsError::ParseError(err.description().to_string())
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
            ListTagsError::ParseError(ref cause) => cause,
            ListTagsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by PutConferencePreference
#[derive(Debug, PartialEq)]
pub enum PutConferencePreferenceError {
    /// <p>The resource is not found.</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl PutConferencePreferenceError {
    pub fn from_response(res: BufferedHttpResponse) -> PutConferencePreferenceError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "NotFoundException" => {
                    return PutConferencePreferenceError::NotFound(String::from(error_message));
                }
                "ValidationException" => {
                    return PutConferencePreferenceError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return PutConferencePreferenceError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for PutConferencePreferenceError {
    fn from(err: serde_json::error::Error) -> PutConferencePreferenceError {
        PutConferencePreferenceError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for PutConferencePreferenceError {
    fn from(err: CredentialsError) -> PutConferencePreferenceError {
        PutConferencePreferenceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutConferencePreferenceError {
    fn from(err: HttpDispatchError) -> PutConferencePreferenceError {
        PutConferencePreferenceError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutConferencePreferenceError {
    fn from(err: io::Error) -> PutConferencePreferenceError {
        PutConferencePreferenceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutConferencePreferenceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutConferencePreferenceError {
    fn description(&self) -> &str {
        match *self {
            PutConferencePreferenceError::NotFound(ref cause) => cause,
            PutConferencePreferenceError::Validation(ref cause) => cause,
            PutConferencePreferenceError::Credentials(ref err) => err.description(),
            PutConferencePreferenceError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            PutConferencePreferenceError::ParseError(ref cause) => cause,
            PutConferencePreferenceError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by PutRoomSkillParameter
#[derive(Debug, PartialEq)]
pub enum PutRoomSkillParameterError {
    /// <p>Concurrent modification of resources. HTTP Status Code: 400.</p>
    ConcurrentModification(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl PutRoomSkillParameterError {
    pub fn from_response(res: BufferedHttpResponse) -> PutRoomSkillParameterError {
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
                    return PutRoomSkillParameterError::ConcurrentModification(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return PutRoomSkillParameterError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return PutRoomSkillParameterError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for PutRoomSkillParameterError {
    fn from(err: serde_json::error::Error) -> PutRoomSkillParameterError {
        PutRoomSkillParameterError::ParseError(err.description().to_string())
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
            PutRoomSkillParameterError::ConcurrentModification(ref cause) => cause,
            PutRoomSkillParameterError::Validation(ref cause) => cause,
            PutRoomSkillParameterError::Credentials(ref err) => err.description(),
            PutRoomSkillParameterError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            PutRoomSkillParameterError::ParseError(ref cause) => cause,
            PutRoomSkillParameterError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by PutSkillAuthorization
#[derive(Debug, PartialEq)]
pub enum PutSkillAuthorizationError {
    /// <p>Concurrent modification of resources. HTTP Status Code: 400.</p>
    ConcurrentModification(String),
    /// <p>The caller has no permissions to operate on the resource involved in the API call.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl PutSkillAuthorizationError {
    pub fn from_response(res: BufferedHttpResponse) -> PutSkillAuthorizationError {
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
                    return PutSkillAuthorizationError::ConcurrentModification(String::from(
                        error_message,
                    ));
                }
                "UnauthorizedException" => {
                    return PutSkillAuthorizationError::Unauthorized(String::from(error_message));
                }
                "ValidationException" => {
                    return PutSkillAuthorizationError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return PutSkillAuthorizationError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for PutSkillAuthorizationError {
    fn from(err: serde_json::error::Error) -> PutSkillAuthorizationError {
        PutSkillAuthorizationError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for PutSkillAuthorizationError {
    fn from(err: CredentialsError) -> PutSkillAuthorizationError {
        PutSkillAuthorizationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutSkillAuthorizationError {
    fn from(err: HttpDispatchError) -> PutSkillAuthorizationError {
        PutSkillAuthorizationError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutSkillAuthorizationError {
    fn from(err: io::Error) -> PutSkillAuthorizationError {
        PutSkillAuthorizationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutSkillAuthorizationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutSkillAuthorizationError {
    fn description(&self) -> &str {
        match *self {
            PutSkillAuthorizationError::ConcurrentModification(ref cause) => cause,
            PutSkillAuthorizationError::Unauthorized(ref cause) => cause,
            PutSkillAuthorizationError::Validation(ref cause) => cause,
            PutSkillAuthorizationError::Credentials(ref err) => err.description(),
            PutSkillAuthorizationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            PutSkillAuthorizationError::ParseError(ref cause) => cause,
            PutSkillAuthorizationError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by RegisterAVSDevice
#[derive(Debug, PartialEq)]
pub enum RegisterAVSDeviceError {
    /// <p>Concurrent modification of resources. HTTP Status Code: 400.</p>
    ConcurrentModification(String),
    /// <p>The device is in an invalid state.</p>
    InvalidDevice(String),
    /// <p>You are performing an action that would put you beyond your account's limits.</p>
    LimitExceeded(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl RegisterAVSDeviceError {
    pub fn from_response(res: BufferedHttpResponse) -> RegisterAVSDeviceError {
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
                    return RegisterAVSDeviceError::ConcurrentModification(String::from(
                        error_message,
                    ));
                }
                "InvalidDeviceException" => {
                    return RegisterAVSDeviceError::InvalidDevice(String::from(error_message));
                }
                "LimitExceededException" => {
                    return RegisterAVSDeviceError::LimitExceeded(String::from(error_message));
                }
                "ValidationException" => {
                    return RegisterAVSDeviceError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return RegisterAVSDeviceError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for RegisterAVSDeviceError {
    fn from(err: serde_json::error::Error) -> RegisterAVSDeviceError {
        RegisterAVSDeviceError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for RegisterAVSDeviceError {
    fn from(err: CredentialsError) -> RegisterAVSDeviceError {
        RegisterAVSDeviceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RegisterAVSDeviceError {
    fn from(err: HttpDispatchError) -> RegisterAVSDeviceError {
        RegisterAVSDeviceError::HttpDispatch(err)
    }
}
impl From<io::Error> for RegisterAVSDeviceError {
    fn from(err: io::Error) -> RegisterAVSDeviceError {
        RegisterAVSDeviceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RegisterAVSDeviceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RegisterAVSDeviceError {
    fn description(&self) -> &str {
        match *self {
            RegisterAVSDeviceError::ConcurrentModification(ref cause) => cause,
            RegisterAVSDeviceError::InvalidDevice(ref cause) => cause,
            RegisterAVSDeviceError::LimitExceeded(ref cause) => cause,
            RegisterAVSDeviceError::Validation(ref cause) => cause,
            RegisterAVSDeviceError::Credentials(ref err) => err.description(),
            RegisterAVSDeviceError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            RegisterAVSDeviceError::ParseError(ref cause) => cause,
            RegisterAVSDeviceError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by RejectSkill
#[derive(Debug, PartialEq)]
pub enum RejectSkillError {
    /// <p>Concurrent modification of resources. HTTP Status Code: 400.</p>
    ConcurrentModification(String),
    /// <p>The resource is not found.</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl RejectSkillError {
    pub fn from_response(res: BufferedHttpResponse) -> RejectSkillError {
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
                    return RejectSkillError::ConcurrentModification(String::from(error_message));
                }
                "NotFoundException" => {
                    return RejectSkillError::NotFound(String::from(error_message));
                }
                "ValidationException" => {
                    return RejectSkillError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return RejectSkillError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for RejectSkillError {
    fn from(err: serde_json::error::Error) -> RejectSkillError {
        RejectSkillError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for RejectSkillError {
    fn from(err: CredentialsError) -> RejectSkillError {
        RejectSkillError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RejectSkillError {
    fn from(err: HttpDispatchError) -> RejectSkillError {
        RejectSkillError::HttpDispatch(err)
    }
}
impl From<io::Error> for RejectSkillError {
    fn from(err: io::Error) -> RejectSkillError {
        RejectSkillError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RejectSkillError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RejectSkillError {
    fn description(&self) -> &str {
        match *self {
            RejectSkillError::ConcurrentModification(ref cause) => cause,
            RejectSkillError::NotFound(ref cause) => cause,
            RejectSkillError::Validation(ref cause) => cause,
            RejectSkillError::Credentials(ref err) => err.description(),
            RejectSkillError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            RejectSkillError::ParseError(ref cause) => cause,
            RejectSkillError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ResolveRoom
#[derive(Debug, PartialEq)]
pub enum ResolveRoomError {
    /// <p>The resource is not found.</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl ResolveRoomError {
    pub fn from_response(res: BufferedHttpResponse) -> ResolveRoomError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "NotFoundException" => {
                    return ResolveRoomError::NotFound(String::from(error_message));
                }
                "ValidationException" => {
                    return ResolveRoomError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return ResolveRoomError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ResolveRoomError {
    fn from(err: serde_json::error::Error) -> ResolveRoomError {
        ResolveRoomError::ParseError(err.description().to_string())
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
            ResolveRoomError::ParseError(ref cause) => cause,
            ResolveRoomError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by RevokeInvitation
#[derive(Debug, PartialEq)]
pub enum RevokeInvitationError {
    /// <p>Concurrent modification of resources. HTTP Status Code: 400.</p>
    ConcurrentModification(String),
    /// <p>The resource is not found.</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl RevokeInvitationError {
    pub fn from_response(res: BufferedHttpResponse) -> RevokeInvitationError {
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
                    return RevokeInvitationError::ConcurrentModification(String::from(
                        error_message,
                    ));
                }
                "NotFoundException" => {
                    return RevokeInvitationError::NotFound(String::from(error_message));
                }
                "ValidationException" => {
                    return RevokeInvitationError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return RevokeInvitationError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for RevokeInvitationError {
    fn from(err: serde_json::error::Error) -> RevokeInvitationError {
        RevokeInvitationError::ParseError(err.description().to_string())
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
            RevokeInvitationError::ConcurrentModification(ref cause) => cause,
            RevokeInvitationError::NotFound(ref cause) => cause,
            RevokeInvitationError::Validation(ref cause) => cause,
            RevokeInvitationError::Credentials(ref err) => err.description(),
            RevokeInvitationError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            RevokeInvitationError::ParseError(ref cause) => cause,
            RevokeInvitationError::Unknown(_) => "unknown error",
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
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl SearchAddressBooksError {
    pub fn from_response(res: BufferedHttpResponse) -> SearchAddressBooksError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ValidationException" => {
                    return SearchAddressBooksError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return SearchAddressBooksError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for SearchAddressBooksError {
    fn from(err: serde_json::error::Error) -> SearchAddressBooksError {
        SearchAddressBooksError::ParseError(err.description().to_string())
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
            SearchAddressBooksError::ParseError(ref cause) => cause,
            SearchAddressBooksError::Unknown(_) => "unknown error",
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
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl SearchContactsError {
    pub fn from_response(res: BufferedHttpResponse) -> SearchContactsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ValidationException" => {
                    return SearchContactsError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return SearchContactsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for SearchContactsError {
    fn from(err: serde_json::error::Error) -> SearchContactsError {
        SearchContactsError::ParseError(err.description().to_string())
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
            SearchContactsError::ParseError(ref cause) => cause,
            SearchContactsError::Unknown(_) => "unknown error",
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
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl SearchDevicesError {
    pub fn from_response(res: BufferedHttpResponse) -> SearchDevicesError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ValidationException" => {
                    return SearchDevicesError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return SearchDevicesError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for SearchDevicesError {
    fn from(err: serde_json::error::Error) -> SearchDevicesError {
        SearchDevicesError::ParseError(err.description().to_string())
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
            SearchDevicesError::ParseError(ref cause) => cause,
            SearchDevicesError::Unknown(_) => "unknown error",
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
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl SearchProfilesError {
    pub fn from_response(res: BufferedHttpResponse) -> SearchProfilesError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ValidationException" => {
                    return SearchProfilesError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return SearchProfilesError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for SearchProfilesError {
    fn from(err: serde_json::error::Error) -> SearchProfilesError {
        SearchProfilesError::ParseError(err.description().to_string())
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
            SearchProfilesError::ParseError(ref cause) => cause,
            SearchProfilesError::Unknown(_) => "unknown error",
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
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl SearchRoomsError {
    pub fn from_response(res: BufferedHttpResponse) -> SearchRoomsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ValidationException" => {
                    return SearchRoomsError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return SearchRoomsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for SearchRoomsError {
    fn from(err: serde_json::error::Error) -> SearchRoomsError {
        SearchRoomsError::ParseError(err.description().to_string())
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
            SearchRoomsError::ParseError(ref cause) => cause,
            SearchRoomsError::Unknown(_) => "unknown error",
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
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl SearchSkillGroupsError {
    pub fn from_response(res: BufferedHttpResponse) -> SearchSkillGroupsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ValidationException" => {
                    return SearchSkillGroupsError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return SearchSkillGroupsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for SearchSkillGroupsError {
    fn from(err: serde_json::error::Error) -> SearchSkillGroupsError {
        SearchSkillGroupsError::ParseError(err.description().to_string())
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
            SearchSkillGroupsError::ParseError(ref cause) => cause,
            SearchSkillGroupsError::Unknown(_) => "unknown error",
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
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl SearchUsersError {
    pub fn from_response(res: BufferedHttpResponse) -> SearchUsersError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ValidationException" => {
                    return SearchUsersError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return SearchUsersError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for SearchUsersError {
    fn from(err: serde_json::error::Error) -> SearchUsersError {
        SearchUsersError::ParseError(err.description().to_string())
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
            SearchUsersError::ParseError(ref cause) => cause,
            SearchUsersError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by SendInvitation
#[derive(Debug, PartialEq)]
pub enum SendInvitationError {
    /// <p>Concurrent modification of resources. HTTP Status Code: 400.</p>
    ConcurrentModification(String),
    /// <p>The attempt to update a user is invalid due to the user's current status.</p>
    InvalidUserStatus(String),
    /// <p>The resource is not found.</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl SendInvitationError {
    pub fn from_response(res: BufferedHttpResponse) -> SendInvitationError {
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
                    return SendInvitationError::ConcurrentModification(String::from(error_message));
                }
                "InvalidUserStatusException" => {
                    return SendInvitationError::InvalidUserStatus(String::from(error_message));
                }
                "NotFoundException" => {
                    return SendInvitationError::NotFound(String::from(error_message));
                }
                "ValidationException" => {
                    return SendInvitationError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return SendInvitationError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for SendInvitationError {
    fn from(err: serde_json::error::Error) -> SendInvitationError {
        SendInvitationError::ParseError(err.description().to_string())
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
            SendInvitationError::ConcurrentModification(ref cause) => cause,
            SendInvitationError::InvalidUserStatus(ref cause) => cause,
            SendInvitationError::NotFound(ref cause) => cause,
            SendInvitationError::Validation(ref cause) => cause,
            SendInvitationError::Credentials(ref err) => err.description(),
            SendInvitationError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            SendInvitationError::ParseError(ref cause) => cause,
            SendInvitationError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by StartDeviceSync
#[derive(Debug, PartialEq)]
pub enum StartDeviceSyncError {
    /// <p>The request failed because this device is no longer registered and therefore no longer managed by this account.</p>
    DeviceNotRegistered(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl StartDeviceSyncError {
    pub fn from_response(res: BufferedHttpResponse) -> StartDeviceSyncError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DeviceNotRegisteredException" => {
                    return StartDeviceSyncError::DeviceNotRegistered(String::from(error_message));
                }
                "ValidationException" => {
                    return StartDeviceSyncError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return StartDeviceSyncError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for StartDeviceSyncError {
    fn from(err: serde_json::error::Error) -> StartDeviceSyncError {
        StartDeviceSyncError::ParseError(err.description().to_string())
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
            StartDeviceSyncError::DeviceNotRegistered(ref cause) => cause,
            StartDeviceSyncError::Validation(ref cause) => cause,
            StartDeviceSyncError::Credentials(ref err) => err.description(),
            StartDeviceSyncError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            StartDeviceSyncError::ParseError(ref cause) => cause,
            StartDeviceSyncError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by StartSmartHomeApplianceDiscovery
#[derive(Debug, PartialEq)]
pub enum StartSmartHomeApplianceDiscoveryError {
    /// <p>The resource is not found.</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl StartSmartHomeApplianceDiscoveryError {
    pub fn from_response(res: BufferedHttpResponse) -> StartSmartHomeApplianceDiscoveryError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "NotFoundException" => {
                    return StartSmartHomeApplianceDiscoveryError::NotFound(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return StartSmartHomeApplianceDiscoveryError::Validation(
                        error_message.to_string(),
                    );
                }
                _ => {}
            }
        }
        return StartSmartHomeApplianceDiscoveryError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for StartSmartHomeApplianceDiscoveryError {
    fn from(err: serde_json::error::Error) -> StartSmartHomeApplianceDiscoveryError {
        StartSmartHomeApplianceDiscoveryError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for StartSmartHomeApplianceDiscoveryError {
    fn from(err: CredentialsError) -> StartSmartHomeApplianceDiscoveryError {
        StartSmartHomeApplianceDiscoveryError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StartSmartHomeApplianceDiscoveryError {
    fn from(err: HttpDispatchError) -> StartSmartHomeApplianceDiscoveryError {
        StartSmartHomeApplianceDiscoveryError::HttpDispatch(err)
    }
}
impl From<io::Error> for StartSmartHomeApplianceDiscoveryError {
    fn from(err: io::Error) -> StartSmartHomeApplianceDiscoveryError {
        StartSmartHomeApplianceDiscoveryError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for StartSmartHomeApplianceDiscoveryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartSmartHomeApplianceDiscoveryError {
    fn description(&self) -> &str {
        match *self {
            StartSmartHomeApplianceDiscoveryError::NotFound(ref cause) => cause,
            StartSmartHomeApplianceDiscoveryError::Validation(ref cause) => cause,
            StartSmartHomeApplianceDiscoveryError::Credentials(ref err) => err.description(),
            StartSmartHomeApplianceDiscoveryError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            StartSmartHomeApplianceDiscoveryError::ParseError(ref cause) => cause,
            StartSmartHomeApplianceDiscoveryError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>The resource is not found.</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> TagResourceError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "NotFoundException" => {
                    return TagResourceError::NotFound(String::from(error_message));
                }
                "ValidationException" => {
                    return TagResourceError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return TagResourceError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for TagResourceError {
    fn from(err: serde_json::error::Error) -> TagResourceError {
        TagResourceError::ParseError(err.description().to_string())
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
            TagResourceError::ParseError(ref cause) => cause,
            TagResourceError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>The resource is not found.</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> UntagResourceError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "NotFoundException" => {
                    return UntagResourceError::NotFound(String::from(error_message));
                }
                "ValidationException" => {
                    return UntagResourceError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return UntagResourceError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UntagResourceError {
    fn from(err: serde_json::error::Error) -> UntagResourceError {
        UntagResourceError::ParseError(err.description().to_string())
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
            UntagResourceError::ParseError(ref cause) => cause,
            UntagResourceError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by UpdateAddressBook
#[derive(Debug, PartialEq)]
pub enum UpdateAddressBookError {
    /// <p>Concurrent modification of resources. HTTP Status Code: 400.</p>
    ConcurrentModification(String),
    /// <p>The name sent in the request is already in use.</p>
    NameInUse(String),
    /// <p>The resource is not found.</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl UpdateAddressBookError {
    pub fn from_response(res: BufferedHttpResponse) -> UpdateAddressBookError {
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
                    return UpdateAddressBookError::ConcurrentModification(String::from(
                        error_message,
                    ));
                }
                "NameInUseException" => {
                    return UpdateAddressBookError::NameInUse(String::from(error_message));
                }
                "NotFoundException" => {
                    return UpdateAddressBookError::NotFound(String::from(error_message));
                }
                "ValidationException" => {
                    return UpdateAddressBookError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return UpdateAddressBookError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UpdateAddressBookError {
    fn from(err: serde_json::error::Error) -> UpdateAddressBookError {
        UpdateAddressBookError::ParseError(err.description().to_string())
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
            UpdateAddressBookError::ConcurrentModification(ref cause) => cause,
            UpdateAddressBookError::NameInUse(ref cause) => cause,
            UpdateAddressBookError::NotFound(ref cause) => cause,
            UpdateAddressBookError::Validation(ref cause) => cause,
            UpdateAddressBookError::Credentials(ref err) => err.description(),
            UpdateAddressBookError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateAddressBookError::ParseError(ref cause) => cause,
            UpdateAddressBookError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by UpdateBusinessReportSchedule
#[derive(Debug, PartialEq)]
pub enum UpdateBusinessReportScheduleError {
    /// <p>Concurrent modification of resources. HTTP Status Code: 400.</p>
    ConcurrentModification(String),
    /// <p>The resource is not found.</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl UpdateBusinessReportScheduleError {
    pub fn from_response(res: BufferedHttpResponse) -> UpdateBusinessReportScheduleError {
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
                    return UpdateBusinessReportScheduleError::ConcurrentModification(String::from(
                        error_message,
                    ));
                }
                "NotFoundException" => {
                    return UpdateBusinessReportScheduleError::NotFound(String::from(error_message));
                }
                "ValidationException" => {
                    return UpdateBusinessReportScheduleError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return UpdateBusinessReportScheduleError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UpdateBusinessReportScheduleError {
    fn from(err: serde_json::error::Error) -> UpdateBusinessReportScheduleError {
        UpdateBusinessReportScheduleError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateBusinessReportScheduleError {
    fn from(err: CredentialsError) -> UpdateBusinessReportScheduleError {
        UpdateBusinessReportScheduleError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateBusinessReportScheduleError {
    fn from(err: HttpDispatchError) -> UpdateBusinessReportScheduleError {
        UpdateBusinessReportScheduleError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateBusinessReportScheduleError {
    fn from(err: io::Error) -> UpdateBusinessReportScheduleError {
        UpdateBusinessReportScheduleError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateBusinessReportScheduleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateBusinessReportScheduleError {
    fn description(&self) -> &str {
        match *self {
            UpdateBusinessReportScheduleError::ConcurrentModification(ref cause) => cause,
            UpdateBusinessReportScheduleError::NotFound(ref cause) => cause,
            UpdateBusinessReportScheduleError::Validation(ref cause) => cause,
            UpdateBusinessReportScheduleError::Credentials(ref err) => err.description(),
            UpdateBusinessReportScheduleError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateBusinessReportScheduleError::ParseError(ref cause) => cause,
            UpdateBusinessReportScheduleError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by UpdateConferenceProvider
#[derive(Debug, PartialEq)]
pub enum UpdateConferenceProviderError {
    /// <p>The resource is not found.</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl UpdateConferenceProviderError {
    pub fn from_response(res: BufferedHttpResponse) -> UpdateConferenceProviderError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "NotFoundException" => {
                    return UpdateConferenceProviderError::NotFound(String::from(error_message));
                }
                "ValidationException" => {
                    return UpdateConferenceProviderError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return UpdateConferenceProviderError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UpdateConferenceProviderError {
    fn from(err: serde_json::error::Error) -> UpdateConferenceProviderError {
        UpdateConferenceProviderError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateConferenceProviderError {
    fn from(err: CredentialsError) -> UpdateConferenceProviderError {
        UpdateConferenceProviderError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateConferenceProviderError {
    fn from(err: HttpDispatchError) -> UpdateConferenceProviderError {
        UpdateConferenceProviderError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateConferenceProviderError {
    fn from(err: io::Error) -> UpdateConferenceProviderError {
        UpdateConferenceProviderError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateConferenceProviderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateConferenceProviderError {
    fn description(&self) -> &str {
        match *self {
            UpdateConferenceProviderError::NotFound(ref cause) => cause,
            UpdateConferenceProviderError::Validation(ref cause) => cause,
            UpdateConferenceProviderError::Credentials(ref err) => err.description(),
            UpdateConferenceProviderError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateConferenceProviderError::ParseError(ref cause) => cause,
            UpdateConferenceProviderError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by UpdateContact
#[derive(Debug, PartialEq)]
pub enum UpdateContactError {
    /// <p>Concurrent modification of resources. HTTP Status Code: 400.</p>
    ConcurrentModification(String),
    /// <p>The resource is not found.</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl UpdateContactError {
    pub fn from_response(res: BufferedHttpResponse) -> UpdateContactError {
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
                    return UpdateContactError::ConcurrentModification(String::from(error_message));
                }
                "NotFoundException" => {
                    return UpdateContactError::NotFound(String::from(error_message));
                }
                "ValidationException" => {
                    return UpdateContactError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return UpdateContactError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UpdateContactError {
    fn from(err: serde_json::error::Error) -> UpdateContactError {
        UpdateContactError::ParseError(err.description().to_string())
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
            UpdateContactError::ConcurrentModification(ref cause) => cause,
            UpdateContactError::NotFound(ref cause) => cause,
            UpdateContactError::Validation(ref cause) => cause,
            UpdateContactError::Credentials(ref err) => err.description(),
            UpdateContactError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateContactError::ParseError(ref cause) => cause,
            UpdateContactError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by UpdateDevice
#[derive(Debug, PartialEq)]
pub enum UpdateDeviceError {
    /// <p>Concurrent modification of resources. HTTP Status Code: 400.</p>
    ConcurrentModification(String),
    /// <p>The request failed because this device is no longer registered and therefore no longer managed by this account.</p>
    DeviceNotRegistered(String),
    /// <p>The resource is not found.</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl UpdateDeviceError {
    pub fn from_response(res: BufferedHttpResponse) -> UpdateDeviceError {
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
                    return UpdateDeviceError::ConcurrentModification(String::from(error_message));
                }
                "DeviceNotRegisteredException" => {
                    return UpdateDeviceError::DeviceNotRegistered(String::from(error_message));
                }
                "NotFoundException" => {
                    return UpdateDeviceError::NotFound(String::from(error_message));
                }
                "ValidationException" => {
                    return UpdateDeviceError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return UpdateDeviceError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UpdateDeviceError {
    fn from(err: serde_json::error::Error) -> UpdateDeviceError {
        UpdateDeviceError::ParseError(err.description().to_string())
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
            UpdateDeviceError::ConcurrentModification(ref cause) => cause,
            UpdateDeviceError::DeviceNotRegistered(ref cause) => cause,
            UpdateDeviceError::NotFound(ref cause) => cause,
            UpdateDeviceError::Validation(ref cause) => cause,
            UpdateDeviceError::Credentials(ref err) => err.description(),
            UpdateDeviceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateDeviceError::ParseError(ref cause) => cause,
            UpdateDeviceError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by UpdateProfile
#[derive(Debug, PartialEq)]
pub enum UpdateProfileError {
    /// <p>Concurrent modification of resources. HTTP Status Code: 400.</p>
    ConcurrentModification(String),
    /// <p>The name sent in the request is already in use.</p>
    NameInUse(String),
    /// <p>The resource is not found.</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl UpdateProfileError {
    pub fn from_response(res: BufferedHttpResponse) -> UpdateProfileError {
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
                    return UpdateProfileError::ConcurrentModification(String::from(error_message));
                }
                "NameInUseException" => {
                    return UpdateProfileError::NameInUse(String::from(error_message));
                }
                "NotFoundException" => {
                    return UpdateProfileError::NotFound(String::from(error_message));
                }
                "ValidationException" => {
                    return UpdateProfileError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return UpdateProfileError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UpdateProfileError {
    fn from(err: serde_json::error::Error) -> UpdateProfileError {
        UpdateProfileError::ParseError(err.description().to_string())
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
            UpdateProfileError::ConcurrentModification(ref cause) => cause,
            UpdateProfileError::NameInUse(ref cause) => cause,
            UpdateProfileError::NotFound(ref cause) => cause,
            UpdateProfileError::Validation(ref cause) => cause,
            UpdateProfileError::Credentials(ref err) => err.description(),
            UpdateProfileError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateProfileError::ParseError(ref cause) => cause,
            UpdateProfileError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by UpdateRoom
#[derive(Debug, PartialEq)]
pub enum UpdateRoomError {
    /// <p>The name sent in the request is already in use.</p>
    NameInUse(String),
    /// <p>The resource is not found.</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl UpdateRoomError {
    pub fn from_response(res: BufferedHttpResponse) -> UpdateRoomError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "NameInUseException" => {
                    return UpdateRoomError::NameInUse(String::from(error_message));
                }
                "NotFoundException" => {
                    return UpdateRoomError::NotFound(String::from(error_message));
                }
                "ValidationException" => {
                    return UpdateRoomError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return UpdateRoomError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UpdateRoomError {
    fn from(err: serde_json::error::Error) -> UpdateRoomError {
        UpdateRoomError::ParseError(err.description().to_string())
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
            UpdateRoomError::ParseError(ref cause) => cause,
            UpdateRoomError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by UpdateSkillGroup
#[derive(Debug, PartialEq)]
pub enum UpdateSkillGroupError {
    /// <p>Concurrent modification of resources. HTTP Status Code: 400.</p>
    ConcurrentModification(String),
    /// <p>The name sent in the request is already in use.</p>
    NameInUse(String),
    /// <p>The resource is not found.</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl UpdateSkillGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> UpdateSkillGroupError {
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
                    return UpdateSkillGroupError::ConcurrentModification(String::from(
                        error_message,
                    ));
                }
                "NameInUseException" => {
                    return UpdateSkillGroupError::NameInUse(String::from(error_message));
                }
                "NotFoundException" => {
                    return UpdateSkillGroupError::NotFound(String::from(error_message));
                }
                "ValidationException" => {
                    return UpdateSkillGroupError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return UpdateSkillGroupError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UpdateSkillGroupError {
    fn from(err: serde_json::error::Error) -> UpdateSkillGroupError {
        UpdateSkillGroupError::ParseError(err.description().to_string())
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
            UpdateSkillGroupError::ConcurrentModification(ref cause) => cause,
            UpdateSkillGroupError::NameInUse(ref cause) => cause,
            UpdateSkillGroupError::NotFound(ref cause) => cause,
            UpdateSkillGroupError::Validation(ref cause) => cause,
            UpdateSkillGroupError::Credentials(ref err) => err.description(),
            UpdateSkillGroupError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateSkillGroupError::ParseError(ref cause) => cause,
            UpdateSkillGroupError::Unknown(_) => "unknown error",
        }
    }
}
/// Trait representing the capabilities of the Alexa For Business API. Alexa For Business clients implement this trait.
pub trait AlexaForBusiness {
    /// <p>Associates a skill with the organization under the customer's AWS account. If a skill is private, the user implicitly accepts access to this skill during enablement.</p>
    fn approve_skill(
        &self,
        input: ApproveSkillRequest,
    ) -> RusotoFuture<ApproveSkillResponse, ApproveSkillError>;

    /// <p>Associates a contact with a given address book.</p>
    fn associate_contact_with_address_book(
        &self,
        input: AssociateContactWithAddressBookRequest,
    ) -> RusotoFuture<AssociateContactWithAddressBookResponse, AssociateContactWithAddressBookError>;

    /// <p>Associates a device with a given room. This applies all the settings from the room profile to the device, and all the skills in any skill groups added to that room. This operation requires the device to be online, or else a manual sync is required. </p>
    fn associate_device_with_room(
        &self,
        input: AssociateDeviceWithRoomRequest,
    ) -> RusotoFuture<AssociateDeviceWithRoomResponse, AssociateDeviceWithRoomError>;

    /// <p>Associates a skill group with a given room. This enables all skills in the associated skill group on all devices in the room.</p>
    fn associate_skill_group_with_room(
        &self,
        input: AssociateSkillGroupWithRoomRequest,
    ) -> RusotoFuture<AssociateSkillGroupWithRoomResponse, AssociateSkillGroupWithRoomError>;

    /// <p>Associates a skill with a skill group.</p>
    fn associate_skill_with_skill_group(
        &self,
        input: AssociateSkillWithSkillGroupRequest,
    ) -> RusotoFuture<AssociateSkillWithSkillGroupResponse, AssociateSkillWithSkillGroupError>;

    /// <p>Makes a private skill available for enrolled users to enable on their devices.</p>
    fn associate_skill_with_users(
        &self,
        input: AssociateSkillWithUsersRequest,
    ) -> RusotoFuture<AssociateSkillWithUsersResponse, AssociateSkillWithUsersError>;

    /// <p>Creates an address book with the specified details.</p>
    fn create_address_book(
        &self,
        input: CreateAddressBookRequest,
    ) -> RusotoFuture<CreateAddressBookResponse, CreateAddressBookError>;

    /// <p>Creates a recurring schedule for usage reports to deliver to the specified S3 location with a specified daily or weekly interval.</p>
    fn create_business_report_schedule(
        &self,
        input: CreateBusinessReportScheduleRequest,
    ) -> RusotoFuture<CreateBusinessReportScheduleResponse, CreateBusinessReportScheduleError>;

    /// <p>Adds a new conference provider under the user's AWS account.</p>
    fn create_conference_provider(
        &self,
        input: CreateConferenceProviderRequest,
    ) -> RusotoFuture<CreateConferenceProviderResponse, CreateConferenceProviderError>;

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

    /// <p>Deletes the recurring report delivery schedule with the specified schedule ARN.</p>
    fn delete_business_report_schedule(
        &self,
        input: DeleteBusinessReportScheduleRequest,
    ) -> RusotoFuture<DeleteBusinessReportScheduleResponse, DeleteBusinessReportScheduleError>;

    /// <p>Deletes a conference provider.</p>
    fn delete_conference_provider(
        &self,
        input: DeleteConferenceProviderRequest,
    ) -> RusotoFuture<DeleteConferenceProviderResponse, DeleteConferenceProviderError>;

    /// <p>Deletes a contact by the contact ARN.</p>
    fn delete_contact(
        &self,
        input: DeleteContactRequest,
    ) -> RusotoFuture<DeleteContactResponse, DeleteContactError>;

    /// <p>Removes a device from Alexa For Business.</p>
    fn delete_device(
        &self,
        input: DeleteDeviceRequest,
    ) -> RusotoFuture<DeleteDeviceResponse, DeleteDeviceError>;

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

    /// <p>Unlinks a third-party account from a skill.</p>
    fn delete_skill_authorization(
        &self,
        input: DeleteSkillAuthorizationRequest,
    ) -> RusotoFuture<DeleteSkillAuthorizationResponse, DeleteSkillAuthorizationError>;

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

    /// <p>Disassociates a skill from a skill group.</p>
    fn disassociate_skill_from_skill_group(
        &self,
        input: DisassociateSkillFromSkillGroupRequest,
    ) -> RusotoFuture<DisassociateSkillFromSkillGroupResponse, DisassociateSkillFromSkillGroupError>;

    /// <p>Makes a private skill unavailable for enrolled users and prevents them from enabling it on their devices.</p>
    fn disassociate_skill_from_users(
        &self,
        input: DisassociateSkillFromUsersRequest,
    ) -> RusotoFuture<DisassociateSkillFromUsersResponse, DisassociateSkillFromUsersError>;

    /// <p>Disassociates a skill group from a specified room. This disables all skills in the skill group on all devices in the room.</p>
    fn disassociate_skill_group_from_room(
        &self,
        input: DisassociateSkillGroupFromRoomRequest,
    ) -> RusotoFuture<DisassociateSkillGroupFromRoomResponse, DisassociateSkillGroupFromRoomError>;

    /// <p>Forgets smart home appliances associated to a room.</p>
    fn forget_smart_home_appliances(
        &self,
        input: ForgetSmartHomeAppliancesRequest,
    ) -> RusotoFuture<ForgetSmartHomeAppliancesResponse, ForgetSmartHomeAppliancesError>;

    /// <p>Gets address the book details by the address book ARN.</p>
    fn get_address_book(
        &self,
        input: GetAddressBookRequest,
    ) -> RusotoFuture<GetAddressBookResponse, GetAddressBookError>;

    /// <p>Retrieves the existing conference preferences.</p>
    fn get_conference_preference(
        &self,
    ) -> RusotoFuture<GetConferencePreferenceResponse, GetConferencePreferenceError>;

    /// <p>Gets details about a specific conference provider.</p>
    fn get_conference_provider(
        &self,
        input: GetConferenceProviderRequest,
    ) -> RusotoFuture<GetConferenceProviderResponse, GetConferenceProviderError>;

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

    /// <p>Lists the details of the schedules that a user configured.</p>
    fn list_business_report_schedules(
        &self,
        input: ListBusinessReportSchedulesRequest,
    ) -> RusotoFuture<ListBusinessReportSchedulesResponse, ListBusinessReportSchedulesError>;

    /// <p>Lists conference providers under a specific AWS account.</p>
    fn list_conference_providers(
        &self,
        input: ListConferenceProvidersRequest,
    ) -> RusotoFuture<ListConferenceProvidersResponse, ListConferenceProvidersError>;

    /// <p>Lists the device event history, including device connection status, for up to 30 days.</p>
    fn list_device_events(
        &self,
        input: ListDeviceEventsRequest,
    ) -> RusotoFuture<ListDeviceEventsResponse, ListDeviceEventsError>;

    /// <p>Lists all enabled skills in a specific skill group.</p>
    fn list_skills(
        &self,
        input: ListSkillsRequest,
    ) -> RusotoFuture<ListSkillsResponse, ListSkillsError>;

    /// <p>Lists all categories in the Alexa skill store.</p>
    fn list_skills_store_categories(
        &self,
        input: ListSkillsStoreCategoriesRequest,
    ) -> RusotoFuture<ListSkillsStoreCategoriesResponse, ListSkillsStoreCategoriesError>;

    /// <p>Lists all skills in the Alexa skill store by category.</p>
    fn list_skills_store_skills_by_category(
        &self,
        input: ListSkillsStoreSkillsByCategoryRequest,
    ) -> RusotoFuture<ListSkillsStoreSkillsByCategoryResponse, ListSkillsStoreSkillsByCategoryError>;

    /// <p>Lists all of the smart home appliances associated with a room.</p>
    fn list_smart_home_appliances(
        &self,
        input: ListSmartHomeAppliancesRequest,
    ) -> RusotoFuture<ListSmartHomeAppliancesResponse, ListSmartHomeAppliancesError>;

    /// <p>Lists all tags for the specified resource.</p>
    fn list_tags(&self, input: ListTagsRequest) -> RusotoFuture<ListTagsResponse, ListTagsError>;

    /// <p>Sets the conference preferences on a specific conference provider at the account level.</p>
    fn put_conference_preference(
        &self,
        input: PutConferencePreferenceRequest,
    ) -> RusotoFuture<PutConferencePreferenceResponse, PutConferencePreferenceError>;

    /// <p>Updates room skill parameter details by room, skill, and parameter key ID. Not all skills have a room skill parameter.</p>
    fn put_room_skill_parameter(
        &self,
        input: PutRoomSkillParameterRequest,
    ) -> RusotoFuture<PutRoomSkillParameterResponse, PutRoomSkillParameterError>;

    /// <p>Links a user's account to a third-party skill provider. If this API operation is called by an assumed IAM role, the skill being linked must be a private skill. Also, the skill must be owned by the AWS account that assumed the IAM role.</p>
    fn put_skill_authorization(
        &self,
        input: PutSkillAuthorizationRequest,
    ) -> RusotoFuture<PutSkillAuthorizationResponse, PutSkillAuthorizationError>;

    /// <p>Registers an Alexa-enabled device built by an Original Equipment Manufacturer (OEM) using Alexa Voice Service (AVS).</p>
    fn register_avs_device(
        &self,
        input: RegisterAVSDeviceRequest,
    ) -> RusotoFuture<RegisterAVSDeviceResponse, RegisterAVSDeviceError>;

    /// <p>Disassociates a skill from the organization under a user's AWS account. If the skill is a private skill, it moves to an AcceptStatus of PENDING. Any private or public skill that is rejected can be added later by calling the ApproveSkill API. </p>
    fn reject_skill(
        &self,
        input: RejectSkillRequest,
    ) -> RusotoFuture<RejectSkillResponse, RejectSkillError>;

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

    /// <p>Resets a device and its account to the known default settings, by clearing all information and settings set by previous users.</p>
    fn start_device_sync(
        &self,
        input: StartDeviceSyncRequest,
    ) -> RusotoFuture<StartDeviceSyncResponse, StartDeviceSyncError>;

    /// <p>Initiates the discovery of any smart home appliances associated with the room.</p>
    fn start_smart_home_appliance_discovery(
        &self,
        input: StartSmartHomeApplianceDiscoveryRequest,
    ) -> RusotoFuture<StartSmartHomeApplianceDiscoveryResponse, StartSmartHomeApplianceDiscoveryError>;

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

    /// <p>Updates the configuration of the report delivery schedule with the specified schedule ARN.</p>
    fn update_business_report_schedule(
        &self,
        input: UpdateBusinessReportScheduleRequest,
    ) -> RusotoFuture<UpdateBusinessReportScheduleResponse, UpdateBusinessReportScheduleError>;

    /// <p>Updates an existing conference provider's settings.</p>
    fn update_conference_provider(
        &self,
        input: UpdateConferenceProviderRequest,
    ) -> RusotoFuture<UpdateConferenceProviderResponse, UpdateConferenceProviderError>;

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
#[derive(Clone)]
pub struct AlexaForBusinessClient {
    client: Client,
    region: region::Region,
}

impl AlexaForBusinessClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> AlexaForBusinessClient {
        AlexaForBusinessClient {
            client: Client::shared(),
            region: region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> AlexaForBusinessClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        AlexaForBusinessClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl AlexaForBusiness for AlexaForBusinessClient {
    /// <p>Associates a skill with the organization under the customer's AWS account. If a skill is private, the user implicitly accepts access to this skill during enablement.</p>
    fn approve_skill(
        &self,
        input: ApproveSkillRequest,
    ) -> RusotoFuture<ApproveSkillResponse, ApproveSkillError> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.ApproveSkill");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ApproveSkillResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ApproveSkillError::from_response(response))),
                )
            }
        })
    }

    /// <p>Associates a contact with a given address book.</p>
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

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<AssociateContactWithAddressBookResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(AssociateContactWithAddressBookError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Associates a device with a given room. This applies all the settings from the room profile to the device, and all the skills in any skill groups added to that room. This operation requires the device to be online, or else a manual sync is required. </p>
    fn associate_device_with_room(
        &self,
        input: AssociateDeviceWithRoomRequest,
    ) -> RusotoFuture<AssociateDeviceWithRoomResponse, AssociateDeviceWithRoomError> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.AssociateDeviceWithRoom");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<AssociateDeviceWithRoomResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(AssociateDeviceWithRoomError::from_response(response))
                }))
            }
        })
    }

    /// <p>Associates a skill group with a given room. This enables all skills in the associated skill group on all devices in the room.</p>
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

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<AssociateSkillGroupWithRoomResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(AssociateSkillGroupWithRoomError::from_response(response))
                }))
            }
        })
    }

    /// <p>Associates a skill with a skill group.</p>
    fn associate_skill_with_skill_group(
        &self,
        input: AssociateSkillWithSkillGroupRequest,
    ) -> RusotoFuture<AssociateSkillWithSkillGroupResponse, AssociateSkillWithSkillGroupError> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AlexaForBusiness.AssociateSkillWithSkillGroup",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<AssociateSkillWithSkillGroupResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(AssociateSkillWithSkillGroupError::from_response(response))
                }))
            }
        })
    }

    /// <p>Makes a private skill available for enrolled users to enable on their devices.</p>
    fn associate_skill_with_users(
        &self,
        input: AssociateSkillWithUsersRequest,
    ) -> RusotoFuture<AssociateSkillWithUsersResponse, AssociateSkillWithUsersError> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.AssociateSkillWithUsers");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<AssociateSkillWithUsersResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(AssociateSkillWithUsersError::from_response(response))
                }))
            }
        })
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

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateAddressBookResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateAddressBookError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates a recurring schedule for usage reports to deliver to the specified S3 location with a specified daily or weekly interval.</p>
    fn create_business_report_schedule(
        &self,
        input: CreateBusinessReportScheduleRequest,
    ) -> RusotoFuture<CreateBusinessReportScheduleResponse, CreateBusinessReportScheduleError> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AlexaForBusiness.CreateBusinessReportSchedule",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateBusinessReportScheduleResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateBusinessReportScheduleError::from_response(response))
                }))
            }
        })
    }

    /// <p>Adds a new conference provider under the user's AWS account.</p>
    fn create_conference_provider(
        &self,
        input: CreateConferenceProviderRequest,
    ) -> RusotoFuture<CreateConferenceProviderResponse, CreateConferenceProviderError> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.CreateConferenceProvider");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateConferenceProviderResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateConferenceProviderError::from_response(response))
                }))
            }
        })
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

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateContactResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateContactError::from_response(response))),
                )
            }
        })
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

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateProfileResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateProfileError::from_response(response))),
                )
            }
        })
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

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateRoomResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
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

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateSkillGroupResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateSkillGroupError::from_response(response))),
                )
            }
        })
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

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateUserResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateUserError::from_response(response))),
                )
            }
        })
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

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteAddressBookResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteAddressBookError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes the recurring report delivery schedule with the specified schedule ARN.</p>
    fn delete_business_report_schedule(
        &self,
        input: DeleteBusinessReportScheduleRequest,
    ) -> RusotoFuture<DeleteBusinessReportScheduleResponse, DeleteBusinessReportScheduleError> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AlexaForBusiness.DeleteBusinessReportSchedule",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteBusinessReportScheduleResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteBusinessReportScheduleError::from_response(response))
                }))
            }
        })
    }

    /// <p>Deletes a conference provider.</p>
    fn delete_conference_provider(
        &self,
        input: DeleteConferenceProviderRequest,
    ) -> RusotoFuture<DeleteConferenceProviderResponse, DeleteConferenceProviderError> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.DeleteConferenceProvider");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteConferenceProviderResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteConferenceProviderError::from_response(response))
                }))
            }
        })
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

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteContactResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteContactError::from_response(response))),
                )
            }
        })
    }

    /// <p>Removes a device from Alexa For Business.</p>
    fn delete_device(
        &self,
        input: DeleteDeviceRequest,
    ) -> RusotoFuture<DeleteDeviceResponse, DeleteDeviceError> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.DeleteDevice");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteDeviceResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteDeviceError::from_response(response))),
                )
            }
        })
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

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteProfileResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteProfileError::from_response(response))),
                )
            }
        })
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

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteRoomResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
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

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteRoomSkillParameterResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteRoomSkillParameterError::from_response(response))
                }))
            }
        })
    }

    /// <p>Unlinks a third-party account from a skill.</p>
    fn delete_skill_authorization(
        &self,
        input: DeleteSkillAuthorizationRequest,
    ) -> RusotoFuture<DeleteSkillAuthorizationResponse, DeleteSkillAuthorizationError> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.DeleteSkillAuthorization");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteSkillAuthorizationResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteSkillAuthorizationError::from_response(response))
                }))
            }
        })
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

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteSkillGroupResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteSkillGroupError::from_response(response))),
                )
            }
        })
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

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteUserResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
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

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DisassociateContactFromAddressBookResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DisassociateContactFromAddressBookError::from_response(
                        response,
                    ))
                }))
            }
        })
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

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DisassociateDeviceFromRoomResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DisassociateDeviceFromRoomError::from_response(response))
                }))
            }
        })
    }

    /// <p>Disassociates a skill from a skill group.</p>
    fn disassociate_skill_from_skill_group(
        &self,
        input: DisassociateSkillFromSkillGroupRequest,
    ) -> RusotoFuture<DisassociateSkillFromSkillGroupResponse, DisassociateSkillFromSkillGroupError>
    {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AlexaForBusiness.DisassociateSkillFromSkillGroup",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DisassociateSkillFromSkillGroupResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DisassociateSkillFromSkillGroupError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Makes a private skill unavailable for enrolled users and prevents them from enabling it on their devices.</p>
    fn disassociate_skill_from_users(
        &self,
        input: DisassociateSkillFromUsersRequest,
    ) -> RusotoFuture<DisassociateSkillFromUsersResponse, DisassociateSkillFromUsersError> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AlexaForBusiness.DisassociateSkillFromUsers",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DisassociateSkillFromUsersResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DisassociateSkillFromUsersError::from_response(response))
                }))
            }
        })
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

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DisassociateSkillGroupFromRoomResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DisassociateSkillGroupFromRoomError::from_response(response))
                }))
            }
        })
    }

    /// <p>Forgets smart home appliances associated to a room.</p>
    fn forget_smart_home_appliances(
        &self,
        input: ForgetSmartHomeAppliancesRequest,
    ) -> RusotoFuture<ForgetSmartHomeAppliancesResponse, ForgetSmartHomeAppliancesError> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.ForgetSmartHomeAppliances");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ForgetSmartHomeAppliancesResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ForgetSmartHomeAppliancesError::from_response(response))
                }))
            }
        })
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

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetAddressBookResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetAddressBookError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves the existing conference preferences.</p>
    fn get_conference_preference(
        &self,
    ) -> RusotoFuture<GetConferencePreferenceResponse, GetConferencePreferenceError> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.GetConferencePreference");
        request.set_payload(Some(b"{}".to_vec()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetConferencePreferenceResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetConferencePreferenceError::from_response(response))
                }))
            }
        })
    }

    /// <p>Gets details about a specific conference provider.</p>
    fn get_conference_provider(
        &self,
        input: GetConferenceProviderRequest,
    ) -> RusotoFuture<GetConferenceProviderResponse, GetConferenceProviderError> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.GetConferenceProvider");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetConferenceProviderResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetConferenceProviderError::from_response(response))
                    }),
                )
            }
        })
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

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetContactResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetContactError::from_response(response))),
                )
            }
        })
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

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
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

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetProfileResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetProfileError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets room details by room ARN.</p>
    fn get_room(&self, input: GetRoomRequest) -> RusotoFuture<GetRoomResponse, GetRoomError> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.GetRoom");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetRoomResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
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

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetRoomSkillParameterResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetRoomSkillParameterError::from_response(response))
                    }),
                )
            }
        })
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

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetSkillGroupResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetSkillGroupError::from_response(response))),
                )
            }
        })
    }

    /// <p>Lists the details of the schedules that a user configured.</p>
    fn list_business_report_schedules(
        &self,
        input: ListBusinessReportSchedulesRequest,
    ) -> RusotoFuture<ListBusinessReportSchedulesResponse, ListBusinessReportSchedulesError> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AlexaForBusiness.ListBusinessReportSchedules",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListBusinessReportSchedulesResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListBusinessReportSchedulesError::from_response(response))
                }))
            }
        })
    }

    /// <p>Lists conference providers under a specific AWS account.</p>
    fn list_conference_providers(
        &self,
        input: ListConferenceProvidersRequest,
    ) -> RusotoFuture<ListConferenceProvidersResponse, ListConferenceProvidersError> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.ListConferenceProviders");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListConferenceProvidersResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListConferenceProvidersError::from_response(response))
                }))
            }
        })
    }

    /// <p>Lists the device event history, including device connection status, for up to 30 days.</p>
    fn list_device_events(
        &self,
        input: ListDeviceEventsRequest,
    ) -> RusotoFuture<ListDeviceEventsResponse, ListDeviceEventsError> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.ListDeviceEvents");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListDeviceEventsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListDeviceEventsError::from_response(response))),
                )
            }
        })
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

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListSkillsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListSkillsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Lists all categories in the Alexa skill store.</p>
    fn list_skills_store_categories(
        &self,
        input: ListSkillsStoreCategoriesRequest,
    ) -> RusotoFuture<ListSkillsStoreCategoriesResponse, ListSkillsStoreCategoriesError> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.ListSkillsStoreCategories");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListSkillsStoreCategoriesResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListSkillsStoreCategoriesError::from_response(response))
                }))
            }
        })
    }

    /// <p>Lists all skills in the Alexa skill store by category.</p>
    fn list_skills_store_skills_by_category(
        &self,
        input: ListSkillsStoreSkillsByCategoryRequest,
    ) -> RusotoFuture<ListSkillsStoreSkillsByCategoryResponse, ListSkillsStoreSkillsByCategoryError>
    {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AlexaForBusiness.ListSkillsStoreSkillsByCategory",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListSkillsStoreSkillsByCategoryResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListSkillsStoreSkillsByCategoryError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Lists all of the smart home appliances associated with a room.</p>
    fn list_smart_home_appliances(
        &self,
        input: ListSmartHomeAppliancesRequest,
    ) -> RusotoFuture<ListSmartHomeAppliancesResponse, ListSmartHomeAppliancesError> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.ListSmartHomeAppliances");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListSmartHomeAppliancesResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListSmartHomeAppliancesError::from_response(response))
                }))
            }
        })
    }

    /// <p>Lists all tags for the specified resource.</p>
    fn list_tags(&self, input: ListTagsRequest) -> RusotoFuture<ListTagsResponse, ListTagsError> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.ListTags");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListTagsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListTagsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Sets the conference preferences on a specific conference provider at the account level.</p>
    fn put_conference_preference(
        &self,
        input: PutConferencePreferenceRequest,
    ) -> RusotoFuture<PutConferencePreferenceResponse, PutConferencePreferenceError> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.PutConferencePreference");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<PutConferencePreferenceResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(PutConferencePreferenceError::from_response(response))
                }))
            }
        })
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

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<PutRoomSkillParameterResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(PutRoomSkillParameterError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Links a user's account to a third-party skill provider. If this API operation is called by an assumed IAM role, the skill being linked must be a private skill. Also, the skill must be owned by the AWS account that assumed the IAM role.</p>
    fn put_skill_authorization(
        &self,
        input: PutSkillAuthorizationRequest,
    ) -> RusotoFuture<PutSkillAuthorizationResponse, PutSkillAuthorizationError> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.PutSkillAuthorization");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<PutSkillAuthorizationResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(PutSkillAuthorizationError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Registers an Alexa-enabled device built by an Original Equipment Manufacturer (OEM) using Alexa Voice Service (AVS).</p>
    fn register_avs_device(
        &self,
        input: RegisterAVSDeviceRequest,
    ) -> RusotoFuture<RegisterAVSDeviceResponse, RegisterAVSDeviceError> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.RegisterAVSDevice");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<RegisterAVSDeviceResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(RegisterAVSDeviceError::from_response(response))),
                )
            }
        })
    }

    /// <p>Disassociates a skill from the organization under a user's AWS account. If the skill is a private skill, it moves to an AcceptStatus of PENDING. Any private or public skill that is rejected can be added later by calling the ApproveSkill API. </p>
    fn reject_skill(
        &self,
        input: RejectSkillRequest,
    ) -> RusotoFuture<RejectSkillResponse, RejectSkillError> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.RejectSkill");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<RejectSkillResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(RejectSkillError::from_response(response))),
                )
            }
        })
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

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ResolveRoomResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ResolveRoomError::from_response(response))),
                )
            }
        })
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

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<RevokeInvitationResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(RevokeInvitationError::from_response(response))),
                )
            }
        })
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

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<SearchAddressBooksResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(SearchAddressBooksError::from_response(response))),
                )
            }
        })
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

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<SearchContactsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(SearchContactsError::from_response(response))),
                )
            }
        })
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

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<SearchDevicesResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(SearchDevicesError::from_response(response))),
                )
            }
        })
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

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<SearchProfilesResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(SearchProfilesError::from_response(response))),
                )
            }
        })
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

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<SearchRoomsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(SearchRoomsError::from_response(response))),
                )
            }
        })
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

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<SearchSkillGroupsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(SearchSkillGroupsError::from_response(response))),
                )
            }
        })
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

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<SearchUsersResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(SearchUsersError::from_response(response))),
                )
            }
        })
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

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<SendInvitationResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(SendInvitationError::from_response(response))),
                )
            }
        })
    }

    /// <p>Resets a device and its account to the known default settings, by clearing all information and settings set by previous users.</p>
    fn start_device_sync(
        &self,
        input: StartDeviceSyncRequest,
    ) -> RusotoFuture<StartDeviceSyncResponse, StartDeviceSyncError> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.StartDeviceSync");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<StartDeviceSyncResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(StartDeviceSyncError::from_response(response))),
                )
            }
        })
    }

    /// <p>Initiates the discovery of any smart home appliances associated with the room.</p>
    fn start_smart_home_appliance_discovery(
        &self,
        input: StartSmartHomeApplianceDiscoveryRequest,
    ) -> RusotoFuture<StartSmartHomeApplianceDiscoveryResponse, StartSmartHomeApplianceDiscoveryError>
    {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AlexaForBusiness.StartSmartHomeApplianceDiscovery",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<StartSmartHomeApplianceDiscoveryResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(StartSmartHomeApplianceDiscoveryError::from_response(
                        response,
                    ))
                }))
            }
        })
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

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<TagResourceResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(TagResourceError::from_response(response))),
                )
            }
        })
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

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UntagResourceResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UntagResourceError::from_response(response))),
                )
            }
        })
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

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdateAddressBookResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateAddressBookError::from_response(response))),
                )
            }
        })
    }

    /// <p>Updates the configuration of the report delivery schedule with the specified schedule ARN.</p>
    fn update_business_report_schedule(
        &self,
        input: UpdateBusinessReportScheduleRequest,
    ) -> RusotoFuture<UpdateBusinessReportScheduleResponse, UpdateBusinessReportScheduleError> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AlexaForBusiness.UpdateBusinessReportSchedule",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdateBusinessReportScheduleResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateBusinessReportScheduleError::from_response(response))
                }))
            }
        })
    }

    /// <p>Updates an existing conference provider's settings.</p>
    fn update_conference_provider(
        &self,
        input: UpdateConferenceProviderRequest,
    ) -> RusotoFuture<UpdateConferenceProviderResponse, UpdateConferenceProviderError> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.UpdateConferenceProvider");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdateConferenceProviderResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateConferenceProviderError::from_response(response))
                }))
            }
        })
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

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdateContactResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateContactError::from_response(response))),
                )
            }
        })
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

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdateDeviceResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateDeviceError::from_response(response))),
                )
            }
        })
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

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdateProfileResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateProfileError::from_response(response))),
                )
            }
        })
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

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdateRoomResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
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

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdateSkillGroupResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateSkillGroupError::from_response(response))),
                )
            }
        })
    }
}

#[cfg(test)]
mod protocol_tests {}
