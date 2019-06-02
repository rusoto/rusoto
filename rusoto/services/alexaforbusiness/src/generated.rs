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

use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
use serde_json;
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
pub struct AssociateDeviceWithNetworkProfileRequest {
    /// <p>The device ARN.</p>
    #[serde(rename = "DeviceArn")]
    pub device_arn: String,
    /// <p>The ARN of the network profile to associate with a device.</p>
    #[serde(rename = "NetworkProfileArn")]
    pub network_profile_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AssociateDeviceWithNetworkProfileResponse {}

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
    /// <p>The private skill ID you want to make available to enrolled users.</p>
    #[serde(rename = "SkillId")]
    pub skill_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AssociateSkillWithUsersResponse {}

/// <p>The audio message. There is a 1 MB limit on the audio file input and the only supported format is MP3. To convert your MP3 audio files to an Alexa-friendly, </p> <p>required codec version (MPEG version 2) and bit rate (48 kbps), you might use converter software. One option for this is a command-line tool, FFmpeg. For more information, see <a href="https://www.ffmpeg.org/">FFmpeg</a>. The following command converts the provided &lt;input-file&gt; to an MP3 file that is played in the announcement:</p> <p> <code>ffmpeg -i &lt;input-file&gt; -ac 2 -codec:a libmp3lame -b:a 48k -ar 16000 &lt;output-file.mp3&gt;</code> </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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

/// <p>The content definition. This can contain only one text, SSML, or audio list object.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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
#[cfg_attr(test, derive(Serialize))]
pub struct CreateGatewayGroupResponse {
    /// <p>The ARN of the created gateway group.</p>
    #[serde(rename = "GatewayGroupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_group_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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
#[cfg_attr(test, derive(Serialize))]
pub struct CreateNetworkProfileResponse {
    /// <p>The ARN of the network profile associated with a device.</p>
    #[serde(rename = "NetworkProfileArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_profile_arn: Option<String>,
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
pub struct DeleteDeviceUsageDataRequest {
    /// <p>The ARN of the device.</p>
    #[serde(rename = "DeviceArn")]
    pub device_arn: String,
    /// <p>The type of usage data to delete.</p>
    #[serde(rename = "DeviceUsageType")]
    pub device_usage_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteDeviceUsageDataResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteGatewayGroupRequest {
    /// <p>The ARN of the gateway group to delete.</p>
    #[serde(rename = "GatewayGroupArn")]
    pub gateway_group_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteGatewayGroupResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteNetworkProfileRequest {
    /// <p>The ARN of the network profile associated with a device.</p>
    #[serde(rename = "NetworkProfileArn")]
    pub network_profile_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteNetworkProfileResponse {}

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

/// <p>Detailed information about a device's network profile.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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

/// <p>The details of the gateway. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
pub struct GetGatewayGroupRequest {
    /// <p>The ARN of the gateway group to get.</p>
    #[serde(rename = "GatewayGroupArn")]
    pub gateway_group_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetGatewayGroupResponse {
    #[serde(rename = "GatewayGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_group: Option<GatewayGroup>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetGatewayRequest {
    /// <p>The ARN of the gateway to get.</p>
    #[serde(rename = "GatewayArn")]
    pub gateway_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetGatewayResponse {
    /// <p>The details of the gateway.</p>
    #[serde(rename = "Gateway")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway: Option<Gateway>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetInvitationConfigurationRequest {}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
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
pub struct GetNetworkProfileRequest {
    /// <p>The ARN of the network profile associated with a device.</p>
    #[serde(rename = "NetworkProfileArn")]
    pub network_profile_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetNetworkProfileResponse {
    /// <p>The network profile associated with a device.</p>
    #[serde(rename = "NetworkProfile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_profile: Option<NetworkProfile>,
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
pub struct ListSkillsRequest {
    /// <p>Whether the skill is enabled under the user's account, or if it requires linking to be used.</p>
    #[serde(rename = "EnablementType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enablement_type: Option<String>,
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

/// <p>The network profile associated with a device.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
pub struct PutInvitationConfigurationResponse {}

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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
pub struct SendAnnouncementResponse {
    /// <p>The identifier of the announcement.</p>
    #[serde(rename = "AnnouncementArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub announcement_arn: Option<String>,
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

/// <p>The SSML message. For more information, see <a href="https://developer.amazon.com/docs/custom-skills/speech-synthesis-markup-language-ssml-reference.html">SSML Reference</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct Ssml {
    /// <p>The locale of the SSML message. Currently, en-US is supported.</p>
    #[serde(rename = "Locale")]
    pub locale: String,
    /// <p>The value of the SSML message in the correct SSML format. The audio tag is not supported.</p>
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

/// <p>The text message.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct Text {
    /// <p>The locale of the text message. Currently, en-US is supported.</p>
    #[serde(rename = "Locale")]
    pub locale: String,
    /// <p>The value of the text message.</p>
    #[serde(rename = "Value")]
    pub value: String,
}

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
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateGatewayGroupResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateGatewayResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateNetworkProfileResponse {}

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
        }
    }
}
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AssociateContactWithAddressBookError {
    fn description(&self) -> &str {
        match *self {
            AssociateContactWithAddressBookError::LimitExceeded(ref cause) => cause,
        }
    }
}
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AssociateDeviceWithNetworkProfileError {
    fn description(&self) -> &str {
        match *self {
            AssociateDeviceWithNetworkProfileError::ConcurrentModification(ref cause) => cause,
            AssociateDeviceWithNetworkProfileError::DeviceNotRegistered(ref cause) => cause,
            AssociateDeviceWithNetworkProfileError::NotFound(ref cause) => cause,
        }
    }
}
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
        }
    }
}
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AssociateSkillGroupWithRoomError {
    fn description(&self) -> &str {
        match *self {
            AssociateSkillGroupWithRoomError::ConcurrentModification(ref cause) => cause,
        }
    }
}
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
        }
    }
}
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AssociateSkillWithUsersError {
    fn description(&self) -> &str {
        match *self {
            AssociateSkillWithUsersError::ConcurrentModification(ref cause) => cause,
            AssociateSkillWithUsersError::NotFound(ref cause) => cause,
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateAddressBookError {
    fn description(&self) -> &str {
        match *self {
            CreateAddressBookError::AlreadyExists(ref cause) => cause,
            CreateAddressBookError::LimitExceeded(ref cause) => cause,
        }
    }
}
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateBusinessReportScheduleError {
    fn description(&self) -> &str {
        match *self {
            CreateBusinessReportScheduleError::AlreadyExists(ref cause) => cause,
        }
    }
}
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateConferenceProviderError {
    fn description(&self) -> &str {
        match *self {
            CreateConferenceProviderError::AlreadyExists(ref cause) => cause,
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateContactError {
    fn description(&self) -> &str {
        match *self {
            CreateContactError::AlreadyExists(ref cause) => cause,
            CreateContactError::LimitExceeded(ref cause) => cause,
        }
    }
}
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateGatewayGroupError {
    fn description(&self) -> &str {
        match *self {
            CreateGatewayGroupError::AlreadyExists(ref cause) => cause,
            CreateGatewayGroupError::LimitExceeded(ref cause) => cause,
        }
    }
}
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateNetworkProfileError {
    fn description(&self) -> &str {
        match *self {
            CreateNetworkProfileError::AlreadyExists(ref cause) => cause,
            CreateNetworkProfileError::ConcurrentModification(ref cause) => cause,
            CreateNetworkProfileError::InvalidCertificateAuthority(ref cause) => cause,
            CreateNetworkProfileError::InvalidServiceLinkedRoleState(ref cause) => cause,
            CreateNetworkProfileError::LimitExceeded(ref cause) => cause,
        }
    }
}
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateRoomError {
    fn description(&self) -> &str {
        match *self {
            CreateRoomError::AlreadyExists(ref cause) => cause,
            CreateRoomError::LimitExceeded(ref cause) => cause,
        }
    }
}
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
        }
    }
}
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
        }
    }
}
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteAddressBookError {
    fn description(&self) -> &str {
        match *self {
            DeleteAddressBookError::ConcurrentModification(ref cause) => cause,
            DeleteAddressBookError::NotFound(ref cause) => cause,
        }
    }
}
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteBusinessReportScheduleError {
    fn description(&self) -> &str {
        match *self {
            DeleteBusinessReportScheduleError::ConcurrentModification(ref cause) => cause,
            DeleteBusinessReportScheduleError::NotFound(ref cause) => cause,
        }
    }
}
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteConferenceProviderError {
    fn description(&self) -> &str {
        match *self {
            DeleteConferenceProviderError::NotFound(ref cause) => cause,
        }
    }
}
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteContactError {
    fn description(&self) -> &str {
        match *self {
            DeleteContactError::ConcurrentModification(ref cause) => cause,
            DeleteContactError::NotFound(ref cause) => cause,
        }
    }
}
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
        }
    }
}
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteDeviceUsageDataError {
    fn description(&self) -> &str {
        match *self {
            DeleteDeviceUsageDataError::DeviceNotRegistered(ref cause) => cause,
            DeleteDeviceUsageDataError::LimitExceeded(ref cause) => cause,
            DeleteDeviceUsageDataError::NotFound(ref cause) => cause,
        }
    }
}
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteGatewayGroupError {
    fn description(&self) -> &str {
        match *self {
            DeleteGatewayGroupError::ResourceAssociated(ref cause) => cause,
        }
    }
}
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteNetworkProfileError {
    fn description(&self) -> &str {
        match *self {
            DeleteNetworkProfileError::ConcurrentModification(ref cause) => cause,
            DeleteNetworkProfileError::NotFound(ref cause) => cause,
            DeleteNetworkProfileError::ResourceInUse(ref cause) => cause,
        }
    }
}
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteProfileError {
    fn description(&self) -> &str {
        match *self {
            DeleteProfileError::ConcurrentModification(ref cause) => cause,
            DeleteProfileError::NotFound(ref cause) => cause,
        }
    }
}
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteRoomError {
    fn description(&self) -> &str {
        match *self {
            DeleteRoomError::ConcurrentModification(ref cause) => cause,
            DeleteRoomError::NotFound(ref cause) => cause,
        }
    }
}
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteRoomSkillParameterError {
    fn description(&self) -> &str {
        match *self {
            DeleteRoomSkillParameterError::ConcurrentModification(ref cause) => cause,
        }
    }
}
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteSkillAuthorizationError {
    fn description(&self) -> &str {
        match *self {
            DeleteSkillAuthorizationError::ConcurrentModification(ref cause) => cause,
            DeleteSkillAuthorizationError::NotFound(ref cause) => cause,
        }
    }
}
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteSkillGroupError {
    fn description(&self) -> &str {
        match *self {
            DeleteSkillGroupError::ConcurrentModification(ref cause) => cause,
            DeleteSkillGroupError::NotFound(ref cause) => cause,
        }
    }
}
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteUserError {
    fn description(&self) -> &str {
        match *self {
            DeleteUserError::ConcurrentModification(ref cause) => cause,
            DeleteUserError::NotFound(ref cause) => cause,
        }
    }
}
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DisassociateContactFromAddressBookError {
    fn description(&self) -> &str {
        match *self {}
    }
}
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DisassociateDeviceFromRoomError {
    fn description(&self) -> &str {
        match *self {
            DisassociateDeviceFromRoomError::ConcurrentModification(ref cause) => cause,
            DisassociateDeviceFromRoomError::DeviceNotRegistered(ref cause) => cause,
        }
    }
}
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DisassociateSkillFromSkillGroupError {
    fn description(&self) -> &str {
        match *self {
            DisassociateSkillFromSkillGroupError::ConcurrentModification(ref cause) => cause,
            DisassociateSkillFromSkillGroupError::NotFound(ref cause) => cause,
        }
    }
}
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DisassociateSkillFromUsersError {
    fn description(&self) -> &str {
        match *self {
            DisassociateSkillFromUsersError::ConcurrentModification(ref cause) => cause,
            DisassociateSkillFromUsersError::NotFound(ref cause) => cause,
        }
    }
}
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DisassociateSkillGroupFromRoomError {
    fn description(&self) -> &str {
        match *self {
            DisassociateSkillGroupFromRoomError::ConcurrentModification(ref cause) => cause,
        }
    }
}
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ForgetSmartHomeAppliancesError {
    fn description(&self) -> &str {
        match *self {
            ForgetSmartHomeAppliancesError::NotFound(ref cause) => cause,
        }
    }
}
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetAddressBookError {
    fn description(&self) -> &str {
        match *self {
            GetAddressBookError::NotFound(ref cause) => cause,
        }
    }
}
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetConferencePreferenceError {
    fn description(&self) -> &str {
        match *self {
            GetConferencePreferenceError::NotFound(ref cause) => cause,
        }
    }
}
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetConferenceProviderError {
    fn description(&self) -> &str {
        match *self {
            GetConferenceProviderError::NotFound(ref cause) => cause,
        }
    }
}
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetContactError {
    fn description(&self) -> &str {
        match *self {
            GetContactError::NotFound(ref cause) => cause,
        }
    }
}
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetDeviceError {
    fn description(&self) -> &str {
        match *self {
            GetDeviceError::NotFound(ref cause) => cause,
        }
    }
}
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetGatewayError {
    fn description(&self) -> &str {
        match *self {
            GetGatewayError::NotFound(ref cause) => cause,
        }
    }
}
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetGatewayGroupError {
    fn description(&self) -> &str {
        match *self {
            GetGatewayGroupError::NotFound(ref cause) => cause,
        }
    }
}
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetInvitationConfigurationError {
    fn description(&self) -> &str {
        match *self {
            GetInvitationConfigurationError::NotFound(ref cause) => cause,
        }
    }
}
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetNetworkProfileError {
    fn description(&self) -> &str {
        match *self {
            GetNetworkProfileError::InvalidSecretsManagerResource(ref cause) => cause,
            GetNetworkProfileError::NotFound(ref cause) => cause,
        }
    }
}
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetProfileError {
    fn description(&self) -> &str {
        match *self {
            GetProfileError::NotFound(ref cause) => cause,
        }
    }
}
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetRoomError {
    fn description(&self) -> &str {
        match *self {
            GetRoomError::NotFound(ref cause) => cause,
        }
    }
}
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetRoomSkillParameterError {
    fn description(&self) -> &str {
        match *self {
            GetRoomSkillParameterError::NotFound(ref cause) => cause,
        }
    }
}
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetSkillGroupError {
    fn description(&self) -> &str {
        match *self {
            GetSkillGroupError::NotFound(ref cause) => cause,
        }
    }
}
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListBusinessReportSchedulesError {
    fn description(&self) -> &str {
        match *self {}
    }
}
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListConferenceProvidersError {
    fn description(&self) -> &str {
        match *self {}
    }
}
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListDeviceEventsError {
    fn description(&self) -> &str {
        match *self {
            ListDeviceEventsError::NotFound(ref cause) => cause,
        }
    }
}
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListGatewayGroupsError {
    fn description(&self) -> &str {
        match *self {}
    }
}
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListGatewaysError {
    fn description(&self) -> &str {
        match *self {}
    }
}
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListSkillsError {
    fn description(&self) -> &str {
        match *self {}
    }
}
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListSkillsStoreCategoriesError {
    fn description(&self) -> &str {
        match *self {}
    }
}
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListSkillsStoreSkillsByCategoryError {
    fn description(&self) -> &str {
        match *self {}
    }
}
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListSmartHomeAppliancesError {
    fn description(&self) -> &str {
        match *self {
            ListSmartHomeAppliancesError::NotFound(ref cause) => cause,
        }
    }
}
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListTagsError {
    fn description(&self) -> &str {
        match *self {
            ListTagsError::NotFound(ref cause) => cause,
        }
    }
}
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutConferencePreferenceError {
    fn description(&self) -> &str {
        match *self {
            PutConferencePreferenceError::NotFound(ref cause) => cause,
        }
    }
}
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutInvitationConfigurationError {
    fn description(&self) -> &str {
        match *self {
            PutInvitationConfigurationError::ConcurrentModification(ref cause) => cause,
            PutInvitationConfigurationError::NotFound(ref cause) => cause,
        }
    }
}
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutRoomSkillParameterError {
    fn description(&self) -> &str {
        match *self {
            PutRoomSkillParameterError::ConcurrentModification(ref cause) => cause,
        }
    }
}
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutSkillAuthorizationError {
    fn description(&self) -> &str {
        match *self {
            PutSkillAuthorizationError::ConcurrentModification(ref cause) => cause,
            PutSkillAuthorizationError::Unauthorized(ref cause) => cause,
        }
    }
}
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
        }
    }
}
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RejectSkillError {
    fn description(&self) -> &str {
        match *self {
            RejectSkillError::ConcurrentModification(ref cause) => cause,
            RejectSkillError::NotFound(ref cause) => cause,
        }
    }
}
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ResolveRoomError {
    fn description(&self) -> &str {
        match *self {
            ResolveRoomError::NotFound(ref cause) => cause,
        }
    }
}
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RevokeInvitationError {
    fn description(&self) -> &str {
        match *self {
            RevokeInvitationError::ConcurrentModification(ref cause) => cause,
            RevokeInvitationError::NotFound(ref cause) => cause,
        }
    }
}
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SearchAddressBooksError {
    fn description(&self) -> &str {
        match *self {}
    }
}
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SearchContactsError {
    fn description(&self) -> &str {
        match *self {}
    }
}
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SearchDevicesError {
    fn description(&self) -> &str {
        match *self {}
    }
}
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SearchNetworkProfilesError {
    fn description(&self) -> &str {
        match *self {}
    }
}
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SearchProfilesError {
    fn description(&self) -> &str {
        match *self {}
    }
}
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SearchRoomsError {
    fn description(&self) -> &str {
        match *self {}
    }
}
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SearchSkillGroupsError {
    fn description(&self) -> &str {
        match *self {}
    }
}
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SearchUsersError {
    fn description(&self) -> &str {
        match *self {}
    }
}
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SendAnnouncementError {
    fn description(&self) -> &str {
        match *self {
            SendAnnouncementError::AlreadyExists(ref cause) => cause,
            SendAnnouncementError::LimitExceeded(ref cause) => cause,
        }
    }
}
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
        }
    }
}
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartDeviceSyncError {
    fn description(&self) -> &str {
        match *self {
            StartDeviceSyncError::DeviceNotRegistered(ref cause) => cause,
        }
    }
}
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartSmartHomeApplianceDiscoveryError {
    fn description(&self) -> &str {
        match *self {
            StartSmartHomeApplianceDiscoveryError::NotFound(ref cause) => cause,
        }
    }
}
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for TagResourceError {
    fn description(&self) -> &str {
        match *self {
            TagResourceError::NotFound(ref cause) => cause,
        }
    }
}
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UntagResourceError {
    fn description(&self) -> &str {
        match *self {
            UntagResourceError::NotFound(ref cause) => cause,
        }
    }
}
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
        }
    }
}
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateBusinessReportScheduleError {
    fn description(&self) -> &str {
        match *self {
            UpdateBusinessReportScheduleError::ConcurrentModification(ref cause) => cause,
            UpdateBusinessReportScheduleError::NotFound(ref cause) => cause,
        }
    }
}
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateConferenceProviderError {
    fn description(&self) -> &str {
        match *self {
            UpdateConferenceProviderError::NotFound(ref cause) => cause,
        }
    }
}
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateContactError {
    fn description(&self) -> &str {
        match *self {
            UpdateContactError::ConcurrentModification(ref cause) => cause,
            UpdateContactError::NotFound(ref cause) => cause,
        }
    }
}
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
        }
    }
}
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateGatewayError {
    fn description(&self) -> &str {
        match *self {
            UpdateGatewayError::NameInUse(ref cause) => cause,
            UpdateGatewayError::NotFound(ref cause) => cause,
        }
    }
}
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateGatewayGroupError {
    fn description(&self) -> &str {
        match *self {
            UpdateGatewayGroupError::NameInUse(ref cause) => cause,
            UpdateGatewayGroupError::NotFound(ref cause) => cause,
        }
    }
}
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateNetworkProfileError {
    fn description(&self) -> &str {
        match *self {
            UpdateNetworkProfileError::ConcurrentModification(ref cause) => cause,
            UpdateNetworkProfileError::InvalidCertificateAuthority(ref cause) => cause,
            UpdateNetworkProfileError::InvalidSecretsManagerResource(ref cause) => cause,
            UpdateNetworkProfileError::NameInUse(ref cause) => cause,
            UpdateNetworkProfileError::NotFound(ref cause) => cause,
        }
    }
}
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateRoomError {
    fn description(&self) -> &str {
        match *self {
            UpdateRoomError::NameInUse(ref cause) => cause,
            UpdateRoomError::NotFound(ref cause) => cause,
        }
    }
}
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
        }
    }
}
/// Trait representing the capabilities of the Alexa For Business API. Alexa For Business clients implement this trait.
pub trait AlexaForBusiness {
    /// <p>Associates a skill with the organization under the customer's AWS account. If a skill is private, the user implicitly accepts access to this skill during enablement.</p>
    fn approve_skill(&self, input: ApproveSkillRequest) -> Request<ApproveSkillRequest>;

