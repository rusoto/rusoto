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

use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
use serde_json;
/// <p>An address book with attributes.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ApproveSkillRequest {
    /// <p>The unique identifier of the skill.</p>
    #[serde(rename = "SkillId")]
    pub skill_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ApproveSkillResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AssociateContactWithAddressBookRequest {
    /// <p>The ARN of the address book with which to associate the contact.</p>
    #[serde(rename = "AddressBookArn")]
    pub address_book_arn: String,
    /// <p>The ARN of the contact to associate with an address book.</p>
    #[serde(rename = "ContactArn")]
    pub contact_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AssociateContactWithAddressBookResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AssociateDeviceWithNetworkProfileRequest {
    /// <p>The device ARN.</p>
    #[serde(rename = "DeviceArn")]
    pub device_arn: String,
    /// <p>The ARN of the network profile to associate with a device.</p>
    #[serde(rename = "NetworkProfileArn")]
    pub network_profile_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AssociateDeviceWithNetworkProfileResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AssociateDeviceWithRoomResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AssociateSkillGroupWithRoomResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AssociateSkillWithSkillGroupResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AssociateSkillWithUsersRequest {
    /// <p>The private skill ID you want to make available to enrolled users.</p>
    #[serde(rename = "SkillId")]
    pub skill_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AssociateSkillWithUsersResponse {}

/// <p>The audio message. There is a 1 MB limit on the audio file input and the only supported format is MP3. To convert your MP3 audio files to an Alexa-friendly, </p> <p>required codec version (MPEG version 2) and bit rate (48 kbps), you might use converter software. One option for this is a command-line tool, FFmpeg. For more information, see <a href="https://www.ffmpeg.org/">FFmpeg</a>. The following command converts the provided &lt;input-file&gt; to an MP3 file that is played in the announcement:</p> <p> <code>ffmpeg -i &lt;input-file&gt; -ac 2 -codec:a libmp3lame -b:a 48k -ar 16000 &lt;output-file.mp3&gt;</code> </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Audio {
    /// <p>The locale of the audio message. Currently, en-US is supported.</p>
    #[serde(rename = "Locale")]
    pub locale: String,
    /// <p>The location of the audio file. Currently, S3 URLs are supported. Only S3 locations comprised of safe characters are valid. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/UsingMetadata.html#Safe%20Characters">Safe Characters</a>.</p>
    #[serde(rename = "Location")]
    pub location: String,
}

/// <p>Usage report with specified parameters.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    /// <p>The phone number of the contact. The phone number type defaults to WORK. You can either specify PhoneNumber or PhoneNumbers. We recommend that you use PhoneNumbers, which lets you specify the phone number type and multiple numbers.</p>
    #[serde(rename = "PhoneNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    /// <p>The list of phone numbers for the contact.</p>
    #[serde(rename = "PhoneNumbers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_numbers: Option<Vec<PhoneNumber>>,
    /// <p>The list of SIP addresses for the contact.</p>
    #[serde(rename = "SipAddresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sip_addresses: Option<Vec<SipAddress>>,
}

/// <p>Information related to a contact.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    /// <p>The phone number of the contact. The phone number type defaults to WORK. You can specify PhoneNumber or PhoneNumbers. We recommend that you use PhoneNumbers, which lets you specify the phone number type and multiple numbers.</p>
    #[serde(rename = "PhoneNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    /// <p>The list of phone numbers for the contact.</p>
    #[serde(rename = "PhoneNumbers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_numbers: Option<Vec<PhoneNumber>>,
    /// <p>The list of SIP addresses for the contact.</p>
    #[serde(rename = "SipAddresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sip_addresses: Option<Vec<SipAddress>>,
}

/// <p>The content definition. This can contain only one text, SSML, or audio list object.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Content {
    /// <p>The list of audio messages.</p>
    #[serde(rename = "AudioList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_list: Option<Vec<Audio>>,
    /// <p>The list of SSML messages.</p>
    #[serde(rename = "SsmlList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssml_list: Option<Vec<Ssml>>,
    /// <p>The list of text messages.</p>
    #[serde(rename = "TextList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_list: Option<Vec<Text>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateAddressBookResponse {
    /// <p>The ARN of the newly created address book.</p>
    #[serde(rename = "AddressBookArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_book_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
    /// <p>The recurrence of the reports. If this isn't specified, the report will only be delivered one time when the API is called. </p>
    #[serde(rename = "Recurrence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurrence: Option<BusinessReportRecurrence>,
    /// <p>The S3 bucket name of the output reports. If this isn't specified, the report can be retrieved from a download link by calling ListBusinessReportSchedule. </p>
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateBusinessReportScheduleResponse {
    /// <p>The ARN of the business report schedule.</p>
    #[serde(rename = "ScheduleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateConferenceProviderResponse {
    /// <p>The ARN of the newly-created conference provider.</p>
    #[serde(rename = "ConferenceProviderArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conference_provider_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
    /// <p>The phone number of the contact in E.164 format. The phone number type defaults to WORK. You can specify PhoneNumber or PhoneNumbers. We recommend that you use PhoneNumbers, which lets you specify the phone number type and multiple numbers.</p>
    #[serde(rename = "PhoneNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    /// <p>The list of phone numbers for the contact.</p>
    #[serde(rename = "PhoneNumbers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_numbers: Option<Vec<PhoneNumber>>,
    /// <p>The list of SIP addresses for the contact.</p>
    #[serde(rename = "SipAddresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sip_addresses: Option<Vec<SipAddress>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateContactResponse {
    /// <p>The ARN of the newly created address book.</p>
    #[serde(rename = "ContactArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_arn: Option<String>,
}

/// <p>Creates settings for the end of meeting reminder feature that are applied to a room profile. The end of meeting reminder enables Alexa to remind users when a meeting is ending.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateEndOfMeetingReminder {
    /// <p>Whether an end of meeting reminder is enabled or not.</p>
    #[serde(rename = "Enabled")]
    pub enabled: bool,
    /// <p> A range of 3 to 15 minutes that determines when the reminder begins.</p>
    #[serde(rename = "ReminderAtMinutes")]
    pub reminder_at_minutes: Vec<i64>,
    /// <p>The type of sound that users hear during the end of meeting reminder. </p>
    #[serde(rename = "ReminderType")]
    pub reminder_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateGatewayGroupRequest {
    /// <p> A unique, user-specified identifier for the request that ensures idempotency.</p>
    #[serde(rename = "ClientRequestToken")]
    pub client_request_token: String,
    /// <p>The description of the gateway group.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name of the gateway group.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateGatewayGroupResponse {
    /// <p>The ARN of the created gateway group.</p>
    #[serde(rename = "GatewayGroupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_group_arn: Option<String>,
}

/// <p>Creates settings for the instant booking feature that are applied to a room profile. When users start their meeting with Alexa, Alexa automatically books the room for the configured duration if the room is available.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateInstantBooking {
    /// <p>Duration between 15 and 240 minutes at increments of 15 that determines how long to book an available room when a meeting is started with Alexa.</p>
    #[serde(rename = "DurationInMinutes")]
    pub duration_in_minutes: i64,
    /// <p>Whether instant booking is enabled or not.</p>
    #[serde(rename = "Enabled")]
    pub enabled: bool,
}

/// <p>Creates meeting room settings of a room profile.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateMeetingRoomConfiguration {
    #[serde(rename = "EndOfMeetingReminder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_of_meeting_reminder: Option<CreateEndOfMeetingReminder>,
    /// <p>Settings to automatically book a room for a configured duration if it's free when joining a meeting with Alexa.</p>
    #[serde(rename = "InstantBooking")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instant_booking: Option<CreateInstantBooking>,
    /// <p>Settings for requiring a check in when a room is reserved. Alexa can cancel a room reservation if it's not checked into to make the room available for others. Users can check in by joining the meeting with Alexa or an AVS device, or by saying “Alexa, check in.”</p>
    #[serde(rename = "RequireCheckIn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_check_in: Option<CreateRequireCheckIn>,
    /// <p>Whether room utilization metrics are enabled or not.</p>
    #[serde(rename = "RoomUtilizationMetricsEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room_utilization_metrics_enabled: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateNetworkProfileRequest {
    /// <p>The ARN of the Private Certificate Authority (PCA) created in AWS Certificate Manager (ACM). This is used to issue certificates to the devices. </p>
    #[serde(rename = "CertificateAuthorityArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_authority_arn: Option<String>,
    #[serde(rename = "ClientRequestToken")]
    pub client_request_token: String,
    /// <p>The current password of the Wi-Fi network.</p>
    #[serde(rename = "CurrentPassword")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_password: Option<String>,
    /// <p>Detailed information about a device's network profile.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The authentication standard that is used in the EAP framework. Currently, EAP_TLS is supported.</p>
    #[serde(rename = "EapMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eap_method: Option<String>,
    /// <p>The name of the network profile associated with a device.</p>
    #[serde(rename = "NetworkProfileName")]
    pub network_profile_name: String,
    /// <p>The next, or subsequent, password of the Wi-Fi network. This password is asynchronously transmitted to the device and is used when the password of the network changes to NextPassword. </p>
    #[serde(rename = "NextPassword")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_password: Option<String>,
    /// <p>The security type of the Wi-Fi network. This can be WPA2_ENTERPRISE, WPA2_PSK, WPA_PSK, WEP, or OPEN.</p>
    #[serde(rename = "SecurityType")]
    pub security_type: String,
    /// <p>The SSID of the Wi-Fi network.</p>
    #[serde(rename = "Ssid")]
    pub ssid: String,
    /// <p>The root certificates of your authentication server that is installed on your devices and used to trust your authentication server during EAP negotiation. </p>
    #[serde(rename = "TrustAnchors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust_anchors: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateNetworkProfileResponse {
    /// <p>The ARN of the network profile associated with a device.</p>
    #[serde(rename = "NetworkProfileArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_profile_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
    /// <p>The locale of the room profile. (This is currently only available to a limited preview audience.)</p>
    #[serde(rename = "Locale")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    /// <p>The maximum volume limit for a room profile.</p>
    #[serde(rename = "MaxVolumeLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_volume_limit: Option<i64>,
    /// <p>The meeting room settings of a room profile.</p>
    #[serde(rename = "MeetingRoomConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meeting_room_configuration: Option<CreateMeetingRoomConfiguration>,
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateProfileResponse {
    /// <p>The ARN of the newly created room profile in the response.</p>
    #[serde(rename = "ProfileArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_arn: Option<String>,
}

