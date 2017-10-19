
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

#[allow(warnings)]
use hyper::Client;
use hyper::status::StatusCode;
use rusoto_core::request::DispatchSignedRequest;
use rusoto_core::region;

use std::fmt;
use std::error::Error;
use std::io;
use std::io::Read;
use rusoto_core::request::HttpDispatchError;
use rusoto_core::credential::{CredentialsError, ProvideAwsCredentials};

use serde_json;
use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::signature::SignedRequest;
use serde_json::from_str;
use serde_json::Value as SerdeJsonValue;
#[doc="Apple Push Notification Service channel definition."]
#[derive(Default,Debug,Clone,Serialize)]
pub struct APNSChannelRequest {
    #[doc="The distribution certificate from Apple."]
    #[serde(rename="Certificate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub certificate: Option<String>,
    #[doc="If the channel is enabled for sending messages."]
    #[serde(rename="Enabled")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub enabled: Option<bool>,
    #[doc="The certificate private key."]
    #[serde(rename="PrivateKey")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub private_key: Option<String>,
}

#[doc="Apple Distribution Push Notification Service channel definition."]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct APNSChannelResponse {
    #[doc="The ID of the application to which the channel applies."]
    #[serde(rename="ApplicationId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub application_id: Option<String>,
    #[doc="When was this segment created"]
    #[serde(rename="CreationDate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub creation_date: Option<String>,
    #[doc="If the channel is enabled for sending messages."]
    #[serde(rename="Enabled")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub enabled: Option<bool>,
    #[doc="Channel ID. Not used. Present only for backwards compatibility."]
    #[serde(rename="Id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<String>,
    #[doc="Is this channel archived"]
    #[serde(rename="IsArchived")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub is_archived: Option<bool>,
    #[doc="Who last updated this entry"]
    #[serde(rename="LastModifiedBy")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_modified_by: Option<String>,
    #[doc="Last date this was updated"]
    #[serde(rename="LastModifiedDate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_modified_date: Option<String>,
    #[doc="The platform type. Will be APNS."]
    #[serde(rename="Platform")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub platform: Option<String>,
    #[doc="Version of channel"]
    #[serde(rename="Version")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub version: Option<i64>,
}

#[doc="APNS Message."]
#[derive(Default,Debug,Clone,Serialize)]
pub struct APNSMessage {
    #[doc="The action that occurs if the user taps a push notification delivered by the campaign: OPEN_APP - Your app launches, or it becomes the foreground app if it has been sent to the background. This is the default action. DEEP_LINK - Uses deep linking features in iOS and Android to open your app and display a designated user interface within the app. URL - The default mobile browser on the user's device launches and opens a web page at the URL you specify. Possible values include: OPEN_APP | DEEP_LINK | URL"]
    #[serde(rename="Action")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub action: Option<String>,
    #[doc="Include this key when you want the system to modify the badge of your app icon. If this key is not included in the dictionary, the badge is not changed. To remove the badge, set the value of this key to 0."]
    #[serde(rename="Badge")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub badge: Option<i64>,
    #[doc="The message body of the notification, the email body or the text message."]
    #[serde(rename="Body")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub body: Option<String>,
    #[doc="Provide this key with a string value that represents the notification's type. This value corresponds to the value in the identifier property of one of your app's registered categories."]
    #[serde(rename="Category")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub category: Option<String>,
    #[doc="The data payload used for a silent push. This payload is added to the notifications' data.pinpoint.jsonBody' object"]
    #[serde(rename="Data")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub data: Option<::std::collections::HashMap<String, String>>,
    #[doc="The URL that points to a video used in the push notification."]
    #[serde(rename="MediaUrl")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub media_url: Option<String>,
    #[doc="The Raw JSON formatted string to be used as the payload. This value overrides the message."]
    #[serde(rename="RawContent")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub raw_content: Option<String>,
    #[doc="Indicates if the message should display on the users device. Silent pushes can be used for Remote Configuration and Phone Home use cases."]
    #[serde(rename="SilentPush")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub silent_push: Option<bool>,
    #[doc="Include this key when you want the system to play a sound. The value of this key is the name of a sound file in your app's main bundle or in the Library/Sounds folder of your app's data container. If the sound file cannot be found, or if you specify defaultfor the value, the system plays the default alert sound."]
    #[serde(rename="Sound")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sound: Option<String>,
    #[doc="Default message substitutions. Can be overridden by individual address substitutions."]
    #[serde(rename="Substitutions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub substitutions: Option<::std::collections::HashMap<String, Vec<String>>>,
    #[doc="Provide this key with a string value that represents the app-specific identifier for grouping notifications. If you provide a Notification Content app extension, you can use this value to group your notifications together."]
    #[serde(rename="ThreadId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub thread_id: Option<String>,
    #[doc="The message title that displays above the message on the user's device."]
    #[serde(rename="Title")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub title: Option<String>,
    #[doc="The URL to open in the user's mobile browser. Used if the value for Action is URL."]
    #[serde(rename="Url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,
}

#[doc="Apple Development Push Notification Service channel definition."]
#[derive(Default,Debug,Clone,Serialize)]
pub struct APNSSandboxChannelRequest {
    #[doc="The distribution certificate from Apple."]
    #[serde(rename="Certificate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub certificate: Option<String>,
    #[doc="If the channel is enabled for sending messages."]
    #[serde(rename="Enabled")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub enabled: Option<bool>,
    #[doc="The certificate private key."]
    #[serde(rename="PrivateKey")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub private_key: Option<String>,
}

#[doc="Apple Development Push Notification Service channel definition."]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct APNSSandboxChannelResponse {
    #[doc="Application id"]
    #[serde(rename="ApplicationId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub application_id: Option<String>,
    #[doc="When was this segment created"]
    #[serde(rename="CreationDate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub creation_date: Option<String>,
    #[doc="If the channel is enabled for sending messages."]
    #[serde(rename="Enabled")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub enabled: Option<bool>,
    #[doc="Channel ID. Not used, only for backwards compatibility."]
    #[serde(rename="Id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<String>,
    #[doc="Is this channel archived"]
    #[serde(rename="IsArchived")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub is_archived: Option<bool>,
    #[doc="Who last updated this entry"]
    #[serde(rename="LastModifiedBy")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_modified_by: Option<String>,
    #[doc="Last date this was updated"]
    #[serde(rename="LastModifiedDate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_modified_date: Option<String>,
    #[doc="The platform type. Will be APNS_SANDBOX."]
    #[serde(rename="Platform")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub platform: Option<String>,
    #[doc="Version of channel"]
    #[serde(rename="Version")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub version: Option<i64>,
}

#[doc="Activities for campaign."]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ActivitiesResponse {
    #[doc="List of campaign activities"]
    #[serde(rename="Item")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub item: Option<Vec<ActivityResponse>>,
}

#[doc="Activity definition"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ActivityResponse {
    #[doc="The ID of the application to which the campaign applies."]
    #[serde(rename="ApplicationId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub application_id: Option<String>,
    #[doc="The ID of the campaign to which the activity applies."]
    #[serde(rename="CampaignId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub campaign_id: Option<String>,
    #[doc="The actual time the activity was marked CANCELLED or COMPLETED. Provided in ISO 8601 format."]
    #[serde(rename="End")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub end: Option<String>,
    #[doc="The unique activity ID."]
    #[serde(rename="Id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<String>,
    #[doc="Indicates whether the activity succeeded.

Valid values: SUCCESS, FAIL"]
    #[serde(rename="Result")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub result: Option<String>,
    #[doc="The scheduled start time for the activity in ISO 8601 format."]
    #[serde(rename="ScheduledStart")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub scheduled_start: Option<String>,
    #[doc="The actual start time of the activity in ISO 8601 format."]
    #[serde(rename="Start")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub start: Option<String>,
    #[doc="The state of the activity.

Valid values: PENDING, INITIALIZING, RUNNING, PAUSED, CANCELLED, COMPLETED"]
    #[serde(rename="State")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub state: Option<String>,
    #[doc="The total number of endpoints to which the campaign successfully delivered messages."]
    #[serde(rename="SuccessfulEndpointCount")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub successful_endpoint_count: Option<i64>,
    #[doc="The total number of timezones completed."]
    #[serde(rename="TimezonesCompletedCount")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub timezones_completed_count: Option<i64>,
    #[doc="The total number of unique timezones present in the segment."]
    #[serde(rename="TimezonesTotalCount")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub timezones_total_count: Option<i64>,
    #[doc="The total number of endpoints to which the campaign attempts to deliver messages."]
    #[serde(rename="TotalEndpointCount")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub total_endpoint_count: Option<i64>,
    #[doc="The ID of a variation of the campaign used for A/B testing."]
    #[serde(rename="TreatmentId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub treatment_id: Option<String>,
}

#[doc="Address configuration."]
#[derive(Default,Debug,Clone,Serialize)]
pub struct AddressConfiguration {
    #[doc="Body override. If specified will override default body."]
    #[serde(rename="BodyOverride")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub body_override: Option<String>,
    #[doc="The channel type.

Valid values: GCM | APNS | SMS | EMAIL"]
    #[serde(rename="ChannelType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub channel_type: Option<String>,
    #[doc="A map of custom attributes to attributes to be attached to the message for this address. This payload is added to the push notification's 'data.pinpoint' object or added to the email/sms delivery receipt event attributes."]
    #[serde(rename="Context")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub context: Option<::std::collections::HashMap<String, String>>,
    #[doc="The Raw JSON formatted string to be used as the payload. This value overrides the message."]
    #[serde(rename="RawContent")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub raw_content: Option<String>,
    #[doc="A map of substitution values for the message to be merged with the DefaultMessage's substitutions. Substitutions on this map take precedence over the all other substitutions."]
    #[serde(rename="Substitutions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub substitutions: Option<::std::collections::HashMap<String, Vec<String>>>,
    #[doc="Title override. If specified will override default title if applicable."]
    #[serde(rename="TitleOverride")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub title_override: Option<String>,
}

#[doc="Application Response."]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ApplicationResponse {
    #[doc="The unique application ID."]
    #[serde(rename="Id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<String>,
    #[doc="The display name of the application."]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
}

#[doc="Application settings."]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ApplicationSettingsResource {
    #[doc="The unique ID for the application."]
    #[serde(rename="ApplicationId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub application_id: Option<String>,
    #[doc="The date that the settings were last updated in ISO 8601 format."]
    #[serde(rename="LastModifiedDate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_modified_date: Option<String>,
    #[doc="The default campaign limits for the app. These limits apply to each campaign for the app, unless the campaign overrides the default with limits of its own."]
    #[serde(rename="Limits")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub limits: Option<CampaignLimits>,
    #[doc="The default quiet time for the app. Each campaign for this app sends no messages during this time unless the campaign overrides the default with a quiet time of its own."]
    #[serde(rename="QuietTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub quiet_time: Option<QuietTime>,
}

#[doc="Get Applications Result."]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ApplicationsResponse {
    #[doc="List of applications returned in this page."]
    #[serde(rename="Item")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub item: Option<Vec<ApplicationResponse>>,
    #[doc="The string that you use in a subsequent request to get the next page of results in a paginated response."]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
}

#[doc="Custom attibute dimension"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct AttributeDimension {
    #[doc="The type of dimension:
INCLUSIVE - Endpoints that match the criteria are included in the segment.
EXCLUSIVE - Endpoints that match the criteria are excluded from the segment."]
    #[serde(rename="AttributeType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub attribute_type: Option<String>,
    #[doc="The criteria values for the segment dimension. Endpoints with matching attribute values are included or excluded from the segment, depending on the setting for Type."]
    #[serde(rename="Values")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[doc="The email message configuration."]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct CampaignEmailMessage {
    #[doc="The email text body."]
    #[serde(rename="Body")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub body: Option<String>,
    #[doc="The email address used to send the email from. Defaults to use FromAddress specified in the Email Channel."]
    #[serde(rename="FromAddress")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub from_address: Option<String>,
    #[doc="The email html body."]
    #[serde(rename="HtmlBody")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_body: Option<String>,
    #[doc="The email title (Or subject)."]
    #[serde(rename="Title")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub title: Option<String>,
}

#[doc="Campaign Limits are used to limit the number of messages that can be sent to a user."]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct CampaignLimits {
    #[doc="The maximum number of messages that the campaign can send daily."]
    #[serde(rename="Daily")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub daily: Option<i64>,
    #[doc="The maximum total number of messages that the campaign can send."]
    #[serde(rename="Total")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub total: Option<i64>,
}

#[doc="Campaign definition"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct CampaignResponse {
    #[doc="Treatments that are defined in addition to the default treatment."]
    #[serde(rename="AdditionalTreatments")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub additional_treatments: Option<Vec<TreatmentResource>>,
    #[doc="The ID of the application to which the campaign applies."]
    #[serde(rename="ApplicationId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub application_id: Option<String>,
    #[doc="The date the campaign was created in ISO 8601 format."]
    #[serde(rename="CreationDate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub creation_date: Option<String>,
    #[doc="The status of the campaign's default treatment. Only present for A/B test campaigns."]
    #[serde(rename="DefaultState")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub default_state: Option<CampaignState>,
    #[doc="A description of the campaign."]
    #[serde(rename="Description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,
    #[doc="The allocated percentage of end users who will not receive messages from this campaign."]
    #[serde(rename="HoldoutPercent")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub holdout_percent: Option<i64>,
    #[doc="The unique campaign ID."]
    #[serde(rename="Id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<String>,
    #[doc="Indicates whether the campaign is paused. A paused campaign does not send messages unless you resume it by setting IsPaused to false."]
    #[serde(rename="IsPaused")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub is_paused: Option<bool>,
    #[doc="The date the campaign was last updated in ISO 8601 format.	"]
    #[serde(rename="LastModifiedDate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_modified_date: Option<String>,
    #[doc="The campaign limits settings."]
    #[serde(rename="Limits")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub limits: Option<CampaignLimits>,
    #[doc="The message configuration settings."]
    #[serde(rename="MessageConfiguration")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub message_configuration: Option<MessageConfiguration>,
    #[doc="The custom name of the campaign."]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    #[doc="The campaign schedule."]
    #[serde(rename="Schedule")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub schedule: Option<Schedule>,
    #[doc="The ID of the segment to which the campaign sends messages."]
    #[serde(rename="SegmentId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub segment_id: Option<String>,
    #[doc="The version of the segment to which the campaign sends messages."]
    #[serde(rename="SegmentVersion")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub segment_version: Option<i64>,
    #[doc="The campaign status.

An A/B test campaign will have a status of COMPLETED only when all treatments have a status of COMPLETED."]
    #[serde(rename="State")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub state: Option<CampaignState>,
    #[doc="A custom description for the treatment."]
    #[serde(rename="TreatmentDescription")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub treatment_description: Option<String>,
    #[doc="The custom name of a variation of the campaign used for A/B testing."]
    #[serde(rename="TreatmentName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub treatment_name: Option<String>,
    #[doc="The campaign version number."]
    #[serde(rename="Version")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub version: Option<i64>,
}

#[doc="SMS message configuration."]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct CampaignSmsMessage {
    #[doc="The SMS text body."]
    #[serde(rename="Body")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub body: Option<String>,
    #[doc="Is this is a transactional SMS message, otherwise a promotional message."]
    #[serde(rename="MessageType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub message_type: Option<String>,
    #[doc="Sender ID of sent message."]
    #[serde(rename="SenderId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sender_id: Option<String>,
}

#[doc="State of the Campaign"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct CampaignState {
    #[doc="The status of the campaign, or the status of a treatment that belongs to an A/B test campaign.

Valid values: SCHEDULED, EXECUTING, PENDING_NEXT_RUN, COMPLETED, PAUSED"]
    #[serde(rename="CampaignStatus")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub campaign_status: Option<String>,
}

#[doc="List of available campaigns."]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct CampaignsResponse {
    #[doc="A list of campaigns."]
    #[serde(rename="Item")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub item: Option<Vec<CampaignResponse>>,
    #[doc="The string that you use in a subsequent request to get the next page of results in a paginated response."]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct CreateAppRequest {
    #[serde(rename="CreateApplicationRequest")]
    pub create_application_request: CreateApplicationRequest,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct CreateAppResponse {
    #[serde(rename="ApplicationResponse")]
    pub application_response: ApplicationResponse,
}