    /// <p>Associates a contact with a given address book.</p>
    fn associate_contact_with_address_book(
        &self,
        input: AssociateContactWithAddressBookRequest,
    ) -> Request<AssociateContactWithAddressBookRequest>;

    /// <p>Associates a device with the specified network profile.</p>
    fn associate_device_with_network_profile(
        &self,
        input: AssociateDeviceWithNetworkProfileRequest,
    ) -> Request<AssociateDeviceWithNetworkProfileRequest>;

    /// <p>Associates a device with a given room. This applies all the settings from the room profile to the device, and all the skills in any skill groups added to that room. This operation requires the device to be online, or else a manual sync is required. </p>
    fn associate_device_with_room(
        &self,
        input: AssociateDeviceWithRoomRequest,
    ) -> Request<AssociateDeviceWithRoomRequest>;

    /// <p>Associates a skill group with a given room. This enables all skills in the associated skill group on all devices in the room.</p>
    fn associate_skill_group_with_room(
        &self,
        input: AssociateSkillGroupWithRoomRequest,
    ) -> Request<AssociateSkillGroupWithRoomRequest>;

    /// <p>Associates a skill with a skill group.</p>
    fn associate_skill_with_skill_group(
        &self,
        input: AssociateSkillWithSkillGroupRequest,
    ) -> Request<AssociateSkillWithSkillGroupRequest>;