/// <p>Creates settings for the require check in feature that are applied to a room profile. Require check in allows a meeting room’s Alexa or AVS device to prompt the user to check in; otherwise, the room will be released.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateRequireCheckIn {
    /// <p>Whether require check in is enabled or not.</p>
    #[serde(rename = "Enabled")]
    pub enabled: bool,
    /// <p>Duration between 5 and 20 minutes to determine when to release the room if it's not checked into.</p>
    #[serde(rename = "ReleaseAfterMinutes")]
    pub release_after_minutes: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateRoomResponse {
    /// <p>The ARN of the newly created room in the response.</p>
    #[serde(rename = "RoomArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateSkillGroupResponse {
    /// <p>The ARN of the newly created skill group in the response.</p>
    #[serde(rename = "SkillGroupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skill_group_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateUserResponse {
    /// <p>The ARN of the newly created user in the response.</p>
    #[serde(rename = "UserArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteAddressBookRequest {
    /// <p>The ARN of the address book to delete.</p>
    #[serde(rename = "AddressBookArn")]
    pub address_book_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteAddressBookResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteBusinessReportScheduleRequest {
    /// <p>The ARN of the business report schedule.</p>
    #[serde(rename = "ScheduleArn")]
    pub schedule_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteBusinessReportScheduleResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteConferenceProviderRequest {
    /// <p>The ARN of the conference provider.</p>
    #[serde(rename = "ConferenceProviderArn")]
    pub conference_provider_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteConferenceProviderResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteContactRequest {
    /// <p>The ARN of the contact to delete.</p>
    #[serde(rename = "ContactArn")]
    pub contact_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteContactResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteDeviceRequest {
    /// <p>The ARN of the device for which to request details.</p>
    #[serde(rename = "DeviceArn")]
    pub device_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteDeviceResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteDeviceUsageDataRequest {
    /// <p>The ARN of the device.</p>
    #[serde(rename = "DeviceArn")]
    pub device_arn: String,
    /// <p>The type of usage data to delete.</p>
    #[serde(rename = "DeviceUsageType")]
    pub device_usage_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteDeviceUsageDataResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteGatewayGroupRequest {
    /// <p>The ARN of the gateway group to delete.</p>
    #[serde(rename = "GatewayGroupArn")]
    pub gateway_group_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteGatewayGroupResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteNetworkProfileRequest {
    /// <p>The ARN of the network profile associated with a device.</p>
    #[serde(rename = "NetworkProfileArn")]
    pub network_profile_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteNetworkProfileResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteProfileRequest {
    /// <p>The ARN of the room profile to delete. Required.</p>
    #[serde(rename = "ProfileArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteProfileResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteRoomRequest {
    /// <p>The ARN of the room to delete. Required.</p>
    #[serde(rename = "RoomArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteRoomResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteRoomSkillParameterResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteSkillAuthorizationResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteSkillGroupRequest {
    /// <p>The ARN of the skill group to delete. Required.</p>
    #[serde(rename = "SkillGroupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skill_group_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteSkillGroupResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteUserResponse {}

/// <p>The details about the developer that published the skill.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    /// <p>Detailed information about a device's network profile.</p>
    #[serde(rename = "NetworkProfileInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_profile_info: Option<DeviceNetworkProfileInfo>,
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeviceData {
    /// <p>The time (in epoch) when the device data was created.</p>
    #[serde(rename = "CreatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
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
    /// <p>The ARN of the network profile associated with a device.</p>
    #[serde(rename = "NetworkProfileArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_profile_arn: Option<String>,
    /// <p>The name of the network profile associated with a device.</p>
    #[serde(rename = "NetworkProfileName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_profile_name: Option<String>,
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

/// <p>Detailed information about a device's network profile.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeviceNetworkProfileInfo {
    /// <p>The ARN of the certificate associated with a device.</p>
    #[serde(rename = "CertificateArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<String>,
    /// <p>The time (in epoch) when the certificate expires.</p>
    #[serde(rename = "CertificateExpirationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_expiration_time: Option<f64>,
    /// <p>The ARN of the network profile associated with a device.</p>
    #[serde(rename = "NetworkProfileArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_profile_arn: Option<String>,
}

/// <p>Details of a device’s status.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeviceStatusDetail {
    /// <p>The device status detail code.</p>
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// <p>The list of available features on the device.</p>
    #[serde(rename = "Feature")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature: Option<String>,
}

/// <p>Detailed information about a device's status.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeviceStatusInfo {
    /// <p>The latest available information about the connection status of a device. </p>
    #[serde(rename = "ConnectionStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_status: Option<String>,
    /// <p>The time (in epoch) when the device connection status changed.</p>
    #[serde(rename = "ConnectionStatusUpdatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_status_updated_time: Option<f64>,
    /// <p>One or more device status detail descriptions.</p>
    #[serde(rename = "DeviceStatusDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_status_details: Option<Vec<DeviceStatusDetail>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DisassociateContactFromAddressBookRequest {
    /// <p>The ARN of the address from which to disassociate the contact.</p>
    #[serde(rename = "AddressBookArn")]
    pub address_book_arn: String,
    /// <p>The ARN of the contact to disassociate from an address book.</p>
    #[serde(rename = "ContactArn")]
    pub contact_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DisassociateContactFromAddressBookResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DisassociateDeviceFromRoomRequest {
    /// <p>The ARN of the device to disassociate from a room. Required.</p>
    #[serde(rename = "DeviceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DisassociateDeviceFromRoomResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DisassociateSkillFromSkillGroupResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DisassociateSkillFromUsersRequest {
    /// <p> The private skill ID you want to make unavailable for enrolled users.</p>
    #[serde(rename = "SkillId")]
    pub skill_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DisassociateSkillFromUsersResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DisassociateSkillGroupFromRoomResponse {}

/// <p>Settings for the end of meeting reminder feature that are applied to a room profile. The end of meeting reminder enables Alexa to remind users when a meeting is ending. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct EndOfMeetingReminder {
    /// <p>Whether an end of meeting reminder is enabled or not.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>A range of 3 to 15 minutes that determines when the reminder begins.</p>
    #[serde(rename = "ReminderAtMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reminder_at_minutes: Option<Vec<i64>>,
    /// <p>The type of sound that users hear during the end of meeting reminder. </p>
    #[serde(rename = "ReminderType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reminder_type: Option<String>,
}

/// <p>A filter name and value pair that is used to return a more specific list of results. Filters can be used to match a set of resources by various criteria.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Filter {
    /// <p>The key of a filter.</p>
    #[serde(rename = "Key")]
    pub key: String,
    /// <p>The values of a filter.</p>
    #[serde(rename = "Values")]
    pub values: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ForgetSmartHomeAppliancesRequest {
    /// <p>The room that the appliances are associated with.</p>
    #[serde(rename = "RoomArn")]
    pub room_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ForgetSmartHomeAppliancesResponse {}

/// <p>The details of the gateway. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Gateway {
    /// <p>The ARN of the gateway.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The description of the gateway.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The ARN of the gateway group that the gateway is associated to.</p>
    #[serde(rename = "GatewayGroupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_group_arn: Option<String>,
    /// <p>The name of the gateway.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The software version of the gateway. The gateway automatically updates its software version during normal operation.</p>
    #[serde(rename = "SoftwareVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub software_version: Option<String>,
}

/// <p>The details of the gateway group.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GatewayGroup {
    /// <p>The ARN of the gateway group.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The description of the gateway group.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name of the gateway group.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>The summary of a gateway group.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GatewayGroupSummary {
    /// <p>The ARN of the gateway group.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The description of the gateway group.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name of the gateway group.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>The summary of a gateway.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GatewaySummary {
    /// <p>The ARN of the gateway.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The description of the gateway.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The ARN of the gateway group that the gateway is associated to.</p>
    #[serde(rename = "GatewayGroupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_group_arn: Option<String>,
    /// <p>The name of the gateway.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The software version of the gateway. The gateway automatically updates its software version during normal operation.</p>
    #[serde(rename = "SoftwareVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub software_version: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetAddressBookRequest {
    /// <p>The ARN of the address book for which to request details.</p>
    #[serde(rename = "AddressBookArn")]
    pub address_book_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetAddressBookResponse {
    /// <p>The details of the requested address book.</p>
    #[serde(rename = "AddressBook")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_book: Option<AddressBook>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetConferencePreferenceRequest {}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetConferencePreferenceResponse {
    /// <p>The conference preference.</p>
    #[serde(rename = "Preference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<ConferencePreference>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetConferenceProviderRequest {
    /// <p>The ARN of the newly created conference provider.</p>
    #[serde(rename = "ConferenceProviderArn")]
    pub conference_provider_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetConferenceProviderResponse {
    /// <p>The conference provider.</p>
    #[serde(rename = "ConferenceProvider")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conference_provider: Option<ConferenceProvider>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetContactRequest {
    /// <p>The ARN of the contact for which to request details.</p>
    #[serde(rename = "ContactArn")]
    pub contact_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetContactResponse {
    /// <p>The details of the requested contact.</p>
    #[serde(rename = "Contact")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<Contact>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetDeviceRequest {
    /// <p>The ARN of the device for which to request details. Required.</p>
    #[serde(rename = "DeviceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetDeviceResponse {
    /// <p>The details of the device requested. Required.</p>
    #[serde(rename = "Device")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<Device>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetGatewayGroupRequest {
    /// <p>The ARN of the gateway group to get.</p>
    #[serde(rename = "GatewayGroupArn")]
    pub gateway_group_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetGatewayGroupResponse {
    #[serde(rename = "GatewayGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_group: Option<GatewayGroup>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetGatewayRequest {
    /// <p>The ARN of the gateway to get.</p>
    #[serde(rename = "GatewayArn")]
    pub gateway_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetGatewayResponse {
    /// <p>The details of the gateway.</p>
    #[serde(rename = "Gateway")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway: Option<Gateway>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetInvitationConfigurationRequest {}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetInvitationConfigurationResponse {
    /// <p>The email ID of the organization or individual contact that the enrolled user can use. </p>
    #[serde(rename = "ContactEmail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_email: Option<String>,
    /// <p>The name of the organization sending the enrollment invite to a user.</p>
    #[serde(rename = "OrganizationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_name: Option<String>,
    /// <p>The list of private skill IDs that you want to recommend to the user to enable in the invitation.</p>
    #[serde(rename = "PrivateSkillIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_skill_ids: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetNetworkProfileRequest {
    /// <p>The ARN of the network profile associated with a device.</p>
    #[serde(rename = "NetworkProfileArn")]
    pub network_profile_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetNetworkProfileResponse {
    /// <p>The network profile associated with a device.</p>
    #[serde(rename = "NetworkProfile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_profile: Option<NetworkProfile>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetProfileRequest {
    /// <p>The ARN of the room profile for which to request details. Required.</p>
    #[serde(rename = "ProfileArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetProfileResponse {
    /// <p>The details of the room profile requested. Required.</p>
    #[serde(rename = "Profile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<Profile>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetRoomRequest {
    /// <p>The ARN of the room for which to request details. Required.</p>
    #[serde(rename = "RoomArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetRoomResponse {
    /// <p>The details of the room requested.</p>
    #[serde(rename = "Room")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room: Option<Room>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetRoomSkillParameterResponse {
    /// <p>The details of the room skill parameter requested. Required.</p>
    #[serde(rename = "RoomSkillParameter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room_skill_parameter: Option<RoomSkillParameter>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetSkillGroupRequest {
    /// <p>The ARN of the skill group for which to get details. Required.</p>
    #[serde(rename = "SkillGroupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skill_group_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

/// <p>Settings for the instant booking feature that are applied to a room profile. When users start their meeting with Alexa, Alexa automatically books the room for the configured duration if the room is available.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct InstantBooking {
    /// <p>Duration between 15 and 240 minutes at increments of 15 that determines how long to book an available room when a meeting is started with Alexa. </p>
    #[serde(rename = "DurationInMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_in_minutes: Option<i64>,
    /// <p>Whether instant booking is enabled or not.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListGatewayGroupsRequest {
    /// <p>The maximum number of gateway group summaries to return. The default is 50.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token used to paginate though multiple pages of gateway group summaries.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListGatewayGroupsResponse {
    /// <p>The gateway groups in the list.</p>
    #[serde(rename = "GatewayGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_groups: Option<Vec<GatewayGroupSummary>>,
    /// <p>The token used to paginate though multiple pages of gateway group summaries.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListGatewaysRequest {
    /// <p>The gateway group ARN for which to list gateways.</p>
    #[serde(rename = "GatewayGroupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_group_arn: Option<String>,
    /// <p>The maximum number of gateway summaries to return. The default is 50.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token used to paginate though multiple pages of gateway summaries.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListGatewaysResponse {
    /// <p>The gateways in the list.</p>
    #[serde(rename = "Gateways")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateways: Option<Vec<GatewaySummary>>,
    /// <p>The token used to paginate though multiple pages of gateway summaries.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListSkillsRequest {
    /// <p>Whether the skill is enabled under the user's account.</p>
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

/// <p>Meeting room settings of a room profile.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct MeetingRoomConfiguration {
    /// <p>Settings for the end of meeting reminder feature that are applied to a room profile. The end of meeting reminder enables Alexa to remind users when a meeting is ending. </p>
    #[serde(rename = "EndOfMeetingReminder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_of_meeting_reminder: Option<EndOfMeetingReminder>,
    /// <p>Settings to automatically book the room if available for a configured duration when joining a meeting with Alexa. </p>
    #[serde(rename = "InstantBooking")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instant_booking: Option<InstantBooking>,
    /// <p>Settings for requiring a check in when a room is reserved. Alexa can cancel a room reservation if it's not checked into. This makes the room available for others. Users can check in by joining the meeting with Alexa or an AVS device, or by saying “Alexa, check in.” </p>
    #[serde(rename = "RequireCheckIn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_check_in: Option<RequireCheckIn>,
    /// <p>Whether room utilization metrics are enabled or not.</p>
    #[serde(rename = "RoomUtilizationMetricsEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room_utilization_metrics_enabled: Option<bool>,
}

/// <p><p>The values that indicate whether a pin is always required (YES), never required (NO), or OPTIONAL.</p> <ul> <li> <p>If YES, Alexa will always ask for a meeting pin.</p> </li> <li> <p>If NO, Alexa will never ask for a meeting pin.</p> </li> <li> <p>If OPTIONAL, Alexa will ask if you have a meeting pin and if the customer responds with yes, it will ask for the meeting pin.</p> </li> </ul></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MeetingSetting {
    /// <p>The values that indicate whether the pin is always required.</p>
    #[serde(rename = "RequirePin")]
    pub require_pin: String,
}

/// <p>The network profile associated with a device.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct NetworkProfile {
    /// <p>The ARN of the Private Certificate Authority (PCA) created in AWS Certificate Manager (ACM). This is used to issue certificates to the devices. </p>
    #[serde(rename = "CertificateAuthorityArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_authority_arn: Option<String>,
    /// <p>The current password of the Wi-Fi network.</p>
    #[serde(rename = "CurrentPassword")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_password: Option<String>,
    /// <p>Detailed information about a device's network profile.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The authentication standard that is used in the EAP framework. Currently, EAP_TLS is supported. </p>
    #[serde(rename = "EapMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eap_method: Option<String>,
    /// <p>The ARN of the network profile associated with a device.</p>
    #[serde(rename = "NetworkProfileArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_profile_arn: Option<String>,
    /// <p>The name of the network profile associated with a device.</p>
    #[serde(rename = "NetworkProfileName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_profile_name: Option<String>,
    /// <p>The next, or subsequent, password of the Wi-Fi network. This password is asynchronously transmitted to the device and is used when the password of the network changes to NextPassword. </p>
    #[serde(rename = "NextPassword")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_password: Option<String>,
    /// <p>The security type of the Wi-Fi network. This can be WPA2_ENTERPRISE, WPA2_PSK, WPA_PSK, WEP, or OPEN.</p>
    #[serde(rename = "SecurityType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_type: Option<String>,
    /// <p>The SSID of the Wi-Fi network.</p>
    #[serde(rename = "Ssid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssid: Option<String>,
    /// <p>The root certificates of your authentication server, which is installed on your devices and used to trust your authentication server during EAP negotiation.</p>
    #[serde(rename = "TrustAnchors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust_anchors: Option<Vec<String>>,
}

/// <p>The data associated with a network profile.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct NetworkProfileData {
    /// <p>The ARN of the Private Certificate Authority (PCA) created in AWS Certificate Manager (ACM). This is used to issue certificates to the devices.</p>
    #[serde(rename = "CertificateAuthorityArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_authority_arn: Option<String>,
    /// <p>Detailed information about a device's network profile.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The authentication standard that is used in the EAP framework. Currently, EAP_TLS is supported.</p>
    #[serde(rename = "EapMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eap_method: Option<String>,
    /// <p>The ARN of the network profile associated with a device.</p>
    #[serde(rename = "NetworkProfileArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_profile_arn: Option<String>,
    /// <p>The name of the network profile associated with a device.</p>
    #[serde(rename = "NetworkProfileName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_profile_name: Option<String>,
    /// <p>The security type of the Wi-Fi network. This can be WPA2_ENTERPRISE, WPA2_PSK, WPA_PSK, WEP, or OPEN.</p>
    #[serde(rename = "SecurityType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_type: Option<String>,
    /// <p>The SSID of the Wi-Fi network.</p>
    #[serde(rename = "Ssid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssid: Option<String>,
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

/// <p>The phone number for the contact containing the raw number and phone number type.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PhoneNumber {
    /// <p>The raw value of the phone number.</p>
    #[serde(rename = "Number")]
    pub number: String,
    /// <p>The type of the phone number.</p>
    #[serde(rename = "Type")]
    pub type_: String,
}

/// <p>A room profile with attributes.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    /// <p>The locale of a room profile. (This is currently available only to a limited preview audience.)</p>
    #[serde(rename = "Locale")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    /// <p>The max volume limit of a room profile.</p>
    #[serde(rename = "MaxVolumeLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_volume_limit: Option<i64>,
    /// <p>Meeting room settings of a room profile.</p>
    #[serde(rename = "MeetingRoomConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meeting_room_configuration: Option<MeetingRoomConfiguration>,
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    /// <p>The locale of a room profile. (This is currently available only to a limited preview audience.)</p>
    #[serde(rename = "Locale")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
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
    /// <p>The time zone of a room profile.</p>
    #[serde(rename = "Timezone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    /// <p>The wake word of a room profile.</p>
    #[serde(rename = "WakeWord")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wake_word: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutConferencePreferenceRequest {
    /// <p>The conference preference of a specific conference provider.</p>
    #[serde(rename = "ConferencePreference")]
    pub conference_preference: ConferencePreference,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutConferencePreferenceResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutInvitationConfigurationRequest {
    /// <p>The email ID of the organization or individual contact that the enrolled user can use. </p>
    #[serde(rename = "ContactEmail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_email: Option<String>,
    /// <p>The name of the organization sending the enrollment invite to a user.</p>
    #[serde(rename = "OrganizationName")]
    pub organization_name: String,
    /// <p>The list of private skill IDs that you want to recommend to the user to enable in the invitation.</p>
    #[serde(rename = "PrivateSkillIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_skill_ids: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutInvitationConfigurationResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutRoomSkillParameterResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutSkillAuthorizationResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RegisterAVSDeviceResponse {
    /// <p>The ARN of the device.</p>
    #[serde(rename = "DeviceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RejectSkillRequest {
    /// <p>The unique identifier of the skill.</p>
    #[serde(rename = "SkillId")]
    pub skill_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RejectSkillResponse {}

/// <p>Settings for the require check in feature that are applied to a room profile. Require check in allows a meeting room’s Alexa or AVS device to prompt the user to check in; otherwise, the room will be released. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RequireCheckIn {
    /// <p>Whether require check in is enabled or not.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>Duration between 5 and 20 minutes to determine when to release the room if it's not checked into. </p>
    #[serde(rename = "ReleaseAfterMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_after_minutes: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ResolveRoomRequest {
    /// <p>The ARN of the skill that was requested. Required.</p>
    #[serde(rename = "SkillId")]
    pub skill_id: String,
    /// <p>The ARN of the user. Required.</p>
    #[serde(rename = "UserId")]
    pub user_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RevokeInvitationResponse {}

/// <p>A room with attributes.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SearchDevicesRequest {
    /// <p>The filters to use to list a specified set of devices. Supported filter keys are DeviceName, DeviceStatus, DeviceStatusDetailCode, RoomName, DeviceType, DeviceSerialNumber, UnassociatedOnly, ConnectionStatus (ONLINE and OFFLINE), NetworkProfileName, NetworkProfileArn, Feature, and FailureCode.</p>
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
    /// <p>The sort order to use in listing the specified set of devices. Supported sort keys are DeviceName, DeviceStatus, RoomName, DeviceType, DeviceSerialNumber, ConnectionStatus, NetworkProfileName, NetworkProfileArn, Feature, and FailureCode.</p>
    #[serde(rename = "SortCriteria")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_criteria: Option<Vec<Sort>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SearchNetworkProfilesRequest {
    /// <p>The filters to use to list a specified set of network profiles. Valid filters are NetworkProfileName, Ssid, and SecurityType.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    /// <p>The maximum number of results to include in the response. If more results exist than the specified MaxResults value, a token is included in the response so that the remaining results can be retrieved. </p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>An optional token returned from a prior request. Use this token for pagination of results from this action. If this parameter is specified, the response includes only results beyond the token, up to the value specified by MaxResults. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The sort order to use to list the specified set of network profiles. Valid sort criteria includes NetworkProfileName, Ssid, and SecurityType.</p>
    #[serde(rename = "SortCriteria")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_criteria: Option<Vec<Sort>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SearchNetworkProfilesResponse {
    /// <p>The network profiles that meet the specified set of filter criteria, in sort order. It is a list of NetworkProfileData objects. </p>
    #[serde(rename = "NetworkProfiles")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_profiles: Option<Vec<NetworkProfileData>>,
    /// <p>An optional token returned from a prior request. Use this token for pagination of results from this action. If this parameter is specified, the response includes only results beyond the token, up to the value specified by MaxResults.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The total number of network profiles returned.</p>
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SendAnnouncementRequest {
    /// <p>The unique, user-specified identifier for the request that ensures idempotency.</p>
    #[serde(rename = "ClientRequestToken")]
    pub client_request_token: String,
    /// <p>The announcement content. This can contain only one of the three possible announcement types (text, SSML or audio).</p>
    #[serde(rename = "Content")]
    pub content: Content,
    /// <p>The filters to use to send an announcement to a specified list of rooms. The supported filter keys are RoomName, ProfileName, RoomArn, and ProfileArn. To send to all rooms, specify an empty RoomFilters list.</p>
    #[serde(rename = "RoomFilters")]
    pub room_filters: Vec<Filter>,
    /// <p>The time to live for an announcement. Default is 300. If delivery doesn't occur within this time, the announcement is not delivered.</p>
    #[serde(rename = "TimeToLiveInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_to_live_in_seconds: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SendAnnouncementResponse {
    /// <p>The identifier of the announcement.</p>
    #[serde(rename = "AnnouncementArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub announcement_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SendInvitationRequest {
    /// <p>The ARN of the user to whom to send an invitation. Required.</p>
    #[serde(rename = "UserArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SendInvitationResponse {}

/// <p>The SIP address for the contact containing the URI and SIP address type.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SipAddress {
    /// <p>The type of the SIP address.</p>
    #[serde(rename = "Type")]
    pub type_: String,
    /// <p>The URI for the SIP address.</p>
    #[serde(rename = "Uri")]
    pub uri: String,
}

/// <p>Granular information about the skill.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Sort {
    /// <p>The sort key of a sort object.</p>
    #[serde(rename = "Key")]
    pub key: String,
    /// <p>The sort value of a sort object.</p>
    #[serde(rename = "Value")]
    pub value: String,
}

/// <p>The SSML message. For more information, see <a href="https://developer.amazon.com/docs/custom-skills/speech-synthesis-markup-language-ssml-reference.html">SSML Reference</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Ssml {
    /// <p>The locale of the SSML message. Currently, en-US is supported.</p>
    #[serde(rename = "Locale")]
    pub locale: String,
    /// <p>The value of the SSML message in the correct SSML format. The audio tag is not supported.</p>
    #[serde(rename = "Value")]
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartDeviceSyncResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartSmartHomeApplianceDiscoveryRequest {
    /// <p>The room where smart home appliance discovery was initiated.</p>
    #[serde(rename = "RoomArn")]
    pub room_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartSmartHomeApplianceDiscoveryResponse {}

/// <p>A key-value pair that can be associated with a resource. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    /// <p>The key of a tag. Tag keys are case-sensitive. </p>
    #[serde(rename = "Key")]
    pub key: String,
    /// <p>The value of a tag. Tag values are case sensitive and can be null.</p>
    #[serde(rename = "Value")]
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceRequest {
    /// <p>The ARN of the resource to which to add metadata tags. Required. </p>
    #[serde(rename = "Arn")]
    pub arn: String,
    /// <p>The tags to be added to the specified resource. Do not provide system tags. Required. </p>
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TagResourceResponse {}

/// <p>The text message.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Text {
    /// <p>The locale of the text message. Currently, en-US is supported.</p>
    #[serde(rename = "Locale")]
    pub locale: String,
    /// <p>The value of the text message.</p>
    #[serde(rename = "Value")]
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceRequest {
    /// <p>The ARN of the resource from which to remove metadata tags. Required. </p>
    #[serde(rename = "Arn")]
    pub arn: String,
    /// <p>The tags to be removed from the specified resource. Do not provide system tags. Required. </p>
    #[serde(rename = "TagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UntagResourceResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateAddressBookResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateBusinessReportScheduleResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateConferenceProviderResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
    /// <p>The updated phone number of the contact. The phone number type defaults to WORK. You can either specify PhoneNumber or PhoneNumbers. We recommend that you use PhoneNumbers, which lets you specify the phone number type and multiple numbers.</p>
    #[serde(rename = "PhoneNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    /// <p>The list of phone numbers for the contact.</p>
    #[serde(rename = "PhoneNumbers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_numbers: Option<Vec<PhoneNumber>>,
    /// <p>The list of SIP addresses for the contact.</p>
    #[serde(rename = "SipAddresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sip_addresses: Option<Vec<SipAddress>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateContactResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateDeviceResponse {}

/// <p>Settings for the end of meeting reminder feature that are applied to a room profile. The end of meeting reminder enables Alexa to remind users when a meeting is ending. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateEndOfMeetingReminder {
    /// <p>Whether an end of meeting reminder is enabled or not.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>Updates settings for the end of meeting reminder feature that are applied to a room profile. The end of meeting reminder enables Alexa to remind users when a meeting is ending. </p>
    #[serde(rename = "ReminderAtMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reminder_at_minutes: Option<Vec<i64>>,
    /// <p>The type of sound that users hear during the end of meeting reminder. </p>
    #[serde(rename = "ReminderType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reminder_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateGatewayGroupRequest {
    /// <p>The updated description of the gateway group.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The ARN of the gateway group to update.</p>
    #[serde(rename = "GatewayGroupArn")]
    pub gateway_group_arn: String,
    /// <p>The updated name of the gateway group.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateGatewayGroupResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateGatewayRequest {
    /// <p>The updated description of the gateway.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The ARN of the gateway to update.</p>
    #[serde(rename = "GatewayArn")]
    pub gateway_arn: String,
    /// <p>The updated name of the gateway.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The updated software version of the gateway. The gateway automatically updates its software version during normal operation.</p>
    #[serde(rename = "SoftwareVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub software_version: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateGatewayResponse {}

/// <p>Updates settings for the instant booking feature that are applied to a room profile. If instant booking is enabled, Alexa automatically reserves a room if it is free when a user joins a meeting with Alexa.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateInstantBooking {
    /// <p>Duration between 15 and 240 minutes at increments of 15 that determines how long to book an available room when a meeting is started with Alexa.</p>
    #[serde(rename = "DurationInMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_in_minutes: Option<i64>,
    /// <p>Whether instant booking is enabled or not.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

/// <p>Updates meeting room settings of a room profile.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateMeetingRoomConfiguration {
    /// <p>Settings for the end of meeting reminder feature that are applied to a room profile. The end of meeting reminder enables Alexa to remind users when a meeting is ending. </p>
    #[serde(rename = "EndOfMeetingReminder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_of_meeting_reminder: Option<UpdateEndOfMeetingReminder>,
    /// <p>Settings to automatically book an available room available for a configured duration when joining a meeting with Alexa.</p>
    #[serde(rename = "InstantBooking")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instant_booking: Option<UpdateInstantBooking>,
    /// <p>Settings for requiring a check in when a room is reserved. Alexa can cancel a room reservation if it's not checked into to make the room available for others. Users can check in by joining the meeting with Alexa or an AVS device, or by saying “Alexa, check in.” </p>
    #[serde(rename = "RequireCheckIn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_check_in: Option<UpdateRequireCheckIn>,
    /// <p>Whether room utilization metrics are enabled or not.</p>
    #[serde(rename = "RoomUtilizationMetricsEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room_utilization_metrics_enabled: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateNetworkProfileRequest {
    /// <p>The ARN of the Private Certificate Authority (PCA) created in AWS Certificate Manager (ACM). This is used to issue certificates to the devices. </p>
    #[serde(rename = "CertificateAuthorityArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_authority_arn: Option<String>,
    /// <p>The current password of the Wi-Fi network.</p>
    #[serde(rename = "CurrentPassword")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_password: Option<String>,
    /// <p>Detailed information about a device's network profile.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The ARN of the network profile associated with a device.</p>
    #[serde(rename = "NetworkProfileArn")]
    pub network_profile_arn: String,
    /// <p>The name of the network profile associated with a device.</p>
    #[serde(rename = "NetworkProfileName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_profile_name: Option<String>,
    /// <p>The next, or subsequent, password of the Wi-Fi network. This password is asynchronously transmitted to the device and is used when the password of the network changes to NextPassword. </p>
    #[serde(rename = "NextPassword")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_password: Option<String>,
    /// <p>The root certificate(s) of your authentication server that will be installed on your devices and used to trust your authentication server during EAP negotiation. </p>
    #[serde(rename = "TrustAnchors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust_anchors: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateNetworkProfileResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
    /// <p>The updated locale for the room profile. (This is currently only available to a limited preview audience.)</p>
    #[serde(rename = "Locale")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    /// <p>The updated maximum volume limit for the room profile.</p>
    #[serde(rename = "MaxVolumeLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_volume_limit: Option<i64>,
    /// <p>The updated meeting room settings of a room profile.</p>
    #[serde(rename = "MeetingRoomConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meeting_room_configuration: Option<UpdateMeetingRoomConfiguration>,
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateProfileResponse {}

/// <p>Updates settings for the require check in feature that are applied to a room profile. Require check in allows a meeting room’s Alexa or AVS device to prompt the user to check in; otherwise, the room will be released. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateRequireCheckIn {
    /// <p>Whether require check in is enabled or not.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>Duration between 5 and 20 minutes to determine when to release the room if it's not checked into. </p>
    #[serde(rename = "ReleaseAfterMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_after_minutes: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateRoomResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateSkillGroupResponse {}

/// <p>Information related to a user.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    /// <p>There is a concurrent modification of resources.</p>
    ConcurrentModification(String),
    /// <p>You are performing an action that would put you beyond your account's limits.</p>
    LimitExceeded(String),
    /// <p>The resource is not found.</p>
    NotFound(String),
}

impl ApproveSkillError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ApproveSkillError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(ApproveSkillError::ConcurrentModification(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(ApproveSkillError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ApproveSkillError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ApproveSkillError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ApproveSkillError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            ApproveSkillError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            ApproveSkillError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ApproveSkillError {}
/// Errors returned by AssociateContactWithAddressBook
#[derive(Debug, PartialEq)]
pub enum AssociateContactWithAddressBookError {
    /// <p>You are performing an action that would put you beyond your account's limits.</p>
    LimitExceeded(String),
}

impl AssociateContactWithAddressBookError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<AssociateContactWithAddressBookError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "LimitExceededException" => {
                    return RusotoError::Service(
                        AssociateContactWithAddressBookError::LimitExceeded(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for AssociateContactWithAddressBookError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AssociateContactWithAddressBookError::LimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for AssociateContactWithAddressBookError {}
/// Errors returned by AssociateDeviceWithNetworkProfile
#[derive(Debug, PartialEq)]
pub enum AssociateDeviceWithNetworkProfileError {
    /// <p>There is a concurrent modification of resources.</p>
    ConcurrentModification(String),
    /// <p>The request failed because this device is no longer registered and therefore no longer managed by this account.</p>
    DeviceNotRegistered(String),
    /// <p>The resource is not found.</p>
    NotFound(String),
}

impl AssociateDeviceWithNetworkProfileError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<AssociateDeviceWithNetworkProfileError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(
                        AssociateDeviceWithNetworkProfileError::ConcurrentModification(err.msg),
                    )
                }
                "DeviceNotRegisteredException" => {
                    return RusotoError::Service(
                        AssociateDeviceWithNetworkProfileError::DeviceNotRegistered(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(AssociateDeviceWithNetworkProfileError::NotFound(
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
impl fmt::Display for AssociateDeviceWithNetworkProfileError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AssociateDeviceWithNetworkProfileError::ConcurrentModification(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociateDeviceWithNetworkProfileError::DeviceNotRegistered(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociateDeviceWithNetworkProfileError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AssociateDeviceWithNetworkProfileError {}
/// Errors returned by AssociateDeviceWithRoom
#[derive(Debug, PartialEq)]
pub enum AssociateDeviceWithRoomError {
    /// <p>There is a concurrent modification of resources.</p>
    ConcurrentModification(String),
    /// <p>The request failed because this device is no longer registered and therefore no longer managed by this account.</p>
    DeviceNotRegistered(String),
    /// <p>You are performing an action that would put you beyond your account's limits.</p>
    LimitExceeded(String),
}

impl AssociateDeviceWithRoomError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AssociateDeviceWithRoomError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(
                        AssociateDeviceWithRoomError::ConcurrentModification(err.msg),
                    )
                }
                "DeviceNotRegisteredException" => {
                    return RusotoError::Service(AssociateDeviceWithRoomError::DeviceNotRegistered(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(AssociateDeviceWithRoomError::LimitExceeded(
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
impl fmt::Display for AssociateDeviceWithRoomError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AssociateDeviceWithRoomError::ConcurrentModification(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociateDeviceWithRoomError::DeviceNotRegistered(ref cause) => write!(f, "{}", cause),
            AssociateDeviceWithRoomError::LimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AssociateDeviceWithRoomError {}
/// Errors returned by AssociateSkillGroupWithRoom
#[derive(Debug, PartialEq)]
pub enum AssociateSkillGroupWithRoomError {
    /// <p>There is a concurrent modification of resources.</p>
    ConcurrentModification(String),
}

impl AssociateSkillGroupWithRoomError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<AssociateSkillGroupWithRoomError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(
                        AssociateSkillGroupWithRoomError::ConcurrentModification(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for AssociateSkillGroupWithRoomError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AssociateSkillGroupWithRoomError::ConcurrentModification(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for AssociateSkillGroupWithRoomError {}
/// Errors returned by AssociateSkillWithSkillGroup
#[derive(Debug, PartialEq)]
pub enum AssociateSkillWithSkillGroupError {
    /// <p>There is a concurrent modification of resources.</p>
    ConcurrentModification(String),
    /// <p>The resource is not found.</p>
    NotFound(String),
    /// <p>The skill must be linked to a third-party account.</p>
    SkillNotLinked(String),
}

impl AssociateSkillWithSkillGroupError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<AssociateSkillWithSkillGroupError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(
                        AssociateSkillWithSkillGroupError::ConcurrentModification(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(AssociateSkillWithSkillGroupError::NotFound(
                        err.msg,
                    ))
                }
                "SkillNotLinkedException" => {
                    return RusotoError::Service(AssociateSkillWithSkillGroupError::SkillNotLinked(
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
impl fmt::Display for AssociateSkillWithSkillGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AssociateSkillWithSkillGroupError::ConcurrentModification(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociateSkillWithSkillGroupError::NotFound(ref cause) => write!(f, "{}", cause),
            AssociateSkillWithSkillGroupError::SkillNotLinked(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AssociateSkillWithSkillGroupError {}
/// Errors returned by AssociateSkillWithUsers
#[derive(Debug, PartialEq)]
pub enum AssociateSkillWithUsersError {
    /// <p>There is a concurrent modification of resources.</p>
    ConcurrentModification(String),
    /// <p>The resource is not found.</p>
    NotFound(String),
}

impl AssociateSkillWithUsersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AssociateSkillWithUsersError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(
                        AssociateSkillWithUsersError::ConcurrentModification(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(AssociateSkillWithUsersError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for AssociateSkillWithUsersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AssociateSkillWithUsersError::ConcurrentModification(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociateSkillWithUsersError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AssociateSkillWithUsersError {}
/// Errors returned by CreateAddressBook
#[derive(Debug, PartialEq)]
pub enum CreateAddressBookError {
    /// <p>The resource being created already exists.</p>
    AlreadyExists(String),
    /// <p>You are performing an action that would put you beyond your account's limits.</p>
    LimitExceeded(String),
}

impl CreateAddressBookError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateAddressBookError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AlreadyExistsException" => {
                    return RusotoError::Service(CreateAddressBookError::AlreadyExists(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateAddressBookError::LimitExceeded(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateAddressBookError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateAddressBookError::AlreadyExists(ref cause) => write!(f, "{}", cause),
            CreateAddressBookError::LimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateAddressBookError {}
/// Errors returned by CreateBusinessReportSchedule
#[derive(Debug, PartialEq)]
pub enum CreateBusinessReportScheduleError {
    /// <p>The resource being created already exists.</p>
    AlreadyExists(String),
}

impl CreateBusinessReportScheduleError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateBusinessReportScheduleError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AlreadyExistsException" => {
                    return RusotoError::Service(CreateBusinessReportScheduleError::AlreadyExists(
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
impl fmt::Display for CreateBusinessReportScheduleError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateBusinessReportScheduleError::AlreadyExists(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateBusinessReportScheduleError {}
/// Errors returned by CreateConferenceProvider
#[derive(Debug, PartialEq)]
pub enum CreateConferenceProviderError {
    /// <p>The resource being created already exists.</p>
    AlreadyExists(String),
}

impl CreateConferenceProviderError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateConferenceProviderError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AlreadyExistsException" => {
                    return RusotoError::Service(CreateConferenceProviderError::AlreadyExists(
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
impl fmt::Display for CreateConferenceProviderError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateConferenceProviderError::AlreadyExists(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateConferenceProviderError {}
/// Errors returned by CreateContact
#[derive(Debug, PartialEq)]
pub enum CreateContactError {
    /// <p>The resource being created already exists.</p>
    AlreadyExists(String),
    /// <p>You are performing an action that would put you beyond your account's limits.</p>
    LimitExceeded(String),
}

impl CreateContactError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateContactError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AlreadyExistsException" => {
                    return RusotoError::Service(CreateContactError::AlreadyExists(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateContactError::LimitExceeded(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateContactError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateContactError::AlreadyExists(ref cause) => write!(f, "{}", cause),
            CreateContactError::LimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateContactError {}
/// Errors returned by CreateGatewayGroup
#[derive(Debug, PartialEq)]
pub enum CreateGatewayGroupError {
    /// <p>The resource being created already exists.</p>
    AlreadyExists(String),
    /// <p>You are performing an action that would put you beyond your account's limits.</p>
    LimitExceeded(String),
}

impl CreateGatewayGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateGatewayGroupError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AlreadyExistsException" => {
                    return RusotoError::Service(CreateGatewayGroupError::AlreadyExists(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateGatewayGroupError::LimitExceeded(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateGatewayGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateGatewayGroupError::AlreadyExists(ref cause) => write!(f, "{}", cause),
            CreateGatewayGroupError::LimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateGatewayGroupError {}
/// Errors returned by CreateNetworkProfile
#[derive(Debug, PartialEq)]
pub enum CreateNetworkProfileError {
    /// <p>The resource being created already exists.</p>
    AlreadyExists(String),
    /// <p>There is a concurrent modification of resources.</p>
    ConcurrentModification(String),
    /// <p>The Certificate Authority can't issue or revoke a certificate.</p>
    InvalidCertificateAuthority(String),
    /// <p>The service linked role is locked for deletion. </p>
    InvalidServiceLinkedRoleState(String),
    /// <p>You are performing an action that would put you beyond your account's limits.</p>
    LimitExceeded(String),
}

impl CreateNetworkProfileError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateNetworkProfileError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AlreadyExistsException" => {
                    return RusotoError::Service(CreateNetworkProfileError::AlreadyExists(err.msg))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(CreateNetworkProfileError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "InvalidCertificateAuthorityException" => {
                    return RusotoError::Service(
                        CreateNetworkProfileError::InvalidCertificateAuthority(err.msg),
                    )
                }
                "InvalidServiceLinkedRoleStateException" => {
                    return RusotoError::Service(
                        CreateNetworkProfileError::InvalidServiceLinkedRoleState(err.msg),
                    )
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateNetworkProfileError::LimitExceeded(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateNetworkProfileError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateNetworkProfileError::AlreadyExists(ref cause) => write!(f, "{}", cause),
            CreateNetworkProfileError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            CreateNetworkProfileError::InvalidCertificateAuthority(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateNetworkProfileError::InvalidServiceLinkedRoleState(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateNetworkProfileError::LimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateNetworkProfileError {}
/// Errors returned by CreateProfile
#[derive(Debug, PartialEq)]
pub enum CreateProfileError {
    /// <p>The resource being created already exists.</p>
    AlreadyExists(String),
    /// <p>There is a concurrent modification of resources.</p>
    ConcurrentModification(String),
    /// <p>You are performing an action that would put you beyond your account's limits.</p>
    LimitExceeded(String),
}

impl CreateProfileError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateProfileError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AlreadyExistsException" => {
                    return RusotoError::Service(CreateProfileError::AlreadyExists(err.msg))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(CreateProfileError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateProfileError::LimitExceeded(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateProfileError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateProfileError::AlreadyExists(ref cause) => write!(f, "{}", cause),
            CreateProfileError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            CreateProfileError::LimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateProfileError {}
/// Errors returned by CreateRoom
#[derive(Debug, PartialEq)]
pub enum CreateRoomError {
    /// <p>The resource being created already exists.</p>
    AlreadyExists(String),
    /// <p>You are performing an action that would put you beyond your account's limits.</p>
    LimitExceeded(String),
}

impl CreateRoomError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateRoomError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AlreadyExistsException" => {
                    return RusotoError::Service(CreateRoomError::AlreadyExists(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateRoomError::LimitExceeded(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateRoomError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateRoomError::AlreadyExists(ref cause) => write!(f, "{}", cause),
            CreateRoomError::LimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateRoomError {}
/// Errors returned by CreateSkillGroup
#[derive(Debug, PartialEq)]
pub enum CreateSkillGroupError {
    /// <p>The resource being created already exists.</p>
    AlreadyExists(String),
    /// <p>There is a concurrent modification of resources.</p>
    ConcurrentModification(String),
    /// <p>You are performing an action that would put you beyond your account's limits.</p>
    LimitExceeded(String),
}

impl CreateSkillGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateSkillGroupError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AlreadyExistsException" => {
                    return RusotoError::Service(CreateSkillGroupError::AlreadyExists(err.msg))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(CreateSkillGroupError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateSkillGroupError::LimitExceeded(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateSkillGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateSkillGroupError::AlreadyExists(ref cause) => write!(f, "{}", cause),
            CreateSkillGroupError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            CreateSkillGroupError::LimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateSkillGroupError {}
/// Errors returned by CreateUser
#[derive(Debug, PartialEq)]
pub enum CreateUserError {
    /// <p>There is a concurrent modification of resources.</p>
    ConcurrentModification(String),
    /// <p>You are performing an action that would put you beyond your account's limits.</p>
    LimitExceeded(String),
    /// <p>The resource in the request is already in use.</p>
    ResourceInUse(String),
}

impl CreateUserError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateUserError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(CreateUserError::ConcurrentModification(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateUserError::LimitExceeded(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(CreateUserError::ResourceInUse(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateUserError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateUserError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            CreateUserError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateUserError::ResourceInUse(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateUserError {}
/// Errors returned by DeleteAddressBook
#[derive(Debug, PartialEq)]
pub enum DeleteAddressBookError {
    /// <p>There is a concurrent modification of resources.</p>
    ConcurrentModification(String),
    /// <p>The resource is not found.</p>
    NotFound(String),
}

impl DeleteAddressBookError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteAddressBookError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(DeleteAddressBookError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteAddressBookError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteAddressBookError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteAddressBookError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            DeleteAddressBookError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteAddressBookError {}
/// Errors returned by DeleteBusinessReportSchedule
#[derive(Debug, PartialEq)]
pub enum DeleteBusinessReportScheduleError {
    /// <p>There is a concurrent modification of resources.</p>
    ConcurrentModification(String),
    /// <p>The resource is not found.</p>
    NotFound(String),
}

impl DeleteBusinessReportScheduleError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteBusinessReportScheduleError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(
                        DeleteBusinessReportScheduleError::ConcurrentModification(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteBusinessReportScheduleError::NotFound(
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
impl fmt::Display for DeleteBusinessReportScheduleError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteBusinessReportScheduleError::ConcurrentModification(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteBusinessReportScheduleError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteBusinessReportScheduleError {}
/// Errors returned by DeleteConferenceProvider
#[derive(Debug, PartialEq)]
pub enum DeleteConferenceProviderError {
    /// <p>The resource is not found.</p>
    NotFound(String),
}

impl DeleteConferenceProviderError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteConferenceProviderError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "NotFoundException" => {
                    return RusotoError::Service(DeleteConferenceProviderError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteConferenceProviderError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteConferenceProviderError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteConferenceProviderError {}
/// Errors returned by DeleteContact
#[derive(Debug, PartialEq)]
pub enum DeleteContactError {
    /// <p>There is a concurrent modification of resources.</p>
    ConcurrentModification(String),
    /// <p>The resource is not found.</p>
    NotFound(String),
}

impl DeleteContactError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteContactError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(DeleteContactError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteContactError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteContactError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteContactError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            DeleteContactError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteContactError {}
/// Errors returned by DeleteDevice
#[derive(Debug, PartialEq)]
pub enum DeleteDeviceError {
    /// <p>There is a concurrent modification of resources.</p>
    ConcurrentModification(String),
    /// <p>The Certificate Authority can't issue or revoke a certificate.</p>
    InvalidCertificateAuthority(String),
    /// <p>The resource is not found.</p>
    NotFound(String),
}

impl DeleteDeviceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteDeviceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(DeleteDeviceError::ConcurrentModification(err.msg))
                }
                "InvalidCertificateAuthorityException" => {
                    return RusotoError::Service(DeleteDeviceError::InvalidCertificateAuthority(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteDeviceError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteDeviceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteDeviceError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            DeleteDeviceError::InvalidCertificateAuthority(ref cause) => write!(f, "{}", cause),
            DeleteDeviceError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteDeviceError {}
/// Errors returned by DeleteDeviceUsageData
#[derive(Debug, PartialEq)]
pub enum DeleteDeviceUsageDataError {
    /// <p>The request failed because this device is no longer registered and therefore no longer managed by this account.</p>
    DeviceNotRegistered(String),
    /// <p>You are performing an action that would put you beyond your account's limits.</p>
    LimitExceeded(String),
    /// <p>The resource is not found.</p>
    NotFound(String),
}

impl DeleteDeviceUsageDataError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteDeviceUsageDataError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DeviceNotRegisteredException" => {
                    return RusotoError::Service(DeleteDeviceUsageDataError::DeviceNotRegistered(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(DeleteDeviceUsageDataError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteDeviceUsageDataError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteDeviceUsageDataError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteDeviceUsageDataError::DeviceNotRegistered(ref cause) => write!(f, "{}", cause),
            DeleteDeviceUsageDataError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            DeleteDeviceUsageDataError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteDeviceUsageDataError {}
/// Errors returned by DeleteGatewayGroup
#[derive(Debug, PartialEq)]
pub enum DeleteGatewayGroupError {
    /// <p>Another resource is associated with the resource in the request.</p>
    ResourceAssociated(String),
}

impl DeleteGatewayGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteGatewayGroupError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceAssociatedException" => {
                    return RusotoError::Service(DeleteGatewayGroupError::ResourceAssociated(
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
impl fmt::Display for DeleteGatewayGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteGatewayGroupError::ResourceAssociated(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteGatewayGroupError {}
/// Errors returned by DeleteNetworkProfile
#[derive(Debug, PartialEq)]
pub enum DeleteNetworkProfileError {
    /// <p>There is a concurrent modification of resources.</p>
    ConcurrentModification(String),
    /// <p>The resource is not found.</p>
    NotFound(String),
    /// <p>The resource in the request is already in use.</p>
    ResourceInUse(String),
}

impl DeleteNetworkProfileError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteNetworkProfileError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(DeleteNetworkProfileError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteNetworkProfileError::NotFound(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(DeleteNetworkProfileError::ResourceInUse(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteNetworkProfileError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteNetworkProfileError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            DeleteNetworkProfileError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteNetworkProfileError::ResourceInUse(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteNetworkProfileError {}
/// Errors returned by DeleteProfile
#[derive(Debug, PartialEq)]
pub enum DeleteProfileError {
    /// <p>There is a concurrent modification of resources.</p>
    ConcurrentModification(String),
    /// <p>The resource is not found.</p>
    NotFound(String),
}

impl DeleteProfileError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteProfileError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(DeleteProfileError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteProfileError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteProfileError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteProfileError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            DeleteProfileError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteProfileError {}
/// Errors returned by DeleteRoom
#[derive(Debug, PartialEq)]
pub enum DeleteRoomError {
    /// <p>There is a concurrent modification of resources.</p>
    ConcurrentModification(String),
    /// <p>The resource is not found.</p>
    NotFound(String),
}

impl DeleteRoomError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteRoomError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(DeleteRoomError::ConcurrentModification(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteRoomError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteRoomError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteRoomError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            DeleteRoomError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteRoomError {}
/// Errors returned by DeleteRoomSkillParameter
#[derive(Debug, PartialEq)]
pub enum DeleteRoomSkillParameterError {
    /// <p>There is a concurrent modification of resources.</p>
    ConcurrentModification(String),
}

impl DeleteRoomSkillParameterError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteRoomSkillParameterError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(
                        DeleteRoomSkillParameterError::ConcurrentModification(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteRoomSkillParameterError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteRoomSkillParameterError::ConcurrentModification(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DeleteRoomSkillParameterError {}
/// Errors returned by DeleteSkillAuthorization
#[derive(Debug, PartialEq)]
pub enum DeleteSkillAuthorizationError {
    /// <p>There is a concurrent modification of resources.</p>
    ConcurrentModification(String),
    /// <p>The resource is not found.</p>
    NotFound(String),
}

impl DeleteSkillAuthorizationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteSkillAuthorizationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(
                        DeleteSkillAuthorizationError::ConcurrentModification(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteSkillAuthorizationError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteSkillAuthorizationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteSkillAuthorizationError::ConcurrentModification(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteSkillAuthorizationError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteSkillAuthorizationError {}
/// Errors returned by DeleteSkillGroup
#[derive(Debug, PartialEq)]
pub enum DeleteSkillGroupError {
    /// <p>There is a concurrent modification of resources.</p>
    ConcurrentModification(String),
    /// <p>The resource is not found.</p>
    NotFound(String),
}

impl DeleteSkillGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteSkillGroupError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(DeleteSkillGroupError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteSkillGroupError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteSkillGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteSkillGroupError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            DeleteSkillGroupError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteSkillGroupError {}
/// Errors returned by DeleteUser
#[derive(Debug, PartialEq)]
pub enum DeleteUserError {
    /// <p>There is a concurrent modification of resources.</p>
    ConcurrentModification(String),
    /// <p>The resource is not found.</p>
    NotFound(String),
}

impl DeleteUserError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteUserError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(DeleteUserError::ConcurrentModification(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteUserError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteUserError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteUserError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            DeleteUserError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteUserError {}
/// Errors returned by DisassociateContactFromAddressBook
#[derive(Debug, PartialEq)]
pub enum DisassociateContactFromAddressBookError {}

impl DisassociateContactFromAddressBookError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DisassociateContactFromAddressBookError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DisassociateContactFromAddressBookError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for DisassociateContactFromAddressBookError {}
/// Errors returned by DisassociateDeviceFromRoom
#[derive(Debug, PartialEq)]
pub enum DisassociateDeviceFromRoomError {
    /// <p>There is a concurrent modification of resources.</p>
    ConcurrentModification(String),
    /// <p>The request failed because this device is no longer registered and therefore no longer managed by this account.</p>
    DeviceNotRegistered(String),
}

impl DisassociateDeviceFromRoomError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DisassociateDeviceFromRoomError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(
                        DisassociateDeviceFromRoomError::ConcurrentModification(err.msg),
                    )
                }
                "DeviceNotRegisteredException" => {
                    return RusotoError::Service(
                        DisassociateDeviceFromRoomError::DeviceNotRegistered(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DisassociateDeviceFromRoomError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisassociateDeviceFromRoomError::ConcurrentModification(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociateDeviceFromRoomError::DeviceNotRegistered(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DisassociateDeviceFromRoomError {}
/// Errors returned by DisassociateSkillFromSkillGroup
#[derive(Debug, PartialEq)]
pub enum DisassociateSkillFromSkillGroupError {
    /// <p>There is a concurrent modification of resources.</p>
    ConcurrentModification(String),
    /// <p>The resource is not found.</p>
    NotFound(String),
}

impl DisassociateSkillFromSkillGroupError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DisassociateSkillFromSkillGroupError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(
                        DisassociateSkillFromSkillGroupError::ConcurrentModification(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(DisassociateSkillFromSkillGroupError::NotFound(
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
impl fmt::Display for DisassociateSkillFromSkillGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisassociateSkillFromSkillGroupError::ConcurrentModification(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociateSkillFromSkillGroupError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DisassociateSkillFromSkillGroupError {}
/// Errors returned by DisassociateSkillFromUsers
#[derive(Debug, PartialEq)]
pub enum DisassociateSkillFromUsersError {
    /// <p>There is a concurrent modification of resources.</p>
    ConcurrentModification(String),
    /// <p>The resource is not found.</p>
    NotFound(String),
}

impl DisassociateSkillFromUsersError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DisassociateSkillFromUsersError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(
                        DisassociateSkillFromUsersError::ConcurrentModification(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(DisassociateSkillFromUsersError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DisassociateSkillFromUsersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisassociateSkillFromUsersError::ConcurrentModification(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociateSkillFromUsersError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DisassociateSkillFromUsersError {}
/// Errors returned by DisassociateSkillGroupFromRoom
#[derive(Debug, PartialEq)]
pub enum DisassociateSkillGroupFromRoomError {
    /// <p>There is a concurrent modification of resources.</p>
    ConcurrentModification(String),
}

impl DisassociateSkillGroupFromRoomError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DisassociateSkillGroupFromRoomError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(
                        DisassociateSkillGroupFromRoomError::ConcurrentModification(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DisassociateSkillGroupFromRoomError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisassociateSkillGroupFromRoomError::ConcurrentModification(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DisassociateSkillGroupFromRoomError {}
/// Errors returned by ForgetSmartHomeAppliances
#[derive(Debug, PartialEq)]
pub enum ForgetSmartHomeAppliancesError {
    /// <p>The resource is not found.</p>
    NotFound(String),
}

impl ForgetSmartHomeAppliancesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ForgetSmartHomeAppliancesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "NotFoundException" => {
                    return RusotoError::Service(ForgetSmartHomeAppliancesError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ForgetSmartHomeAppliancesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ForgetSmartHomeAppliancesError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ForgetSmartHomeAppliancesError {}
/// Errors returned by GetAddressBook
#[derive(Debug, PartialEq)]
pub enum GetAddressBookError {
    /// <p>The resource is not found.</p>
    NotFound(String),
}

impl GetAddressBookError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetAddressBookError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "NotFoundException" => {
                    return RusotoError::Service(GetAddressBookError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetAddressBookError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetAddressBookError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetAddressBookError {}
/// Errors returned by GetConferencePreference
#[derive(Debug, PartialEq)]
pub enum GetConferencePreferenceError {
    /// <p>The resource is not found.</p>
    NotFound(String),
}

impl GetConferencePreferenceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetConferencePreferenceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "NotFoundException" => {
                    return RusotoError::Service(GetConferencePreferenceError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetConferencePreferenceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetConferencePreferenceError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetConferencePreferenceError {}
/// Errors returned by GetConferenceProvider
#[derive(Debug, PartialEq)]
pub enum GetConferenceProviderError {
    /// <p>The resource is not found.</p>
    NotFound(String),
}

impl GetConferenceProviderError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetConferenceProviderError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "NotFoundException" => {
                    return RusotoError::Service(GetConferenceProviderError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetConferenceProviderError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetConferenceProviderError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetConferenceProviderError {}
/// Errors returned by GetContact
#[derive(Debug, PartialEq)]
pub enum GetContactError {
    /// <p>The resource is not found.</p>
    NotFound(String),
}

impl GetContactError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetContactError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "NotFoundException" => {
                    return RusotoError::Service(GetContactError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetContactError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetContactError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetContactError {}
/// Errors returned by GetDevice
#[derive(Debug, PartialEq)]
pub enum GetDeviceError {
    /// <p>The resource is not found.</p>
    NotFound(String),
}

impl GetDeviceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetDeviceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "NotFoundException" => {
                    return RusotoError::Service(GetDeviceError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetDeviceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetDeviceError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetDeviceError {}
/// Errors returned by GetGateway
#[derive(Debug, PartialEq)]
pub enum GetGatewayError {
    /// <p>The resource is not found.</p>
    NotFound(String),
}

impl GetGatewayError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetGatewayError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "NotFoundException" => {
                    return RusotoError::Service(GetGatewayError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetGatewayError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetGatewayError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetGatewayError {}
/// Errors returned by GetGatewayGroup
#[derive(Debug, PartialEq)]
pub enum GetGatewayGroupError {
    /// <p>The resource is not found.</p>
    NotFound(String),
}

impl GetGatewayGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetGatewayGroupError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "NotFoundException" => {
                    return RusotoError::Service(GetGatewayGroupError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetGatewayGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetGatewayGroupError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetGatewayGroupError {}
/// Errors returned by GetInvitationConfiguration
#[derive(Debug, PartialEq)]
pub enum GetInvitationConfigurationError {
    /// <p>The resource is not found.</p>
    NotFound(String),
}

impl GetInvitationConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetInvitationConfigurationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "NotFoundException" => {
                    return RusotoError::Service(GetInvitationConfigurationError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetInvitationConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetInvitationConfigurationError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetInvitationConfigurationError {}
/// Errors returned by GetNetworkProfile
#[derive(Debug, PartialEq)]
pub enum GetNetworkProfileError {
    /// <p>A password in SecretsManager is in an invalid state.</p>
    InvalidSecretsManagerResource(String),
    /// <p>The resource is not found.</p>
    NotFound(String),
}

impl GetNetworkProfileError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetNetworkProfileError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidSecretsManagerResourceException" => {
                    return RusotoError::Service(
                        GetNetworkProfileError::InvalidSecretsManagerResource(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetNetworkProfileError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetNetworkProfileError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetNetworkProfileError::InvalidSecretsManagerResource(ref cause) => {
                write!(f, "{}", cause)
            }
            GetNetworkProfileError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetNetworkProfileError {}
/// Errors returned by GetProfile
#[derive(Debug, PartialEq)]
pub enum GetProfileError {
    /// <p>The resource is not found.</p>
    NotFound(String),
}

impl GetProfileError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetProfileError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "NotFoundException" => {
                    return RusotoError::Service(GetProfileError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetProfileError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetProfileError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetProfileError {}
/// Errors returned by GetRoom
#[derive(Debug, PartialEq)]
pub enum GetRoomError {
    /// <p>The resource is not found.</p>
    NotFound(String),
}

impl GetRoomError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetRoomError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "NotFoundException" => {
                    return RusotoError::Service(GetRoomError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetRoomError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetRoomError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetRoomError {}
/// Errors returned by GetRoomSkillParameter
#[derive(Debug, PartialEq)]
pub enum GetRoomSkillParameterError {
    /// <p>The resource is not found.</p>
    NotFound(String),
}

impl GetRoomSkillParameterError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetRoomSkillParameterError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "NotFoundException" => {
                    return RusotoError::Service(GetRoomSkillParameterError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetRoomSkillParameterError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetRoomSkillParameterError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetRoomSkillParameterError {}
/// Errors returned by GetSkillGroup
#[derive(Debug, PartialEq)]
pub enum GetSkillGroupError {
    /// <p>The resource is not found.</p>
    NotFound(String),
}

impl GetSkillGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetSkillGroupError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "NotFoundException" => {
                    return RusotoError::Service(GetSkillGroupError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetSkillGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetSkillGroupError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetSkillGroupError {}
/// Errors returned by ListBusinessReportSchedules
#[derive(Debug, PartialEq)]
pub enum ListBusinessReportSchedulesError {}

impl ListBusinessReportSchedulesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListBusinessReportSchedulesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListBusinessReportSchedulesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for ListBusinessReportSchedulesError {}
/// Errors returned by ListConferenceProviders
#[derive(Debug, PartialEq)]
pub enum ListConferenceProvidersError {}

impl ListConferenceProvidersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListConferenceProvidersError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListConferenceProvidersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for ListConferenceProvidersError {}
/// Errors returned by ListDeviceEvents
#[derive(Debug, PartialEq)]
pub enum ListDeviceEventsError {
    /// <p>The resource is not found.</p>
    NotFound(String),
}

impl ListDeviceEventsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListDeviceEventsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "NotFoundException" => {
                    return RusotoError::Service(ListDeviceEventsError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListDeviceEventsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListDeviceEventsError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListDeviceEventsError {}
/// Errors returned by ListGatewayGroups
#[derive(Debug, PartialEq)]
pub enum ListGatewayGroupsError {}

impl ListGatewayGroupsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListGatewayGroupsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListGatewayGroupsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for ListGatewayGroupsError {}
/// Errors returned by ListGateways
#[derive(Debug, PartialEq)]
pub enum ListGatewaysError {}

impl ListGatewaysError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListGatewaysError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListGatewaysError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for ListGatewaysError {}
/// Errors returned by ListSkills
#[derive(Debug, PartialEq)]
pub enum ListSkillsError {}

impl ListSkillsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListSkillsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListSkillsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for ListSkillsError {}
/// Errors returned by ListSkillsStoreCategories
#[derive(Debug, PartialEq)]
pub enum ListSkillsStoreCategoriesError {}

impl ListSkillsStoreCategoriesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListSkillsStoreCategoriesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListSkillsStoreCategoriesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for ListSkillsStoreCategoriesError {}
/// Errors returned by ListSkillsStoreSkillsByCategory
#[derive(Debug, PartialEq)]
pub enum ListSkillsStoreSkillsByCategoryError {}

impl ListSkillsStoreSkillsByCategoryError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListSkillsStoreSkillsByCategoryError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListSkillsStoreSkillsByCategoryError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for ListSkillsStoreSkillsByCategoryError {}
/// Errors returned by ListSmartHomeAppliances
#[derive(Debug, PartialEq)]
pub enum ListSmartHomeAppliancesError {
    /// <p>The resource is not found.</p>
    NotFound(String),
}

impl ListSmartHomeAppliancesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListSmartHomeAppliancesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "NotFoundException" => {
                    return RusotoError::Service(ListSmartHomeAppliancesError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListSmartHomeAppliancesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListSmartHomeAppliancesError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListSmartHomeAppliancesError {}
/// Errors returned by ListTags
#[derive(Debug, PartialEq)]
pub enum ListTagsError {
    /// <p>The resource is not found.</p>
    NotFound(String),
}

impl ListTagsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "NotFoundException" => {
                    return RusotoError::Service(ListTagsError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListTagsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListTagsError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTagsError {}
/// Errors returned by PutConferencePreference
#[derive(Debug, PartialEq)]
pub enum PutConferencePreferenceError {
    /// <p>The resource is not found.</p>
    NotFound(String),
}

impl PutConferencePreferenceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutConferencePreferenceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "NotFoundException" => {
                    return RusotoError::Service(PutConferencePreferenceError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for PutConferencePreferenceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutConferencePreferenceError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutConferencePreferenceError {}
/// Errors returned by PutInvitationConfiguration
#[derive(Debug, PartialEq)]
pub enum PutInvitationConfigurationError {
    /// <p>There is a concurrent modification of resources.</p>
    ConcurrentModification(String),
    /// <p>The resource is not found.</p>
    NotFound(String),
}

impl PutInvitationConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<PutInvitationConfigurationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(
                        PutInvitationConfigurationError::ConcurrentModification(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(PutInvitationConfigurationError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for PutInvitationConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutInvitationConfigurationError::ConcurrentModification(ref cause) => {
                write!(f, "{}", cause)
            }
            PutInvitationConfigurationError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutInvitationConfigurationError {}
/// Errors returned by PutRoomSkillParameter
#[derive(Debug, PartialEq)]
pub enum PutRoomSkillParameterError {
    /// <p>There is a concurrent modification of resources.</p>
    ConcurrentModification(String),
}

impl PutRoomSkillParameterError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutRoomSkillParameterError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(
                        PutRoomSkillParameterError::ConcurrentModification(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for PutRoomSkillParameterError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutRoomSkillParameterError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutRoomSkillParameterError {}
/// Errors returned by PutSkillAuthorization
#[derive(Debug, PartialEq)]
pub enum PutSkillAuthorizationError {
    /// <p>There is a concurrent modification of resources.</p>
    ConcurrentModification(String),
    /// <p>The caller has no permissions to operate on the resource involved in the API call.</p>
    Unauthorized(String),
}

impl PutSkillAuthorizationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutSkillAuthorizationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(
                        PutSkillAuthorizationError::ConcurrentModification(err.msg),
                    )
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(PutSkillAuthorizationError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for PutSkillAuthorizationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutSkillAuthorizationError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            PutSkillAuthorizationError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutSkillAuthorizationError {}
/// Errors returned by RegisterAVSDevice
#[derive(Debug, PartialEq)]
pub enum RegisterAVSDeviceError {
    /// <p>There is a concurrent modification of resources.</p>
    ConcurrentModification(String),
    /// <p>The device is in an invalid state.</p>
    InvalidDevice(String),
    /// <p>You are performing an action that would put you beyond your account's limits.</p>
    LimitExceeded(String),
}

impl RegisterAVSDeviceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RegisterAVSDeviceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(RegisterAVSDeviceError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "InvalidDeviceException" => {
                    return RusotoError::Service(RegisterAVSDeviceError::InvalidDevice(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(RegisterAVSDeviceError::LimitExceeded(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for RegisterAVSDeviceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RegisterAVSDeviceError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            RegisterAVSDeviceError::InvalidDevice(ref cause) => write!(f, "{}", cause),
            RegisterAVSDeviceError::LimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RegisterAVSDeviceError {}
/// Errors returned by RejectSkill
#[derive(Debug, PartialEq)]
pub enum RejectSkillError {
    /// <p>There is a concurrent modification of resources.</p>
    ConcurrentModification(String),
    /// <p>The resource is not found.</p>
    NotFound(String),
}

impl RejectSkillError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RejectSkillError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(RejectSkillError::ConcurrentModification(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(RejectSkillError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for RejectSkillError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RejectSkillError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            RejectSkillError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RejectSkillError {}
/// Errors returned by ResolveRoom
#[derive(Debug, PartialEq)]
pub enum ResolveRoomError {
    /// <p>The resource is not found.</p>
    NotFound(String),
}

impl ResolveRoomError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ResolveRoomError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "NotFoundException" => {
                    return RusotoError::Service(ResolveRoomError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ResolveRoomError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ResolveRoomError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ResolveRoomError {}
/// Errors returned by RevokeInvitation
#[derive(Debug, PartialEq)]
pub enum RevokeInvitationError {
    /// <p>There is a concurrent modification of resources.</p>
    ConcurrentModification(String),
    /// <p>The resource is not found.</p>
    NotFound(String),
}

impl RevokeInvitationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RevokeInvitationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(RevokeInvitationError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(RevokeInvitationError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for RevokeInvitationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RevokeInvitationError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            RevokeInvitationError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RevokeInvitationError {}
/// Errors returned by SearchAddressBooks
#[derive(Debug, PartialEq)]
pub enum SearchAddressBooksError {}

impl SearchAddressBooksError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SearchAddressBooksError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for SearchAddressBooksError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for SearchAddressBooksError {}
/// Errors returned by SearchContacts
#[derive(Debug, PartialEq)]
pub enum SearchContactsError {}

impl SearchContactsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SearchContactsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for SearchContactsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for SearchContactsError {}
/// Errors returned by SearchDevices
#[derive(Debug, PartialEq)]
pub enum SearchDevicesError {}

impl SearchDevicesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SearchDevicesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for SearchDevicesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for SearchDevicesError {}
/// Errors returned by SearchNetworkProfiles
#[derive(Debug, PartialEq)]
pub enum SearchNetworkProfilesError {}

impl SearchNetworkProfilesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SearchNetworkProfilesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for SearchNetworkProfilesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for SearchNetworkProfilesError {}
/// Errors returned by SearchProfiles
#[derive(Debug, PartialEq)]
pub enum SearchProfilesError {}

impl SearchProfilesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SearchProfilesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for SearchProfilesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for SearchProfilesError {}
/// Errors returned by SearchRooms
#[derive(Debug, PartialEq)]
pub enum SearchRoomsError {}

impl SearchRoomsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SearchRoomsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for SearchRoomsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for SearchRoomsError {}
/// Errors returned by SearchSkillGroups
#[derive(Debug, PartialEq)]
pub enum SearchSkillGroupsError {}

impl SearchSkillGroupsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SearchSkillGroupsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for SearchSkillGroupsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for SearchSkillGroupsError {}
/// Errors returned by SearchUsers
#[derive(Debug, PartialEq)]
pub enum SearchUsersError {}

impl SearchUsersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SearchUsersError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for SearchUsersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for SearchUsersError {}
/// Errors returned by SendAnnouncement
#[derive(Debug, PartialEq)]
pub enum SendAnnouncementError {
    /// <p>The resource being created already exists.</p>
    AlreadyExists(String),
    /// <p>You are performing an action that would put you beyond your account's limits.</p>
    LimitExceeded(String),
}

impl SendAnnouncementError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SendAnnouncementError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AlreadyExistsException" => {
                    return RusotoError::Service(SendAnnouncementError::AlreadyExists(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(SendAnnouncementError::LimitExceeded(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for SendAnnouncementError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SendAnnouncementError::AlreadyExists(ref cause) => write!(f, "{}", cause),
            SendAnnouncementError::LimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for SendAnnouncementError {}
/// Errors returned by SendInvitation
#[derive(Debug, PartialEq)]
pub enum SendInvitationError {
    /// <p>There is a concurrent modification of resources.</p>
    ConcurrentModification(String),
    /// <p>The attempt to update a user is invalid due to the user's current status.</p>
    InvalidUserStatus(String),
    /// <p>The resource is not found.</p>
    NotFound(String),
}

impl SendInvitationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SendInvitationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(SendInvitationError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "InvalidUserStatusException" => {
                    return RusotoError::Service(SendInvitationError::InvalidUserStatus(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(SendInvitationError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for SendInvitationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SendInvitationError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            SendInvitationError::InvalidUserStatus(ref cause) => write!(f, "{}", cause),
            SendInvitationError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for SendInvitationError {}
/// Errors returned by StartDeviceSync
#[derive(Debug, PartialEq)]
pub enum StartDeviceSyncError {
    /// <p>The request failed because this device is no longer registered and therefore no longer managed by this account.</p>
    DeviceNotRegistered(String),
}

impl StartDeviceSyncError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartDeviceSyncError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DeviceNotRegisteredException" => {
                    return RusotoError::Service(StartDeviceSyncError::DeviceNotRegistered(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for StartDeviceSyncError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartDeviceSyncError::DeviceNotRegistered(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StartDeviceSyncError {}
/// Errors returned by StartSmartHomeApplianceDiscovery
#[derive(Debug, PartialEq)]
pub enum StartSmartHomeApplianceDiscoveryError {
    /// <p>The resource is not found.</p>
    NotFound(String),
}

impl StartSmartHomeApplianceDiscoveryError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<StartSmartHomeApplianceDiscoveryError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "NotFoundException" => {
                    return RusotoError::Service(StartSmartHomeApplianceDiscoveryError::NotFound(
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
impl fmt::Display for StartSmartHomeApplianceDiscoveryError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartSmartHomeApplianceDiscoveryError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StartSmartHomeApplianceDiscoveryError {}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>The resource is not found.</p>
    NotFound(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "NotFoundException" => {
                    return RusotoError::Service(TagResourceError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for TagResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TagResourceError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TagResourceError {}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>The resource is not found.</p>
    NotFound(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "NotFoundException" => {
                    return RusotoError::Service(UntagResourceError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UntagResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UntagResourceError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UntagResourceError {}
/// Errors returned by UpdateAddressBook
#[derive(Debug, PartialEq)]
pub enum UpdateAddressBookError {
    /// <p>There is a concurrent modification of resources.</p>
    ConcurrentModification(String),
    /// <p>The name sent in the request is already in use.</p>
    NameInUse(String),
    /// <p>The resource is not found.</p>
    NotFound(String),
}

impl UpdateAddressBookError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateAddressBookError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(UpdateAddressBookError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "NameInUseException" => {
                    return RusotoError::Service(UpdateAddressBookError::NameInUse(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateAddressBookError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateAddressBookError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateAddressBookError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            UpdateAddressBookError::NameInUse(ref cause) => write!(f, "{}", cause),
            UpdateAddressBookError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateAddressBookError {}
/// Errors returned by UpdateBusinessReportSchedule
#[derive(Debug, PartialEq)]
pub enum UpdateBusinessReportScheduleError {
    /// <p>There is a concurrent modification of resources.</p>
    ConcurrentModification(String),
    /// <p>The resource is not found.</p>
    NotFound(String),
}

impl UpdateBusinessReportScheduleError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateBusinessReportScheduleError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(
                        UpdateBusinessReportScheduleError::ConcurrentModification(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateBusinessReportScheduleError::NotFound(
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
impl fmt::Display for UpdateBusinessReportScheduleError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateBusinessReportScheduleError::ConcurrentModification(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateBusinessReportScheduleError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateBusinessReportScheduleError {}
/// Errors returned by UpdateConferenceProvider
#[derive(Debug, PartialEq)]
pub enum UpdateConferenceProviderError {
    /// <p>The resource is not found.</p>
    NotFound(String),
}

impl UpdateConferenceProviderError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateConferenceProviderError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "NotFoundException" => {
                    return RusotoError::Service(UpdateConferenceProviderError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateConferenceProviderError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateConferenceProviderError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateConferenceProviderError {}
/// Errors returned by UpdateContact
#[derive(Debug, PartialEq)]
pub enum UpdateContactError {
    /// <p>There is a concurrent modification of resources.</p>
    ConcurrentModification(String),
    /// <p>The resource is not found.</p>
    NotFound(String),
}

impl UpdateContactError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateContactError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(UpdateContactError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateContactError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateContactError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateContactError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            UpdateContactError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateContactError {}
/// Errors returned by UpdateDevice
#[derive(Debug, PartialEq)]
pub enum UpdateDeviceError {
    /// <p>There is a concurrent modification of resources.</p>
    ConcurrentModification(String),
    /// <p>The request failed because this device is no longer registered and therefore no longer managed by this account.</p>
    DeviceNotRegistered(String),
    /// <p>The resource is not found.</p>
    NotFound(String),
}

impl UpdateDeviceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateDeviceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(UpdateDeviceError::ConcurrentModification(err.msg))
                }
                "DeviceNotRegisteredException" => {
                    return RusotoError::Service(UpdateDeviceError::DeviceNotRegistered(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateDeviceError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateDeviceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateDeviceError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            UpdateDeviceError::DeviceNotRegistered(ref cause) => write!(f, "{}", cause),
            UpdateDeviceError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateDeviceError {}
/// Errors returned by UpdateGateway
#[derive(Debug, PartialEq)]
pub enum UpdateGatewayError {
    /// <p>The name sent in the request is already in use.</p>
    NameInUse(String),
    /// <p>The resource is not found.</p>
    NotFound(String),
}

impl UpdateGatewayError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateGatewayError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "NameInUseException" => {
                    return RusotoError::Service(UpdateGatewayError::NameInUse(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateGatewayError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateGatewayError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateGatewayError::NameInUse(ref cause) => write!(f, "{}", cause),
            UpdateGatewayError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateGatewayError {}
/// Errors returned by UpdateGatewayGroup
#[derive(Debug, PartialEq)]
pub enum UpdateGatewayGroupError {
    /// <p>The name sent in the request is already in use.</p>
    NameInUse(String),
    /// <p>The resource is not found.</p>
    NotFound(String),
}

impl UpdateGatewayGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateGatewayGroupError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "NameInUseException" => {
                    return RusotoError::Service(UpdateGatewayGroupError::NameInUse(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateGatewayGroupError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateGatewayGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateGatewayGroupError::NameInUse(ref cause) => write!(f, "{}", cause),
            UpdateGatewayGroupError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateGatewayGroupError {}
/// Errors returned by UpdateNetworkProfile
#[derive(Debug, PartialEq)]
pub enum UpdateNetworkProfileError {
    /// <p>There is a concurrent modification of resources.</p>
    ConcurrentModification(String),
    /// <p>The Certificate Authority can't issue or revoke a certificate.</p>
    InvalidCertificateAuthority(String),
    /// <p>A password in SecretsManager is in an invalid state.</p>
    InvalidSecretsManagerResource(String),
    /// <p>The name sent in the request is already in use.</p>
    NameInUse(String),
    /// <p>The resource is not found.</p>
    NotFound(String),
}

impl UpdateNetworkProfileError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateNetworkProfileError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(UpdateNetworkProfileError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "InvalidCertificateAuthorityException" => {
                    return RusotoError::Service(
                        UpdateNetworkProfileError::InvalidCertificateAuthority(err.msg),
                    )
                }
                "InvalidSecretsManagerResourceException" => {
                    return RusotoError::Service(
                        UpdateNetworkProfileError::InvalidSecretsManagerResource(err.msg),
                    )
                }
                "NameInUseException" => {
                    return RusotoError::Service(UpdateNetworkProfileError::NameInUse(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateNetworkProfileError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateNetworkProfileError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateNetworkProfileError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            UpdateNetworkProfileError::InvalidCertificateAuthority(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateNetworkProfileError::InvalidSecretsManagerResource(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateNetworkProfileError::NameInUse(ref cause) => write!(f, "{}", cause),
            UpdateNetworkProfileError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateNetworkProfileError {}
/// Errors returned by UpdateProfile
#[derive(Debug, PartialEq)]
pub enum UpdateProfileError {
    /// <p>There is a concurrent modification of resources.</p>
    ConcurrentModification(String),
    /// <p>The name sent in the request is already in use.</p>
    NameInUse(String),
    /// <p>The resource is not found.</p>
    NotFound(String),
}

impl UpdateProfileError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateProfileError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(UpdateProfileError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "NameInUseException" => {
                    return RusotoError::Service(UpdateProfileError::NameInUse(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateProfileError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateProfileError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateProfileError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            UpdateProfileError::NameInUse(ref cause) => write!(f, "{}", cause),
            UpdateProfileError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateProfileError {}
/// Errors returned by UpdateRoom
#[derive(Debug, PartialEq)]
pub enum UpdateRoomError {
    /// <p>The name sent in the request is already in use.</p>
    NameInUse(String),
    /// <p>The resource is not found.</p>
    NotFound(String),
}

impl UpdateRoomError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateRoomError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "NameInUseException" => {
                    return RusotoError::Service(UpdateRoomError::NameInUse(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateRoomError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateRoomError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateRoomError::NameInUse(ref cause) => write!(f, "{}", cause),
            UpdateRoomError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateRoomError {}
/// Errors returned by UpdateSkillGroup
#[derive(Debug, PartialEq)]
pub enum UpdateSkillGroupError {
    /// <p>There is a concurrent modification of resources.</p>
    ConcurrentModification(String),
    /// <p>The name sent in the request is already in use.</p>
    NameInUse(String),
    /// <p>The resource is not found.</p>
    NotFound(String),
}

impl UpdateSkillGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateSkillGroupError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(UpdateSkillGroupError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "NameInUseException" => {
                    return RusotoError::Service(UpdateSkillGroupError::NameInUse(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateSkillGroupError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateSkillGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateSkillGroupError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            UpdateSkillGroupError::NameInUse(ref cause) => write!(f, "{}", cause),
            UpdateSkillGroupError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateSkillGroupError {}
/// Trait representing the capabilities of the Alexa For Business API. Alexa For Business clients implement this trait.
#[async_trait]
pub trait AlexaForBusiness {
    /// <p>Associates a skill with the organization under the customer's AWS account. If a skill is private, the user implicitly accepts access to this skill during enablement.</p>
    async fn approve_skill(
        &self,
        input: ApproveSkillRequest,
    ) -> Result<ApproveSkillResponse, RusotoError<ApproveSkillError>>;

    /// <p>Associates a contact with a given address book.</p>
    async fn associate_contact_with_address_book(
        &self,
        input: AssociateContactWithAddressBookRequest,
    ) -> Result<
        AssociateContactWithAddressBookResponse,
        RusotoError<AssociateContactWithAddressBookError>,
    >;

    /// <p>Associates a device with the specified network profile.</p>
    async fn associate_device_with_network_profile(
        &self,
        input: AssociateDeviceWithNetworkProfileRequest,
    ) -> Result<
        AssociateDeviceWithNetworkProfileResponse,
        RusotoError<AssociateDeviceWithNetworkProfileError>,
    >;

    /// <p>Associates a device with a given room. This applies all the settings from the room profile to the device, and all the skills in any skill groups added to that room. This operation requires the device to be online, or else a manual sync is required. </p>
    async fn associate_device_with_room(
        &self,
        input: AssociateDeviceWithRoomRequest,
    ) -> Result<AssociateDeviceWithRoomResponse, RusotoError<AssociateDeviceWithRoomError>>;

    /// <p>Associates a skill group with a given room. This enables all skills in the associated skill group on all devices in the room.</p>
    async fn associate_skill_group_with_room(
        &self,
        input: AssociateSkillGroupWithRoomRequest,
    ) -> Result<AssociateSkillGroupWithRoomResponse, RusotoError<AssociateSkillGroupWithRoomError>>;

    /// <p>Associates a skill with a skill group.</p>
    async fn associate_skill_with_skill_group(
        &self,
        input: AssociateSkillWithSkillGroupRequest,
    ) -> Result<AssociateSkillWithSkillGroupResponse, RusotoError<AssociateSkillWithSkillGroupError>>;

    /// <p>Makes a private skill available for enrolled users to enable on their devices.</p>
    async fn associate_skill_with_users(
        &self,
        input: AssociateSkillWithUsersRequest,
    ) -> Result<AssociateSkillWithUsersResponse, RusotoError<AssociateSkillWithUsersError>>;

    /// <p>Creates an address book with the specified details.</p>
    async fn create_address_book(
        &self,
        input: CreateAddressBookRequest,
    ) -> Result<CreateAddressBookResponse, RusotoError<CreateAddressBookError>>;

    /// <p>Creates a recurring schedule for usage reports to deliver to the specified S3 location with a specified daily or weekly interval.</p>
    async fn create_business_report_schedule(
        &self,
        input: CreateBusinessReportScheduleRequest,
    ) -> Result<CreateBusinessReportScheduleResponse, RusotoError<CreateBusinessReportScheduleError>>;

    /// <p>Adds a new conference provider under the user's AWS account.</p>
    async fn create_conference_provider(
        &self,
        input: CreateConferenceProviderRequest,
    ) -> Result<CreateConferenceProviderResponse, RusotoError<CreateConferenceProviderError>>;

    /// <p>Creates a contact with the specified details.</p>
    async fn create_contact(
        &self,
        input: CreateContactRequest,
    ) -> Result<CreateContactResponse, RusotoError<CreateContactError>>;

    /// <p>Creates a gateway group with the specified details.</p>
    async fn create_gateway_group(
        &self,
        input: CreateGatewayGroupRequest,
    ) -> Result<CreateGatewayGroupResponse, RusotoError<CreateGatewayGroupError>>;

    /// <p>Creates a network profile with the specified details.</p>
    async fn create_network_profile(
        &self,
        input: CreateNetworkProfileRequest,
    ) -> Result<CreateNetworkProfileResponse, RusotoError<CreateNetworkProfileError>>;

    /// <p>Creates a new room profile with the specified details.</p>
    async fn create_profile(
        &self,
        input: CreateProfileRequest,
    ) -> Result<CreateProfileResponse, RusotoError<CreateProfileError>>;

    /// <p>Creates a room with the specified details.</p>
    async fn create_room(
        &self,
        input: CreateRoomRequest,
    ) -> Result<CreateRoomResponse, RusotoError<CreateRoomError>>;

    /// <p>Creates a skill group with a specified name and description.</p>
    async fn create_skill_group(
        &self,
        input: CreateSkillGroupRequest,
    ) -> Result<CreateSkillGroupResponse, RusotoError<CreateSkillGroupError>>;

    /// <p>Creates a user.</p>
    async fn create_user(
        &self,
        input: CreateUserRequest,
    ) -> Result<CreateUserResponse, RusotoError<CreateUserError>>;

    /// <p>Deletes an address book by the address book ARN.</p>
    async fn delete_address_book(
        &self,
        input: DeleteAddressBookRequest,
    ) -> Result<DeleteAddressBookResponse, RusotoError<DeleteAddressBookError>>;

    /// <p>Deletes the recurring report delivery schedule with the specified schedule ARN.</p>
    async fn delete_business_report_schedule(
        &self,
        input: DeleteBusinessReportScheduleRequest,
    ) -> Result<DeleteBusinessReportScheduleResponse, RusotoError<DeleteBusinessReportScheduleError>>;

    /// <p>Deletes a conference provider.</p>
    async fn delete_conference_provider(
        &self,
        input: DeleteConferenceProviderRequest,
    ) -> Result<DeleteConferenceProviderResponse, RusotoError<DeleteConferenceProviderError>>;

    /// <p>Deletes a contact by the contact ARN.</p>
    async fn delete_contact(
        &self,
        input: DeleteContactRequest,
    ) -> Result<DeleteContactResponse, RusotoError<DeleteContactError>>;

    /// <p>Removes a device from Alexa For Business.</p>
    async fn delete_device(
        &self,
        input: DeleteDeviceRequest,
    ) -> Result<DeleteDeviceResponse, RusotoError<DeleteDeviceError>>;

    /// <p>When this action is called for a specified shared device, it allows authorized users to delete the device's entire previous history of voice input data and associated response data. This action can be called once every 24 hours for a specific shared device.</p>
    async fn delete_device_usage_data(
        &self,
        input: DeleteDeviceUsageDataRequest,
    ) -> Result<DeleteDeviceUsageDataResponse, RusotoError<DeleteDeviceUsageDataError>>;

    /// <p>Deletes a gateway group.</p>
    async fn delete_gateway_group(
        &self,
        input: DeleteGatewayGroupRequest,
    ) -> Result<DeleteGatewayGroupResponse, RusotoError<DeleteGatewayGroupError>>;

    /// <p>Deletes a network profile by the network profile ARN.</p>
    async fn delete_network_profile(
        &self,
        input: DeleteNetworkProfileRequest,
    ) -> Result<DeleteNetworkProfileResponse, RusotoError<DeleteNetworkProfileError>>;

    /// <p>Deletes a room profile by the profile ARN.</p>
    async fn delete_profile(
        &self,
        input: DeleteProfileRequest,
    ) -> Result<DeleteProfileResponse, RusotoError<DeleteProfileError>>;

    /// <p>Deletes a room by the room ARN.</p>
    async fn delete_room(
        &self,
        input: DeleteRoomRequest,
    ) -> Result<DeleteRoomResponse, RusotoError<DeleteRoomError>>;

    /// <p>Deletes room skill parameter details by room, skill, and parameter key ID.</p>
    async fn delete_room_skill_parameter(
        &self,
        input: DeleteRoomSkillParameterRequest,
    ) -> Result<DeleteRoomSkillParameterResponse, RusotoError<DeleteRoomSkillParameterError>>;

    /// <p>Unlinks a third-party account from a skill.</p>
    async fn delete_skill_authorization(
        &self,
        input: DeleteSkillAuthorizationRequest,
    ) -> Result<DeleteSkillAuthorizationResponse, RusotoError<DeleteSkillAuthorizationError>>;

    /// <p>Deletes a skill group by skill group ARN.</p>
    async fn delete_skill_group(
        &self,
        input: DeleteSkillGroupRequest,
    ) -> Result<DeleteSkillGroupResponse, RusotoError<DeleteSkillGroupError>>;

    /// <p>Deletes a specified user by user ARN and enrollment ARN.</p>
    async fn delete_user(
        &self,
        input: DeleteUserRequest,
    ) -> Result<DeleteUserResponse, RusotoError<DeleteUserError>>;

    /// <p>Disassociates a contact from a given address book.</p>
    async fn disassociate_contact_from_address_book(
        &self,
        input: DisassociateContactFromAddressBookRequest,
    ) -> Result<
        DisassociateContactFromAddressBookResponse,
        RusotoError<DisassociateContactFromAddressBookError>,
    >;

    /// <p>Disassociates a device from its current room. The device continues to be connected to the Wi-Fi network and is still registered to the account. The device settings and skills are removed from the room.</p>
    async fn disassociate_device_from_room(
        &self,
        input: DisassociateDeviceFromRoomRequest,
    ) -> Result<DisassociateDeviceFromRoomResponse, RusotoError<DisassociateDeviceFromRoomError>>;

    /// <p>Disassociates a skill from a skill group.</p>
    async fn disassociate_skill_from_skill_group(
        &self,
        input: DisassociateSkillFromSkillGroupRequest,
    ) -> Result<
        DisassociateSkillFromSkillGroupResponse,
        RusotoError<DisassociateSkillFromSkillGroupError>,
    >;

    /// <p>Makes a private skill unavailable for enrolled users and prevents them from enabling it on their devices.</p>
    async fn disassociate_skill_from_users(
        &self,
        input: DisassociateSkillFromUsersRequest,
    ) -> Result<DisassociateSkillFromUsersResponse, RusotoError<DisassociateSkillFromUsersError>>;

    /// <p>Disassociates a skill group from a specified room. This disables all skills in the skill group on all devices in the room.</p>
    async fn disassociate_skill_group_from_room(
        &self,
        input: DisassociateSkillGroupFromRoomRequest,
    ) -> Result<
        DisassociateSkillGroupFromRoomResponse,
        RusotoError<DisassociateSkillGroupFromRoomError>,
    >;

    /// <p>Forgets smart home appliances associated to a room.</p>
    async fn forget_smart_home_appliances(
        &self,
        input: ForgetSmartHomeAppliancesRequest,
    ) -> Result<ForgetSmartHomeAppliancesResponse, RusotoError<ForgetSmartHomeAppliancesError>>;

    /// <p>Gets address the book details by the address book ARN.</p>
    async fn get_address_book(
        &self,
        input: GetAddressBookRequest,
    ) -> Result<GetAddressBookResponse, RusotoError<GetAddressBookError>>;

    /// <p>Retrieves the existing conference preferences.</p>
    async fn get_conference_preference(
        &self,
    ) -> Result<GetConferencePreferenceResponse, RusotoError<GetConferencePreferenceError>>;

    /// <p>Gets details about a specific conference provider.</p>
    async fn get_conference_provider(
        &self,
        input: GetConferenceProviderRequest,
    ) -> Result<GetConferenceProviderResponse, RusotoError<GetConferenceProviderError>>;

    /// <p>Gets the contact details by the contact ARN.</p>
    async fn get_contact(
        &self,
        input: GetContactRequest,
    ) -> Result<GetContactResponse, RusotoError<GetContactError>>;

    /// <p>Gets the details of a device by device ARN.</p>
    async fn get_device(
        &self,
        input: GetDeviceRequest,
    ) -> Result<GetDeviceResponse, RusotoError<GetDeviceError>>;

    /// <p>Retrieves the details of a gateway.</p>
    async fn get_gateway(
        &self,
        input: GetGatewayRequest,
    ) -> Result<GetGatewayResponse, RusotoError<GetGatewayError>>;

    /// <p>Retrieves the details of a gateway group.</p>
    async fn get_gateway_group(
        &self,
        input: GetGatewayGroupRequest,
    ) -> Result<GetGatewayGroupResponse, RusotoError<GetGatewayGroupError>>;

    /// <p>Retrieves the configured values for the user enrollment invitation email template.</p>
    async fn get_invitation_configuration(
        &self,
    ) -> Result<GetInvitationConfigurationResponse, RusotoError<GetInvitationConfigurationError>>;

    /// <p>Gets the network profile details by the network profile ARN.</p>
    async fn get_network_profile(
        &self,
        input: GetNetworkProfileRequest,
    ) -> Result<GetNetworkProfileResponse, RusotoError<GetNetworkProfileError>>;

    /// <p>Gets the details of a room profile by profile ARN.</p>
    async fn get_profile(
        &self,
        input: GetProfileRequest,
    ) -> Result<GetProfileResponse, RusotoError<GetProfileError>>;

    /// <p>Gets room details by room ARN.</p>
    async fn get_room(
        &self,
        input: GetRoomRequest,
    ) -> Result<GetRoomResponse, RusotoError<GetRoomError>>;

    /// <p>Gets room skill parameter details by room, skill, and parameter key ARN.</p>
    async fn get_room_skill_parameter(
        &self,
        input: GetRoomSkillParameterRequest,
    ) -> Result<GetRoomSkillParameterResponse, RusotoError<GetRoomSkillParameterError>>;

    /// <p>Gets skill group details by skill group ARN.</p>
    async fn get_skill_group(
        &self,
        input: GetSkillGroupRequest,
    ) -> Result<GetSkillGroupResponse, RusotoError<GetSkillGroupError>>;

    /// <p>Lists the details of the schedules that a user configured. A download URL of the report associated with each schedule is returned every time this action is called. A new download URL is returned each time, and is valid for 24 hours.</p>
    async fn list_business_report_schedules(
        &self,
        input: ListBusinessReportSchedulesRequest,
    ) -> Result<ListBusinessReportSchedulesResponse, RusotoError<ListBusinessReportSchedulesError>>;

    /// <p>Lists conference providers under a specific AWS account.</p>
    async fn list_conference_providers(
        &self,
        input: ListConferenceProvidersRequest,
    ) -> Result<ListConferenceProvidersResponse, RusotoError<ListConferenceProvidersError>>;

    /// <p>Lists the device event history, including device connection status, for up to 30 days.</p>
    async fn list_device_events(
        &self,
        input: ListDeviceEventsRequest,
    ) -> Result<ListDeviceEventsResponse, RusotoError<ListDeviceEventsError>>;

    /// <p>Retrieves a list of gateway group summaries. Use GetGatewayGroup to retrieve details of a specific gateway group.</p>
    async fn list_gateway_groups(
        &self,
        input: ListGatewayGroupsRequest,
    ) -> Result<ListGatewayGroupsResponse, RusotoError<ListGatewayGroupsError>>;

    /// <p>Retrieves a list of gateway summaries. Use GetGateway to retrieve details of a specific gateway. An optional gateway group ARN can be provided to only retrieve gateway summaries of gateways that are associated with that gateway group ARN.</p>
    async fn list_gateways(
        &self,
        input: ListGatewaysRequest,
    ) -> Result<ListGatewaysResponse, RusotoError<ListGatewaysError>>;

    /// <p>Lists all enabled skills in a specific skill group.</p>
    async fn list_skills(
        &self,
        input: ListSkillsRequest,
    ) -> Result<ListSkillsResponse, RusotoError<ListSkillsError>>;

    /// <p>Lists all categories in the Alexa skill store.</p>
    async fn list_skills_store_categories(
        &self,
        input: ListSkillsStoreCategoriesRequest,
    ) -> Result<ListSkillsStoreCategoriesResponse, RusotoError<ListSkillsStoreCategoriesError>>;

    /// <p>Lists all skills in the Alexa skill store by category.</p>
    async fn list_skills_store_skills_by_category(
        &self,
        input: ListSkillsStoreSkillsByCategoryRequest,
    ) -> Result<
        ListSkillsStoreSkillsByCategoryResponse,
        RusotoError<ListSkillsStoreSkillsByCategoryError>,
    >;

    /// <p>Lists all of the smart home appliances associated with a room.</p>
    async fn list_smart_home_appliances(
        &self,
        input: ListSmartHomeAppliancesRequest,
    ) -> Result<ListSmartHomeAppliancesResponse, RusotoError<ListSmartHomeAppliancesError>>;

    /// <p>Lists all tags for the specified resource.</p>
    async fn list_tags(
        &self,
        input: ListTagsRequest,
    ) -> Result<ListTagsResponse, RusotoError<ListTagsError>>;

    /// <p>Sets the conference preferences on a specific conference provider at the account level.</p>
    async fn put_conference_preference(
        &self,
        input: PutConferencePreferenceRequest,
    ) -> Result<PutConferencePreferenceResponse, RusotoError<PutConferencePreferenceError>>;

    /// <p>Configures the email template for the user enrollment invitation with the specified attributes.</p>
    async fn put_invitation_configuration(
        &self,
        input: PutInvitationConfigurationRequest,
    ) -> Result<PutInvitationConfigurationResponse, RusotoError<PutInvitationConfigurationError>>;

    /// <p>Updates room skill parameter details by room, skill, and parameter key ID. Not all skills have a room skill parameter.</p>
    async fn put_room_skill_parameter(
        &self,
        input: PutRoomSkillParameterRequest,
    ) -> Result<PutRoomSkillParameterResponse, RusotoError<PutRoomSkillParameterError>>;

    /// <p>Links a user's account to a third-party skill provider. If this API operation is called by an assumed IAM role, the skill being linked must be a private skill. Also, the skill must be owned by the AWS account that assumed the IAM role.</p>
    async fn put_skill_authorization(
        &self,
        input: PutSkillAuthorizationRequest,
    ) -> Result<PutSkillAuthorizationResponse, RusotoError<PutSkillAuthorizationError>>;

    /// <p>Registers an Alexa-enabled device built by an Original Equipment Manufacturer (OEM) using Alexa Voice Service (AVS).</p>
    async fn register_avs_device(
        &self,
        input: RegisterAVSDeviceRequest,
    ) -> Result<RegisterAVSDeviceResponse, RusotoError<RegisterAVSDeviceError>>;

    /// <p>Disassociates a skill from the organization under a user's AWS account. If the skill is a private skill, it moves to an AcceptStatus of PENDING. Any private or public skill that is rejected can be added later by calling the ApproveSkill API. </p>
    async fn reject_skill(
        &self,
        input: RejectSkillRequest,
    ) -> Result<RejectSkillResponse, RusotoError<RejectSkillError>>;

    /// <p>Determines the details for the room from which a skill request was invoked. This operation is used by skill developers.</p>
    async fn resolve_room(
        &self,
        input: ResolveRoomRequest,
    ) -> Result<ResolveRoomResponse, RusotoError<ResolveRoomError>>;

    /// <p>Revokes an invitation and invalidates the enrollment URL.</p>
    async fn revoke_invitation(
        &self,
        input: RevokeInvitationRequest,
    ) -> Result<RevokeInvitationResponse, RusotoError<RevokeInvitationError>>;

    /// <p>Searches address books and lists the ones that meet a set of filter and sort criteria.</p>
    async fn search_address_books(
        &self,
        input: SearchAddressBooksRequest,
    ) -> Result<SearchAddressBooksResponse, RusotoError<SearchAddressBooksError>>;

    /// <p>Searches contacts and lists the ones that meet a set of filter and sort criteria.</p>
    async fn search_contacts(
        &self,
        input: SearchContactsRequest,
    ) -> Result<SearchContactsResponse, RusotoError<SearchContactsError>>;

    /// <p>Searches devices and lists the ones that meet a set of filter criteria.</p>
    async fn search_devices(
        &self,
        input: SearchDevicesRequest,
    ) -> Result<SearchDevicesResponse, RusotoError<SearchDevicesError>>;

    /// <p>Searches network profiles and lists the ones that meet a set of filter and sort criteria.</p>
    async fn search_network_profiles(
        &self,
        input: SearchNetworkProfilesRequest,
    ) -> Result<SearchNetworkProfilesResponse, RusotoError<SearchNetworkProfilesError>>;

    /// <p>Searches room profiles and lists the ones that meet a set of filter criteria.</p>
    async fn search_profiles(
        &self,
        input: SearchProfilesRequest,
    ) -> Result<SearchProfilesResponse, RusotoError<SearchProfilesError>>;

    /// <p>Searches rooms and lists the ones that meet a set of filter and sort criteria.</p>
    async fn search_rooms(
        &self,
        input: SearchRoomsRequest,
    ) -> Result<SearchRoomsResponse, RusotoError<SearchRoomsError>>;

    /// <p>Searches skill groups and lists the ones that meet a set of filter and sort criteria.</p>
    async fn search_skill_groups(
        &self,
        input: SearchSkillGroupsRequest,
    ) -> Result<SearchSkillGroupsResponse, RusotoError<SearchSkillGroupsError>>;

    /// <p>Searches users and lists the ones that meet a set of filter and sort criteria.</p>
    async fn search_users(
        &self,
        input: SearchUsersRequest,
    ) -> Result<SearchUsersResponse, RusotoError<SearchUsersError>>;

    /// <p>Triggers an asynchronous flow to send text, SSML, or audio announcements to rooms that are identified by a search or filter. </p>
    async fn send_announcement(
        &self,
        input: SendAnnouncementRequest,
    ) -> Result<SendAnnouncementResponse, RusotoError<SendAnnouncementError>>;

    /// <p>Sends an enrollment invitation email with a URL to a user. The URL is valid for 30 days or until you call this operation again, whichever comes first. </p>
    async fn send_invitation(
        &self,
        input: SendInvitationRequest,
    ) -> Result<SendInvitationResponse, RusotoError<SendInvitationError>>;

    /// <p><p>Resets a device and its account to the known default settings. This clears all information and settings set by previous users in the following ways:</p> <ul> <li> <p>Bluetooth - This unpairs all bluetooth devices paired with your echo device.</p> </li> <li> <p>Volume - This resets the echo device&#39;s volume to the default value.</p> </li> <li> <p>Notifications - This clears all notifications from your echo device.</p> </li> <li> <p>Lists - This clears all to-do items from your echo device.</p> </li> <li> <p>Settings - This internally syncs the room&#39;s profile (if the device is assigned to a room), contacts, address books, delegation access for account linking, and communications (if enabled on the room profile).</p> </li> </ul></p>
    async fn start_device_sync(
        &self,
        input: StartDeviceSyncRequest,
    ) -> Result<StartDeviceSyncResponse, RusotoError<StartDeviceSyncError>>;

    /// <p>Initiates the discovery of any smart home appliances associated with the room.</p>
    async fn start_smart_home_appliance_discovery(
        &self,
        input: StartSmartHomeApplianceDiscoveryRequest,
    ) -> Result<
        StartSmartHomeApplianceDiscoveryResponse,
        RusotoError<StartSmartHomeApplianceDiscoveryError>,
    >;

    /// <p>Adds metadata tags to a specified resource.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>>;

    /// <p>Removes metadata tags from a specified resource.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>>;

    /// <p>Updates address book details by the address book ARN.</p>
    async fn update_address_book(
        &self,
        input: UpdateAddressBookRequest,
    ) -> Result<UpdateAddressBookResponse, RusotoError<UpdateAddressBookError>>;

    /// <p>Updates the configuration of the report delivery schedule with the specified schedule ARN.</p>
    async fn update_business_report_schedule(
        &self,
        input: UpdateBusinessReportScheduleRequest,
    ) -> Result<UpdateBusinessReportScheduleResponse, RusotoError<UpdateBusinessReportScheduleError>>;

    /// <p>Updates an existing conference provider's settings.</p>
    async fn update_conference_provider(
        &self,
        input: UpdateConferenceProviderRequest,
    ) -> Result<UpdateConferenceProviderResponse, RusotoError<UpdateConferenceProviderError>>;

    /// <p>Updates the contact details by the contact ARN.</p>
    async fn update_contact(
        &self,
        input: UpdateContactRequest,
    ) -> Result<UpdateContactResponse, RusotoError<UpdateContactError>>;

    /// <p>Updates the device name by device ARN.</p>
    async fn update_device(
        &self,
        input: UpdateDeviceRequest,
    ) -> Result<UpdateDeviceResponse, RusotoError<UpdateDeviceError>>;

    /// <p>Updates the details of a gateway. If any optional field is not provided, the existing corresponding value is left unmodified.</p>
    async fn update_gateway(
        &self,
        input: UpdateGatewayRequest,
    ) -> Result<UpdateGatewayResponse, RusotoError<UpdateGatewayError>>;

    /// <p>Updates the details of a gateway group. If any optional field is not provided, the existing corresponding value is left unmodified.</p>
    async fn update_gateway_group(
        &self,
        input: UpdateGatewayGroupRequest,
    ) -> Result<UpdateGatewayGroupResponse, RusotoError<UpdateGatewayGroupError>>;

    /// <p>Updates a network profile by the network profile ARN.</p>
    async fn update_network_profile(
        &self,
        input: UpdateNetworkProfileRequest,
    ) -> Result<UpdateNetworkProfileResponse, RusotoError<UpdateNetworkProfileError>>;

    /// <p>Updates an existing room profile by room profile ARN.</p>
    async fn update_profile(
        &self,
        input: UpdateProfileRequest,
    ) -> Result<UpdateProfileResponse, RusotoError<UpdateProfileError>>;

    /// <p>Updates room details by room ARN.</p>
    async fn update_room(
        &self,
        input: UpdateRoomRequest,
    ) -> Result<UpdateRoomResponse, RusotoError<UpdateRoomError>>;

    /// <p>Updates skill group details by skill group ARN.</p>
    async fn update_skill_group(
        &self,
        input: UpdateSkillGroupRequest,
    ) -> Result<UpdateSkillGroupResponse, RusotoError<UpdateSkillGroupError>>;
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
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> AlexaForBusinessClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        AlexaForBusinessClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> AlexaForBusinessClient {
        AlexaForBusinessClient { client, region }
    }
}

#[async_trait]
impl AlexaForBusiness for AlexaForBusinessClient {
    /// <p>Associates a skill with the organization under the customer's AWS account. If a skill is private, the user implicitly accepts access to this skill during enablement.</p>
    async fn approve_skill(
        &self,
        input: ApproveSkillRequest,
    ) -> Result<ApproveSkillResponse, RusotoError<ApproveSkillError>> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.ApproveSkill");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<ApproveSkillResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ApproveSkillError::from_response(response))
        }
    }

    /// <p>Associates a contact with a given address book.</p>
    async fn associate_contact_with_address_book(
        &self,
        input: AssociateContactWithAddressBookRequest,
    ) -> Result<
        AssociateContactWithAddressBookResponse,
        RusotoError<AssociateContactWithAddressBookError>,
    > {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AlexaForBusiness.AssociateContactWithAddressBook",
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
                .deserialize::<AssociateContactWithAddressBookResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(AssociateContactWithAddressBookError::from_response(
                response,
            ))
        }
    }

    /// <p>Associates a device with the specified network profile.</p>
    async fn associate_device_with_network_profile(
        &self,
        input: AssociateDeviceWithNetworkProfileRequest,
    ) -> Result<
        AssociateDeviceWithNetworkProfileResponse,
        RusotoError<AssociateDeviceWithNetworkProfileError>,
    > {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AlexaForBusiness.AssociateDeviceWithNetworkProfile",
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
                .deserialize::<AssociateDeviceWithNetworkProfileResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(AssociateDeviceWithNetworkProfileError::from_response(
                response,
            ))
        }
    }

    /// <p>Associates a device with a given room. This applies all the settings from the room profile to the device, and all the skills in any skill groups added to that room. This operation requires the device to be online, or else a manual sync is required. </p>
    async fn associate_device_with_room(
        &self,
        input: AssociateDeviceWithRoomRequest,
    ) -> Result<AssociateDeviceWithRoomResponse, RusotoError<AssociateDeviceWithRoomError>> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.AssociateDeviceWithRoom");
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
                .deserialize::<AssociateDeviceWithRoomResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(AssociateDeviceWithRoomError::from_response(response))
        }
    }

    /// <p>Associates a skill group with a given room. This enables all skills in the associated skill group on all devices in the room.</p>
    async fn associate_skill_group_with_room(
        &self,
        input: AssociateSkillGroupWithRoomRequest,
    ) -> Result<AssociateSkillGroupWithRoomResponse, RusotoError<AssociateSkillGroupWithRoomError>>
    {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AlexaForBusiness.AssociateSkillGroupWithRoom",
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
                .deserialize::<AssociateSkillGroupWithRoomResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(AssociateSkillGroupWithRoomError::from_response(response))
        }
    }

    /// <p>Associates a skill with a skill group.</p>
    async fn associate_skill_with_skill_group(
        &self,
        input: AssociateSkillWithSkillGroupRequest,
    ) -> Result<AssociateSkillWithSkillGroupResponse, RusotoError<AssociateSkillWithSkillGroupError>>
    {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AlexaForBusiness.AssociateSkillWithSkillGroup",
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
                .deserialize::<AssociateSkillWithSkillGroupResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(AssociateSkillWithSkillGroupError::from_response(response))
        }
    }

    /// <p>Makes a private skill available for enrolled users to enable on their devices.</p>
    async fn associate_skill_with_users(
        &self,
        input: AssociateSkillWithUsersRequest,
    ) -> Result<AssociateSkillWithUsersResponse, RusotoError<AssociateSkillWithUsersError>> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.AssociateSkillWithUsers");
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
                .deserialize::<AssociateSkillWithUsersResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(AssociateSkillWithUsersError::from_response(response))
        }
    }

    /// <p>Creates an address book with the specified details.</p>
    async fn create_address_book(
        &self,
        input: CreateAddressBookRequest,
    ) -> Result<CreateAddressBookResponse, RusotoError<CreateAddressBookError>> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.CreateAddressBook");
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
                .deserialize::<CreateAddressBookResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateAddressBookError::from_response(response))
        }
    }

    /// <p>Creates a recurring schedule for usage reports to deliver to the specified S3 location with a specified daily or weekly interval.</p>
    async fn create_business_report_schedule(
        &self,
        input: CreateBusinessReportScheduleRequest,
    ) -> Result<CreateBusinessReportScheduleResponse, RusotoError<CreateBusinessReportScheduleError>>
    {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AlexaForBusiness.CreateBusinessReportSchedule",
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
                .deserialize::<CreateBusinessReportScheduleResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateBusinessReportScheduleError::from_response(response))
        }
    }

    /// <p>Adds a new conference provider under the user's AWS account.</p>
    async fn create_conference_provider(
        &self,
        input: CreateConferenceProviderRequest,
    ) -> Result<CreateConferenceProviderResponse, RusotoError<CreateConferenceProviderError>> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.CreateConferenceProvider");
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
                .deserialize::<CreateConferenceProviderResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateConferenceProviderError::from_response(response))
        }
    }

    /// <p>Creates a contact with the specified details.</p>
    async fn create_contact(
        &self,
        input: CreateContactRequest,
    ) -> Result<CreateContactResponse, RusotoError<CreateContactError>> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.CreateContact");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<CreateContactResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateContactError::from_response(response))
        }
    }

    /// <p>Creates a gateway group with the specified details.</p>
    async fn create_gateway_group(
        &self,
        input: CreateGatewayGroupRequest,
    ) -> Result<CreateGatewayGroupResponse, RusotoError<CreateGatewayGroupError>> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.CreateGatewayGroup");
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
                .deserialize::<CreateGatewayGroupResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateGatewayGroupError::from_response(response))
        }
    }

    /// <p>Creates a network profile with the specified details.</p>
    async fn create_network_profile(
        &self,
        input: CreateNetworkProfileRequest,
    ) -> Result<CreateNetworkProfileResponse, RusotoError<CreateNetworkProfileError>> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.CreateNetworkProfile");
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
                .deserialize::<CreateNetworkProfileResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateNetworkProfileError::from_response(response))
        }
    }

    /// <p>Creates a new room profile with the specified details.</p>
    async fn create_profile(
        &self,
        input: CreateProfileRequest,
    ) -> Result<CreateProfileResponse, RusotoError<CreateProfileError>> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.CreateProfile");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<CreateProfileResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateProfileError::from_response(response))
        }
    }

    /// <p>Creates a room with the specified details.</p>
    async fn create_room(
        &self,
        input: CreateRoomRequest,
    ) -> Result<CreateRoomResponse, RusotoError<CreateRoomError>> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.CreateRoom");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<CreateRoomResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateRoomError::from_response(response))
        }
    }

    /// <p>Creates a skill group with a specified name and description.</p>
    async fn create_skill_group(
        &self,
        input: CreateSkillGroupRequest,
    ) -> Result<CreateSkillGroupResponse, RusotoError<CreateSkillGroupError>> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.CreateSkillGroup");
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
                .deserialize::<CreateSkillGroupResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateSkillGroupError::from_response(response))
        }
    }

    /// <p>Creates a user.</p>
    async fn create_user(
        &self,
        input: CreateUserRequest,
    ) -> Result<CreateUserResponse, RusotoError<CreateUserError>> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.CreateUser");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<CreateUserResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateUserError::from_response(response))
        }
    }

    /// <p>Deletes an address book by the address book ARN.</p>
    async fn delete_address_book(
        &self,
        input: DeleteAddressBookRequest,
    ) -> Result<DeleteAddressBookResponse, RusotoError<DeleteAddressBookError>> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.DeleteAddressBook");
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
                .deserialize::<DeleteAddressBookResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteAddressBookError::from_response(response))
        }
    }

    /// <p>Deletes the recurring report delivery schedule with the specified schedule ARN.</p>
    async fn delete_business_report_schedule(
        &self,
        input: DeleteBusinessReportScheduleRequest,
    ) -> Result<DeleteBusinessReportScheduleResponse, RusotoError<DeleteBusinessReportScheduleError>>
    {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AlexaForBusiness.DeleteBusinessReportSchedule",
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
                .deserialize::<DeleteBusinessReportScheduleResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteBusinessReportScheduleError::from_response(response))
        }
    }

    /// <p>Deletes a conference provider.</p>
    async fn delete_conference_provider(
        &self,
        input: DeleteConferenceProviderRequest,
    ) -> Result<DeleteConferenceProviderResponse, RusotoError<DeleteConferenceProviderError>> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.DeleteConferenceProvider");
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
                .deserialize::<DeleteConferenceProviderResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteConferenceProviderError::from_response(response))
        }
    }

    /// <p>Deletes a contact by the contact ARN.</p>
    async fn delete_contact(
        &self,
        input: DeleteContactRequest,
    ) -> Result<DeleteContactResponse, RusotoError<DeleteContactError>> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.DeleteContact");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DeleteContactResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteContactError::from_response(response))
        }
    }

    /// <p>Removes a device from Alexa For Business.</p>
    async fn delete_device(
        &self,
        input: DeleteDeviceRequest,
    ) -> Result<DeleteDeviceResponse, RusotoError<DeleteDeviceError>> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.DeleteDevice");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DeleteDeviceResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteDeviceError::from_response(response))
        }
    }

    /// <p>When this action is called for a specified shared device, it allows authorized users to delete the device's entire previous history of voice input data and associated response data. This action can be called once every 24 hours for a specific shared device.</p>
    async fn delete_device_usage_data(
        &self,
        input: DeleteDeviceUsageDataRequest,
    ) -> Result<DeleteDeviceUsageDataResponse, RusotoError<DeleteDeviceUsageDataError>> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.DeleteDeviceUsageData");
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
                .deserialize::<DeleteDeviceUsageDataResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteDeviceUsageDataError::from_response(response))
        }
    }

    /// <p>Deletes a gateway group.</p>
    async fn delete_gateway_group(
        &self,
        input: DeleteGatewayGroupRequest,
    ) -> Result<DeleteGatewayGroupResponse, RusotoError<DeleteGatewayGroupError>> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.DeleteGatewayGroup");
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
                .deserialize::<DeleteGatewayGroupResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteGatewayGroupError::from_response(response))
        }
    }

    /// <p>Deletes a network profile by the network profile ARN.</p>
    async fn delete_network_profile(
        &self,
        input: DeleteNetworkProfileRequest,
    ) -> Result<DeleteNetworkProfileResponse, RusotoError<DeleteNetworkProfileError>> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.DeleteNetworkProfile");
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
                .deserialize::<DeleteNetworkProfileResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteNetworkProfileError::from_response(response))
        }
    }

    /// <p>Deletes a room profile by the profile ARN.</p>
    async fn delete_profile(
        &self,
        input: DeleteProfileRequest,
    ) -> Result<DeleteProfileResponse, RusotoError<DeleteProfileError>> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.DeleteProfile");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DeleteProfileResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteProfileError::from_response(response))
        }
    }

    /// <p>Deletes a room by the room ARN.</p>
    async fn delete_room(
        &self,
        input: DeleteRoomRequest,
    ) -> Result<DeleteRoomResponse, RusotoError<DeleteRoomError>> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.DeleteRoom");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DeleteRoomResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteRoomError::from_response(response))
        }
    }

    /// <p>Deletes room skill parameter details by room, skill, and parameter key ID.</p>
    async fn delete_room_skill_parameter(
        &self,
        input: DeleteRoomSkillParameterRequest,
    ) -> Result<DeleteRoomSkillParameterResponse, RusotoError<DeleteRoomSkillParameterError>> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.DeleteRoomSkillParameter");
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
                .deserialize::<DeleteRoomSkillParameterResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteRoomSkillParameterError::from_response(response))
        }
    }

    /// <p>Unlinks a third-party account from a skill.</p>
    async fn delete_skill_authorization(
        &self,
        input: DeleteSkillAuthorizationRequest,
    ) -> Result<DeleteSkillAuthorizationResponse, RusotoError<DeleteSkillAuthorizationError>> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.DeleteSkillAuthorization");
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
                .deserialize::<DeleteSkillAuthorizationResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteSkillAuthorizationError::from_response(response))
        }
    }

    /// <p>Deletes a skill group by skill group ARN.</p>
    async fn delete_skill_group(
        &self,
        input: DeleteSkillGroupRequest,
    ) -> Result<DeleteSkillGroupResponse, RusotoError<DeleteSkillGroupError>> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.DeleteSkillGroup");
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
                .deserialize::<DeleteSkillGroupResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteSkillGroupError::from_response(response))
        }
    }

    /// <p>Deletes a specified user by user ARN and enrollment ARN.</p>
    async fn delete_user(
        &self,
        input: DeleteUserRequest,
    ) -> Result<DeleteUserResponse, RusotoError<DeleteUserError>> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.DeleteUser");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DeleteUserResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteUserError::from_response(response))
        }
    }

    /// <p>Disassociates a contact from a given address book.</p>
    async fn disassociate_contact_from_address_book(
        &self,
        input: DisassociateContactFromAddressBookRequest,
    ) -> Result<
        DisassociateContactFromAddressBookResponse,
        RusotoError<DisassociateContactFromAddressBookError>,
    > {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AlexaForBusiness.DisassociateContactFromAddressBook",
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
                .deserialize::<DisassociateContactFromAddressBookResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DisassociateContactFromAddressBookError::from_response(
                response,
            ))
        }
    }

    /// <p>Disassociates a device from its current room. The device continues to be connected to the Wi-Fi network and is still registered to the account. The device settings and skills are removed from the room.</p>
    async fn disassociate_device_from_room(
        &self,
        input: DisassociateDeviceFromRoomRequest,
    ) -> Result<DisassociateDeviceFromRoomResponse, RusotoError<DisassociateDeviceFromRoomError>>
    {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AlexaForBusiness.DisassociateDeviceFromRoom",
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
                .deserialize::<DisassociateDeviceFromRoomResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DisassociateDeviceFromRoomError::from_response(response))
        }
    }

    /// <p>Disassociates a skill from a skill group.</p>
    async fn disassociate_skill_from_skill_group(
        &self,
        input: DisassociateSkillFromSkillGroupRequest,
    ) -> Result<
        DisassociateSkillFromSkillGroupResponse,
        RusotoError<DisassociateSkillFromSkillGroupError>,
    > {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AlexaForBusiness.DisassociateSkillFromSkillGroup",
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
                .deserialize::<DisassociateSkillFromSkillGroupResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DisassociateSkillFromSkillGroupError::from_response(
                response,
            ))
        }
    }

    /// <p>Makes a private skill unavailable for enrolled users and prevents them from enabling it on their devices.</p>
    async fn disassociate_skill_from_users(
        &self,
        input: DisassociateSkillFromUsersRequest,
    ) -> Result<DisassociateSkillFromUsersResponse, RusotoError<DisassociateSkillFromUsersError>>
    {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AlexaForBusiness.DisassociateSkillFromUsers",
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
                .deserialize::<DisassociateSkillFromUsersResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DisassociateSkillFromUsersError::from_response(response))
        }
    }

    /// <p>Disassociates a skill group from a specified room. This disables all skills in the skill group on all devices in the room.</p>
    async fn disassociate_skill_group_from_room(
        &self,
        input: DisassociateSkillGroupFromRoomRequest,
    ) -> Result<
        DisassociateSkillGroupFromRoomResponse,
        RusotoError<DisassociateSkillGroupFromRoomError>,
    > {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AlexaForBusiness.DisassociateSkillGroupFromRoom",
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
                .deserialize::<DisassociateSkillGroupFromRoomResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DisassociateSkillGroupFromRoomError::from_response(response))
        }
    }

    /// <p>Forgets smart home appliances associated to a room.</p>
    async fn forget_smart_home_appliances(
        &self,
        input: ForgetSmartHomeAppliancesRequest,
    ) -> Result<ForgetSmartHomeAppliancesResponse, RusotoError<ForgetSmartHomeAppliancesError>>
    {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.ForgetSmartHomeAppliances");
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
                .deserialize::<ForgetSmartHomeAppliancesResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ForgetSmartHomeAppliancesError::from_response(response))
        }
    }

    /// <p>Gets address the book details by the address book ARN.</p>
    async fn get_address_book(
        &self,
        input: GetAddressBookRequest,
    ) -> Result<GetAddressBookResponse, RusotoError<GetAddressBookError>> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.GetAddressBook");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<GetAddressBookResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetAddressBookError::from_response(response))
        }
    }

    /// <p>Retrieves the existing conference preferences.</p>
    async fn get_conference_preference(
        &self,
    ) -> Result<GetConferencePreferenceResponse, RusotoError<GetConferencePreferenceError>> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.GetConferencePreference");
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<GetConferencePreferenceResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetConferencePreferenceError::from_response(response))
        }
    }

    /// <p>Gets details about a specific conference provider.</p>
    async fn get_conference_provider(
        &self,
        input: GetConferenceProviderRequest,
    ) -> Result<GetConferenceProviderResponse, RusotoError<GetConferenceProviderError>> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.GetConferenceProvider");
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
                .deserialize::<GetConferenceProviderResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetConferenceProviderError::from_response(response))
        }
    }

    /// <p>Gets the contact details by the contact ARN.</p>
    async fn get_contact(
        &self,
        input: GetContactRequest,
    ) -> Result<GetContactResponse, RusotoError<GetContactError>> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.GetContact");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<GetContactResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetContactError::from_response(response))
        }
    }

    /// <p>Gets the details of a device by device ARN.</p>
    async fn get_device(
        &self,
        input: GetDeviceRequest,
    ) -> Result<GetDeviceResponse, RusotoError<GetDeviceError>> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.GetDevice");
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

    /// <p>Retrieves the details of a gateway.</p>
    async fn get_gateway(
        &self,
        input: GetGatewayRequest,
    ) -> Result<GetGatewayResponse, RusotoError<GetGatewayError>> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.GetGateway");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<GetGatewayResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetGatewayError::from_response(response))
        }
    }

    /// <p>Retrieves the details of a gateway group.</p>
    async fn get_gateway_group(
        &self,
        input: GetGatewayGroupRequest,
    ) -> Result<GetGatewayGroupResponse, RusotoError<GetGatewayGroupError>> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.GetGatewayGroup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<GetGatewayGroupResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetGatewayGroupError::from_response(response))
        }
    }

    /// <p>Retrieves the configured values for the user enrollment invitation email template.</p>
    async fn get_invitation_configuration(
        &self,
    ) -> Result<GetInvitationConfigurationResponse, RusotoError<GetInvitationConfigurationError>>
    {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AlexaForBusiness.GetInvitationConfiguration",
        );
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<GetInvitationConfigurationResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetInvitationConfigurationError::from_response(response))
        }
    }

    /// <p>Gets the network profile details by the network profile ARN.</p>
    async fn get_network_profile(
        &self,
        input: GetNetworkProfileRequest,
    ) -> Result<GetNetworkProfileResponse, RusotoError<GetNetworkProfileError>> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.GetNetworkProfile");
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
                .deserialize::<GetNetworkProfileResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetNetworkProfileError::from_response(response))
        }
    }

    /// <p>Gets the details of a room profile by profile ARN.</p>
    async fn get_profile(
        &self,
        input: GetProfileRequest,
    ) -> Result<GetProfileResponse, RusotoError<GetProfileError>> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.GetProfile");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<GetProfileResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetProfileError::from_response(response))
        }
    }

    /// <p>Gets room details by room ARN.</p>
    async fn get_room(
        &self,
        input: GetRoomRequest,
    ) -> Result<GetRoomResponse, RusotoError<GetRoomError>> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.GetRoom");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<GetRoomResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetRoomError::from_response(response))
        }
    }

    /// <p>Gets room skill parameter details by room, skill, and parameter key ARN.</p>
    async fn get_room_skill_parameter(
        &self,
        input: GetRoomSkillParameterRequest,
    ) -> Result<GetRoomSkillParameterResponse, RusotoError<GetRoomSkillParameterError>> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.GetRoomSkillParameter");
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
                .deserialize::<GetRoomSkillParameterResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetRoomSkillParameterError::from_response(response))
        }
    }

    /// <p>Gets skill group details by skill group ARN.</p>
    async fn get_skill_group(
        &self,
        input: GetSkillGroupRequest,
    ) -> Result<GetSkillGroupResponse, RusotoError<GetSkillGroupError>> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.GetSkillGroup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<GetSkillGroupResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetSkillGroupError::from_response(response))
        }
    }

    /// <p>Lists the details of the schedules that a user configured. A download URL of the report associated with each schedule is returned every time this action is called. A new download URL is returned each time, and is valid for 24 hours.</p>
    async fn list_business_report_schedules(
        &self,
        input: ListBusinessReportSchedulesRequest,
    ) -> Result<ListBusinessReportSchedulesResponse, RusotoError<ListBusinessReportSchedulesError>>
    {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AlexaForBusiness.ListBusinessReportSchedules",
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
                .deserialize::<ListBusinessReportSchedulesResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListBusinessReportSchedulesError::from_response(response))
        }
    }

    /// <p>Lists conference providers under a specific AWS account.</p>
    async fn list_conference_providers(
        &self,
        input: ListConferenceProvidersRequest,
    ) -> Result<ListConferenceProvidersResponse, RusotoError<ListConferenceProvidersError>> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.ListConferenceProviders");
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
                .deserialize::<ListConferenceProvidersResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListConferenceProvidersError::from_response(response))
        }
    }

    /// <p>Lists the device event history, including device connection status, for up to 30 days.</p>
    async fn list_device_events(
        &self,
        input: ListDeviceEventsRequest,
    ) -> Result<ListDeviceEventsResponse, RusotoError<ListDeviceEventsError>> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.ListDeviceEvents");
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
                .deserialize::<ListDeviceEventsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListDeviceEventsError::from_response(response))
        }
    }

    /// <p>Retrieves a list of gateway group summaries. Use GetGatewayGroup to retrieve details of a specific gateway group.</p>
    async fn list_gateway_groups(
        &self,
        input: ListGatewayGroupsRequest,
    ) -> Result<ListGatewayGroupsResponse, RusotoError<ListGatewayGroupsError>> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.ListGatewayGroups");
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
                .deserialize::<ListGatewayGroupsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListGatewayGroupsError::from_response(response))
        }
    }

    /// <p>Retrieves a list of gateway summaries. Use GetGateway to retrieve details of a specific gateway. An optional gateway group ARN can be provided to only retrieve gateway summaries of gateways that are associated with that gateway group ARN.</p>
    async fn list_gateways(
        &self,
        input: ListGatewaysRequest,
    ) -> Result<ListGatewaysResponse, RusotoError<ListGatewaysError>> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.ListGateways");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<ListGatewaysResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListGatewaysError::from_response(response))
        }
    }

    /// <p>Lists all enabled skills in a specific skill group.</p>
    async fn list_skills(
        &self,
        input: ListSkillsRequest,
    ) -> Result<ListSkillsResponse, RusotoError<ListSkillsError>> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.ListSkills");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<ListSkillsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListSkillsError::from_response(response))
        }
    }

    /// <p>Lists all categories in the Alexa skill store.</p>
    async fn list_skills_store_categories(
        &self,
        input: ListSkillsStoreCategoriesRequest,
    ) -> Result<ListSkillsStoreCategoriesResponse, RusotoError<ListSkillsStoreCategoriesError>>
    {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.ListSkillsStoreCategories");
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
                .deserialize::<ListSkillsStoreCategoriesResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListSkillsStoreCategoriesError::from_response(response))
        }
    }

    /// <p>Lists all skills in the Alexa skill store by category.</p>
    async fn list_skills_store_skills_by_category(
        &self,
        input: ListSkillsStoreSkillsByCategoryRequest,
    ) -> Result<
        ListSkillsStoreSkillsByCategoryResponse,
        RusotoError<ListSkillsStoreSkillsByCategoryError>,
    > {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AlexaForBusiness.ListSkillsStoreSkillsByCategory",
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
                .deserialize::<ListSkillsStoreSkillsByCategoryResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListSkillsStoreSkillsByCategoryError::from_response(
                response,
            ))
        }
    }

    /// <p>Lists all of the smart home appliances associated with a room.</p>
    async fn list_smart_home_appliances(
        &self,
        input: ListSmartHomeAppliancesRequest,
    ) -> Result<ListSmartHomeAppliancesResponse, RusotoError<ListSmartHomeAppliancesError>> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.ListSmartHomeAppliances");
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
                .deserialize::<ListSmartHomeAppliancesResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListSmartHomeAppliancesError::from_response(response))
        }
    }

    /// <p>Lists all tags for the specified resource.</p>
    async fn list_tags(
        &self,
        input: ListTagsRequest,
    ) -> Result<ListTagsResponse, RusotoError<ListTagsError>> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.ListTags");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<ListTagsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListTagsError::from_response(response))
        }
    }

    /// <p>Sets the conference preferences on a specific conference provider at the account level.</p>
    async fn put_conference_preference(
        &self,
        input: PutConferencePreferenceRequest,
    ) -> Result<PutConferencePreferenceResponse, RusotoError<PutConferencePreferenceError>> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.PutConferencePreference");
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
                .deserialize::<PutConferencePreferenceResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(PutConferencePreferenceError::from_response(response))
        }
    }

    /// <p>Configures the email template for the user enrollment invitation with the specified attributes.</p>
    async fn put_invitation_configuration(
        &self,
        input: PutInvitationConfigurationRequest,
    ) -> Result<PutInvitationConfigurationResponse, RusotoError<PutInvitationConfigurationError>>
    {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AlexaForBusiness.PutInvitationConfiguration",
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
                .deserialize::<PutInvitationConfigurationResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(PutInvitationConfigurationError::from_response(response))
        }
    }

    /// <p>Updates room skill parameter details by room, skill, and parameter key ID. Not all skills have a room skill parameter.</p>
    async fn put_room_skill_parameter(
        &self,
        input: PutRoomSkillParameterRequest,
    ) -> Result<PutRoomSkillParameterResponse, RusotoError<PutRoomSkillParameterError>> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.PutRoomSkillParameter");
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
                .deserialize::<PutRoomSkillParameterResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(PutRoomSkillParameterError::from_response(response))
        }
    }

    /// <p>Links a user's account to a third-party skill provider. If this API operation is called by an assumed IAM role, the skill being linked must be a private skill. Also, the skill must be owned by the AWS account that assumed the IAM role.</p>
    async fn put_skill_authorization(
        &self,
        input: PutSkillAuthorizationRequest,
    ) -> Result<PutSkillAuthorizationResponse, RusotoError<PutSkillAuthorizationError>> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.PutSkillAuthorization");
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
                .deserialize::<PutSkillAuthorizationResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(PutSkillAuthorizationError::from_response(response))
        }
    }

    /// <p>Registers an Alexa-enabled device built by an Original Equipment Manufacturer (OEM) using Alexa Voice Service (AVS).</p>
    async fn register_avs_device(
        &self,
        input: RegisterAVSDeviceRequest,
    ) -> Result<RegisterAVSDeviceResponse, RusotoError<RegisterAVSDeviceError>> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.RegisterAVSDevice");
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
                .deserialize::<RegisterAVSDeviceResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(RegisterAVSDeviceError::from_response(response))
        }
    }

    /// <p>Disassociates a skill from the organization under a user's AWS account. If the skill is a private skill, it moves to an AcceptStatus of PENDING. Any private or public skill that is rejected can be added later by calling the ApproveSkill API. </p>
    async fn reject_skill(
        &self,
        input: RejectSkillRequest,
    ) -> Result<RejectSkillResponse, RusotoError<RejectSkillError>> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.RejectSkill");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<RejectSkillResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(RejectSkillError::from_response(response))
        }
    }

    /// <p>Determines the details for the room from which a skill request was invoked. This operation is used by skill developers.</p>
    async fn resolve_room(
        &self,
        input: ResolveRoomRequest,
    ) -> Result<ResolveRoomResponse, RusotoError<ResolveRoomError>> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.ResolveRoom");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<ResolveRoomResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ResolveRoomError::from_response(response))
        }
    }

    /// <p>Revokes an invitation and invalidates the enrollment URL.</p>
    async fn revoke_invitation(
        &self,
        input: RevokeInvitationRequest,
    ) -> Result<RevokeInvitationResponse, RusotoError<RevokeInvitationError>> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.RevokeInvitation");
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
                .deserialize::<RevokeInvitationResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(RevokeInvitationError::from_response(response))
        }
    }

    /// <p>Searches address books and lists the ones that meet a set of filter and sort criteria.</p>
    async fn search_address_books(
        &self,
        input: SearchAddressBooksRequest,
    ) -> Result<SearchAddressBooksResponse, RusotoError<SearchAddressBooksError>> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.SearchAddressBooks");
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
                .deserialize::<SearchAddressBooksResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(SearchAddressBooksError::from_response(response))
        }
    }

    /// <p>Searches contacts and lists the ones that meet a set of filter and sort criteria.</p>
    async fn search_contacts(
        &self,
        input: SearchContactsRequest,
    ) -> Result<SearchContactsResponse, RusotoError<SearchContactsError>> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.SearchContacts");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<SearchContactsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(SearchContactsError::from_response(response))
        }
    }

    /// <p>Searches devices and lists the ones that meet a set of filter criteria.</p>
    async fn search_devices(
        &self,
        input: SearchDevicesRequest,
    ) -> Result<SearchDevicesResponse, RusotoError<SearchDevicesError>> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.SearchDevices");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<SearchDevicesResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(SearchDevicesError::from_response(response))
        }
    }

    /// <p>Searches network profiles and lists the ones that meet a set of filter and sort criteria.</p>
    async fn search_network_profiles(
        &self,
        input: SearchNetworkProfilesRequest,
    ) -> Result<SearchNetworkProfilesResponse, RusotoError<SearchNetworkProfilesError>> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.SearchNetworkProfiles");
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
                .deserialize::<SearchNetworkProfilesResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(SearchNetworkProfilesError::from_response(response))
        }
    }

    /// <p>Searches room profiles and lists the ones that meet a set of filter criteria.</p>
    async fn search_profiles(
        &self,
        input: SearchProfilesRequest,
    ) -> Result<SearchProfilesResponse, RusotoError<SearchProfilesError>> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.SearchProfiles");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<SearchProfilesResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(SearchProfilesError::from_response(response))
        }
    }

    /// <p>Searches rooms and lists the ones that meet a set of filter and sort criteria.</p>
    async fn search_rooms(
        &self,
        input: SearchRoomsRequest,
    ) -> Result<SearchRoomsResponse, RusotoError<SearchRoomsError>> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.SearchRooms");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<SearchRoomsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(SearchRoomsError::from_response(response))
        }
    }

    /// <p>Searches skill groups and lists the ones that meet a set of filter and sort criteria.</p>
    async fn search_skill_groups(
        &self,
        input: SearchSkillGroupsRequest,
    ) -> Result<SearchSkillGroupsResponse, RusotoError<SearchSkillGroupsError>> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.SearchSkillGroups");
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
                .deserialize::<SearchSkillGroupsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(SearchSkillGroupsError::from_response(response))
        }
    }

    /// <p>Searches users and lists the ones that meet a set of filter and sort criteria.</p>
    async fn search_users(
        &self,
        input: SearchUsersRequest,
    ) -> Result<SearchUsersResponse, RusotoError<SearchUsersError>> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.SearchUsers");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<SearchUsersResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(SearchUsersError::from_response(response))
        }
    }

    /// <p>Triggers an asynchronous flow to send text, SSML, or audio announcements to rooms that are identified by a search or filter. </p>
    async fn send_announcement(
        &self,
        input: SendAnnouncementRequest,
    ) -> Result<SendAnnouncementResponse, RusotoError<SendAnnouncementError>> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.SendAnnouncement");
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
                .deserialize::<SendAnnouncementResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(SendAnnouncementError::from_response(response))
        }
    }

    /// <p>Sends an enrollment invitation email with a URL to a user. The URL is valid for 30 days or until you call this operation again, whichever comes first. </p>
    async fn send_invitation(
        &self,
        input: SendInvitationRequest,
    ) -> Result<SendInvitationResponse, RusotoError<SendInvitationError>> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.SendInvitation");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<SendInvitationResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(SendInvitationError::from_response(response))
        }
    }

    /// <p><p>Resets a device and its account to the known default settings. This clears all information and settings set by previous users in the following ways:</p> <ul> <li> <p>Bluetooth - This unpairs all bluetooth devices paired with your echo device.</p> </li> <li> <p>Volume - This resets the echo device&#39;s volume to the default value.</p> </li> <li> <p>Notifications - This clears all notifications from your echo device.</p> </li> <li> <p>Lists - This clears all to-do items from your echo device.</p> </li> <li> <p>Settings - This internally syncs the room&#39;s profile (if the device is assigned to a room), contacts, address books, delegation access for account linking, and communications (if enabled on the room profile).</p> </li> </ul></p>
    async fn start_device_sync(
        &self,
        input: StartDeviceSyncRequest,
    ) -> Result<StartDeviceSyncResponse, RusotoError<StartDeviceSyncError>> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.StartDeviceSync");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<StartDeviceSyncResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(StartDeviceSyncError::from_response(response))
        }
    }

    /// <p>Initiates the discovery of any smart home appliances associated with the room.</p>
    async fn start_smart_home_appliance_discovery(
        &self,
        input: StartSmartHomeApplianceDiscoveryRequest,
    ) -> Result<
        StartSmartHomeApplianceDiscoveryResponse,
        RusotoError<StartSmartHomeApplianceDiscoveryError>,
    > {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AlexaForBusiness.StartSmartHomeApplianceDiscovery",
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
                .deserialize::<StartSmartHomeApplianceDiscoveryResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(StartSmartHomeApplianceDiscoveryError::from_response(
                response,
            ))
        }
    }

    /// <p>Adds metadata tags to a specified resource.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.TagResource");
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

    /// <p>Removes metadata tags from a specified resource.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.UntagResource");
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

    /// <p>Updates address book details by the address book ARN.</p>
    async fn update_address_book(
        &self,
        input: UpdateAddressBookRequest,
    ) -> Result<UpdateAddressBookResponse, RusotoError<UpdateAddressBookError>> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.UpdateAddressBook");
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
                .deserialize::<UpdateAddressBookResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateAddressBookError::from_response(response))
        }
    }

    /// <p>Updates the configuration of the report delivery schedule with the specified schedule ARN.</p>
    async fn update_business_report_schedule(
        &self,
        input: UpdateBusinessReportScheduleRequest,
    ) -> Result<UpdateBusinessReportScheduleResponse, RusotoError<UpdateBusinessReportScheduleError>>
    {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AlexaForBusiness.UpdateBusinessReportSchedule",
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
                .deserialize::<UpdateBusinessReportScheduleResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateBusinessReportScheduleError::from_response(response))
        }
    }

    /// <p>Updates an existing conference provider's settings.</p>
    async fn update_conference_provider(
        &self,
        input: UpdateConferenceProviderRequest,
    ) -> Result<UpdateConferenceProviderResponse, RusotoError<UpdateConferenceProviderError>> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.UpdateConferenceProvider");
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
                .deserialize::<UpdateConferenceProviderResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateConferenceProviderError::from_response(response))
        }
    }

    /// <p>Updates the contact details by the contact ARN.</p>
    async fn update_contact(
        &self,
        input: UpdateContactRequest,
    ) -> Result<UpdateContactResponse, RusotoError<UpdateContactError>> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.UpdateContact");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<UpdateContactResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateContactError::from_response(response))
        }
    }

    /// <p>Updates the device name by device ARN.</p>
    async fn update_device(
        &self,
        input: UpdateDeviceRequest,
    ) -> Result<UpdateDeviceResponse, RusotoError<UpdateDeviceError>> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.UpdateDevice");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<UpdateDeviceResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateDeviceError::from_response(response))
        }
    }

    /// <p>Updates the details of a gateway. If any optional field is not provided, the existing corresponding value is left unmodified.</p>
    async fn update_gateway(
        &self,
        input: UpdateGatewayRequest,
    ) -> Result<UpdateGatewayResponse, RusotoError<UpdateGatewayError>> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.UpdateGateway");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<UpdateGatewayResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateGatewayError::from_response(response))
        }
    }

    /// <p>Updates the details of a gateway group. If any optional field is not provided, the existing corresponding value is left unmodified.</p>
    async fn update_gateway_group(
        &self,
        input: UpdateGatewayGroupRequest,
    ) -> Result<UpdateGatewayGroupResponse, RusotoError<UpdateGatewayGroupError>> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.UpdateGatewayGroup");
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
                .deserialize::<UpdateGatewayGroupResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateGatewayGroupError::from_response(response))
        }
    }

    /// <p>Updates a network profile by the network profile ARN.</p>
    async fn update_network_profile(
        &self,
        input: UpdateNetworkProfileRequest,
    ) -> Result<UpdateNetworkProfileResponse, RusotoError<UpdateNetworkProfileError>> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.UpdateNetworkProfile");
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
                .deserialize::<UpdateNetworkProfileResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateNetworkProfileError::from_response(response))
        }
    }

    /// <p>Updates an existing room profile by room profile ARN.</p>
    async fn update_profile(
        &self,
        input: UpdateProfileRequest,
    ) -> Result<UpdateProfileResponse, RusotoError<UpdateProfileError>> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.UpdateProfile");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<UpdateProfileResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateProfileError::from_response(response))
        }
    }

    /// <p>Updates room details by room ARN.</p>
    async fn update_room(
        &self,
        input: UpdateRoomRequest,
    ) -> Result<UpdateRoomResponse, RusotoError<UpdateRoomError>> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.UpdateRoom");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<UpdateRoomResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateRoomError::from_response(response))
        }
    }

    /// <p>Updates skill group details by skill group ARN.</p>
    async fn update_skill_group(
        &self,
        input: UpdateSkillGroupRequest,
    ) -> Result<UpdateSkillGroupResponse, RusotoError<UpdateSkillGroupError>> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.UpdateSkillGroup");
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
                .deserialize::<UpdateSkillGroupResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateSkillGroupError::from_response(response))
        }
    }
}