#[doc="Application Request."]
#[derive(Default,Debug,Clone,Serialize)]
pub struct CreateApplicationRequest {
    #[doc="The display name of the application. Used in the Amazon Pinpoint console."]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct CreateCampaignRequest {
    #[serde(rename="ApplicationId")]
    pub application_id: String,
    #[serde(rename="WriteCampaignRequest")]
    pub write_campaign_request: WriteCampaignRequest,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct CreateCampaignResponse {
    #[serde(rename="CampaignResponse")]
    pub campaign_response: CampaignResponse,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct CreateImportJobRequest {
    #[serde(rename="ApplicationId")]
    pub application_id: String,
    #[serde(rename="ImportJobRequest")]
    pub import_job_request: ImportJobRequest,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct CreateImportJobResponse {
    #[serde(rename="ImportJobResponse")]
    pub import_job_response: ImportJobResponse,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct CreateSegmentRequest {
    #[serde(rename="ApplicationId")]
    pub application_id: String,
    #[serde(rename="WriteSegmentRequest")]
    pub write_segment_request: WriteSegmentRequest,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct CreateSegmentResponse {
    #[serde(rename="SegmentResponse")]
    pub segment_response: SegmentResponse,
}

#[doc="Default Message across push notification, email, and sms."]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DefaultMessage {
    #[doc="The message body of the notification, the email body or the text message."]
    #[serde(rename="Body")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub body: Option<String>,
    #[doc="Default message substitutions. Can be overridden by individual address substitutions."]
    #[serde(rename="Substitutions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub substitutions: Option<::std::collections::HashMap<String, Vec<String>>>,
}

#[doc="Default Push Notification Message."]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DefaultPushNotificationMessage {
    #[doc="The action that occurs if the user taps a push notification delivered by the campaign: OPEN_APP - Your app launches, or it becomes the foreground app if it has been sent to the background. This is the default action. DEEP_LINK - Uses deep linking features in iOS and Android to open your app and display a designated user interface within the app. URL - The default mobile browser on the user's device launches and opens a web page at the URL you specify. Possible values include: OPEN_APP | DEEP_LINK | URL"]
    #[serde(rename="Action")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub action: Option<String>,
    #[doc="The message body of the notification, the email body or the text message."]
    #[serde(rename="Body")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub body: Option<String>,
    #[doc="The data payload used for a silent push. This payload is added to the notifications' data.pinpoint.jsonBody' object"]
    #[serde(rename="Data")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub data: Option<::std::collections::HashMap<String, String>>,
    #[doc="Indicates if the message should display on the users device. Silent pushes can be used for Remote Configuration and Phone Home use cases."]
    #[serde(rename="SilentPush")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub silent_push: Option<bool>,
    #[doc="Default message substitutions. Can be overridden by individual address substitutions."]
    #[serde(rename="Substitutions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub substitutions: Option<::std::collections::HashMap<String, Vec<String>>>,
    #[doc="The message title that displays above the message on the user's device."]
    #[serde(rename="Title")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub title: Option<String>,
    #[doc="The URL to open in the user's mobile browser. Used if the value for Action is URL."]
    #[serde(rename="Url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteApnsChannelRequest {
    #[serde(rename="ApplicationId")]
    pub application_id: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DeleteApnsChannelResponse {
    #[serde(rename="APNSChannelResponse")]
    pub apns_channel_response: APNSChannelResponse,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteApnsSandboxChannelRequest {
    #[serde(rename="ApplicationId")]
    pub application_id: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DeleteApnsSandboxChannelResponse {
    #[serde(rename="APNSSandboxChannelResponse")]
    pub apns_sandbox_channel_response: APNSSandboxChannelResponse,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteAppRequest {
    #[serde(rename="ApplicationId")]
    pub application_id: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DeleteAppResponse {
    #[serde(rename="ApplicationResponse")]
    pub application_response: ApplicationResponse,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteCampaignRequest {
    #[serde(rename="ApplicationId")]
    pub application_id: String,
    #[serde(rename="CampaignId")]
    pub campaign_id: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DeleteCampaignResponse {
    #[serde(rename="CampaignResponse")]
    pub campaign_response: CampaignResponse,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteEmailChannelRequest {
    #[serde(rename="ApplicationId")]
    pub application_id: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DeleteEmailChannelResponse {
    #[serde(rename="EmailChannelResponse")]
    pub email_channel_response: EmailChannelResponse,
}

#[doc="DeleteEventStream Request"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteEventStreamRequest {
    #[doc="ApplicationId"]
    #[serde(rename="ApplicationId")]
    pub application_id: String,
}

#[doc="DeleteEventStream Response"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DeleteEventStreamResponse {
    #[serde(rename="EventStream")]
    pub event_stream: EventStream,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteGcmChannelRequest {
    #[serde(rename="ApplicationId")]
    pub application_id: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DeleteGcmChannelResponse {
    #[serde(rename="GCMChannelResponse")]
    pub gcm_channel_response: GCMChannelResponse,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteSegmentRequest {
    #[serde(rename="ApplicationId")]
    pub application_id: String,
    #[serde(rename="SegmentId")]
    pub segment_id: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DeleteSegmentResponse {
    #[serde(rename="SegmentResponse")]
    pub segment_response: SegmentResponse,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteSmsChannelRequest {
    #[serde(rename="ApplicationId")]
    pub application_id: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DeleteSmsChannelResponse {
    #[serde(rename="SMSChannelResponse")]
    pub sms_channel_response: SMSChannelResponse,
}

#[doc="The message configuration."]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DirectMessageConfiguration {
    #[doc="The message to APNS channels. Overrides the default push notification message."]
    #[serde(rename="APNSMessage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub apns_message: Option<APNSMessage>,
    #[doc="The default message for all channels."]
    #[serde(rename="DefaultMessage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub default_message: Option<DefaultMessage>,
    #[doc="The default push notification message for all push channels."]
    #[serde(rename="DefaultPushNotificationMessage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub default_push_notification_message: Option<DefaultPushNotificationMessage>,
    #[doc="The message to GCM channels. Overrides the default push notification message."]
    #[serde(rename="GCMMessage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub gcm_message: Option<GCMMessage>,
    #[doc="The message to SMS channels. Overrides the default message."]
    #[serde(rename="SMSMessage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sms_message: Option<SMSMessage>,
}

#[doc="Email Channel Request"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct EmailChannelRequest {
    #[doc="If the channel is enabled for sending messages."]
    #[serde(rename="Enabled")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub enabled: Option<bool>,
    #[doc="The email address used to send emails from."]
    #[serde(rename="FromAddress")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub from_address: Option<String>,
    #[doc="The ARN of an identity verified with SES."]
    #[serde(rename="Identity")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub identity: Option<String>,
    #[doc="The ARN of an IAM Role used to submit events to Mobile Analytics' event ingestion service"]
    #[serde(rename="RoleArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub role_arn: Option<String>,
}

#[doc="Email Channel Response."]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct EmailChannelResponse {
    #[doc="The unique ID of the application to which the email channel belongs."]
    #[serde(rename="ApplicationId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub application_id: Option<String>,
    #[doc="The date that the settings were last updated in ISO 8601 format."]
    #[serde(rename="CreationDate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub creation_date: Option<String>,
    #[doc="If the channel is enabled for sending messages."]
    #[serde(rename="Enabled")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub enabled: Option<bool>,
    #[doc="The email address used to send emails from."]
    #[serde(rename="FromAddress")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub from_address: Option<String>,
    #[doc="Channel ID. Not used, only for backwards compatibility."]
    #[serde(rename="Id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<String>,
    #[doc="The ARN of an identity verified with SES."]
    #[serde(rename="Identity")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub identity: Option<String>,
    #[doc="Is this channel archived"]
    #[serde(rename="IsArchived")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub is_archived: Option<bool>,
    #[doc="Who last updated this entry"]
    #[serde(rename="LastModifiedBy")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_modified_by: Option<String>,
    #[doc="Last date this was updated"]
    #[serde(rename="LastModifiedDate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_modified_date: Option<String>,
    #[doc="Platform type. Will be \"EMAIL\""]
    #[serde(rename="Platform")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub platform: Option<String>,
    #[doc="The ARN of an IAM Role used to submit events to Mobile Analytics' event ingestion service"]
    #[serde(rename="RoleArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub role_arn: Option<String>,
    #[doc="Version of channel"]
    #[serde(rename="Version")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub version: Option<i64>,
}

#[doc="Endpoint update request"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct EndpointBatchItem {
    #[doc="The address or token of the endpoint as provided by your push provider (e.g. DeviceToken or RegistrationId)."]
    #[serde(rename="Address")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub address: Option<String>,
    #[doc="Custom attributes that your app reports to Amazon Pinpoint. You can use these attributes as selection criteria when you create a segment."]
    #[serde(rename="Attributes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub attributes: Option<::std::collections::HashMap<String, Vec<String>>>,
    #[doc="The channel type.

Valid values: GCM | APNS | SMS | EMAIL"]
    #[serde(rename="ChannelType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub channel_type: Option<String>,
    #[doc="The endpoint demographic attributes."]
    #[serde(rename="Demographic")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub demographic: Option<EndpointDemographic>,
    #[doc="The last time the endpoint was updated. Provided in ISO 8601 format."]
    #[serde(rename="EffectiveDate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub effective_date: Option<String>,
    #[doc="The endpoint status. Can be either ACTIVE or INACTIVE. Will be set to INACTIVE if a delivery fails. Will be set to ACTIVE if the address is updated."]
    #[serde(rename="EndpointStatus")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub endpoint_status: Option<String>,
    #[doc="The unique Id for the Endpoint in the batch."]
    #[serde(rename="Id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<String>,
    #[doc="The endpoint location attributes."]
    #[serde(rename="Location")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub location: Option<EndpointLocation>,
    #[doc="Custom metrics that your app reports to Amazon Pinpoint."]
    #[serde(rename="Metrics")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub metrics: Option<::std::collections::HashMap<String, f64>>,
    #[doc="Indicates whether a user has opted out of receiving messages with one of the following values:

ALL - User has opted out of all messages.

NONE - Users has not opted out and receives all messages."]
    #[serde(rename="OptOut")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub opt_out: Option<String>,
    #[doc="The unique ID for the most recent request to update the endpoint."]
    #[serde(rename="RequestId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub request_id: Option<String>,
    #[doc="Custom user-specific attributes that your app reports to Amazon Pinpoint."]
    #[serde(rename="User")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user: Option<EndpointUser>,
}

#[doc="Endpoint batch update request."]
#[derive(Default,Debug,Clone,Serialize)]
pub struct EndpointBatchRequest {
    #[doc="List of items to update. Maximum 100 items"]
    #[serde(rename="Item")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub item: Option<Vec<EndpointBatchItem>>,
}

#[doc="Endpoint demographic data"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct EndpointDemographic {
    #[doc="The version of the application associated with the endpoint."]
    #[serde(rename="AppVersion")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub app_version: Option<String>,
    #[doc="The endpoint locale in the following format: The ISO 639-1 alpha-2 code, followed by an underscore, followed by an ISO 3166-1 alpha-2 value.
"]
    #[serde(rename="Locale")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub locale: Option<String>,
    #[doc="The endpoint make, such as such as Apple or Samsung."]
    #[serde(rename="Make")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub make: Option<String>,
    #[doc="The endpoint model, such as iPhone."]
    #[serde(rename="Model")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub model: Option<String>,
    #[doc="The endpoint model version."]
    #[serde(rename="ModelVersion")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub model_version: Option<String>,
    #[doc="The endpoint platform, such as ios or android."]
    #[serde(rename="Platform")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub platform: Option<String>,
    #[doc="The endpoint platform version."]
    #[serde(rename="PlatformVersion")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub platform_version: Option<String>,
    #[doc="The timezone of the endpoint. Specified as a tz database value, such as Americas/Los_Angeles."]
    #[serde(rename="Timezone")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub timezone: Option<String>,
}

#[doc="Endpoint location data"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct EndpointLocation {
    #[doc="The city where the endpoint is located."]
    #[serde(rename="City")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub city: Option<String>,
    #[doc="Country according to ISO 3166-1 Alpha-2 codes. For example, US."]
    #[serde(rename="Country")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub country: Option<String>,
    #[doc="The latitude of the endpoint location. Rounded to one decimal (Roughly corresponding to a mile)."]
    #[serde(rename="Latitude")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub latitude: Option<f64>,
    #[doc="The longitude of the endpoint location. Rounded to one decimal (Roughly corresponding to a mile)."]
    #[serde(rename="Longitude")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub longitude: Option<f64>,
    #[doc="The postal code or zip code of the endpoint."]
    #[serde(rename="PostalCode")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub postal_code: Option<String>,
    #[doc="The region of the endpoint location. For example, corresponds to a state in US."]
    #[serde(rename="Region")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub region: Option<String>,
}

#[doc="Endpoint update request"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct EndpointRequest {
    #[doc="The address or token of the endpoint as provided by your push provider (e.g. DeviceToken or RegistrationId)."]
    #[serde(rename="Address")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub address: Option<String>,
    #[doc="Custom attributes that your app reports to Amazon Pinpoint. You can use these attributes as selection criteria when you create a segment."]
    #[serde(rename="Attributes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub attributes: Option<::std::collections::HashMap<String, Vec<String>>>,
    #[doc="The channel type.

Valid values: GCM | APNS | SMS | EMAIL"]
    #[serde(rename="ChannelType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub channel_type: Option<String>,
    #[doc="The endpoint demographic attributes."]
    #[serde(rename="Demographic")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub demographic: Option<EndpointDemographic>,
    #[doc="The last time the endpoint was updated. Provided in ISO 8601 format."]
    #[serde(rename="EffectiveDate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub effective_date: Option<String>,
    #[doc="The endpoint status. Can be either ACTIVE or INACTIVE. Will be set to INACTIVE if a delivery fails. Will be set to ACTIVE if the address is updated."]
    #[serde(rename="EndpointStatus")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub endpoint_status: Option<String>,
    #[doc="The endpoint location attributes."]
    #[serde(rename="Location")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub location: Option<EndpointLocation>,
    #[doc="Custom metrics that your app reports to Amazon Pinpoint."]
    #[serde(rename="Metrics")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub metrics: Option<::std::collections::HashMap<String, f64>>,
    #[doc="Indicates whether a user has opted out of receiving messages with one of the following values:

ALL - User has opted out of all messages.

NONE - Users has not opted out and receives all messages."]
    #[serde(rename="OptOut")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub opt_out: Option<String>,
    #[doc="The unique ID for the most recent request to update the endpoint."]
    #[serde(rename="RequestId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub request_id: Option<String>,
    #[doc="Custom user-specific attributes that your app reports to Amazon Pinpoint."]
    #[serde(rename="User")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user: Option<EndpointUser>,
}

#[doc="Endpoint response"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct EndpointResponse {
    #[doc="The address or token of the endpoint as provided by your push provider (e.g. DeviceToken or RegistrationId)."]
    #[serde(rename="Address")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub address: Option<String>,
    #[doc="The ID of the application associated with the endpoint."]
    #[serde(rename="ApplicationId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub application_id: Option<String>,
    #[doc="Custom attributes that your app reports to Amazon Pinpoint. You can use these attributes as selection criteria when you create a segment."]
    #[serde(rename="Attributes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub attributes: Option<::std::collections::HashMap<String, Vec<String>>>,
    #[doc="The channel type.

Valid values: GCM | APNS | SMS | EMAIL"]
    #[serde(rename="ChannelType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub channel_type: Option<String>,
    #[doc="A number from 0 - 99 that represents the cohort the endpoint is assigned to. Endpoints are grouped into cohorts randomly, and each cohort contains approximately 1 percent of the endpoints for an app. Amazon Pinpoint assigns cohorts to the holdout or treatment allocations for a campaign."]
    #[serde(rename="CohortId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cohort_id: Option<String>,
    #[doc="The last time the endpoint was created. Provided in ISO 8601 format."]
    #[serde(rename="CreationDate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub creation_date: Option<String>,
    #[doc="The endpoint demographic attributes."]
    #[serde(rename="Demographic")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub demographic: Option<EndpointDemographic>,
    #[doc="The last time the endpoint was updated. Provided in ISO 8601 format."]
    #[serde(rename="EffectiveDate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub effective_date: Option<String>,
    #[doc="The endpoint status. Can be either ACTIVE or INACTIVE. Will be set to INACTIVE if a delivery fails. Will be set to ACTIVE if the address is updated."]
    #[serde(rename="EndpointStatus")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub endpoint_status: Option<String>,
    #[doc="The unique ID that you assigned to the endpoint. The ID should be a globally unique identifier (GUID) to ensure that it is unique compared to all other endpoints for the application."]
    #[serde(rename="Id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<String>,
    #[doc="The endpoint location attributes."]
    #[serde(rename="Location")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub location: Option<EndpointLocation>,
    #[doc="Custom metrics that your app reports to Amazon Pinpoint."]
    #[serde(rename="Metrics")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub metrics: Option<::std::collections::HashMap<String, f64>>,
    #[doc="Indicates whether a user has opted out of receiving messages with one of the following values:

ALL - User has opted out of all messages.

NONE - Users has not opted out and receives all messages."]
    #[serde(rename="OptOut")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub opt_out: Option<String>,
    #[doc="The unique ID for the most recent request to update the endpoint."]
    #[serde(rename="RequestId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub request_id: Option<String>,
    #[doc="The ShardId of endpoint"]
    #[serde(rename="ShardId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub shard_id: Option<String>,
    #[doc="Custom user-specific attributes that your app reports to Amazon Pinpoint."]
    #[serde(rename="User")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user: Option<EndpointUser>,
}

#[doc="Endpoint user specific custom userAttributes"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct EndpointUser {
    #[doc="Custom attributes specific to the user."]
    #[serde(rename="UserAttributes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user_attributes: Option<::std::collections::HashMap<String, Vec<String>>>,
    #[doc="The unique ID of the user."]
    #[serde(rename="UserId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user_id: Option<String>,
}

#[doc="Model for an event publishing subscription export."]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct EventStream {
    #[doc="The ID of the application from which events should be published."]
    #[serde(rename="ApplicationId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub application_id: Option<String>,
    #[doc="The Amazon Resource Name (ARN) of the Amazon Kinesis stream or Firehose delivery stream to which you want to publish events.
 Firehose ARN: arn:aws:firehose:REGION:ACCOUNT_ID:deliverystream/STREAM_NAME
 Kinesis ARN: arn:aws:kinesis:REGION:ACCOUNT_ID:stream/STREAM_NAME"]
    #[serde(rename="DestinationStreamArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub destination_stream_arn: Option<String>,
    #[doc="The external ID assigned the IAM role that authorizes Amazon Pinpoint to publish to the stream."]
    #[serde(rename="ExternalId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub external_id: Option<String>,
    #[doc="The date the event stream was last updated in ISO 8601 format."]
    #[serde(rename="LastModifiedDate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_modified_date: Option<String>,
    #[doc="The IAM user who last modified the event stream."]
    #[serde(rename="LastUpdatedBy")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_updated_by: Option<String>,
    #[doc="The IAM role that authorizes Amazon Pinpoint to publish events to the stream in your account."]
    #[serde(rename="RoleArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub role_arn: Option<String>,
}

#[doc="Google Cloud Messaging credentials"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct GCMChannelRequest {
    #[doc="Platform credential API key from Google."]
    #[serde(rename="ApiKey")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub api_key: Option<String>,
    #[doc="If the channel is enabled for sending messages."]
    #[serde(rename="Enabled")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub enabled: Option<bool>,
}

#[doc="Google Cloud Messaging channel definition"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct GCMChannelResponse {
    #[doc="The ID of the application to which the channel applies."]
    #[serde(rename="ApplicationId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub application_id: Option<String>,
    #[doc="When was this segment created"]
    #[serde(rename="CreationDate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub creation_date: Option<String>,
    #[doc="The GCM API key from Google."]
    #[serde(rename="Credential")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub credential: Option<String>,
    #[doc="If the channel is enabled for sending messages."]
    #[serde(rename="Enabled")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub enabled: Option<bool>,
    #[doc="Channel ID. Not used. Present only for backwards compatibility."]
    #[serde(rename="Id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<String>,
    #[doc="Is this channel archived"]
    #[serde(rename="IsArchived")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub is_archived: Option<bool>,
    #[doc="Who last updated this entry"]
    #[serde(rename="LastModifiedBy")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_modified_by: Option<String>,
    #[doc="Last date this was updated"]
    #[serde(rename="LastModifiedDate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_modified_date: Option<String>,
    #[doc="The platform type. Will be GCM"]
    #[serde(rename="Platform")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub platform: Option<String>,
    #[doc="Version of channel"]
    #[serde(rename="Version")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub version: Option<i64>,
}

#[doc="GCM Message."]
#[derive(Default,Debug,Clone,Serialize)]
pub struct GCMMessage {
    #[doc="The action that occurs if the user taps a push notification delivered by the campaign: OPEN_APP - Your app launches, or it becomes the foreground app if it has been sent to the background. This is the default action. DEEP_LINK - Uses deep linking features in iOS and Android to open your app and display a designated user interface within the app. URL - The default mobile browser on the user's device launches and opens a web page at the URL you specify. Possible values include: OPEN_APP | DEEP_LINK | URL"]
    #[serde(rename="Action")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub action: Option<String>,
    #[doc="The message body of the notification, the email body or the text message."]
    #[serde(rename="Body")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub body: Option<String>,
    #[doc="This parameter identifies a group of messages (e.g., with collapse_key: \"Updates Available\") that can be collapsed, so that only the last message gets sent when delivery can be resumed. This is intended to avoid sending too many of the same messages when the device comes back online or becomes active."]
    #[serde(rename="CollapseKey")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub collapse_key: Option<String>,
    #[doc="The data payload used for a silent push. This payload is added to the notifications' data.pinpoint.jsonBody' object"]
    #[serde(rename="Data")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub data: Option<::std::collections::HashMap<String, String>>,
    #[doc="The icon image name of the asset saved in your application."]
    #[serde(rename="IconReference")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub icon_reference: Option<String>,
    #[doc="The URL that points to an image used as the large icon to the notification content view."]
    #[serde(rename="ImageIconUrl")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub image_icon_url: Option<String>,
    #[doc="The URL that points to an image used in the push notification."]
    #[serde(rename="ImageUrl")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub image_url: Option<String>,
    #[doc="The Raw JSON formatted string to be used as the payload. This value overrides the message."]
    #[serde(rename="RawContent")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub raw_content: Option<String>,
    #[doc="This parameter specifies the package name of the application where the registration tokens must match in order to receive the message."]
    #[serde(rename="RestrictedPackageName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub restricted_package_name: Option<String>,
    #[doc="Indicates if the message should display on the users device. Silent pushes can be used for Remote Configuration and Phone Home use cases."]
    #[serde(rename="SilentPush")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub silent_push: Option<bool>,
    #[doc="The URL that points to an image used as the small icon for the notification which will be used to represent the notification in the status bar and content view"]
    #[serde(rename="SmallImageIconUrl")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub small_image_icon_url: Option<String>,
    #[doc="Indicates a sound to play when the device receives the notification. Supports default, or the filename of a sound resource bundled in the app. Android sound files must reside in /res/raw/"]
    #[serde(rename="Sound")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sound: Option<String>,
    #[doc="Default message substitutions. Can be overridden by individual address substitutions."]
    #[serde(rename="Substitutions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub substitutions: Option<::std::collections::HashMap<String, Vec<String>>>,
    #[doc="The message title that displays above the message on the user's device."]
    #[serde(rename="Title")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub title: Option<String>,
    #[doc="The URL to open in the user's mobile browser. Used if the value for Action is URL."]
    #[serde(rename="Url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct GetApnsChannelRequest {
    #[serde(rename="ApplicationId")]
    pub application_id: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetApnsChannelResponse {
    #[serde(rename="APNSChannelResponse")]
    pub apns_channel_response: APNSChannelResponse,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct GetApnsSandboxChannelRequest {
    #[serde(rename="ApplicationId")]
    pub application_id: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetApnsSandboxChannelResponse {
    #[serde(rename="APNSSandboxChannelResponse")]
    pub apns_sandbox_channel_response: APNSSandboxChannelResponse,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct GetAppRequest {
    #[serde(rename="ApplicationId")]
    pub application_id: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetAppResponse {
    #[serde(rename="ApplicationResponse")]
    pub application_response: ApplicationResponse,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct GetApplicationSettingsRequest {
    #[serde(rename="ApplicationId")]
    pub application_id: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetApplicationSettingsResponse {
    #[serde(rename="ApplicationSettingsResource")]
    pub application_settings_resource: ApplicationSettingsResource,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct GetAppsRequest {
    #[serde(rename="PageSize")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub page_size: Option<String>,
    #[serde(rename="Token")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub token: Option<String>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetAppsResponse {
    #[serde(rename="ApplicationsResponse")]
    pub applications_response: ApplicationsResponse,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct GetCampaignActivitiesRequest {
    #[serde(rename="ApplicationId")]
    pub application_id: String,
    #[serde(rename="CampaignId")]
    pub campaign_id: String,
    #[doc="The number of entries you want on each page in the response."]
    #[serde(rename="PageSize")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub page_size: Option<String>,
    #[doc="The NextToken string returned on a previous page that you use to get the next page of results in a paginated response."]
    #[serde(rename="Token")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub token: Option<String>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetCampaignActivitiesResponse {
    #[serde(rename="ActivitiesResponse")]
    pub activities_response: ActivitiesResponse,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct GetCampaignRequest {
    #[serde(rename="ApplicationId")]
    pub application_id: String,
    #[serde(rename="CampaignId")]
    pub campaign_id: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetCampaignResponse {
    #[serde(rename="CampaignResponse")]
    pub campaign_response: CampaignResponse,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct GetCampaignVersionRequest {
    #[serde(rename="ApplicationId")]
    pub application_id: String,
    #[serde(rename="CampaignId")]
    pub campaign_id: String,
    #[serde(rename="Version")]
    pub version: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetCampaignVersionResponse {
    #[serde(rename="CampaignResponse")]
    pub campaign_response: CampaignResponse,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct GetCampaignVersionsRequest {
    #[serde(rename="ApplicationId")]
    pub application_id: String,
    #[serde(rename="CampaignId")]
    pub campaign_id: String,
    #[doc="The number of entries you want on each page in the response."]
    #[serde(rename="PageSize")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub page_size: Option<String>,
    #[doc="The NextToken string returned on a previous page that you use to get the next page of results in a paginated response."]
    #[serde(rename="Token")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub token: Option<String>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetCampaignVersionsResponse {
    #[serde(rename="CampaignsResponse")]
    pub campaigns_response: CampaignsResponse,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct GetCampaignsRequest {
    #[serde(rename="ApplicationId")]
    pub application_id: String,
    #[doc="The number of entries you want on each page in the response."]
    #[serde(rename="PageSize")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub page_size: Option<String>,
    #[doc="The NextToken string returned on a previous page that you use to get the next page of results in a paginated response."]
    #[serde(rename="Token")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub token: Option<String>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetCampaignsResponse {
    #[serde(rename="CampaignsResponse")]
    pub campaigns_response: CampaignsResponse,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct GetEmailChannelRequest {
    #[serde(rename="ApplicationId")]
    pub application_id: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetEmailChannelResponse {
    #[serde(rename="EmailChannelResponse")]
    pub email_channel_response: EmailChannelResponse,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct GetEndpointRequest {
    #[serde(rename="ApplicationId")]
    pub application_id: String,
    #[serde(rename="EndpointId")]
    pub endpoint_id: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetEndpointResponse {
    #[serde(rename="EndpointResponse")]
    pub endpoint_response: EndpointResponse,
}

#[doc="GetEventStream Request"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct GetEventStreamRequest {
    #[doc="ApplicationId"]
    #[serde(rename="ApplicationId")]
    pub application_id: String,
}

#[doc="GetEventStream Response"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetEventStreamResponse {
    #[serde(rename="EventStream")]
    pub event_stream: EventStream,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct GetGcmChannelRequest {
    #[serde(rename="ApplicationId")]
    pub application_id: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetGcmChannelResponse {
    #[serde(rename="GCMChannelResponse")]
    pub gcm_channel_response: GCMChannelResponse,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct GetImportJobRequest {
    #[serde(rename="ApplicationId")]
    pub application_id: String,
    #[serde(rename="JobId")]
    pub job_id: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetImportJobResponse {
    #[serde(rename="ImportJobResponse")]
    pub import_job_response: ImportJobResponse,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct GetImportJobsRequest {
    #[serde(rename="ApplicationId")]
    pub application_id: String,
    #[doc="The number of entries you want on each page in the response."]
    #[serde(rename="PageSize")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub page_size: Option<String>,
    #[doc="The NextToken string returned on a previous page that you use to get the next page of results in a paginated response."]
    #[serde(rename="Token")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub token: Option<String>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetImportJobsResponse {
    #[serde(rename="ImportJobsResponse")]
    pub import_jobs_response: ImportJobsResponse,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct GetSegmentImportJobsRequest {
    #[serde(rename="ApplicationId")]
    pub application_id: String,
    #[doc="The number of entries you want on each page in the response."]
    #[serde(rename="PageSize")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub page_size: Option<String>,
    #[serde(rename="SegmentId")]
    pub segment_id: String,
    #[doc="The NextToken string returned on a previous page that you use to get the next page of results in a paginated response."]
    #[serde(rename="Token")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub token: Option<String>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetSegmentImportJobsResponse {
    #[serde(rename="ImportJobsResponse")]
    pub import_jobs_response: ImportJobsResponse,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct GetSegmentRequest {
    #[serde(rename="ApplicationId")]
    pub application_id: String,
    #[serde(rename="SegmentId")]
    pub segment_id: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetSegmentResponse {
    #[serde(rename="SegmentResponse")]
    pub segment_response: SegmentResponse,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct GetSegmentVersionRequest {
    #[serde(rename="ApplicationId")]
    pub application_id: String,
    #[serde(rename="SegmentId")]
    pub segment_id: String,
    #[serde(rename="Version")]
    pub version: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetSegmentVersionResponse {
    #[serde(rename="SegmentResponse")]
    pub segment_response: SegmentResponse,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct GetSegmentVersionsRequest {
    #[serde(rename="ApplicationId")]
    pub application_id: String,
    #[doc="The number of entries you want on each page in the response."]
    #[serde(rename="PageSize")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub page_size: Option<String>,
    #[serde(rename="SegmentId")]
    pub segment_id: String,
    #[doc="The NextToken string returned on a previous page that you use to get the next page of results in a paginated response."]
    #[serde(rename="Token")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub token: Option<String>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetSegmentVersionsResponse {
    #[serde(rename="SegmentsResponse")]
    pub segments_response: SegmentsResponse,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct GetSegmentsRequest {
    #[serde(rename="ApplicationId")]
    pub application_id: String,
    #[doc="The number of entries you want on each page in the response."]
    #[serde(rename="PageSize")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub page_size: Option<String>,
    #[doc="The NextToken string returned on a previous page that you use to get the next page of results in a paginated response."]
    #[serde(rename="Token")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub token: Option<String>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetSegmentsResponse {
    #[serde(rename="SegmentsResponse")]
    pub segments_response: SegmentsResponse,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct GetSmsChannelRequest {
    #[serde(rename="ApplicationId")]
    pub application_id: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetSmsChannelResponse {
    #[serde(rename="SMSChannelResponse")]
    pub sms_channel_response: SMSChannelResponse,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct ImportJobRequest {
    #[doc="Sets whether the endpoints create a segment when they are imported."]
    #[serde(rename="DefineSegment")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub define_segment: Option<bool>,
    #[doc="A unique, custom ID assigned to the IAM role that restricts who can assume the role.	"]
    #[serde(rename="ExternalId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub external_id: Option<String>,
    #[doc="The format of the files that contain the endpoint definitions.
Valid values: CSV, JSON"]
    #[serde(rename="Format")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub format: Option<String>,
    #[doc="Sets whether the endpoints are registered with Amazon Pinpoint when they are imported."]
    #[serde(rename="RegisterEndpoints")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub register_endpoints: Option<bool>,
    #[doc="The Amazon Resource Name (ARN) of an IAM role that grants Amazon Pinpoint access to the Amazon S3 location that contains the endpoints to import."]
    #[serde(rename="RoleArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub role_arn: Option<String>,
    #[doc="A URL that points to the location within an Amazon S3 bucket that contains the endpoints to import. The location can be a folder or a single file.
The URL should follow this format: s3://bucket-name/folder-name/file-name

Amazon Pinpoint will import endpoints from this location and any subfolders it contains."]
    #[serde(rename="S3Url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub s3_url: Option<String>,
    #[doc="The ID of the segment to update if the import job is meant to update an existing segment."]
    #[serde(rename="SegmentId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub segment_id: Option<String>,
    #[doc="A custom name for the segment created by the import job. Use if DefineSegment is true."]
    #[serde(rename="SegmentName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub segment_name: Option<String>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct ImportJobResource {
    #[doc="Sets whether the endpoints create a segment when they are imported."]
    #[serde(rename="DefineSegment")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub define_segment: Option<bool>,
    #[doc="A unique, custom ID assigned to the IAM role that restricts who can assume the role.	"]
    #[serde(rename="ExternalId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub external_id: Option<String>,
    #[doc="The format of the files that contain the endpoint definitions.
Valid values: CSV, JSON"]
    #[serde(rename="Format")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub format: Option<String>,
    #[doc="Sets whether the endpoints are registered with Amazon Pinpoint when they are imported."]
    #[serde(rename="RegisterEndpoints")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub register_endpoints: Option<bool>,
    #[doc="The Amazon Resource Name (ARN) of an IAM role that grants Amazon Pinpoint access to the Amazon S3 location that contains the endpoints to import."]
    #[serde(rename="RoleArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub role_arn: Option<String>,
    #[doc="A URL that points to the location within an Amazon S3 bucket that contains the endpoints to import. The location can be a folder or a single file.
The URL should follow this format: s3://bucket-name/folder-name/file-name

Amazon Pinpoint will import endpoints from this location and any subfolders it contains."]
    #[serde(rename="S3Url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub s3_url: Option<String>,
    #[doc="The ID of the segment to update if the import job is meant to update an existing segment."]
    #[serde(rename="SegmentId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub segment_id: Option<String>,
    #[doc="A custom name for the segment created by the import job. Use if DefineSegment is true."]
    #[serde(rename="SegmentName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub segment_name: Option<String>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct ImportJobResponse {
    #[doc="The unique ID of the application to which the import job applies."]
    #[serde(rename="ApplicationId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub application_id: Option<String>,
    #[doc="The number of pieces that have successfully imported as of the time of the request."]
    #[serde(rename="CompletedPieces")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub completed_pieces: Option<i64>,
    #[doc="The date the import job completed in ISO 8601 format."]
    #[serde(rename="CompletionDate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub completion_date: Option<String>,
    #[doc="The date the import job was created in ISO 8601 format."]
    #[serde(rename="CreationDate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub creation_date: Option<String>,
    #[doc="The import job settings."]
    #[serde(rename="Definition")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub definition: Option<ImportJobResource>,
    #[doc="The number of pieces that have failed to import as of the time of the request."]
    #[serde(rename="FailedPieces")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub failed_pieces: Option<i64>,
    #[doc="Provides up to 100 of the first failed entries for the job, if any exist."]
    #[serde(rename="Failures")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub failures: Option<Vec<String>>,
    #[doc="The unique ID of the import job."]
    #[serde(rename="Id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<String>,
    #[doc="The status of the import job.
Valid values: CREATED, INITIALIZING, PROCESSING, COMPLETING, COMPLETED, FAILING, FAILED

The job status is FAILED if one or more pieces failed to import."]
    #[serde(rename="JobStatus")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub job_status: Option<String>,
    #[doc="The number of endpoints that failed to import; for example, because of syntax errors."]
    #[serde(rename="TotalFailures")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub total_failures: Option<i64>,
    #[doc="The total number of pieces that must be imported to finish the job. Each piece is an approximately equal portion of the endpoints to import."]
    #[serde(rename="TotalPieces")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub total_pieces: Option<i64>,
    #[doc="The number of endpoints that were processed by the import job."]
    #[serde(rename="TotalProcessed")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub total_processed: Option<i64>,
    #[doc="The job type. Will be Import."]
    #[serde(rename="Type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub type_: Option<String>,
}

#[doc="Import job list."]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ImportJobsResponse {
    #[doc="A list of import jobs for the application."]
    #[serde(rename="Item")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub item: Option<Vec<ImportJobResponse>>,
    #[doc="The string that you use in a subsequent request to get the next page of results in a paginated response."]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct Message {
    #[doc="The action that occurs if the user taps a push notification delivered by the campaign:
OPEN_APP - Your app launches, or it becomes the foreground app if it has been sent to the background. This is the default action.

DEEP_LINK - Uses deep linking features in iOS and Android to open your app and display a designated user interface within the app.

URL - The default mobile browser on the user's device launches and opens a web page at the URL you specify."]
    #[serde(rename="Action")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub action: Option<String>,
    #[doc="The message body. Can include up to 140 characters."]
    #[serde(rename="Body")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub body: Option<String>,
    #[doc="The URL that points to the icon image for the push notification icon, for example, the app icon."]
    #[serde(rename="ImageIconUrl")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub image_icon_url: Option<String>,
    #[doc="The URL that points to the small icon image for the push notification icon, for example, the app icon."]
    #[serde(rename="ImageSmallIconUrl")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub image_small_icon_url: Option<String>,
    #[doc="The URL that points to an image used in the push notification."]
    #[serde(rename="ImageUrl")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub image_url: Option<String>,
    #[doc="The JSON payload used for a silent push."]
    #[serde(rename="JsonBody")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub json_body: Option<String>,
    #[doc="The URL that points to the media resource, for example a .mp4 or .gif file."]
    #[serde(rename="MediaUrl")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub media_url: Option<String>,
    #[doc="The Raw JSON formatted string to be used as the payload. This value overrides the message."]
    #[serde(rename="RawContent")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub raw_content: Option<String>,
    #[doc="Indicates if the message should display on the users device.

Silent pushes can be used for Remote Configuration and Phone Home use cases. "]
    #[serde(rename="SilentPush")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub silent_push: Option<bool>,
    #[doc="The message title that displays above the message on the user's device."]
    #[serde(rename="Title")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub title: Option<String>,
    #[doc="The URL to open in the user's mobile browser. Used if the value for Action is URL."]
    #[serde(rename="Url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,
}

#[doc="Simple message object."]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct MessageBody {
    #[doc="The error message returned from the API."]
    #[serde(rename="Message")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,
    #[doc="The unique message body ID."]
    #[serde(rename="RequestID")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub request_id: Option<String>,
}

#[doc="Message configuration for a campaign."]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct MessageConfiguration {
    #[doc="The message that the campaign delivers to APNS channels. Overrides the default message."]
    #[serde(rename="APNSMessage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub apns_message: Option<Message>,
    #[doc="The default message for all channels."]
    #[serde(rename="DefaultMessage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub default_message: Option<Message>,
    #[doc="The email message configuration."]
    #[serde(rename="EmailMessage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub email_message: Option<CampaignEmailMessage>,
    #[doc="The message that the campaign delivers to GCM channels. Overrides the default message."]
    #[serde(rename="GCMMessage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub gcm_message: Option<Message>,
    #[doc="The SMS message configuration."]
    #[serde(rename="SMSMessage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sms_message: Option<CampaignSmsMessage>,
}

#[doc="Send message request."]
#[derive(Default,Debug,Clone,Serialize)]
pub struct MessageRequest {
    #[doc="A map of destination addresses, with the address as the key(Email address, phone number or push token) and the Address Configuration as the value."]
    #[serde(rename="Addresses")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub addresses: Option<::std::collections::HashMap<String, AddressConfiguration>>,
    #[doc="A map of custom attributes to attributes to be attached to the message. This payload is added to the push notification's 'data.pinpoint' object or added to the email/sms delivery receipt event attributes."]
    #[serde(rename="Context")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub context: Option<::std::collections::HashMap<String, String>>,
    #[doc="Message configuration."]
    #[serde(rename="MessageConfiguration")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub message_configuration: Option<DirectMessageConfiguration>,
}

#[doc="Send message response."]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct MessageResponse {
    #[doc="Application id of the message."]
    #[serde(rename="ApplicationId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub application_id: Option<String>,
    #[doc="Original request Id for which this message was delivered."]
    #[serde(rename="RequestId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub request_id: Option<String>,
    #[doc="A map containing a multi part response for each address, with the address as the key(Email address, phone number or push token) and the result as the value."]
    #[serde(rename="Result")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub result: Option<::std::collections::HashMap<String, MessageResult>>,
}

#[doc="The result from sending a message to an address."]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct MessageResult {
    #[doc="Delivery status of message."]
    #[serde(rename="DeliveryStatus")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub delivery_status: Option<String>,
    #[doc="Downstream service status code."]
    #[serde(rename="StatusCode")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status_code: Option<i64>,
    #[doc="Status message for message delivery."]
    #[serde(rename="StatusMessage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status_message: Option<String>,
    #[doc="If token was updated as part of delivery. (This is GCM Specific)"]
    #[serde(rename="UpdatedToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub updated_token: Option<String>,
}

#[doc="PutEventStream Request"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct PutEventStreamRequest {
    #[doc="ApplicationId"]
    #[serde(rename="ApplicationId")]
    pub application_id: String,
    #[doc="EventStream to write."]
    #[serde(rename="WriteEventStream")]
    pub write_event_stream: WriteEventStream,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct PutEventStreamResponse {
    #[serde(rename="EventStream")]
    pub event_stream: EventStream,
}

#[doc="Quiet Time"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct QuietTime {
    #[doc="The default end time for quiet time in ISO 8601 format."]
    #[serde(rename="End")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub end: Option<String>,
    #[doc="The default start time for quiet time in ISO 8601 format."]
    #[serde(rename="Start")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub start: Option<String>,
}

#[doc="Define how a segment based on recency of use."]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct RecencyDimension {
    #[doc="The length of time during which users have been active or inactive with your app.
Valid values: HR_24, DAY_7, DAY_14, DAY_30"]
    #[serde(rename="Duration")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub duration: Option<String>,
    #[doc="The recency dimension type:
ACTIVE - Users who have used your app within the specified duration are included in the segment.
INACTIVE - Users who have not used your app within the specified duration are included in the segment."]
    #[serde(rename="RecencyType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub recency_type: Option<String>,
}

#[doc="SMS Channel Request"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct SMSChannelRequest {
    #[doc="If the channel is enabled for sending messages."]
    #[serde(rename="Enabled")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub enabled: Option<bool>,
    #[doc="Sender identifier of your messages."]
    #[serde(rename="SenderId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sender_id: Option<String>,
}

#[doc="SMS Channel Response."]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct SMSChannelResponse {
    #[doc="The unique ID of the application to which the SMS channel belongs."]
    #[serde(rename="ApplicationId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub application_id: Option<String>,
    #[doc="The date that the settings were last updated in ISO 8601 format."]
    #[serde(rename="CreationDate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub creation_date: Option<String>,
    #[doc="If the channel is enabled for sending messages."]
    #[serde(rename="Enabled")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub enabled: Option<bool>,
    #[doc="Channel ID. Not used, only for backwards compatibility."]
    #[serde(rename="Id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<String>,
    #[doc="Is this channel archived"]
    #[serde(rename="IsArchived")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub is_archived: Option<bool>,
    #[doc="Who last updated this entry"]
    #[serde(rename="LastModifiedBy")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_modified_by: Option<String>,
    #[doc="Last date this was updated"]
    #[serde(rename="LastModifiedDate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_modified_date: Option<String>,
    #[doc="Platform type. Will be \"SMS\""]
    #[serde(rename="Platform")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub platform: Option<String>,
    #[doc="Sender identifier of your messages."]
    #[serde(rename="SenderId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sender_id: Option<String>,
    #[doc="The short code registered with the phone provider."]
    #[serde(rename="ShortCode")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub short_code: Option<String>,
    #[doc="Version of channel"]
    #[serde(rename="Version")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub version: Option<i64>,
}

#[doc="SMS Message."]
#[derive(Default,Debug,Clone,Serialize)]
pub struct SMSMessage {
    #[doc="The message body of the notification, the email body or the text message."]
    #[serde(rename="Body")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub body: Option<String>,
    #[doc="Is this a transaction priority message or lower priority."]
    #[serde(rename="MessageType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub message_type: Option<String>,
    #[doc="Sender ID of sent message."]
    #[serde(rename="SenderId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sender_id: Option<String>,
    #[doc="Default message substitutions. Can be overridden by individual address substitutions."]
    #[serde(rename="Substitutions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub substitutions: Option<::std::collections::HashMap<String, Vec<String>>>,
}

#[doc="Shcedule that defines when a campaign is run."]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct Schedule {
    #[doc="The scheduled time that the campaign ends in ISO 8601 format."]
    #[serde(rename="EndTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub end_time: Option<String>,
    #[doc="How often the campaign delivers messages.

Valid values: ONCE, HOURLY, DAILY, WEEKLY, MONTHLY"]
    #[serde(rename="Frequency")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub frequency: Option<String>,
    #[doc="Indicates whether the campaign schedule takes effect according to each user's local time."]
    #[serde(rename="IsLocalTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub is_local_time: Option<bool>,
    #[doc="The time during which the campaign sends no messages."]
    #[serde(rename="QuietTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub quiet_time: Option<QuietTime>,
    #[doc="The scheduled time that the campaign begins in ISO 8601 format."]
    #[serde(rename="StartTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub start_time: Option<String>,
    #[doc="The starting UTC offset for the schedule if the value for isLocalTime is true

Valid values: 
UTC
UTC+01
UTC+02
UTC+03
UTC+03:30
UTC+04
UTC+04:30
UTC+05
UTC+05:30
UTC+05:45
UTC+06
UTC+06:30
UTC+07
UTC+08
UTC+09
UTC+09:30
UTC+10
UTC+10:30
UTC+11
UTC+12
UTC+13
UTC-02
UTC-03
UTC-04
UTC-05
UTC-06
UTC-07
UTC-08
UTC-09
UTC-10
UTC-11"]
    #[serde(rename="Timezone")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub timezone: Option<String>,
}

#[doc="Segment behavior dimensions"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct SegmentBehaviors {
    #[doc="The recency of use."]
    #[serde(rename="Recency")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub recency: Option<RecencyDimension>,
}

#[doc="Segment demographic dimensions"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct SegmentDemographics {
    #[doc="The app version criteria for the segment."]
    #[serde(rename="AppVersion")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub app_version: Option<SetDimension>,
    #[doc="The channel criteria for the segment."]
    #[serde(rename="Channel")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub channel: Option<SetDimension>,
    #[doc="The device type criteria for the segment."]
    #[serde(rename="DeviceType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub device_type: Option<SetDimension>,
    #[doc="The device make criteria for the segment."]
    #[serde(rename="Make")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub make: Option<SetDimension>,
    #[doc="The device model criteria for the segment."]
    #[serde(rename="Model")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub model: Option<SetDimension>,
    #[doc="The device platform criteria for the segment."]
    #[serde(rename="Platform")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub platform: Option<SetDimension>,
}

#[doc="Segment dimensions"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct SegmentDimensions {
    #[doc="Custom segment attributes."]
    #[serde(rename="Attributes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub attributes: Option<::std::collections::HashMap<String, AttributeDimension>>,
    #[doc="The segment behaviors attributes."]
    #[serde(rename="Behavior")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub behavior: Option<SegmentBehaviors>,
    #[doc="The segment demographics attributes."]
    #[serde(rename="Demographic")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub demographic: Option<SegmentDemographics>,
    #[doc="The segment location attributes."]
    #[serde(rename="Location")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub location: Option<SegmentLocation>,
    #[doc="Custom segment user attributes."]
    #[serde(rename="UserAttributes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user_attributes: Option<::std::collections::HashMap<String, AttributeDimension>>,
}

#[doc="Segment import definition."]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct SegmentImportResource {
    #[doc="Channel type counts"]
    #[serde(rename="ChannelCounts")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub channel_counts: Option<::std::collections::HashMap<String, i64>>,
    #[doc="A unique, custom ID assigned to the IAM role that restricts who can assume the role."]
    #[serde(rename="ExternalId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub external_id: Option<String>,
    #[doc="The format of the endpoint files that were imported to create this segment.
Valid values: CSV, JSON"]
    #[serde(rename="Format")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub format: Option<String>,
    #[doc="The Amazon Resource Name (ARN) of an IAM role that grants Amazon Pinpoint access to the endpoints in Amazon S3."]
    #[serde(rename="RoleArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub role_arn: Option<String>,
    #[doc="A URL that points to the Amazon S3 location from which the endpoints for this segment were imported."]
    #[serde(rename="S3Url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub s3_url: Option<String>,
    #[doc="The number of endpoints that were successfully imported to create this segment."]
    #[serde(rename="Size")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub size: Option<i64>,
}

#[doc="Segment location dimensions"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct SegmentLocation {
    #[doc="The country filter according to ISO 3166-1 Alpha-2 codes."]
    #[serde(rename="Country")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub country: Option<SetDimension>,
}

#[doc="Segment definition."]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct SegmentResponse {
    #[doc="The ID of the application to which the segment applies."]
    #[serde(rename="ApplicationId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub application_id: Option<String>,
    #[doc="The date the segment was created in ISO 8601 format."]
    #[serde(rename="CreationDate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub creation_date: Option<String>,
    #[doc="The segment dimensions attributes."]
    #[serde(rename="Dimensions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub dimensions: Option<SegmentDimensions>,
    #[doc="The unique segment ID."]
    #[serde(rename="Id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<String>,
    #[doc="The import job settings."]
    #[serde(rename="ImportDefinition")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub import_definition: Option<SegmentImportResource>,
    #[doc="The date the segment was last updated in ISO 8601 format."]
    #[serde(rename="LastModifiedDate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_modified_date: Option<String>,
    #[doc="The name of segment"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    #[doc="The segment type:
DIMENSIONAL - A dynamic segment built from selection criteria based on endpoint data reported by your app. You create this type of segment by using the segment builder in the Amazon Pinpoint console or by making a POST request to the segments resource.
IMPORT - A static segment built from an imported set of endpoint definitions. You create this type of segment by importing a segment in the Amazon Pinpoint console or by making a POST request to the jobs/import resource."]
    #[serde(rename="SegmentType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub segment_type: Option<String>,
    #[doc="The segment version number."]
    #[serde(rename="Version")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub version: Option<i64>,
}

#[doc="Segments in your account."]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct SegmentsResponse {
    #[doc="The list of segments."]
    #[serde(rename="Item")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub item: Option<Vec<SegmentResponse>>,
    #[doc="An identifier used to retrieve the next page of results. The token is null if no additional pages exist."]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct SendMessagesRequest {
    #[serde(rename="ApplicationId")]
    pub application_id: String,
    #[serde(rename="MessageRequest")]
    pub message_request: MessageRequest,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct SendMessagesResponse {
    #[serde(rename="MessageResponse")]
    pub message_response: MessageResponse,
}

#[doc="Dimension specification of a segment."]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct SetDimension {
    #[doc="The type of dimension:
INCLUSIVE - Endpoints that match the criteria are included in the segment.
EXCLUSIVE - Endpoints that match the criteria are excluded from the segment."]
    #[serde(rename="DimensionType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub dimension_type: Option<String>,
    #[doc="The criteria values for the segment dimension. Endpoints with matching attribute values are included or excluded from the segment, depending on the setting for Type."]
    #[serde(rename="Values")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[doc="Treatment resource"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct TreatmentResource {
    #[doc="The unique treatment ID."]
    #[serde(rename="Id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<String>,
    #[doc="The message configuration settings."]
    #[serde(rename="MessageConfiguration")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub message_configuration: Option<MessageConfiguration>,
    #[doc="The campaign schedule."]
    #[serde(rename="Schedule")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub schedule: Option<Schedule>,
    #[doc="The allocated percentage of users for this treatment."]
    #[serde(rename="SizePercent")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub size_percent: Option<i64>,
    #[doc="The treatment status."]
    #[serde(rename="State")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub state: Option<CampaignState>,
    #[doc="A custom description for the treatment."]
    #[serde(rename="TreatmentDescription")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub treatment_description: Option<String>,
    #[doc="The custom name of a variation of the campaign used for A/B testing."]
    #[serde(rename="TreatmentName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub treatment_name: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct UpdateApnsChannelRequest {
    #[serde(rename="APNSChannelRequest")]
    pub apns_channel_request: APNSChannelRequest,
    #[serde(rename="ApplicationId")]
    pub application_id: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct UpdateApnsChannelResponse {
    #[serde(rename="APNSChannelResponse")]
    pub apns_channel_response: APNSChannelResponse,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct UpdateApnsSandboxChannelRequest {
    #[serde(rename="APNSSandboxChannelRequest")]
    pub apns_sandbox_channel_request: APNSSandboxChannelRequest,
    #[serde(rename="ApplicationId")]
    pub application_id: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct UpdateApnsSandboxChannelResponse {
    #[serde(rename="APNSSandboxChannelResponse")]
    pub apns_sandbox_channel_response: APNSSandboxChannelResponse,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct UpdateApplicationSettingsRequest {
    #[serde(rename="ApplicationId")]
    pub application_id: String,
    #[serde(rename="WriteApplicationSettingsRequest")]
    pub write_application_settings_request: WriteApplicationSettingsRequest,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct UpdateApplicationSettingsResponse {
    #[serde(rename="ApplicationSettingsResource")]
    pub application_settings_resource: ApplicationSettingsResource,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct UpdateCampaignRequest {
    #[serde(rename="ApplicationId")]
    pub application_id: String,
    #[serde(rename="CampaignId")]
    pub campaign_id: String,
    #[serde(rename="WriteCampaignRequest")]
    pub write_campaign_request: WriteCampaignRequest,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct UpdateCampaignResponse {
    #[serde(rename="CampaignResponse")]
    pub campaign_response: CampaignResponse,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct UpdateEmailChannelRequest {
    #[serde(rename="ApplicationId")]
    pub application_id: String,
    #[serde(rename="EmailChannelRequest")]
    pub email_channel_request: EmailChannelRequest,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct UpdateEmailChannelResponse {
    #[serde(rename="EmailChannelResponse")]
    pub email_channel_response: EmailChannelResponse,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct UpdateEndpointRequest {
    #[serde(rename="ApplicationId")]
    pub application_id: String,
    #[serde(rename="EndpointId")]
    pub endpoint_id: String,
    #[serde(rename="EndpointRequest")]
    pub endpoint_request: EndpointRequest,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct UpdateEndpointResponse {
    #[serde(rename="MessageBody")]
    pub message_body: MessageBody,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct UpdateEndpointsBatchRequest {
    #[serde(rename="ApplicationId")]
    pub application_id: String,
    #[serde(rename="EndpointBatchRequest")]
    pub endpoint_batch_request: EndpointBatchRequest,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct UpdateEndpointsBatchResponse {
    #[serde(rename="MessageBody")]
    pub message_body: MessageBody,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct UpdateGcmChannelRequest {
    #[serde(rename="ApplicationId")]
    pub application_id: String,
    #[serde(rename="GCMChannelRequest")]
    pub gcm_channel_request: GCMChannelRequest,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct UpdateGcmChannelResponse {
    #[serde(rename="GCMChannelResponse")]
    pub gcm_channel_response: GCMChannelResponse,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct UpdateSegmentRequest {
    #[serde(rename="ApplicationId")]
    pub application_id: String,
    #[serde(rename="SegmentId")]
    pub segment_id: String,
    #[serde(rename="WriteSegmentRequest")]
    pub write_segment_request: WriteSegmentRequest,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct UpdateSegmentResponse {
    #[serde(rename="SegmentResponse")]
    pub segment_response: SegmentResponse,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct UpdateSmsChannelRequest {
    #[serde(rename="ApplicationId")]
    pub application_id: String,
    #[serde(rename="SMSChannelRequest")]
    pub sms_channel_request: SMSChannelRequest,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct UpdateSmsChannelResponse {
    #[serde(rename="SMSChannelResponse")]
    pub sms_channel_response: SMSChannelResponse,
}

#[doc="Creating application setting request"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct WriteApplicationSettingsRequest {
    #[doc="The default campaign limits for the app. These limits apply to each campaign for the app, unless the campaign overrides the default with limits of its own."]
    #[serde(rename="Limits")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub limits: Option<CampaignLimits>,
    #[doc="The default quiet time for the app. Each campaign for this app sends no messages during this time unless the campaign overrides the default with a quiet time of its own."]
    #[serde(rename="QuietTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub quiet_time: Option<QuietTime>,
}

#[doc="Used to create a campaign."]
#[derive(Default,Debug,Clone,Serialize)]
pub struct WriteCampaignRequest {
    #[doc="Treatments that are defined in addition to the default treatment."]
    #[serde(rename="AdditionalTreatments")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub additional_treatments: Option<Vec<WriteTreatmentResource>>,
    #[doc="A description of the campaign."]
    #[serde(rename="Description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,
    #[doc="The allocated percentage of end users who will not receive messages from this campaign."]
    #[serde(rename="HoldoutPercent")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub holdout_percent: Option<i64>,
    #[doc="Indicates whether the campaign is paused. A paused campaign does not send messages unless you resume it by setting IsPaused to false."]
    #[serde(rename="IsPaused")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub is_paused: Option<bool>,
    #[doc="The campaign limits settings."]
    #[serde(rename="Limits")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub limits: Option<CampaignLimits>,
    #[doc="The message configuration settings."]
    #[serde(rename="MessageConfiguration")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub message_configuration: Option<MessageConfiguration>,
    #[doc="The custom name of the campaign."]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    #[doc="The campaign schedule."]
    #[serde(rename="Schedule")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub schedule: Option<Schedule>,
    #[doc="The ID of the segment to which the campaign sends messages."]
    #[serde(rename="SegmentId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub segment_id: Option<String>,
    #[doc="The version of the segment to which the campaign sends messages."]
    #[serde(rename="SegmentVersion")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub segment_version: Option<i64>,
    #[doc="A custom description for the treatment."]
    #[serde(rename="TreatmentDescription")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub treatment_description: Option<String>,
    #[doc="The custom name of a variation of the campaign used for A/B testing."]
    #[serde(rename="TreatmentName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub treatment_name: Option<String>,
}

#[doc="Request to save an EventStream."]
#[derive(Default,Debug,Clone,Serialize)]
pub struct WriteEventStream {
    #[doc="The Amazon Resource Name (ARN) of the Amazon Kinesis stream or Firehose delivery stream to which you want to publish events.
 Firehose ARN: arn:aws:firehose:REGION:ACCOUNT_ID:deliverystream/STREAM_NAME
 Kinesis ARN: arn:aws:kinesis:REGION:ACCOUNT_ID:stream/STREAM_NAME"]
    #[serde(rename="DestinationStreamArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub destination_stream_arn: Option<String>,
    #[doc="The external ID assigned the IAM role that authorizes Amazon Pinpoint to publish to the stream."]
    #[serde(rename="ExternalId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub external_id: Option<String>,
    #[doc="The IAM role that authorizes Amazon Pinpoint to publish events to the stream in your account."]
    #[serde(rename="RoleArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub role_arn: Option<String>,
}

#[doc="Segment definition."]
#[derive(Default,Debug,Clone,Serialize)]
pub struct WriteSegmentRequest {
    #[doc="The segment dimensions attributes."]
    #[serde(rename="Dimensions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub dimensions: Option<SegmentDimensions>,
    #[doc="The name of segment"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
}

#[doc="Used to create a campaign treatment."]
#[derive(Default,Debug,Clone,Serialize)]
pub struct WriteTreatmentResource {
    #[doc="The message configuration settings."]
    #[serde(rename="MessageConfiguration")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub message_configuration: Option<MessageConfiguration>,
    #[doc="The campaign schedule."]
    #[serde(rename="Schedule")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub schedule: Option<Schedule>,
    #[doc="The allocated percentage of users for this treatment."]
    #[serde(rename="SizePercent")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub size_percent: Option<i64>,
    #[doc="A custom description for the treatment."]
    #[serde(rename="TreatmentDescription")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub treatment_description: Option<String>,
    #[doc="The custom name of a variation of the campaign used for A/B testing."]
    #[serde(rename="TreatmentName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub treatment_name: Option<String>,
}

/// Errors returned by CreateApp
#[derive(Debug, PartialEq)]
pub enum CreateAppError {
    ///Simple message object.
    BadRequest(String),
    ///Simple message object.
    Forbidden(String),
    ///Simple message object.
    InternalServerError(String),
    ///Simple message object.
    MethodNotAllowed(String),
    ///Simple message object.
    NotFound(String),
    ///Simple message object.
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


impl CreateAppError {
    pub fn from_body(body: &str) -> CreateAppError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        CreateAppError::BadRequest(String::from(error_message))
                    }
                    "ForbiddenException" => CreateAppError::Forbidden(String::from(error_message)),
                    "InternalServerErrorException" => {
                        CreateAppError::InternalServerError(String::from(error_message))
                    }
                    "MethodNotAllowedException" => {
                        CreateAppError::MethodNotAllowed(String::from(error_message))
                    }
                    "NotFoundException" => CreateAppError::NotFound(String::from(error_message)),
                    "TooManyRequestsException" => {
                        CreateAppError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => CreateAppError::Validation(error_message.to_string()),
                    _ => CreateAppError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateAppError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateAppError {
    fn from(err: serde_json::error::Error) -> CreateAppError {
        CreateAppError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateAppError {
    fn from(err: CredentialsError) -> CreateAppError {
        CreateAppError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateAppError {
    fn from(err: HttpDispatchError) -> CreateAppError {
        CreateAppError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateAppError {
    fn from(err: io::Error) -> CreateAppError {
        CreateAppError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateAppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateAppError {
    fn description(&self) -> &str {
        match *self {
            CreateAppError::BadRequest(ref cause) => cause,
            CreateAppError::Forbidden(ref cause) => cause,
            CreateAppError::InternalServerError(ref cause) => cause,
            CreateAppError::MethodNotAllowed(ref cause) => cause,
            CreateAppError::NotFound(ref cause) => cause,
            CreateAppError::TooManyRequests(ref cause) => cause,
            CreateAppError::Validation(ref cause) => cause,
            CreateAppError::Credentials(ref err) => err.description(),
            CreateAppError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateAppError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateCampaign
#[derive(Debug, PartialEq)]
pub enum CreateCampaignError {
    ///Simple message object.
    BadRequest(String),
    ///Simple message object.
    Forbidden(String),
    ///Simple message object.
    InternalServerError(String),
    ///Simple message object.
    MethodNotAllowed(String),
    ///Simple message object.
    NotFound(String),
    ///Simple message object.
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


impl CreateCampaignError {
    pub fn from_body(body: &str) -> CreateCampaignError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        CreateCampaignError::BadRequest(String::from(error_message))
                    }
                    "ForbiddenException" => {
                        CreateCampaignError::Forbidden(String::from(error_message))
                    }
                    "InternalServerErrorException" => {
                        CreateCampaignError::InternalServerError(String::from(error_message))
                    }
                    "MethodNotAllowedException" => {
                        CreateCampaignError::MethodNotAllowed(String::from(error_message))
                    }
                    "NotFoundException" => {
                        CreateCampaignError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        CreateCampaignError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateCampaignError::Validation(error_message.to_string())
                    }
                    _ => CreateCampaignError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateCampaignError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateCampaignError {
    fn from(err: serde_json::error::Error) -> CreateCampaignError {
        CreateCampaignError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateCampaignError {
    fn from(err: CredentialsError) -> CreateCampaignError {
        CreateCampaignError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateCampaignError {
    fn from(err: HttpDispatchError) -> CreateCampaignError {
        CreateCampaignError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateCampaignError {
    fn from(err: io::Error) -> CreateCampaignError {
        CreateCampaignError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateCampaignError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateCampaignError {
    fn description(&self) -> &str {
        match *self {
            CreateCampaignError::BadRequest(ref cause) => cause,
            CreateCampaignError::Forbidden(ref cause) => cause,
            CreateCampaignError::InternalServerError(ref cause) => cause,
            CreateCampaignError::MethodNotAllowed(ref cause) => cause,
            CreateCampaignError::NotFound(ref cause) => cause,
            CreateCampaignError::TooManyRequests(ref cause) => cause,
            CreateCampaignError::Validation(ref cause) => cause,
            CreateCampaignError::Credentials(ref err) => err.description(),
            CreateCampaignError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateCampaignError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateImportJob
#[derive(Debug, PartialEq)]
pub enum CreateImportJobError {
    ///Simple message object.
    BadRequest(String),
    ///Simple message object.
    Forbidden(String),
    ///Simple message object.
    InternalServerError(String),
    ///Simple message object.
    MethodNotAllowed(String),
    ///Simple message object.
    NotFound(String),
    ///Simple message object.
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


impl CreateImportJobError {
    pub fn from_body(body: &str) -> CreateImportJobError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        CreateImportJobError::BadRequest(String::from(error_message))
                    }
                    "ForbiddenException" => {
                        CreateImportJobError::Forbidden(String::from(error_message))
                    }
                    "InternalServerErrorException" => {
                        CreateImportJobError::InternalServerError(String::from(error_message))
                    }
                    "MethodNotAllowedException" => {
                        CreateImportJobError::MethodNotAllowed(String::from(error_message))
                    }
                    "NotFoundException" => {
                        CreateImportJobError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        CreateImportJobError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateImportJobError::Validation(error_message.to_string())
                    }
                    _ => CreateImportJobError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateImportJobError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateImportJobError {
    fn from(err: serde_json::error::Error) -> CreateImportJobError {
        CreateImportJobError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateImportJobError {
    fn from(err: CredentialsError) -> CreateImportJobError {
        CreateImportJobError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateImportJobError {
    fn from(err: HttpDispatchError) -> CreateImportJobError {
        CreateImportJobError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateImportJobError {
    fn from(err: io::Error) -> CreateImportJobError {
        CreateImportJobError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateImportJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateImportJobError {
    fn description(&self) -> &str {
        match *self {
            CreateImportJobError::BadRequest(ref cause) => cause,
            CreateImportJobError::Forbidden(ref cause) => cause,
            CreateImportJobError::InternalServerError(ref cause) => cause,
            CreateImportJobError::MethodNotAllowed(ref cause) => cause,
            CreateImportJobError::NotFound(ref cause) => cause,
            CreateImportJobError::TooManyRequests(ref cause) => cause,
            CreateImportJobError::Validation(ref cause) => cause,
            CreateImportJobError::Credentials(ref err) => err.description(),
            CreateImportJobError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateImportJobError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateSegment
#[derive(Debug, PartialEq)]
pub enum CreateSegmentError {
    ///Simple message object.
    BadRequest(String),
    ///Simple message object.
    Forbidden(String),
    ///Simple message object.
    InternalServerError(String),
    ///Simple message object.
    MethodNotAllowed(String),
    ///Simple message object.
    NotFound(String),
    ///Simple message object.
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


impl CreateSegmentError {
    pub fn from_body(body: &str) -> CreateSegmentError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        CreateSegmentError::BadRequest(String::from(error_message))
                    }
                    "ForbiddenException" => {
                        CreateSegmentError::Forbidden(String::from(error_message))
                    }
                    "InternalServerErrorException" => {
                        CreateSegmentError::InternalServerError(String::from(error_message))
                    }
                    "MethodNotAllowedException" => {
                        CreateSegmentError::MethodNotAllowed(String::from(error_message))
                    }
                    "NotFoundException" => {
                        CreateSegmentError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        CreateSegmentError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateSegmentError::Validation(error_message.to_string())
                    }
                    _ => CreateSegmentError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateSegmentError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateSegmentError {
    fn from(err: serde_json::error::Error) -> CreateSegmentError {
        CreateSegmentError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateSegmentError {
    fn from(err: CredentialsError) -> CreateSegmentError {
        CreateSegmentError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateSegmentError {
    fn from(err: HttpDispatchError) -> CreateSegmentError {
        CreateSegmentError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateSegmentError {
    fn from(err: io::Error) -> CreateSegmentError {
        CreateSegmentError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateSegmentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateSegmentError {
    fn description(&self) -> &str {
        match *self {
            CreateSegmentError::BadRequest(ref cause) => cause,
            CreateSegmentError::Forbidden(ref cause) => cause,
            CreateSegmentError::InternalServerError(ref cause) => cause,
            CreateSegmentError::MethodNotAllowed(ref cause) => cause,
            CreateSegmentError::NotFound(ref cause) => cause,
            CreateSegmentError::TooManyRequests(ref cause) => cause,
            CreateSegmentError::Validation(ref cause) => cause,
            CreateSegmentError::Credentials(ref err) => err.description(),
            CreateSegmentError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateSegmentError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteApnsChannel
#[derive(Debug, PartialEq)]
pub enum DeleteApnsChannelError {
    ///Simple message object.
    BadRequest(String),
    ///Simple message object.
    Forbidden(String),
    ///Simple message object.
    InternalServerError(String),
    ///Simple message object.
    MethodNotAllowed(String),
    ///Simple message object.
    NotFound(String),
    ///Simple message object.
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


impl DeleteApnsChannelError {
    pub fn from_body(body: &str) -> DeleteApnsChannelError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        DeleteApnsChannelError::BadRequest(String::from(error_message))
                    }
                    "ForbiddenException" => {
                        DeleteApnsChannelError::Forbidden(String::from(error_message))
                    }
                    "InternalServerErrorException" => {
                        DeleteApnsChannelError::InternalServerError(String::from(error_message))
                    }
                    "MethodNotAllowedException" => {
                        DeleteApnsChannelError::MethodNotAllowed(String::from(error_message))
                    }
                    "NotFoundException" => {
                        DeleteApnsChannelError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        DeleteApnsChannelError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteApnsChannelError::Validation(error_message.to_string())
                    }
                    _ => DeleteApnsChannelError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteApnsChannelError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteApnsChannelError {
    fn from(err: serde_json::error::Error) -> DeleteApnsChannelError {
        DeleteApnsChannelError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteApnsChannelError {
    fn from(err: CredentialsError) -> DeleteApnsChannelError {
        DeleteApnsChannelError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteApnsChannelError {
    fn from(err: HttpDispatchError) -> DeleteApnsChannelError {
        DeleteApnsChannelError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteApnsChannelError {
    fn from(err: io::Error) -> DeleteApnsChannelError {
        DeleteApnsChannelError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteApnsChannelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteApnsChannelError {
    fn description(&self) -> &str {
        match *self {
            DeleteApnsChannelError::BadRequest(ref cause) => cause,
            DeleteApnsChannelError::Forbidden(ref cause) => cause,
            DeleteApnsChannelError::InternalServerError(ref cause) => cause,
            DeleteApnsChannelError::MethodNotAllowed(ref cause) => cause,
            DeleteApnsChannelError::NotFound(ref cause) => cause,
            DeleteApnsChannelError::TooManyRequests(ref cause) => cause,
            DeleteApnsChannelError::Validation(ref cause) => cause,
            DeleteApnsChannelError::Credentials(ref err) => err.description(),
            DeleteApnsChannelError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteApnsChannelError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteApnsSandboxChannel
#[derive(Debug, PartialEq)]
pub enum DeleteApnsSandboxChannelError {
    ///Simple message object.
    BadRequest(String),
    ///Simple message object.
    Forbidden(String),
    ///Simple message object.
    InternalServerError(String),
    ///Simple message object.
    MethodNotAllowed(String),
    ///Simple message object.
    NotFound(String),
    ///Simple message object.
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


impl DeleteApnsSandboxChannelError {
    pub fn from_body(body: &str) -> DeleteApnsSandboxChannelError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        DeleteApnsSandboxChannelError::BadRequest(String::from(error_message))
                    }
                    "ForbiddenException" => {
                        DeleteApnsSandboxChannelError::Forbidden(String::from(error_message))
                    }
                    "InternalServerErrorException" => DeleteApnsSandboxChannelError::InternalServerError(String::from(error_message)),
                    "MethodNotAllowedException" => {
                        DeleteApnsSandboxChannelError::MethodNotAllowed(String::from(error_message))
                    }
                    "NotFoundException" => {
                        DeleteApnsSandboxChannelError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        DeleteApnsSandboxChannelError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteApnsSandboxChannelError::Validation(error_message.to_string())
                    }
                    _ => DeleteApnsSandboxChannelError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteApnsSandboxChannelError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteApnsSandboxChannelError {
    fn from(err: serde_json::error::Error) -> DeleteApnsSandboxChannelError {
        DeleteApnsSandboxChannelError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteApnsSandboxChannelError {
    fn from(err: CredentialsError) -> DeleteApnsSandboxChannelError {
        DeleteApnsSandboxChannelError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteApnsSandboxChannelError {
    fn from(err: HttpDispatchError) -> DeleteApnsSandboxChannelError {
        DeleteApnsSandboxChannelError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteApnsSandboxChannelError {
    fn from(err: io::Error) -> DeleteApnsSandboxChannelError {
        DeleteApnsSandboxChannelError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteApnsSandboxChannelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteApnsSandboxChannelError {
    fn description(&self) -> &str {
        match *self {
            DeleteApnsSandboxChannelError::BadRequest(ref cause) => cause,
            DeleteApnsSandboxChannelError::Forbidden(ref cause) => cause,
            DeleteApnsSandboxChannelError::InternalServerError(ref cause) => cause,
            DeleteApnsSandboxChannelError::MethodNotAllowed(ref cause) => cause,
            DeleteApnsSandboxChannelError::NotFound(ref cause) => cause,
            DeleteApnsSandboxChannelError::TooManyRequests(ref cause) => cause,
            DeleteApnsSandboxChannelError::Validation(ref cause) => cause,
            DeleteApnsSandboxChannelError::Credentials(ref err) => err.description(),
            DeleteApnsSandboxChannelError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteApnsSandboxChannelError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteApp
#[derive(Debug, PartialEq)]
pub enum DeleteAppError {
    ///Simple message object.
    BadRequest(String),
    ///Simple message object.
    Forbidden(String),
    ///Simple message object.
    InternalServerError(String),
    ///Simple message object.
    MethodNotAllowed(String),
    ///Simple message object.
    NotFound(String),
    ///Simple message object.
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


impl DeleteAppError {
    pub fn from_body(body: &str) -> DeleteAppError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        DeleteAppError::BadRequest(String::from(error_message))
                    }
                    "ForbiddenException" => DeleteAppError::Forbidden(String::from(error_message)),
                    "InternalServerErrorException" => {
                        DeleteAppError::InternalServerError(String::from(error_message))
                    }
                    "MethodNotAllowedException" => {
                        DeleteAppError::MethodNotAllowed(String::from(error_message))
                    }
                    "NotFoundException" => DeleteAppError::NotFound(String::from(error_message)),
                    "TooManyRequestsException" => {
                        DeleteAppError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => DeleteAppError::Validation(error_message.to_string()),
                    _ => DeleteAppError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteAppError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteAppError {
    fn from(err: serde_json::error::Error) -> DeleteAppError {
        DeleteAppError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteAppError {
    fn from(err: CredentialsError) -> DeleteAppError {
        DeleteAppError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteAppError {
    fn from(err: HttpDispatchError) -> DeleteAppError {
        DeleteAppError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteAppError {
    fn from(err: io::Error) -> DeleteAppError {
        DeleteAppError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteAppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteAppError {
    fn description(&self) -> &str {
        match *self {
            DeleteAppError::BadRequest(ref cause) => cause,
            DeleteAppError::Forbidden(ref cause) => cause,
            DeleteAppError::InternalServerError(ref cause) => cause,
            DeleteAppError::MethodNotAllowed(ref cause) => cause,
            DeleteAppError::NotFound(ref cause) => cause,
            DeleteAppError::TooManyRequests(ref cause) => cause,
            DeleteAppError::Validation(ref cause) => cause,
            DeleteAppError::Credentials(ref err) => err.description(),
            DeleteAppError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteAppError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteCampaign
#[derive(Debug, PartialEq)]
pub enum DeleteCampaignError {
    ///Simple message object.
    BadRequest(String),
    ///Simple message object.
    Forbidden(String),
    ///Simple message object.
    InternalServerError(String),
    ///Simple message object.
    MethodNotAllowed(String),
    ///Simple message object.
    NotFound(String),
    ///Simple message object.
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


impl DeleteCampaignError {
    pub fn from_body(body: &str) -> DeleteCampaignError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        DeleteCampaignError::BadRequest(String::from(error_message))
                    }
                    "ForbiddenException" => {
                        DeleteCampaignError::Forbidden(String::from(error_message))
                    }
                    "InternalServerErrorException" => {
                        DeleteCampaignError::InternalServerError(String::from(error_message))
                    }
                    "MethodNotAllowedException" => {
                        DeleteCampaignError::MethodNotAllowed(String::from(error_message))
                    }
                    "NotFoundException" => {
                        DeleteCampaignError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        DeleteCampaignError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteCampaignError::Validation(error_message.to_string())
                    }
                    _ => DeleteCampaignError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteCampaignError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteCampaignError {
    fn from(err: serde_json::error::Error) -> DeleteCampaignError {
        DeleteCampaignError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteCampaignError {
    fn from(err: CredentialsError) -> DeleteCampaignError {
        DeleteCampaignError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteCampaignError {
    fn from(err: HttpDispatchError) -> DeleteCampaignError {
        DeleteCampaignError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteCampaignError {
    fn from(err: io::Error) -> DeleteCampaignError {
        DeleteCampaignError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteCampaignError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteCampaignError {
    fn description(&self) -> &str {
        match *self {
            DeleteCampaignError::BadRequest(ref cause) => cause,
            DeleteCampaignError::Forbidden(ref cause) => cause,
            DeleteCampaignError::InternalServerError(ref cause) => cause,
            DeleteCampaignError::MethodNotAllowed(ref cause) => cause,
            DeleteCampaignError::NotFound(ref cause) => cause,
            DeleteCampaignError::TooManyRequests(ref cause) => cause,
            DeleteCampaignError::Validation(ref cause) => cause,
            DeleteCampaignError::Credentials(ref err) => err.description(),
            DeleteCampaignError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteCampaignError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteEmailChannel
#[derive(Debug, PartialEq)]
pub enum DeleteEmailChannelError {
    ///Simple message object.
    BadRequest(String),
    ///Simple message object.
    Forbidden(String),
    ///Simple message object.
    InternalServerError(String),
    ///Simple message object.
    MethodNotAllowed(String),
    ///Simple message object.
    NotFound(String),
    ///Simple message object.
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


impl DeleteEmailChannelError {
    pub fn from_body(body: &str) -> DeleteEmailChannelError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        DeleteEmailChannelError::BadRequest(String::from(error_message))
                    }
                    "ForbiddenException" => {
                        DeleteEmailChannelError::Forbidden(String::from(error_message))
                    }
                    "InternalServerErrorException" => {
                        DeleteEmailChannelError::InternalServerError(String::from(error_message))
                    }
                    "MethodNotAllowedException" => {
                        DeleteEmailChannelError::MethodNotAllowed(String::from(error_message))
                    }
                    "NotFoundException" => {
                        DeleteEmailChannelError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        DeleteEmailChannelError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteEmailChannelError::Validation(error_message.to_string())
                    }
                    _ => DeleteEmailChannelError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteEmailChannelError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteEmailChannelError {
    fn from(err: serde_json::error::Error) -> DeleteEmailChannelError {
        DeleteEmailChannelError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteEmailChannelError {
    fn from(err: CredentialsError) -> DeleteEmailChannelError {
        DeleteEmailChannelError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteEmailChannelError {
    fn from(err: HttpDispatchError) -> DeleteEmailChannelError {
        DeleteEmailChannelError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteEmailChannelError {
    fn from(err: io::Error) -> DeleteEmailChannelError {
        DeleteEmailChannelError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteEmailChannelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteEmailChannelError {
    fn description(&self) -> &str {
        match *self {
            DeleteEmailChannelError::BadRequest(ref cause) => cause,
            DeleteEmailChannelError::Forbidden(ref cause) => cause,
            DeleteEmailChannelError::InternalServerError(ref cause) => cause,
            DeleteEmailChannelError::MethodNotAllowed(ref cause) => cause,
            DeleteEmailChannelError::NotFound(ref cause) => cause,
            DeleteEmailChannelError::TooManyRequests(ref cause) => cause,
            DeleteEmailChannelError::Validation(ref cause) => cause,
            DeleteEmailChannelError::Credentials(ref err) => err.description(),
            DeleteEmailChannelError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteEmailChannelError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteEventStream
#[derive(Debug, PartialEq)]
pub enum DeleteEventStreamError {
    ///Simple message object.
    BadRequest(String),
    ///Simple message object.
    Forbidden(String),
    ///Simple message object.
    InternalServerError(String),
    ///Simple message object.
    MethodNotAllowed(String),
    ///Simple message object.
    NotFound(String),
    ///Simple message object.
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


impl DeleteEventStreamError {
    pub fn from_body(body: &str) -> DeleteEventStreamError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        DeleteEventStreamError::BadRequest(String::from(error_message))
                    }
                    "ForbiddenException" => {
                        DeleteEventStreamError::Forbidden(String::from(error_message))
                    }
                    "InternalServerErrorException" => {
                        DeleteEventStreamError::InternalServerError(String::from(error_message))
                    }
                    "MethodNotAllowedException" => {
                        DeleteEventStreamError::MethodNotAllowed(String::from(error_message))
                    }
                    "NotFoundException" => {
                        DeleteEventStreamError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        DeleteEventStreamError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteEventStreamError::Validation(error_message.to_string())
                    }
                    _ => DeleteEventStreamError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteEventStreamError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteEventStreamError {
    fn from(err: serde_json::error::Error) -> DeleteEventStreamError {
        DeleteEventStreamError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteEventStreamError {
    fn from(err: CredentialsError) -> DeleteEventStreamError {
        DeleteEventStreamError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteEventStreamError {
    fn from(err: HttpDispatchError) -> DeleteEventStreamError {
        DeleteEventStreamError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteEventStreamError {
    fn from(err: io::Error) -> DeleteEventStreamError {
        DeleteEventStreamError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteEventStreamError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteEventStreamError {
    fn description(&self) -> &str {
        match *self {
            DeleteEventStreamError::BadRequest(ref cause) => cause,
            DeleteEventStreamError::Forbidden(ref cause) => cause,
            DeleteEventStreamError::InternalServerError(ref cause) => cause,
            DeleteEventStreamError::MethodNotAllowed(ref cause) => cause,
            DeleteEventStreamError::NotFound(ref cause) => cause,
            DeleteEventStreamError::TooManyRequests(ref cause) => cause,
            DeleteEventStreamError::Validation(ref cause) => cause,
            DeleteEventStreamError::Credentials(ref err) => err.description(),
            DeleteEventStreamError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteEventStreamError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteGcmChannel
#[derive(Debug, PartialEq)]
pub enum DeleteGcmChannelError {
    ///Simple message object.
    BadRequest(String),
    ///Simple message object.
    Forbidden(String),
    ///Simple message object.
    InternalServerError(String),
    ///Simple message object.
    MethodNotAllowed(String),
    ///Simple message object.
    NotFound(String),
    ///Simple message object.
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


impl DeleteGcmChannelError {
    pub fn from_body(body: &str) -> DeleteGcmChannelError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        DeleteGcmChannelError::BadRequest(String::from(error_message))
                    }
                    "ForbiddenException" => {
                        DeleteGcmChannelError::Forbidden(String::from(error_message))
                    }
                    "InternalServerErrorException" => {
                        DeleteGcmChannelError::InternalServerError(String::from(error_message))
                    }
                    "MethodNotAllowedException" => {
                        DeleteGcmChannelError::MethodNotAllowed(String::from(error_message))
                    }
                    "NotFoundException" => {
                        DeleteGcmChannelError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        DeleteGcmChannelError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteGcmChannelError::Validation(error_message.to_string())
                    }
                    _ => DeleteGcmChannelError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteGcmChannelError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteGcmChannelError {
    fn from(err: serde_json::error::Error) -> DeleteGcmChannelError {
        DeleteGcmChannelError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteGcmChannelError {
    fn from(err: CredentialsError) -> DeleteGcmChannelError {
        DeleteGcmChannelError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteGcmChannelError {
    fn from(err: HttpDispatchError) -> DeleteGcmChannelError {
        DeleteGcmChannelError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteGcmChannelError {
    fn from(err: io::Error) -> DeleteGcmChannelError {
        DeleteGcmChannelError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteGcmChannelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteGcmChannelError {
    fn description(&self) -> &str {
        match *self {
            DeleteGcmChannelError::BadRequest(ref cause) => cause,
            DeleteGcmChannelError::Forbidden(ref cause) => cause,
            DeleteGcmChannelError::InternalServerError(ref cause) => cause,
            DeleteGcmChannelError::MethodNotAllowed(ref cause) => cause,
            DeleteGcmChannelError::NotFound(ref cause) => cause,
            DeleteGcmChannelError::TooManyRequests(ref cause) => cause,
            DeleteGcmChannelError::Validation(ref cause) => cause,
            DeleteGcmChannelError::Credentials(ref err) => err.description(),
            DeleteGcmChannelError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteGcmChannelError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteSegment
#[derive(Debug, PartialEq)]
pub enum DeleteSegmentError {
    ///Simple message object.
    BadRequest(String),
    ///Simple message object.
    Forbidden(String),
    ///Simple message object.
    InternalServerError(String),
    ///Simple message object.
    MethodNotAllowed(String),
    ///Simple message object.
    NotFound(String),
    ///Simple message object.
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


impl DeleteSegmentError {
    pub fn from_body(body: &str) -> DeleteSegmentError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        DeleteSegmentError::BadRequest(String::from(error_message))
                    }
                    "ForbiddenException" => {
                        DeleteSegmentError::Forbidden(String::from(error_message))
                    }
                    "InternalServerErrorException" => {
                        DeleteSegmentError::InternalServerError(String::from(error_message))
                    }
                    "MethodNotAllowedException" => {
                        DeleteSegmentError::MethodNotAllowed(String::from(error_message))
                    }
                    "NotFoundException" => {
                        DeleteSegmentError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        DeleteSegmentError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteSegmentError::Validation(error_message.to_string())
                    }
                    _ => DeleteSegmentError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteSegmentError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteSegmentError {
    fn from(err: serde_json::error::Error) -> DeleteSegmentError {
        DeleteSegmentError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteSegmentError {
    fn from(err: CredentialsError) -> DeleteSegmentError {
        DeleteSegmentError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteSegmentError {
    fn from(err: HttpDispatchError) -> DeleteSegmentError {
        DeleteSegmentError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteSegmentError {
    fn from(err: io::Error) -> DeleteSegmentError {
        DeleteSegmentError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteSegmentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteSegmentError {
    fn description(&self) -> &str {
        match *self {
            DeleteSegmentError::BadRequest(ref cause) => cause,
            DeleteSegmentError::Forbidden(ref cause) => cause,
            DeleteSegmentError::InternalServerError(ref cause) => cause,
            DeleteSegmentError::MethodNotAllowed(ref cause) => cause,
            DeleteSegmentError::NotFound(ref cause) => cause,
            DeleteSegmentError::TooManyRequests(ref cause) => cause,
            DeleteSegmentError::Validation(ref cause) => cause,
            DeleteSegmentError::Credentials(ref err) => err.description(),
            DeleteSegmentError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteSegmentError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteSmsChannel
#[derive(Debug, PartialEq)]
pub enum DeleteSmsChannelError {
    ///Simple message object.
    BadRequest(String),
    ///Simple message object.
    Forbidden(String),
    ///Simple message object.
    InternalServerError(String),
    ///Simple message object.
    MethodNotAllowed(String),
    ///Simple message object.
    NotFound(String),
    ///Simple message object.
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


impl DeleteSmsChannelError {
    pub fn from_body(body: &str) -> DeleteSmsChannelError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        DeleteSmsChannelError::BadRequest(String::from(error_message))
                    }
                    "ForbiddenException" => {
                        DeleteSmsChannelError::Forbidden(String::from(error_message))
                    }
                    "InternalServerErrorException" => {
                        DeleteSmsChannelError::InternalServerError(String::from(error_message))
                    }
                    "MethodNotAllowedException" => {
                        DeleteSmsChannelError::MethodNotAllowed(String::from(error_message))
                    }
                    "NotFoundException" => {
                        DeleteSmsChannelError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        DeleteSmsChannelError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteSmsChannelError::Validation(error_message.to_string())
                    }
                    _ => DeleteSmsChannelError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteSmsChannelError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteSmsChannelError {
    fn from(err: serde_json::error::Error) -> DeleteSmsChannelError {
        DeleteSmsChannelError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteSmsChannelError {
    fn from(err: CredentialsError) -> DeleteSmsChannelError {
        DeleteSmsChannelError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteSmsChannelError {
    fn from(err: HttpDispatchError) -> DeleteSmsChannelError {
        DeleteSmsChannelError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteSmsChannelError {
    fn from(err: io::Error) -> DeleteSmsChannelError {
        DeleteSmsChannelError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteSmsChannelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteSmsChannelError {
    fn description(&self) -> &str {
        match *self {
            DeleteSmsChannelError::BadRequest(ref cause) => cause,
            DeleteSmsChannelError::Forbidden(ref cause) => cause,
            DeleteSmsChannelError::InternalServerError(ref cause) => cause,
            DeleteSmsChannelError::MethodNotAllowed(ref cause) => cause,
            DeleteSmsChannelError::NotFound(ref cause) => cause,
            DeleteSmsChannelError::TooManyRequests(ref cause) => cause,
            DeleteSmsChannelError::Validation(ref cause) => cause,
            DeleteSmsChannelError::Credentials(ref err) => err.description(),
            DeleteSmsChannelError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteSmsChannelError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetApnsChannel
#[derive(Debug, PartialEq)]
pub enum GetApnsChannelError {
    ///Simple message object.
    BadRequest(String),
    ///Simple message object.
    Forbidden(String),
    ///Simple message object.
    InternalServerError(String),
    ///Simple message object.
    MethodNotAllowed(String),
    ///Simple message object.
    NotFound(String),
    ///Simple message object.
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


impl GetApnsChannelError {
    pub fn from_body(body: &str) -> GetApnsChannelError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        GetApnsChannelError::BadRequest(String::from(error_message))
                    }
                    "ForbiddenException" => {
                        GetApnsChannelError::Forbidden(String::from(error_message))
                    }
                    "InternalServerErrorException" => {
                        GetApnsChannelError::InternalServerError(String::from(error_message))
                    }
                    "MethodNotAllowedException" => {
                        GetApnsChannelError::MethodNotAllowed(String::from(error_message))
                    }
                    "NotFoundException" => {
                        GetApnsChannelError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        GetApnsChannelError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetApnsChannelError::Validation(error_message.to_string())
                    }
                    _ => GetApnsChannelError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetApnsChannelError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetApnsChannelError {
    fn from(err: serde_json::error::Error) -> GetApnsChannelError {
        GetApnsChannelError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetApnsChannelError {
    fn from(err: CredentialsError) -> GetApnsChannelError {
        GetApnsChannelError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetApnsChannelError {
    fn from(err: HttpDispatchError) -> GetApnsChannelError {
        GetApnsChannelError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetApnsChannelError {
    fn from(err: io::Error) -> GetApnsChannelError {
        GetApnsChannelError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetApnsChannelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetApnsChannelError {
    fn description(&self) -> &str {
        match *self {
            GetApnsChannelError::BadRequest(ref cause) => cause,
            GetApnsChannelError::Forbidden(ref cause) => cause,
            GetApnsChannelError::InternalServerError(ref cause) => cause,
            GetApnsChannelError::MethodNotAllowed(ref cause) => cause,
            GetApnsChannelError::NotFound(ref cause) => cause,
            GetApnsChannelError::TooManyRequests(ref cause) => cause,
            GetApnsChannelError::Validation(ref cause) => cause,
            GetApnsChannelError::Credentials(ref err) => err.description(),
            GetApnsChannelError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetApnsChannelError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetApnsSandboxChannel
#[derive(Debug, PartialEq)]
pub enum GetApnsSandboxChannelError {
    ///Simple message object.
    BadRequest(String),
    ///Simple message object.
    Forbidden(String),
    ///Simple message object.
    InternalServerError(String),
    ///Simple message object.
    MethodNotAllowed(String),
    ///Simple message object.
    NotFound(String),
    ///Simple message object.
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


impl GetApnsSandboxChannelError {
    pub fn from_body(body: &str) -> GetApnsSandboxChannelError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        GetApnsSandboxChannelError::BadRequest(String::from(error_message))
                    }
                    "ForbiddenException" => {
                        GetApnsSandboxChannelError::Forbidden(String::from(error_message))
                    }
                    "InternalServerErrorException" => {
                        GetApnsSandboxChannelError::InternalServerError(String::from(error_message))
                    }
                    "MethodNotAllowedException" => {
                        GetApnsSandboxChannelError::MethodNotAllowed(String::from(error_message))
                    }
                    "NotFoundException" => {
                        GetApnsSandboxChannelError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        GetApnsSandboxChannelError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetApnsSandboxChannelError::Validation(error_message.to_string())
                    }
                    _ => GetApnsSandboxChannelError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetApnsSandboxChannelError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetApnsSandboxChannelError {
    fn from(err: serde_json::error::Error) -> GetApnsSandboxChannelError {
        GetApnsSandboxChannelError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetApnsSandboxChannelError {
    fn from(err: CredentialsError) -> GetApnsSandboxChannelError {
        GetApnsSandboxChannelError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetApnsSandboxChannelError {
    fn from(err: HttpDispatchError) -> GetApnsSandboxChannelError {
        GetApnsSandboxChannelError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetApnsSandboxChannelError {
    fn from(err: io::Error) -> GetApnsSandboxChannelError {
        GetApnsSandboxChannelError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetApnsSandboxChannelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetApnsSandboxChannelError {
    fn description(&self) -> &str {
        match *self {
            GetApnsSandboxChannelError::BadRequest(ref cause) => cause,
            GetApnsSandboxChannelError::Forbidden(ref cause) => cause,
            GetApnsSandboxChannelError::InternalServerError(ref cause) => cause,
            GetApnsSandboxChannelError::MethodNotAllowed(ref cause) => cause,
            GetApnsSandboxChannelError::NotFound(ref cause) => cause,
            GetApnsSandboxChannelError::TooManyRequests(ref cause) => cause,
            GetApnsSandboxChannelError::Validation(ref cause) => cause,
            GetApnsSandboxChannelError::Credentials(ref err) => err.description(),
            GetApnsSandboxChannelError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetApnsSandboxChannelError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetApp
#[derive(Debug, PartialEq)]
pub enum GetAppError {
    ///Simple message object.
    BadRequest(String),
    ///Simple message object.
    Forbidden(String),
    ///Simple message object.
    InternalServerError(String),
    ///Simple message object.
    MethodNotAllowed(String),
    ///Simple message object.
    NotFound(String),
    ///Simple message object.
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


impl GetAppError {
    pub fn from_body(body: &str) -> GetAppError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => GetAppError::BadRequest(String::from(error_message)),
                    "ForbiddenException" => GetAppError::Forbidden(String::from(error_message)),
                    "InternalServerErrorException" => {
                        GetAppError::InternalServerError(String::from(error_message))
                    }
                    "MethodNotAllowedException" => {
                        GetAppError::MethodNotAllowed(String::from(error_message))
                    }
                    "NotFoundException" => GetAppError::NotFound(String::from(error_message)),
                    "TooManyRequestsException" => {
                        GetAppError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => GetAppError::Validation(error_message.to_string()),
                    _ => GetAppError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetAppError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetAppError {
    fn from(err: serde_json::error::Error) -> GetAppError {
        GetAppError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetAppError {
    fn from(err: CredentialsError) -> GetAppError {
        GetAppError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetAppError {
    fn from(err: HttpDispatchError) -> GetAppError {
        GetAppError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetAppError {
    fn from(err: io::Error) -> GetAppError {
        GetAppError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetAppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetAppError {
    fn description(&self) -> &str {
        match *self {
            GetAppError::BadRequest(ref cause) => cause,
            GetAppError::Forbidden(ref cause) => cause,
            GetAppError::InternalServerError(ref cause) => cause,
            GetAppError::MethodNotAllowed(ref cause) => cause,
            GetAppError::NotFound(ref cause) => cause,
            GetAppError::TooManyRequests(ref cause) => cause,
            GetAppError::Validation(ref cause) => cause,
            GetAppError::Credentials(ref err) => err.description(),
            GetAppError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetAppError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetApplicationSettings
#[derive(Debug, PartialEq)]
pub enum GetApplicationSettingsError {
    ///Simple message object.
    BadRequest(String),
    ///Simple message object.
    Forbidden(String),
    ///Simple message object.
    InternalServerError(String),
    ///Simple message object.
    MethodNotAllowed(String),
    ///Simple message object.
    NotFound(String),
    ///Simple message object.
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


impl GetApplicationSettingsError {
    pub fn from_body(body: &str) -> GetApplicationSettingsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        GetApplicationSettingsError::BadRequest(String::from(error_message))
                    }
                    "ForbiddenException" => {
                        GetApplicationSettingsError::Forbidden(String::from(error_message))
                    }
                    "InternalServerErrorException" => GetApplicationSettingsError::InternalServerError(String::from(error_message)),
                    "MethodNotAllowedException" => {
                        GetApplicationSettingsError::MethodNotAllowed(String::from(error_message))
                    }
                    "NotFoundException" => {
                        GetApplicationSettingsError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        GetApplicationSettingsError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetApplicationSettingsError::Validation(error_message.to_string())
                    }
                    _ => GetApplicationSettingsError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetApplicationSettingsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetApplicationSettingsError {
    fn from(err: serde_json::error::Error) -> GetApplicationSettingsError {
        GetApplicationSettingsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetApplicationSettingsError {
    fn from(err: CredentialsError) -> GetApplicationSettingsError {
        GetApplicationSettingsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetApplicationSettingsError {
    fn from(err: HttpDispatchError) -> GetApplicationSettingsError {
        GetApplicationSettingsError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetApplicationSettingsError {
    fn from(err: io::Error) -> GetApplicationSettingsError {
        GetApplicationSettingsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetApplicationSettingsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetApplicationSettingsError {
    fn description(&self) -> &str {
        match *self {
            GetApplicationSettingsError::BadRequest(ref cause) => cause,
            GetApplicationSettingsError::Forbidden(ref cause) => cause,
            GetApplicationSettingsError::InternalServerError(ref cause) => cause,
            GetApplicationSettingsError::MethodNotAllowed(ref cause) => cause,
            GetApplicationSettingsError::NotFound(ref cause) => cause,
            GetApplicationSettingsError::TooManyRequests(ref cause) => cause,
            GetApplicationSettingsError::Validation(ref cause) => cause,
            GetApplicationSettingsError::Credentials(ref err) => err.description(),
            GetApplicationSettingsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetApplicationSettingsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetApps
#[derive(Debug, PartialEq)]
pub enum GetAppsError {
    ///Simple message object.
    BadRequest(String),
    ///Simple message object.
    Forbidden(String),
    ///Simple message object.
    InternalServerError(String),
    ///Simple message object.
    MethodNotAllowed(String),
    ///Simple message object.
    NotFound(String),
    ///Simple message object.
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


impl GetAppsError {
    pub fn from_body(body: &str) -> GetAppsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => GetAppsError::BadRequest(String::from(error_message)),
                    "ForbiddenException" => GetAppsError::Forbidden(String::from(error_message)),
                    "InternalServerErrorException" => {
                        GetAppsError::InternalServerError(String::from(error_message))
                    }
                    "MethodNotAllowedException" => {
                        GetAppsError::MethodNotAllowed(String::from(error_message))
                    }
                    "NotFoundException" => GetAppsError::NotFound(String::from(error_message)),
                    "TooManyRequestsException" => {
                        GetAppsError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => GetAppsError::Validation(error_message.to_string()),
                    _ => GetAppsError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetAppsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetAppsError {
    fn from(err: serde_json::error::Error) -> GetAppsError {
        GetAppsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetAppsError {
    fn from(err: CredentialsError) -> GetAppsError {
        GetAppsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetAppsError {
    fn from(err: HttpDispatchError) -> GetAppsError {
        GetAppsError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetAppsError {
    fn from(err: io::Error) -> GetAppsError {
        GetAppsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetAppsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetAppsError {
    fn description(&self) -> &str {
        match *self {
            GetAppsError::BadRequest(ref cause) => cause,
            GetAppsError::Forbidden(ref cause) => cause,
            GetAppsError::InternalServerError(ref cause) => cause,
            GetAppsError::MethodNotAllowed(ref cause) => cause,
            GetAppsError::NotFound(ref cause) => cause,
            GetAppsError::TooManyRequests(ref cause) => cause,
            GetAppsError::Validation(ref cause) => cause,
            GetAppsError::Credentials(ref err) => err.description(),
            GetAppsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetAppsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetCampaign
#[derive(Debug, PartialEq)]
pub enum GetCampaignError {
    ///Simple message object.
    BadRequest(String),
    ///Simple message object.
    Forbidden(String),
    ///Simple message object.
    InternalServerError(String),
    ///Simple message object.
    MethodNotAllowed(String),
    ///Simple message object.
    NotFound(String),
    ///Simple message object.
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


impl GetCampaignError {
    pub fn from_body(body: &str) -> GetCampaignError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        GetCampaignError::BadRequest(String::from(error_message))
                    }
                    "ForbiddenException" => {
                        GetCampaignError::Forbidden(String::from(error_message))
                    }
                    "InternalServerErrorException" => {
                        GetCampaignError::InternalServerError(String::from(error_message))
                    }
                    "MethodNotAllowedException" => {
                        GetCampaignError::MethodNotAllowed(String::from(error_message))
                    }
                    "NotFoundException" => GetCampaignError::NotFound(String::from(error_message)),
                    "TooManyRequestsException" => {
                        GetCampaignError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetCampaignError::Validation(error_message.to_string())
                    }
                    _ => GetCampaignError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetCampaignError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetCampaignError {
    fn from(err: serde_json::error::Error) -> GetCampaignError {
        GetCampaignError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetCampaignError {
    fn from(err: CredentialsError) -> GetCampaignError {
        GetCampaignError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetCampaignError {
    fn from(err: HttpDispatchError) -> GetCampaignError {
        GetCampaignError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetCampaignError {
    fn from(err: io::Error) -> GetCampaignError {
        GetCampaignError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetCampaignError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetCampaignError {
    fn description(&self) -> &str {
        match *self {
            GetCampaignError::BadRequest(ref cause) => cause,
            GetCampaignError::Forbidden(ref cause) => cause,
            GetCampaignError::InternalServerError(ref cause) => cause,
            GetCampaignError::MethodNotAllowed(ref cause) => cause,
            GetCampaignError::NotFound(ref cause) => cause,
            GetCampaignError::TooManyRequests(ref cause) => cause,
            GetCampaignError::Validation(ref cause) => cause,
            GetCampaignError::Credentials(ref err) => err.description(),
            GetCampaignError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetCampaignError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetCampaignActivities
#[derive(Debug, PartialEq)]
pub enum GetCampaignActivitiesError {
    ///Simple message object.
    BadRequest(String),
    ///Simple message object.
    Forbidden(String),
    ///Simple message object.
    InternalServerError(String),
    ///Simple message object.
    MethodNotAllowed(String),
    ///Simple message object.
    NotFound(String),
    ///Simple message object.
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


impl GetCampaignActivitiesError {
    pub fn from_body(body: &str) -> GetCampaignActivitiesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        GetCampaignActivitiesError::BadRequest(String::from(error_message))
                    }
                    "ForbiddenException" => {
                        GetCampaignActivitiesError::Forbidden(String::from(error_message))
                    }
                    "InternalServerErrorException" => {
                        GetCampaignActivitiesError::InternalServerError(String::from(error_message))
                    }
                    "MethodNotAllowedException" => {
                        GetCampaignActivitiesError::MethodNotAllowed(String::from(error_message))
                    }
                    "NotFoundException" => {
                        GetCampaignActivitiesError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        GetCampaignActivitiesError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetCampaignActivitiesError::Validation(error_message.to_string())
                    }
                    _ => GetCampaignActivitiesError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetCampaignActivitiesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetCampaignActivitiesError {
    fn from(err: serde_json::error::Error) -> GetCampaignActivitiesError {
        GetCampaignActivitiesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetCampaignActivitiesError {
    fn from(err: CredentialsError) -> GetCampaignActivitiesError {
        GetCampaignActivitiesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetCampaignActivitiesError {
    fn from(err: HttpDispatchError) -> GetCampaignActivitiesError {
        GetCampaignActivitiesError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetCampaignActivitiesError {
    fn from(err: io::Error) -> GetCampaignActivitiesError {
        GetCampaignActivitiesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetCampaignActivitiesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetCampaignActivitiesError {
    fn description(&self) -> &str {
        match *self {
            GetCampaignActivitiesError::BadRequest(ref cause) => cause,
            GetCampaignActivitiesError::Forbidden(ref cause) => cause,
            GetCampaignActivitiesError::InternalServerError(ref cause) => cause,
            GetCampaignActivitiesError::MethodNotAllowed(ref cause) => cause,
            GetCampaignActivitiesError::NotFound(ref cause) => cause,
            GetCampaignActivitiesError::TooManyRequests(ref cause) => cause,
            GetCampaignActivitiesError::Validation(ref cause) => cause,
            GetCampaignActivitiesError::Credentials(ref err) => err.description(),
            GetCampaignActivitiesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetCampaignActivitiesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetCampaignVersion
#[derive(Debug, PartialEq)]
pub enum GetCampaignVersionError {
    ///Simple message object.
    BadRequest(String),
    ///Simple message object.
    Forbidden(String),
    ///Simple message object.
    InternalServerError(String),
    ///Simple message object.
    MethodNotAllowed(String),
    ///Simple message object.
    NotFound(String),
    ///Simple message object.
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


impl GetCampaignVersionError {
    pub fn from_body(body: &str) -> GetCampaignVersionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        GetCampaignVersionError::BadRequest(String::from(error_message))
                    }
                    "ForbiddenException" => {
                        GetCampaignVersionError::Forbidden(String::from(error_message))
                    }
                    "InternalServerErrorException" => {
                        GetCampaignVersionError::InternalServerError(String::from(error_message))
                    }
                    "MethodNotAllowedException" => {
                        GetCampaignVersionError::MethodNotAllowed(String::from(error_message))
                    }
                    "NotFoundException" => {
                        GetCampaignVersionError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        GetCampaignVersionError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetCampaignVersionError::Validation(error_message.to_string())
                    }
                    _ => GetCampaignVersionError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetCampaignVersionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetCampaignVersionError {
    fn from(err: serde_json::error::Error) -> GetCampaignVersionError {
        GetCampaignVersionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetCampaignVersionError {
    fn from(err: CredentialsError) -> GetCampaignVersionError {
        GetCampaignVersionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetCampaignVersionError {
    fn from(err: HttpDispatchError) -> GetCampaignVersionError {
        GetCampaignVersionError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetCampaignVersionError {
    fn from(err: io::Error) -> GetCampaignVersionError {
        GetCampaignVersionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetCampaignVersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetCampaignVersionError {
    fn description(&self) -> &str {
        match *self {
            GetCampaignVersionError::BadRequest(ref cause) => cause,
            GetCampaignVersionError::Forbidden(ref cause) => cause,
            GetCampaignVersionError::InternalServerError(ref cause) => cause,
            GetCampaignVersionError::MethodNotAllowed(ref cause) => cause,
            GetCampaignVersionError::NotFound(ref cause) => cause,
            GetCampaignVersionError::TooManyRequests(ref cause) => cause,
            GetCampaignVersionError::Validation(ref cause) => cause,
            GetCampaignVersionError::Credentials(ref err) => err.description(),
            GetCampaignVersionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetCampaignVersionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetCampaignVersions
#[derive(Debug, PartialEq)]
pub enum GetCampaignVersionsError {
    ///Simple message object.
    BadRequest(String),
    ///Simple message object.
    Forbidden(String),
    ///Simple message object.
    InternalServerError(String),
    ///Simple message object.
    MethodNotAllowed(String),
    ///Simple message object.
    NotFound(String),
    ///Simple message object.
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


impl GetCampaignVersionsError {
    pub fn from_body(body: &str) -> GetCampaignVersionsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        GetCampaignVersionsError::BadRequest(String::from(error_message))
                    }
                    "ForbiddenException" => {
                        GetCampaignVersionsError::Forbidden(String::from(error_message))
                    }
                    "InternalServerErrorException" => {
                        GetCampaignVersionsError::InternalServerError(String::from(error_message))
                    }
                    "MethodNotAllowedException" => {
                        GetCampaignVersionsError::MethodNotAllowed(String::from(error_message))
                    }
                    "NotFoundException" => {
                        GetCampaignVersionsError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        GetCampaignVersionsError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetCampaignVersionsError::Validation(error_message.to_string())
                    }
                    _ => GetCampaignVersionsError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetCampaignVersionsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetCampaignVersionsError {
    fn from(err: serde_json::error::Error) -> GetCampaignVersionsError {
        GetCampaignVersionsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetCampaignVersionsError {
    fn from(err: CredentialsError) -> GetCampaignVersionsError {
        GetCampaignVersionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetCampaignVersionsError {
    fn from(err: HttpDispatchError) -> GetCampaignVersionsError {
        GetCampaignVersionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetCampaignVersionsError {
    fn from(err: io::Error) -> GetCampaignVersionsError {
        GetCampaignVersionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetCampaignVersionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetCampaignVersionsError {
    fn description(&self) -> &str {
        match *self {
            GetCampaignVersionsError::BadRequest(ref cause) => cause,
            GetCampaignVersionsError::Forbidden(ref cause) => cause,
            GetCampaignVersionsError::InternalServerError(ref cause) => cause,
            GetCampaignVersionsError::MethodNotAllowed(ref cause) => cause,
            GetCampaignVersionsError::NotFound(ref cause) => cause,
            GetCampaignVersionsError::TooManyRequests(ref cause) => cause,
            GetCampaignVersionsError::Validation(ref cause) => cause,
            GetCampaignVersionsError::Credentials(ref err) => err.description(),
            GetCampaignVersionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetCampaignVersionsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetCampaigns
#[derive(Debug, PartialEq)]
pub enum GetCampaignsError {
    ///Simple message object.
    BadRequest(String),
    ///Simple message object.
    Forbidden(String),
    ///Simple message object.
    InternalServerError(String),
    ///Simple message object.
    MethodNotAllowed(String),
    ///Simple message object.
    NotFound(String),
    ///Simple message object.
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


impl GetCampaignsError {
    pub fn from_body(body: &str) -> GetCampaignsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        GetCampaignsError::BadRequest(String::from(error_message))
                    }
                    "ForbiddenException" => {
                        GetCampaignsError::Forbidden(String::from(error_message))
                    }
                    "InternalServerErrorException" => {
                        GetCampaignsError::InternalServerError(String::from(error_message))
                    }
                    "MethodNotAllowedException" => {
                        GetCampaignsError::MethodNotAllowed(String::from(error_message))
                    }
                    "NotFoundException" => GetCampaignsError::NotFound(String::from(error_message)),
                    "TooManyRequestsException" => {
                        GetCampaignsError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetCampaignsError::Validation(error_message.to_string())
                    }
                    _ => GetCampaignsError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetCampaignsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetCampaignsError {
    fn from(err: serde_json::error::Error) -> GetCampaignsError {
        GetCampaignsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetCampaignsError {
    fn from(err: CredentialsError) -> GetCampaignsError {
        GetCampaignsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetCampaignsError {
    fn from(err: HttpDispatchError) -> GetCampaignsError {
        GetCampaignsError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetCampaignsError {
    fn from(err: io::Error) -> GetCampaignsError {
        GetCampaignsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetCampaignsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetCampaignsError {
    fn description(&self) -> &str {
        match *self {
            GetCampaignsError::BadRequest(ref cause) => cause,
            GetCampaignsError::Forbidden(ref cause) => cause,
            GetCampaignsError::InternalServerError(ref cause) => cause,
            GetCampaignsError::MethodNotAllowed(ref cause) => cause,
            GetCampaignsError::NotFound(ref cause) => cause,
            GetCampaignsError::TooManyRequests(ref cause) => cause,
            GetCampaignsError::Validation(ref cause) => cause,
            GetCampaignsError::Credentials(ref err) => err.description(),
            GetCampaignsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetCampaignsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetEmailChannel
#[derive(Debug, PartialEq)]
pub enum GetEmailChannelError {
    ///Simple message object.
    BadRequest(String),
    ///Simple message object.
    Forbidden(String),
    ///Simple message object.
    InternalServerError(String),
    ///Simple message object.
    MethodNotAllowed(String),
    ///Simple message object.
    NotFound(String),
    ///Simple message object.
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


impl GetEmailChannelError {
    pub fn from_body(body: &str) -> GetEmailChannelError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        GetEmailChannelError::BadRequest(String::from(error_message))
                    }
                    "ForbiddenException" => {
                        GetEmailChannelError::Forbidden(String::from(error_message))
                    }
                    "InternalServerErrorException" => {
                        GetEmailChannelError::InternalServerError(String::from(error_message))
                    }
                    "MethodNotAllowedException" => {
                        GetEmailChannelError::MethodNotAllowed(String::from(error_message))
                    }
                    "NotFoundException" => {
                        GetEmailChannelError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        GetEmailChannelError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetEmailChannelError::Validation(error_message.to_string())
                    }
                    _ => GetEmailChannelError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetEmailChannelError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetEmailChannelError {
    fn from(err: serde_json::error::Error) -> GetEmailChannelError {
        GetEmailChannelError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetEmailChannelError {
    fn from(err: CredentialsError) -> GetEmailChannelError {
        GetEmailChannelError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetEmailChannelError {
    fn from(err: HttpDispatchError) -> GetEmailChannelError {
        GetEmailChannelError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetEmailChannelError {
    fn from(err: io::Error) -> GetEmailChannelError {
        GetEmailChannelError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetEmailChannelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetEmailChannelError {
    fn description(&self) -> &str {
        match *self {
            GetEmailChannelError::BadRequest(ref cause) => cause,
            GetEmailChannelError::Forbidden(ref cause) => cause,
            GetEmailChannelError::InternalServerError(ref cause) => cause,
            GetEmailChannelError::MethodNotAllowed(ref cause) => cause,
            GetEmailChannelError::NotFound(ref cause) => cause,
            GetEmailChannelError::TooManyRequests(ref cause) => cause,
            GetEmailChannelError::Validation(ref cause) => cause,
            GetEmailChannelError::Credentials(ref err) => err.description(),
            GetEmailChannelError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetEmailChannelError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetEndpoint
#[derive(Debug, PartialEq)]
pub enum GetEndpointError {
    ///Simple message object.
    BadRequest(String),
    ///Simple message object.
    Forbidden(String),
    ///Simple message object.
    InternalServerError(String),
    ///Simple message object.
    MethodNotAllowed(String),
    ///Simple message object.
    NotFound(String),
    ///Simple message object.
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


impl GetEndpointError {
    pub fn from_body(body: &str) -> GetEndpointError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        GetEndpointError::BadRequest(String::from(error_message))
                    }
                    "ForbiddenException" => {
                        GetEndpointError::Forbidden(String::from(error_message))
                    }
                    "InternalServerErrorException" => {
                        GetEndpointError::InternalServerError(String::from(error_message))
                    }
                    "MethodNotAllowedException" => {
                        GetEndpointError::MethodNotAllowed(String::from(error_message))
                    }
                    "NotFoundException" => GetEndpointError::NotFound(String::from(error_message)),
                    "TooManyRequestsException" => {
                        GetEndpointError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetEndpointError::Validation(error_message.to_string())
                    }
                    _ => GetEndpointError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetEndpointError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetEndpointError {
    fn from(err: serde_json::error::Error) -> GetEndpointError {
        GetEndpointError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetEndpointError {
    fn from(err: CredentialsError) -> GetEndpointError {
        GetEndpointError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetEndpointError {
    fn from(err: HttpDispatchError) -> GetEndpointError {
        GetEndpointError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetEndpointError {
    fn from(err: io::Error) -> GetEndpointError {
        GetEndpointError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetEndpointError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetEndpointError {
    fn description(&self) -> &str {
        match *self {
            GetEndpointError::BadRequest(ref cause) => cause,
            GetEndpointError::Forbidden(ref cause) => cause,
            GetEndpointError::InternalServerError(ref cause) => cause,
            GetEndpointError::MethodNotAllowed(ref cause) => cause,
            GetEndpointError::NotFound(ref cause) => cause,
            GetEndpointError::TooManyRequests(ref cause) => cause,
            GetEndpointError::Validation(ref cause) => cause,
            GetEndpointError::Credentials(ref err) => err.description(),
            GetEndpointError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetEndpointError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetEventStream
#[derive(Debug, PartialEq)]
pub enum GetEventStreamError {
    ///Simple message object.
    BadRequest(String),
    ///Simple message object.
    Forbidden(String),
    ///Simple message object.
    InternalServerError(String),
    ///Simple message object.
    MethodNotAllowed(String),
    ///Simple message object.
    NotFound(String),
    ///Simple message object.
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


impl GetEventStreamError {
    pub fn from_body(body: &str) -> GetEventStreamError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        GetEventStreamError::BadRequest(String::from(error_message))
                    }
                    "ForbiddenException" => {
                        GetEventStreamError::Forbidden(String::from(error_message))
                    }
                    "InternalServerErrorException" => {
                        GetEventStreamError::InternalServerError(String::from(error_message))
                    }
                    "MethodNotAllowedException" => {
                        GetEventStreamError::MethodNotAllowed(String::from(error_message))
                    }
                    "NotFoundException" => {
                        GetEventStreamError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        GetEventStreamError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetEventStreamError::Validation(error_message.to_string())
                    }
                    _ => GetEventStreamError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetEventStreamError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetEventStreamError {
    fn from(err: serde_json::error::Error) -> GetEventStreamError {
        GetEventStreamError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetEventStreamError {
    fn from(err: CredentialsError) -> GetEventStreamError {
        GetEventStreamError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetEventStreamError {
    fn from(err: HttpDispatchError) -> GetEventStreamError {
        GetEventStreamError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetEventStreamError {
    fn from(err: io::Error) -> GetEventStreamError {
        GetEventStreamError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetEventStreamError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetEventStreamError {
    fn description(&self) -> &str {
        match *self {
            GetEventStreamError::BadRequest(ref cause) => cause,
            GetEventStreamError::Forbidden(ref cause) => cause,
            GetEventStreamError::InternalServerError(ref cause) => cause,
            GetEventStreamError::MethodNotAllowed(ref cause) => cause,
            GetEventStreamError::NotFound(ref cause) => cause,
            GetEventStreamError::TooManyRequests(ref cause) => cause,
            GetEventStreamError::Validation(ref cause) => cause,
            GetEventStreamError::Credentials(ref err) => err.description(),
            GetEventStreamError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetEventStreamError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetGcmChannel
#[derive(Debug, PartialEq)]
pub enum GetGcmChannelError {
    ///Simple message object.
    BadRequest(String),
    ///Simple message object.
    Forbidden(String),
    ///Simple message object.
    InternalServerError(String),
    ///Simple message object.
    MethodNotAllowed(String),
    ///Simple message object.
    NotFound(String),
    ///Simple message object.
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


impl GetGcmChannelError {
    pub fn from_body(body: &str) -> GetGcmChannelError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        GetGcmChannelError::BadRequest(String::from(error_message))
                    }
                    "ForbiddenException" => {
                        GetGcmChannelError::Forbidden(String::from(error_message))
                    }
                    "InternalServerErrorException" => {
                        GetGcmChannelError::InternalServerError(String::from(error_message))
                    }
                    "MethodNotAllowedException" => {
                        GetGcmChannelError::MethodNotAllowed(String::from(error_message))
                    }
                    "NotFoundException" => {
                        GetGcmChannelError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        GetGcmChannelError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetGcmChannelError::Validation(error_message.to_string())
                    }
                    _ => GetGcmChannelError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetGcmChannelError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetGcmChannelError {
    fn from(err: serde_json::error::Error) -> GetGcmChannelError {
        GetGcmChannelError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetGcmChannelError {
    fn from(err: CredentialsError) -> GetGcmChannelError {
        GetGcmChannelError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetGcmChannelError {
    fn from(err: HttpDispatchError) -> GetGcmChannelError {
        GetGcmChannelError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetGcmChannelError {
    fn from(err: io::Error) -> GetGcmChannelError {
        GetGcmChannelError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetGcmChannelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetGcmChannelError {
    fn description(&self) -> &str {
        match *self {
            GetGcmChannelError::BadRequest(ref cause) => cause,
            GetGcmChannelError::Forbidden(ref cause) => cause,
            GetGcmChannelError::InternalServerError(ref cause) => cause,
            GetGcmChannelError::MethodNotAllowed(ref cause) => cause,
            GetGcmChannelError::NotFound(ref cause) => cause,
            GetGcmChannelError::TooManyRequests(ref cause) => cause,
            GetGcmChannelError::Validation(ref cause) => cause,
            GetGcmChannelError::Credentials(ref err) => err.description(),
            GetGcmChannelError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetGcmChannelError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetImportJob
#[derive(Debug, PartialEq)]
pub enum GetImportJobError {
    ///Simple message object.
    BadRequest(String),
    ///Simple message object.
    Forbidden(String),
    ///Simple message object.
    InternalServerError(String),
    ///Simple message object.
    MethodNotAllowed(String),
    ///Simple message object.
    NotFound(String),
    ///Simple message object.
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


impl GetImportJobError {
    pub fn from_body(body: &str) -> GetImportJobError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        GetImportJobError::BadRequest(String::from(error_message))
                    }
                    "ForbiddenException" => {
                        GetImportJobError::Forbidden(String::from(error_message))
                    }
                    "InternalServerErrorException" => {
                        GetImportJobError::InternalServerError(String::from(error_message))
                    }
                    "MethodNotAllowedException" => {
                        GetImportJobError::MethodNotAllowed(String::from(error_message))
                    }
                    "NotFoundException" => GetImportJobError::NotFound(String::from(error_message)),
                    "TooManyRequestsException" => {
                        GetImportJobError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetImportJobError::Validation(error_message.to_string())
                    }
                    _ => GetImportJobError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetImportJobError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetImportJobError {
    fn from(err: serde_json::error::Error) -> GetImportJobError {
        GetImportJobError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetImportJobError {
    fn from(err: CredentialsError) -> GetImportJobError {
        GetImportJobError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetImportJobError {
    fn from(err: HttpDispatchError) -> GetImportJobError {
        GetImportJobError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetImportJobError {
    fn from(err: io::Error) -> GetImportJobError {
        GetImportJobError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetImportJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetImportJobError {
    fn description(&self) -> &str {
        match *self {
            GetImportJobError::BadRequest(ref cause) => cause,
            GetImportJobError::Forbidden(ref cause) => cause,
            GetImportJobError::InternalServerError(ref cause) => cause,
            GetImportJobError::MethodNotAllowed(ref cause) => cause,
            GetImportJobError::NotFound(ref cause) => cause,
            GetImportJobError::TooManyRequests(ref cause) => cause,
            GetImportJobError::Validation(ref cause) => cause,
            GetImportJobError::Credentials(ref err) => err.description(),
            GetImportJobError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetImportJobError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetImportJobs
#[derive(Debug, PartialEq)]
pub enum GetImportJobsError {
    ///Simple message object.
    BadRequest(String),
    ///Simple message object.
    Forbidden(String),
    ///Simple message object.
    InternalServerError(String),
    ///Simple message object.
    MethodNotAllowed(String),
    ///Simple message object.
    NotFound(String),
    ///Simple message object.
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


impl GetImportJobsError {
    pub fn from_body(body: &str) -> GetImportJobsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        GetImportJobsError::BadRequest(String::from(error_message))
                    }
                    "ForbiddenException" => {
                        GetImportJobsError::Forbidden(String::from(error_message))
                    }
                    "InternalServerErrorException" => {
                        GetImportJobsError::InternalServerError(String::from(error_message))
                    }
                    "MethodNotAllowedException" => {
                        GetImportJobsError::MethodNotAllowed(String::from(error_message))
                    }
                    "NotFoundException" => {
                        GetImportJobsError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        GetImportJobsError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetImportJobsError::Validation(error_message.to_string())
                    }
                    _ => GetImportJobsError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetImportJobsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetImportJobsError {
    fn from(err: serde_json::error::Error) -> GetImportJobsError {
        GetImportJobsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetImportJobsError {
    fn from(err: CredentialsError) -> GetImportJobsError {
        GetImportJobsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetImportJobsError {
    fn from(err: HttpDispatchError) -> GetImportJobsError {
        GetImportJobsError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetImportJobsError {
    fn from(err: io::Error) -> GetImportJobsError {
        GetImportJobsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetImportJobsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetImportJobsError {
    fn description(&self) -> &str {
        match *self {
            GetImportJobsError::BadRequest(ref cause) => cause,
            GetImportJobsError::Forbidden(ref cause) => cause,
            GetImportJobsError::InternalServerError(ref cause) => cause,
            GetImportJobsError::MethodNotAllowed(ref cause) => cause,
            GetImportJobsError::NotFound(ref cause) => cause,
            GetImportJobsError::TooManyRequests(ref cause) => cause,
            GetImportJobsError::Validation(ref cause) => cause,
            GetImportJobsError::Credentials(ref err) => err.description(),
            GetImportJobsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetImportJobsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetSegment
#[derive(Debug, PartialEq)]
pub enum GetSegmentError {
    ///Simple message object.
    BadRequest(String),
    ///Simple message object.
    Forbidden(String),
    ///Simple message object.
    InternalServerError(String),
    ///Simple message object.
    MethodNotAllowed(String),
    ///Simple message object.
    NotFound(String),
    ///Simple message object.
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


impl GetSegmentError {
    pub fn from_body(body: &str) -> GetSegmentError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        GetSegmentError::BadRequest(String::from(error_message))
                    }
                    "ForbiddenException" => GetSegmentError::Forbidden(String::from(error_message)),
                    "InternalServerErrorException" => {
                        GetSegmentError::InternalServerError(String::from(error_message))
                    }
                    "MethodNotAllowedException" => {
                        GetSegmentError::MethodNotAllowed(String::from(error_message))
                    }
                    "NotFoundException" => GetSegmentError::NotFound(String::from(error_message)),
                    "TooManyRequestsException" => {
                        GetSegmentError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => GetSegmentError::Validation(error_message.to_string()),
                    _ => GetSegmentError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetSegmentError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetSegmentError {
    fn from(err: serde_json::error::Error) -> GetSegmentError {
        GetSegmentError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetSegmentError {
    fn from(err: CredentialsError) -> GetSegmentError {
        GetSegmentError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetSegmentError {
    fn from(err: HttpDispatchError) -> GetSegmentError {
        GetSegmentError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetSegmentError {
    fn from(err: io::Error) -> GetSegmentError {
        GetSegmentError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetSegmentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetSegmentError {
    fn description(&self) -> &str {
        match *self {
            GetSegmentError::BadRequest(ref cause) => cause,
            GetSegmentError::Forbidden(ref cause) => cause,
            GetSegmentError::InternalServerError(ref cause) => cause,
            GetSegmentError::MethodNotAllowed(ref cause) => cause,
            GetSegmentError::NotFound(ref cause) => cause,
            GetSegmentError::TooManyRequests(ref cause) => cause,
            GetSegmentError::Validation(ref cause) => cause,
            GetSegmentError::Credentials(ref err) => err.description(),
            GetSegmentError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetSegmentError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetSegmentImportJobs
#[derive(Debug, PartialEq)]
pub enum GetSegmentImportJobsError {
    ///Simple message object.
    BadRequest(String),
    ///Simple message object.
    Forbidden(String),
    ///Simple message object.
    InternalServerError(String),
    ///Simple message object.
    MethodNotAllowed(String),
    ///Simple message object.
    NotFound(String),
    ///Simple message object.
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


impl GetSegmentImportJobsError {
    pub fn from_body(body: &str) -> GetSegmentImportJobsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        GetSegmentImportJobsError::BadRequest(String::from(error_message))
                    }
                    "ForbiddenException" => {
                        GetSegmentImportJobsError::Forbidden(String::from(error_message))
                    }
                    "InternalServerErrorException" => {
                        GetSegmentImportJobsError::InternalServerError(String::from(error_message))
                    }
                    "MethodNotAllowedException" => {
                        GetSegmentImportJobsError::MethodNotAllowed(String::from(error_message))
                    }
                    "NotFoundException" => {
                        GetSegmentImportJobsError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        GetSegmentImportJobsError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetSegmentImportJobsError::Validation(error_message.to_string())
                    }
                    _ => GetSegmentImportJobsError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetSegmentImportJobsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetSegmentImportJobsError {
    fn from(err: serde_json::error::Error) -> GetSegmentImportJobsError {
        GetSegmentImportJobsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetSegmentImportJobsError {
    fn from(err: CredentialsError) -> GetSegmentImportJobsError {
        GetSegmentImportJobsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetSegmentImportJobsError {
    fn from(err: HttpDispatchError) -> GetSegmentImportJobsError {
        GetSegmentImportJobsError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetSegmentImportJobsError {
    fn from(err: io::Error) -> GetSegmentImportJobsError {
        GetSegmentImportJobsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetSegmentImportJobsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetSegmentImportJobsError {
    fn description(&self) -> &str {
        match *self {
            GetSegmentImportJobsError::BadRequest(ref cause) => cause,
            GetSegmentImportJobsError::Forbidden(ref cause) => cause,
            GetSegmentImportJobsError::InternalServerError(ref cause) => cause,
            GetSegmentImportJobsError::MethodNotAllowed(ref cause) => cause,
            GetSegmentImportJobsError::NotFound(ref cause) => cause,
            GetSegmentImportJobsError::TooManyRequests(ref cause) => cause,
            GetSegmentImportJobsError::Validation(ref cause) => cause,
            GetSegmentImportJobsError::Credentials(ref err) => err.description(),
            GetSegmentImportJobsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetSegmentImportJobsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetSegmentVersion
#[derive(Debug, PartialEq)]
pub enum GetSegmentVersionError {
    ///Simple message object.
    BadRequest(String),
    ///Simple message object.
    Forbidden(String),
    ///Simple message object.
    InternalServerError(String),
    ///Simple message object.
    MethodNotAllowed(String),
    ///Simple message object.
    NotFound(String),
    ///Simple message object.
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


impl GetSegmentVersionError {
    pub fn from_body(body: &str) -> GetSegmentVersionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        GetSegmentVersionError::BadRequest(String::from(error_message))
                    }
                    "ForbiddenException" => {
                        GetSegmentVersionError::Forbidden(String::from(error_message))
                    }
                    "InternalServerErrorException" => {
                        GetSegmentVersionError::InternalServerError(String::from(error_message))
                    }
                    "MethodNotAllowedException" => {
                        GetSegmentVersionError::MethodNotAllowed(String::from(error_message))
                    }
                    "NotFoundException" => {
                        GetSegmentVersionError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        GetSegmentVersionError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetSegmentVersionError::Validation(error_message.to_string())
                    }
                    _ => GetSegmentVersionError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetSegmentVersionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetSegmentVersionError {
    fn from(err: serde_json::error::Error) -> GetSegmentVersionError {
        GetSegmentVersionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetSegmentVersionError {
    fn from(err: CredentialsError) -> GetSegmentVersionError {
        GetSegmentVersionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetSegmentVersionError {
    fn from(err: HttpDispatchError) -> GetSegmentVersionError {
        GetSegmentVersionError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetSegmentVersionError {
    fn from(err: io::Error) -> GetSegmentVersionError {
        GetSegmentVersionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetSegmentVersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetSegmentVersionError {
    fn description(&self) -> &str {
        match *self {
            GetSegmentVersionError::BadRequest(ref cause) => cause,
            GetSegmentVersionError::Forbidden(ref cause) => cause,
            GetSegmentVersionError::InternalServerError(ref cause) => cause,
            GetSegmentVersionError::MethodNotAllowed(ref cause) => cause,
            GetSegmentVersionError::NotFound(ref cause) => cause,
            GetSegmentVersionError::TooManyRequests(ref cause) => cause,
            GetSegmentVersionError::Validation(ref cause) => cause,
            GetSegmentVersionError::Credentials(ref err) => err.description(),
            GetSegmentVersionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetSegmentVersionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetSegmentVersions
#[derive(Debug, PartialEq)]
pub enum GetSegmentVersionsError {
    ///Simple message object.
    BadRequest(String),
    ///Simple message object.
    Forbidden(String),
    ///Simple message object.
    InternalServerError(String),
    ///Simple message object.
    MethodNotAllowed(String),
    ///Simple message object.
    NotFound(String),
    ///Simple message object.
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


impl GetSegmentVersionsError {
    pub fn from_body(body: &str) -> GetSegmentVersionsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        GetSegmentVersionsError::BadRequest(String::from(error_message))
                    }
                    "ForbiddenException" => {
                        GetSegmentVersionsError::Forbidden(String::from(error_message))
                    }
                    "InternalServerErrorException" => {
                        GetSegmentVersionsError::InternalServerError(String::from(error_message))
                    }
                    "MethodNotAllowedException" => {
                        GetSegmentVersionsError::MethodNotAllowed(String::from(error_message))
                    }
                    "NotFoundException" => {
                        GetSegmentVersionsError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        GetSegmentVersionsError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetSegmentVersionsError::Validation(error_message.to_string())
                    }
                    _ => GetSegmentVersionsError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetSegmentVersionsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetSegmentVersionsError {
    fn from(err: serde_json::error::Error) -> GetSegmentVersionsError {
        GetSegmentVersionsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetSegmentVersionsError {
    fn from(err: CredentialsError) -> GetSegmentVersionsError {
        GetSegmentVersionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetSegmentVersionsError {
    fn from(err: HttpDispatchError) -> GetSegmentVersionsError {
        GetSegmentVersionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetSegmentVersionsError {
    fn from(err: io::Error) -> GetSegmentVersionsError {
        GetSegmentVersionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetSegmentVersionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetSegmentVersionsError {
    fn description(&self) -> &str {
        match *self {
            GetSegmentVersionsError::BadRequest(ref cause) => cause,
            GetSegmentVersionsError::Forbidden(ref cause) => cause,
            GetSegmentVersionsError::InternalServerError(ref cause) => cause,
            GetSegmentVersionsError::MethodNotAllowed(ref cause) => cause,
            GetSegmentVersionsError::NotFound(ref cause) => cause,
            GetSegmentVersionsError::TooManyRequests(ref cause) => cause,
            GetSegmentVersionsError::Validation(ref cause) => cause,
            GetSegmentVersionsError::Credentials(ref err) => err.description(),
            GetSegmentVersionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetSegmentVersionsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetSegments
#[derive(Debug, PartialEq)]
pub enum GetSegmentsError {
    ///Simple message object.
    BadRequest(String),
    ///Simple message object.
    Forbidden(String),
    ///Simple message object.
    InternalServerError(String),
    ///Simple message object.
    MethodNotAllowed(String),
    ///Simple message object.
    NotFound(String),
    ///Simple message object.
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


impl GetSegmentsError {
    pub fn from_body(body: &str) -> GetSegmentsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        GetSegmentsError::BadRequest(String::from(error_message))
                    }
                    "ForbiddenException" => {
                        GetSegmentsError::Forbidden(String::from(error_message))
                    }
                    "InternalServerErrorException" => {
                        GetSegmentsError::InternalServerError(String::from(error_message))
                    }
                    "MethodNotAllowedException" => {
                        GetSegmentsError::MethodNotAllowed(String::from(error_message))
                    }
                    "NotFoundException" => GetSegmentsError::NotFound(String::from(error_message)),
                    "TooManyRequestsException" => {
                        GetSegmentsError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetSegmentsError::Validation(error_message.to_string())
                    }
                    _ => GetSegmentsError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetSegmentsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetSegmentsError {
    fn from(err: serde_json::error::Error) -> GetSegmentsError {
        GetSegmentsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetSegmentsError {
    fn from(err: CredentialsError) -> GetSegmentsError {
        GetSegmentsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetSegmentsError {
    fn from(err: HttpDispatchError) -> GetSegmentsError {
        GetSegmentsError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetSegmentsError {
    fn from(err: io::Error) -> GetSegmentsError {
        GetSegmentsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetSegmentsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetSegmentsError {
    fn description(&self) -> &str {
        match *self {
            GetSegmentsError::BadRequest(ref cause) => cause,
            GetSegmentsError::Forbidden(ref cause) => cause,
            GetSegmentsError::InternalServerError(ref cause) => cause,
            GetSegmentsError::MethodNotAllowed(ref cause) => cause,
            GetSegmentsError::NotFound(ref cause) => cause,
            GetSegmentsError::TooManyRequests(ref cause) => cause,
            GetSegmentsError::Validation(ref cause) => cause,
            GetSegmentsError::Credentials(ref err) => err.description(),
            GetSegmentsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetSegmentsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetSmsChannel
#[derive(Debug, PartialEq)]
pub enum GetSmsChannelError {
    ///Simple message object.
    BadRequest(String),
    ///Simple message object.
    Forbidden(String),
    ///Simple message object.
    InternalServerError(String),
    ///Simple message object.
    MethodNotAllowed(String),
    ///Simple message object.
    NotFound(String),
    ///Simple message object.
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


impl GetSmsChannelError {
    pub fn from_body(body: &str) -> GetSmsChannelError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        GetSmsChannelError::BadRequest(String::from(error_message))
                    }
                    "ForbiddenException" => {
                        GetSmsChannelError::Forbidden(String::from(error_message))
                    }
                    "InternalServerErrorException" => {
                        GetSmsChannelError::InternalServerError(String::from(error_message))
                    }
                    "MethodNotAllowedException" => {
                        GetSmsChannelError::MethodNotAllowed(String::from(error_message))
                    }
                    "NotFoundException" => {
                        GetSmsChannelError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        GetSmsChannelError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetSmsChannelError::Validation(error_message.to_string())
                    }
                    _ => GetSmsChannelError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetSmsChannelError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetSmsChannelError {
    fn from(err: serde_json::error::Error) -> GetSmsChannelError {
        GetSmsChannelError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetSmsChannelError {
    fn from(err: CredentialsError) -> GetSmsChannelError {
        GetSmsChannelError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetSmsChannelError {
    fn from(err: HttpDispatchError) -> GetSmsChannelError {
        GetSmsChannelError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetSmsChannelError {
    fn from(err: io::Error) -> GetSmsChannelError {
        GetSmsChannelError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetSmsChannelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetSmsChannelError {
    fn description(&self) -> &str {
        match *self {
            GetSmsChannelError::BadRequest(ref cause) => cause,
            GetSmsChannelError::Forbidden(ref cause) => cause,
            GetSmsChannelError::InternalServerError(ref cause) => cause,
            GetSmsChannelError::MethodNotAllowed(ref cause) => cause,
            GetSmsChannelError::NotFound(ref cause) => cause,
            GetSmsChannelError::TooManyRequests(ref cause) => cause,
            GetSmsChannelError::Validation(ref cause) => cause,
            GetSmsChannelError::Credentials(ref err) => err.description(),
            GetSmsChannelError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetSmsChannelError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PutEventStream
#[derive(Debug, PartialEq)]
pub enum PutEventStreamError {
    ///Simple message object.
    BadRequest(String),
    ///Simple message object.
    Forbidden(String),
    ///Simple message object.
    InternalServerError(String),
    ///Simple message object.
    MethodNotAllowed(String),
    ///Simple message object.
    NotFound(String),
    ///Simple message object.
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


impl PutEventStreamError {
    pub fn from_body(body: &str) -> PutEventStreamError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        PutEventStreamError::BadRequest(String::from(error_message))
                    }
                    "ForbiddenException" => {
                        PutEventStreamError::Forbidden(String::from(error_message))
                    }
                    "InternalServerErrorException" => {
                        PutEventStreamError::InternalServerError(String::from(error_message))
                    }
                    "MethodNotAllowedException" => {
                        PutEventStreamError::MethodNotAllowed(String::from(error_message))
                    }
                    "NotFoundException" => {
                        PutEventStreamError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        PutEventStreamError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        PutEventStreamError::Validation(error_message.to_string())
                    }
                    _ => PutEventStreamError::Unknown(String::from(body)),
                }
            }
            Err(_) => PutEventStreamError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for PutEventStreamError {
    fn from(err: serde_json::error::Error) -> PutEventStreamError {
        PutEventStreamError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for PutEventStreamError {
    fn from(err: CredentialsError) -> PutEventStreamError {
        PutEventStreamError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutEventStreamError {
    fn from(err: HttpDispatchError) -> PutEventStreamError {
        PutEventStreamError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutEventStreamError {
    fn from(err: io::Error) -> PutEventStreamError {
        PutEventStreamError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutEventStreamError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutEventStreamError {
    fn description(&self) -> &str {
        match *self {
            PutEventStreamError::BadRequest(ref cause) => cause,
            PutEventStreamError::Forbidden(ref cause) => cause,
            PutEventStreamError::InternalServerError(ref cause) => cause,
            PutEventStreamError::MethodNotAllowed(ref cause) => cause,
            PutEventStreamError::NotFound(ref cause) => cause,
            PutEventStreamError::TooManyRequests(ref cause) => cause,
            PutEventStreamError::Validation(ref cause) => cause,
            PutEventStreamError::Credentials(ref err) => err.description(),
            PutEventStreamError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            PutEventStreamError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by SendMessages
#[derive(Debug, PartialEq)]
pub enum SendMessagesError {
    ///Simple message object.
    BadRequest(String),
    ///Simple message object.
    Forbidden(String),
    ///Simple message object.
    InternalServerError(String),
    ///Simple message object.
    MethodNotAllowed(String),
    ///Simple message object.
    NotFound(String),
    ///Simple message object.
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


impl SendMessagesError {
    pub fn from_body(body: &str) -> SendMessagesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        SendMessagesError::BadRequest(String::from(error_message))
                    }
                    "ForbiddenException" => {
                        SendMessagesError::Forbidden(String::from(error_message))
                    }
                    "InternalServerErrorException" => {
                        SendMessagesError::InternalServerError(String::from(error_message))
                    }
                    "MethodNotAllowedException" => {
                        SendMessagesError::MethodNotAllowed(String::from(error_message))
                    }
                    "NotFoundException" => SendMessagesError::NotFound(String::from(error_message)),
                    "TooManyRequestsException" => {
                        SendMessagesError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        SendMessagesError::Validation(error_message.to_string())
                    }
                    _ => SendMessagesError::Unknown(String::from(body)),
                }
            }
            Err(_) => SendMessagesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for SendMessagesError {
    fn from(err: serde_json::error::Error) -> SendMessagesError {
        SendMessagesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for SendMessagesError {
    fn from(err: CredentialsError) -> SendMessagesError {
        SendMessagesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for SendMessagesError {
    fn from(err: HttpDispatchError) -> SendMessagesError {
        SendMessagesError::HttpDispatch(err)
    }
}
impl From<io::Error> for SendMessagesError {
    fn from(err: io::Error) -> SendMessagesError {
        SendMessagesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for SendMessagesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SendMessagesError {
    fn description(&self) -> &str {
        match *self {
            SendMessagesError::BadRequest(ref cause) => cause,
            SendMessagesError::Forbidden(ref cause) => cause,
            SendMessagesError::InternalServerError(ref cause) => cause,
            SendMessagesError::MethodNotAllowed(ref cause) => cause,
            SendMessagesError::NotFound(ref cause) => cause,
            SendMessagesError::TooManyRequests(ref cause) => cause,
            SendMessagesError::Validation(ref cause) => cause,
            SendMessagesError::Credentials(ref err) => err.description(),
            SendMessagesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            SendMessagesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateApnsChannel
#[derive(Debug, PartialEq)]
pub enum UpdateApnsChannelError {
    ///Simple message object.
    BadRequest(String),
    ///Simple message object.
    Forbidden(String),
    ///Simple message object.
    InternalServerError(String),
    ///Simple message object.
    MethodNotAllowed(String),
    ///Simple message object.
    NotFound(String),
    ///Simple message object.
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


impl UpdateApnsChannelError {
    pub fn from_body(body: &str) -> UpdateApnsChannelError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        UpdateApnsChannelError::BadRequest(String::from(error_message))
                    }
                    "ForbiddenException" => {
                        UpdateApnsChannelError::Forbidden(String::from(error_message))
                    }
                    "InternalServerErrorException" => {
                        UpdateApnsChannelError::InternalServerError(String::from(error_message))
                    }
                    "MethodNotAllowedException" => {
                        UpdateApnsChannelError::MethodNotAllowed(String::from(error_message))
                    }
                    "NotFoundException" => {
                        UpdateApnsChannelError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        UpdateApnsChannelError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateApnsChannelError::Validation(error_message.to_string())
                    }
                    _ => UpdateApnsChannelError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateApnsChannelError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateApnsChannelError {
    fn from(err: serde_json::error::Error) -> UpdateApnsChannelError {
        UpdateApnsChannelError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateApnsChannelError {
    fn from(err: CredentialsError) -> UpdateApnsChannelError {
        UpdateApnsChannelError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateApnsChannelError {
    fn from(err: HttpDispatchError) -> UpdateApnsChannelError {
        UpdateApnsChannelError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateApnsChannelError {
    fn from(err: io::Error) -> UpdateApnsChannelError {
        UpdateApnsChannelError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateApnsChannelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateApnsChannelError {
    fn description(&self) -> &str {
        match *self {
            UpdateApnsChannelError::BadRequest(ref cause) => cause,
            UpdateApnsChannelError::Forbidden(ref cause) => cause,
            UpdateApnsChannelError::InternalServerError(ref cause) => cause,
            UpdateApnsChannelError::MethodNotAllowed(ref cause) => cause,
            UpdateApnsChannelError::NotFound(ref cause) => cause,
            UpdateApnsChannelError::TooManyRequests(ref cause) => cause,
            UpdateApnsChannelError::Validation(ref cause) => cause,
            UpdateApnsChannelError::Credentials(ref err) => err.description(),
            UpdateApnsChannelError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateApnsChannelError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateApnsSandboxChannel
#[derive(Debug, PartialEq)]
pub enum UpdateApnsSandboxChannelError {
    ///Simple message object.
    BadRequest(String),
    ///Simple message object.
    Forbidden(String),
    ///Simple message object.
    InternalServerError(String),
    ///Simple message object.
    MethodNotAllowed(String),
    ///Simple message object.
    NotFound(String),
    ///Simple message object.
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


impl UpdateApnsSandboxChannelError {
    pub fn from_body(body: &str) -> UpdateApnsSandboxChannelError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        UpdateApnsSandboxChannelError::BadRequest(String::from(error_message))
                    }
                    "ForbiddenException" => {
                        UpdateApnsSandboxChannelError::Forbidden(String::from(error_message))
                    }
                    "InternalServerErrorException" => UpdateApnsSandboxChannelError::InternalServerError(String::from(error_message)),
                    "MethodNotAllowedException" => {
                        UpdateApnsSandboxChannelError::MethodNotAllowed(String::from(error_message))
                    }
                    "NotFoundException" => {
                        UpdateApnsSandboxChannelError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        UpdateApnsSandboxChannelError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateApnsSandboxChannelError::Validation(error_message.to_string())
                    }
                    _ => UpdateApnsSandboxChannelError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateApnsSandboxChannelError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateApnsSandboxChannelError {
    fn from(err: serde_json::error::Error) -> UpdateApnsSandboxChannelError {
        UpdateApnsSandboxChannelError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateApnsSandboxChannelError {
    fn from(err: CredentialsError) -> UpdateApnsSandboxChannelError {
        UpdateApnsSandboxChannelError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateApnsSandboxChannelError {
    fn from(err: HttpDispatchError) -> UpdateApnsSandboxChannelError {
        UpdateApnsSandboxChannelError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateApnsSandboxChannelError {
    fn from(err: io::Error) -> UpdateApnsSandboxChannelError {
        UpdateApnsSandboxChannelError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateApnsSandboxChannelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateApnsSandboxChannelError {
    fn description(&self) -> &str {
        match *self {
            UpdateApnsSandboxChannelError::BadRequest(ref cause) => cause,
            UpdateApnsSandboxChannelError::Forbidden(ref cause) => cause,
            UpdateApnsSandboxChannelError::InternalServerError(ref cause) => cause,
            UpdateApnsSandboxChannelError::MethodNotAllowed(ref cause) => cause,
            UpdateApnsSandboxChannelError::NotFound(ref cause) => cause,
            UpdateApnsSandboxChannelError::TooManyRequests(ref cause) => cause,
            UpdateApnsSandboxChannelError::Validation(ref cause) => cause,
            UpdateApnsSandboxChannelError::Credentials(ref err) => err.description(),
            UpdateApnsSandboxChannelError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateApnsSandboxChannelError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateApplicationSettings
#[derive(Debug, PartialEq)]
pub enum UpdateApplicationSettingsError {
    ///Simple message object.
    BadRequest(String),
    ///Simple message object.
    Forbidden(String),
    ///Simple message object.
    InternalServerError(String),
    ///Simple message object.
    MethodNotAllowed(String),
    ///Simple message object.
    NotFound(String),
    ///Simple message object.
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


impl UpdateApplicationSettingsError {
    pub fn from_body(body: &str) -> UpdateApplicationSettingsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        UpdateApplicationSettingsError::BadRequest(String::from(error_message))
                    }
                    "ForbiddenException" => {
                        UpdateApplicationSettingsError::Forbidden(String::from(error_message))
                    }
                    "InternalServerErrorException" => UpdateApplicationSettingsError::InternalServerError(String::from(error_message)),
                    "MethodNotAllowedException" => UpdateApplicationSettingsError::MethodNotAllowed(String::from(error_message)),
                    "NotFoundException" => {
                        UpdateApplicationSettingsError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        UpdateApplicationSettingsError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateApplicationSettingsError::Validation(error_message.to_string())
                    }
                    _ => UpdateApplicationSettingsError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateApplicationSettingsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateApplicationSettingsError {
    fn from(err: serde_json::error::Error) -> UpdateApplicationSettingsError {
        UpdateApplicationSettingsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateApplicationSettingsError {
    fn from(err: CredentialsError) -> UpdateApplicationSettingsError {
        UpdateApplicationSettingsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateApplicationSettingsError {
    fn from(err: HttpDispatchError) -> UpdateApplicationSettingsError {
        UpdateApplicationSettingsError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateApplicationSettingsError {
    fn from(err: io::Error) -> UpdateApplicationSettingsError {
        UpdateApplicationSettingsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateApplicationSettingsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateApplicationSettingsError {
    fn description(&self) -> &str {
        match *self {
            UpdateApplicationSettingsError::BadRequest(ref cause) => cause,
            UpdateApplicationSettingsError::Forbidden(ref cause) => cause,
            UpdateApplicationSettingsError::InternalServerError(ref cause) => cause,
            UpdateApplicationSettingsError::MethodNotAllowed(ref cause) => cause,
            UpdateApplicationSettingsError::NotFound(ref cause) => cause,
            UpdateApplicationSettingsError::TooManyRequests(ref cause) => cause,
            UpdateApplicationSettingsError::Validation(ref cause) => cause,
            UpdateApplicationSettingsError::Credentials(ref err) => err.description(),
            UpdateApplicationSettingsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateApplicationSettingsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateCampaign
#[derive(Debug, PartialEq)]
pub enum UpdateCampaignError {
    ///Simple message object.
    BadRequest(String),
    ///Simple message object.
    Forbidden(String),
    ///Simple message object.
    InternalServerError(String),
    ///Simple message object.
    MethodNotAllowed(String),
    ///Simple message object.
    NotFound(String),
    ///Simple message object.
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


impl UpdateCampaignError {
    pub fn from_body(body: &str) -> UpdateCampaignError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        UpdateCampaignError::BadRequest(String::from(error_message))
                    }
                    "ForbiddenException" => {
                        UpdateCampaignError::Forbidden(String::from(error_message))
                    }
                    "InternalServerErrorException" => {
                        UpdateCampaignError::InternalServerError(String::from(error_message))
                    }
                    "MethodNotAllowedException" => {
                        UpdateCampaignError::MethodNotAllowed(String::from(error_message))
                    }
                    "NotFoundException" => {
                        UpdateCampaignError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        UpdateCampaignError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateCampaignError::Validation(error_message.to_string())
                    }
                    _ => UpdateCampaignError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateCampaignError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateCampaignError {
    fn from(err: serde_json::error::Error) -> UpdateCampaignError {
        UpdateCampaignError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateCampaignError {
    fn from(err: CredentialsError) -> UpdateCampaignError {
        UpdateCampaignError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateCampaignError {
    fn from(err: HttpDispatchError) -> UpdateCampaignError {
        UpdateCampaignError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateCampaignError {
    fn from(err: io::Error) -> UpdateCampaignError {
        UpdateCampaignError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateCampaignError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateCampaignError {
    fn description(&self) -> &str {
        match *self {
            UpdateCampaignError::BadRequest(ref cause) => cause,
            UpdateCampaignError::Forbidden(ref cause) => cause,
            UpdateCampaignError::InternalServerError(ref cause) => cause,
            UpdateCampaignError::MethodNotAllowed(ref cause) => cause,
            UpdateCampaignError::NotFound(ref cause) => cause,
            UpdateCampaignError::TooManyRequests(ref cause) => cause,
            UpdateCampaignError::Validation(ref cause) => cause,
            UpdateCampaignError::Credentials(ref err) => err.description(),
            UpdateCampaignError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateCampaignError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateEmailChannel
#[derive(Debug, PartialEq)]
pub enum UpdateEmailChannelError {
    ///Simple message object.
    BadRequest(String),
    ///Simple message object.
    Forbidden(String),
    ///Simple message object.
    InternalServerError(String),
    ///Simple message object.
    MethodNotAllowed(String),
    ///Simple message object.
    NotFound(String),
    ///Simple message object.
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


impl UpdateEmailChannelError {
    pub fn from_body(body: &str) -> UpdateEmailChannelError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        UpdateEmailChannelError::BadRequest(String::from(error_message))
                    }
                    "ForbiddenException" => {
                        UpdateEmailChannelError::Forbidden(String::from(error_message))
                    }
                    "InternalServerErrorException" => {
                        UpdateEmailChannelError::InternalServerError(String::from(error_message))
                    }
                    "MethodNotAllowedException" => {
                        UpdateEmailChannelError::MethodNotAllowed(String::from(error_message))
                    }
                    "NotFoundException" => {
                        UpdateEmailChannelError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        UpdateEmailChannelError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateEmailChannelError::Validation(error_message.to_string())
                    }
                    _ => UpdateEmailChannelError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateEmailChannelError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateEmailChannelError {
    fn from(err: serde_json::error::Error) -> UpdateEmailChannelError {
        UpdateEmailChannelError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateEmailChannelError {
    fn from(err: CredentialsError) -> UpdateEmailChannelError {
        UpdateEmailChannelError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateEmailChannelError {
    fn from(err: HttpDispatchError) -> UpdateEmailChannelError {
        UpdateEmailChannelError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateEmailChannelError {
    fn from(err: io::Error) -> UpdateEmailChannelError {
        UpdateEmailChannelError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateEmailChannelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateEmailChannelError {
    fn description(&self) -> &str {
        match *self {
            UpdateEmailChannelError::BadRequest(ref cause) => cause,
            UpdateEmailChannelError::Forbidden(ref cause) => cause,
            UpdateEmailChannelError::InternalServerError(ref cause) => cause,
            UpdateEmailChannelError::MethodNotAllowed(ref cause) => cause,
            UpdateEmailChannelError::NotFound(ref cause) => cause,
            UpdateEmailChannelError::TooManyRequests(ref cause) => cause,
            UpdateEmailChannelError::Validation(ref cause) => cause,
            UpdateEmailChannelError::Credentials(ref err) => err.description(),
            UpdateEmailChannelError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateEmailChannelError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateEndpoint
#[derive(Debug, PartialEq)]
pub enum UpdateEndpointError {
    ///Simple message object.
    BadRequest(String),
    ///Simple message object.
    Forbidden(String),
    ///Simple message object.
    InternalServerError(String),
    ///Simple message object.
    MethodNotAllowed(String),
    ///Simple message object.
    NotFound(String),
    ///Simple message object.
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


impl UpdateEndpointError {
    pub fn from_body(body: &str) -> UpdateEndpointError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        UpdateEndpointError::BadRequest(String::from(error_message))
                    }
                    "ForbiddenException" => {
                        UpdateEndpointError::Forbidden(String::from(error_message))
                    }
                    "InternalServerErrorException" => {
                        UpdateEndpointError::InternalServerError(String::from(error_message))
                    }
                    "MethodNotAllowedException" => {
                        UpdateEndpointError::MethodNotAllowed(String::from(error_message))
                    }
                    "NotFoundException" => {
                        UpdateEndpointError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        UpdateEndpointError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateEndpointError::Validation(error_message.to_string())
                    }
                    _ => UpdateEndpointError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateEndpointError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateEndpointError {
    fn from(err: serde_json::error::Error) -> UpdateEndpointError {
        UpdateEndpointError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateEndpointError {
    fn from(err: CredentialsError) -> UpdateEndpointError {
        UpdateEndpointError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateEndpointError {
    fn from(err: HttpDispatchError) -> UpdateEndpointError {
        UpdateEndpointError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateEndpointError {
    fn from(err: io::Error) -> UpdateEndpointError {
        UpdateEndpointError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateEndpointError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateEndpointError {
    fn description(&self) -> &str {
        match *self {
            UpdateEndpointError::BadRequest(ref cause) => cause,
            UpdateEndpointError::Forbidden(ref cause) => cause,
            UpdateEndpointError::InternalServerError(ref cause) => cause,
            UpdateEndpointError::MethodNotAllowed(ref cause) => cause,
            UpdateEndpointError::NotFound(ref cause) => cause,
            UpdateEndpointError::TooManyRequests(ref cause) => cause,
            UpdateEndpointError::Validation(ref cause) => cause,
            UpdateEndpointError::Credentials(ref err) => err.description(),
            UpdateEndpointError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateEndpointError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateEndpointsBatch
#[derive(Debug, PartialEq)]
pub enum UpdateEndpointsBatchError {
    ///Simple message object.
    BadRequest(String),
    ///Simple message object.
    Forbidden(String),
    ///Simple message object.
    InternalServerError(String),
    ///Simple message object.
    MethodNotAllowed(String),
    ///Simple message object.
    NotFound(String),
    ///Simple message object.
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


impl UpdateEndpointsBatchError {
    pub fn from_body(body: &str) -> UpdateEndpointsBatchError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        UpdateEndpointsBatchError::BadRequest(String::from(error_message))
                    }
                    "ForbiddenException" => {
                        UpdateEndpointsBatchError::Forbidden(String::from(error_message))
                    }
                    "InternalServerErrorException" => {
                        UpdateEndpointsBatchError::InternalServerError(String::from(error_message))
                    }
                    "MethodNotAllowedException" => {
                        UpdateEndpointsBatchError::MethodNotAllowed(String::from(error_message))
                    }
                    "NotFoundException" => {
                        UpdateEndpointsBatchError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        UpdateEndpointsBatchError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateEndpointsBatchError::Validation(error_message.to_string())
                    }
                    _ => UpdateEndpointsBatchError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateEndpointsBatchError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateEndpointsBatchError {
    fn from(err: serde_json::error::Error) -> UpdateEndpointsBatchError {
        UpdateEndpointsBatchError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateEndpointsBatchError {
    fn from(err: CredentialsError) -> UpdateEndpointsBatchError {
        UpdateEndpointsBatchError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateEndpointsBatchError {
    fn from(err: HttpDispatchError) -> UpdateEndpointsBatchError {
        UpdateEndpointsBatchError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateEndpointsBatchError {
    fn from(err: io::Error) -> UpdateEndpointsBatchError {
        UpdateEndpointsBatchError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateEndpointsBatchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateEndpointsBatchError {
    fn description(&self) -> &str {
        match *self {
            UpdateEndpointsBatchError::BadRequest(ref cause) => cause,
            UpdateEndpointsBatchError::Forbidden(ref cause) => cause,
            UpdateEndpointsBatchError::InternalServerError(ref cause) => cause,
            UpdateEndpointsBatchError::MethodNotAllowed(ref cause) => cause,
            UpdateEndpointsBatchError::NotFound(ref cause) => cause,
            UpdateEndpointsBatchError::TooManyRequests(ref cause) => cause,
            UpdateEndpointsBatchError::Validation(ref cause) => cause,
            UpdateEndpointsBatchError::Credentials(ref err) => err.description(),
            UpdateEndpointsBatchError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateEndpointsBatchError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateGcmChannel
#[derive(Debug, PartialEq)]
pub enum UpdateGcmChannelError {
    ///Simple message object.
    BadRequest(String),
    ///Simple message object.
    Forbidden(String),
    ///Simple message object.
    InternalServerError(String),
    ///Simple message object.
    MethodNotAllowed(String),
    ///Simple message object.
    NotFound(String),
    ///Simple message object.
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


impl UpdateGcmChannelError {
    pub fn from_body(body: &str) -> UpdateGcmChannelError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        UpdateGcmChannelError::BadRequest(String::from(error_message))
                    }
                    "ForbiddenException" => {
                        UpdateGcmChannelError::Forbidden(String::from(error_message))
                    }
                    "InternalServerErrorException" => {
                        UpdateGcmChannelError::InternalServerError(String::from(error_message))
                    }
                    "MethodNotAllowedException" => {
                        UpdateGcmChannelError::MethodNotAllowed(String::from(error_message))
                    }
                    "NotFoundException" => {
                        UpdateGcmChannelError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        UpdateGcmChannelError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateGcmChannelError::Validation(error_message.to_string())
                    }
                    _ => UpdateGcmChannelError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateGcmChannelError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateGcmChannelError {
    fn from(err: serde_json::error::Error) -> UpdateGcmChannelError {
        UpdateGcmChannelError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateGcmChannelError {
    fn from(err: CredentialsError) -> UpdateGcmChannelError {
        UpdateGcmChannelError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateGcmChannelError {
    fn from(err: HttpDispatchError) -> UpdateGcmChannelError {
        UpdateGcmChannelError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateGcmChannelError {
    fn from(err: io::Error) -> UpdateGcmChannelError {
        UpdateGcmChannelError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateGcmChannelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateGcmChannelError {
    fn description(&self) -> &str {
        match *self {
            UpdateGcmChannelError::BadRequest(ref cause) => cause,
            UpdateGcmChannelError::Forbidden(ref cause) => cause,
            UpdateGcmChannelError::InternalServerError(ref cause) => cause,
            UpdateGcmChannelError::MethodNotAllowed(ref cause) => cause,
            UpdateGcmChannelError::NotFound(ref cause) => cause,
            UpdateGcmChannelError::TooManyRequests(ref cause) => cause,
            UpdateGcmChannelError::Validation(ref cause) => cause,
            UpdateGcmChannelError::Credentials(ref err) => err.description(),
            UpdateGcmChannelError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateGcmChannelError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateSegment
#[derive(Debug, PartialEq)]
pub enum UpdateSegmentError {
    ///Simple message object.
    BadRequest(String),
    ///Simple message object.
    Forbidden(String),
    ///Simple message object.
    InternalServerError(String),
    ///Simple message object.
    MethodNotAllowed(String),
    ///Simple message object.
    NotFound(String),
    ///Simple message object.
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


impl UpdateSegmentError {
    pub fn from_body(body: &str) -> UpdateSegmentError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        UpdateSegmentError::BadRequest(String::from(error_message))
                    }
                    "ForbiddenException" => {
                        UpdateSegmentError::Forbidden(String::from(error_message))
                    }
                    "InternalServerErrorException" => {
                        UpdateSegmentError::InternalServerError(String::from(error_message))
                    }
                    "MethodNotAllowedException" => {
                        UpdateSegmentError::MethodNotAllowed(String::from(error_message))
                    }
                    "NotFoundException" => {
                        UpdateSegmentError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        UpdateSegmentError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateSegmentError::Validation(error_message.to_string())
                    }
                    _ => UpdateSegmentError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateSegmentError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateSegmentError {
    fn from(err: serde_json::error::Error) -> UpdateSegmentError {
        UpdateSegmentError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateSegmentError {
    fn from(err: CredentialsError) -> UpdateSegmentError {
        UpdateSegmentError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateSegmentError {
    fn from(err: HttpDispatchError) -> UpdateSegmentError {
        UpdateSegmentError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateSegmentError {
    fn from(err: io::Error) -> UpdateSegmentError {
        UpdateSegmentError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateSegmentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateSegmentError {
    fn description(&self) -> &str {
        match *self {
            UpdateSegmentError::BadRequest(ref cause) => cause,
            UpdateSegmentError::Forbidden(ref cause) => cause,
            UpdateSegmentError::InternalServerError(ref cause) => cause,
            UpdateSegmentError::MethodNotAllowed(ref cause) => cause,
            UpdateSegmentError::NotFound(ref cause) => cause,
            UpdateSegmentError::TooManyRequests(ref cause) => cause,
            UpdateSegmentError::Validation(ref cause) => cause,
            UpdateSegmentError::Credentials(ref err) => err.description(),
            UpdateSegmentError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateSegmentError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateSmsChannel
#[derive(Debug, PartialEq)]
pub enum UpdateSmsChannelError {
    ///Simple message object.
    BadRequest(String),
    ///Simple message object.
    Forbidden(String),
    ///Simple message object.
    InternalServerError(String),
    ///Simple message object.
    MethodNotAllowed(String),
    ///Simple message object.
    NotFound(String),
    ///Simple message object.
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


impl UpdateSmsChannelError {
    pub fn from_body(body: &str) -> UpdateSmsChannelError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        UpdateSmsChannelError::BadRequest(String::from(error_message))
                    }
                    "ForbiddenException" => {
                        UpdateSmsChannelError::Forbidden(String::from(error_message))
                    }
                    "InternalServerErrorException" => {
                        UpdateSmsChannelError::InternalServerError(String::from(error_message))
                    }
                    "MethodNotAllowedException" => {
                        UpdateSmsChannelError::MethodNotAllowed(String::from(error_message))
                    }
                    "NotFoundException" => {
                        UpdateSmsChannelError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        UpdateSmsChannelError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateSmsChannelError::Validation(error_message.to_string())
                    }
                    _ => UpdateSmsChannelError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateSmsChannelError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateSmsChannelError {
    fn from(err: serde_json::error::Error) -> UpdateSmsChannelError {
        UpdateSmsChannelError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateSmsChannelError {
    fn from(err: CredentialsError) -> UpdateSmsChannelError {
        UpdateSmsChannelError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateSmsChannelError {
    fn from(err: HttpDispatchError) -> UpdateSmsChannelError {
        UpdateSmsChannelError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateSmsChannelError {
    fn from(err: io::Error) -> UpdateSmsChannelError {
        UpdateSmsChannelError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateSmsChannelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateSmsChannelError {
    fn description(&self) -> &str {
        match *self {
            UpdateSmsChannelError::BadRequest(ref cause) => cause,
            UpdateSmsChannelError::Forbidden(ref cause) => cause,
            UpdateSmsChannelError::InternalServerError(ref cause) => cause,
            UpdateSmsChannelError::MethodNotAllowed(ref cause) => cause,
            UpdateSmsChannelError::NotFound(ref cause) => cause,
            UpdateSmsChannelError::TooManyRequests(ref cause) => cause,
            UpdateSmsChannelError::Validation(ref cause) => cause,
            UpdateSmsChannelError::Credentials(ref err) => err.description(),
            UpdateSmsChannelError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateSmsChannelError::Unknown(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the Amazon Pinpoint API. Amazon Pinpoint clients implement this trait.
pub trait Pinpoint {
    #[doc="Used to create an app."]
    fn create_app(&self, input: &CreateAppRequest) -> Result<CreateAppResponse, CreateAppError>;


    #[doc="Creates or updates a campaign."]
    fn create_campaign(&self,
                       input: &CreateCampaignRequest)
                       -> Result<CreateCampaignResponse, CreateCampaignError>;


    #[doc="Creates or updates an import job."]
    fn create_import_job(&self,
                         input: &CreateImportJobRequest)
                         -> Result<CreateImportJobResponse, CreateImportJobError>;


    #[doc="Used to create or update a segment."]
    fn create_segment(&self,
                      input: &CreateSegmentRequest)
                      -> Result<CreateSegmentResponse, CreateSegmentError>;


    #[doc="Deletes the APNs channel for an app."]
    fn delete_apns_channel(&self,
                           input: &DeleteApnsChannelRequest)
                           -> Result<DeleteApnsChannelResponse, DeleteApnsChannelError>;


    #[doc="Delete an APNS sandbox channel"]
    fn delete_apns_sandbox_channel
        (&self,
         input: &DeleteApnsSandboxChannelRequest)
         -> Result<DeleteApnsSandboxChannelResponse, DeleteApnsSandboxChannelError>;


    #[doc="Deletes an app."]
    fn delete_app(&self, input: &DeleteAppRequest) -> Result<DeleteAppResponse, DeleteAppError>;


    #[doc="Deletes a campaign."]
    fn delete_campaign(&self,
                       input: &DeleteCampaignRequest)
                       -> Result<DeleteCampaignResponse, DeleteCampaignError>;


    #[doc="Delete an email channel"]
    fn delete_email_channel(&self,
                            input: &DeleteEmailChannelRequest)
                            -> Result<DeleteEmailChannelResponse, DeleteEmailChannelError>;


    #[doc="Deletes the event stream for an app."]
    fn delete_event_stream(&self,
                           input: &DeleteEventStreamRequest)
                           -> Result<DeleteEventStreamResponse, DeleteEventStreamError>;


    #[doc="Deletes the GCM channel for an app."]
    fn delete_gcm_channel(&self,
                          input: &DeleteGcmChannelRequest)
                          -> Result<DeleteGcmChannelResponse, DeleteGcmChannelError>;


    #[doc="Deletes a segment."]
    fn delete_segment(&self,
                      input: &DeleteSegmentRequest)
                      -> Result<DeleteSegmentResponse, DeleteSegmentError>;


    #[doc="Delete an SMS channel"]
    fn delete_sms_channel(&self,
                          input: &DeleteSmsChannelRequest)
                          -> Result<DeleteSmsChannelResponse, DeleteSmsChannelError>;


    #[doc="Returns information about the APNs channel for an app."]
    fn get_apns_channel(&self,
                        input: &GetApnsChannelRequest)
                        -> Result<GetApnsChannelResponse, GetApnsChannelError>;


    #[doc="Get an APNS sandbox channel"]
    fn get_apns_sandbox_channel
        (&self,
         input: &GetApnsSandboxChannelRequest)
         -> Result<GetApnsSandboxChannelResponse, GetApnsSandboxChannelError>;


    #[doc="Returns information about an app."]
    fn get_app(&self, input: &GetAppRequest) -> Result<GetAppResponse, GetAppError>;


    #[doc="Used to request the settings for an app."]
    fn get_application_settings
        (&self,
         input: &GetApplicationSettingsRequest)
         -> Result<GetApplicationSettingsResponse, GetApplicationSettingsError>;


    #[doc="Returns information about your apps."]
    fn get_apps(&self, input: &GetAppsRequest) -> Result<GetAppsResponse, GetAppsError>;


    #[doc="Returns information about a campaign."]
    fn get_campaign(&self,
                    input: &GetCampaignRequest)
                    -> Result<GetCampaignResponse, GetCampaignError>;


    #[doc="Returns information about the activity performed by a campaign."]
    fn get_campaign_activities
        (&self,
         input: &GetCampaignActivitiesRequest)
         -> Result<GetCampaignActivitiesResponse, GetCampaignActivitiesError>;


    #[doc="Returns information about a specific version of a campaign."]
    fn get_campaign_version(&self,
                            input: &GetCampaignVersionRequest)
                            -> Result<GetCampaignVersionResponse, GetCampaignVersionError>;


    #[doc="Returns information about your campaign versions."]
    fn get_campaign_versions(&self,
                             input: &GetCampaignVersionsRequest)
                             -> Result<GetCampaignVersionsResponse, GetCampaignVersionsError>;


    #[doc="Returns information about your campaigns."]
    fn get_campaigns(&self,
                     input: &GetCampaignsRequest)
                     -> Result<GetCampaignsResponse, GetCampaignsError>;


    #[doc="Get an email channel"]
    fn get_email_channel(&self,
                         input: &GetEmailChannelRequest)
                         -> Result<GetEmailChannelResponse, GetEmailChannelError>;


    #[doc="Returns information about an endpoint."]
    fn get_endpoint(&self,
                    input: &GetEndpointRequest)
                    -> Result<GetEndpointResponse, GetEndpointError>;


    #[doc="Returns the event stream for an app."]
    fn get_event_stream(&self,
                        input: &GetEventStreamRequest)
                        -> Result<GetEventStreamResponse, GetEventStreamError>;


    #[doc="Returns information about the GCM channel for an app."]
    fn get_gcm_channel(&self,
                       input: &GetGcmChannelRequest)
                       -> Result<GetGcmChannelResponse, GetGcmChannelError>;


    #[doc="Returns information about an import job."]
    fn get_import_job(&self,
                      input: &GetImportJobRequest)
                      -> Result<GetImportJobResponse, GetImportJobError>;


    #[doc="Returns information about your import jobs."]
    fn get_import_jobs(&self,
                       input: &GetImportJobsRequest)
                       -> Result<GetImportJobsResponse, GetImportJobsError>;


    #[doc="Returns information about a segment."]
    fn get_segment(&self,
                   input: &GetSegmentRequest)
                   -> Result<GetSegmentResponse, GetSegmentError>;


    #[doc="Returns a list of import jobs for a specific segment."]
    fn get_segment_import_jobs
        (&self,
         input: &GetSegmentImportJobsRequest)
         -> Result<GetSegmentImportJobsResponse, GetSegmentImportJobsError>;


    #[doc="Returns information about a segment version."]
    fn get_segment_version(&self,
                           input: &GetSegmentVersionRequest)
                           -> Result<GetSegmentVersionResponse, GetSegmentVersionError>;


    #[doc="Returns information about your segment versions."]
    fn get_segment_versions(&self,
                            input: &GetSegmentVersionsRequest)
                            -> Result<GetSegmentVersionsResponse, GetSegmentVersionsError>;


    #[doc="Used to get information about your segments."]
    fn get_segments(&self,
                    input: &GetSegmentsRequest)
                    -> Result<GetSegmentsResponse, GetSegmentsError>;


    #[doc="Get an SMS channel"]
    fn get_sms_channel(&self,
                       input: &GetSmsChannelRequest)
                       -> Result<GetSmsChannelResponse, GetSmsChannelError>;


    #[doc="Use to create or update the event stream for an app."]
    fn put_event_stream(&self,
                        input: &PutEventStreamRequest)
                        -> Result<PutEventStreamResponse, PutEventStreamError>;


    #[doc="Send a batch of messages"]
    fn send_messages(&self,
                     input: &SendMessagesRequest)
                     -> Result<SendMessagesResponse, SendMessagesError>;


    #[doc="Use to update the APNs channel for an app."]
    fn update_apns_channel(&self,
                           input: &UpdateApnsChannelRequest)
                           -> Result<UpdateApnsChannelResponse, UpdateApnsChannelError>;


    #[doc="Update an APNS sandbox channel"]
    fn update_apns_sandbox_channel
        (&self,
         input: &UpdateApnsSandboxChannelRequest)
         -> Result<UpdateApnsSandboxChannelResponse, UpdateApnsSandboxChannelError>;


    #[doc="Used to update the settings for an app."]
    fn update_application_settings
        (&self,
         input: &UpdateApplicationSettingsRequest)
         -> Result<UpdateApplicationSettingsResponse, UpdateApplicationSettingsError>;


    #[doc="Use to update a campaign."]
    fn update_campaign(&self,
                       input: &UpdateCampaignRequest)
                       -> Result<UpdateCampaignResponse, UpdateCampaignError>;


    #[doc="Update an email channel"]
    fn update_email_channel(&self,
                            input: &UpdateEmailChannelRequest)
                            -> Result<UpdateEmailChannelResponse, UpdateEmailChannelError>;


    #[doc="Use to update an endpoint."]
    fn update_endpoint(&self,
                       input: &UpdateEndpointRequest)
                       -> Result<UpdateEndpointResponse, UpdateEndpointError>;


    #[doc="Use to update a batch of endpoints."]
    fn update_endpoints_batch
        (&self,
         input: &UpdateEndpointsBatchRequest)
         -> Result<UpdateEndpointsBatchResponse, UpdateEndpointsBatchError>;


    #[doc="Use to update the GCM channel for an app."]
    fn update_gcm_channel(&self,
                          input: &UpdateGcmChannelRequest)
                          -> Result<UpdateGcmChannelResponse, UpdateGcmChannelError>;


    #[doc="Use to update a segment."]
    fn update_segment(&self,
                      input: &UpdateSegmentRequest)
                      -> Result<UpdateSegmentResponse, UpdateSegmentError>;


    #[doc="Update an SMS channel"]
    fn update_sms_channel(&self,
                          input: &UpdateSmsChannelRequest)
                          -> Result<UpdateSmsChannelResponse, UpdateSmsChannelError>;
}
/// A client for the Amazon Pinpoint API.
pub struct PinpointClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    credentials_provider: P,
    region: region::Region,
    dispatcher: D,
}

impl<P, D> PinpointClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
        PinpointClient {
            credentials_provider: credentials_provider,
            region: region,
            dispatcher: request_dispatcher,
        }
    }
}

impl<P, D> Pinpoint for PinpointClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    #[doc="Used to create an app."]
    fn create_app(&self, input: &CreateAppRequest) -> Result<CreateAppResponse, CreateAppError> {
        let request_uri = "/v1/apps";

        let mut request = SignedRequest::new("POST", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input.create_application_request).unwrap());
        request.set_payload(encoded);



        request.sign(&self.credentials_provider.credentials()?);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Created => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<CreateAppResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(CreateAppError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="Creates or updates a campaign."]
    fn create_campaign(&self,
                       input: &CreateCampaignRequest)
                       -> Result<CreateCampaignResponse, CreateCampaignError> {
        let request_uri = format!("/v1/apps/{application-id}/campaigns",
                                  application_id = input.application_id);

        let mut request = SignedRequest::new("POST", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input.write_campaign_request).unwrap());
        request.set_payload(encoded);



        request.sign(&self.credentials_provider.credentials()?);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Created => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<CreateCampaignResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(CreateCampaignError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="Creates or updates an import job."]
    fn create_import_job(&self,
                         input: &CreateImportJobRequest)
                         -> Result<CreateImportJobResponse, CreateImportJobError> {
        let request_uri = format!("/v1/apps/{application-id}/jobs/import",
                                  application_id = input.application_id);

        let mut request = SignedRequest::new("POST", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input.import_job_request).unwrap());
        request.set_payload(encoded);



        request.sign(&self.credentials_provider.credentials()?);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Created => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<CreateImportJobResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(CreateImportJobError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="Used to create or update a segment."]
    fn create_segment(&self,
                      input: &CreateSegmentRequest)
                      -> Result<CreateSegmentResponse, CreateSegmentError> {
        let request_uri = format!("/v1/apps/{application-id}/segments",
                                  application_id = input.application_id);

        let mut request = SignedRequest::new("POST", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input.write_segment_request).unwrap());
        request.set_payload(encoded);



        request.sign(&self.credentials_provider.credentials()?);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Created => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<CreateSegmentResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(CreateSegmentError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="Deletes the APNs channel for an app."]
    fn delete_apns_channel(&self,
                           input: &DeleteApnsChannelRequest)
                           -> Result<DeleteApnsChannelResponse, DeleteApnsChannelError> {
        let request_uri = format!("/v1/apps/{application-id}/channels/apns",
                                  application_id = input.application_id);

        let mut request =
            SignedRequest::new("DELETE", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());




        request.sign(&self.credentials_provider.credentials()?);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<DeleteApnsChannelResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DeleteApnsChannelError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="Delete an APNS sandbox channel"]
    fn delete_apns_sandbox_channel
        (&self,
         input: &DeleteApnsSandboxChannelRequest)
         -> Result<DeleteApnsSandboxChannelResponse, DeleteApnsSandboxChannelError> {
        let request_uri = format!("/v1/apps/{application-id}/channels/apns_sandbox",
                                  application_id = input.application_id);

        let mut request =
            SignedRequest::new("DELETE", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());




        request.sign(&self.credentials_provider.credentials()?);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<DeleteApnsSandboxChannelResponse>(&body)
                    .unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DeleteApnsSandboxChannelError::from_body(String::from_utf8_lossy(&body)
                                                                 .as_ref()))
            }
        }
    }


    #[doc="Deletes an app."]
    fn delete_app(&self, input: &DeleteAppRequest) -> Result<DeleteAppResponse, DeleteAppError> {
        let request_uri = format!("/v1/apps/{application-id}",
                                  application_id = input.application_id);

        let mut request =
            SignedRequest::new("DELETE", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());




        request.sign(&self.credentials_provider.credentials()?);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<DeleteAppResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DeleteAppError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="Deletes a campaign."]
    fn delete_campaign(&self,
                       input: &DeleteCampaignRequest)
                       -> Result<DeleteCampaignResponse, DeleteCampaignError> {
        let request_uri = format!("/v1/apps/{application-id}/campaigns/{campaign-id}",
                                  application_id = input.application_id,
                                  campaign_id = input.campaign_id);

        let mut request =
            SignedRequest::new("DELETE", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());




        request.sign(&self.credentials_provider.credentials()?);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<DeleteCampaignResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DeleteCampaignError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="Delete an email channel"]
    fn delete_email_channel(&self,
                            input: &DeleteEmailChannelRequest)
                            -> Result<DeleteEmailChannelResponse, DeleteEmailChannelError> {
        let request_uri = format!("/v1/apps/{application-id}/channels/email",
                                  application_id = input.application_id);

        let mut request =
            SignedRequest::new("DELETE", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());




        request.sign(&self.credentials_provider.credentials()?);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<DeleteEmailChannelResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DeleteEmailChannelError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="Deletes the event stream for an app."]
    fn delete_event_stream(&self,
                           input: &DeleteEventStreamRequest)
                           -> Result<DeleteEventStreamResponse, DeleteEventStreamError> {
        let request_uri = format!("/v1/apps/{application-id}/eventstream",
                                  application_id = input.application_id);

        let mut request =
            SignedRequest::new("DELETE", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());




        request.sign(&self.credentials_provider.credentials()?);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<DeleteEventStreamResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DeleteEventStreamError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="Deletes the GCM channel for an app."]
    fn delete_gcm_channel(&self,
                          input: &DeleteGcmChannelRequest)
                          -> Result<DeleteGcmChannelResponse, DeleteGcmChannelError> {
        let request_uri = format!("/v1/apps/{application-id}/channels/gcm",
                                  application_id = input.application_id);

        let mut request =
            SignedRequest::new("DELETE", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());




        request.sign(&self.credentials_provider.credentials()?);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<DeleteGcmChannelResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DeleteGcmChannelError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="Deletes a segment."]
    fn delete_segment(&self,
                      input: &DeleteSegmentRequest)
                      -> Result<DeleteSegmentResponse, DeleteSegmentError> {
        let request_uri = format!("/v1/apps/{application-id}/segments/{segment-id}",
                                  application_id = input.application_id,
                                  segment_id = input.segment_id);

        let mut request =
            SignedRequest::new("DELETE", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());




        request.sign(&self.credentials_provider.credentials()?);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<DeleteSegmentResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DeleteSegmentError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="Delete an SMS channel"]
    fn delete_sms_channel(&self,
                          input: &DeleteSmsChannelRequest)
                          -> Result<DeleteSmsChannelResponse, DeleteSmsChannelError> {
        let request_uri = format!("/v1/apps/{application-id}/channels/sms",
                                  application_id = input.application_id);

        let mut request =
            SignedRequest::new("DELETE", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());




        request.sign(&self.credentials_provider.credentials()?);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<DeleteSmsChannelResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DeleteSmsChannelError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="Returns information about the APNs channel for an app."]
    fn get_apns_channel(&self,
                        input: &GetApnsChannelRequest)
                        -> Result<GetApnsChannelResponse, GetApnsChannelError> {
        let request_uri = format!("/v1/apps/{application-id}/channels/apns",
                                  application_id = input.application_id);

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());




        request.sign(&self.credentials_provider.credentials()?);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<GetApnsChannelResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetApnsChannelError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="Get an APNS sandbox channel"]
    fn get_apns_sandbox_channel
        (&self,
         input: &GetApnsSandboxChannelRequest)
         -> Result<GetApnsSandboxChannelResponse, GetApnsSandboxChannelError> {
        let request_uri = format!("/v1/apps/{application-id}/channels/apns_sandbox",
                                  application_id = input.application_id);

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());




        request.sign(&self.credentials_provider.credentials()?);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<GetApnsSandboxChannelResponse>(&body)
                    .unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetApnsSandboxChannelError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="Returns information about an app."]
    fn get_app(&self, input: &GetAppRequest) -> Result<GetAppResponse, GetAppError> {
        let request_uri = format!("/v1/apps/{application-id}",
                                  application_id = input.application_id);

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());




        request.sign(&self.credentials_provider.credentials()?);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<GetAppResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetAppError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="Used to request the settings for an app."]
    fn get_application_settings
        (&self,
         input: &GetApplicationSettingsRequest)
         -> Result<GetApplicationSettingsResponse, GetApplicationSettingsError> {
        let request_uri = format!("/v1/apps/{application-id}/settings",
                                  application_id = input.application_id);

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());




        request.sign(&self.credentials_provider.credentials()?);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<GetApplicationSettingsResponse>(&body)
                    .unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetApplicationSettingsError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="Returns information about your apps."]
    fn get_apps(&self, input: &GetAppsRequest) -> Result<GetAppsResponse, GetAppsError> {
        let request_uri = "/v1/apps";

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());


        let mut params = Params::new();
        if let Some(ref x) = input.page_size {
            params.put("page-size", x);
        }
        if let Some(ref x) = input.token {
            params.put("token", x);
        }
        request.set_params(params);

        request.sign(&self.credentials_provider.credentials()?);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<GetAppsResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetAppsError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="Returns information about a campaign."]
    fn get_campaign(&self,
                    input: &GetCampaignRequest)
                    -> Result<GetCampaignResponse, GetCampaignError> {
        let request_uri = format!("/v1/apps/{application-id}/campaigns/{campaign-id}",
                                  application_id = input.application_id,
                                  campaign_id = input.campaign_id);

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());




        request.sign(&self.credentials_provider.credentials()?);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<GetCampaignResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetCampaignError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="Returns information about the activity performed by a campaign."]
    fn get_campaign_activities
        (&self,
         input: &GetCampaignActivitiesRequest)
         -> Result<GetCampaignActivitiesResponse, GetCampaignActivitiesError> {
        let request_uri = format!("/v1/apps/{application-id}/campaigns/{campaign-id}/activities",
                                  application_id = input.application_id,
                                  campaign_id = input.campaign_id);

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());


        let mut params = Params::new();
        if let Some(ref x) = input.page_size {
            params.put("page-size", x);
        }
        if let Some(ref x) = input.token {
            params.put("token", x);
        }
        request.set_params(params);

        request.sign(&self.credentials_provider.credentials()?);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<GetCampaignActivitiesResponse>(&body)
                    .unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetCampaignActivitiesError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="Returns information about a specific version of a campaign."]
    fn get_campaign_version(&self,
                            input: &GetCampaignVersionRequest)
                            -> Result<GetCampaignVersionResponse, GetCampaignVersionError> {
        let request_uri = format!("/v1/apps/{application-id}/campaigns/{campaign-id}/versions/{version}",
                                  application_id = input.application_id,
                                  campaign_id = input.campaign_id,
                                  version = input.version);

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());




        request.sign(&self.credentials_provider.credentials()?);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<GetCampaignVersionResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetCampaignVersionError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="Returns information about your campaign versions."]
    fn get_campaign_versions(&self,
                             input: &GetCampaignVersionsRequest)
                             -> Result<GetCampaignVersionsResponse, GetCampaignVersionsError> {
        let request_uri = format!("/v1/apps/{application-id}/campaigns/{campaign-id}/versions",
                                  application_id = input.application_id,
                                  campaign_id = input.campaign_id);

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());


        let mut params = Params::new();
        if let Some(ref x) = input.page_size {
            params.put("page-size", x);
        }
        if let Some(ref x) = input.token {
            params.put("token", x);
        }
        request.set_params(params);

        request.sign(&self.credentials_provider.credentials()?);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<GetCampaignVersionsResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetCampaignVersionsError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="Returns information about your campaigns."]
    fn get_campaigns(&self,
                     input: &GetCampaignsRequest)
                     -> Result<GetCampaignsResponse, GetCampaignsError> {
        let request_uri = format!("/v1/apps/{application-id}/campaigns",
                                  application_id = input.application_id);

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());


        let mut params = Params::new();
        if let Some(ref x) = input.page_size {
            params.put("page-size", x);
        }
        if let Some(ref x) = input.token {
            params.put("token", x);
        }
        request.set_params(params);

        request.sign(&self.credentials_provider.credentials()?);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<GetCampaignsResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetCampaignsError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="Get an email channel"]
    fn get_email_channel(&self,
                         input: &GetEmailChannelRequest)
                         -> Result<GetEmailChannelResponse, GetEmailChannelError> {
        let request_uri = format!("/v1/apps/{application-id}/channels/email",
                                  application_id = input.application_id);

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());




        request.sign(&self.credentials_provider.credentials()?);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<GetEmailChannelResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetEmailChannelError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="Returns information about an endpoint."]
    fn get_endpoint(&self,
                    input: &GetEndpointRequest)
                    -> Result<GetEndpointResponse, GetEndpointError> {
        let request_uri = format!("/v1/apps/{application-id}/endpoints/{endpoint-id}",
                                  application_id = input.application_id,
                                  endpoint_id = input.endpoint_id);

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());




        request.sign(&self.credentials_provider.credentials()?);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<GetEndpointResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetEndpointError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="Returns the event stream for an app."]
    fn get_event_stream(&self,
                        input: &GetEventStreamRequest)
                        -> Result<GetEventStreamResponse, GetEventStreamError> {
        let request_uri = format!("/v1/apps/{application-id}/eventstream",
                                  application_id = input.application_id);

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());




        request.sign(&self.credentials_provider.credentials()?);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<GetEventStreamResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetEventStreamError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="Returns information about the GCM channel for an app."]
    fn get_gcm_channel(&self,
                       input: &GetGcmChannelRequest)
                       -> Result<GetGcmChannelResponse, GetGcmChannelError> {
        let request_uri = format!("/v1/apps/{application-id}/channels/gcm",
                                  application_id = input.application_id);

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());




        request.sign(&self.credentials_provider.credentials()?);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<GetGcmChannelResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetGcmChannelError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="Returns information about an import job."]
    fn get_import_job(&self,
                      input: &GetImportJobRequest)
                      -> Result<GetImportJobResponse, GetImportJobError> {
        let request_uri = format!("/v1/apps/{application-id}/jobs/import/{job-id}",
                                  application_id = input.application_id,
                                  job_id = input.job_id);

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());




        request.sign(&self.credentials_provider.credentials()?);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<GetImportJobResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetImportJobError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="Returns information about your import jobs."]
    fn get_import_jobs(&self,
                       input: &GetImportJobsRequest)
                       -> Result<GetImportJobsResponse, GetImportJobsError> {
        let request_uri = format!("/v1/apps/{application-id}/jobs/import",
                                  application_id = input.application_id);

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());


        let mut params = Params::new();
        if let Some(ref x) = input.page_size {
            params.put("page-size", x);
        }
        if let Some(ref x) = input.token {
            params.put("token", x);
        }
        request.set_params(params);

        request.sign(&self.credentials_provider.credentials()?);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<GetImportJobsResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetImportJobsError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="Returns information about a segment."]
    fn get_segment(&self,
                   input: &GetSegmentRequest)
                   -> Result<GetSegmentResponse, GetSegmentError> {
        let request_uri = format!("/v1/apps/{application-id}/segments/{segment-id}",
                                  application_id = input.application_id,
                                  segment_id = input.segment_id);

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());




        request.sign(&self.credentials_provider.credentials()?);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<GetSegmentResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetSegmentError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="Returns a list of import jobs for a specific segment."]
    fn get_segment_import_jobs
        (&self,
         input: &GetSegmentImportJobsRequest)
         -> Result<GetSegmentImportJobsResponse, GetSegmentImportJobsError> {
        let request_uri = format!("/v1/apps/{application-id}/segments/{segment-id}/jobs/import",
                                  application_id = input.application_id,
                                  segment_id = input.segment_id);

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());


        let mut params = Params::new();
        if let Some(ref x) = input.page_size {
            params.put("page-size", x);
        }
        if let Some(ref x) = input.token {
            params.put("token", x);
        }
        request.set_params(params);

        request.sign(&self.credentials_provider.credentials()?);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<GetSegmentImportJobsResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetSegmentImportJobsError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="Returns information about a segment version."]
    fn get_segment_version(&self,
                           input: &GetSegmentVersionRequest)
                           -> Result<GetSegmentVersionResponse, GetSegmentVersionError> {
        let request_uri = format!("/v1/apps/{application-id}/segments/{segment-id}/versions/{version}",
                                  application_id = input.application_id,
                                  segment_id = input.segment_id,
                                  version = input.version);

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());




        request.sign(&self.credentials_provider.credentials()?);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<GetSegmentVersionResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetSegmentVersionError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="Returns information about your segment versions."]
    fn get_segment_versions(&self,
                            input: &GetSegmentVersionsRequest)
                            -> Result<GetSegmentVersionsResponse, GetSegmentVersionsError> {
        let request_uri = format!("/v1/apps/{application-id}/segments/{segment-id}/versions",
                                  application_id = input.application_id,
                                  segment_id = input.segment_id);

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());


        let mut params = Params::new();
        if let Some(ref x) = input.page_size {
            params.put("page-size", x);
        }
        if let Some(ref x) = input.token {
            params.put("token", x);
        }
        request.set_params(params);

        request.sign(&self.credentials_provider.credentials()?);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<GetSegmentVersionsResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetSegmentVersionsError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="Used to get information about your segments."]
    fn get_segments(&self,
                    input: &GetSegmentsRequest)
                    -> Result<GetSegmentsResponse, GetSegmentsError> {
        let request_uri = format!("/v1/apps/{application-id}/segments",
                                  application_id = input.application_id);

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());


        let mut params = Params::new();
        if let Some(ref x) = input.page_size {
            params.put("page-size", x);
        }
        if let Some(ref x) = input.token {
            params.put("token", x);
        }
        request.set_params(params);

        request.sign(&self.credentials_provider.credentials()?);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<GetSegmentsResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetSegmentsError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="Get an SMS channel"]
    fn get_sms_channel(&self,
                       input: &GetSmsChannelRequest)
                       -> Result<GetSmsChannelResponse, GetSmsChannelError> {
        let request_uri = format!("/v1/apps/{application-id}/channels/sms",
                                  application_id = input.application_id);

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());




        request.sign(&self.credentials_provider.credentials()?);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<GetSmsChannelResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetSmsChannelError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="Use to create or update the event stream for an app."]
    fn put_event_stream(&self,
                        input: &PutEventStreamRequest)
                        -> Result<PutEventStreamResponse, PutEventStreamError> {
        let request_uri = format!("/v1/apps/{application-id}/eventstream",
                                  application_id = input.application_id);

        let mut request = SignedRequest::new("POST", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input.write_event_stream).unwrap());
        request.set_payload(encoded);



        request.sign(&self.credentials_provider.credentials()?);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<PutEventStreamResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(PutEventStreamError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="Send a batch of messages"]
    fn send_messages(&self,
                     input: &SendMessagesRequest)
                     -> Result<SendMessagesResponse, SendMessagesError> {
        let request_uri = format!("/v1/apps/{application-id}/messages",
                                  application_id = input.application_id);

        let mut request = SignedRequest::new("POST", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input.message_request).unwrap());
        request.set_payload(encoded);



        request.sign(&self.credentials_provider.credentials()?);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<SendMessagesResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(SendMessagesError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="Use to update the APNs channel for an app."]
    fn update_apns_channel(&self,
                           input: &UpdateApnsChannelRequest)
                           -> Result<UpdateApnsChannelResponse, UpdateApnsChannelError> {
        let request_uri = format!("/v1/apps/{application-id}/channels/apns",
                                  application_id = input.application_id);

        let mut request = SignedRequest::new("PUT", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input.apns_channel_request).unwrap());
        request.set_payload(encoded);



        request.sign(&self.credentials_provider.credentials()?);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<UpdateApnsChannelResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(UpdateApnsChannelError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="Update an APNS sandbox channel"]
    fn update_apns_sandbox_channel
        (&self,
         input: &UpdateApnsSandboxChannelRequest)
         -> Result<UpdateApnsSandboxChannelResponse, UpdateApnsSandboxChannelError> {
        let request_uri = format!("/v1/apps/{application-id}/channels/apns_sandbox",
                                  application_id = input.application_id);

        let mut request = SignedRequest::new("PUT", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input.apns_sandbox_channel_request).unwrap());
        request.set_payload(encoded);



        request.sign(&self.credentials_provider.credentials()?);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<UpdateApnsSandboxChannelResponse>(&body)
                    .unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(UpdateApnsSandboxChannelError::from_body(String::from_utf8_lossy(&body)
                                                                 .as_ref()))
            }
        }
    }


    #[doc="Used to update the settings for an app."]
    fn update_application_settings
        (&self,
         input: &UpdateApplicationSettingsRequest)
         -> Result<UpdateApplicationSettingsResponse, UpdateApplicationSettingsError> {
        let request_uri = format!("/v1/apps/{application-id}/settings",
                                  application_id = input.application_id);

        let mut request = SignedRequest::new("PUT", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input.write_application_settings_request).unwrap());
        request.set_payload(encoded);



        request.sign(&self.credentials_provider.credentials()?);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<UpdateApplicationSettingsResponse>(&body)
                    .unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(UpdateApplicationSettingsError::from_body(String::from_utf8_lossy(&body)
                                                                  .as_ref()))
            }
        }
    }


    #[doc="Use to update a campaign."]
    fn update_campaign(&self,
                       input: &UpdateCampaignRequest)
                       -> Result<UpdateCampaignResponse, UpdateCampaignError> {
        let request_uri = format!("/v1/apps/{application-id}/campaigns/{campaign-id}",
                                  application_id = input.application_id,
                                  campaign_id = input.campaign_id);

        let mut request = SignedRequest::new("PUT", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input.write_campaign_request).unwrap());
        request.set_payload(encoded);



        request.sign(&self.credentials_provider.credentials()?);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<UpdateCampaignResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(UpdateCampaignError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="Update an email channel"]
    fn update_email_channel(&self,
                            input: &UpdateEmailChannelRequest)
                            -> Result<UpdateEmailChannelResponse, UpdateEmailChannelError> {
        let request_uri = format!("/v1/apps/{application-id}/channels/email",
                                  application_id = input.application_id);

        let mut request = SignedRequest::new("PUT", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input.email_channel_request).unwrap());
        request.set_payload(encoded);



        request.sign(&self.credentials_provider.credentials()?);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<UpdateEmailChannelResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(UpdateEmailChannelError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="Use to update an endpoint."]
    fn update_endpoint(&self,
                       input: &UpdateEndpointRequest)
                       -> Result<UpdateEndpointResponse, UpdateEndpointError> {
        let request_uri = format!("/v1/apps/{application-id}/endpoints/{endpoint-id}",
                                  application_id = input.application_id,
                                  endpoint_id = input.endpoint_id);

        let mut request = SignedRequest::new("PUT", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input.endpoint_request).unwrap());
        request.set_payload(encoded);



        request.sign(&self.credentials_provider.credentials()?);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Accepted => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<UpdateEndpointResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(UpdateEndpointError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="Use to update a batch of endpoints."]
    fn update_endpoints_batch
        (&self,
         input: &UpdateEndpointsBatchRequest)
         -> Result<UpdateEndpointsBatchResponse, UpdateEndpointsBatchError> {
        let request_uri = format!("/v1/apps/{application-id}/endpoints",
                                  application_id = input.application_id);

        let mut request = SignedRequest::new("PUT", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input.endpoint_batch_request).unwrap());
        request.set_payload(encoded);



        request.sign(&self.credentials_provider.credentials()?);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Accepted => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<UpdateEndpointsBatchResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(UpdateEndpointsBatchError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="Use to update the GCM channel for an app."]
    fn update_gcm_channel(&self,
                          input: &UpdateGcmChannelRequest)
                          -> Result<UpdateGcmChannelResponse, UpdateGcmChannelError> {
        let request_uri = format!("/v1/apps/{application-id}/channels/gcm",
                                  application_id = input.application_id);

        let mut request = SignedRequest::new("PUT", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input.gcm_channel_request).unwrap());
        request.set_payload(encoded);



        request.sign(&self.credentials_provider.credentials()?);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<UpdateGcmChannelResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(UpdateGcmChannelError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="Use to update a segment."]
    fn update_segment(&self,
                      input: &UpdateSegmentRequest)
                      -> Result<UpdateSegmentResponse, UpdateSegmentError> {
        let request_uri = format!("/v1/apps/{application-id}/segments/{segment-id}",
                                  application_id = input.application_id,
                                  segment_id = input.segment_id);

        let mut request = SignedRequest::new("PUT", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input.write_segment_request).unwrap());
        request.set_payload(encoded);



        request.sign(&self.credentials_provider.credentials()?);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<UpdateSegmentResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(UpdateSegmentError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="Update an SMS channel"]
    fn update_sms_channel(&self,
                          input: &UpdateSmsChannelRequest)
                          -> Result<UpdateSmsChannelResponse, UpdateSmsChannelError> {
        let request_uri = format!("/v1/apps/{application-id}/channels/sms",
                                  application_id = input.application_id);

        let mut request = SignedRequest::new("PUT", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input.sms_channel_request).unwrap());
        request.set_payload(encoded);



        request.sign(&self.credentials_provider.credentials()?);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<UpdateSmsChannelResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(UpdateSmsChannelError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }
}

#[cfg(test)]
mod protocol_tests {}