    /// <p>Makes a private skill available for enrolled users to enable on their devices.</p>
    fn associate_skill_with_users(
        &self,
        input: AssociateSkillWithUsersRequest,
    ) -> Request<AssociateSkillWithUsersRequest>;

    /// <p>Creates an address book with the specified details.</p>
    fn create_address_book(
        &self,
        input: CreateAddressBookRequest,
    ) -> Request<CreateAddressBookRequest>;

    /// <p>Creates a recurring schedule for usage reports to deliver to the specified S3 location with a specified daily or weekly interval.</p>
    fn create_business_report_schedule(
        &self,
        input: CreateBusinessReportScheduleRequest,
    ) -> Request<CreateBusinessReportScheduleRequest>;

    /// <p>Adds a new conference provider under the user's AWS account.</p>
    fn create_conference_provider(
        &self,
        input: CreateConferenceProviderRequest,
    ) -> Request<CreateConferenceProviderRequest>;

    /// <p>Creates a contact with the specified details.</p>
    fn create_contact(&self, input: CreateContactRequest) -> Request<CreateContactRequest>;

    /// <p>Creates a gateway group with the specified details.</p>
    fn create_gateway_group(
        &self,
        input: CreateGatewayGroupRequest,
    ) -> Request<CreateGatewayGroupRequest>;

    /// <p>Creates a network profile with the specified details.</p>
    fn create_network_profile(
        &self,
        input: CreateNetworkProfileRequest,
    ) -> Request<CreateNetworkProfileRequest>;

    /// <p>Creates a new room profile with the specified details.</p>
    fn create_profile(&self, input: CreateProfileRequest) -> Request<CreateProfileRequest>;

    /// <p>Creates a room with the specified details.</p>
    fn create_room(&self, input: CreateRoomRequest) -> Request<CreateRoomRequest>;

    /// <p>Creates a skill group with a specified name and description.</p>
    fn create_skill_group(
        &self,
        input: CreateSkillGroupRequest,
    ) -> Request<CreateSkillGroupRequest>;

    /// <p>Creates a user.</p>
    fn create_user(&self, input: CreateUserRequest) -> Request<CreateUserRequest>;

    /// <p>Deletes an address book by the address book ARN.</p>
    fn delete_address_book(
        &self,
        input: DeleteAddressBookRequest,
    ) -> Request<DeleteAddressBookRequest>;

    /// <p>Deletes the recurring report delivery schedule with the specified schedule ARN.</p>
    fn delete_business_report_schedule(
        &self,
        input: DeleteBusinessReportScheduleRequest,
    ) -> Request<DeleteBusinessReportScheduleRequest>;

    /// <p>Deletes a conference provider.</p>
    fn delete_conference_provider(
        &self,
        input: DeleteConferenceProviderRequest,
    ) -> Request<DeleteConferenceProviderRequest>;

    /// <p>Deletes a contact by the contact ARN.</p>
    fn delete_contact(&self, input: DeleteContactRequest) -> Request<DeleteContactRequest>;

    /// <p>Removes a device from Alexa For Business.</p>
    fn delete_device(&self, input: DeleteDeviceRequest) -> Request<DeleteDeviceRequest>;

    /// <p>When this action is called for a specified shared device, it allows authorized users to delete the device's entire previous history of voice input data. This action can be called once every 24 hours for a specific shared device. </p>
    fn delete_device_usage_data(
        &self,
        input: DeleteDeviceUsageDataRequest,
    ) -> Request<DeleteDeviceUsageDataRequest>;

    /// <p>Deletes a gateway group.</p>
    fn delete_gateway_group(
        &self,
        input: DeleteGatewayGroupRequest,
    ) -> Request<DeleteGatewayGroupRequest>;

    /// <p>Deletes a network profile by the network profile ARN.</p>
    fn delete_network_profile(
        &self,
        input: DeleteNetworkProfileRequest,
    ) -> Request<DeleteNetworkProfileRequest>;

    /// <p>Deletes a room profile by the profile ARN.</p>
    fn delete_profile(&self, input: DeleteProfileRequest) -> Request<DeleteProfileRequest>;

    /// <p>Deletes a room by the room ARN.</p>
    fn delete_room(&self, input: DeleteRoomRequest) -> Request<DeleteRoomRequest>;

    /// <p>Deletes room skill parameter details by room, skill, and parameter key ID.</p>
    fn delete_room_skill_parameter(
        &self,
        input: DeleteRoomSkillParameterRequest,
    ) -> Request<DeleteRoomSkillParameterRequest>;

    /// <p>Unlinks a third-party account from a skill.</p>
    fn delete_skill_authorization(
        &self,
        input: DeleteSkillAuthorizationRequest,
    ) -> Request<DeleteSkillAuthorizationRequest>;

    /// <p>Deletes a skill group by skill group ARN.</p>
    fn delete_skill_group(
        &self,
        input: DeleteSkillGroupRequest,
    ) -> Request<DeleteSkillGroupRequest>;

    /// <p>Deletes a specified user by user ARN and enrollment ARN.</p>
    fn delete_user(&self, input: DeleteUserRequest) -> Request<DeleteUserRequest>;

    /// <p>Disassociates a contact from a given address book.</p>
    fn disassociate_contact_from_address_book(
        &self,
        input: DisassociateContactFromAddressBookRequest,
    ) -> Request<DisassociateContactFromAddressBookRequest>;

    /// <p>Disassociates a device from its current room. The device continues to be connected to the Wi-Fi network and is still registered to the account. The device settings and skills are removed from the room.</p>
    fn disassociate_device_from_room(
        &self,
        input: DisassociateDeviceFromRoomRequest,
    ) -> Request<DisassociateDeviceFromRoomRequest>;

    /// <p>Disassociates a skill from a skill group.</p>
    fn disassociate_skill_from_skill_group(
        &self,
        input: DisassociateSkillFromSkillGroupRequest,
    ) -> Request<DisassociateSkillFromSkillGroupRequest>;

    /// <p>Makes a private skill unavailable for enrolled users and prevents them from enabling it on their devices.</p>
    fn disassociate_skill_from_users(
        &self,
        input: DisassociateSkillFromUsersRequest,
    ) -> Request<DisassociateSkillFromUsersRequest>;

    /// <p>Disassociates a skill group from a specified room. This disables all skills in the skill group on all devices in the room.</p>
    fn disassociate_skill_group_from_room(
        &self,
        input: DisassociateSkillGroupFromRoomRequest,
    ) -> Request<DisassociateSkillGroupFromRoomRequest>;

    /// <p>Forgets smart home appliances associated to a room.</p>
    fn forget_smart_home_appliances(
        &self,
        input: ForgetSmartHomeAppliancesRequest,
    ) -> Request<ForgetSmartHomeAppliancesRequest>;

    /// <p>Gets address the book details by the address book ARN.</p>
    fn get_address_book(&self, input: GetAddressBookRequest) -> Request<GetAddressBookRequest>;

    /// <p>Retrieves the existing conference preferences.</p>
    fn get_conference_preference(&self) -> Request<GetConferencePreferenceRequest>;

    /// <p>Gets details about a specific conference provider.</p>
    fn get_conference_provider(
        &self,
        input: GetConferenceProviderRequest,
    ) -> Request<GetConferenceProviderRequest>;

    /// <p>Gets the contact details by the contact ARN.</p>
    fn get_contact(&self, input: GetContactRequest) -> Request<GetContactRequest>;

    /// <p>Gets the details of a device by device ARN.</p>
    fn get_device(&self, input: GetDeviceRequest) -> Request<GetDeviceRequest>;

    /// <p>Retrieves the details of a gateway.</p>
    fn get_gateway(&self, input: GetGatewayRequest) -> Request<GetGatewayRequest>;

    /// <p>Retrieves the details of a gateway group.</p>
    fn get_gateway_group(&self, input: GetGatewayGroupRequest) -> Request<GetGatewayGroupRequest>;

    /// <p>Retrieves the configured values for the user enrollment invitation email template.</p>
    fn get_invitation_configuration(&self) -> Request<GetInvitationConfigurationRequest>;

    /// <p>Gets the network profile details by the network profile ARN.</p>
    fn get_network_profile(
        &self,
        input: GetNetworkProfileRequest,
    ) -> Request<GetNetworkProfileRequest>;

    /// <p>Gets the details of a room profile by profile ARN.</p>
    fn get_profile(&self, input: GetProfileRequest) -> Request<GetProfileRequest>;

    /// <p>Gets room details by room ARN.</p>
    fn get_room(&self, input: GetRoomRequest) -> Request<GetRoomRequest>;

    /// <p>Gets room skill parameter details by room, skill, and parameter key ARN.</p>
    fn get_room_skill_parameter(
        &self,
        input: GetRoomSkillParameterRequest,
    ) -> Request<GetRoomSkillParameterRequest>;

    /// <p>Gets skill group details by skill group ARN.</p>
    fn get_skill_group(&self, input: GetSkillGroupRequest) -> Request<GetSkillGroupRequest>;

    /// <p>Lists the details of the schedules that a user configured.</p>
    fn list_business_report_schedules(
        &self,
        input: ListBusinessReportSchedulesRequest,
    ) -> Request<ListBusinessReportSchedulesRequest>;

    /// <p>Lists conference providers under a specific AWS account.</p>
    fn list_conference_providers(
        &self,
        input: ListConferenceProvidersRequest,
    ) -> Request<ListConferenceProvidersRequest>;

    /// <p>Lists the device event history, including device connection status, for up to 30 days.</p>
    fn list_device_events(
        &self,
        input: ListDeviceEventsRequest,
    ) -> Request<ListDeviceEventsRequest>;

    /// <p>Retrieves a list of gateway group summaries. Use GetGatewayGroup to retrieve details of a specific gateway group.</p>
    fn list_gateway_groups(
        &self,
        input: ListGatewayGroupsRequest,
    ) -> Request<ListGatewayGroupsRequest>;

    /// <p>Retrieves a list of gateway summaries. Use GetGateway to retrieve details of a specific gateway. An optional gateway group ARN can be provided to only retrieve gateway summaries of gateways that are associated with that gateway group ARN.</p>
    fn list_gateways(&self, input: ListGatewaysRequest) -> Request<ListGatewaysRequest>;

    /// <p>Lists all enabled skills in a specific skill group.</p>
    fn list_skills(&self, input: ListSkillsRequest) -> Request<ListSkillsRequest>;

    /// <p>Lists all categories in the Alexa skill store.</p>
    fn list_skills_store_categories(
        &self,
        input: ListSkillsStoreCategoriesRequest,
    ) -> Request<ListSkillsStoreCategoriesRequest>;

    /// <p>Lists all skills in the Alexa skill store by category.</p>
    fn list_skills_store_skills_by_category(
        &self,
        input: ListSkillsStoreSkillsByCategoryRequest,
    ) -> Request<ListSkillsStoreSkillsByCategoryRequest>;

    /// <p>Lists all of the smart home appliances associated with a room.</p>
    fn list_smart_home_appliances(
        &self,
        input: ListSmartHomeAppliancesRequest,
    ) -> Request<ListSmartHomeAppliancesRequest>;

    /// <p>Lists all tags for the specified resource.</p>
    fn list_tags(&self, input: ListTagsRequest) -> Request<ListTagsRequest>;

    /// <p>Sets the conference preferences on a specific conference provider at the account level.</p>
    fn put_conference_preference(
        &self,
        input: PutConferencePreferenceRequest,
    ) -> Request<PutConferencePreferenceRequest>;

    /// <p>Configures the email template for the user enrollment invitation with the specified attributes.</p>
    fn put_invitation_configuration(
        &self,
        input: PutInvitationConfigurationRequest,
    ) -> Request<PutInvitationConfigurationRequest>;

    /// <p>Updates room skill parameter details by room, skill, and parameter key ID. Not all skills have a room skill parameter.</p>
    fn put_room_skill_parameter(
        &self,
        input: PutRoomSkillParameterRequest,
    ) -> Request<PutRoomSkillParameterRequest>;

    /// <p>Links a user's account to a third-party skill provider. If this API operation is called by an assumed IAM role, the skill being linked must be a private skill. Also, the skill must be owned by the AWS account that assumed the IAM role.</p>
    fn put_skill_authorization(
        &self,
        input: PutSkillAuthorizationRequest,
    ) -> Request<PutSkillAuthorizationRequest>;

    /// <p>Registers an Alexa-enabled device built by an Original Equipment Manufacturer (OEM) using Alexa Voice Service (AVS).</p>
    fn register_avs_device(
        &self,
        input: RegisterAVSDeviceRequest,
    ) -> Request<RegisterAVSDeviceRequest>;

    /// <p>Disassociates a skill from the organization under a user's AWS account. If the skill is a private skill, it moves to an AcceptStatus of PENDING. Any private or public skill that is rejected can be added later by calling the ApproveSkill API. </p>
    fn reject_skill(&self, input: RejectSkillRequest) -> Request<RejectSkillRequest>;

    /// <p>Determines the details for the room from which a skill request was invoked. This operation is used by skill developers.</p>
    fn resolve_room(&self, input: ResolveRoomRequest) -> Request<ResolveRoomRequest>;

    /// <p>Revokes an invitation and invalidates the enrollment URL.</p>
    fn revoke_invitation(&self, input: RevokeInvitationRequest)
        -> Request<RevokeInvitationRequest>;

    /// <p>Searches address books and lists the ones that meet a set of filter and sort criteria.</p>
    fn search_address_books(
        &self,
        input: SearchAddressBooksRequest,
    ) -> Request<SearchAddressBooksRequest>;

    /// <p>Searches contacts and lists the ones that meet a set of filter and sort criteria.</p>
    fn search_contacts(&self, input: SearchContactsRequest) -> Request<SearchContactsRequest>;

    /// <p>Searches devices and lists the ones that meet a set of filter criteria.</p>
    fn search_devices(&self, input: SearchDevicesRequest) -> Request<SearchDevicesRequest>;

    /// <p>Searches network profiles and lists the ones that meet a set of filter and sort criteria.</p>
    fn search_network_profiles(
        &self,
        input: SearchNetworkProfilesRequest,
    ) -> Request<SearchNetworkProfilesRequest>;

    /// <p>Searches room profiles and lists the ones that meet a set of filter criteria.</p>
    fn search_profiles(&self, input: SearchProfilesRequest) -> Request<SearchProfilesRequest>;

    /// <p>Searches rooms and lists the ones that meet a set of filter and sort criteria.</p>
    fn search_rooms(&self, input: SearchRoomsRequest) -> Request<SearchRoomsRequest>;

    /// <p>Searches skill groups and lists the ones that meet a set of filter and sort criteria.</p>
    fn search_skill_groups(
        &self,
        input: SearchSkillGroupsRequest,
    ) -> Request<SearchSkillGroupsRequest>;

    /// <p>Searches users and lists the ones that meet a set of filter and sort criteria.</p>
    fn search_users(&self, input: SearchUsersRequest) -> Request<SearchUsersRequest>;

    /// <p>Triggers an asynchronous flow to send text, SSML, or audio announcements to rooms that are identified by a search or filter. </p>
    fn send_announcement(&self, input: SendAnnouncementRequest)
        -> Request<SendAnnouncementRequest>;

    /// <p>Sends an enrollment invitation email with a URL to a user. The URL is valid for 72 hours or until you call this operation again, whichever comes first. </p>
    fn send_invitation(&self, input: SendInvitationRequest) -> Request<SendInvitationRequest>;

    /// <p><p>Resets a device and its account to the known default settings. This clears all information and settings set by previous users in the following ways:</p> <ul> <li> <p>Bluetooth - This unpairs all bluetooth devices paired with your echo device.</p> </li> <li> <p>Volume - This resets the echo device&#39;s volume to the default value.</p> </li> <li> <p>Notifications - This clears all notifications from your echo device.</p> </li> <li> <p>Lists - This clears all to-do items from your echo device.</p> </li> <li> <p>Settings - This internally syncs the room&#39;s profile (if the device is assigned to a room), contacts, address books, delegation access for account linking, and communications (if enabled on the room profile).</p> </li> </ul></p>
    fn start_device_sync(&self, input: StartDeviceSyncRequest) -> Request<StartDeviceSyncRequest>;

    /// <p>Initiates the discovery of any smart home appliances associated with the room.</p>
    fn start_smart_home_appliance_discovery(
        &self,
        input: StartSmartHomeApplianceDiscoveryRequest,
    ) -> Request<StartSmartHomeApplianceDiscoveryRequest>;

    /// <p>Adds metadata tags to a specified resource.</p>
    fn tag_resource(&self, input: TagResourceRequest) -> Request<TagResourceRequest>;

    /// <p>Removes metadata tags from a specified resource.</p>
    fn untag_resource(&self, input: UntagResourceRequest) -> Request<UntagResourceRequest>;

    /// <p>Updates address book details by the address book ARN.</p>
    fn update_address_book(
        &self,
        input: UpdateAddressBookRequest,
    ) -> Request<UpdateAddressBookRequest>;

    /// <p>Updates the configuration of the report delivery schedule with the specified schedule ARN.</p>
    fn update_business_report_schedule(
        &self,
        input: UpdateBusinessReportScheduleRequest,
    ) -> Request<UpdateBusinessReportScheduleRequest>;

    /// <p>Updates an existing conference provider's settings.</p>
    fn update_conference_provider(
        &self,
        input: UpdateConferenceProviderRequest,
    ) -> Request<UpdateConferenceProviderRequest>;

    /// <p>Updates the contact details by the contact ARN.</p>
    fn update_contact(&self, input: UpdateContactRequest) -> Request<UpdateContactRequest>;

    /// <p>Updates the device name by device ARN.</p>
    fn update_device(&self, input: UpdateDeviceRequest) -> Request<UpdateDeviceRequest>;

    /// <p>Updates the details of a gateway. If any optional field is not provided, the existing corresponding value is left unmodified.</p>
    fn update_gateway(&self, input: UpdateGatewayRequest) -> Request<UpdateGatewayRequest>;

    /// <p>Updates the details of a gateway group. If any optional field is not provided, the existing corresponding value is left unmodified.</p>
    fn update_gateway_group(
        &self,
        input: UpdateGatewayGroupRequest,
    ) -> Request<UpdateGatewayGroupRequest>;

    /// <p>Updates a network profile by the network profile ARN.</p>
    fn update_network_profile(
        &self,
        input: UpdateNetworkProfileRequest,
    ) -> Request<UpdateNetworkProfileRequest>;

    /// <p>Updates an existing room profile by room profile ARN.</p>
    fn update_profile(&self, input: UpdateProfileRequest) -> Request<UpdateProfileRequest>;

    /// <p>Updates room details by room ARN.</p>
    fn update_room(&self, input: UpdateRoomRequest) -> Request<UpdateRoomRequest>;

    /// <p>Updates skill group details by skill group ARN.</p>
    fn update_skill_group(
        &self,
        input: UpdateSkillGroupRequest,
    ) -> Request<UpdateSkillGroupRequest>;
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
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        AlexaForBusinessClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }
}

impl AlexaForBusiness for AlexaForBusinessClient {
    /// <p>Associates a skill with the organization under the customer's AWS account. If a skill is private, the user implicitly accepts access to this skill during enablement.</p>
    fn approve_skill(&self, input: ApproveSkillRequest) -> Request<ApproveSkillRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Associates a contact with a given address book.</p>
    fn associate_contact_with_address_book(
        &self,
        input: AssociateContactWithAddressBookRequest,
    ) -> Request<AssociateContactWithAddressBookRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Associates a device with the specified network profile.</p>
    fn associate_device_with_network_profile(
        &self,
        input: AssociateDeviceWithNetworkProfileRequest,
    ) -> Request<AssociateDeviceWithNetworkProfileRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Associates a device with a given room. This applies all the settings from the room profile to the device, and all the skills in any skill groups added to that room. This operation requires the device to be online, or else a manual sync is required. </p>
    fn associate_device_with_room(
        &self,
        input: AssociateDeviceWithRoomRequest,
    ) -> Request<AssociateDeviceWithRoomRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Associates a skill group with a given room. This enables all skills in the associated skill group on all devices in the room.</p>
    fn associate_skill_group_with_room(
        &self,
        input: AssociateSkillGroupWithRoomRequest,
    ) -> Request<AssociateSkillGroupWithRoomRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Associates a skill with a skill group.</p>
    fn associate_skill_with_skill_group(
        &self,
        input: AssociateSkillWithSkillGroupRequest,
    ) -> Request<AssociateSkillWithSkillGroupRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Makes a private skill available for enrolled users to enable on their devices.</p>
    fn associate_skill_with_users(
        &self,
        input: AssociateSkillWithUsersRequest,
    ) -> Request<AssociateSkillWithUsersRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Creates an address book with the specified details.</p>
    fn create_address_book(
        &self,
        input: CreateAddressBookRequest,
    ) -> Request<CreateAddressBookRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Creates a recurring schedule for usage reports to deliver to the specified S3 location with a specified daily or weekly interval.</p>
    fn create_business_report_schedule(
        &self,
        input: CreateBusinessReportScheduleRequest,
    ) -> Request<CreateBusinessReportScheduleRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Adds a new conference provider under the user's AWS account.</p>
    fn create_conference_provider(
        &self,
        input: CreateConferenceProviderRequest,
    ) -> Request<CreateConferenceProviderRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Creates a contact with the specified details.</p>
    fn create_contact(&self, input: CreateContactRequest) -> Request<CreateContactRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Creates a gateway group with the specified details.</p>
    fn create_gateway_group(
        &self,
        input: CreateGatewayGroupRequest,
    ) -> Request<CreateGatewayGroupRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Creates a network profile with the specified details.</p>
    fn create_network_profile(
        &self,
        input: CreateNetworkProfileRequest,
    ) -> Request<CreateNetworkProfileRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Creates a new room profile with the specified details.</p>
    fn create_profile(&self, input: CreateProfileRequest) -> Request<CreateProfileRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Creates a room with the specified details.</p>
    fn create_room(&self, input: CreateRoomRequest) -> Request<CreateRoomRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Creates a skill group with a specified name and description.</p>
    fn create_skill_group(
        &self,
        input: CreateSkillGroupRequest,
    ) -> Request<CreateSkillGroupRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Creates a user.</p>
    fn create_user(&self, input: CreateUserRequest) -> Request<CreateUserRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Deletes an address book by the address book ARN.</p>
    fn delete_address_book(
        &self,
        input: DeleteAddressBookRequest,
    ) -> Request<DeleteAddressBookRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Deletes the recurring report delivery schedule with the specified schedule ARN.</p>
    fn delete_business_report_schedule(
        &self,
        input: DeleteBusinessReportScheduleRequest,
    ) -> Request<DeleteBusinessReportScheduleRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Deletes a conference provider.</p>
    fn delete_conference_provider(
        &self,
        input: DeleteConferenceProviderRequest,
    ) -> Request<DeleteConferenceProviderRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Deletes a contact by the contact ARN.</p>
    fn delete_contact(&self, input: DeleteContactRequest) -> Request<DeleteContactRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Removes a device from Alexa For Business.</p>
    fn delete_device(&self, input: DeleteDeviceRequest) -> Request<DeleteDeviceRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>When this action is called for a specified shared device, it allows authorized users to delete the device's entire previous history of voice input data. This action can be called once every 24 hours for a specific shared device. </p>
    fn delete_device_usage_data(
        &self,
        input: DeleteDeviceUsageDataRequest,
    ) -> Request<DeleteDeviceUsageDataRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Deletes a gateway group.</p>
    fn delete_gateway_group(
        &self,
        input: DeleteGatewayGroupRequest,
    ) -> Request<DeleteGatewayGroupRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Deletes a network profile by the network profile ARN.</p>
    fn delete_network_profile(
        &self,
        input: DeleteNetworkProfileRequest,
    ) -> Request<DeleteNetworkProfileRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Deletes a room profile by the profile ARN.</p>
    fn delete_profile(&self, input: DeleteProfileRequest) -> Request<DeleteProfileRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Deletes a room by the room ARN.</p>
    fn delete_room(&self, input: DeleteRoomRequest) -> Request<DeleteRoomRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Deletes room skill parameter details by room, skill, and parameter key ID.</p>
    fn delete_room_skill_parameter(
        &self,
        input: DeleteRoomSkillParameterRequest,
    ) -> Request<DeleteRoomSkillParameterRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Unlinks a third-party account from a skill.</p>
    fn delete_skill_authorization(
        &self,
        input: DeleteSkillAuthorizationRequest,
    ) -> Request<DeleteSkillAuthorizationRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Deletes a skill group by skill group ARN.</p>
    fn delete_skill_group(
        &self,
        input: DeleteSkillGroupRequest,
    ) -> Request<DeleteSkillGroupRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Deletes a specified user by user ARN and enrollment ARN.</p>
    fn delete_user(&self, input: DeleteUserRequest) -> Request<DeleteUserRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Disassociates a contact from a given address book.</p>
    fn disassociate_contact_from_address_book(
        &self,
        input: DisassociateContactFromAddressBookRequest,
    ) -> Request<DisassociateContactFromAddressBookRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Disassociates a device from its current room. The device continues to be connected to the Wi-Fi network and is still registered to the account. The device settings and skills are removed from the room.</p>
    fn disassociate_device_from_room(
        &self,
        input: DisassociateDeviceFromRoomRequest,
    ) -> Request<DisassociateDeviceFromRoomRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Disassociates a skill from a skill group.</p>
    fn disassociate_skill_from_skill_group(
        &self,
        input: DisassociateSkillFromSkillGroupRequest,
    ) -> Request<DisassociateSkillFromSkillGroupRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Makes a private skill unavailable for enrolled users and prevents them from enabling it on their devices.</p>
    fn disassociate_skill_from_users(
        &self,
        input: DisassociateSkillFromUsersRequest,
    ) -> Request<DisassociateSkillFromUsersRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Disassociates a skill group from a specified room. This disables all skills in the skill group on all devices in the room.</p>
    fn disassociate_skill_group_from_room(
        &self,
        input: DisassociateSkillGroupFromRoomRequest,
    ) -> Request<DisassociateSkillGroupFromRoomRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Forgets smart home appliances associated to a room.</p>
    fn forget_smart_home_appliances(
        &self,
        input: ForgetSmartHomeAppliancesRequest,
    ) -> Request<ForgetSmartHomeAppliancesRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Gets address the book details by the address book ARN.</p>
    fn get_address_book(&self, input: GetAddressBookRequest) -> Request<GetAddressBookRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Retrieves the existing conference preferences.</p>
    fn get_conference_preference(&self) -> Request<GetConferencePreferenceRequest> {
        Request::new(
            GetConferencePreferenceRequest {},
            self.region.clone(),
            self.client.clone(),
        )
    }

    /// <p>Gets details about a specific conference provider.</p>
    fn get_conference_provider(
        &self,
        input: GetConferenceProviderRequest,
    ) -> Request<GetConferenceProviderRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Gets the contact details by the contact ARN.</p>
    fn get_contact(&self, input: GetContactRequest) -> Request<GetContactRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Gets the details of a device by device ARN.</p>
    fn get_device(&self, input: GetDeviceRequest) -> Request<GetDeviceRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Retrieves the details of a gateway.</p>
    fn get_gateway(&self, input: GetGatewayRequest) -> Request<GetGatewayRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Retrieves the details of a gateway group.</p>
    fn get_gateway_group(&self, input: GetGatewayGroupRequest) -> Request<GetGatewayGroupRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Retrieves the configured values for the user enrollment invitation email template.</p>
    fn get_invitation_configuration(&self) -> Request<GetInvitationConfigurationRequest> {
        Request::new(
            GetInvitationConfigurationRequest {},
            self.region.clone(),
            self.client.clone(),
        )
    }

    /// <p>Gets the network profile details by the network profile ARN.</p>
    fn get_network_profile(
        &self,
        input: GetNetworkProfileRequest,
    ) -> Request<GetNetworkProfileRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Gets the details of a room profile by profile ARN.</p>
    fn get_profile(&self, input: GetProfileRequest) -> Request<GetProfileRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Gets room details by room ARN.</p>
    fn get_room(&self, input: GetRoomRequest) -> Request<GetRoomRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Gets room skill parameter details by room, skill, and parameter key ARN.</p>
    fn get_room_skill_parameter(
        &self,
        input: GetRoomSkillParameterRequest,
    ) -> Request<GetRoomSkillParameterRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Gets skill group details by skill group ARN.</p>
    fn get_skill_group(&self, input: GetSkillGroupRequest) -> Request<GetSkillGroupRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Lists the details of the schedules that a user configured.</p>
    fn list_business_report_schedules(
        &self,
        input: ListBusinessReportSchedulesRequest,
    ) -> Request<ListBusinessReportSchedulesRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Lists conference providers under a specific AWS account.</p>
    fn list_conference_providers(
        &self,
        input: ListConferenceProvidersRequest,
    ) -> Request<ListConferenceProvidersRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Lists the device event history, including device connection status, for up to 30 days.</p>
    fn list_device_events(
        &self,
        input: ListDeviceEventsRequest,
    ) -> Request<ListDeviceEventsRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Retrieves a list of gateway group summaries. Use GetGatewayGroup to retrieve details of a specific gateway group.</p>
    fn list_gateway_groups(
        &self,
        input: ListGatewayGroupsRequest,
    ) -> Request<ListGatewayGroupsRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Retrieves a list of gateway summaries. Use GetGateway to retrieve details of a specific gateway. An optional gateway group ARN can be provided to only retrieve gateway summaries of gateways that are associated with that gateway group ARN.</p>
    fn list_gateways(&self, input: ListGatewaysRequest) -> Request<ListGatewaysRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Lists all enabled skills in a specific skill group.</p>
    fn list_skills(&self, input: ListSkillsRequest) -> Request<ListSkillsRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Lists all categories in the Alexa skill store.</p>
    fn list_skills_store_categories(
        &self,
        input: ListSkillsStoreCategoriesRequest,
    ) -> Request<ListSkillsStoreCategoriesRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Lists all skills in the Alexa skill store by category.</p>
    fn list_skills_store_skills_by_category(
        &self,
        input: ListSkillsStoreSkillsByCategoryRequest,
    ) -> Request<ListSkillsStoreSkillsByCategoryRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Lists all of the smart home appliances associated with a room.</p>
    fn list_smart_home_appliances(
        &self,
        input: ListSmartHomeAppliancesRequest,
    ) -> Request<ListSmartHomeAppliancesRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Lists all tags for the specified resource.</p>
    fn list_tags(&self, input: ListTagsRequest) -> Request<ListTagsRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Sets the conference preferences on a specific conference provider at the account level.</p>
    fn put_conference_preference(
        &self,
        input: PutConferencePreferenceRequest,
    ) -> Request<PutConferencePreferenceRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Configures the email template for the user enrollment invitation with the specified attributes.</p>
    fn put_invitation_configuration(
        &self,
        input: PutInvitationConfigurationRequest,
    ) -> Request<PutInvitationConfigurationRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Updates room skill parameter details by room, skill, and parameter key ID. Not all skills have a room skill parameter.</p>
    fn put_room_skill_parameter(
        &self,
        input: PutRoomSkillParameterRequest,
    ) -> Request<PutRoomSkillParameterRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Links a user's account to a third-party skill provider. If this API operation is called by an assumed IAM role, the skill being linked must be a private skill. Also, the skill must be owned by the AWS account that assumed the IAM role.</p>
    fn put_skill_authorization(
        &self,
        input: PutSkillAuthorizationRequest,
    ) -> Request<PutSkillAuthorizationRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Registers an Alexa-enabled device built by an Original Equipment Manufacturer (OEM) using Alexa Voice Service (AVS).</p>
    fn register_avs_device(
        &self,
        input: RegisterAVSDeviceRequest,
    ) -> Request<RegisterAVSDeviceRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Disassociates a skill from the organization under a user's AWS account. If the skill is a private skill, it moves to an AcceptStatus of PENDING. Any private or public skill that is rejected can be added later by calling the ApproveSkill API. </p>
    fn reject_skill(&self, input: RejectSkillRequest) -> Request<RejectSkillRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Determines the details for the room from which a skill request was invoked. This operation is used by skill developers.</p>
    fn resolve_room(&self, input: ResolveRoomRequest) -> Request<ResolveRoomRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Revokes an invitation and invalidates the enrollment URL.</p>
    fn revoke_invitation(
        &self,
        input: RevokeInvitationRequest,
    ) -> Request<RevokeInvitationRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Searches address books and lists the ones that meet a set of filter and sort criteria.</p>
    fn search_address_books(
        &self,
        input: SearchAddressBooksRequest,
    ) -> Request<SearchAddressBooksRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Searches contacts and lists the ones that meet a set of filter and sort criteria.</p>
    fn search_contacts(&self, input: SearchContactsRequest) -> Request<SearchContactsRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Searches devices and lists the ones that meet a set of filter criteria.</p>
    fn search_devices(&self, input: SearchDevicesRequest) -> Request<SearchDevicesRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Searches network profiles and lists the ones that meet a set of filter and sort criteria.</p>
    fn search_network_profiles(
        &self,
        input: SearchNetworkProfilesRequest,
    ) -> Request<SearchNetworkProfilesRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Searches room profiles and lists the ones that meet a set of filter criteria.</p>
    fn search_profiles(&self, input: SearchProfilesRequest) -> Request<SearchProfilesRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Searches rooms and lists the ones that meet a set of filter and sort criteria.</p>
    fn search_rooms(&self, input: SearchRoomsRequest) -> Request<SearchRoomsRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Searches skill groups and lists the ones that meet a set of filter and sort criteria.</p>
    fn search_skill_groups(
        &self,
        input: SearchSkillGroupsRequest,
    ) -> Request<SearchSkillGroupsRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Searches users and lists the ones that meet a set of filter and sort criteria.</p>
    fn search_users(&self, input: SearchUsersRequest) -> Request<SearchUsersRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Triggers an asynchronous flow to send text, SSML, or audio announcements to rooms that are identified by a search or filter. </p>
    fn send_announcement(
        &self,
        input: SendAnnouncementRequest,
    ) -> Request<SendAnnouncementRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Sends an enrollment invitation email with a URL to a user. The URL is valid for 72 hours or until you call this operation again, whichever comes first. </p>
    fn send_invitation(&self, input: SendInvitationRequest) -> Request<SendInvitationRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p><p>Resets a device and its account to the known default settings. This clears all information and settings set by previous users in the following ways:</p> <ul> <li> <p>Bluetooth - This unpairs all bluetooth devices paired with your echo device.</p> </li> <li> <p>Volume - This resets the echo device&#39;s volume to the default value.</p> </li> <li> <p>Notifications - This clears all notifications from your echo device.</p> </li> <li> <p>Lists - This clears all to-do items from your echo device.</p> </li> <li> <p>Settings - This internally syncs the room&#39;s profile (if the device is assigned to a room), contacts, address books, delegation access for account linking, and communications (if enabled on the room profile).</p> </li> </ul></p>
    fn start_device_sync(&self, input: StartDeviceSyncRequest) -> Request<StartDeviceSyncRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Initiates the discovery of any smart home appliances associated with the room.</p>
    fn start_smart_home_appliance_discovery(
        &self,
        input: StartSmartHomeApplianceDiscoveryRequest,
    ) -> Request<StartSmartHomeApplianceDiscoveryRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Adds metadata tags to a specified resource.</p>
    fn tag_resource(&self, input: TagResourceRequest) -> Request<TagResourceRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Removes metadata tags from a specified resource.</p>
    fn untag_resource(&self, input: UntagResourceRequest) -> Request<UntagResourceRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Updates address book details by the address book ARN.</p>
    fn update_address_book(
        &self,
        input: UpdateAddressBookRequest,
    ) -> Request<UpdateAddressBookRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Updates the configuration of the report delivery schedule with the specified schedule ARN.</p>
    fn update_business_report_schedule(
        &self,
        input: UpdateBusinessReportScheduleRequest,
    ) -> Request<UpdateBusinessReportScheduleRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Updates an existing conference provider's settings.</p>
    fn update_conference_provider(
        &self,
        input: UpdateConferenceProviderRequest,
    ) -> Request<UpdateConferenceProviderRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Updates the contact details by the contact ARN.</p>
    fn update_contact(&self, input: UpdateContactRequest) -> Request<UpdateContactRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Updates the device name by device ARN.</p>
    fn update_device(&self, input: UpdateDeviceRequest) -> Request<UpdateDeviceRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Updates the details of a gateway. If any optional field is not provided, the existing corresponding value is left unmodified.</p>
    fn update_gateway(&self, input: UpdateGatewayRequest) -> Request<UpdateGatewayRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Updates the details of a gateway group. If any optional field is not provided, the existing corresponding value is left unmodified.</p>
    fn update_gateway_group(
        &self,
        input: UpdateGatewayGroupRequest,
    ) -> Request<UpdateGatewayGroupRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Updates a network profile by the network profile ARN.</p>
    fn update_network_profile(
        &self,
        input: UpdateNetworkProfileRequest,
    ) -> Request<UpdateNetworkProfileRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Updates an existing room profile by room profile ARN.</p>
    fn update_profile(&self, input: UpdateProfileRequest) -> Request<UpdateProfileRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Updates room details by room ARN.</p>
    fn update_room(&self, input: UpdateRoomRequest) -> Request<UpdateRoomRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Updates skill group details by skill group ARN.</p>
    fn update_skill_group(
        &self,
        input: UpdateSkillGroupRequest,
    ) -> Request<UpdateSkillGroupRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }
}

impl ServiceRequest for ApproveSkillRequest {
    type Output = ApproveSkillResponse;
    type Error = ApproveSkillError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "a4b", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.ApproveSkill");
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ApproveSkillResponse, _>()
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
}

impl ServiceRequest for AssociateContactWithAddressBookRequest {
    type Output = AssociateContactWithAddressBookResponse;
    type Error = AssociateContactWithAddressBookError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "a4b", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AlexaForBusiness.AssociateContactWithAddressBook",
        );
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<AssociateContactWithAddressBookResponse, _>()
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
}

impl ServiceRequest for AssociateDeviceWithNetworkProfileRequest {
    type Output = AssociateDeviceWithNetworkProfileResponse;
    type Error = AssociateDeviceWithNetworkProfileError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "a4b", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AlexaForBusiness.AssociateDeviceWithNetworkProfile",
        );
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<AssociateDeviceWithNetworkProfileResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(AssociateDeviceWithNetworkProfileError::from_response(
                        response,
                    ))
                }))
            }
        })
    }
}

impl ServiceRequest for AssociateDeviceWithRoomRequest {
    type Output = AssociateDeviceWithRoomResponse;
    type Error = AssociateDeviceWithRoomError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "a4b", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.AssociateDeviceWithRoom");
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<AssociateDeviceWithRoomResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(AssociateDeviceWithRoomError::from_response(response))
                }))
            }
        })
    }
}

impl ServiceRequest for AssociateSkillGroupWithRoomRequest {
    type Output = AssociateSkillGroupWithRoomResponse;
    type Error = AssociateSkillGroupWithRoomError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "a4b", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AlexaForBusiness.AssociateSkillGroupWithRoom",
        );
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<AssociateSkillGroupWithRoomResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(AssociateSkillGroupWithRoomError::from_response(response))
                }))
            }
        })
    }
}

impl ServiceRequest for AssociateSkillWithSkillGroupRequest {
    type Output = AssociateSkillWithSkillGroupResponse;
    type Error = AssociateSkillWithSkillGroupError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "a4b", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AlexaForBusiness.AssociateSkillWithSkillGroup",
        );
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<AssociateSkillWithSkillGroupResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(AssociateSkillWithSkillGroupError::from_response(response))
                }))
            }
        })
    }
}

impl ServiceRequest for AssociateSkillWithUsersRequest {
    type Output = AssociateSkillWithUsersResponse;
    type Error = AssociateSkillWithUsersError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "a4b", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.AssociateSkillWithUsers");
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<AssociateSkillWithUsersResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(AssociateSkillWithUsersError::from_response(response))
                }))
            }
        })
    }
}

impl ServiceRequest for CreateAddressBookRequest {
    type Output = CreateAddressBookResponse;
    type Error = CreateAddressBookError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "a4b", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.CreateAddressBook");
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateAddressBookResponse, _>()
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
}

impl ServiceRequest for CreateBusinessReportScheduleRequest {
    type Output = CreateBusinessReportScheduleResponse;
    type Error = CreateBusinessReportScheduleError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "a4b", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AlexaForBusiness.CreateBusinessReportSchedule",
        );
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateBusinessReportScheduleResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateBusinessReportScheduleError::from_response(response))
                }))
            }
        })
    }
}

impl ServiceRequest for CreateConferenceProviderRequest {
    type Output = CreateConferenceProviderResponse;
    type Error = CreateConferenceProviderError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "a4b", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.CreateConferenceProvider");
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateConferenceProviderResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateConferenceProviderError::from_response(response))
                }))
            }
        })
    }
}

impl ServiceRequest for CreateContactRequest {
    type Output = CreateContactResponse;
    type Error = CreateContactError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "a4b", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.CreateContact");
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateContactResponse, _>()
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
}

impl ServiceRequest for CreateGatewayGroupRequest {
    type Output = CreateGatewayGroupResponse;
    type Error = CreateGatewayGroupError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "a4b", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.CreateGatewayGroup");
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateGatewayGroupResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateGatewayGroupError::from_response(response))),
                )
            }
        })
    }
}

impl ServiceRequest for CreateNetworkProfileRequest {
    type Output = CreateNetworkProfileResponse;
    type Error = CreateNetworkProfileError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "a4b", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.CreateNetworkProfile");
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateNetworkProfileResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(CreateNetworkProfileError::from_response(response))
                    }),
                )
            }
        })
    }
}

impl ServiceRequest for CreateProfileRequest {
    type Output = CreateProfileResponse;
    type Error = CreateProfileError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "a4b", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.CreateProfile");
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateProfileResponse, _>()
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
}

impl ServiceRequest for CreateRoomRequest {
    type Output = CreateRoomResponse;
    type Error = CreateRoomError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "a4b", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.CreateRoom");
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateRoomResponse, _>()
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
}

impl ServiceRequest for CreateSkillGroupRequest {
    type Output = CreateSkillGroupResponse;
    type Error = CreateSkillGroupError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "a4b", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.CreateSkillGroup");
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateSkillGroupResponse, _>()
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
}

impl ServiceRequest for CreateUserRequest {
    type Output = CreateUserResponse;
    type Error = CreateUserError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "a4b", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.CreateUser");
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateUserResponse, _>()
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
}

impl ServiceRequest for DeleteAddressBookRequest {
    type Output = DeleteAddressBookResponse;
    type Error = DeleteAddressBookError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "a4b", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.DeleteAddressBook");
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteAddressBookResponse, _>()
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
}

impl ServiceRequest for DeleteBusinessReportScheduleRequest {
    type Output = DeleteBusinessReportScheduleResponse;
    type Error = DeleteBusinessReportScheduleError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "a4b", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AlexaForBusiness.DeleteBusinessReportSchedule",
        );
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteBusinessReportScheduleResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteBusinessReportScheduleError::from_response(response))
                }))
            }
        })
    }
}

impl ServiceRequest for DeleteConferenceProviderRequest {
    type Output = DeleteConferenceProviderResponse;
    type Error = DeleteConferenceProviderError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "a4b", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.DeleteConferenceProvider");
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteConferenceProviderResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteConferenceProviderError::from_response(response))
                }))
            }
        })
    }
}

impl ServiceRequest for DeleteContactRequest {
    type Output = DeleteContactResponse;
    type Error = DeleteContactError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "a4b", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.DeleteContact");
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteContactResponse, _>()
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
}

impl ServiceRequest for DeleteDeviceRequest {
    type Output = DeleteDeviceResponse;
    type Error = DeleteDeviceError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "a4b", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.DeleteDevice");
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteDeviceResponse, _>()
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
}

impl ServiceRequest for DeleteDeviceUsageDataRequest {
    type Output = DeleteDeviceUsageDataResponse;
    type Error = DeleteDeviceUsageDataError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "a4b", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.DeleteDeviceUsageData");
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteDeviceUsageDataResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DeleteDeviceUsageDataError::from_response(response))
                    }),
                )
            }
        })
    }
}

impl ServiceRequest for DeleteGatewayGroupRequest {
    type Output = DeleteGatewayGroupResponse;
    type Error = DeleteGatewayGroupError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "a4b", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.DeleteGatewayGroup");
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteGatewayGroupResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteGatewayGroupError::from_response(response))),
                )
            }
        })
    }
}

impl ServiceRequest for DeleteNetworkProfileRequest {
    type Output = DeleteNetworkProfileResponse;
    type Error = DeleteNetworkProfileError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "a4b", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.DeleteNetworkProfile");
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteNetworkProfileResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DeleteNetworkProfileError::from_response(response))
                    }),
                )
            }
        })
    }
}

impl ServiceRequest for DeleteProfileRequest {
    type Output = DeleteProfileResponse;
    type Error = DeleteProfileError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "a4b", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.DeleteProfile");
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteProfileResponse, _>()
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
}

impl ServiceRequest for DeleteRoomRequest {
    type Output = DeleteRoomResponse;
    type Error = DeleteRoomError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "a4b", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.DeleteRoom");
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteRoomResponse, _>()
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
}

impl ServiceRequest for DeleteRoomSkillParameterRequest {
    type Output = DeleteRoomSkillParameterResponse;
    type Error = DeleteRoomSkillParameterError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "a4b", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.DeleteRoomSkillParameter");
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteRoomSkillParameterResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteRoomSkillParameterError::from_response(response))
                }))
            }
        })
    }
}

impl ServiceRequest for DeleteSkillAuthorizationRequest {
    type Output = DeleteSkillAuthorizationResponse;
    type Error = DeleteSkillAuthorizationError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "a4b", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.DeleteSkillAuthorization");
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteSkillAuthorizationResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteSkillAuthorizationError::from_response(response))
                }))
            }
        })
    }
}

impl ServiceRequest for DeleteSkillGroupRequest {
    type Output = DeleteSkillGroupResponse;
    type Error = DeleteSkillGroupError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "a4b", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.DeleteSkillGroup");
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteSkillGroupResponse, _>()
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
}

impl ServiceRequest for DeleteUserRequest {
    type Output = DeleteUserResponse;
    type Error = DeleteUserError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "a4b", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.DeleteUser");
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteUserResponse, _>()
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
}

impl ServiceRequest for DisassociateContactFromAddressBookRequest {
    type Output = DisassociateContactFromAddressBookResponse;
    type Error = DisassociateContactFromAddressBookError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "a4b", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AlexaForBusiness.DisassociateContactFromAddressBook",
        );
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DisassociateContactFromAddressBookResponse, _>()
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
}

impl ServiceRequest for DisassociateDeviceFromRoomRequest {
    type Output = DisassociateDeviceFromRoomResponse;
    type Error = DisassociateDeviceFromRoomError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "a4b", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AlexaForBusiness.DisassociateDeviceFromRoom",
        );
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DisassociateDeviceFromRoomResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DisassociateDeviceFromRoomError::from_response(response))
                }))
            }
        })
    }
}

impl ServiceRequest for DisassociateSkillFromSkillGroupRequest {
    type Output = DisassociateSkillFromSkillGroupResponse;
    type Error = DisassociateSkillFromSkillGroupError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "a4b", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AlexaForBusiness.DisassociateSkillFromSkillGroup",
        );
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DisassociateSkillFromSkillGroupResponse, _>()
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
}

impl ServiceRequest for DisassociateSkillFromUsersRequest {
    type Output = DisassociateSkillFromUsersResponse;
    type Error = DisassociateSkillFromUsersError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "a4b", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AlexaForBusiness.DisassociateSkillFromUsers",
        );
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DisassociateSkillFromUsersResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DisassociateSkillFromUsersError::from_response(response))
                }))
            }
        })
    }
}

impl ServiceRequest for DisassociateSkillGroupFromRoomRequest {
    type Output = DisassociateSkillGroupFromRoomResponse;
    type Error = DisassociateSkillGroupFromRoomError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "a4b", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AlexaForBusiness.DisassociateSkillGroupFromRoom",
        );
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DisassociateSkillGroupFromRoomResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DisassociateSkillGroupFromRoomError::from_response(response))
                }))
            }
        })
    }
}

impl ServiceRequest for ForgetSmartHomeAppliancesRequest {
    type Output = ForgetSmartHomeAppliancesResponse;
    type Error = ForgetSmartHomeAppliancesError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "a4b", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.ForgetSmartHomeAppliances");
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ForgetSmartHomeAppliancesResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ForgetSmartHomeAppliancesError::from_response(response))
                }))
            }
        })
    }
}

impl ServiceRequest for GetAddressBookRequest {
    type Output = GetAddressBookResponse;
    type Error = GetAddressBookError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "a4b", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.GetAddressBook");
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetAddressBookResponse, _>()
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
}

impl ServiceRequest for GetConferencePreferenceRequest {
    type Output = GetConferencePreferenceResponse;
    type Error = GetConferencePreferenceError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "a4b", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.GetConferencePreference");
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetConferencePreferenceResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetConferencePreferenceError::from_response(response))
                }))
            }
        })
    }
}

impl ServiceRequest for GetConferenceProviderRequest {
    type Output = GetConferenceProviderResponse;
    type Error = GetConferenceProviderError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "a4b", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.GetConferenceProvider");
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetConferenceProviderResponse, _>()
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
}

impl ServiceRequest for GetContactRequest {
    type Output = GetContactResponse;
    type Error = GetContactError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "a4b", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.GetContact");
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetContactResponse, _>()
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
}

impl ServiceRequest for GetDeviceRequest {
    type Output = GetDeviceResponse;
    type Error = GetDeviceError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "a4b", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.GetDevice");
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetDeviceResponse, _>()
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
}

impl ServiceRequest for GetGatewayRequest {
    type Output = GetGatewayResponse;
    type Error = GetGatewayError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "a4b", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.GetGateway");
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetGatewayResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetGatewayError::from_response(response))),
                )
            }
        })
    }
}

impl ServiceRequest for GetGatewayGroupRequest {
    type Output = GetGatewayGroupResponse;
    type Error = GetGatewayGroupError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "a4b", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.GetGatewayGroup");
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetGatewayGroupResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetGatewayGroupError::from_response(response))),
                )
            }
        })
    }
}

impl ServiceRequest for GetInvitationConfigurationRequest {
    type Output = GetInvitationConfigurationResponse;
    type Error = GetInvitationConfigurationError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "a4b", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AlexaForBusiness.GetInvitationConfiguration",
        );
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetInvitationConfigurationResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetInvitationConfigurationError::from_response(response))
                }))
            }
        })
    }
}

impl ServiceRequest for GetNetworkProfileRequest {
    type Output = GetNetworkProfileResponse;
    type Error = GetNetworkProfileError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "a4b", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.GetNetworkProfile");
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetNetworkProfileResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetNetworkProfileError::from_response(response))),
                )
            }
        })
    }
}

impl ServiceRequest for GetProfileRequest {
    type Output = GetProfileResponse;
    type Error = GetProfileError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "a4b", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.GetProfile");
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetProfileResponse, _>()
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
}

impl ServiceRequest for GetRoomRequest {
    type Output = GetRoomResponse;
    type Error = GetRoomError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "a4b", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.GetRoom");
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response).deserialize::<GetRoomResponse, _>()
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
}

impl ServiceRequest for GetRoomSkillParameterRequest {
    type Output = GetRoomSkillParameterResponse;
    type Error = GetRoomSkillParameterError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "a4b", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.GetRoomSkillParameter");
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetRoomSkillParameterResponse, _>()
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
}

impl ServiceRequest for GetSkillGroupRequest {
    type Output = GetSkillGroupResponse;
    type Error = GetSkillGroupError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "a4b", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.GetSkillGroup");
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetSkillGroupResponse, _>()
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
}

impl ServiceRequest for ListBusinessReportSchedulesRequest {
    type Output = ListBusinessReportSchedulesResponse;
    type Error = ListBusinessReportSchedulesError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "a4b", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AlexaForBusiness.ListBusinessReportSchedules",
        );
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListBusinessReportSchedulesResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListBusinessReportSchedulesError::from_response(response))
                }))
            }
        })
    }
}

impl ServiceRequest for ListConferenceProvidersRequest {
    type Output = ListConferenceProvidersResponse;
    type Error = ListConferenceProvidersError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "a4b", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.ListConferenceProviders");
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListConferenceProvidersResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListConferenceProvidersError::from_response(response))
                }))
            }
        })
    }
}

impl ServiceRequest for ListDeviceEventsRequest {
    type Output = ListDeviceEventsResponse;
    type Error = ListDeviceEventsError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "a4b", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.ListDeviceEvents");
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListDeviceEventsResponse, _>()
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
}

impl ServiceRequest for ListGatewayGroupsRequest {
    type Output = ListGatewayGroupsResponse;
    type Error = ListGatewayGroupsError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "a4b", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.ListGatewayGroups");
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListGatewayGroupsResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListGatewayGroupsError::from_response(response))),
                )
            }
        })
    }
}

impl ServiceRequest for ListGatewaysRequest {
    type Output = ListGatewaysResponse;
    type Error = ListGatewaysError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "a4b", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.ListGateways");
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListGatewaysResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListGatewaysError::from_response(response))),
                )
            }
        })
    }
}

impl ServiceRequest for ListSkillsRequest {
    type Output = ListSkillsResponse;
    type Error = ListSkillsError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "a4b", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.ListSkills");
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListSkillsResponse, _>()
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
}

impl ServiceRequest for ListSkillsStoreCategoriesRequest {
    type Output = ListSkillsStoreCategoriesResponse;
    type Error = ListSkillsStoreCategoriesError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "a4b", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.ListSkillsStoreCategories");
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListSkillsStoreCategoriesResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListSkillsStoreCategoriesError::from_response(response))
                }))
            }
        })
    }
}

impl ServiceRequest for ListSkillsStoreSkillsByCategoryRequest {
    type Output = ListSkillsStoreSkillsByCategoryResponse;
    type Error = ListSkillsStoreSkillsByCategoryError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "a4b", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AlexaForBusiness.ListSkillsStoreSkillsByCategory",
        );
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListSkillsStoreSkillsByCategoryResponse, _>()
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
}

impl ServiceRequest for ListSmartHomeAppliancesRequest {
    type Output = ListSmartHomeAppliancesResponse;
    type Error = ListSmartHomeAppliancesError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "a4b", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.ListSmartHomeAppliances");
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListSmartHomeAppliancesResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListSmartHomeAppliancesError::from_response(response))
                }))
            }
        })
    }
}

impl ServiceRequest for ListTagsRequest {
    type Output = ListTagsResponse;
    type Error = ListTagsError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "a4b", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.ListTags");
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListTagsResponse, _>()
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
}

impl ServiceRequest for PutConferencePreferenceRequest {
    type Output = PutConferencePreferenceResponse;
    type Error = PutConferencePreferenceError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "a4b", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.PutConferencePreference");
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<PutConferencePreferenceResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(PutConferencePreferenceError::from_response(response))
                }))
            }
        })
    }
}

impl ServiceRequest for PutInvitationConfigurationRequest {
    type Output = PutInvitationConfigurationResponse;
    type Error = PutInvitationConfigurationError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "a4b", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AlexaForBusiness.PutInvitationConfiguration",
        );
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<PutInvitationConfigurationResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(PutInvitationConfigurationError::from_response(response))
                }))
            }
        })
    }
}

impl ServiceRequest for PutRoomSkillParameterRequest {
    type Output = PutRoomSkillParameterResponse;
    type Error = PutRoomSkillParameterError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "a4b", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.PutRoomSkillParameter");
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<PutRoomSkillParameterResponse, _>()
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
}

impl ServiceRequest for PutSkillAuthorizationRequest {
    type Output = PutSkillAuthorizationResponse;
    type Error = PutSkillAuthorizationError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "a4b", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.PutSkillAuthorization");
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<PutSkillAuthorizationResponse, _>()
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
}

impl ServiceRequest for RegisterAVSDeviceRequest {
    type Output = RegisterAVSDeviceResponse;
    type Error = RegisterAVSDeviceError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "a4b", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.RegisterAVSDevice");
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<RegisterAVSDeviceResponse, _>()
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
}

impl ServiceRequest for RejectSkillRequest {
    type Output = RejectSkillResponse;
    type Error = RejectSkillError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "a4b", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.RejectSkill");
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<RejectSkillResponse, _>()
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
}

impl ServiceRequest for ResolveRoomRequest {
    type Output = ResolveRoomResponse;
    type Error = ResolveRoomError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "a4b", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.ResolveRoom");
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ResolveRoomResponse, _>()
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
}

impl ServiceRequest for RevokeInvitationRequest {
    type Output = RevokeInvitationResponse;
    type Error = RevokeInvitationError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "a4b", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.RevokeInvitation");
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<RevokeInvitationResponse, _>()
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
}

impl ServiceRequest for SearchAddressBooksRequest {
    type Output = SearchAddressBooksResponse;
    type Error = SearchAddressBooksError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "a4b", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.SearchAddressBooks");
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<SearchAddressBooksResponse, _>()
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
}

impl ServiceRequest for SearchContactsRequest {
    type Output = SearchContactsResponse;
    type Error = SearchContactsError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "a4b", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.SearchContacts");
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<SearchContactsResponse, _>()
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
}

impl ServiceRequest for SearchDevicesRequest {
    type Output = SearchDevicesResponse;
    type Error = SearchDevicesError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "a4b", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.SearchDevices");
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<SearchDevicesResponse, _>()
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
}

impl ServiceRequest for SearchNetworkProfilesRequest {
    type Output = SearchNetworkProfilesResponse;
    type Error = SearchNetworkProfilesError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "a4b", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.SearchNetworkProfiles");
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<SearchNetworkProfilesResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(SearchNetworkProfilesError::from_response(response))
                    }),
                )
            }
        })
    }
}

impl ServiceRequest for SearchProfilesRequest {
    type Output = SearchProfilesResponse;
    type Error = SearchProfilesError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "a4b", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.SearchProfiles");
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<SearchProfilesResponse, _>()
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
}

impl ServiceRequest for SearchRoomsRequest {
    type Output = SearchRoomsResponse;
    type Error = SearchRoomsError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "a4b", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.SearchRooms");
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<SearchRoomsResponse, _>()
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
}

impl ServiceRequest for SearchSkillGroupsRequest {
    type Output = SearchSkillGroupsResponse;
    type Error = SearchSkillGroupsError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "a4b", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.SearchSkillGroups");
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<SearchSkillGroupsResponse, _>()
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
}

impl ServiceRequest for SearchUsersRequest {
    type Output = SearchUsersResponse;
    type Error = SearchUsersError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "a4b", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.SearchUsers");
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<SearchUsersResponse, _>()
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
}

impl ServiceRequest for SendAnnouncementRequest {
    type Output = SendAnnouncementResponse;
    type Error = SendAnnouncementError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "a4b", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.SendAnnouncement");
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<SendAnnouncementResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(SendAnnouncementError::from_response(response))),
                )
            }
        })
    }
}

impl ServiceRequest for SendInvitationRequest {
    type Output = SendInvitationResponse;
    type Error = SendInvitationError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "a4b", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.SendInvitation");
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<SendInvitationResponse, _>()
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
}

impl ServiceRequest for StartDeviceSyncRequest {
    type Output = StartDeviceSyncResponse;
    type Error = StartDeviceSyncError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "a4b", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.StartDeviceSync");
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<StartDeviceSyncResponse, _>()
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
}

impl ServiceRequest for StartSmartHomeApplianceDiscoveryRequest {
    type Output = StartSmartHomeApplianceDiscoveryResponse;
    type Error = StartSmartHomeApplianceDiscoveryError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "a4b", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AlexaForBusiness.StartSmartHomeApplianceDiscovery",
        );
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<StartSmartHomeApplianceDiscoveryResponse, _>()
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
}

impl ServiceRequest for TagResourceRequest {
    type Output = TagResourceResponse;
    type Error = TagResourceError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "a4b", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.TagResource");
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<TagResourceResponse, _>()
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
}

impl ServiceRequest for UntagResourceRequest {
    type Output = UntagResourceResponse;
    type Error = UntagResourceError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "a4b", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.UntagResource");
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<UntagResourceResponse, _>()
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
}

impl ServiceRequest for UpdateAddressBookRequest {
    type Output = UpdateAddressBookResponse;
    type Error = UpdateAddressBookError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "a4b", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.UpdateAddressBook");
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateAddressBookResponse, _>()
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
}

impl ServiceRequest for UpdateBusinessReportScheduleRequest {
    type Output = UpdateBusinessReportScheduleResponse;
    type Error = UpdateBusinessReportScheduleError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "a4b", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AlexaForBusiness.UpdateBusinessReportSchedule",
        );
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateBusinessReportScheduleResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateBusinessReportScheduleError::from_response(response))
                }))
            }
        })
    }
}

impl ServiceRequest for UpdateConferenceProviderRequest {
    type Output = UpdateConferenceProviderResponse;
    type Error = UpdateConferenceProviderError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "a4b", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.UpdateConferenceProvider");
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateConferenceProviderResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateConferenceProviderError::from_response(response))
                }))
            }
        })
    }
}

impl ServiceRequest for UpdateContactRequest {
    type Output = UpdateContactResponse;
    type Error = UpdateContactError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "a4b", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.UpdateContact");
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateContactResponse, _>()
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
}

impl ServiceRequest for UpdateDeviceRequest {
    type Output = UpdateDeviceResponse;
    type Error = UpdateDeviceError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "a4b", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.UpdateDevice");
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateDeviceResponse, _>()
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
}

impl ServiceRequest for UpdateGatewayRequest {
    type Output = UpdateGatewayResponse;
    type Error = UpdateGatewayError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "a4b", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.UpdateGateway");
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateGatewayResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateGatewayError::from_response(response))),
                )
            }
        })
    }
}

impl ServiceRequest for UpdateGatewayGroupRequest {
    type Output = UpdateGatewayGroupResponse;
    type Error = UpdateGatewayGroupError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "a4b", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.UpdateGatewayGroup");
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateGatewayGroupResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateGatewayGroupError::from_response(response))),
                )
            }
        })
    }
}

impl ServiceRequest for UpdateNetworkProfileRequest {
    type Output = UpdateNetworkProfileResponse;
    type Error = UpdateNetworkProfileError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "a4b", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.UpdateNetworkProfile");
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateNetworkProfileResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(UpdateNetworkProfileError::from_response(response))
                    }),
                )
            }
        })
    }
}

impl ServiceRequest for UpdateProfileRequest {
    type Output = UpdateProfileResponse;
    type Error = UpdateProfileError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "a4b", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.UpdateProfile");
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateProfileResponse, _>()
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
}

impl ServiceRequest for UpdateRoomRequest {
    type Output = UpdateRoomResponse;
    type Error = UpdateRoomError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "a4b", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.UpdateRoom");
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateRoomResponse, _>()
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
}

impl ServiceRequest for UpdateSkillGroupRequest {
    type Output = UpdateSkillGroupResponse;
    type Error = UpdateSkillGroupError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "a4b", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.UpdateSkillGroup");
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateSkillGroupResponse, _>()
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
