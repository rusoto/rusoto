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
#[allow(unused_imports)]
use rusoto_core::pagination::{all_pages, PagedOutput, PagedRequest, RusotoStream};
use rusoto_core::region;
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoError};

use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
use serde_json;
/// <p>An object that contains information about your account details.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AccountDetails {
    /// <p>Additional email addresses where updates are sent about your account review process.</p>
    #[serde(rename = "AdditionalContactEmailAddresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_contact_email_addresses: Option<Vec<String>>,
    /// <p>The language you would prefer for the case. The contact language can be one of <code>ENGLISH</code> or <code>JAPANESE</code>.</p>
    #[serde(rename = "ContactLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_language: Option<String>,
    /// <p><p>The type of email your account is sending. The mail type can be one of the following:</p> <ul> <li> <p> <code>MARKETING</code> – Most of your sending traffic is to keep your customers informed of your latest offering.</p> </li> <li> <p> <code>TRANSACTIONAL</code> – Most of your sending traffic is to communicate during a transaction with a customer.</p> </li> </ul></p>
    #[serde(rename = "MailType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mail_type: Option<String>,
    /// <p>Information about the review of the latest details you submitted.</p>
    #[serde(rename = "ReviewDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub review_details: Option<ReviewDetails>,
    /// <p>A description of the types of email that you plan to send.</p>
    #[serde(rename = "UseCaseDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_case_description: Option<String>,
    /// <p>The URL of your website. This information helps us better understand the type of content that you plan to send.</p>
    #[serde(rename = "WebsiteURL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website_url: Option<String>,
}

/// <p>An object that contains information about a blacklisting event that impacts one of the dedicated IP addresses that is associated with your account.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BlacklistEntry {
    /// <p>Additional information about the blacklisting event, as provided by the blacklist maintainer.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The time when the blacklisting event occurred, shown in Unix time format.</p>
    #[serde(rename = "ListingTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listing_time: Option<f64>,
    /// <p>The name of the blacklist that the IP address appears on.</p>
    #[serde(rename = "RblName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rbl_name: Option<String>,
}

/// <p>Represents the body of the email message.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Body {
    /// <p>An object that represents the version of the message that is displayed in email clients that support HTML. HTML messages can include formatted text, hyperlinks, images, and more. </p>
    #[serde(rename = "Html")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html: Option<Content>,
    /// <p>An object that represents the version of the message that is displayed in email clients that don't support HTML, or clients where the recipient has disabled HTML rendering.</p>
    #[serde(rename = "Text")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<Content>,
}

/// <p>An object that contains the body of the message. You can specify a template message.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BulkEmailContent {
    /// <p>The template to use for the bulk email message.</p>
    #[serde(rename = "Template")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<Template>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BulkEmailEntry {
    /// <p><p>Represents the destination of the message, consisting of To:, CC:, and BCC: fields.</p> <note> <p>Amazon SES does not support the SMTPUTF8 extension, as described in <a href="https://tools.ietf.org/html/rfc6531">RFC6531</a>. For this reason, the local part of a destination email address (the part of the email address that precedes the @ sign) may only contain <a href="https://en.wikipedia.org/wiki/Email_address#Local-part">7-bit ASCII characters</a>. If the domain part of an address (the part after the @ sign) contains non-ASCII characters, they must be encoded using Punycode, as described in <a href="https://tools.ietf.org/html/rfc3492.html">RFC3492</a>.</p> </note></p>
    #[serde(rename = "Destination")]
    pub destination: Destination,
    /// <p>The <code>ReplacementEmailContent</code> associated with a <code>BulkEmailEntry</code>.</p>
    #[serde(rename = "ReplacementEmailContent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replacement_email_content: Option<ReplacementEmailContent>,
    /// <p>A list of tags, in the form of name/value pairs, to apply to an email that you send using the <code>SendBulkTemplatedEmail</code> operation. Tags correspond to characteristics of the email that you define, so that you can publish email sending events.</p>
    #[serde(rename = "ReplacementTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replacement_tags: Option<Vec<MessageTag>>,
}

/// <p>The result of the <code>SendBulkEmail</code> operation of each specified <code>BulkEmailEntry</code>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BulkEmailEntryResult {
    /// <p>A description of an error that prevented a message being sent using the <code>SendBulkTemplatedEmail</code> operation.</p>
    #[serde(rename = "Error")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// <p>The unique message identifier returned from the <code>SendBulkTemplatedEmail</code> operation.</p>
    #[serde(rename = "MessageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    /// <p><p>The status of a message sent using the <code>SendBulkTemplatedEmail</code> operation.</p> <p>Possible values for this parameter include:</p> <ul> <li> <p>SUCCESS: Amazon SES accepted the message, and will attempt to deliver it to the recipients.</p> </li> <li> <p>MESSAGE<em>REJECTED: The message was rejected because it contained a virus.</p> </li> <li> <p>MAIL</em>FROM<em>DOMAIN</em>NOT<em>VERIFIED: The sender&#39;s email address or domain was not verified.</p> </li> <li> <p>CONFIGURATION</em>SET<em>DOES</em>NOT<em>EXIST: The configuration set you specified does not exist.</p> </li> <li> <p>TEMPLATE</em>DOES<em>NOT</em>EXIST: The template you specified does not exist.</p> </li> <li> <p>ACCOUNT<em>SUSPENDED: Your account has been shut down because of issues related to your email sending practices.</p> </li> <li> <p>ACCOUNT</em>THROTTLED: The number of emails you can send has been reduced because your account has exceeded its allocated sending limit.</p> </li> <li> <p>ACCOUNT<em>DAILY</em>QUOTA<em>EXCEEDED: You have reached or exceeded the maximum number of emails you can send from your account in a 24-hour period.</p> </li> <li> <p>INVALID</em>SENDING<em>POOL</em>NAME: The configuration set you specified refers to an IP pool that does not exist.</p> </li> <li> <p>ACCOUNT<em>SENDING</em>PAUSED: Email sending for the Amazon SES account was disabled using the <a href="https://docs.aws.amazon.com/ses/latest/APIReference/API_UpdateAccountSendingEnabled.html">UpdateAccountSendingEnabled</a> operation.</p> </li> <li> <p>CONFIGURATION<em>SET</em>SENDING<em>PAUSED: Email sending for this configuration set was disabled using the &lt;a href=&quot;https://docs.aws.amazon.com/ses/latest/APIReference/API</em>UpdateConfigurationSetSendingEnabled.html&quot;&gt;UpdateConfigurationSetSendingEnabled</a> operation.</p> </li> <li> <p>INVALID<em>PARAMETER</em>VALUE: One or more of the parameters you specified when calling this operation was invalid. See the error message for additional information.</p> </li> <li> <p>TRANSIENT_FAILURE: Amazon SES was unable to process your request because of a temporary issue.</p> </li> <li> <p>FAILED: Amazon SES was unable to process your request. See the error message for additional information.</p> </li> </ul></p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>An object that defines an Amazon CloudWatch destination for email events. You can use Amazon CloudWatch to monitor and gain insights on your email sending metrics.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct CloudWatchDestination {
    /// <p>An array of objects that define the dimensions to use when you send email events to Amazon CloudWatch.</p>
    #[serde(rename = "DimensionConfigurations")]
    pub dimension_configurations: Vec<CloudWatchDimensionConfiguration>,
}

/// <p>An object that defines the dimension configuration to use when you send email events to Amazon CloudWatch.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct CloudWatchDimensionConfiguration {
    /// <p><p>The default value of the dimension that is published to Amazon CloudWatch if you don&#39;t provide the value of the dimension when you send an email. This value has to meet the following criteria:</p> <ul> <li> <p>It can only contain ASCII letters (a–z, A–Z), numbers (0–9), underscores (_), or dashes (-).</p> </li> <li> <p>It can contain no more than 256 characters.</p> </li> </ul></p>
    #[serde(rename = "DefaultDimensionValue")]
    pub default_dimension_value: String,
    /// <p><p>The name of an Amazon CloudWatch dimension associated with an email sending metric. The name has to meet the following criteria:</p> <ul> <li> <p>It can only contain ASCII letters (a–z, A–Z), numbers (0–9), underscores (_), or dashes (-).</p> </li> <li> <p>It can contain no more than 256 characters.</p> </li> </ul></p>
    #[serde(rename = "DimensionName")]
    pub dimension_name: String,
    /// <p>The location where the Amazon SES API v2 finds the value of a dimension to publish to Amazon CloudWatch. If you want to use the message tags that you specify using an <code>X-SES-MESSAGE-TAGS</code> header or a parameter to the <code>SendEmail</code> or <code>SendRawEmail</code> API, choose <code>messageTag</code>. If you want to use your own email headers, choose <code>emailHeader</code>. If you want to use link tags, choose <code>linkTags</code>.</p>
    #[serde(rename = "DimensionValueSource")]
    pub dimension_value_source: String,
}

/// <p>A contact is the end-user who is receiving the email.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Contact {
    /// <p>The contact's email address.</p>
    #[serde(rename = "EmailAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    /// <p>A timestamp noting the last time the contact's information was updated.</p>
    #[serde(rename = "LastUpdatedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_timestamp: Option<f64>,
    /// <p>The default topic preferences applied to the contact.</p>
    #[serde(rename = "TopicDefaultPreferences")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_default_preferences: Option<Vec<TopicPreference>>,
    /// <p>The contact's preference for being opted-in to or opted-out of a topic.</p>
    #[serde(rename = "TopicPreferences")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_preferences: Option<Vec<TopicPreference>>,
    /// <p>A boolean value status noting if the contact is unsubscribed from all contact list topics.</p>
    #[serde(rename = "UnsubscribeAll")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unsubscribe_all: Option<bool>,
}

/// <p>A list that contains contacts that have subscribed to a particular topic or topics.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ContactList {
    /// <p>The name of the contact list.</p>
    #[serde(rename = "ContactListName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_list_name: Option<String>,
    /// <p>A timestamp noting the last time the contact list was updated.</p>
    #[serde(rename = "LastUpdatedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_timestamp: Option<f64>,
}

/// <p>An object that contains details about the action of a contact list.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ContactListDestination {
    /// <p><p>&gt;The type of action that you want to perform on the addresses. Acceptable values:</p> <ul> <li> <p>PUT: add the addresses to the contact list. If the record already exists, it will override it with the new value.</p> </li> <li> <p>DELETE: remove the addresses from the contact list.</p> </li> </ul></p>
    #[serde(rename = "ContactListImportAction")]
    pub contact_list_import_action: String,
    /// <p>The name of the contact list.</p>
    #[serde(rename = "ContactListName")]
    pub contact_list_name: String,
}

/// <p>An object that represents the content of the email, and optionally a character set specification.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Content {
    /// <p>The character set for the content. Because of the constraints of the SMTP protocol, Amazon SES uses 7-bit ASCII by default. If the text includes characters outside of the ASCII range, you have to specify a character set. For example, you could specify <code>UTF-8</code>, <code>ISO-8859-1</code>, or <code>Shift_JIS</code>.</p>
    #[serde(rename = "Charset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charset: Option<String>,
    /// <p>The content of the message itself.</p>
    #[serde(rename = "Data")]
    pub data: String,
}

/// <p>A request to add an event destination to a configuration set.</p>
/// see [SesV2::create_configuration_set_event_destination]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateConfigurationSetEventDestinationRequest {
    /// <p>The name of the configuration set that you want to add an event destination to.</p>
    #[serde(rename = "ConfigurationSetName")]
    pub configuration_set_name: String,
    /// <p>An object that defines the event destination.</p>
    #[serde(rename = "EventDestination")]
    pub event_destination: EventDestinationDefinition,
    /// <p>A name that identifies the event destination within the configuration set.</p>
    #[serde(rename = "EventDestinationName")]
    pub event_destination_name: String,
}

/// <p>An HTTP 200 response if the request succeeds, or an error message if the request fails.</p>
/// see [SesV2::create_configuration_set_event_destination]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateConfigurationSetEventDestinationResponse {}

/// <p>A request to create a configuration set.</p>
/// see [SesV2::create_configuration_set]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateConfigurationSetRequest {
    /// <p>The name of the configuration set.</p>
    #[serde(rename = "ConfigurationSetName")]
    pub configuration_set_name: String,
    /// <p>An object that defines the dedicated IP pool that is used to send emails that you send using the configuration set.</p>
    #[serde(rename = "DeliveryOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_options: Option<DeliveryOptions>,
    /// <p>An object that defines whether or not Amazon SES collects reputation metrics for the emails that you send that use the configuration set.</p>
    #[serde(rename = "ReputationOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reputation_options: Option<ReputationOptions>,
    /// <p>An object that defines whether or not Amazon SES can send email that you send using the configuration set.</p>
    #[serde(rename = "SendingOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sending_options: Option<SendingOptions>,
    #[serde(rename = "SuppressionOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suppression_options: Option<SuppressionOptions>,
    /// <p>An array of objects that define the tags (keys and values) that you want to associate with the configuration set.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>An object that defines the open and click tracking options for emails that you send using the configuration set.</p>
    #[serde(rename = "TrackingOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_options: Option<TrackingOptions>,
}

/// <p>An HTTP 200 response if the request succeeds, or an error message if the request fails.</p>
/// see [SesV2::create_configuration_set]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateConfigurationSetResponse {}

/// see [SesV2::create_contact_list]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateContactListRequest {
    /// <p>The name of the contact list.</p>
    #[serde(rename = "ContactListName")]
    pub contact_list_name: String,
    /// <p>A description of what the contact list is about.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The tags associated with a contact list.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>An interest group, theme, or label within a list. A contact list can have multiple topics.</p>
    #[serde(rename = "Topics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topics: Option<Vec<Topic>>,
}

/// see [SesV2::create_contact_list]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateContactListResponse {}

/// see [SesV2::create_contact]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateContactRequest {
    /// <p>The attribute data attached to a contact.</p>
    #[serde(rename = "AttributesData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes_data: Option<String>,
    /// <p>The name of the contact list to which the contact should be added.</p>
    #[serde(rename = "ContactListName")]
    pub contact_list_name: String,
    /// <p>The contact's email address.</p>
    #[serde(rename = "EmailAddress")]
    pub email_address: String,
    /// <p>The contact's preferences for being opted-in to or opted-out of topics.</p>
    #[serde(rename = "TopicPreferences")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_preferences: Option<Vec<TopicPreference>>,
    /// <p>A boolean value status noting if the contact is unsubscribed from all contact list topics.</p>
    #[serde(rename = "UnsubscribeAll")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unsubscribe_all: Option<bool>,
}

/// see [SesV2::create_contact]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateContactResponse {}

/// <p>Represents a request to create a custom verification email template.</p>
/// see [SesV2::create_custom_verification_email_template]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateCustomVerificationEmailTemplateRequest {
    /// <p>The URL that the recipient of the verification email is sent to if his or her address is not successfully verified.</p>
    #[serde(rename = "FailureRedirectionURL")]
    pub failure_redirection_url: String,
    /// <p>The email address that the custom verification email is sent from.</p>
    #[serde(rename = "FromEmailAddress")]
    pub from_email_address: String,
    /// <p>The URL that the recipient of the verification email is sent to if his or her address is successfully verified.</p>
    #[serde(rename = "SuccessRedirectionURL")]
    pub success_redirection_url: String,
    /// <p>The content of the custom verification email. The total size of the email must be less than 10 MB. The message body may contain HTML, with some limitations. For more information, see <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/send-email-verify-address-custom.html#custom-verification-emails-faq">Custom Verification Email Frequently Asked Questions</a> in the <i>Amazon SES Developer Guide</i>.</p>
    #[serde(rename = "TemplateContent")]
    pub template_content: String,
    /// <p>The name of the custom verification email template.</p>
    #[serde(rename = "TemplateName")]
    pub template_name: String,
    /// <p>The subject line of the custom verification email.</p>
    #[serde(rename = "TemplateSubject")]
    pub template_subject: String,
}

/// <p>If the action is successful, the service sends back an HTTP 200 response with an empty HTTP body.</p>
/// see [SesV2::create_custom_verification_email_template]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateCustomVerificationEmailTemplateResponse {}

/// <p>A request to create a new dedicated IP pool.</p>
/// see [SesV2::create_dedicated_ip_pool]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateDedicatedIpPoolRequest {
    /// <p>The name of the dedicated IP pool.</p>
    #[serde(rename = "PoolName")]
    pub pool_name: String,
    /// <p>An object that defines the tags (keys and values) that you want to associate with the pool.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// <p>An HTTP 200 response if the request succeeds, or an error message if the request fails.</p>
/// see [SesV2::create_dedicated_ip_pool]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateDedicatedIpPoolResponse {}

/// <p>A request to perform a predictive inbox placement test. Predictive inbox placement tests can help you predict how your messages will be handled by various email providers around the world. When you perform a predictive inbox placement test, you provide a sample message that contains the content that you plan to send to your customers. We send that message to special email addresses spread across several major email providers around the world. The test takes about 24 hours to complete. When the test is complete, you can use the <code>GetDeliverabilityTestReport</code> operation to view the results of the test.</p>
/// see [SesV2::create_deliverability_test_report]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateDeliverabilityTestReportRequest {
    /// <p>The HTML body of the message that you sent when you performed the predictive inbox placement test.</p>
    #[serde(rename = "Content")]
    pub content: EmailContent,
    /// <p>The email address that the predictive inbox placement test email was sent from.</p>
    #[serde(rename = "FromEmailAddress")]
    pub from_email_address: String,
    /// <p>A unique name that helps you to identify the predictive inbox placement test when you retrieve the results.</p>
    #[serde(rename = "ReportName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_name: Option<String>,
    /// <p>An array of objects that define the tags (keys and values) that you want to associate with the predictive inbox placement test.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// <p>Information about the predictive inbox placement test that you created.</p>
/// see [SesV2::create_deliverability_test_report]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateDeliverabilityTestReportResponse {
    /// <p>The status of the predictive inbox placement test. If the status is <code>IN_PROGRESS</code>, then the predictive inbox placement test is currently running. Predictive inbox placement tests are usually complete within 24 hours of creating the test. If the status is <code>COMPLETE</code>, then the test is finished, and you can use the <code>GetDeliverabilityTestReport</code> to view the results of the test.</p>
    #[serde(rename = "DeliverabilityTestStatus")]
    pub deliverability_test_status: String,
    /// <p>A unique string that identifies the predictive inbox placement test.</p>
    #[serde(rename = "ReportId")]
    pub report_id: String,
}

/// <p>Represents a request to create a sending authorization policy for an identity. Sending authorization is an Amazon SES feature that enables you to authorize other senders to use your identities. For information, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/sending-authorization-identity-owner-tasks-management.html">Amazon SES Developer Guide</a>.</p>
/// see [SesV2::create_email_identity_policy]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateEmailIdentityPolicyRequest {
    /// <p>The email identity for which you want to create a policy.</p>
    #[serde(rename = "EmailIdentity")]
    pub email_identity: String,
    /// <p>The text of the policy in JSON format. The policy cannot exceed 4 KB.</p> <p>For information about the syntax of sending authorization policies, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/sending-authorization-policies.html">Amazon SES Developer Guide</a>.</p>
    #[serde(rename = "Policy")]
    pub policy: String,
    /// <p>The name of the policy.</p> <p>The policy name cannot exceed 64 characters and can only include alphanumeric characters, dashes, and underscores.</p>
    #[serde(rename = "PolicyName")]
    pub policy_name: String,
}

/// <p>An HTTP 200 response if the request succeeds, or an error message if the request fails.</p>
/// see [SesV2::create_email_identity_policy]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateEmailIdentityPolicyResponse {}

/// <p>A request to begin the verification process for an email identity (an email address or domain).</p>
/// see [SesV2::create_email_identity]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateEmailIdentityRequest {
    /// <p>If your request includes this object, Amazon SES configures the identity to use Bring Your Own DKIM (BYODKIM) for DKIM authentication purposes, as opposed to the default method, <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/easy-dkim.html">Easy DKIM</a>.</p> <p>You can only specify this object if the email identity is a domain, as opposed to an address.</p>
    #[serde(rename = "DkimSigningAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dkim_signing_attributes: Option<DkimSigningAttributes>,
    /// <p>The email address or domain that you want to verify.</p>
    #[serde(rename = "EmailIdentity")]
    pub email_identity: String,
    /// <p>An array of objects that define the tags (keys and values) that you want to associate with the email identity.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// <p>If the email identity is a domain, this object contains information about the DKIM verification status for the domain.</p> <p>If the email identity is an email address, this object is empty. </p>
/// see [SesV2::create_email_identity]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateEmailIdentityResponse {
    /// <p>An object that contains information about the DKIM attributes for the identity.</p>
    #[serde(rename = "DkimAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dkim_attributes: Option<DkimAttributes>,
    /// <p>The email identity type.</p>
    #[serde(rename = "IdentityType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_type: Option<String>,
    /// <p>Specifies whether or not the identity is verified. You can only send email from verified email addresses or domains. For more information about verifying identities, see the <a href="https://docs.aws.amazon.com/pinpoint/latest/userguide/channels-email-manage-verify.html">Amazon Pinpoint User Guide</a>.</p>
    #[serde(rename = "VerifiedForSendingStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified_for_sending_status: Option<bool>,
}

/// <p>Represents a request to create an email template. For more information, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/send-personalized-email-api.html">Amazon SES Developer Guide</a>.</p>
/// see [SesV2::create_email_template]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateEmailTemplateRequest {
    /// <p>The content of the email template, composed of a subject line, an HTML part, and a text-only part.</p>
    #[serde(rename = "TemplateContent")]
    pub template_content: EmailTemplateContent,
    /// <p>The name of the template you want to create.</p>
    #[serde(rename = "TemplateName")]
    pub template_name: String,
}

/// <p>If the action is successful, the service sends back an HTTP 200 response with an empty HTTP body.</p>
/// see [SesV2::create_email_template]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateEmailTemplateResponse {}

/// <p>Represents a request to create an import job from a data source for a data destination.</p>
/// see [SesV2::create_import_job]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateImportJobRequest {
    /// <p>The data source for the import job.</p>
    #[serde(rename = "ImportDataSource")]
    pub import_data_source: ImportDataSource,
    /// <p>The destination for the import job.</p>
    #[serde(rename = "ImportDestination")]
    pub import_destination: ImportDestination,
}

/// <p>An HTTP 200 response if the request succeeds, or an error message if the request fails.</p>
/// see [SesV2::create_import_job]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateImportJobResponse {
    /// <p>A string that represents the import job ID.</p>
    #[serde(rename = "JobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
}

/// <p>Contains information about a custom verification email template.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CustomVerificationEmailTemplateMetadata {
    /// <p>The URL that the recipient of the verification email is sent to if his or her address is not successfully verified.</p>
    #[serde(rename = "FailureRedirectionURL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_redirection_url: Option<String>,
    /// <p>The email address that the custom verification email is sent from.</p>
    #[serde(rename = "FromEmailAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_email_address: Option<String>,
    /// <p>The URL that the recipient of the verification email is sent to if his or her address is successfully verified.</p>
    #[serde(rename = "SuccessRedirectionURL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_redirection_url: Option<String>,
    /// <p>The name of the custom verification email template.</p>
    #[serde(rename = "TemplateName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_name: Option<String>,
    /// <p>The subject line of the custom verification email.</p>
    #[serde(rename = "TemplateSubject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_subject: Option<String>,
}

/// <p>An object that contains information about the volume of email sent on each day of the analysis period.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DailyVolume {
    /// <p>An object that contains inbox placement metrics for a specified day in the analysis period, broken out by the recipient's email provider.</p>
    #[serde(rename = "DomainIspPlacements")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_isp_placements: Option<Vec<DomainIspPlacement>>,
    /// <p>The date that the DailyVolume metrics apply to, in Unix time.</p>
    #[serde(rename = "StartDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<f64>,
    /// <p>An object that contains inbox placement metrics for a specific day in the analysis period.</p>
    #[serde(rename = "VolumeStatistics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_statistics: Option<VolumeStatistics>,
}

/// <p>Contains information about a dedicated IP address that is associated with your Amazon SES account.</p> <p>To learn more about requesting dedicated IP addresses, see <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/dedicated-ip-case.html">Requesting and Relinquishing Dedicated IP Addresses</a> in the <i>Amazon SES Developer Guide</i>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DedicatedIp {
    /// <p>An IPv4 address.</p>
    #[serde(rename = "Ip")]
    pub ip: String,
    /// <p>The name of the dedicated IP pool that the IP address is associated with.</p>
    #[serde(rename = "PoolName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pool_name: Option<String>,
    /// <p>Indicates how complete the dedicated IP warm-up process is. When this value equals 1, the address has completed the warm-up process and is ready for use.</p>
    #[serde(rename = "WarmupPercentage")]
    pub warmup_percentage: i64,
    /// <p><p>The warm-up status of a dedicated IP address. The status can have one of the following values:</p> <ul> <li> <p> <code>IN_PROGRESS</code> – The IP address isn&#39;t ready to use because the dedicated IP warm-up process is ongoing.</p> </li> <li> <p> <code>DONE</code> – The dedicated IP warm-up process is complete, and the IP address is ready to use.</p> </li> </ul></p>
    #[serde(rename = "WarmupStatus")]
    pub warmup_status: String,
}

/// <p>A request to delete an event destination from a configuration set.</p>
/// see [SesV2::delete_configuration_set_event_destination]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteConfigurationSetEventDestinationRequest {
    /// <p>The name of the configuration set that contains the event destination that you want to delete.</p>
    #[serde(rename = "ConfigurationSetName")]
    pub configuration_set_name: String,
    /// <p>The name of the event destination that you want to delete.</p>
    #[serde(rename = "EventDestinationName")]
    pub event_destination_name: String,
}

/// <p>An HTTP 200 response if the request succeeds, or an error message if the request fails.</p>
/// see [SesV2::delete_configuration_set_event_destination]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteConfigurationSetEventDestinationResponse {}

/// <p>A request to delete a configuration set.</p>
/// see [SesV2::delete_configuration_set]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteConfigurationSetRequest {
    /// <p>The name of the configuration set that you want to delete.</p>
    #[serde(rename = "ConfigurationSetName")]
    pub configuration_set_name: String,
}

/// <p>An HTTP 200 response if the request succeeds, or an error message if the request fails.</p>
/// see [SesV2::delete_configuration_set]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteConfigurationSetResponse {}

/// see [SesV2::delete_contact_list]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteContactListRequest {
    /// <p>The name of the contact list.</p>
    #[serde(rename = "ContactListName")]
    pub contact_list_name: String,
}

/// see [SesV2::delete_contact_list]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteContactListResponse {}

/// see [SesV2::delete_contact]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteContactRequest {
    /// <p>The name of the contact list from which the contact should be removed.</p>
    #[serde(rename = "ContactListName")]
    pub contact_list_name: String,
    /// <p>The contact's email address.</p>
    #[serde(rename = "EmailAddress")]
    pub email_address: String,
}

/// see [SesV2::delete_contact]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteContactResponse {}

/// <p>Represents a request to delete an existing custom verification email template.</p>
/// see [SesV2::delete_custom_verification_email_template]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteCustomVerificationEmailTemplateRequest {
    /// <p>The name of the custom verification email template that you want to delete.</p>
    #[serde(rename = "TemplateName")]
    pub template_name: String,
}

/// <p>If the action is successful, the service sends back an HTTP 200 response with an empty HTTP body.</p>
/// see [SesV2::delete_custom_verification_email_template]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteCustomVerificationEmailTemplateResponse {}

/// <p>A request to delete a dedicated IP pool.</p>
/// see [SesV2::delete_dedicated_ip_pool]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteDedicatedIpPoolRequest {
    /// <p>The name of the dedicated IP pool that you want to delete.</p>
    #[serde(rename = "PoolName")]
    pub pool_name: String,
}

/// <p>An HTTP 200 response if the request succeeds, or an error message if the request fails.</p>
/// see [SesV2::delete_dedicated_ip_pool]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteDedicatedIpPoolResponse {}

/// <p>Represents a request to delete a sending authorization policy for an identity. Sending authorization is an Amazon SES feature that enables you to authorize other senders to use your identities. For information, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/sending-authorization-identity-owner-tasks-management.html">Amazon SES Developer Guide</a>.</p>
/// see [SesV2::delete_email_identity_policy]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteEmailIdentityPolicyRequest {
    /// <p>The email identity for which you want to delete a policy.</p>
    #[serde(rename = "EmailIdentity")]
    pub email_identity: String,
    /// <p>The name of the policy.</p> <p>The policy name cannot exceed 64 characters and can only include alphanumeric characters, dashes, and underscores.</p>
    #[serde(rename = "PolicyName")]
    pub policy_name: String,
}

/// <p>An HTTP 200 response if the request succeeds, or an error message if the request fails.</p>
/// see [SesV2::delete_email_identity_policy]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteEmailIdentityPolicyResponse {}

/// <p>A request to delete an existing email identity. When you delete an identity, you lose the ability to send email from that identity. You can restore your ability to send email by completing the verification process for the identity again.</p>
/// see [SesV2::delete_email_identity]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteEmailIdentityRequest {
    /// <p>The identity (that is, the email address or domain) that you want to delete.</p>
    #[serde(rename = "EmailIdentity")]
    pub email_identity: String,
}

/// <p>An HTTP 200 response if the request succeeds, or an error message if the request fails.</p>
/// see [SesV2::delete_email_identity]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteEmailIdentityResponse {}

/// <p>Represents a request to delete an email template. For more information, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/send-personalized-email-api.html">Amazon SES Developer Guide</a>.</p>
/// see [SesV2::delete_email_template]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteEmailTemplateRequest {
    /// <p>The name of the template to be deleted.</p>
    #[serde(rename = "TemplateName")]
    pub template_name: String,
}

/// <p>If the action is successful, the service sends back an HTTP 200 response with an empty HTTP body.</p>
/// see [SesV2::delete_email_template]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteEmailTemplateResponse {}

/// <p>A request to remove an email address from the suppression list for your account.</p>
/// see [SesV2::delete_suppressed_destination]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteSuppressedDestinationRequest {
    /// <p>The suppressed email destination to remove from the account suppression list.</p>
    #[serde(rename = "EmailAddress")]
    pub email_address: String,
}

/// <p>An HTTP 200 response if the request succeeds, or an error message if the request fails.</p>
/// see [SesV2::delete_suppressed_destination]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteSuppressedDestinationResponse {}

/// <p>An object that contains metadata related to a predictive inbox placement test.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeliverabilityTestReport {
    /// <p>The date and time when the predictive inbox placement test was created, in Unix time format.</p>
    #[serde(rename = "CreateDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<f64>,
    /// <p>The status of the predictive inbox placement test. If the status is <code>IN_PROGRESS</code>, then the predictive inbox placement test is currently running. Predictive inbox placement tests are usually complete within 24 hours of creating the test. If the status is <code>COMPLETE</code>, then the test is finished, and you can use the <code>GetDeliverabilityTestReport</code> to view the results of the test.</p>
    #[serde(rename = "DeliverabilityTestStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deliverability_test_status: Option<String>,
    /// <p>The sender address that you specified for the predictive inbox placement test.</p>
    #[serde(rename = "FromEmailAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_email_address: Option<String>,
    /// <p>A unique string that identifies the predictive inbox placement test.</p>
    #[serde(rename = "ReportId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_id: Option<String>,
    /// <p>A name that helps you identify a predictive inbox placement test report.</p>
    #[serde(rename = "ReportName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_name: Option<String>,
    /// <p>The subject line for an email that you submitted in a predictive inbox placement test.</p>
    #[serde(rename = "Subject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
}

/// <p>Used to associate a configuration set with a dedicated IP pool.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct DeliveryOptions {
    /// <p>The name of the dedicated IP pool that you want to associate with the configuration set.</p>
    #[serde(rename = "SendingPoolName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sending_pool_name: Option<String>,
    /// <p>Specifies whether messages that use the configuration set are required to use Transport Layer Security (TLS). If the value is <code>Require</code>, messages are only delivered if a TLS connection can be established. If the value is <code>Optional</code>, messages can be delivered in plain text if a TLS connection can't be established.</p>
    #[serde(rename = "TlsPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls_policy: Option<String>,
}

/// <p>An object that describes the recipients for an email.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Destination {
    /// <p>An array that contains the email addresses of the "BCC" (blind carbon copy) recipients for the email.</p>
    #[serde(rename = "BccAddresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bcc_addresses: Option<Vec<String>>,
    /// <p>An array that contains the email addresses of the "CC" (carbon copy) recipients for the email.</p>
    #[serde(rename = "CcAddresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cc_addresses: Option<Vec<String>>,
    /// <p>An array that contains the email addresses of the "To" recipients for the email.</p>
    #[serde(rename = "ToAddresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_addresses: Option<Vec<String>>,
}

/// <p>An object that contains information about the DKIM authentication status for an email identity.</p> <p>Amazon SES determines the authentication status by searching for specific records in the DNS configuration for the domain. If you used <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/easy-dkim.html">Easy DKIM</a> to set up DKIM authentication, Amazon SES tries to find three unique CNAME records in the DNS configuration for your domain. If you provided a public key to perform DKIM authentication, Amazon SES tries to find a TXT record that uses the selector that you specified. The value of the TXT record must be a public key that's paired with the private key that you specified in the process of creating the identity</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DkimAttributes {
    /// <p><p>A string that indicates how DKIM was configured for the identity. There are two possible values:</p> <ul> <li> <p> <code>AWS_SES</code> – Indicates that DKIM was configured for the identity by using <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/easy-dkim.html">Easy DKIM</a>.</p> </li> <li> <p> <code>EXTERNAL</code> – Indicates that DKIM was configured for the identity by using Bring Your Own DKIM (BYODKIM).</p> </li> </ul></p>
    #[serde(rename = "SigningAttributesOrigin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_attributes_origin: Option<String>,
    /// <p>If the value is <code>true</code>, then the messages that you send from the identity are signed using DKIM. If the value is <code>false</code>, then the messages that you send from the identity aren't DKIM-signed.</p>
    #[serde(rename = "SigningEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_enabled: Option<bool>,
    /// <p><p>Describes whether or not Amazon SES has successfully located the DKIM records in the DNS records for the domain. The status can be one of the following:</p> <ul> <li> <p> <code>PENDING</code> – The verification process was initiated, but Amazon SES hasn&#39;t yet detected the DKIM records in the DNS configuration for the domain.</p> </li> <li> <p> <code>SUCCESS</code> – The verification process completed successfully.</p> </li> <li> <p> <code>FAILED</code> – The verification process failed. This typically occurs when Amazon SES fails to find the DKIM records in the DNS configuration of the domain.</p> </li> <li> <p> <code>TEMPORARY<em>FAILURE</code> – A temporary issue is preventing Amazon SES from determining the DKIM authentication status of the domain.</p> </li> <li> <p> <code>NOT</em>STARTED</code> – The DKIM verification process hasn&#39;t been initiated for the domain.</p> </li> </ul></p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>If you used <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/easy-dkim.html">Easy DKIM</a> to configure DKIM authentication for the domain, then this object contains a set of unique strings that you use to create a set of CNAME records that you add to the DNS configuration for your domain. When Amazon SES detects these records in the DNS configuration for your domain, the DKIM authentication process is complete.</p> <p>If you configured DKIM authentication for the domain by providing your own public-private key pair, then this object contains the selector for the public key.</p> <p>Regardless of the DKIM authentication method you use, Amazon SES searches for the appropriate records in the DNS configuration of the domain for up to 72 hours.</p>
    #[serde(rename = "Tokens")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tokens: Option<Vec<String>>,
}

/// <p>An object that contains information about the tokens used for setting up Bring Your Own DKIM (BYODKIM).</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DkimSigningAttributes {
    /// <p>A private key that's used to generate a DKIM signature.</p> <p>The private key must use 1024-bit RSA encryption, and must be encoded using base64 encoding.</p>
    #[serde(rename = "DomainSigningPrivateKey")]
    pub domain_signing_private_key: String,
    /// <p>A string that's used to identify a public key in the DNS configuration for a domain.</p>
    #[serde(rename = "DomainSigningSelector")]
    pub domain_signing_selector: String,
}

/// <p>An object that contains the deliverability data for a specific campaign. This data is available for a campaign only if the campaign sent email by using a domain that the Deliverability dashboard is enabled for (<code>PutDeliverabilityDashboardOption</code> operation).</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DomainDeliverabilityCampaign {
    /// <p>The unique identifier for the campaign. The Deliverability dashboard automatically generates and assigns this identifier to a campaign.</p>
    #[serde(rename = "CampaignId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_id: Option<String>,
    /// <p>The percentage of email messages that were deleted by recipients, without being opened first. Due to technical limitations, this value only includes recipients who opened the message by using an email client that supports images.</p>
    #[serde(rename = "DeleteRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_rate: Option<f64>,
    /// <p>The major email providers who handled the email message.</p>
    #[serde(rename = "Esps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub esps: Option<Vec<String>>,
    /// <p>The first time, in Unix time format, when the email message was delivered to any recipient's inbox. This value can help you determine how long it took for a campaign to deliver an email message.</p>
    #[serde(rename = "FirstSeenDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_seen_date_time: Option<f64>,
    /// <p>The verified email address that the email message was sent from.</p>
    #[serde(rename = "FromAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_address: Option<String>,
    /// <p>The URL of an image that contains a snapshot of the email message that was sent.</p>
    #[serde(rename = "ImageUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    /// <p>The number of email messages that were delivered to recipients’ inboxes.</p>
    #[serde(rename = "InboxCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbox_count: Option<i64>,
    /// <p>The last time, in Unix time format, when the email message was delivered to any recipient's inbox. This value can help you determine how long it took for a campaign to deliver an email message.</p>
    #[serde(rename = "LastSeenDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_seen_date_time: Option<f64>,
    /// <p>The projected number of recipients that the email message was sent to.</p>
    #[serde(rename = "ProjectedVolume")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projected_volume: Option<i64>,
    /// <p>The percentage of email messages that were opened and then deleted by recipients. Due to technical limitations, this value only includes recipients who opened the message by using an email client that supports images.</p>
    #[serde(rename = "ReadDeleteRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_delete_rate: Option<f64>,
    /// <p>The percentage of email messages that were opened by recipients. Due to technical limitations, this value only includes recipients who opened the message by using an email client that supports images.</p>
    #[serde(rename = "ReadRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_rate: Option<f64>,
    /// <p>The IP addresses that were used to send the email message.</p>
    #[serde(rename = "SendingIps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sending_ips: Option<Vec<String>>,
    /// <p>The number of email messages that were delivered to recipients' spam or junk mail folders.</p>
    #[serde(rename = "SpamCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spam_count: Option<i64>,
    /// <p>The subject line, or title, of the email message.</p>
    #[serde(rename = "Subject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
}

/// <p>An object that contains information about the Deliverability dashboard subscription for a verified domain that you use to send email and currently has an active Deliverability dashboard subscription. If a Deliverability dashboard subscription is active for a domain, you gain access to reputation, inbox placement, and other metrics for the domain.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct DomainDeliverabilityTrackingOption {
    /// <p>A verified domain that’s associated with your AWS account and currently has an active Deliverability dashboard subscription.</p>
    #[serde(rename = "Domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// <p>An object that contains information about the inbox placement data settings for the domain.</p>
    #[serde(rename = "InboxPlacementTrackingOption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbox_placement_tracking_option: Option<InboxPlacementTrackingOption>,
    /// <p>The date, in Unix time format, when you enabled the Deliverability dashboard for the domain.</p>
    #[serde(rename = "SubscriptionStartDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_start_date: Option<f64>,
}

/// <p>An object that contains inbox placement data for email sent from one of your email domains to a specific email provider.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DomainIspPlacement {
    /// <p>The percentage of messages that were sent from the selected domain to the specified email provider that arrived in recipients' inboxes.</p>
    #[serde(rename = "InboxPercentage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbox_percentage: Option<f64>,
    /// <p>The total number of messages that were sent from the selected domain to the specified email provider that arrived in recipients' inboxes.</p>
    #[serde(rename = "InboxRawCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbox_raw_count: Option<i64>,
    /// <p>The name of the email provider that the inbox placement data applies to.</p>
    #[serde(rename = "IspName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isp_name: Option<String>,
    /// <p>The percentage of messages that were sent from the selected domain to the specified email provider that arrived in recipients' spam or junk mail folders.</p>
    #[serde(rename = "SpamPercentage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spam_percentage: Option<f64>,
    /// <p>The total number of messages that were sent from the selected domain to the specified email provider that arrived in recipients' spam or junk mail folders.</p>
    #[serde(rename = "SpamRawCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spam_raw_count: Option<i64>,
}

/// <p>An object that defines the entire content of the email, including the message headers and the body content. You can create a simple email message, in which you specify the subject and the text and HTML versions of the message body. You can also create raw messages, in which you specify a complete MIME-formatted message. Raw messages can include attachments and custom headers.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct EmailContent {
    /// <p><p>The raw email message. The message has to meet the following criteria:</p> <ul> <li> <p>The message has to contain a header and a body, separated by one blank line.</p> </li> <li> <p>All of the required header fields must be present in the message.</p> </li> <li> <p>Each part of a multipart MIME message must be formatted properly.</p> </li> <li> <p>If you include attachments, they must be in a file format that the Amazon SES API v2 supports. </p> </li> <li> <p>The entire message must be Base64 encoded.</p> </li> <li> <p>If any of the MIME parts in your message contain content that is outside of the 7-bit ASCII character range, you should encode that content to ensure that recipients&#39; email clients render the message properly.</p> </li> <li> <p>The length of any single line of text in the message can&#39;t exceed 1,000 characters. This restriction is defined in <a href="https://tools.ietf.org/html/rfc5321">RFC 5321</a>.</p> </li> </ul></p>
    #[serde(rename = "Raw")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw: Option<RawMessage>,
    /// <p>The simple email message. The message consists of a subject and a message body.</p>
    #[serde(rename = "Simple")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simple: Option<Message>,
    /// <p>The template to use for the email message.</p>
    #[serde(rename = "Template")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<Template>,
}

/// <p>The content of the email, composed of a subject line, an HTML part, and a text-only part.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct EmailTemplateContent {
    /// <p>The HTML body of the email.</p>
    #[serde(rename = "Html")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html: Option<String>,
    /// <p>The subject line of the email.</p>
    #[serde(rename = "Subject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    /// <p>The email body that will be visible to recipients whose email clients do not display HTML.</p>
    #[serde(rename = "Text")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

/// <p>Contains information about an email template.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct EmailTemplateMetadata {
    /// <p>The time and date the template was created.</p>
    #[serde(rename = "CreatedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_timestamp: Option<f64>,
    /// <p>The name of the template.</p>
    #[serde(rename = "TemplateName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_name: Option<String>,
}

/// <p>In the Amazon SES API v2, <i>events</i> include message sends, deliveries, opens, clicks, bounces, complaints and delivery delays. <i>Event destinations</i> are places that you can send information about these events to. For example, you can send event data to Amazon SNS to receive notifications when you receive bounces or complaints, or you can use Amazon Kinesis Data Firehose to stream data to Amazon S3 for long-term storage.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct EventDestination {
    /// <p>An object that defines an Amazon CloudWatch destination for email events. You can use Amazon CloudWatch to monitor and gain insights on your email sending metrics.</p>
    #[serde(rename = "CloudWatchDestination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_destination: Option<CloudWatchDestination>,
    /// <p>If <code>true</code>, the event destination is enabled. When the event destination is enabled, the specified event types are sent to the destinations in this <code>EventDestinationDefinition</code>.</p> <p>If <code>false</code>, the event destination is disabled. When the event destination is disabled, events aren't sent to the specified destinations.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>An object that defines an Amazon Kinesis Data Firehose destination for email events. You can use Amazon Kinesis Data Firehose to stream data to other services, such as Amazon S3 and Amazon Redshift.</p>
    #[serde(rename = "KinesisFirehoseDestination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis_firehose_destination: Option<KinesisFirehoseDestination>,
    /// <p>The types of events that Amazon SES sends to the specified event destinations.</p>
    #[serde(rename = "MatchingEventTypes")]
    pub matching_event_types: Vec<String>,
    /// <p>A name that identifies the event destination.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>An object that defines an Amazon Pinpoint project destination for email events. You can send email event data to a Amazon Pinpoint project to view metrics using the Transactional Messaging dashboards that are built in to Amazon Pinpoint. For more information, see <a href="https://docs.aws.amazon.com/pinpoint/latest/userguide/analytics-transactional-messages.html">Transactional Messaging Charts</a> in the <i>Amazon Pinpoint User Guide</i>.</p>
    #[serde(rename = "PinpointDestination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pinpoint_destination: Option<PinpointDestination>,
    /// <p>An object that defines an Amazon SNS destination for email events. You can use Amazon SNS to send notification when certain email events occur.</p>
    #[serde(rename = "SnsDestination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_destination: Option<SnsDestination>,
}

/// <p>An object that defines the event destination. Specifically, it defines which services receive events from emails sent using the configuration set that the event destination is associated with. Also defines the types of events that are sent to the event destination.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct EventDestinationDefinition {
    /// <p>An object that defines an Amazon CloudWatch destination for email events. You can use Amazon CloudWatch to monitor and gain insights on your email sending metrics.</p>
    #[serde(rename = "CloudWatchDestination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_destination: Option<CloudWatchDestination>,
    /// <p>If <code>true</code>, the event destination is enabled. When the event destination is enabled, the specified event types are sent to the destinations in this <code>EventDestinationDefinition</code>.</p> <p>If <code>false</code>, the event destination is disabled. When the event destination is disabled, events aren't sent to the specified destinations.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>An object that defines an Amazon Kinesis Data Firehose destination for email events. You can use Amazon Kinesis Data Firehose to stream data to other services, such as Amazon S3 and Amazon Redshift.</p>
    #[serde(rename = "KinesisFirehoseDestination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis_firehose_destination: Option<KinesisFirehoseDestination>,
    /// <p>An array that specifies which events the Amazon SES API v2 should send to the destinations in this <code>EventDestinationDefinition</code>.</p>
    #[serde(rename = "MatchingEventTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matching_event_types: Option<Vec<String>>,
    /// <p>An object that defines an Amazon Pinpoint project destination for email events. You can send email event data to a Amazon Pinpoint project to view metrics using the Transactional Messaging dashboards that are built in to Amazon Pinpoint. For more information, see <a href="https://docs.aws.amazon.com/pinpoint/latest/userguide/analytics-transactional-messages.html">Transactional Messaging Charts</a> in the <i>Amazon Pinpoint User Guide</i>.</p>
    #[serde(rename = "PinpointDestination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pinpoint_destination: Option<PinpointDestination>,
    /// <p>An object that defines an Amazon SNS destination for email events. You can use Amazon SNS to send notification when certain email events occur.</p>
    #[serde(rename = "SnsDestination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_destination: Option<SnsDestination>,
}

/// <p>An object that contains the failure details about an import job.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct FailureInfo {
    /// <p>A message about why the import job failed.</p>
    #[serde(rename = "ErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>An Amazon S3 presigned URL that contains all the failed records and related information.</p>
    #[serde(rename = "FailedRecordsS3Url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_records_s3_url: Option<String>,
}

/// <p>A request to obtain information about the email-sending capabilities of your Amazon SES account.</p>
/// see [SesV2::get_account]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetAccountRequest {}

/// <p>A list of details about the email-sending capabilities of your Amazon SES account in the current AWS Region.</p>
/// see [SesV2::get_account]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetAccountResponse {
    /// <p>Indicates whether or not the automatic warm-up feature is enabled for dedicated IP addresses that are associated with your account.</p>
    #[serde(rename = "DedicatedIpAutoWarmupEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dedicated_ip_auto_warmup_enabled: Option<bool>,
    /// <p>An object that defines your account details.</p>
    #[serde(rename = "Details")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<AccountDetails>,
    /// <p><p>The reputation status of your Amazon SES account. The status can be one of the following:</p> <ul> <li> <p> <code>HEALTHY</code> – There are no reputation-related issues that currently impact your account.</p> </li> <li> <p> <code>PROBATION</code> – We&#39;ve identified potential issues with your Amazon SES account. We&#39;re placing your account under review while you work on correcting these issues.</p> </li> <li> <p> <code>SHUTDOWN</code> – Your account&#39;s ability to send email is currently paused because of an issue with the email sent from your account. When you correct the issue, you can contact us and request that your account&#39;s ability to send email is resumed.</p> </li> </ul></p>
    #[serde(rename = "EnforcementStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enforcement_status: Option<String>,
    /// <p>Indicates whether or not your account has production access in the current AWS Region.</p> <p>If the value is <code>false</code>, then your account is in the <i>sandbox</i>. When your account is in the sandbox, you can only send email to verified identities. Additionally, the maximum number of emails you can send in a 24-hour period (your sending quota) is 200, and the maximum number of emails you can send per second (your maximum sending rate) is 1.</p> <p>If the value is <code>true</code>, then your account has production access. When your account has production access, you can send email to any address. The sending quota and maximum sending rate for your account vary based on your specific use case.</p>
    #[serde(rename = "ProductionAccessEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub production_access_enabled: Option<bool>,
    /// <p>An object that contains information about the per-day and per-second sending limits for your Amazon SES account in the current AWS Region.</p>
    #[serde(rename = "SendQuota")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_quota: Option<SendQuota>,
    /// <p>Indicates whether or not email sending is enabled for your Amazon SES account in the current AWS Region.</p>
    #[serde(rename = "SendingEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sending_enabled: Option<bool>,
    /// <p>An object that contains information about the email address suppression preferences for your account in the current AWS Region.</p>
    #[serde(rename = "SuppressionAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suppression_attributes: Option<SuppressionAttributes>,
}

/// <p>A request to retrieve a list of the blacklists that your dedicated IP addresses appear on.</p>
/// see [SesV2::get_blacklist_reports]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetBlacklistReportsRequest {
    /// <p>A list of IP addresses that you want to retrieve blacklist information about. You can only specify the dedicated IP addresses that you use to send email using Amazon SES or Amazon Pinpoint.</p>
    #[serde(rename = "BlacklistItemNames")]
    pub blacklist_item_names: Vec<String>,
}

/// <p>An object that contains information about blacklist events.</p>
/// see [SesV2::get_blacklist_reports]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetBlacklistReportsResponse {
    /// <p>An object that contains information about a blacklist that one of your dedicated IP addresses appears on.</p>
    #[serde(rename = "BlacklistReport")]
    pub blacklist_report: ::std::collections::HashMap<String, Vec<BlacklistEntry>>,
}

/// <p>A request to obtain information about the event destinations for a configuration set.</p>
/// see [SesV2::get_configuration_set_event_destinations]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetConfigurationSetEventDestinationsRequest {
    /// <p>The name of the configuration set that contains the event destination.</p>
    #[serde(rename = "ConfigurationSetName")]
    pub configuration_set_name: String,
}

/// <p>Information about an event destination for a configuration set.</p>
/// see [SesV2::get_configuration_set_event_destinations]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetConfigurationSetEventDestinationsResponse {
    /// <p>An array that includes all of the events destinations that have been configured for the configuration set.</p>
    #[serde(rename = "EventDestinations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_destinations: Option<Vec<EventDestination>>,
}

/// <p>A request to obtain information about a configuration set.</p>
/// see [SesV2::get_configuration_set]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetConfigurationSetRequest {
    /// <p>The name of the configuration set that you want to obtain more information about.</p>
    #[serde(rename = "ConfigurationSetName")]
    pub configuration_set_name: String,
}

/// <p>Information about a configuration set.</p>
/// see [SesV2::get_configuration_set]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetConfigurationSetResponse {
    /// <p>The name of the configuration set.</p>
    #[serde(rename = "ConfigurationSetName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_set_name: Option<String>,
    /// <p>An object that defines the dedicated IP pool that is used to send emails that you send using the configuration set.</p>
    #[serde(rename = "DeliveryOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_options: Option<DeliveryOptions>,
    /// <p>An object that defines whether or not Amazon SES collects reputation metrics for the emails that you send that use the configuration set.</p>
    #[serde(rename = "ReputationOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reputation_options: Option<ReputationOptions>,
    /// <p>An object that defines whether or not Amazon SES can send email that you send using the configuration set.</p>
    #[serde(rename = "SendingOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sending_options: Option<SendingOptions>,
    /// <p>An object that contains information about the suppression list preferences for your account.</p>
    #[serde(rename = "SuppressionOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suppression_options: Option<SuppressionOptions>,
    /// <p>An array of objects that define the tags (keys and values) that are associated with the configuration set.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>An object that defines the open and click tracking options for emails that you send using the configuration set.</p>
    #[serde(rename = "TrackingOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_options: Option<TrackingOptions>,
}

/// see [SesV2::get_contact_list]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetContactListRequest {
    /// <p>The name of the contact list.</p>
    #[serde(rename = "ContactListName")]
    pub contact_list_name: String,
}

/// see [SesV2::get_contact_list]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetContactListResponse {
    /// <p>The name of the contact list.</p>
    #[serde(rename = "ContactListName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_list_name: Option<String>,
    /// <p>A timestamp noting when the contact list was created.</p>
    #[serde(rename = "CreatedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_timestamp: Option<f64>,
    /// <p>A description of what the contact list is about.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>A timestamp noting the last time the contact list was updated.</p>
    #[serde(rename = "LastUpdatedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_timestamp: Option<f64>,
    /// <p>The tags associated with a contact list.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>An interest group, theme, or label within a list. A contact list can have multiple topics.</p>
    #[serde(rename = "Topics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topics: Option<Vec<Topic>>,
}

/// see [SesV2::get_contact]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetContactRequest {
    /// <p>The name of the contact list to which the contact belongs.</p>
    #[serde(rename = "ContactListName")]
    pub contact_list_name: String,
    /// <p>The contact's email addres.</p>
    #[serde(rename = "EmailAddress")]
    pub email_address: String,
}

/// see [SesV2::get_contact]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetContactResponse {
    /// <p>The attribute data attached to a contact.</p>
    #[serde(rename = "AttributesData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes_data: Option<String>,
    /// <p>The name of the contact list to which the contact belongs.</p>
    #[serde(rename = "ContactListName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_list_name: Option<String>,
    /// <p>A timestamp noting when the contact was created.</p>
    #[serde(rename = "CreatedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_timestamp: Option<f64>,
    /// <p>The contact's email addres.</p>
    #[serde(rename = "EmailAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    /// <p>A timestamp noting the last time the contact's information was updated.</p>
    #[serde(rename = "LastUpdatedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_timestamp: Option<f64>,
    /// <p>The default topic preferences applied to the contact.</p>
    #[serde(rename = "TopicDefaultPreferences")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_default_preferences: Option<Vec<TopicPreference>>,
    /// <p>The contact's preference for being opted-in to or opted-out of a topic.&gt;</p>
    #[serde(rename = "TopicPreferences")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_preferences: Option<Vec<TopicPreference>>,
    /// <p>A boolean value status noting if the contact is unsubscribed from all contact list topics.</p>
    #[serde(rename = "UnsubscribeAll")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unsubscribe_all: Option<bool>,
}

/// <p>Represents a request to retrieve an existing custom verification email template.</p>
/// see [SesV2::get_custom_verification_email_template]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetCustomVerificationEmailTemplateRequest {
    /// <p>The name of the custom verification email template that you want to retrieve.</p>
    #[serde(rename = "TemplateName")]
    pub template_name: String,
}

/// <p>The following elements are returned by the service.</p>
/// see [SesV2::get_custom_verification_email_template]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetCustomVerificationEmailTemplateResponse {
    /// <p>The URL that the recipient of the verification email is sent to if his or her address is not successfully verified.</p>
    #[serde(rename = "FailureRedirectionURL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_redirection_url: Option<String>,
    /// <p>The email address that the custom verification email is sent from.</p>
    #[serde(rename = "FromEmailAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_email_address: Option<String>,
    /// <p>The URL that the recipient of the verification email is sent to if his or her address is successfully verified.</p>
    #[serde(rename = "SuccessRedirectionURL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_redirection_url: Option<String>,
    /// <p>The content of the custom verification email.</p>
    #[serde(rename = "TemplateContent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_content: Option<String>,
    /// <p>The name of the custom verification email template.</p>
    #[serde(rename = "TemplateName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_name: Option<String>,
    /// <p>The subject line of the custom verification email.</p>
    #[serde(rename = "TemplateSubject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_subject: Option<String>,
}

/// <p>A request to obtain more information about a dedicated IP address.</p>
/// see [SesV2::get_dedicated_ip]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetDedicatedIpRequest {
    /// <p>The IP address that you want to obtain more information about. The value you specify has to be a dedicated IP address that's assocaited with your AWS account.</p>
    #[serde(rename = "Ip")]
    pub ip: String,
}

/// <p>Information about a dedicated IP address.</p>
/// see [SesV2::get_dedicated_ip]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetDedicatedIpResponse {
    /// <p>An object that contains information about a dedicated IP address.</p>
    #[serde(rename = "DedicatedIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dedicated_ip: Option<DedicatedIp>,
}

/// <p>A request to obtain more information about dedicated IP pools.</p>
/// see [SesV2::get_dedicated_ips]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetDedicatedIpsRequest {
    /// <p>A token returned from a previous call to <code>GetDedicatedIps</code> to indicate the position of the dedicated IP pool in the list of IP pools.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The number of results to show in a single call to <code>GetDedicatedIpsRequest</code>. If the number of results is larger than the number you specified in this parameter, then the response includes a <code>NextToken</code> element, which you can use to obtain additional results.</p>
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// <p>The name of the IP pool that the dedicated IP address is associated with.</p>
    #[serde(rename = "PoolName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pool_name: Option<String>,
}

/// <p>Information about the dedicated IP addresses that are associated with your AWS account.</p>
/// see [SesV2::get_dedicated_ips]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetDedicatedIpsResponse {
    /// <p>A list of dedicated IP addresses that are associated with your AWS account.</p>
    #[serde(rename = "DedicatedIps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dedicated_ips: Option<Vec<DedicatedIp>>,
    /// <p>A token that indicates that there are additional dedicated IP addresses to list. To view additional addresses, issue another request to <code>GetDedicatedIps</code>, passing this token in the <code>NextToken</code> parameter.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Retrieve information about the status of the Deliverability dashboard for your AWS account. When the Deliverability dashboard is enabled, you gain access to reputation, deliverability, and other metrics for your domains. You also gain the ability to perform predictive inbox placement tests.</p> <p>When you use the Deliverability dashboard, you pay a monthly subscription charge, in addition to any other fees that you accrue by using Amazon SES and other AWS services. For more information about the features and cost of a Deliverability dashboard subscription, see <a href="http://aws.amazon.com/pinpoint/pricing/">Amazon Pinpoint Pricing</a>.</p>
/// see [SesV2::get_deliverability_dashboard_options]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetDeliverabilityDashboardOptionsRequest {}

/// <p>An object that shows the status of the Deliverability dashboard.</p>
/// see [SesV2::get_deliverability_dashboard_options]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetDeliverabilityDashboardOptionsResponse {
    /// <p>The current status of your Deliverability dashboard subscription. If this value is <code>PENDING_EXPIRATION</code>, your subscription is scheduled to expire at the end of the current calendar month.</p>
    #[serde(rename = "AccountStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_status: Option<String>,
    /// <p>An array of objects, one for each verified domain that you use to send email and currently has an active Deliverability dashboard subscription that isn’t scheduled to expire at the end of the current calendar month.</p>
    #[serde(rename = "ActiveSubscribedDomains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_subscribed_domains: Option<Vec<DomainDeliverabilityTrackingOption>>,
    /// <p>Specifies whether the Deliverability dashboard is enabled. If this value is <code>true</code>, the dashboard is enabled.</p>
    #[serde(rename = "DashboardEnabled")]
    pub dashboard_enabled: bool,
    /// <p>An array of objects, one for each verified domain that you use to send email and currently has an active Deliverability dashboard subscription that's scheduled to expire at the end of the current calendar month.</p>
    #[serde(rename = "PendingExpirationSubscribedDomains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_expiration_subscribed_domains: Option<Vec<DomainDeliverabilityTrackingOption>>,
    /// <p>The date, in Unix time format, when your current subscription to the Deliverability dashboard is scheduled to expire, if your subscription is scheduled to expire at the end of the current calendar month. This value is null if you have an active subscription that isn’t due to expire at the end of the month.</p>
    #[serde(rename = "SubscriptionExpiryDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_expiry_date: Option<f64>,
}

/// <p>A request to retrieve the results of a predictive inbox placement test.</p>
/// see [SesV2::get_deliverability_test_report]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetDeliverabilityTestReportRequest {
    /// <p>A unique string that identifies the predictive inbox placement test.</p>
    #[serde(rename = "ReportId")]
    pub report_id: String,
}

/// <p>The results of the predictive inbox placement test.</p>
/// see [SesV2::get_deliverability_test_report]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetDeliverabilityTestReportResponse {
    /// <p>An object that contains the results of the predictive inbox placement test.</p>
    #[serde(rename = "DeliverabilityTestReport")]
    pub deliverability_test_report: DeliverabilityTestReport,
    /// <p>An object that describes how the test email was handled by several email providers, including Gmail, Hotmail, Yahoo, AOL, and others.</p>
    #[serde(rename = "IspPlacements")]
    pub isp_placements: Vec<IspPlacement>,
    /// <p>An object that contains the message that you sent when you performed this predictive inbox placement test.</p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p>An object that specifies how many test messages that were sent during the predictive inbox placement test were delivered to recipients' inboxes, how many were sent to recipients' spam folders, and how many weren't delivered.</p>
    #[serde(rename = "OverallPlacement")]
    pub overall_placement: PlacementStatistics,
    /// <p>An array of objects that define the tags (keys and values) that are associated with the predictive inbox placement test.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// <p>Retrieve all the deliverability data for a specific campaign. This data is available for a campaign only if the campaign sent email by using a domain that the Deliverability dashboard is enabled for (<code>PutDeliverabilityDashboardOption</code> operation).</p>
/// see [SesV2::get_domain_deliverability_campaign]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetDomainDeliverabilityCampaignRequest {
    /// <p>The unique identifier for the campaign. The Deliverability dashboard automatically generates and assigns this identifier to a campaign.</p>
    #[serde(rename = "CampaignId")]
    pub campaign_id: String,
}

/// <p>An object that contains all the deliverability data for a specific campaign. This data is available for a campaign only if the campaign sent email by using a domain that the Deliverability dashboard is enabled for.</p>
/// see [SesV2::get_domain_deliverability_campaign]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetDomainDeliverabilityCampaignResponse {
    /// <p>An object that contains the deliverability data for the campaign.</p>
    #[serde(rename = "DomainDeliverabilityCampaign")]
    pub domain_deliverability_campaign: DomainDeliverabilityCampaign,
}

/// <p>A request to obtain deliverability metrics for a domain.</p>
/// see [SesV2::get_domain_statistics_report]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetDomainStatisticsReportRequest {
    /// <p>The domain that you want to obtain deliverability metrics for.</p>
    #[serde(rename = "Domain")]
    pub domain: String,
    /// <p>The last day (in Unix time) that you want to obtain domain deliverability metrics for. The <code>EndDate</code> that you specify has to be less than or equal to 30 days after the <code>StartDate</code>.</p>
    #[serde(rename = "EndDate")]
    pub end_date: f64,
    /// <p>The first day (in Unix time) that you want to obtain domain deliverability metrics for.</p>
    #[serde(rename = "StartDate")]
    pub start_date: f64,
}

/// <p>An object that includes statistics that are related to the domain that you specified.</p>
/// see [SesV2::get_domain_statistics_report]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetDomainStatisticsReportResponse {
    /// <p>An object that contains deliverability metrics for the domain that you specified. This object contains data for each day, starting on the <code>StartDate</code> and ending on the <code>EndDate</code>.</p>
    #[serde(rename = "DailyVolumes")]
    pub daily_volumes: Vec<DailyVolume>,
    /// <p>An object that contains deliverability metrics for the domain that you specified. The data in this object is a summary of all of the data that was collected from the <code>StartDate</code> to the <code>EndDate</code>.</p>
    #[serde(rename = "OverallVolume")]
    pub overall_volume: OverallVolume,
}

/// <p>A request to return the policies of an email identity.</p>
/// see [SesV2::get_email_identity_policies]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetEmailIdentityPoliciesRequest {
    /// <p>The email identity that you want to retrieve policies for.</p>
    #[serde(rename = "EmailIdentity")]
    pub email_identity: String,
}

/// <p>Identity policies associated with email identity.</p>
/// see [SesV2::get_email_identity_policies]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetEmailIdentityPoliciesResponse {
    /// <p>A map of policy names to policies.</p>
    #[serde(rename = "Policies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policies: Option<::std::collections::HashMap<String, String>>,
}

/// <p>A request to return details about an email identity.</p>
/// see [SesV2::get_email_identity]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetEmailIdentityRequest {
    /// <p>The email identity that you want to retrieve details for.</p>
    #[serde(rename = "EmailIdentity")]
    pub email_identity: String,
}

/// <p>Details about an email identity.</p>
/// see [SesV2::get_email_identity]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetEmailIdentityResponse {
    /// <p>An object that contains information about the DKIM attributes for the identity.</p>
    #[serde(rename = "DkimAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dkim_attributes: Option<DkimAttributes>,
    /// <p>The feedback forwarding configuration for the identity.</p> <p>If the value is <code>true</code>, you receive email notifications when bounce or complaint events occur. These notifications are sent to the address that you specified in the <code>Return-Path</code> header of the original email.</p> <p>You're required to have a method of tracking bounces and complaints. If you haven't set up another mechanism for receiving bounce or complaint notifications (for example, by setting up an event destination), you receive an email notification when these events occur (even if this setting is disabled).</p>
    #[serde(rename = "FeedbackForwardingStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feedback_forwarding_status: Option<bool>,
    /// <p>The email identity type.</p>
    #[serde(rename = "IdentityType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_type: Option<String>,
    /// <p>An object that contains information about the Mail-From attributes for the email identity.</p>
    #[serde(rename = "MailFromAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mail_from_attributes: Option<MailFromAttributes>,
    /// <p>A map of policy names to policies.</p>
    #[serde(rename = "Policies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policies: Option<::std::collections::HashMap<String, String>>,
    /// <p>An array of objects that define the tags (keys and values) that are associated with the email identity.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>Specifies whether or not the identity is verified. You can only send email from verified email addresses or domains. For more information about verifying identities, see the <a href="https://docs.aws.amazon.com/pinpoint/latest/userguide/channels-email-manage-verify.html">Amazon Pinpoint User Guide</a>.</p>
    #[serde(rename = "VerifiedForSendingStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified_for_sending_status: Option<bool>,
}

/// <p>Represents a request to display the template object (which includes the subject line, HTML part and text part) for the template you specify.</p>
/// see [SesV2::get_email_template]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetEmailTemplateRequest {
    /// <p>The name of the template you want to retrieve.</p>
    #[serde(rename = "TemplateName")]
    pub template_name: String,
}

/// <p>The following element is returned by the service.</p>
/// see [SesV2::get_email_template]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetEmailTemplateResponse {
    /// <p>The content of the email template, composed of a subject line, an HTML part, and a text-only part.</p>
    #[serde(rename = "TemplateContent")]
    pub template_content: EmailTemplateContent,
    /// <p>The name of the template you want to retrieve.</p>
    #[serde(rename = "TemplateName")]
    pub template_name: String,
}

/// <p>Represents a request for information about an import job using the import job ID.</p>
/// see [SesV2::get_import_job]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetImportJobRequest {
    /// <p>The ID of the import job.</p>
    #[serde(rename = "JobId")]
    pub job_id: String,
}

/// <p>An HTTP 200 response if the request succeeds, or an error message if the request fails.</p>
/// see [SesV2::get_import_job]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetImportJobResponse {
    /// <p>The time stamp of when the import job was completed.</p>
    #[serde(rename = "CompletedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_timestamp: Option<f64>,
    /// <p>The time stamp of when the import job was created.</p>
    #[serde(rename = "CreatedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_timestamp: Option<f64>,
    /// <p>The number of records that failed processing because of invalid input or other reasons.</p>
    #[serde(rename = "FailedRecordsCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_records_count: Option<i64>,
    /// <p>The failure details about an import job.</p>
    #[serde(rename = "FailureInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_info: Option<FailureInfo>,
    /// <p>The data source of the import job.</p>
    #[serde(rename = "ImportDataSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_data_source: Option<ImportDataSource>,
    /// <p>The destination of the import job.</p>
    #[serde(rename = "ImportDestination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_destination: Option<ImportDestination>,
    /// <p>A string that represents the import job ID.</p>
    #[serde(rename = "JobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    /// <p>The status of the import job.</p>
    #[serde(rename = "JobStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
    /// <p>The current number of records processed.</p>
    #[serde(rename = "ProcessedRecordsCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processed_records_count: Option<i64>,
}

/// <p>A request to retrieve information about an email address that's on the suppression list for your account.</p>
/// see [SesV2::get_suppressed_destination]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetSuppressedDestinationRequest {
    /// <p>The email address that's on the account suppression list.</p>
    #[serde(rename = "EmailAddress")]
    pub email_address: String,
}

/// <p>Information about the suppressed email address.</p>
/// see [SesV2::get_suppressed_destination]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetSuppressedDestinationResponse {
    /// <p>An object containing information about the suppressed email address.</p>
    #[serde(rename = "SuppressedDestination")]
    pub suppressed_destination: SuppressedDestination,
}

/// <p>Information about an email identity.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct IdentityInfo {
    /// <p>The address or domain of the identity.</p>
    #[serde(rename = "IdentityName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_name: Option<String>,
    /// <p><p>The email identity type. The identity type can be one of the following:</p> <ul> <li> <p> <code>EMAIL<em>ADDRESS</code> – The identity is an email address.</p> </li> <li> <p> <code>DOMAIN</code> – The identity is a domain.</p> </li> <li> <p> <code>MANAGED</em>DOMAIN</code> – The identity is a domain that is managed by AWS.</p> </li> </ul></p>
    #[serde(rename = "IdentityType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_type: Option<String>,
    /// <p>Indicates whether or not you can send email from the identity.</p> <p>An <i>identity</i> is an email address or domain that you send email from. Before you can send email from an identity, you have to demostrate that you own the identity, and that you authorize Amazon SES to send email from that identity.</p>
    #[serde(rename = "SendingEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sending_enabled: Option<bool>,
}

/// <p>An object that contains details about the data source of the import job.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ImportDataSource {
    /// <p>The data format of the import job's data source.</p>
    #[serde(rename = "DataFormat")]
    pub data_format: String,
    /// <p>An Amazon S3 URL in the format s3://<i>&lt;bucket_name&gt;</i>/<i>&lt;object&gt;</i>.</p>
    #[serde(rename = "S3Url")]
    pub s3_url: String,
}

/// <p>An object that contains details about the resource destination the import job is going to target.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ImportDestination {
    /// <p>An object that contains the action of the import job towards a contact list.</p>
    #[serde(rename = "ContactListDestination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_list_destination: Option<ContactListDestination>,
    /// <p>An object that contains the action of the import job towards suppression list.</p>
    #[serde(rename = "SuppressionListDestination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suppression_list_destination: Option<SuppressionListDestination>,
}

/// <p>A summary of the import job.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ImportJobSummary {
    /// <p>The date and time when the import job was created.</p>
    #[serde(rename = "CreatedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_timestamp: Option<f64>,
    #[serde(rename = "ImportDestination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_destination: Option<ImportDestination>,
    #[serde(rename = "JobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(rename = "JobStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
}

/// <p>An object that contains information about the inbox placement data settings for a verified domain that’s associated with your AWS account. This data is available only if you enabled the Deliverability dashboard for the domain.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct InboxPlacementTrackingOption {
    /// <p>Specifies whether inbox placement data is being tracked for the domain.</p>
    #[serde(rename = "Global")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global: Option<bool>,
    /// <p>An array of strings, one for each major email provider that the inbox placement data applies to.</p>
    #[serde(rename = "TrackedIsps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracked_isps: Option<Vec<String>>,
}

/// <p>An object that describes how email sent during the predictive inbox placement test was handled by a certain email provider.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct IspPlacement {
    /// <p>The name of the email provider that the inbox placement data applies to.</p>
    #[serde(rename = "IspName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isp_name: Option<String>,
    /// <p>An object that contains inbox placement metrics for a specific email provider.</p>
    #[serde(rename = "PlacementStatistics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_statistics: Option<PlacementStatistics>,
}

/// <p>An object that defines an Amazon Kinesis Data Firehose destination for email events. You can use Amazon Kinesis Data Firehose to stream data to other services, such as Amazon S3 and Amazon Redshift.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct KinesisFirehoseDestination {
    /// <p>The Amazon Resource Name (ARN) of the Amazon Kinesis Data Firehose stream that the Amazon SES API v2 sends email events to.</p>
    #[serde(rename = "DeliveryStreamArn")]
    pub delivery_stream_arn: String,
    /// <p>The Amazon Resource Name (ARN) of the IAM role that the Amazon SES API v2 uses to send email events to the Amazon Kinesis Data Firehose stream.</p>
    #[serde(rename = "IamRoleArn")]
    pub iam_role_arn: String,
}

/// <p>A request to obtain a list of configuration sets for your Amazon SES account in the current AWS Region.</p>
/// see [SesV2::list_configuration_sets]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListConfigurationSetsRequest {
    /// <p>A token returned from a previous call to <code>ListConfigurationSets</code> to indicate the position in the list of configuration sets.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The number of results to show in a single call to <code>ListConfigurationSets</code>. If the number of results is larger than the number you specified in this parameter, then the response includes a <code>NextToken</code> element, which you can use to obtain additional results.</p>
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
}

/// <p>A list of configuration sets in your Amazon SES account in the current AWS Region.</p>
/// see [SesV2::list_configuration_sets]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListConfigurationSetsResponse {
    /// <p>An array that contains all of the configuration sets in your Amazon SES account in the current AWS Region.</p>
    #[serde(rename = "ConfigurationSets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_sets: Option<Vec<String>>,
    /// <p>A token that indicates that there are additional configuration sets to list. To view additional configuration sets, issue another request to <code>ListConfigurationSets</code>, and pass this token in the <code>NextToken</code> parameter.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// see [SesV2::list_contact_lists]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListContactListsRequest {
    /// <p>A string token indicating that there might be additional contact lists available to be listed. Use the token provided in the Response to use in the subsequent call to ListContactLists with the same parameters to retrieve the next page of contact lists.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Maximum number of contact lists to return at once. Use this parameter to paginate results. If additional contact lists exist beyond the specified limit, the <code>NextToken</code> element is sent in the response. Use the <code>NextToken</code> value in subsequent requests to retrieve additional lists.</p>
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
}

/// see [SesV2::list_contact_lists]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListContactListsResponse {
    /// <p>The available contact lists.</p>
    #[serde(rename = "ContactLists")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_lists: Option<Vec<ContactList>>,
    /// <p>A string token indicating that there might be additional contact lists available to be listed. Copy this token to a subsequent call to <code>ListContactLists</code> with the same parameters to retrieve the next page of contact lists.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>A filter that can be applied to a list of contacts.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListContactsFilter {
    /// <p>The status by which you are filtering: <code>OPT_IN</code> or <code>OPT_OUT</code>.</p>
    #[serde(rename = "FilteredStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filtered_status: Option<String>,
    /// <p>Used for filtering by a specific topic preference.</p>
    #[serde(rename = "TopicFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_filter: Option<TopicFilter>,
}

/// see [SesV2::list_contacts]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListContactsRequest {
    /// <p>The name of the contact list.</p>
    #[serde(rename = "ContactListName")]
    pub contact_list_name: String,
    /// <p>A filter that can be applied to a list of contacts.</p>
    #[serde(rename = "Filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<ListContactsFilter>,
    /// <p>A string token indicating that there might be additional contacts available to be listed. Use the token provided in the Response to use in the subsequent call to ListContacts with the same parameters to retrieve the next page of contacts.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The number of contacts that may be returned at once, which is dependent on if there are more or less contacts than the value of the PageSize. Use this parameter to paginate results. If additional contacts exist beyond the specified limit, the <code>NextToken</code> element is sent in the response. Use the <code>NextToken</code> value in subsequent requests to retrieve additional contacts.</p>
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
}

/// see [SesV2::list_contacts]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListContactsResponse {
    /// <p>The contacts present in a specific contact list.</p>
    #[serde(rename = "Contacts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contacts: Option<Vec<Contact>>,
    /// <p>A string token indicating that there might be additional contacts available to be listed. Copy this token to a subsequent call to <code>ListContacts</code> with the same parameters to retrieve the next page of contacts.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Represents a request to list the existing custom verification email templates for your account.</p>
/// see [SesV2::list_custom_verification_email_templates]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListCustomVerificationEmailTemplatesRequest {
    /// <p>A token returned from a previous call to <code>ListCustomVerificationEmailTemplates</code> to indicate the position in the list of custom verification email templates.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The number of results to show in a single call to <code>ListCustomVerificationEmailTemplates</code>. If the number of results is larger than the number you specified in this parameter, then the response includes a <code>NextToken</code> element, which you can use to obtain additional results.</p> <p>The value you specify has to be at least 1, and can be no more than 50.</p>
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
}

/// <p>The following elements are returned by the service.</p>
/// see [SesV2::list_custom_verification_email_templates]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListCustomVerificationEmailTemplatesResponse {
    /// <p>A list of the custom verification email templates that exist in your account.</p>
    #[serde(rename = "CustomVerificationEmailTemplates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_verification_email_templates: Option<Vec<CustomVerificationEmailTemplateMetadata>>,
    /// <p>A token indicating that there are additional custom verification email templates available to be listed. Pass this token to a subsequent call to <code>ListCustomVerificationEmailTemplates</code> to retrieve the next 50 custom verification email templates.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>A request to obtain a list of dedicated IP pools.</p>
/// see [SesV2::list_dedicated_ip_pools]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListDedicatedIpPoolsRequest {
    /// <p>A token returned from a previous call to <code>ListDedicatedIpPools</code> to indicate the position in the list of dedicated IP pools.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The number of results to show in a single call to <code>ListDedicatedIpPools</code>. If the number of results is larger than the number you specified in this parameter, then the response includes a <code>NextToken</code> element, which you can use to obtain additional results.</p>
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
}

/// <p>A list of dedicated IP pools.</p>
/// see [SesV2::list_dedicated_ip_pools]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListDedicatedIpPoolsResponse {
    /// <p>A list of all of the dedicated IP pools that are associated with your AWS account in the current Region.</p>
    #[serde(rename = "DedicatedIpPools")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dedicated_ip_pools: Option<Vec<String>>,
    /// <p>A token that indicates that there are additional IP pools to list. To view additional IP pools, issue another request to <code>ListDedicatedIpPools</code>, passing this token in the <code>NextToken</code> parameter.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>A request to list all of the predictive inbox placement tests that you've performed.</p>
/// see [SesV2::list_deliverability_test_reports]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListDeliverabilityTestReportsRequest {
    /// <p>A token returned from a previous call to <code>ListDeliverabilityTestReports</code> to indicate the position in the list of predictive inbox placement tests.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The number of results to show in a single call to <code>ListDeliverabilityTestReports</code>. If the number of results is larger than the number you specified in this parameter, then the response includes a <code>NextToken</code> element, which you can use to obtain additional results.</p> <p>The value you specify has to be at least 0, and can be no more than 1000.</p>
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
}

/// <p>A list of the predictive inbox placement test reports that are available for your account, regardless of whether or not those tests are complete.</p>
/// see [SesV2::list_deliverability_test_reports]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListDeliverabilityTestReportsResponse {
    /// <p>An object that contains a lists of predictive inbox placement tests that you've performed.</p>
    #[serde(rename = "DeliverabilityTestReports")]
    pub deliverability_test_reports: Vec<DeliverabilityTestReport>,
    /// <p>A token that indicates that there are additional predictive inbox placement tests to list. To view additional predictive inbox placement tests, issue another request to <code>ListDeliverabilityTestReports</code>, and pass this token in the <code>NextToken</code> parameter.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Retrieve deliverability data for all the campaigns that used a specific domain to send email during a specified time range. This data is available for a domain only if you enabled the Deliverability dashboard.</p>
/// see [SesV2::list_domain_deliverability_campaigns]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListDomainDeliverabilityCampaignsRequest {
    /// <p>The last day, in Unix time format, that you want to obtain deliverability data for. This value has to be less than or equal to 30 days after the value of the <code>StartDate</code> parameter.</p>
    #[serde(rename = "EndDate")]
    pub end_date: f64,
    /// <p>A token that’s returned from a previous call to the <code>ListDomainDeliverabilityCampaigns</code> operation. This token indicates the position of a campaign in the list of campaigns.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The maximum number of results to include in response to a single call to the <code>ListDomainDeliverabilityCampaigns</code> operation. If the number of results is larger than the number that you specify in this parameter, the response includes a <code>NextToken</code> element, which you can use to obtain additional results.</p>
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// <p>The first day, in Unix time format, that you want to obtain deliverability data for.</p>
    #[serde(rename = "StartDate")]
    pub start_date: f64,
    /// <p>The domain to obtain deliverability data for.</p>
    #[serde(rename = "SubscribedDomain")]
    pub subscribed_domain: String,
}

/// <p>An array of objects that provide deliverability data for all the campaigns that used a specific domain to send email during a specified time range. This data is available for a domain only if you enabled the Deliverability dashboard for the domain.</p>
/// see [SesV2::list_domain_deliverability_campaigns]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListDomainDeliverabilityCampaignsResponse {
    /// <p>An array of responses, one for each campaign that used the domain to send email during the specified time range.</p>
    #[serde(rename = "DomainDeliverabilityCampaigns")]
    pub domain_deliverability_campaigns: Vec<DomainDeliverabilityCampaign>,
    /// <p>A token that’s returned from a previous call to the <code>ListDomainDeliverabilityCampaigns</code> operation. This token indicates the position of the campaign in the list of campaigns.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>A request to list all of the email identities associated with your AWS account. This list includes identities that you've already verified, identities that are unverified, and identities that were verified in the past, but are no longer verified.</p>
/// see [SesV2::list_email_identities]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListEmailIdentitiesRequest {
    /// <p>A token returned from a previous call to <code>ListEmailIdentities</code> to indicate the position in the list of identities.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The number of results to show in a single call to <code>ListEmailIdentities</code>. If the number of results is larger than the number you specified in this parameter, then the response includes a <code>NextToken</code> element, which you can use to obtain additional results.</p> <p>The value you specify has to be at least 0, and can be no more than 1000.</p>
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
}

/// <p>A list of all of the identities that you've attempted to verify, regardless of whether or not those identities were successfully verified.</p>
/// see [SesV2::list_email_identities]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListEmailIdentitiesResponse {
    /// <p>An array that includes all of the email identities associated with your AWS account.</p>
    #[serde(rename = "EmailIdentities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_identities: Option<Vec<IdentityInfo>>,
    /// <p>A token that indicates that there are additional configuration sets to list. To view additional configuration sets, issue another request to <code>ListEmailIdentities</code>, and pass this token in the <code>NextToken</code> parameter.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Represents a request to list the email templates present in your Amazon SES account in the current AWS Region. For more information, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/send-personalized-email-api.html">Amazon SES Developer Guide</a>.</p>
/// see [SesV2::list_email_templates]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListEmailTemplatesRequest {
    /// <p>A token returned from a previous call to <code>ListEmailTemplates</code> to indicate the position in the list of email templates.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The number of results to show in a single call to <code>ListEmailTemplates</code>. If the number of results is larger than the number you specified in this parameter, then the response includes a <code>NextToken</code> element, which you can use to obtain additional results.</p> <p>The value you specify has to be at least 1, and can be no more than 10.</p>
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
}

/// <p>The following elements are returned by the service.</p>
/// see [SesV2::list_email_templates]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListEmailTemplatesResponse {
    /// <p>A token indicating that there are additional email templates available to be listed. Pass this token to a subsequent <code>ListEmailTemplates</code> call to retrieve the next 10 email templates.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>An array the contains the name and creation time stamp for each template in your Amazon SES account.</p>
    #[serde(rename = "TemplatesMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub templates_metadata: Option<Vec<EmailTemplateMetadata>>,
}

/// <p>Represents a request to list all of the import jobs for a data destination within the specified maximum number of import jobs.</p>
/// see [SesV2::list_import_jobs]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListImportJobsRequest {
    /// <p>The destination of the import job, which can be used to list import jobs that have a certain <code>ImportDestinationType</code>.</p>
    #[serde(rename = "ImportDestinationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_destination_type: Option<String>,
    /// <p>A string token indicating that there might be additional import jobs available to be listed. Copy this token to a subsequent call to <code>ListImportJobs</code> with the same parameters to retrieve the next page of import jobs.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Maximum number of import jobs to return at once. Use this parameter to paginate results. If additional import jobs exist beyond the specified limit, the <code>NextToken</code> element is sent in the response. Use the <code>NextToken</code> value in subsequent requests to retrieve additional addresses.</p>
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
}

/// <p>An HTTP 200 response if the request succeeds, or an error message if the request fails.</p>
/// see [SesV2::list_import_jobs]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListImportJobsResponse {
    /// <p>A list of the import job summaries.</p>
    #[serde(rename = "ImportJobs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_jobs: Option<Vec<ImportJobSummary>>,
    /// <p>A string token indicating that there might be additional import jobs available to be listed. Copy this token to a subsequent call to <code>ListImportJobs</code> with the same parameters to retrieve the next page of import jobs.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>An object used to specify a list or topic to which an email belongs, which will be used when a contact chooses to unsubscribe.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListManagementOptions {
    /// <p>The name of the contact list.</p>
    #[serde(rename = "ContactListName")]
    pub contact_list_name: String,
    /// <p>The name of the topic.</p>
    #[serde(rename = "TopicName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_name: Option<String>,
}

/// <p>A request to obtain a list of email destinations that are on the suppression list for your account.</p>
/// see [SesV2::list_suppressed_destinations]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListSuppressedDestinationsRequest {
    /// <p>Used to filter the list of suppressed email destinations so that it only includes addresses that were added to the list before a specific date. The date that you specify should be in Unix time format.</p>
    #[serde(rename = "EndDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<f64>,
    /// <p>A token returned from a previous call to <code>ListSuppressedDestinations</code> to indicate the position in the list of suppressed email addresses.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The number of results to show in a single call to <code>ListSuppressedDestinations</code>. If the number of results is larger than the number you specified in this parameter, then the response includes a <code>NextToken</code> element, which you can use to obtain additional results.</p>
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// <p>The factors that caused the email address to be added to .</p>
    #[serde(rename = "Reasons")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasons: Option<Vec<String>>,
    /// <p>Used to filter the list of suppressed email destinations so that it only includes addresses that were added to the list after a specific date. The date that you specify should be in Unix time format.</p>
    #[serde(rename = "StartDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<f64>,
}

/// <p>A list of suppressed email addresses.</p>
/// see [SesV2::list_suppressed_destinations]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListSuppressedDestinationsResponse {
    /// <p>A token that indicates that there are additional email addresses on the suppression list for your account. To view additional suppressed addresses, issue another request to <code>ListSuppressedDestinations</code>, and pass this token in the <code>NextToken</code> parameter.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of summaries, each containing a summary for a suppressed email destination.</p>
    #[serde(rename = "SuppressedDestinationSummaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suppressed_destination_summaries: Option<Vec<SuppressedDestinationSummary>>,
}

/// see [SesV2::list_tags_for_resource]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the resource that you want to retrieve tag information for.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
}

/// see [SesV2::list_tags_for_resource]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForResourceResponse {
    /// <p>An array that lists all the tags that are associated with the resource. Each tag consists of a required tag key (<code>Key</code>) and an associated tag value (<code>Value</code>)</p>
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,
}

/// <p>A list of attributes that are associated with a MAIL FROM domain.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct MailFromAttributes {
    /// <p>The action that you want to take if the required MX record can't be found when you send an email. When you set this value to <code>UseDefaultValue</code>, the mail is sent using <i>amazonses.com</i> as the MAIL FROM domain. When you set this value to <code>RejectMessage</code>, the Amazon SES API v2 returns a <code>MailFromDomainNotVerified</code> error, and doesn't attempt to deliver the email.</p> <p>These behaviors are taken when the custom MAIL FROM domain configuration is in the <code>Pending</code>, <code>Failed</code>, and <code>TemporaryFailure</code> states.</p>
    #[serde(rename = "BehaviorOnMxFailure")]
    pub behavior_on_mx_failure: String,
    /// <p>The name of a domain that an email identity uses as a custom MAIL FROM domain.</p>
    #[serde(rename = "MailFromDomain")]
    pub mail_from_domain: String,
    /// <p><p>The status of the MAIL FROM domain. This status can have the following values:</p> <ul> <li> <p> <code>PENDING</code> – Amazon SES hasn&#39;t started searching for the MX record yet.</p> </li> <li> <p> <code>SUCCESS</code> – Amazon SES detected the required MX record for the MAIL FROM domain.</p> </li> <li> <p> <code>FAILED</code> – Amazon SES can&#39;t find the required MX record, or the record no longer exists.</p> </li> <li> <p> <code>TEMPORARY_FAILURE</code> – A temporary issue occurred, which prevented Amazon SES from determining the status of the MAIL FROM domain.</p> </li> </ul></p>
    #[serde(rename = "MailFromDomainStatus")]
    pub mail_from_domain_status: String,
}

/// <p>Represents the email message that you're sending. The <code>Message</code> object consists of a subject line and a message body.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Message {
    /// <p>The body of the message. You can specify an HTML version of the message, a text-only version of the message, or both.</p>
    #[serde(rename = "Body")]
    pub body: Body,
    /// <p>The subject line of the email. The subject line can only contain 7-bit ASCII characters. However, you can specify non-ASCII characters in the subject line by using encoded-word syntax, as described in <a href="https://tools.ietf.org/html/rfc2047">RFC 2047</a>.</p>
    #[serde(rename = "Subject")]
    pub subject: Content,
}

/// <p>Contains the name and value of a tag that you apply to an email. You can use message tags when you publish email sending events. </p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct MessageTag {
    /// <p><p>The name of the message tag. The message tag name has to meet the following criteria:</p> <ul> <li> <p>It can only contain ASCII letters (a–z, A–Z), numbers (0–9), underscores (_), or dashes (-).</p> </li> <li> <p>It can contain no more than 256 characters.</p> </li> </ul></p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p><p>The value of the message tag. The message tag value has to meet the following criteria:</p> <ul> <li> <p>It can only contain ASCII letters (a–z, A–Z), numbers (0–9), underscores (_), or dashes (-).</p> </li> <li> <p>It can contain no more than 256 characters.</p> </li> </ul></p>
    #[serde(rename = "Value")]
    pub value: String,
}

/// <p>An object that contains information about email that was sent from the selected domain.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct OverallVolume {
    /// <p>An object that contains inbox and junk mail placement metrics for individual email providers.</p>
    #[serde(rename = "DomainIspPlacements")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_isp_placements: Option<Vec<DomainIspPlacement>>,
    /// <p>The percentage of emails that were sent from the domain that were read by their recipients.</p>
    #[serde(rename = "ReadRatePercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_rate_percent: Option<f64>,
    /// <p>An object that contains information about the numbers of messages that arrived in recipients' inboxes and junk mail folders.</p>
    #[serde(rename = "VolumeStatistics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_statistics: Option<VolumeStatistics>,
}

/// <p>An object that defines an Amazon Pinpoint project destination for email events. You can send email event data to a Amazon Pinpoint project to view metrics using the Transactional Messaging dashboards that are built in to Amazon Pinpoint. For more information, see <a href="https://docs.aws.amazon.com/pinpoint/latest/userguide/analytics-transactional-messages.html">Transactional Messaging Charts</a> in the <i>Amazon Pinpoint User Guide</i>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct PinpointDestination {
    /// <p>The Amazon Resource Name (ARN) of the Amazon Pinpoint project that you want to send email events to.</p>
    #[serde(rename = "ApplicationArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_arn: Option<String>,
}

/// <p>An object that contains inbox placement data for an email provider.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PlacementStatistics {
    /// <p>The percentage of emails that were authenticated by using DomainKeys Identified Mail (DKIM) during the predictive inbox placement test.</p>
    #[serde(rename = "DkimPercentage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dkim_percentage: Option<f64>,
    /// <p>The percentage of emails that arrived in recipients' inboxes during the predictive inbox placement test.</p>
    #[serde(rename = "InboxPercentage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbox_percentage: Option<f64>,
    /// <p>The percentage of emails that didn't arrive in recipients' inboxes at all during the predictive inbox placement test.</p>
    #[serde(rename = "MissingPercentage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_percentage: Option<f64>,
    /// <p>The percentage of emails that arrived in recipients' spam or junk mail folders during the predictive inbox placement test.</p>
    #[serde(rename = "SpamPercentage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spam_percentage: Option<f64>,
    /// <p>The percentage of emails that were authenticated by using Sender Policy Framework (SPF) during the predictive inbox placement test.</p>
    #[serde(rename = "SpfPercentage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spf_percentage: Option<f64>,
}

/// <p>A request to enable or disable the automatic IP address warm-up feature.</p>
/// see [SesV2::put_account_dedicated_ip_warmup_attributes]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutAccountDedicatedIpWarmupAttributesRequest {
    /// <p>Enables or disables the automatic warm-up feature for dedicated IP addresses that are associated with your Amazon SES account in the current AWS Region. Set to <code>true</code> to enable the automatic warm-up feature, or set to <code>false</code> to disable it.</p>
    #[serde(rename = "AutoWarmupEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_warmup_enabled: Option<bool>,
}

/// <p>An HTTP 200 response if the request succeeds, or an error message if the request fails.</p>
/// see [SesV2::put_account_dedicated_ip_warmup_attributes]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutAccountDedicatedIpWarmupAttributesResponse {}

/// <p>A request to submit new account details.</p>
/// see [SesV2::put_account_details]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutAccountDetailsRequest {
    /// <p>Additional email addresses that you would like to be notified regarding Amazon SES matters.</p>
    #[serde(rename = "AdditionalContactEmailAddresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_contact_email_addresses: Option<Vec<String>>,
    /// <p>The language you would prefer to be contacted with.</p>
    #[serde(rename = "ContactLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_language: Option<String>,
    /// <p>The type of email your account will send.</p>
    #[serde(rename = "MailType")]
    pub mail_type: String,
    /// <p>Indicates whether or not your account should have production access in the current AWS Region.</p> <p>If the value is <code>false</code>, then your account is in the <i>sandbox</i>. When your account is in the sandbox, you can only send email to verified identities. Additionally, the maximum number of emails you can send in a 24-hour period (your sending quota) is 200, and the maximum number of emails you can send per second (your maximum sending rate) is 1.</p> <p>If the value is <code>true</code>, then your account has production access. When your account has production access, you can send email to any address. The sending quota and maximum sending rate for your account vary based on your specific use case.</p>
    #[serde(rename = "ProductionAccessEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub production_access_enabled: Option<bool>,
    /// <p>A description of the types of email that you plan to send.</p>
    #[serde(rename = "UseCaseDescription")]
    pub use_case_description: String,
    /// <p>The URL of your website. This information helps us better understand the type of content that you plan to send.</p>
    #[serde(rename = "WebsiteURL")]
    pub website_url: String,
}

/// <p>An HTTP 200 response if the request succeeds, or an error message if the request fails.</p>
/// see [SesV2::put_account_details]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutAccountDetailsResponse {}

/// <p>A request to change the ability of your account to send email.</p>
/// see [SesV2::put_account_sending_attributes]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutAccountSendingAttributesRequest {
    /// <p><p>Enables or disables your account&#39;s ability to send email. Set to <code>true</code> to enable email sending, or set to <code>false</code> to disable email sending.</p> <note> <p>If AWS paused your account&#39;s ability to send email, you can&#39;t use this operation to resume your account&#39;s ability to send email.</p> </note></p>
    #[serde(rename = "SendingEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sending_enabled: Option<bool>,
}

/// <p>An HTTP 200 response if the request succeeds, or an error message if the request fails.</p>
/// see [SesV2::put_account_sending_attributes]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutAccountSendingAttributesResponse {}

/// <p>A request to change your account's suppression preferences.</p>
/// see [SesV2::put_account_suppression_attributes]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutAccountSuppressionAttributesRequest {
    /// <p><p>A list that contains the reasons that email addresses will be automatically added to the suppression list for your account. This list can contain any or all of the following:</p> <ul> <li> <p> <code>COMPLAINT</code> – Amazon SES adds an email address to the suppression list for your account when a message sent to that address results in a complaint.</p> </li> <li> <p> <code>BOUNCE</code> – Amazon SES adds an email address to the suppression list for your account when a message sent to that address results in a hard bounce.</p> </li> </ul></p>
    #[serde(rename = "SuppressedReasons")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suppressed_reasons: Option<Vec<String>>,
}

/// <p>An HTTP 200 response if the request succeeds, or an error message if the request fails.</p>
/// see [SesV2::put_account_suppression_attributes]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutAccountSuppressionAttributesResponse {}

/// <p>A request to associate a configuration set with a dedicated IP pool.</p>
/// see [SesV2::put_configuration_set_delivery_options]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutConfigurationSetDeliveryOptionsRequest {
    /// <p>The name of the configuration set that you want to associate with a dedicated IP pool.</p>
    #[serde(rename = "ConfigurationSetName")]
    pub configuration_set_name: String,
    /// <p>The name of the dedicated IP pool that you want to associate with the configuration set.</p>
    #[serde(rename = "SendingPoolName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sending_pool_name: Option<String>,
    /// <p>Specifies whether messages that use the configuration set are required to use Transport Layer Security (TLS). If the value is <code>Require</code>, messages are only delivered if a TLS connection can be established. If the value is <code>Optional</code>, messages can be delivered in plain text if a TLS connection can't be established.</p>
    #[serde(rename = "TlsPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls_policy: Option<String>,
}

/// <p>An HTTP 200 response if the request succeeds, or an error message if the request fails.</p>
/// see [SesV2::put_configuration_set_delivery_options]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutConfigurationSetDeliveryOptionsResponse {}

/// <p>A request to enable or disable tracking of reputation metrics for a configuration set.</p>
/// see [SesV2::put_configuration_set_reputation_options]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutConfigurationSetReputationOptionsRequest {
    /// <p>The name of the configuration set that you want to enable or disable reputation metric tracking for.</p>
    #[serde(rename = "ConfigurationSetName")]
    pub configuration_set_name: String,
    /// <p>If <code>true</code>, tracking of reputation metrics is enabled for the configuration set. If <code>false</code>, tracking of reputation metrics is disabled for the configuration set.</p>
    #[serde(rename = "ReputationMetricsEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reputation_metrics_enabled: Option<bool>,
}

/// <p>An HTTP 200 response if the request succeeds, or an error message if the request fails.</p>
/// see [SesV2::put_configuration_set_reputation_options]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutConfigurationSetReputationOptionsResponse {}

/// <p>A request to enable or disable the ability of Amazon SES to send emails that use a specific configuration set.</p>
/// see [SesV2::put_configuration_set_sending_options]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutConfigurationSetSendingOptionsRequest {
    /// <p>The name of the configuration set that you want to enable or disable email sending for.</p>
    #[serde(rename = "ConfigurationSetName")]
    pub configuration_set_name: String,
    /// <p>If <code>true</code>, email sending is enabled for the configuration set. If <code>false</code>, email sending is disabled for the configuration set.</p>
    #[serde(rename = "SendingEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sending_enabled: Option<bool>,
}

/// <p>An HTTP 200 response if the request succeeds, or an error message if the request fails.</p>
/// see [SesV2::put_configuration_set_sending_options]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutConfigurationSetSendingOptionsResponse {}

/// <p>A request to change the account suppression list preferences for a specific configuration set.</p>
/// see [SesV2::put_configuration_set_suppression_options]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutConfigurationSetSuppressionOptionsRequest {
    /// <p>The name of the configuration set that you want to change the suppression list preferences for.</p>
    #[serde(rename = "ConfigurationSetName")]
    pub configuration_set_name: String,
    /// <p><p>A list that contains the reasons that email addresses are automatically added to the suppression list for your account. This list can contain any or all of the following:</p> <ul> <li> <p> <code>COMPLAINT</code> – Amazon SES adds an email address to the suppression list for your account when a message sent to that address results in a complaint.</p> </li> <li> <p> <code>BOUNCE</code> – Amazon SES adds an email address to the suppression list for your account when a message sent to that address results in a hard bounce.</p> </li> </ul></p>
    #[serde(rename = "SuppressedReasons")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suppressed_reasons: Option<Vec<String>>,
}

/// <p>An HTTP 200 response if the request succeeds, or an error message if the request fails.</p>
/// see [SesV2::put_configuration_set_suppression_options]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutConfigurationSetSuppressionOptionsResponse {}

/// <p>A request to add a custom domain for tracking open and click events to a configuration set.</p>
/// see [SesV2::put_configuration_set_tracking_options]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutConfigurationSetTrackingOptionsRequest {
    /// <p>The name of the configuration set that you want to add a custom tracking domain to.</p>
    #[serde(rename = "ConfigurationSetName")]
    pub configuration_set_name: String,
    /// <p>The domain that you want to use to track open and click events.</p>
    #[serde(rename = "CustomRedirectDomain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_redirect_domain: Option<String>,
}

/// <p>An HTTP 200 response if the request succeeds, or an error message if the request fails.</p>
/// see [SesV2::put_configuration_set_tracking_options]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutConfigurationSetTrackingOptionsResponse {}

/// <p>A request to move a dedicated IP address to a dedicated IP pool.</p>
/// see [SesV2::put_dedicated_ip_in_pool]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutDedicatedIpInPoolRequest {
    /// <p>The name of the IP pool that you want to add the dedicated IP address to. You have to specify an IP pool that already exists.</p>
    #[serde(rename = "DestinationPoolName")]
    pub destination_pool_name: String,
    /// <p>The IP address that you want to move to the dedicated IP pool. The value you specify has to be a dedicated IP address that's associated with your AWS account.</p>
    #[serde(rename = "Ip")]
    pub ip: String,
}

/// <p>An HTTP 200 response if the request succeeds, or an error message if the request fails.</p>
/// see [SesV2::put_dedicated_ip_in_pool]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutDedicatedIpInPoolResponse {}

/// <p>A request to change the warm-up attributes for a dedicated IP address. This operation is useful when you want to resume the warm-up process for an existing IP address.</p>
/// see [SesV2::put_dedicated_ip_warmup_attributes]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutDedicatedIpWarmupAttributesRequest {
    /// <p>The dedicated IP address that you want to update the warm-up attributes for.</p>
    #[serde(rename = "Ip")]
    pub ip: String,
    /// <p>The warm-up percentage that you want to associate with the dedicated IP address.</p>
    #[serde(rename = "WarmupPercentage")]
    pub warmup_percentage: i64,
}

/// <p>An HTTP 200 response if the request succeeds, or an error message if the request fails.</p>
/// see [SesV2::put_dedicated_ip_warmup_attributes]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutDedicatedIpWarmupAttributesResponse {}

/// <p>Enable or disable the Deliverability dashboard. When you enable the Deliverability dashboard, you gain access to reputation, deliverability, and other metrics for the domains that you use to send email using Amazon SES API v2. You also gain the ability to perform predictive inbox placement tests.</p> <p>When you use the Deliverability dashboard, you pay a monthly subscription charge, in addition to any other fees that you accrue by using Amazon SES and other AWS services. For more information about the features and cost of a Deliverability dashboard subscription, see <a href="http://aws.amazon.com/pinpoint/pricing/">Amazon Pinpoint Pricing</a>.</p>
/// see [SesV2::put_deliverability_dashboard_option]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutDeliverabilityDashboardOptionRequest {
    /// <p>Specifies whether to enable the Deliverability dashboard. To enable the dashboard, set this value to <code>true</code>.</p>
    #[serde(rename = "DashboardEnabled")]
    pub dashboard_enabled: bool,
    /// <p>An array of objects, one for each verified domain that you use to send email and enabled the Deliverability dashboard for.</p>
    #[serde(rename = "SubscribedDomains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribed_domains: Option<Vec<DomainDeliverabilityTrackingOption>>,
}

/// <p>A response that indicates whether the Deliverability dashboard is enabled.</p>
/// see [SesV2::put_deliverability_dashboard_option]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutDeliverabilityDashboardOptionResponse {}

/// <p>A request to enable or disable DKIM signing of email that you send from an email identity.</p>
/// see [SesV2::put_email_identity_dkim_attributes]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutEmailIdentityDkimAttributesRequest {
    /// <p>The email identity that you want to change the DKIM settings for.</p>
    #[serde(rename = "EmailIdentity")]
    pub email_identity: String,
    /// <p>Sets the DKIM signing configuration for the identity.</p> <p>When you set this value <code>true</code>, then the messages that are sent from the identity are signed using DKIM. If you set this value to <code>false</code>, your messages are sent without DKIM signing.</p>
    #[serde(rename = "SigningEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_enabled: Option<bool>,
}

/// <p>An HTTP 200 response if the request succeeds, or an error message if the request fails.</p>
/// see [SesV2::put_email_identity_dkim_attributes]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutEmailIdentityDkimAttributesResponse {}

/// <p>A request to change the DKIM attributes for an email identity.</p>
/// see [SesV2::put_email_identity_dkim_signing_attributes]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutEmailIdentityDkimSigningAttributesRequest {
    /// <p>The email identity that you want to configure DKIM for.</p>
    #[serde(rename = "EmailIdentity")]
    pub email_identity: String,
    /// <p>An object that contains information about the private key and selector that you want to use to configure DKIM for the identity. This object is only required if you want to configure Bring Your Own DKIM (BYODKIM) for the identity.</p>
    #[serde(rename = "SigningAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_attributes: Option<DkimSigningAttributes>,
    /// <p><p>The method that you want to use to configure DKIM for the identity. There are two possible values:</p> <ul> <li> <p> <code>AWS_SES</code> – Configure DKIM for the identity by using <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/easy-dkim.html">Easy DKIM</a>.</p> </li> <li> <p> <code>EXTERNAL</code> – Configure DKIM for the identity by using Bring Your Own DKIM (BYODKIM).</p> </li> </ul></p>
    #[serde(rename = "SigningAttributesOrigin")]
    pub signing_attributes_origin: String,
}

/// <p>If the action is successful, the service sends back an HTTP 200 response.</p> <p>The following data is returned in JSON format by the service.</p>
/// see [SesV2::put_email_identity_dkim_signing_attributes]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutEmailIdentityDkimSigningAttributesResponse {
    /// <p><p>The DKIM authentication status of the identity. Amazon SES determines the authentication status by searching for specific records in the DNS configuration for your domain. If you used <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/easy-dkim.html">Easy DKIM</a> to set up DKIM authentication, Amazon SES tries to find three unique CNAME records in the DNS configuration for your domain.</p> <p>If you provided a public key to perform DKIM authentication, Amazon SES tries to find a TXT record that uses the selector that you specified. The value of the TXT record must be a public key that&#39;s paired with the private key that you specified in the process of creating the identity.</p> <p>The status can be one of the following:</p> <ul> <li> <p> <code>PENDING</code> – The verification process was initiated, but Amazon SES hasn&#39;t yet detected the DKIM records in the DNS configuration for the domain.</p> </li> <li> <p> <code>SUCCESS</code> – The verification process completed successfully.</p> </li> <li> <p> <code>FAILED</code> – The verification process failed. This typically occurs when Amazon SES fails to find the DKIM records in the DNS configuration of the domain.</p> </li> <li> <p> <code>TEMPORARY<em>FAILURE</code> – A temporary issue is preventing Amazon SES from determining the DKIM authentication status of the domain.</p> </li> <li> <p> <code>NOT</em>STARTED</code> – The DKIM verification process hasn&#39;t been initiated for the domain.</p> </li> </ul></p>
    #[serde(rename = "DkimStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dkim_status: Option<String>,
    /// <p>If you used <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/easy-dkim.html">Easy DKIM</a> to configure DKIM authentication for the domain, then this object contains a set of unique strings that you use to create a set of CNAME records that you add to the DNS configuration for your domain. When Amazon SES detects these records in the DNS configuration for your domain, the DKIM authentication process is complete.</p> <p>If you configured DKIM authentication for the domain by providing your own public-private key pair, then this object contains the selector that's associated with your public key.</p> <p>Regardless of the DKIM authentication method you use, Amazon SES searches for the appropriate records in the DNS configuration of the domain for up to 72 hours.</p>
    #[serde(rename = "DkimTokens")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dkim_tokens: Option<Vec<String>>,
}

/// <p>A request to set the attributes that control how bounce and complaint events are processed.</p>
/// see [SesV2::put_email_identity_feedback_attributes]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutEmailIdentityFeedbackAttributesRequest {
    /// <p>Sets the feedback forwarding configuration for the identity.</p> <p>If the value is <code>true</code>, you receive email notifications when bounce or complaint events occur. These notifications are sent to the address that you specified in the <code>Return-Path</code> header of the original email.</p> <p>You're required to have a method of tracking bounces and complaints. If you haven't set up another mechanism for receiving bounce or complaint notifications (for example, by setting up an event destination), you receive an email notification when these events occur (even if this setting is disabled).</p>
    #[serde(rename = "EmailForwardingEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_forwarding_enabled: Option<bool>,
    /// <p>The email identity that you want to configure bounce and complaint feedback forwarding for.</p>
    #[serde(rename = "EmailIdentity")]
    pub email_identity: String,
}

/// <p>An HTTP 200 response if the request succeeds, or an error message if the request fails.</p>
/// see [SesV2::put_email_identity_feedback_attributes]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutEmailIdentityFeedbackAttributesResponse {}

/// <p>A request to configure the custom MAIL FROM domain for a verified identity.</p>
/// see [SesV2::put_email_identity_mail_from_attributes]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutEmailIdentityMailFromAttributesRequest {
    /// <p>The action that you want to take if the required MX record isn't found when you send an email. When you set this value to <code>UseDefaultValue</code>, the mail is sent using <i>amazonses.com</i> as the MAIL FROM domain. When you set this value to <code>RejectMessage</code>, the Amazon SES API v2 returns a <code>MailFromDomainNotVerified</code> error, and doesn't attempt to deliver the email.</p> <p>These behaviors are taken when the custom MAIL FROM domain configuration is in the <code>Pending</code>, <code>Failed</code>, and <code>TemporaryFailure</code> states.</p>
    #[serde(rename = "BehaviorOnMxFailure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub behavior_on_mx_failure: Option<String>,
    /// <p>The verified email identity that you want to set up the custom MAIL FROM domain for.</p>
    #[serde(rename = "EmailIdentity")]
    pub email_identity: String,
    /// <p><p> The custom MAIL FROM domain that you want the verified identity to use. The MAIL FROM domain must meet the following criteria:</p> <ul> <li> <p>It has to be a subdomain of the verified identity.</p> </li> <li> <p>It can&#39;t be used to receive email.</p> </li> <li> <p>It can&#39;t be used in a &quot;From&quot; address if the MAIL FROM domain is a destination for feedback forwarding emails.</p> </li> </ul></p>
    #[serde(rename = "MailFromDomain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mail_from_domain: Option<String>,
}

/// <p>An HTTP 200 response if the request succeeds, or an error message if the request fails.</p>
/// see [SesV2::put_email_identity_mail_from_attributes]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutEmailIdentityMailFromAttributesResponse {}

/// <p>A request to add an email destination to the suppression list for your account.</p>
/// see [SesV2::put_suppressed_destination]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutSuppressedDestinationRequest {
    /// <p>The email address that should be added to the suppression list for your account.</p>
    #[serde(rename = "EmailAddress")]
    pub email_address: String,
    /// <p>The factors that should cause the email address to be added to the suppression list for your account.</p>
    #[serde(rename = "Reason")]
    pub reason: String,
}

/// <p>An HTTP 200 response if the request succeeds, or an error message if the request fails.</p>
/// see [SesV2::put_suppressed_destination]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutSuppressedDestinationResponse {}

/// <p>Represents the raw content of an email message.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RawMessage {
    /// <p><p>The raw email message. The message has to meet the following criteria:</p> <ul> <li> <p>The message has to contain a header and a body, separated by one blank line.</p> </li> <li> <p>All of the required header fields must be present in the message.</p> </li> <li> <p>Each part of a multipart MIME message must be formatted properly.</p> </li> <li> <p>Attachments must be in a file format that the Amazon SES supports.</p> </li> <li> <p>The entire message must be Base64 encoded.</p> </li> <li> <p>If any of the MIME parts in your message contain content that is outside of the 7-bit ASCII character range, you should encode that content to ensure that recipients&#39; email clients render the message properly.</p> </li> <li> <p>The length of any single line of text in the message can&#39;t exceed 1,000 characters. This restriction is defined in <a href="https://tools.ietf.org/html/rfc5321">RFC 5321</a>.</p> </li> </ul></p>
    #[serde(rename = "Data")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    pub data: bytes::Bytes,
}

/// <p>The <code>ReplaceEmailContent</code> object to be used for a specific <code>BulkEmailEntry</code>. The <code>ReplacementTemplate</code> can be specified within this object.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ReplacementEmailContent {
    /// <p>The <code>ReplacementTemplate</code> associated with <code>ReplacementEmailContent</code>.</p>
    #[serde(rename = "ReplacementTemplate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replacement_template: Option<ReplacementTemplate>,
}

/// <p>An object which contains <code>ReplacementTemplateData</code> to be used for a specific <code>BulkEmailEntry</code>.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ReplacementTemplate {
    /// <p>A list of replacement values to apply to the template. This parameter is a JSON object, typically consisting of key-value pairs in which the keys correspond to replacement tags in the email template.</p>
    #[serde(rename = "ReplacementTemplateData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replacement_template_data: Option<String>,
}

/// <p>Enable or disable collection of reputation metrics for emails that you send using this configuration set in the current AWS Region. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ReputationOptions {
    /// <p>The date and time (in Unix time) when the reputation metrics were last given a fresh start. When your account is given a fresh start, your reputation metrics are calculated starting from the date of the fresh start.</p>
    #[serde(rename = "LastFreshStart")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_fresh_start: Option<f64>,
    /// <p>If <code>true</code>, tracking of reputation metrics is enabled for the configuration set. If <code>false</code>, tracking of reputation metrics is disabled for the configuration set.</p>
    #[serde(rename = "ReputationMetricsEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reputation_metrics_enabled: Option<bool>,
}

/// <p>An object that contains information about your account details review.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ReviewDetails {
    /// <p>The associated support center case ID (if any).</p>
    #[serde(rename = "CaseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub case_id: Option<String>,
    /// <p><p>The status of the latest review of your account. The status can be one of the following:</p> <ul> <li> <p> <code>PENDING</code> – We have received your appeal and are in the process of reviewing it.</p> </li> <li> <p> <code>GRANTED</code> – Your appeal has been reviewed and your production access has been granted.</p> </li> <li> <p> <code>DENIED</code> – Your appeal has been reviewed and your production access has been denied.</p> </li> <li> <p> <code>FAILED</code> – An internal error occurred and we didn&#39;t receive your appeal. You can submit your appeal again.</p> </li> </ul></p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>Represents a request to send email messages to multiple destinations using Amazon SES. For more information, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/send-personalized-email-api.html">Amazon SES Developer Guide</a>.</p>
/// see [SesV2::send_bulk_email]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SendBulkEmailRequest {
    /// <p>The list of bulk email entry objects.</p>
    #[serde(rename = "BulkEmailEntries")]
    pub bulk_email_entries: Vec<BulkEmailEntry>,
    /// <p>The name of the configuration set that you want to use when sending the email.</p>
    #[serde(rename = "ConfigurationSetName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_set_name: Option<String>,
    /// <p>An object that contains the body of the message. You can specify a template message.</p>
    #[serde(rename = "DefaultContent")]
    pub default_content: BulkEmailContent,
    /// <p>A list of tags, in the form of name/value pairs, to apply to an email that you send using the <code>SendEmail</code> operation. Tags correspond to characteristics of the email that you define, so that you can publish email sending events.</p>
    #[serde(rename = "DefaultEmailTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_email_tags: Option<Vec<MessageTag>>,
    /// <p>The address that you want bounce and complaint notifications to be sent to.</p>
    #[serde(rename = "FeedbackForwardingEmailAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feedback_forwarding_email_address: Option<String>,
    /// <p>This parameter is used only for sending authorization. It is the ARN of the identity that is associated with the sending authorization policy that permits you to use the email address specified in the <code>FeedbackForwardingEmailAddress</code> parameter.</p> <p>For example, if the owner of example.com (which has ARN arn:aws:ses:us-east-1:123456789012:identity/example.com) attaches a policy to it that authorizes you to use feedback@example.com, then you would specify the <code>FeedbackForwardingEmailAddressIdentityArn</code> to be arn:aws:ses:us-east-1:123456789012:identity/example.com, and the <code>FeedbackForwardingEmailAddress</code> to be feedback@example.com.</p> <p>For more information about sending authorization, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/sending-authorization.html">Amazon SES Developer Guide</a>.</p>
    #[serde(rename = "FeedbackForwardingEmailAddressIdentityArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feedback_forwarding_email_address_identity_arn: Option<String>,
    /// <p>The email address that you want to use as the "From" address for the email. The address that you specify has to be verified.</p>
    #[serde(rename = "FromEmailAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_email_address: Option<String>,
    /// <p>This parameter is used only for sending authorization. It is the ARN of the identity that is associated with the sending authorization policy that permits you to use the email address specified in the <code>FromEmailAddress</code> parameter.</p> <p>For example, if the owner of example.com (which has ARN arn:aws:ses:us-east-1:123456789012:identity/example.com) attaches a policy to it that authorizes you to use sender@example.com, then you would specify the <code>FromEmailAddressIdentityArn</code> to be arn:aws:ses:us-east-1:123456789012:identity/example.com, and the <code>FromEmailAddress</code> to be sender@example.com.</p> <p>For more information about sending authorization, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/sending-authorization.html">Amazon SES Developer Guide</a>.</p>
    #[serde(rename = "FromEmailAddressIdentityArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_email_address_identity_arn: Option<String>,
    /// <p>The "Reply-to" email addresses for the message. When the recipient replies to the message, each Reply-to address receives the reply.</p>
    #[serde(rename = "ReplyToAddresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_addresses: Option<Vec<String>>,
}

/// <p>The following data is returned in JSON format by the service.</p>
/// see [SesV2::send_bulk_email]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SendBulkEmailResponse {
    #[serde(rename = "BulkEmailEntryResults")]
    pub bulk_email_entry_results: Vec<BulkEmailEntryResult>,
}

/// <p>Represents a request to send a custom verification email to a specified recipient.</p>
/// see [SesV2::send_custom_verification_email]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SendCustomVerificationEmailRequest {
    /// <p>Name of a configuration set to use when sending the verification email.</p>
    #[serde(rename = "ConfigurationSetName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_set_name: Option<String>,
    /// <p>The email address to verify.</p>
    #[serde(rename = "EmailAddress")]
    pub email_address: String,
    /// <p>The name of the custom verification email template to use when sending the verification email.</p>
    #[serde(rename = "TemplateName")]
    pub template_name: String,
}

/// <p>The following element is returned by the service.</p>
/// see [SesV2::send_custom_verification_email]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SendCustomVerificationEmailResponse {
    /// <p>The unique message identifier returned from the <code>SendCustomVerificationEmail</code> operation.</p>
    #[serde(rename = "MessageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
}

/// <p>Represents a request to send a single formatted email using Amazon SES. For more information, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/send-email-formatted.html">Amazon SES Developer Guide</a>.</p>
/// see [SesV2::send_email]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SendEmailRequest {
    /// <p>The name of the configuration set that you want to use when sending the email.</p>
    #[serde(rename = "ConfigurationSetName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_set_name: Option<String>,
    /// <p>An object that contains the body of the message. You can send either a Simple message Raw message or a template Message.</p>
    #[serde(rename = "Content")]
    pub content: EmailContent,
    /// <p>An object that contains the recipients of the email message.</p>
    #[serde(rename = "Destination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<Destination>,
    /// <p>A list of tags, in the form of name/value pairs, to apply to an email that you send using the <code>SendEmail</code> operation. Tags correspond to characteristics of the email that you define, so that you can publish email sending events. </p>
    #[serde(rename = "EmailTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_tags: Option<Vec<MessageTag>>,
    /// <p>The address that you want bounce and complaint notifications to be sent to.</p>
    #[serde(rename = "FeedbackForwardingEmailAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feedback_forwarding_email_address: Option<String>,
    /// <p>This parameter is used only for sending authorization. It is the ARN of the identity that is associated with the sending authorization policy that permits you to use the email address specified in the <code>FeedbackForwardingEmailAddress</code> parameter.</p> <p>For example, if the owner of example.com (which has ARN arn:aws:ses:us-east-1:123456789012:identity/example.com) attaches a policy to it that authorizes you to use feedback@example.com, then you would specify the <code>FeedbackForwardingEmailAddressIdentityArn</code> to be arn:aws:ses:us-east-1:123456789012:identity/example.com, and the <code>FeedbackForwardingEmailAddress</code> to be feedback@example.com.</p> <p>For more information about sending authorization, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/sending-authorization.html">Amazon SES Developer Guide</a>.</p>
    #[serde(rename = "FeedbackForwardingEmailAddressIdentityArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feedback_forwarding_email_address_identity_arn: Option<String>,
    /// <p>The email address that you want to use as the "From" address for the email. The address that you specify has to be verified. </p>
    #[serde(rename = "FromEmailAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_email_address: Option<String>,
    /// <p>This parameter is used only for sending authorization. It is the ARN of the identity that is associated with the sending authorization policy that permits you to use the email address specified in the <code>FromEmailAddress</code> parameter.</p> <p>For example, if the owner of example.com (which has ARN arn:aws:ses:us-east-1:123456789012:identity/example.com) attaches a policy to it that authorizes you to use sender@example.com, then you would specify the <code>FromEmailAddressIdentityArn</code> to be arn:aws:ses:us-east-1:123456789012:identity/example.com, and the <code>FromEmailAddress</code> to be sender@example.com.</p> <p>For more information about sending authorization, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/sending-authorization.html">Amazon SES Developer Guide</a>.</p> <p>For Raw emails, the <code>FromEmailAddressIdentityArn</code> value overrides the X-SES-SOURCE-ARN and X-SES-FROM-ARN headers specified in raw email message content.</p>
    #[serde(rename = "FromEmailAddressIdentityArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_email_address_identity_arn: Option<String>,
    /// <p>An object used to specify a list or topic to which an email belongs, which will be used when a contact chooses to unsubscribe.</p>
    #[serde(rename = "ListManagementOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_management_options: Option<ListManagementOptions>,
    /// <p>The "Reply-to" email addresses for the message. When the recipient replies to the message, each Reply-to address receives the reply.</p>
    #[serde(rename = "ReplyToAddresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_addresses: Option<Vec<String>>,
}

/// <p>A unique message ID that you receive when an email is accepted for sending.</p>
/// see [SesV2::send_email]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SendEmailResponse {
    /// <p><p>A unique identifier for the message that is generated when the message is accepted.</p> <note> <p>It&#39;s possible for Amazon SES to accept a message without sending it. This can happen when the message that you&#39;re trying to send has an attachment contains a virus, or when you send a templated email that contains invalid personalization content, for example.</p> </note></p>
    #[serde(rename = "MessageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
}

/// <p>An object that contains information about the per-day and per-second sending limits for your Amazon SES account in the current AWS Region.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SendQuota {
    /// <p>The maximum number of emails that you can send in the current AWS Region over a 24-hour period. This value is also called your <i>sending quota</i>.</p>
    #[serde(rename = "Max24HourSend")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_24_hour_send: Option<f64>,
    /// <p>The maximum number of emails that you can send per second in the current AWS Region. This value is also called your <i>maximum sending rate</i> or your <i>maximum TPS (transactions per second) rate</i>.</p>
    #[serde(rename = "MaxSendRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_send_rate: Option<f64>,
    /// <p>The number of emails sent from your Amazon SES account in the current AWS Region over the past 24 hours.</p>
    #[serde(rename = "SentLast24Hours")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sent_last_24_hours: Option<f64>,
}

/// <p>Used to enable or disable email sending for messages that use this configuration set in the current AWS Region.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct SendingOptions {
    /// <p>If <code>true</code>, email sending is enabled for the configuration set. If <code>false</code>, email sending is disabled for the configuration set.</p>
    #[serde(rename = "SendingEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sending_enabled: Option<bool>,
}

/// <p>An object that defines an Amazon SNS destination for email events. You can use Amazon SNS to send notification when certain email events occur.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct SnsDestination {
    /// <p>The Amazon Resource Name (ARN) of the Amazon SNS topic that you want to publish email events to. For more information about Amazon SNS topics, see the <a href="https://docs.aws.amazon.com/sns/latest/dg/CreateTopic.html">Amazon SNS Developer Guide</a>.</p>
    #[serde(rename = "TopicArn")]
    pub topic_arn: String,
}

/// <p>An object that contains information about an email address that is on the suppression list for your account.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SuppressedDestination {
    /// <p>An optional value that can contain additional information about the reasons that the address was added to the suppression list for your account.</p>
    #[serde(rename = "Attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<SuppressedDestinationAttributes>,
    /// <p>The email address that is on the suppression list for your account.</p>
    #[serde(rename = "EmailAddress")]
    pub email_address: String,
    /// <p>The date and time when the suppressed destination was last updated, shown in Unix time format.</p>
    #[serde(rename = "LastUpdateTime")]
    pub last_update_time: f64,
    /// <p>The reason that the address was added to the suppression list for your account.</p>
    #[serde(rename = "Reason")]
    pub reason: String,
}

/// <p>An object that contains additional attributes that are related an email address that is on the suppression list for your account.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SuppressedDestinationAttributes {
    /// <p>A unique identifier that's generated when an email address is added to the suppression list for your account.</p>
    #[serde(rename = "FeedbackId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feedback_id: Option<String>,
    /// <p>The unique identifier of the email message that caused the email address to be added to the suppression list for your account.</p>
    #[serde(rename = "MessageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
}

/// <p>A summary that describes the suppressed email address.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SuppressedDestinationSummary {
    /// <p>The email address that's on the suppression list for your account.</p>
    #[serde(rename = "EmailAddress")]
    pub email_address: String,
    /// <p>The date and time when the suppressed destination was last updated, shown in Unix time format.</p>
    #[serde(rename = "LastUpdateTime")]
    pub last_update_time: f64,
    /// <p>The reason that the address was added to the suppression list for your account.</p>
    #[serde(rename = "Reason")]
    pub reason: String,
}

/// <p>An object that contains information about the email address suppression preferences for your account in the current AWS Region.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SuppressionAttributes {
    /// <p><p>A list that contains the reasons that email addresses will be automatically added to the suppression list for your account. This list can contain any or all of the following:</p> <ul> <li> <p> <code>COMPLAINT</code> – Amazon SES adds an email address to the suppression list for your account when a message sent to that address results in a complaint.</p> </li> <li> <p> <code>BOUNCE</code> – Amazon SES adds an email address to the suppression list for your account when a message sent to that address results in a hard bounce.</p> </li> </ul></p>
    #[serde(rename = "SuppressedReasons")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suppressed_reasons: Option<Vec<String>>,
}

/// <p>An object that contains details about the action of suppression list.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct SuppressionListDestination {
    /// <p><p>The type of action that you want to perform on the address. Acceptable values:</p> <ul> <li> <p>PUT: add the addresses to the suppression list. If the record already exists, it will override it with the new value.</p> </li> <li> <p>DELETE: remove the addresses from the suppression list.</p> </li> </ul></p>
    #[serde(rename = "SuppressionListImportAction")]
    pub suppression_list_import_action: String,
}

/// <p>An object that contains information about the suppression list preferences for your account.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct SuppressionOptions {
    /// <p><p>A list that contains the reasons that email addresses are automatically added to the suppression list for your account. This list can contain any or all of the following:</p> <ul> <li> <p> <code>COMPLAINT</code> – Amazon SES adds an email address to the suppression list for your account when a message sent to that address results in a complaint.</p> </li> <li> <p> <code>BOUNCE</code> – Amazon SES adds an email address to the suppression list for your account when a message sent to that address results in a hard bounce.</p> </li> </ul></p>
    #[serde(rename = "SuppressedReasons")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suppressed_reasons: Option<Vec<String>>,
}

/// <p><p>An object that defines the tags that are associated with a resource. A <i>tag</i> is a label that you optionally define and associate with a resource. Tags can help you categorize and manage resources in different ways, such as by purpose, owner, environment, or other criteria. A resource can have as many as 50 tags.</p> <p>Each tag consists of a required <i>tag key</i> and an associated <i>tag value</i>, both of which you define. A tag key is a general label that acts as a category for a more specific tag value. A tag value acts as a descriptor within a tag key. A tag key can contain as many as 128 characters. A tag value can contain as many as 256 characters. The characters can be Unicode letters, digits, white space, or one of the following symbols: _ . : / = + -. The following additional restrictions apply to tags:</p> <ul> <li> <p>Tag keys and values are case sensitive.</p> </li> <li> <p>For each associated resource, each tag key must be unique and it can have only one value.</p> </li> <li> <p>The <code>aws:</code> prefix is reserved for use by AWS; you can’t use it in any tag keys or values that you define. In addition, you can&#39;t edit or remove tag keys or values that use this prefix. Tags that use this prefix don’t count against the limit of 50 tags per resource.</p> </li> <li> <p>You can associate tags with public or shared resources, but the tags are available only for your AWS account, not any other accounts that share the resource. In addition, the tags are available only for resources that are located in the specified AWS Region for your AWS account.</p> </li> </ul></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Tag {
    /// <p>One part of a key-value pair that defines a tag. The maximum length of a tag key is 128 characters. The minimum length is 1 character.</p>
    #[serde(rename = "Key")]
    pub key: String,
    /// <p>The optional part of a key-value pair that defines a tag. The maximum length of a tag value is 256 characters. The minimum length is 0 characters. If you don't want a resource to have a specific tag value, don't specify a value for this parameter. If you don't specify a value, Amazon SES sets the value to an empty string.</p>
    #[serde(rename = "Value")]
    pub value: String,
}

/// see [SesV2::tag_resource]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the resource that you want to add one or more tags to.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <p>A list of the tags that you want to add to the resource. A tag consists of a required tag key (<code>Key</code>) and an associated tag value (<code>Value</code>). The maximum length of a tag key is 128 characters. The maximum length of a tag value is 256 characters.</p>
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,
}

/// see [SesV2::tag_resource]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TagResourceResponse {}

/// <p>An object that defines the email template to use for an email message, and the values to use for any message variables in that template. An <i>email template</i> is a type of message template that contains content that you want to define, save, and reuse in email messages that you send.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Template {
    /// <p>The Amazon Resource Name (ARN) of the template.</p>
    #[serde(rename = "TemplateArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_arn: Option<String>,
    /// <p>An object that defines the values to use for message variables in the template. This object is a set of key-value pairs. Each key defines a message variable in the template. The corresponding value defines the value to use for that variable.</p>
    #[serde(rename = "TemplateData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_data: Option<String>,
    /// <p>The name of the template. You will refer to this name when you send email using the <code>SendTemplatedEmail</code> or <code>SendBulkTemplatedEmail</code> operations. </p>
    #[serde(rename = "TemplateName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_name: Option<String>,
}

/// <p>&gt;Represents a request to create a preview of the MIME content of an email when provided with a template and a set of replacement data.</p>
/// see [SesV2::test_render_email_template]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TestRenderEmailTemplateRequest {
    /// <p>A list of replacement values to apply to the template. This parameter is a JSON object, typically consisting of key-value pairs in which the keys correspond to replacement tags in the email template.</p>
    #[serde(rename = "TemplateData")]
    pub template_data: String,
    /// <p>The name of the template that you want to render.</p>
    #[serde(rename = "TemplateName")]
    pub template_name: String,
}

/// <p>The following element is returned by the service.</p>
/// see [SesV2::test_render_email_template]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TestRenderEmailTemplateResponse {
    /// <p>The complete MIME message rendered by applying the data in the <code>TemplateData</code> parameter to the template specified in the TemplateName parameter.</p>
    #[serde(rename = "RenderedTemplate")]
    pub rendered_template: String,
}

/// <p>An interest group, theme, or label within a list. Lists can have multiple topics.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Topic {
    /// <p>The default subscription status to be applied to a contact if the contact has not noted their preference for subscribing to a topic.</p>
    #[serde(rename = "DefaultSubscriptionStatus")]
    pub default_subscription_status: String,
    /// <p>A description of what the topic is about, which the contact will see.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name of the topic the contact will see.</p>
    #[serde(rename = "DisplayName")]
    pub display_name: String,
    /// <p>The name of the topic.</p>
    #[serde(rename = "TopicName")]
    pub topic_name: String,
}

/// <p>Used for filtering by a specific topic preference.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TopicFilter {
    /// <p>The name of a topic on which you wish to apply the filter.</p>
    #[serde(rename = "TopicName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_name: Option<String>,
    /// <p>Notes that the default subscription status should be applied to a contact because the contact has not noted their preference for subscribing to a topic.</p>
    #[serde(rename = "UseDefaultIfPreferenceUnavailable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_default_if_preference_unavailable: Option<bool>,
}

/// <p>The contact's preference for being opted-in to or opted-out of a topic.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct TopicPreference {
    /// <p>The contact's subscription status to a topic which is either <code>OPT_IN</code> or <code>OPT_OUT</code>.</p>
    #[serde(rename = "SubscriptionStatus")]
    pub subscription_status: String,
    /// <p>The name of the topic.</p>
    #[serde(rename = "TopicName")]
    pub topic_name: String,
}

/// <p>An object that defines the tracking options for a configuration set. When you use the Amazon SES API v2 to send an email, it contains an invisible image that's used to track when recipients open your email. If your email contains links, those links are changed slightly in order to track when recipients click them.</p> <p>These images and links include references to a domain operated by AWS. You can optionally configure the Amazon SES to use a domain that you operate for these images and links.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct TrackingOptions {
    /// <p>The domain that you want to use for tracking open and click events.</p>
    #[serde(rename = "CustomRedirectDomain")]
    pub custom_redirect_domain: String,
}

/// see [SesV2::untag_resource]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the resource that you want to remove one or more tags from.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <p>The tags (tag keys) that you want to remove from the resource. When you specify a tag key, the action removes both that key and its associated tag value.</p> <p>To remove more than one tag from the resource, append the <code>TagKeys</code> parameter and argument for each additional tag to remove, separated by an ampersand. For example: <code>/v2/email/tags?ResourceArn=ResourceArn&amp;TagKeys=Key1&amp;TagKeys=Key2</code> </p>
    #[serde(rename = "TagKeys")]
    pub tag_keys: Vec<String>,
}

/// see [SesV2::untag_resource]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UntagResourceResponse {}

/// <p>A request to change the settings for an event destination for a configuration set.</p>
/// see [SesV2::update_configuration_set_event_destination]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateConfigurationSetEventDestinationRequest {
    /// <p>The name of the configuration set that contains the event destination that you want to modify.</p>
    #[serde(rename = "ConfigurationSetName")]
    pub configuration_set_name: String,
    /// <p>An object that defines the event destination.</p>
    #[serde(rename = "EventDestination")]
    pub event_destination: EventDestinationDefinition,
    /// <p>The name of the event destination that you want to modify.</p>
    #[serde(rename = "EventDestinationName")]
    pub event_destination_name: String,
}

/// <p>An HTTP 200 response if the request succeeds, or an error message if the request fails.</p>
/// see [SesV2::update_configuration_set_event_destination]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateConfigurationSetEventDestinationResponse {}

/// see [SesV2::update_contact_list]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateContactListRequest {
    /// <p>The name of the contact list.</p>
    #[serde(rename = "ContactListName")]
    pub contact_list_name: String,
    /// <p>A description of what the contact list is about.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>An interest group, theme, or label within a list. A contact list can have multiple topics.</p>
    #[serde(rename = "Topics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topics: Option<Vec<Topic>>,
}

/// see [SesV2::update_contact_list]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateContactListResponse {}

/// see [SesV2::update_contact]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateContactRequest {
    /// <p>The attribute data attached to a contact.</p>
    #[serde(rename = "AttributesData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes_data: Option<String>,
    /// <p>The name of the contact list.</p>
    #[serde(rename = "ContactListName")]
    pub contact_list_name: String,
    /// <p>The contact's email addres.</p>
    #[serde(rename = "EmailAddress")]
    pub email_address: String,
    /// <p>The contact's preference for being opted-in to or opted-out of a topic.</p>
    #[serde(rename = "TopicPreferences")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_preferences: Option<Vec<TopicPreference>>,
    /// <p>A boolean value status noting if the contact is unsubscribed from all contact list topics.</p>
    #[serde(rename = "UnsubscribeAll")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unsubscribe_all: Option<bool>,
}

/// see [SesV2::update_contact]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateContactResponse {}

/// <p>Represents a request to update an existing custom verification email template.</p>
/// see [SesV2::update_custom_verification_email_template]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateCustomVerificationEmailTemplateRequest {
    /// <p>The URL that the recipient of the verification email is sent to if his or her address is not successfully verified.</p>
    #[serde(rename = "FailureRedirectionURL")]
    pub failure_redirection_url: String,
    /// <p>The email address that the custom verification email is sent from.</p>
    #[serde(rename = "FromEmailAddress")]
    pub from_email_address: String,
    /// <p>The URL that the recipient of the verification email is sent to if his or her address is successfully verified.</p>
    #[serde(rename = "SuccessRedirectionURL")]
    pub success_redirection_url: String,
    /// <p>The content of the custom verification email. The total size of the email must be less than 10 MB. The message body may contain HTML, with some limitations. For more information, see <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/send-email-verify-address-custom.html#custom-verification-emails-faq">Custom Verification Email Frequently Asked Questions</a> in the <i>Amazon SES Developer Guide</i>.</p>
    #[serde(rename = "TemplateContent")]
    pub template_content: String,
    /// <p>The name of the custom verification email template that you want to update.</p>
    #[serde(rename = "TemplateName")]
    pub template_name: String,
    /// <p>The subject line of the custom verification email.</p>
    #[serde(rename = "TemplateSubject")]
    pub template_subject: String,
}

/// <p>If the action is successful, the service sends back an HTTP 200 response with an empty HTTP body.</p>
/// see [SesV2::update_custom_verification_email_template]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateCustomVerificationEmailTemplateResponse {}

/// <p>Represents a request to update a sending authorization policy for an identity. Sending authorization is an Amazon SES feature that enables you to authorize other senders to use your identities. For information, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/sending-authorization-identity-owner-tasks-management.html">Amazon SES Developer Guide</a>.</p>
/// see [SesV2::update_email_identity_policy]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateEmailIdentityPolicyRequest {
    /// <p>The email identity for which you want to update policy.</p>
    #[serde(rename = "EmailIdentity")]
    pub email_identity: String,
    /// <p>The text of the policy in JSON format. The policy cannot exceed 4 KB.</p> <p> For information about the syntax of sending authorization policies, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/sending-authorization-policies.html">Amazon SES Developer Guide</a>.</p>
    #[serde(rename = "Policy")]
    pub policy: String,
    /// <p>The name of the policy.</p> <p>The policy name cannot exceed 64 characters and can only include alphanumeric characters, dashes, and underscores.</p>
    #[serde(rename = "PolicyName")]
    pub policy_name: String,
}

/// <p>An HTTP 200 response if the request succeeds, or an error message if the request fails.</p>
/// see [SesV2::update_email_identity_policy]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateEmailIdentityPolicyResponse {}

/// <p>Represents a request to update an email template. For more information, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/send-personalized-email-api.html">Amazon SES Developer Guide</a>.</p>
/// see [SesV2::update_email_template]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateEmailTemplateRequest {
    /// <p>The content of the email template, composed of a subject line, an HTML part, and a text-only part.</p>
    #[serde(rename = "TemplateContent")]
    pub template_content: EmailTemplateContent,
    /// <p>The name of the template you want to update.</p>
    #[serde(rename = "TemplateName")]
    pub template_name: String,
}

/// <p>If the action is successful, the service sends back an HTTP 200 response with an empty HTTP body.</p>
/// see [SesV2::update_email_template]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateEmailTemplateResponse {}

/// <p>An object that contains information about the amount of email that was delivered to recipients.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct VolumeStatistics {
    /// <p>The total number of emails that arrived in recipients' inboxes.</p>
    #[serde(rename = "InboxRawCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbox_raw_count: Option<i64>,
    /// <p>An estimate of the percentage of emails sent from the current domain that will arrive in recipients' inboxes.</p>
    #[serde(rename = "ProjectedInbox")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projected_inbox: Option<i64>,
    /// <p>An estimate of the percentage of emails sent from the current domain that will arrive in recipients' spam or junk mail folders.</p>
    #[serde(rename = "ProjectedSpam")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projected_spam: Option<i64>,
    /// <p>The total number of emails that arrived in recipients' spam or junk mail folders.</p>
    #[serde(rename = "SpamRawCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spam_raw_count: Option<i64>,
}

/// Errors returned by CreateConfigurationSet
#[derive(Debug, PartialEq)]
pub enum CreateConfigurationSetError {
    /// <p>The resource specified in your request already exists.</p>
    AlreadyExists(String),
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>The resource is being modified by another operation or thread.</p>
    ConcurrentModification(String),
    /// <p>There are too many instances of the specified resource type.</p>
    LimitExceeded(String),
    /// <p>The resource you attempted to access doesn't exist.</p>
    NotFound(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl CreateConfigurationSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateConfigurationSetError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AlreadyExistsException" => {
                    return RusotoError::Service(CreateConfigurationSetError::AlreadyExists(
                        err.msg,
                    ))
                }
                "BadRequestException" => {
                    return RusotoError::Service(CreateConfigurationSetError::BadRequest(err.msg))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(
                        CreateConfigurationSetError::ConcurrentModification(err.msg),
                    )
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateConfigurationSetError::LimitExceeded(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateConfigurationSetError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateConfigurationSetError::TooManyRequests(
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
impl fmt::Display for CreateConfigurationSetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateConfigurationSetError::AlreadyExists(ref cause) => write!(f, "{}", cause),
            CreateConfigurationSetError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateConfigurationSetError::ConcurrentModification(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateConfigurationSetError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateConfigurationSetError::NotFound(ref cause) => write!(f, "{}", cause),
            CreateConfigurationSetError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateConfigurationSetError {}
/// Errors returned by CreateConfigurationSetEventDestination
#[derive(Debug, PartialEq)]
pub enum CreateConfigurationSetEventDestinationError {
    /// <p>The resource specified in your request already exists.</p>
    AlreadyExists(String),
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>There are too many instances of the specified resource type.</p>
    LimitExceeded(String),
    /// <p>The resource you attempted to access doesn't exist.</p>
    NotFound(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl CreateConfigurationSetEventDestinationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateConfigurationSetEventDestinationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AlreadyExistsException" => {
                    return RusotoError::Service(
                        CreateConfigurationSetEventDestinationError::AlreadyExists(err.msg),
                    )
                }
                "BadRequestException" => {
                    return RusotoError::Service(
                        CreateConfigurationSetEventDestinationError::BadRequest(err.msg),
                    )
                }
                "LimitExceededException" => {
                    return RusotoError::Service(
                        CreateConfigurationSetEventDestinationError::LimitExceeded(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(
                        CreateConfigurationSetEventDestinationError::NotFound(err.msg),
                    )
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        CreateConfigurationSetEventDestinationError::TooManyRequests(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateConfigurationSetEventDestinationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateConfigurationSetEventDestinationError::AlreadyExists(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateConfigurationSetEventDestinationError::BadRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateConfigurationSetEventDestinationError::LimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateConfigurationSetEventDestinationError::NotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateConfigurationSetEventDestinationError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for CreateConfigurationSetEventDestinationError {}
/// Errors returned by CreateContact
#[derive(Debug, PartialEq)]
pub enum CreateContactError {
    /// <p>The resource specified in your request already exists.</p>
    AlreadyExists(String),
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>The resource you attempted to access doesn't exist.</p>
    NotFound(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl CreateContactError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateContactError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AlreadyExistsException" => {
                    return RusotoError::Service(CreateContactError::AlreadyExists(err.msg))
                }
                "BadRequestException" => {
                    return RusotoError::Service(CreateContactError::BadRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateContactError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateContactError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateContactError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateContactError::AlreadyExists(ref cause) => write!(f, "{}", cause),
            CreateContactError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateContactError::NotFound(ref cause) => write!(f, "{}", cause),
            CreateContactError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateContactError {}
/// Errors returned by CreateContactList
#[derive(Debug, PartialEq)]
pub enum CreateContactListError {
    /// <p>The resource specified in your request already exists.</p>
    AlreadyExists(String),
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>There are too many instances of the specified resource type.</p>
    LimitExceeded(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl CreateContactListError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateContactListError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AlreadyExistsException" => {
                    return RusotoError::Service(CreateContactListError::AlreadyExists(err.msg))
                }
                "BadRequestException" => {
                    return RusotoError::Service(CreateContactListError::BadRequest(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateContactListError::LimitExceeded(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateContactListError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateContactListError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateContactListError::AlreadyExists(ref cause) => write!(f, "{}", cause),
            CreateContactListError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateContactListError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateContactListError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateContactListError {}
/// Errors returned by CreateCustomVerificationEmailTemplate
#[derive(Debug, PartialEq)]
pub enum CreateCustomVerificationEmailTemplateError {
    /// <p>The resource specified in your request already exists.</p>
    AlreadyExists(String),
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>There are too many instances of the specified resource type.</p>
    LimitExceeded(String),
    /// <p>The resource you attempted to access doesn't exist.</p>
    NotFound(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl CreateCustomVerificationEmailTemplateError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateCustomVerificationEmailTemplateError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AlreadyExistsException" => {
                    return RusotoError::Service(
                        CreateCustomVerificationEmailTemplateError::AlreadyExists(err.msg),
                    )
                }
                "BadRequestException" => {
                    return RusotoError::Service(
                        CreateCustomVerificationEmailTemplateError::BadRequest(err.msg),
                    )
                }
                "LimitExceededException" => {
                    return RusotoError::Service(
                        CreateCustomVerificationEmailTemplateError::LimitExceeded(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(
                        CreateCustomVerificationEmailTemplateError::NotFound(err.msg),
                    )
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        CreateCustomVerificationEmailTemplateError::TooManyRequests(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateCustomVerificationEmailTemplateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateCustomVerificationEmailTemplateError::AlreadyExists(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateCustomVerificationEmailTemplateError::BadRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateCustomVerificationEmailTemplateError::LimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateCustomVerificationEmailTemplateError::NotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateCustomVerificationEmailTemplateError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for CreateCustomVerificationEmailTemplateError {}
/// Errors returned by CreateDedicatedIpPool
#[derive(Debug, PartialEq)]
pub enum CreateDedicatedIpPoolError {
    /// <p>The resource specified in your request already exists.</p>
    AlreadyExists(String),
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>The resource is being modified by another operation or thread.</p>
    ConcurrentModification(String),
    /// <p>There are too many instances of the specified resource type.</p>
    LimitExceeded(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl CreateDedicatedIpPoolError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateDedicatedIpPoolError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AlreadyExistsException" => {
                    return RusotoError::Service(CreateDedicatedIpPoolError::AlreadyExists(err.msg))
                }
                "BadRequestException" => {
                    return RusotoError::Service(CreateDedicatedIpPoolError::BadRequest(err.msg))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(
                        CreateDedicatedIpPoolError::ConcurrentModification(err.msg),
                    )
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateDedicatedIpPoolError::LimitExceeded(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateDedicatedIpPoolError::TooManyRequests(
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
impl fmt::Display for CreateDedicatedIpPoolError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateDedicatedIpPoolError::AlreadyExists(ref cause) => write!(f, "{}", cause),
            CreateDedicatedIpPoolError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateDedicatedIpPoolError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            CreateDedicatedIpPoolError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateDedicatedIpPoolError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateDedicatedIpPoolError {}
/// Errors returned by CreateDeliverabilityTestReport
#[derive(Debug, PartialEq)]
pub enum CreateDeliverabilityTestReportError {
    /// <p>The message can't be sent because the account's ability to send email has been permanently restricted.</p>
    AccountSuspended(String),
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>The resource is being modified by another operation or thread.</p>
    ConcurrentModification(String),
    /// <p>There are too many instances of the specified resource type.</p>
    LimitExceeded(String),
    /// <p>The message can't be sent because the sending domain isn't verified.</p>
    MailFromDomainNotVerified(String),
    /// <p>The message can't be sent because it contains invalid content.</p>
    MessageRejected(String),
    /// <p>The resource you attempted to access doesn't exist.</p>
    NotFound(String),
    /// <p>The message can't be sent because the account's ability to send email is currently paused.</p>
    SendingPaused(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl CreateDeliverabilityTestReportError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateDeliverabilityTestReportError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccountSuspendedException" => {
                    return RusotoError::Service(
                        CreateDeliverabilityTestReportError::AccountSuspended(err.msg),
                    )
                }
                "BadRequestException" => {
                    return RusotoError::Service(CreateDeliverabilityTestReportError::BadRequest(
                        err.msg,
                    ))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(
                        CreateDeliverabilityTestReportError::ConcurrentModification(err.msg),
                    )
                }
                "LimitExceededException" => {
                    return RusotoError::Service(
                        CreateDeliverabilityTestReportError::LimitExceeded(err.msg),
                    )
                }
                "MailFromDomainNotVerifiedException" => {
                    return RusotoError::Service(
                        CreateDeliverabilityTestReportError::MailFromDomainNotVerified(err.msg),
                    )
                }
                "MessageRejected" => {
                    return RusotoError::Service(
                        CreateDeliverabilityTestReportError::MessageRejected(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateDeliverabilityTestReportError::NotFound(
                        err.msg,
                    ))
                }
                "SendingPausedException" => {
                    return RusotoError::Service(
                        CreateDeliverabilityTestReportError::SendingPaused(err.msg),
                    )
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        CreateDeliverabilityTestReportError::TooManyRequests(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateDeliverabilityTestReportError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateDeliverabilityTestReportError::AccountSuspended(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateDeliverabilityTestReportError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateDeliverabilityTestReportError::ConcurrentModification(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateDeliverabilityTestReportError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateDeliverabilityTestReportError::MailFromDomainNotVerified(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateDeliverabilityTestReportError::MessageRejected(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateDeliverabilityTestReportError::NotFound(ref cause) => write!(f, "{}", cause),
            CreateDeliverabilityTestReportError::SendingPaused(ref cause) => write!(f, "{}", cause),
            CreateDeliverabilityTestReportError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for CreateDeliverabilityTestReportError {}
/// Errors returned by CreateEmailIdentity
#[derive(Debug, PartialEq)]
pub enum CreateEmailIdentityError {
    /// <p>The resource specified in your request already exists.</p>
    AlreadyExists(String),
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>The resource is being modified by another operation or thread.</p>
    ConcurrentModification(String),
    /// <p>There are too many instances of the specified resource type.</p>
    LimitExceeded(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl CreateEmailIdentityError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateEmailIdentityError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AlreadyExistsException" => {
                    return RusotoError::Service(CreateEmailIdentityError::AlreadyExists(err.msg))
                }
                "BadRequestException" => {
                    return RusotoError::Service(CreateEmailIdentityError::BadRequest(err.msg))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(CreateEmailIdentityError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateEmailIdentityError::LimitExceeded(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateEmailIdentityError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateEmailIdentityError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateEmailIdentityError::AlreadyExists(ref cause) => write!(f, "{}", cause),
            CreateEmailIdentityError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateEmailIdentityError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            CreateEmailIdentityError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateEmailIdentityError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateEmailIdentityError {}
/// Errors returned by CreateEmailIdentityPolicy
#[derive(Debug, PartialEq)]
pub enum CreateEmailIdentityPolicyError {
    /// <p>The resource specified in your request already exists.</p>
    AlreadyExists(String),
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>There are too many instances of the specified resource type.</p>
    LimitExceeded(String),
    /// <p>The resource you attempted to access doesn't exist.</p>
    NotFound(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl CreateEmailIdentityPolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateEmailIdentityPolicyError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AlreadyExistsException" => {
                    return RusotoError::Service(CreateEmailIdentityPolicyError::AlreadyExists(
                        err.msg,
                    ))
                }
                "BadRequestException" => {
                    return RusotoError::Service(CreateEmailIdentityPolicyError::BadRequest(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateEmailIdentityPolicyError::LimitExceeded(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateEmailIdentityPolicyError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateEmailIdentityPolicyError::TooManyRequests(
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
impl fmt::Display for CreateEmailIdentityPolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateEmailIdentityPolicyError::AlreadyExists(ref cause) => write!(f, "{}", cause),
            CreateEmailIdentityPolicyError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateEmailIdentityPolicyError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateEmailIdentityPolicyError::NotFound(ref cause) => write!(f, "{}", cause),
            CreateEmailIdentityPolicyError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateEmailIdentityPolicyError {}
/// Errors returned by CreateEmailTemplate
#[derive(Debug, PartialEq)]
pub enum CreateEmailTemplateError {
    /// <p>The resource specified in your request already exists.</p>
    AlreadyExists(String),
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>There are too many instances of the specified resource type.</p>
    LimitExceeded(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl CreateEmailTemplateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateEmailTemplateError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AlreadyExistsException" => {
                    return RusotoError::Service(CreateEmailTemplateError::AlreadyExists(err.msg))
                }
                "BadRequestException" => {
                    return RusotoError::Service(CreateEmailTemplateError::BadRequest(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateEmailTemplateError::LimitExceeded(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateEmailTemplateError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateEmailTemplateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateEmailTemplateError::AlreadyExists(ref cause) => write!(f, "{}", cause),
            CreateEmailTemplateError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateEmailTemplateError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateEmailTemplateError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateEmailTemplateError {}
/// Errors returned by CreateImportJob
#[derive(Debug, PartialEq)]
pub enum CreateImportJobError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>There are too many instances of the specified resource type.</p>
    LimitExceeded(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl CreateImportJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateImportJobError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateImportJobError::BadRequest(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateImportJobError::LimitExceeded(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateImportJobError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateImportJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateImportJobError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateImportJobError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateImportJobError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateImportJobError {}
/// Errors returned by DeleteConfigurationSet
#[derive(Debug, PartialEq)]
pub enum DeleteConfigurationSetError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>The resource is being modified by another operation or thread.</p>
    ConcurrentModification(String),
    /// <p>The resource you attempted to access doesn't exist.</p>
    NotFound(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl DeleteConfigurationSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteConfigurationSetError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteConfigurationSetError::BadRequest(err.msg))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(
                        DeleteConfigurationSetError::ConcurrentModification(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteConfigurationSetError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteConfigurationSetError::TooManyRequests(
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
impl fmt::Display for DeleteConfigurationSetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteConfigurationSetError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteConfigurationSetError::ConcurrentModification(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteConfigurationSetError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteConfigurationSetError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteConfigurationSetError {}
/// Errors returned by DeleteConfigurationSetEventDestination
#[derive(Debug, PartialEq)]
pub enum DeleteConfigurationSetEventDestinationError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>The resource you attempted to access doesn't exist.</p>
    NotFound(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl DeleteConfigurationSetEventDestinationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteConfigurationSetEventDestinationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(
                        DeleteConfigurationSetEventDestinationError::BadRequest(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(
                        DeleteConfigurationSetEventDestinationError::NotFound(err.msg),
                    )
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        DeleteConfigurationSetEventDestinationError::TooManyRequests(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteConfigurationSetEventDestinationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteConfigurationSetEventDestinationError::BadRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteConfigurationSetEventDestinationError::NotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteConfigurationSetEventDestinationError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DeleteConfigurationSetEventDestinationError {}
/// Errors returned by DeleteContact
#[derive(Debug, PartialEq)]
pub enum DeleteContactError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>The resource you attempted to access doesn't exist.</p>
    NotFound(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl DeleteContactError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteContactError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteContactError::BadRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteContactError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteContactError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteContactError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteContactError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteContactError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteContactError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteContactError {}
/// Errors returned by DeleteContactList
#[derive(Debug, PartialEq)]
pub enum DeleteContactListError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>The resource is being modified by another operation or thread.</p>
    ConcurrentModification(String),
    /// <p>The resource you attempted to access doesn't exist.</p>
    NotFound(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl DeleteContactListError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteContactListError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteContactListError::BadRequest(err.msg))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(DeleteContactListError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteContactListError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteContactListError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteContactListError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteContactListError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteContactListError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            DeleteContactListError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteContactListError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteContactListError {}
/// Errors returned by DeleteCustomVerificationEmailTemplate
#[derive(Debug, PartialEq)]
pub enum DeleteCustomVerificationEmailTemplateError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>The resource you attempted to access doesn't exist.</p>
    NotFound(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl DeleteCustomVerificationEmailTemplateError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteCustomVerificationEmailTemplateError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(
                        DeleteCustomVerificationEmailTemplateError::BadRequest(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(
                        DeleteCustomVerificationEmailTemplateError::NotFound(err.msg),
                    )
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        DeleteCustomVerificationEmailTemplateError::TooManyRequests(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteCustomVerificationEmailTemplateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteCustomVerificationEmailTemplateError::BadRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteCustomVerificationEmailTemplateError::NotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteCustomVerificationEmailTemplateError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DeleteCustomVerificationEmailTemplateError {}
/// Errors returned by DeleteDedicatedIpPool
#[derive(Debug, PartialEq)]
pub enum DeleteDedicatedIpPoolError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>The resource is being modified by another operation or thread.</p>
    ConcurrentModification(String),
    /// <p>The resource you attempted to access doesn't exist.</p>
    NotFound(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl DeleteDedicatedIpPoolError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteDedicatedIpPoolError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteDedicatedIpPoolError::BadRequest(err.msg))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(
                        DeleteDedicatedIpPoolError::ConcurrentModification(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteDedicatedIpPoolError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteDedicatedIpPoolError::TooManyRequests(
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
impl fmt::Display for DeleteDedicatedIpPoolError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteDedicatedIpPoolError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteDedicatedIpPoolError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            DeleteDedicatedIpPoolError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteDedicatedIpPoolError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteDedicatedIpPoolError {}
/// Errors returned by DeleteEmailIdentity
#[derive(Debug, PartialEq)]
pub enum DeleteEmailIdentityError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>The resource is being modified by another operation or thread.</p>
    ConcurrentModification(String),
    /// <p>The resource you attempted to access doesn't exist.</p>
    NotFound(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl DeleteEmailIdentityError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteEmailIdentityError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteEmailIdentityError::BadRequest(err.msg))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(DeleteEmailIdentityError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteEmailIdentityError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteEmailIdentityError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteEmailIdentityError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteEmailIdentityError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteEmailIdentityError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            DeleteEmailIdentityError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteEmailIdentityError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteEmailIdentityError {}
/// Errors returned by DeleteEmailIdentityPolicy
#[derive(Debug, PartialEq)]
pub enum DeleteEmailIdentityPolicyError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>The resource you attempted to access doesn't exist.</p>
    NotFound(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl DeleteEmailIdentityPolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteEmailIdentityPolicyError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteEmailIdentityPolicyError::BadRequest(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteEmailIdentityPolicyError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteEmailIdentityPolicyError::TooManyRequests(
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
impl fmt::Display for DeleteEmailIdentityPolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteEmailIdentityPolicyError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteEmailIdentityPolicyError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteEmailIdentityPolicyError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteEmailIdentityPolicyError {}
/// Errors returned by DeleteEmailTemplate
#[derive(Debug, PartialEq)]
pub enum DeleteEmailTemplateError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>The resource you attempted to access doesn't exist.</p>
    NotFound(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl DeleteEmailTemplateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteEmailTemplateError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteEmailTemplateError::BadRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteEmailTemplateError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteEmailTemplateError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteEmailTemplateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteEmailTemplateError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteEmailTemplateError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteEmailTemplateError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteEmailTemplateError {}
/// Errors returned by DeleteSuppressedDestination
#[derive(Debug, PartialEq)]
pub enum DeleteSuppressedDestinationError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>The resource you attempted to access doesn't exist.</p>
    NotFound(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl DeleteSuppressedDestinationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteSuppressedDestinationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteSuppressedDestinationError::BadRequest(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteSuppressedDestinationError::NotFound(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteSuppressedDestinationError::TooManyRequests(
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
impl fmt::Display for DeleteSuppressedDestinationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteSuppressedDestinationError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteSuppressedDestinationError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteSuppressedDestinationError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteSuppressedDestinationError {}
/// Errors returned by GetAccount
#[derive(Debug, PartialEq)]
pub enum GetAccountError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl GetAccountError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetAccountError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetAccountError::BadRequest(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetAccountError::TooManyRequests(err.msg))
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
            GetAccountError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetAccountError {}
/// Errors returned by GetBlacklistReports
#[derive(Debug, PartialEq)]
pub enum GetBlacklistReportsError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>The resource you attempted to access doesn't exist.</p>
    NotFound(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl GetBlacklistReportsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetBlacklistReportsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetBlacklistReportsError::BadRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetBlacklistReportsError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetBlacklistReportsError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetBlacklistReportsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetBlacklistReportsError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetBlacklistReportsError::NotFound(ref cause) => write!(f, "{}", cause),
            GetBlacklistReportsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetBlacklistReportsError {}
/// Errors returned by GetConfigurationSet
#[derive(Debug, PartialEq)]
pub enum GetConfigurationSetError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>The resource you attempted to access doesn't exist.</p>
    NotFound(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl GetConfigurationSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetConfigurationSetError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetConfigurationSetError::BadRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetConfigurationSetError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetConfigurationSetError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetConfigurationSetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetConfigurationSetError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetConfigurationSetError::NotFound(ref cause) => write!(f, "{}", cause),
            GetConfigurationSetError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetConfigurationSetError {}
/// Errors returned by GetConfigurationSetEventDestinations
#[derive(Debug, PartialEq)]
pub enum GetConfigurationSetEventDestinationsError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>The resource you attempted to access doesn't exist.</p>
    NotFound(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl GetConfigurationSetEventDestinationsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetConfigurationSetEventDestinationsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(
                        GetConfigurationSetEventDestinationsError::BadRequest(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(
                        GetConfigurationSetEventDestinationsError::NotFound(err.msg),
                    )
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        GetConfigurationSetEventDestinationsError::TooManyRequests(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetConfigurationSetEventDestinationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetConfigurationSetEventDestinationsError::BadRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            GetConfigurationSetEventDestinationsError::NotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            GetConfigurationSetEventDestinationsError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for GetConfigurationSetEventDestinationsError {}
/// Errors returned by GetContact
#[derive(Debug, PartialEq)]
pub enum GetContactError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>The resource you attempted to access doesn't exist.</p>
    NotFound(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl GetContactError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetContactError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetContactError::BadRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetContactError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetContactError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetContactError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetContactError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetContactError::NotFound(ref cause) => write!(f, "{}", cause),
            GetContactError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetContactError {}
/// Errors returned by GetContactList
#[derive(Debug, PartialEq)]
pub enum GetContactListError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>The resource you attempted to access doesn't exist.</p>
    NotFound(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl GetContactListError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetContactListError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetContactListError::BadRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetContactListError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetContactListError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetContactListError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetContactListError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetContactListError::NotFound(ref cause) => write!(f, "{}", cause),
            GetContactListError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetContactListError {}
/// Errors returned by GetCustomVerificationEmailTemplate
#[derive(Debug, PartialEq)]
pub enum GetCustomVerificationEmailTemplateError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>The resource you attempted to access doesn't exist.</p>
    NotFound(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl GetCustomVerificationEmailTemplateError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetCustomVerificationEmailTemplateError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(
                        GetCustomVerificationEmailTemplateError::BadRequest(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetCustomVerificationEmailTemplateError::NotFound(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        GetCustomVerificationEmailTemplateError::TooManyRequests(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetCustomVerificationEmailTemplateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetCustomVerificationEmailTemplateError::BadRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            GetCustomVerificationEmailTemplateError::NotFound(ref cause) => write!(f, "{}", cause),
            GetCustomVerificationEmailTemplateError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for GetCustomVerificationEmailTemplateError {}
/// Errors returned by GetDedicatedIp
#[derive(Debug, PartialEq)]
pub enum GetDedicatedIpError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>The resource you attempted to access doesn't exist.</p>
    NotFound(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl GetDedicatedIpError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetDedicatedIpError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetDedicatedIpError::BadRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetDedicatedIpError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetDedicatedIpError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetDedicatedIpError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetDedicatedIpError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetDedicatedIpError::NotFound(ref cause) => write!(f, "{}", cause),
            GetDedicatedIpError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetDedicatedIpError {}
/// Errors returned by GetDedicatedIps
#[derive(Debug, PartialEq)]
pub enum GetDedicatedIpsError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>The resource you attempted to access doesn't exist.</p>
    NotFound(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl GetDedicatedIpsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetDedicatedIpsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetDedicatedIpsError::BadRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetDedicatedIpsError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetDedicatedIpsError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetDedicatedIpsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetDedicatedIpsError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetDedicatedIpsError::NotFound(ref cause) => write!(f, "{}", cause),
            GetDedicatedIpsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetDedicatedIpsError {}
/// Errors returned by GetDeliverabilityDashboardOptions
#[derive(Debug, PartialEq)]
pub enum GetDeliverabilityDashboardOptionsError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>There are too many instances of the specified resource type.</p>
    LimitExceeded(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl GetDeliverabilityDashboardOptionsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetDeliverabilityDashboardOptionsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(
                        GetDeliverabilityDashboardOptionsError::BadRequest(err.msg),
                    )
                }
                "LimitExceededException" => {
                    return RusotoError::Service(
                        GetDeliverabilityDashboardOptionsError::LimitExceeded(err.msg),
                    )
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        GetDeliverabilityDashboardOptionsError::TooManyRequests(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetDeliverabilityDashboardOptionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetDeliverabilityDashboardOptionsError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetDeliverabilityDashboardOptionsError::LimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            GetDeliverabilityDashboardOptionsError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for GetDeliverabilityDashboardOptionsError {}
/// Errors returned by GetDeliverabilityTestReport
#[derive(Debug, PartialEq)]
pub enum GetDeliverabilityTestReportError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>The resource you attempted to access doesn't exist.</p>
    NotFound(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl GetDeliverabilityTestReportError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetDeliverabilityTestReportError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetDeliverabilityTestReportError::BadRequest(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetDeliverabilityTestReportError::NotFound(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetDeliverabilityTestReportError::TooManyRequests(
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
impl fmt::Display for GetDeliverabilityTestReportError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetDeliverabilityTestReportError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetDeliverabilityTestReportError::NotFound(ref cause) => write!(f, "{}", cause),
            GetDeliverabilityTestReportError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetDeliverabilityTestReportError {}
/// Errors returned by GetDomainDeliverabilityCampaign
#[derive(Debug, PartialEq)]
pub enum GetDomainDeliverabilityCampaignError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>The resource you attempted to access doesn't exist.</p>
    NotFound(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl GetDomainDeliverabilityCampaignError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetDomainDeliverabilityCampaignError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetDomainDeliverabilityCampaignError::BadRequest(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetDomainDeliverabilityCampaignError::NotFound(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        GetDomainDeliverabilityCampaignError::TooManyRequests(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetDomainDeliverabilityCampaignError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetDomainDeliverabilityCampaignError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetDomainDeliverabilityCampaignError::NotFound(ref cause) => write!(f, "{}", cause),
            GetDomainDeliverabilityCampaignError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for GetDomainDeliverabilityCampaignError {}
/// Errors returned by GetDomainStatisticsReport
#[derive(Debug, PartialEq)]
pub enum GetDomainStatisticsReportError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>The resource you attempted to access doesn't exist.</p>
    NotFound(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl GetDomainStatisticsReportError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetDomainStatisticsReportError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetDomainStatisticsReportError::BadRequest(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetDomainStatisticsReportError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetDomainStatisticsReportError::TooManyRequests(
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
impl fmt::Display for GetDomainStatisticsReportError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetDomainStatisticsReportError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetDomainStatisticsReportError::NotFound(ref cause) => write!(f, "{}", cause),
            GetDomainStatisticsReportError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetDomainStatisticsReportError {}
/// Errors returned by GetEmailIdentity
#[derive(Debug, PartialEq)]
pub enum GetEmailIdentityError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>The resource you attempted to access doesn't exist.</p>
    NotFound(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl GetEmailIdentityError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetEmailIdentityError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetEmailIdentityError::BadRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetEmailIdentityError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetEmailIdentityError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetEmailIdentityError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetEmailIdentityError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetEmailIdentityError::NotFound(ref cause) => write!(f, "{}", cause),
            GetEmailIdentityError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetEmailIdentityError {}
/// Errors returned by GetEmailIdentityPolicies
#[derive(Debug, PartialEq)]
pub enum GetEmailIdentityPoliciesError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>The resource you attempted to access doesn't exist.</p>
    NotFound(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl GetEmailIdentityPoliciesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetEmailIdentityPoliciesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetEmailIdentityPoliciesError::BadRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetEmailIdentityPoliciesError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetEmailIdentityPoliciesError::TooManyRequests(
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
impl fmt::Display for GetEmailIdentityPoliciesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetEmailIdentityPoliciesError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetEmailIdentityPoliciesError::NotFound(ref cause) => write!(f, "{}", cause),
            GetEmailIdentityPoliciesError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetEmailIdentityPoliciesError {}
/// Errors returned by GetEmailTemplate
#[derive(Debug, PartialEq)]
pub enum GetEmailTemplateError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>The resource you attempted to access doesn't exist.</p>
    NotFound(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl GetEmailTemplateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetEmailTemplateError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetEmailTemplateError::BadRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetEmailTemplateError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetEmailTemplateError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetEmailTemplateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetEmailTemplateError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetEmailTemplateError::NotFound(ref cause) => write!(f, "{}", cause),
            GetEmailTemplateError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetEmailTemplateError {}
/// Errors returned by GetImportJob
#[derive(Debug, PartialEq)]
pub enum GetImportJobError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>The resource you attempted to access doesn't exist.</p>
    NotFound(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl GetImportJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetImportJobError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetImportJobError::BadRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetImportJobError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetImportJobError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetImportJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetImportJobError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetImportJobError::NotFound(ref cause) => write!(f, "{}", cause),
            GetImportJobError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetImportJobError {}
/// Errors returned by GetSuppressedDestination
#[derive(Debug, PartialEq)]
pub enum GetSuppressedDestinationError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>The resource you attempted to access doesn't exist.</p>
    NotFound(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl GetSuppressedDestinationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetSuppressedDestinationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetSuppressedDestinationError::BadRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetSuppressedDestinationError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetSuppressedDestinationError::TooManyRequests(
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
impl fmt::Display for GetSuppressedDestinationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetSuppressedDestinationError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetSuppressedDestinationError::NotFound(ref cause) => write!(f, "{}", cause),
            GetSuppressedDestinationError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetSuppressedDestinationError {}
/// Errors returned by ListConfigurationSets
#[derive(Debug, PartialEq)]
pub enum ListConfigurationSetsError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl ListConfigurationSetsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListConfigurationSetsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListConfigurationSetsError::BadRequest(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListConfigurationSetsError::TooManyRequests(
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
impl fmt::Display for ListConfigurationSetsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListConfigurationSetsError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListConfigurationSetsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListConfigurationSetsError {}
/// Errors returned by ListContactLists
#[derive(Debug, PartialEq)]
pub enum ListContactListsError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl ListContactListsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListContactListsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListContactListsError::BadRequest(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListContactListsError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListContactListsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListContactListsError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListContactListsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListContactListsError {}
/// Errors returned by ListContacts
#[derive(Debug, PartialEq)]
pub enum ListContactsError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>The resource you attempted to access doesn't exist.</p>
    NotFound(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl ListContactsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListContactsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListContactsError::BadRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListContactsError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListContactsError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListContactsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListContactsError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListContactsError::NotFound(ref cause) => write!(f, "{}", cause),
            ListContactsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListContactsError {}
/// Errors returned by ListCustomVerificationEmailTemplates
#[derive(Debug, PartialEq)]
pub enum ListCustomVerificationEmailTemplatesError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl ListCustomVerificationEmailTemplatesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListCustomVerificationEmailTemplatesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(
                        ListCustomVerificationEmailTemplatesError::BadRequest(err.msg),
                    )
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        ListCustomVerificationEmailTemplatesError::TooManyRequests(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListCustomVerificationEmailTemplatesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListCustomVerificationEmailTemplatesError::BadRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            ListCustomVerificationEmailTemplatesError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ListCustomVerificationEmailTemplatesError {}
/// Errors returned by ListDedicatedIpPools
#[derive(Debug, PartialEq)]
pub enum ListDedicatedIpPoolsError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl ListDedicatedIpPoolsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListDedicatedIpPoolsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListDedicatedIpPoolsError::BadRequest(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListDedicatedIpPoolsError::TooManyRequests(
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
impl fmt::Display for ListDedicatedIpPoolsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListDedicatedIpPoolsError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListDedicatedIpPoolsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListDedicatedIpPoolsError {}
/// Errors returned by ListDeliverabilityTestReports
#[derive(Debug, PartialEq)]
pub enum ListDeliverabilityTestReportsError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>The resource you attempted to access doesn't exist.</p>
    NotFound(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl ListDeliverabilityTestReportsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListDeliverabilityTestReportsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListDeliverabilityTestReportsError::BadRequest(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListDeliverabilityTestReportsError::NotFound(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        ListDeliverabilityTestReportsError::TooManyRequests(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListDeliverabilityTestReportsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListDeliverabilityTestReportsError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListDeliverabilityTestReportsError::NotFound(ref cause) => write!(f, "{}", cause),
            ListDeliverabilityTestReportsError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ListDeliverabilityTestReportsError {}
/// Errors returned by ListDomainDeliverabilityCampaigns
#[derive(Debug, PartialEq)]
pub enum ListDomainDeliverabilityCampaignsError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>The resource you attempted to access doesn't exist.</p>
    NotFound(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl ListDomainDeliverabilityCampaignsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListDomainDeliverabilityCampaignsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(
                        ListDomainDeliverabilityCampaignsError::BadRequest(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListDomainDeliverabilityCampaignsError::NotFound(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        ListDomainDeliverabilityCampaignsError::TooManyRequests(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListDomainDeliverabilityCampaignsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListDomainDeliverabilityCampaignsError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListDomainDeliverabilityCampaignsError::NotFound(ref cause) => write!(f, "{}", cause),
            ListDomainDeliverabilityCampaignsError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ListDomainDeliverabilityCampaignsError {}
/// Errors returned by ListEmailIdentities
#[derive(Debug, PartialEq)]
pub enum ListEmailIdentitiesError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl ListEmailIdentitiesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListEmailIdentitiesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListEmailIdentitiesError::BadRequest(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListEmailIdentitiesError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListEmailIdentitiesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListEmailIdentitiesError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListEmailIdentitiesError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListEmailIdentitiesError {}
/// Errors returned by ListEmailTemplates
#[derive(Debug, PartialEq)]
pub enum ListEmailTemplatesError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl ListEmailTemplatesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListEmailTemplatesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListEmailTemplatesError::BadRequest(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListEmailTemplatesError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListEmailTemplatesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListEmailTemplatesError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListEmailTemplatesError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListEmailTemplatesError {}
/// Errors returned by ListImportJobs
#[derive(Debug, PartialEq)]
pub enum ListImportJobsError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl ListImportJobsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListImportJobsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListImportJobsError::BadRequest(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListImportJobsError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListImportJobsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListImportJobsError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListImportJobsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListImportJobsError {}
/// Errors returned by ListSuppressedDestinations
#[derive(Debug, PartialEq)]
pub enum ListSuppressedDestinationsError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>The specified request includes an invalid or expired token.</p>
    InvalidNextToken(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl ListSuppressedDestinationsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListSuppressedDestinationsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListSuppressedDestinationsError::BadRequest(
                        err.msg,
                    ))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListSuppressedDestinationsError::InvalidNextToken(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListSuppressedDestinationsError::TooManyRequests(
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
impl fmt::Display for ListSuppressedDestinationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListSuppressedDestinationsError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListSuppressedDestinationsError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            ListSuppressedDestinationsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListSuppressedDestinationsError {}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>The resource you attempted to access doesn't exist.</p>
    NotFound(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListTagsForResourceError::BadRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListTagsForResourceError::NotFound(err.msg))
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
            ListTagsForResourceError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::NotFound(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTagsForResourceError {}
/// Errors returned by PutAccountDedicatedIpWarmupAttributes
#[derive(Debug, PartialEq)]
pub enum PutAccountDedicatedIpWarmupAttributesError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl PutAccountDedicatedIpWarmupAttributesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<PutAccountDedicatedIpWarmupAttributesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(
                        PutAccountDedicatedIpWarmupAttributesError::BadRequest(err.msg),
                    )
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        PutAccountDedicatedIpWarmupAttributesError::TooManyRequests(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutAccountDedicatedIpWarmupAttributesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutAccountDedicatedIpWarmupAttributesError::BadRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            PutAccountDedicatedIpWarmupAttributesError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for PutAccountDedicatedIpWarmupAttributesError {}
/// Errors returned by PutAccountDetails
#[derive(Debug, PartialEq)]
pub enum PutAccountDetailsError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>If there is already an ongoing account details update under review.</p>
    Conflict(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl PutAccountDetailsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutAccountDetailsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(PutAccountDetailsError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(PutAccountDetailsError::Conflict(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(PutAccountDetailsError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutAccountDetailsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutAccountDetailsError::BadRequest(ref cause) => write!(f, "{}", cause),
            PutAccountDetailsError::Conflict(ref cause) => write!(f, "{}", cause),
            PutAccountDetailsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutAccountDetailsError {}
/// Errors returned by PutAccountSendingAttributes
#[derive(Debug, PartialEq)]
pub enum PutAccountSendingAttributesError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl PutAccountSendingAttributesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<PutAccountSendingAttributesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(PutAccountSendingAttributesError::BadRequest(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(PutAccountSendingAttributesError::TooManyRequests(
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
impl fmt::Display for PutAccountSendingAttributesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutAccountSendingAttributesError::BadRequest(ref cause) => write!(f, "{}", cause),
            PutAccountSendingAttributesError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutAccountSendingAttributesError {}
/// Errors returned by PutAccountSuppressionAttributes
#[derive(Debug, PartialEq)]
pub enum PutAccountSuppressionAttributesError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl PutAccountSuppressionAttributesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<PutAccountSuppressionAttributesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(PutAccountSuppressionAttributesError::BadRequest(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        PutAccountSuppressionAttributesError::TooManyRequests(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutAccountSuppressionAttributesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutAccountSuppressionAttributesError::BadRequest(ref cause) => write!(f, "{}", cause),
            PutAccountSuppressionAttributesError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for PutAccountSuppressionAttributesError {}
/// Errors returned by PutConfigurationSetDeliveryOptions
#[derive(Debug, PartialEq)]
pub enum PutConfigurationSetDeliveryOptionsError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>The resource you attempted to access doesn't exist.</p>
    NotFound(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl PutConfigurationSetDeliveryOptionsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<PutConfigurationSetDeliveryOptionsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(
                        PutConfigurationSetDeliveryOptionsError::BadRequest(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(PutConfigurationSetDeliveryOptionsError::NotFound(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        PutConfigurationSetDeliveryOptionsError::TooManyRequests(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutConfigurationSetDeliveryOptionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutConfigurationSetDeliveryOptionsError::BadRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            PutConfigurationSetDeliveryOptionsError::NotFound(ref cause) => write!(f, "{}", cause),
            PutConfigurationSetDeliveryOptionsError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for PutConfigurationSetDeliveryOptionsError {}
/// Errors returned by PutConfigurationSetReputationOptions
#[derive(Debug, PartialEq)]
pub enum PutConfigurationSetReputationOptionsError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>The resource you attempted to access doesn't exist.</p>
    NotFound(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl PutConfigurationSetReputationOptionsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<PutConfigurationSetReputationOptionsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(
                        PutConfigurationSetReputationOptionsError::BadRequest(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(
                        PutConfigurationSetReputationOptionsError::NotFound(err.msg),
                    )
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        PutConfigurationSetReputationOptionsError::TooManyRequests(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutConfigurationSetReputationOptionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutConfigurationSetReputationOptionsError::BadRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            PutConfigurationSetReputationOptionsError::NotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            PutConfigurationSetReputationOptionsError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for PutConfigurationSetReputationOptionsError {}
/// Errors returned by PutConfigurationSetSendingOptions
#[derive(Debug, PartialEq)]
pub enum PutConfigurationSetSendingOptionsError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>The resource you attempted to access doesn't exist.</p>
    NotFound(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl PutConfigurationSetSendingOptionsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<PutConfigurationSetSendingOptionsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(
                        PutConfigurationSetSendingOptionsError::BadRequest(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(PutConfigurationSetSendingOptionsError::NotFound(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        PutConfigurationSetSendingOptionsError::TooManyRequests(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutConfigurationSetSendingOptionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutConfigurationSetSendingOptionsError::BadRequest(ref cause) => write!(f, "{}", cause),
            PutConfigurationSetSendingOptionsError::NotFound(ref cause) => write!(f, "{}", cause),
            PutConfigurationSetSendingOptionsError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for PutConfigurationSetSendingOptionsError {}
/// Errors returned by PutConfigurationSetSuppressionOptions
#[derive(Debug, PartialEq)]
pub enum PutConfigurationSetSuppressionOptionsError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>The resource you attempted to access doesn't exist.</p>
    NotFound(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl PutConfigurationSetSuppressionOptionsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<PutConfigurationSetSuppressionOptionsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(
                        PutConfigurationSetSuppressionOptionsError::BadRequest(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(
                        PutConfigurationSetSuppressionOptionsError::NotFound(err.msg),
                    )
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        PutConfigurationSetSuppressionOptionsError::TooManyRequests(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutConfigurationSetSuppressionOptionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutConfigurationSetSuppressionOptionsError::BadRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            PutConfigurationSetSuppressionOptionsError::NotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            PutConfigurationSetSuppressionOptionsError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for PutConfigurationSetSuppressionOptionsError {}
/// Errors returned by PutConfigurationSetTrackingOptions
#[derive(Debug, PartialEq)]
pub enum PutConfigurationSetTrackingOptionsError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>The resource you attempted to access doesn't exist.</p>
    NotFound(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl PutConfigurationSetTrackingOptionsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<PutConfigurationSetTrackingOptionsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(
                        PutConfigurationSetTrackingOptionsError::BadRequest(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(PutConfigurationSetTrackingOptionsError::NotFound(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        PutConfigurationSetTrackingOptionsError::TooManyRequests(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutConfigurationSetTrackingOptionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutConfigurationSetTrackingOptionsError::BadRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            PutConfigurationSetTrackingOptionsError::NotFound(ref cause) => write!(f, "{}", cause),
            PutConfigurationSetTrackingOptionsError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for PutConfigurationSetTrackingOptionsError {}
/// Errors returned by PutDedicatedIpInPool
#[derive(Debug, PartialEq)]
pub enum PutDedicatedIpInPoolError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>The resource you attempted to access doesn't exist.</p>
    NotFound(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl PutDedicatedIpInPoolError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutDedicatedIpInPoolError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(PutDedicatedIpInPoolError::BadRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(PutDedicatedIpInPoolError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(PutDedicatedIpInPoolError::TooManyRequests(
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
impl fmt::Display for PutDedicatedIpInPoolError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutDedicatedIpInPoolError::BadRequest(ref cause) => write!(f, "{}", cause),
            PutDedicatedIpInPoolError::NotFound(ref cause) => write!(f, "{}", cause),
            PutDedicatedIpInPoolError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutDedicatedIpInPoolError {}
/// Errors returned by PutDedicatedIpWarmupAttributes
#[derive(Debug, PartialEq)]
pub enum PutDedicatedIpWarmupAttributesError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>The resource you attempted to access doesn't exist.</p>
    NotFound(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl PutDedicatedIpWarmupAttributesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<PutDedicatedIpWarmupAttributesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(PutDedicatedIpWarmupAttributesError::BadRequest(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(PutDedicatedIpWarmupAttributesError::NotFound(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        PutDedicatedIpWarmupAttributesError::TooManyRequests(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutDedicatedIpWarmupAttributesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutDedicatedIpWarmupAttributesError::BadRequest(ref cause) => write!(f, "{}", cause),
            PutDedicatedIpWarmupAttributesError::NotFound(ref cause) => write!(f, "{}", cause),
            PutDedicatedIpWarmupAttributesError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for PutDedicatedIpWarmupAttributesError {}
/// Errors returned by PutDeliverabilityDashboardOption
#[derive(Debug, PartialEq)]
pub enum PutDeliverabilityDashboardOptionError {
    /// <p>The resource specified in your request already exists.</p>
    AlreadyExists(String),
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>There are too many instances of the specified resource type.</p>
    LimitExceeded(String),
    /// <p>The resource you attempted to access doesn't exist.</p>
    NotFound(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl PutDeliverabilityDashboardOptionError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<PutDeliverabilityDashboardOptionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AlreadyExistsException" => {
                    return RusotoError::Service(
                        PutDeliverabilityDashboardOptionError::AlreadyExists(err.msg),
                    )
                }
                "BadRequestException" => {
                    return RusotoError::Service(PutDeliverabilityDashboardOptionError::BadRequest(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(
                        PutDeliverabilityDashboardOptionError::LimitExceeded(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(PutDeliverabilityDashboardOptionError::NotFound(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        PutDeliverabilityDashboardOptionError::TooManyRequests(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutDeliverabilityDashboardOptionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutDeliverabilityDashboardOptionError::AlreadyExists(ref cause) => {
                write!(f, "{}", cause)
            }
            PutDeliverabilityDashboardOptionError::BadRequest(ref cause) => write!(f, "{}", cause),
            PutDeliverabilityDashboardOptionError::LimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            PutDeliverabilityDashboardOptionError::NotFound(ref cause) => write!(f, "{}", cause),
            PutDeliverabilityDashboardOptionError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for PutDeliverabilityDashboardOptionError {}
/// Errors returned by PutEmailIdentityDkimAttributes
#[derive(Debug, PartialEq)]
pub enum PutEmailIdentityDkimAttributesError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>The resource you attempted to access doesn't exist.</p>
    NotFound(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl PutEmailIdentityDkimAttributesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<PutEmailIdentityDkimAttributesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(PutEmailIdentityDkimAttributesError::BadRequest(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(PutEmailIdentityDkimAttributesError::NotFound(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        PutEmailIdentityDkimAttributesError::TooManyRequests(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutEmailIdentityDkimAttributesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutEmailIdentityDkimAttributesError::BadRequest(ref cause) => write!(f, "{}", cause),
            PutEmailIdentityDkimAttributesError::NotFound(ref cause) => write!(f, "{}", cause),
            PutEmailIdentityDkimAttributesError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for PutEmailIdentityDkimAttributesError {}
/// Errors returned by PutEmailIdentityDkimSigningAttributes
#[derive(Debug, PartialEq)]
pub enum PutEmailIdentityDkimSigningAttributesError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>The resource you attempted to access doesn't exist.</p>
    NotFound(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl PutEmailIdentityDkimSigningAttributesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<PutEmailIdentityDkimSigningAttributesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(
                        PutEmailIdentityDkimSigningAttributesError::BadRequest(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(
                        PutEmailIdentityDkimSigningAttributesError::NotFound(err.msg),
                    )
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        PutEmailIdentityDkimSigningAttributesError::TooManyRequests(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutEmailIdentityDkimSigningAttributesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutEmailIdentityDkimSigningAttributesError::BadRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            PutEmailIdentityDkimSigningAttributesError::NotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            PutEmailIdentityDkimSigningAttributesError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for PutEmailIdentityDkimSigningAttributesError {}
/// Errors returned by PutEmailIdentityFeedbackAttributes
#[derive(Debug, PartialEq)]
pub enum PutEmailIdentityFeedbackAttributesError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>The resource you attempted to access doesn't exist.</p>
    NotFound(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl PutEmailIdentityFeedbackAttributesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<PutEmailIdentityFeedbackAttributesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(
                        PutEmailIdentityFeedbackAttributesError::BadRequest(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(PutEmailIdentityFeedbackAttributesError::NotFound(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        PutEmailIdentityFeedbackAttributesError::TooManyRequests(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutEmailIdentityFeedbackAttributesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutEmailIdentityFeedbackAttributesError::BadRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            PutEmailIdentityFeedbackAttributesError::NotFound(ref cause) => write!(f, "{}", cause),
            PutEmailIdentityFeedbackAttributesError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for PutEmailIdentityFeedbackAttributesError {}
/// Errors returned by PutEmailIdentityMailFromAttributes
#[derive(Debug, PartialEq)]
pub enum PutEmailIdentityMailFromAttributesError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>The resource you attempted to access doesn't exist.</p>
    NotFound(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl PutEmailIdentityMailFromAttributesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<PutEmailIdentityMailFromAttributesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(
                        PutEmailIdentityMailFromAttributesError::BadRequest(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(PutEmailIdentityMailFromAttributesError::NotFound(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        PutEmailIdentityMailFromAttributesError::TooManyRequests(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutEmailIdentityMailFromAttributesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutEmailIdentityMailFromAttributesError::BadRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            PutEmailIdentityMailFromAttributesError::NotFound(ref cause) => write!(f, "{}", cause),
            PutEmailIdentityMailFromAttributesError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for PutEmailIdentityMailFromAttributesError {}
/// Errors returned by PutSuppressedDestination
#[derive(Debug, PartialEq)]
pub enum PutSuppressedDestinationError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl PutSuppressedDestinationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutSuppressedDestinationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(PutSuppressedDestinationError::BadRequest(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(PutSuppressedDestinationError::TooManyRequests(
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
impl fmt::Display for PutSuppressedDestinationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutSuppressedDestinationError::BadRequest(ref cause) => write!(f, "{}", cause),
            PutSuppressedDestinationError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutSuppressedDestinationError {}
/// Errors returned by SendBulkEmail
#[derive(Debug, PartialEq)]
pub enum SendBulkEmailError {
    /// <p>The message can't be sent because the account's ability to send email has been permanently restricted.</p>
    AccountSuspended(String),
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>There are too many instances of the specified resource type.</p>
    LimitExceeded(String),
    /// <p>The message can't be sent because the sending domain isn't verified.</p>
    MailFromDomainNotVerified(String),
    /// <p>The message can't be sent because it contains invalid content.</p>
    MessageRejected(String),
    /// <p>The resource you attempted to access doesn't exist.</p>
    NotFound(String),
    /// <p>The message can't be sent because the account's ability to send email is currently paused.</p>
    SendingPaused(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl SendBulkEmailError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SendBulkEmailError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccountSuspendedException" => {
                    return RusotoError::Service(SendBulkEmailError::AccountSuspended(err.msg))
                }
                "BadRequestException" => {
                    return RusotoError::Service(SendBulkEmailError::BadRequest(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(SendBulkEmailError::LimitExceeded(err.msg))
                }
                "MailFromDomainNotVerifiedException" => {
                    return RusotoError::Service(SendBulkEmailError::MailFromDomainNotVerified(
                        err.msg,
                    ))
                }
                "MessageRejected" => {
                    return RusotoError::Service(SendBulkEmailError::MessageRejected(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(SendBulkEmailError::NotFound(err.msg))
                }
                "SendingPausedException" => {
                    return RusotoError::Service(SendBulkEmailError::SendingPaused(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(SendBulkEmailError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for SendBulkEmailError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SendBulkEmailError::AccountSuspended(ref cause) => write!(f, "{}", cause),
            SendBulkEmailError::BadRequest(ref cause) => write!(f, "{}", cause),
            SendBulkEmailError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            SendBulkEmailError::MailFromDomainNotVerified(ref cause) => write!(f, "{}", cause),
            SendBulkEmailError::MessageRejected(ref cause) => write!(f, "{}", cause),
            SendBulkEmailError::NotFound(ref cause) => write!(f, "{}", cause),
            SendBulkEmailError::SendingPaused(ref cause) => write!(f, "{}", cause),
            SendBulkEmailError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for SendBulkEmailError {}
/// Errors returned by SendCustomVerificationEmail
#[derive(Debug, PartialEq)]
pub enum SendCustomVerificationEmailError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>There are too many instances of the specified resource type.</p>
    LimitExceeded(String),
    /// <p>The message can't be sent because the sending domain isn't verified.</p>
    MailFromDomainNotVerified(String),
    /// <p>The message can't be sent because it contains invalid content.</p>
    MessageRejected(String),
    /// <p>The resource you attempted to access doesn't exist.</p>
    NotFound(String),
    /// <p>The message can't be sent because the account's ability to send email is currently paused.</p>
    SendingPaused(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl SendCustomVerificationEmailError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<SendCustomVerificationEmailError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(SendCustomVerificationEmailError::BadRequest(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(SendCustomVerificationEmailError::LimitExceeded(
                        err.msg,
                    ))
                }
                "MailFromDomainNotVerifiedException" => {
                    return RusotoError::Service(
                        SendCustomVerificationEmailError::MailFromDomainNotVerified(err.msg),
                    )
                }
                "MessageRejected" => {
                    return RusotoError::Service(SendCustomVerificationEmailError::MessageRejected(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(SendCustomVerificationEmailError::NotFound(
                        err.msg,
                    ))
                }
                "SendingPausedException" => {
                    return RusotoError::Service(SendCustomVerificationEmailError::SendingPaused(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(SendCustomVerificationEmailError::TooManyRequests(
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
impl fmt::Display for SendCustomVerificationEmailError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SendCustomVerificationEmailError::BadRequest(ref cause) => write!(f, "{}", cause),
            SendCustomVerificationEmailError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            SendCustomVerificationEmailError::MailFromDomainNotVerified(ref cause) => {
                write!(f, "{}", cause)
            }
            SendCustomVerificationEmailError::MessageRejected(ref cause) => write!(f, "{}", cause),
            SendCustomVerificationEmailError::NotFound(ref cause) => write!(f, "{}", cause),
            SendCustomVerificationEmailError::SendingPaused(ref cause) => write!(f, "{}", cause),
            SendCustomVerificationEmailError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for SendCustomVerificationEmailError {}
/// Errors returned by SendEmail
#[derive(Debug, PartialEq)]
pub enum SendEmailError {
    /// <p>The message can't be sent because the account's ability to send email has been permanently restricted.</p>
    AccountSuspended(String),
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>There are too many instances of the specified resource type.</p>
    LimitExceeded(String),
    /// <p>The message can't be sent because the sending domain isn't verified.</p>
    MailFromDomainNotVerified(String),
    /// <p>The message can't be sent because it contains invalid content.</p>
    MessageRejected(String),
    /// <p>The resource you attempted to access doesn't exist.</p>
    NotFound(String),
    /// <p>The message can't be sent because the account's ability to send email is currently paused.</p>
    SendingPaused(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl SendEmailError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SendEmailError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccountSuspendedException" => {
                    return RusotoError::Service(SendEmailError::AccountSuspended(err.msg))
                }
                "BadRequestException" => {
                    return RusotoError::Service(SendEmailError::BadRequest(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(SendEmailError::LimitExceeded(err.msg))
                }
                "MailFromDomainNotVerifiedException" => {
                    return RusotoError::Service(SendEmailError::MailFromDomainNotVerified(err.msg))
                }
                "MessageRejected" => {
                    return RusotoError::Service(SendEmailError::MessageRejected(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(SendEmailError::NotFound(err.msg))
                }
                "SendingPausedException" => {
                    return RusotoError::Service(SendEmailError::SendingPaused(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(SendEmailError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for SendEmailError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SendEmailError::AccountSuspended(ref cause) => write!(f, "{}", cause),
            SendEmailError::BadRequest(ref cause) => write!(f, "{}", cause),
            SendEmailError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            SendEmailError::MailFromDomainNotVerified(ref cause) => write!(f, "{}", cause),
            SendEmailError::MessageRejected(ref cause) => write!(f, "{}", cause),
            SendEmailError::NotFound(ref cause) => write!(f, "{}", cause),
            SendEmailError::SendingPaused(ref cause) => write!(f, "{}", cause),
            SendEmailError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for SendEmailError {}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>The resource is being modified by another operation or thread.</p>
    ConcurrentModification(String),
    /// <p>The resource you attempted to access doesn't exist.</p>
    NotFound(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(TagResourceError::BadRequest(err.msg))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(TagResourceError::ConcurrentModification(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(TagResourceError::NotFound(err.msg))
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
            TagResourceError::BadRequest(ref cause) => write!(f, "{}", cause),
            TagResourceError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            TagResourceError::NotFound(ref cause) => write!(f, "{}", cause),
            TagResourceError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TagResourceError {}
/// Errors returned by TestRenderEmailTemplate
#[derive(Debug, PartialEq)]
pub enum TestRenderEmailTemplateError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>The resource you attempted to access doesn't exist.</p>
    NotFound(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl TestRenderEmailTemplateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TestRenderEmailTemplateError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(TestRenderEmailTemplateError::BadRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(TestRenderEmailTemplateError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(TestRenderEmailTemplateError::TooManyRequests(
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
impl fmt::Display for TestRenderEmailTemplateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TestRenderEmailTemplateError::BadRequest(ref cause) => write!(f, "{}", cause),
            TestRenderEmailTemplateError::NotFound(ref cause) => write!(f, "{}", cause),
            TestRenderEmailTemplateError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TestRenderEmailTemplateError {}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>The resource is being modified by another operation or thread.</p>
    ConcurrentModification(String),
    /// <p>The resource you attempted to access doesn't exist.</p>
    NotFound(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UntagResourceError::BadRequest(err.msg))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(UntagResourceError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UntagResourceError::NotFound(err.msg))
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
            UntagResourceError::BadRequest(ref cause) => write!(f, "{}", cause),
            UntagResourceError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            UntagResourceError::NotFound(ref cause) => write!(f, "{}", cause),
            UntagResourceError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UntagResourceError {}
/// Errors returned by UpdateConfigurationSetEventDestination
#[derive(Debug, PartialEq)]
pub enum UpdateConfigurationSetEventDestinationError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>The resource you attempted to access doesn't exist.</p>
    NotFound(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl UpdateConfigurationSetEventDestinationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateConfigurationSetEventDestinationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(
                        UpdateConfigurationSetEventDestinationError::BadRequest(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(
                        UpdateConfigurationSetEventDestinationError::NotFound(err.msg),
                    )
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        UpdateConfigurationSetEventDestinationError::TooManyRequests(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateConfigurationSetEventDestinationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateConfigurationSetEventDestinationError::BadRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateConfigurationSetEventDestinationError::NotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateConfigurationSetEventDestinationError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for UpdateConfigurationSetEventDestinationError {}
/// Errors returned by UpdateContact
#[derive(Debug, PartialEq)]
pub enum UpdateContactError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>The resource is being modified by another operation or thread.</p>
    ConcurrentModification(String),
    /// <p>The resource you attempted to access doesn't exist.</p>
    NotFound(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl UpdateContactError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateContactError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateContactError::BadRequest(err.msg))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(UpdateContactError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateContactError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateContactError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateContactError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateContactError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateContactError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            UpdateContactError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateContactError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateContactError {}
/// Errors returned by UpdateContactList
#[derive(Debug, PartialEq)]
pub enum UpdateContactListError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>The resource is being modified by another operation or thread.</p>
    ConcurrentModification(String),
    /// <p>The resource you attempted to access doesn't exist.</p>
    NotFound(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl UpdateContactListError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateContactListError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateContactListError::BadRequest(err.msg))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(UpdateContactListError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateContactListError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateContactListError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateContactListError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateContactListError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateContactListError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            UpdateContactListError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateContactListError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateContactListError {}
/// Errors returned by UpdateCustomVerificationEmailTemplate
#[derive(Debug, PartialEq)]
pub enum UpdateCustomVerificationEmailTemplateError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>The resource you attempted to access doesn't exist.</p>
    NotFound(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl UpdateCustomVerificationEmailTemplateError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateCustomVerificationEmailTemplateError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(
                        UpdateCustomVerificationEmailTemplateError::BadRequest(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(
                        UpdateCustomVerificationEmailTemplateError::NotFound(err.msg),
                    )
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        UpdateCustomVerificationEmailTemplateError::TooManyRequests(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateCustomVerificationEmailTemplateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateCustomVerificationEmailTemplateError::BadRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateCustomVerificationEmailTemplateError::NotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateCustomVerificationEmailTemplateError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for UpdateCustomVerificationEmailTemplateError {}
/// Errors returned by UpdateEmailIdentityPolicy
#[derive(Debug, PartialEq)]
pub enum UpdateEmailIdentityPolicyError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>The resource you attempted to access doesn't exist.</p>
    NotFound(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl UpdateEmailIdentityPolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateEmailIdentityPolicyError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateEmailIdentityPolicyError::BadRequest(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateEmailIdentityPolicyError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateEmailIdentityPolicyError::TooManyRequests(
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
impl fmt::Display for UpdateEmailIdentityPolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateEmailIdentityPolicyError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateEmailIdentityPolicyError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateEmailIdentityPolicyError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateEmailIdentityPolicyError {}
/// Errors returned by UpdateEmailTemplate
#[derive(Debug, PartialEq)]
pub enum UpdateEmailTemplateError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>The resource you attempted to access doesn't exist.</p>
    NotFound(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl UpdateEmailTemplateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateEmailTemplateError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateEmailTemplateError::BadRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateEmailTemplateError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateEmailTemplateError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateEmailTemplateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateEmailTemplateError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateEmailTemplateError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateEmailTemplateError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateEmailTemplateError {}
/// Trait representing the capabilities of the Amazon SES V2 API. Amazon SES V2 clients implement this trait.
#[async_trait]
pub trait SesV2: Clone + Sync + Send + 'static {
    /// <p>Create a configuration set. <i>Configuration sets</i> are groups of rules that you can apply to the emails that you send. You apply a configuration set to an email by specifying the name of the configuration set when you call the Amazon SES API v2. When you apply a configuration set to an email, all of the rules in that configuration set are applied to the email. </p>
    async fn create_configuration_set(
        &self,
        input: CreateConfigurationSetRequest,
    ) -> Result<CreateConfigurationSetResponse, RusotoError<CreateConfigurationSetError>>;

    /// <p>Create an event destination. <i>Events</i> include message sends, deliveries, opens, clicks, bounces, and complaints. <i>Event destinations</i> are places that you can send information about these events to. For example, you can send event data to Amazon SNS to receive notifications when you receive bounces or complaints, or you can use Amazon Kinesis Data Firehose to stream data to Amazon S3 for long-term storage.</p> <p>A single configuration set can include more than one event destination.</p>
    async fn create_configuration_set_event_destination(
        &self,
        input: CreateConfigurationSetEventDestinationRequest,
    ) -> Result<
        CreateConfigurationSetEventDestinationResponse,
        RusotoError<CreateConfigurationSetEventDestinationError>,
    >;

    /// <p>Creates a contact, which is an end-user who is receiving the email, and adds them to a contact list.</p>
    async fn create_contact(
        &self,
        input: CreateContactRequest,
    ) -> Result<CreateContactResponse, RusotoError<CreateContactError>>;

    /// <p>Creates a contact list.</p>
    async fn create_contact_list(
        &self,
        input: CreateContactListRequest,
    ) -> Result<CreateContactListResponse, RusotoError<CreateContactListError>>;

    /// <p>Creates a new custom verification email template.</p> <p>For more information about custom verification email templates, see <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/send-email-verify-address-custom.html">Using Custom Verification Email Templates</a> in the <i>Amazon SES Developer Guide</i>.</p> <p>You can execute this operation no more than once per second.</p>
    async fn create_custom_verification_email_template(
        &self,
        input: CreateCustomVerificationEmailTemplateRequest,
    ) -> Result<
        CreateCustomVerificationEmailTemplateResponse,
        RusotoError<CreateCustomVerificationEmailTemplateError>,
    >;

    /// <p>Create a new pool of dedicated IP addresses. A pool can include one or more dedicated IP addresses that are associated with your AWS account. You can associate a pool with a configuration set. When you send an email that uses that configuration set, the message is sent from one of the addresses in the associated pool.</p>
    async fn create_dedicated_ip_pool(
        &self,
        input: CreateDedicatedIpPoolRequest,
    ) -> Result<CreateDedicatedIpPoolResponse, RusotoError<CreateDedicatedIpPoolError>>;

    /// <p>Create a new predictive inbox placement test. Predictive inbox placement tests can help you predict how your messages will be handled by various email providers around the world. When you perform a predictive inbox placement test, you provide a sample message that contains the content that you plan to send to your customers. Amazon SES then sends that message to special email addresses spread across several major email providers. After about 24 hours, the test is complete, and you can use the <code>GetDeliverabilityTestReport</code> operation to view the results of the test.</p>
    async fn create_deliverability_test_report(
        &self,
        input: CreateDeliverabilityTestReportRequest,
    ) -> Result<
        CreateDeliverabilityTestReportResponse,
        RusotoError<CreateDeliverabilityTestReportError>,
    >;

    /// <p>Starts the process of verifying an email identity. An <i>identity</i> is an email address or domain that you use when you send email. Before you can use an identity to send email, you first have to verify it. By verifying an identity, you demonstrate that you're the owner of the identity, and that you've given Amazon SES API v2 permission to send email from the identity.</p> <p>When you verify an email address, Amazon SES sends an email to the address. Your email address is verified as soon as you follow the link in the verification email. </p> <p>When you verify a domain without specifying the <code>DkimSigningAttributes</code> object, this operation provides a set of DKIM tokens. You can convert these tokens into CNAME records, which you then add to the DNS configuration for your domain. Your domain is verified when Amazon SES detects these records in the DNS configuration for your domain. This verification method is known as <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/easy-dkim.html">Easy DKIM</a>.</p> <p>Alternatively, you can perform the verification process by providing your own public-private key pair. This verification method is known as Bring Your Own DKIM (BYODKIM). To use BYODKIM, your call to the <code>CreateEmailIdentity</code> operation has to include the <code>DkimSigningAttributes</code> object. When you specify this object, you provide a selector (a component of the DNS record name that identifies the public key that you want to use for DKIM authentication) and a private key.</p>
    async fn create_email_identity(
        &self,
        input: CreateEmailIdentityRequest,
    ) -> Result<CreateEmailIdentityResponse, RusotoError<CreateEmailIdentityError>>;

    /// <p>Creates the specified sending authorization policy for the given identity (an email address or a domain).</p> <note> <p>This API is for the identity owner only. If you have not verified the identity, this API will return an error.</p> </note> <p>Sending authorization is a feature that enables an identity owner to authorize other senders to use its identities. For information about using sending authorization, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/sending-authorization.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    async fn create_email_identity_policy(
        &self,
        input: CreateEmailIdentityPolicyRequest,
    ) -> Result<CreateEmailIdentityPolicyResponse, RusotoError<CreateEmailIdentityPolicyError>>;

    /// <p>Creates an email template. Email templates enable you to send personalized email to one or more destinations in a single API operation. For more information, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/send-personalized-email-api.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    async fn create_email_template(
        &self,
        input: CreateEmailTemplateRequest,
    ) -> Result<CreateEmailTemplateResponse, RusotoError<CreateEmailTemplateError>>;

    /// <p>Creates an import job for a data destination.</p>
    async fn create_import_job(
        &self,
        input: CreateImportJobRequest,
    ) -> Result<CreateImportJobResponse, RusotoError<CreateImportJobError>>;

    /// <p>Delete an existing configuration set.</p> <p> <i>Configuration sets</i> are groups of rules that you can apply to the emails you send. You apply a configuration set to an email by including a reference to the configuration set in the headers of the email. When you apply a configuration set to an email, all of the rules in that configuration set are applied to the email.</p>
    async fn delete_configuration_set(
        &self,
        input: DeleteConfigurationSetRequest,
    ) -> Result<DeleteConfigurationSetResponse, RusotoError<DeleteConfigurationSetError>>;

    /// <p>Delete an event destination.</p> <p> <i>Events</i> include message sends, deliveries, opens, clicks, bounces, and complaints. <i>Event destinations</i> are places that you can send information about these events to. For example, you can send event data to Amazon SNS to receive notifications when you receive bounces or complaints, or you can use Amazon Kinesis Data Firehose to stream data to Amazon S3 for long-term storage.</p>
    async fn delete_configuration_set_event_destination(
        &self,
        input: DeleteConfigurationSetEventDestinationRequest,
    ) -> Result<
        DeleteConfigurationSetEventDestinationResponse,
        RusotoError<DeleteConfigurationSetEventDestinationError>,
    >;

    /// <p>Removes a contact from a contact list.</p>
    async fn delete_contact(
        &self,
        input: DeleteContactRequest,
    ) -> Result<DeleteContactResponse, RusotoError<DeleteContactError>>;

    /// <p>Deletes a contact list and all of the contacts on that list.</p>
    async fn delete_contact_list(
        &self,
        input: DeleteContactListRequest,
    ) -> Result<DeleteContactListResponse, RusotoError<DeleteContactListError>>;

    /// <p>Deletes an existing custom verification email template.</p> <p>For more information about custom verification email templates, see <a href="https://docs.aws.amazon.com/es/latest/DeveloperGuide/send-email-verify-address-custom.html">Using Custom Verification Email Templates</a> in the <i>Amazon SES Developer Guide</i>.</p> <p>You can execute this operation no more than once per second.</p>
    async fn delete_custom_verification_email_template(
        &self,
        input: DeleteCustomVerificationEmailTemplateRequest,
    ) -> Result<
        DeleteCustomVerificationEmailTemplateResponse,
        RusotoError<DeleteCustomVerificationEmailTemplateError>,
    >;

    /// <p>Delete a dedicated IP pool.</p>
    async fn delete_dedicated_ip_pool(
        &self,
        input: DeleteDedicatedIpPoolRequest,
    ) -> Result<DeleteDedicatedIpPoolResponse, RusotoError<DeleteDedicatedIpPoolError>>;

    /// <p>Deletes an email identity. An identity can be either an email address or a domain name.</p>
    async fn delete_email_identity(
        &self,
        input: DeleteEmailIdentityRequest,
    ) -> Result<DeleteEmailIdentityResponse, RusotoError<DeleteEmailIdentityError>>;

    /// <p>Deletes the specified sending authorization policy for the given identity (an email address or a domain). This API returns successfully even if a policy with the specified name does not exist.</p> <note> <p>This API is for the identity owner only. If you have not verified the identity, this API will return an error.</p> </note> <p>Sending authorization is a feature that enables an identity owner to authorize other senders to use its identities. For information about using sending authorization, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/sending-authorization.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    async fn delete_email_identity_policy(
        &self,
        input: DeleteEmailIdentityPolicyRequest,
    ) -> Result<DeleteEmailIdentityPolicyResponse, RusotoError<DeleteEmailIdentityPolicyError>>;

    /// <p>Deletes an email template.</p> <p>You can execute this operation no more than once per second.</p>
    async fn delete_email_template(
        &self,
        input: DeleteEmailTemplateRequest,
    ) -> Result<DeleteEmailTemplateResponse, RusotoError<DeleteEmailTemplateError>>;

    /// <p>Removes an email address from the suppression list for your account.</p>
    async fn delete_suppressed_destination(
        &self,
        input: DeleteSuppressedDestinationRequest,
    ) -> Result<DeleteSuppressedDestinationResponse, RusotoError<DeleteSuppressedDestinationError>>;

    /// <p>Obtain information about the email-sending status and capabilities of your Amazon SES account in the current AWS Region.</p>
    async fn get_account(&self) -> Result<GetAccountResponse, RusotoError<GetAccountError>>;

    /// <p>Retrieve a list of the blacklists that your dedicated IP addresses appear on.</p>
    async fn get_blacklist_reports(
        &self,
        input: GetBlacklistReportsRequest,
    ) -> Result<GetBlacklistReportsResponse, RusotoError<GetBlacklistReportsError>>;

    /// <p>Get information about an existing configuration set, including the dedicated IP pool that it's associated with, whether or not it's enabled for sending email, and more.</p> <p> <i>Configuration sets</i> are groups of rules that you can apply to the emails you send. You apply a configuration set to an email by including a reference to the configuration set in the headers of the email. When you apply a configuration set to an email, all of the rules in that configuration set are applied to the email.</p>
    async fn get_configuration_set(
        &self,
        input: GetConfigurationSetRequest,
    ) -> Result<GetConfigurationSetResponse, RusotoError<GetConfigurationSetError>>;

    /// <p>Retrieve a list of event destinations that are associated with a configuration set.</p> <p> <i>Events</i> include message sends, deliveries, opens, clicks, bounces, and complaints. <i>Event destinations</i> are places that you can send information about these events to. For example, you can send event data to Amazon SNS to receive notifications when you receive bounces or complaints, or you can use Amazon Kinesis Data Firehose to stream data to Amazon S3 for long-term storage.</p>
    async fn get_configuration_set_event_destinations(
        &self,
        input: GetConfigurationSetEventDestinationsRequest,
    ) -> Result<
        GetConfigurationSetEventDestinationsResponse,
        RusotoError<GetConfigurationSetEventDestinationsError>,
    >;

    /// <p>Returns a contact from a contact list.</p>
    async fn get_contact(
        &self,
        input: GetContactRequest,
    ) -> Result<GetContactResponse, RusotoError<GetContactError>>;

    /// <p>Returns contact list metadata. It does not return any information about the contacts present in the list.</p>
    async fn get_contact_list(
        &self,
        input: GetContactListRequest,
    ) -> Result<GetContactListResponse, RusotoError<GetContactListError>>;

    /// <p>Returns the custom email verification template for the template name you specify.</p> <p>For more information about custom verification email templates, see <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/send-email-verify-address-custom.html">Using Custom Verification Email Templates</a> in the <i>Amazon SES Developer Guide</i>.</p> <p>You can execute this operation no more than once per second.</p>
    async fn get_custom_verification_email_template(
        &self,
        input: GetCustomVerificationEmailTemplateRequest,
    ) -> Result<
        GetCustomVerificationEmailTemplateResponse,
        RusotoError<GetCustomVerificationEmailTemplateError>,
    >;

    /// <p>Get information about a dedicated IP address, including the name of the dedicated IP pool that it's associated with, as well information about the automatic warm-up process for the address.</p>
    async fn get_dedicated_ip(
        &self,
        input: GetDedicatedIpRequest,
    ) -> Result<GetDedicatedIpResponse, RusotoError<GetDedicatedIpError>>;

    /// <p>List the dedicated IP addresses that are associated with your AWS account.</p>
    async fn get_dedicated_ips(
        &self,
        input: GetDedicatedIpsRequest,
    ) -> Result<GetDedicatedIpsResponse, RusotoError<GetDedicatedIpsError>>;

    /// <p>Retrieve information about the status of the Deliverability dashboard for your account. When the Deliverability dashboard is enabled, you gain access to reputation, deliverability, and other metrics for the domains that you use to send email. You also gain the ability to perform predictive inbox placement tests.</p> <p>When you use the Deliverability dashboard, you pay a monthly subscription charge, in addition to any other fees that you accrue by using Amazon SES and other AWS services. For more information about the features and cost of a Deliverability dashboard subscription, see <a href="http://aws.amazon.com/ses/pricing/">Amazon SES Pricing</a>.</p>
    async fn get_deliverability_dashboard_options(
        &self,
    ) -> Result<
        GetDeliverabilityDashboardOptionsResponse,
        RusotoError<GetDeliverabilityDashboardOptionsError>,
    >;

    /// <p>Retrieve the results of a predictive inbox placement test.</p>
    async fn get_deliverability_test_report(
        &self,
        input: GetDeliverabilityTestReportRequest,
    ) -> Result<GetDeliverabilityTestReportResponse, RusotoError<GetDeliverabilityTestReportError>>;

    /// <p>Retrieve all the deliverability data for a specific campaign. This data is available for a campaign only if the campaign sent email by using a domain that the Deliverability dashboard is enabled for.</p>
    async fn get_domain_deliverability_campaign(
        &self,
        input: GetDomainDeliverabilityCampaignRequest,
    ) -> Result<
        GetDomainDeliverabilityCampaignResponse,
        RusotoError<GetDomainDeliverabilityCampaignError>,
    >;

    /// <p>Retrieve inbox placement and engagement rates for the domains that you use to send email.</p>
    async fn get_domain_statistics_report(
        &self,
        input: GetDomainStatisticsReportRequest,
    ) -> Result<GetDomainStatisticsReportResponse, RusotoError<GetDomainStatisticsReportError>>;

    /// <p>Provides information about a specific identity, including the identity's verification status, sending authorization policies, its DKIM authentication status, and its custom Mail-From settings.</p>
    async fn get_email_identity(
        &self,
        input: GetEmailIdentityRequest,
    ) -> Result<GetEmailIdentityResponse, RusotoError<GetEmailIdentityError>>;

    /// <p>Returns the requested sending authorization policies for the given identity (an email address or a domain). The policies are returned as a map of policy names to policy contents. You can retrieve a maximum of 20 policies at a time.</p> <note> <p>This API is for the identity owner only. If you have not verified the identity, this API will return an error.</p> </note> <p>Sending authorization is a feature that enables an identity owner to authorize other senders to use its identities. For information about using sending authorization, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/sending-authorization.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    async fn get_email_identity_policies(
        &self,
        input: GetEmailIdentityPoliciesRequest,
    ) -> Result<GetEmailIdentityPoliciesResponse, RusotoError<GetEmailIdentityPoliciesError>>;

    /// <p>Displays the template object (which includes the subject line, HTML part and text part) for the template you specify.</p> <p>You can execute this operation no more than once per second.</p>
    async fn get_email_template(
        &self,
        input: GetEmailTemplateRequest,
    ) -> Result<GetEmailTemplateResponse, RusotoError<GetEmailTemplateError>>;

    /// <p>Provides information about an import job.</p>
    async fn get_import_job(
        &self,
        input: GetImportJobRequest,
    ) -> Result<GetImportJobResponse, RusotoError<GetImportJobError>>;

    /// <p>Retrieves information about a specific email address that's on the suppression list for your account.</p>
    async fn get_suppressed_destination(
        &self,
        input: GetSuppressedDestinationRequest,
    ) -> Result<GetSuppressedDestinationResponse, RusotoError<GetSuppressedDestinationError>>;

    /// <p>List all of the configuration sets associated with your account in the current region.</p> <p> <i>Configuration sets</i> are groups of rules that you can apply to the emails you send. You apply a configuration set to an email by including a reference to the configuration set in the headers of the email. When you apply a configuration set to an email, all of the rules in that configuration set are applied to the email.</p>
    async fn list_configuration_sets(
        &self,
        input: ListConfigurationSetsRequest,
    ) -> Result<ListConfigurationSetsResponse, RusotoError<ListConfigurationSetsError>>;

    /// <p>Lists all of the contact lists available.</p>
    async fn list_contact_lists(
        &self,
        input: ListContactListsRequest,
    ) -> Result<ListContactListsResponse, RusotoError<ListContactListsError>>;

    /// <p>Lists the contacts present in a specific contact list.</p>
    async fn list_contacts(
        &self,
        input: ListContactsRequest,
    ) -> Result<ListContactsResponse, RusotoError<ListContactsError>>;

    /// <p>Lists the existing custom verification email templates for your account in the current AWS Region.</p> <p>For more information about custom verification email templates, see <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/send-email-verify-address-custom.html">Using Custom Verification Email Templates</a> in the <i>Amazon SES Developer Guide</i>.</p> <p>You can execute this operation no more than once per second.</p>
    async fn list_custom_verification_email_templates(
        &self,
        input: ListCustomVerificationEmailTemplatesRequest,
    ) -> Result<
        ListCustomVerificationEmailTemplatesResponse,
        RusotoError<ListCustomVerificationEmailTemplatesError>,
    >;

    /// <p>List all of the dedicated IP pools that exist in your AWS account in the current Region.</p>
    async fn list_dedicated_ip_pools(
        &self,
        input: ListDedicatedIpPoolsRequest,
    ) -> Result<ListDedicatedIpPoolsResponse, RusotoError<ListDedicatedIpPoolsError>>;

    /// <p>Show a list of the predictive inbox placement tests that you've performed, regardless of their statuses. For predictive inbox placement tests that are complete, you can use the <code>GetDeliverabilityTestReport</code> operation to view the results.</p>
    async fn list_deliverability_test_reports(
        &self,
        input: ListDeliverabilityTestReportsRequest,
    ) -> Result<
        ListDeliverabilityTestReportsResponse,
        RusotoError<ListDeliverabilityTestReportsError>,
    >;

    /// <p>Retrieve deliverability data for all the campaigns that used a specific domain to send email during a specified time range. This data is available for a domain only if you enabled the Deliverability dashboard for the domain.</p>
    async fn list_domain_deliverability_campaigns(
        &self,
        input: ListDomainDeliverabilityCampaignsRequest,
    ) -> Result<
        ListDomainDeliverabilityCampaignsResponse,
        RusotoError<ListDomainDeliverabilityCampaignsError>,
    >;

    /// <p>Returns a list of all of the email identities that are associated with your AWS account. An identity can be either an email address or a domain. This operation returns identities that are verified as well as those that aren't. This operation returns identities that are associated with Amazon SES and Amazon Pinpoint.</p>
    async fn list_email_identities(
        &self,
        input: ListEmailIdentitiesRequest,
    ) -> Result<ListEmailIdentitiesResponse, RusotoError<ListEmailIdentitiesError>>;

    /// <p>Lists the email templates present in your Amazon SES account in the current AWS Region.</p> <p>You can execute this operation no more than once per second.</p>
    async fn list_email_templates(
        &self,
        input: ListEmailTemplatesRequest,
    ) -> Result<ListEmailTemplatesResponse, RusotoError<ListEmailTemplatesError>>;

    /// <p>Lists all of the import jobs.</p>
    async fn list_import_jobs(
        &self,
        input: ListImportJobsRequest,
    ) -> Result<ListImportJobsResponse, RusotoError<ListImportJobsError>>;

    /// <p>Retrieves a list of email addresses that are on the suppression list for your account.</p>
    async fn list_suppressed_destinations(
        &self,
        input: ListSuppressedDestinationsRequest,
    ) -> Result<ListSuppressedDestinationsResponse, RusotoError<ListSuppressedDestinationsError>>;

    /// <p>Retrieve a list of the tags (keys and values) that are associated with a specified resource. A <i>tag</i> is a label that you optionally define and associate with a resource. Each tag consists of a required <i>tag key</i> and an optional associated <i>tag value</i>. A tag key is a general label that acts as a category for more specific tag values. A tag value acts as a descriptor within a tag key.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>>;

    /// <p>Enable or disable the automatic warm-up feature for dedicated IP addresses.</p>
    async fn put_account_dedicated_ip_warmup_attributes(
        &self,
        input: PutAccountDedicatedIpWarmupAttributesRequest,
    ) -> Result<
        PutAccountDedicatedIpWarmupAttributesResponse,
        RusotoError<PutAccountDedicatedIpWarmupAttributesError>,
    >;

    /// <p>Update your Amazon SES account details.</p>
    async fn put_account_details(
        &self,
        input: PutAccountDetailsRequest,
    ) -> Result<PutAccountDetailsResponse, RusotoError<PutAccountDetailsError>>;

    /// <p>Enable or disable the ability of your account to send email.</p>
    async fn put_account_sending_attributes(
        &self,
        input: PutAccountSendingAttributesRequest,
    ) -> Result<PutAccountSendingAttributesResponse, RusotoError<PutAccountSendingAttributesError>>;

    /// <p>Change the settings for the account-level suppression list.</p>
    async fn put_account_suppression_attributes(
        &self,
        input: PutAccountSuppressionAttributesRequest,
    ) -> Result<
        PutAccountSuppressionAttributesResponse,
        RusotoError<PutAccountSuppressionAttributesError>,
    >;

    /// <p>Associate a configuration set with a dedicated IP pool. You can use dedicated IP pools to create groups of dedicated IP addresses for sending specific types of email.</p>
    async fn put_configuration_set_delivery_options(
        &self,
        input: PutConfigurationSetDeliveryOptionsRequest,
    ) -> Result<
        PutConfigurationSetDeliveryOptionsResponse,
        RusotoError<PutConfigurationSetDeliveryOptionsError>,
    >;

    /// <p>Enable or disable collection of reputation metrics for emails that you send using a particular configuration set in a specific AWS Region.</p>
    async fn put_configuration_set_reputation_options(
        &self,
        input: PutConfigurationSetReputationOptionsRequest,
    ) -> Result<
        PutConfigurationSetReputationOptionsResponse,
        RusotoError<PutConfigurationSetReputationOptionsError>,
    >;

    /// <p>Enable or disable email sending for messages that use a particular configuration set in a specific AWS Region.</p>
    async fn put_configuration_set_sending_options(
        &self,
        input: PutConfigurationSetSendingOptionsRequest,
    ) -> Result<
        PutConfigurationSetSendingOptionsResponse,
        RusotoError<PutConfigurationSetSendingOptionsError>,
    >;

    /// <p>Specify the account suppression list preferences for a configuration set.</p>
    async fn put_configuration_set_suppression_options(
        &self,
        input: PutConfigurationSetSuppressionOptionsRequest,
    ) -> Result<
        PutConfigurationSetSuppressionOptionsResponse,
        RusotoError<PutConfigurationSetSuppressionOptionsError>,
    >;

    /// <p>Specify a custom domain to use for open and click tracking elements in email that you send.</p>
    async fn put_configuration_set_tracking_options(
        &self,
        input: PutConfigurationSetTrackingOptionsRequest,
    ) -> Result<
        PutConfigurationSetTrackingOptionsResponse,
        RusotoError<PutConfigurationSetTrackingOptionsError>,
    >;

    /// <p><p>Move a dedicated IP address to an existing dedicated IP pool.</p> <note> <p>The dedicated IP address that you specify must already exist, and must be associated with your AWS account. </p> <p>The dedicated IP pool you specify must already exist. You can create a new pool by using the <code>CreateDedicatedIpPool</code> operation.</p> </note></p>
    async fn put_dedicated_ip_in_pool(
        &self,
        input: PutDedicatedIpInPoolRequest,
    ) -> Result<PutDedicatedIpInPoolResponse, RusotoError<PutDedicatedIpInPoolError>>;

    /// <p><p/></p>
    async fn put_dedicated_ip_warmup_attributes(
        &self,
        input: PutDedicatedIpWarmupAttributesRequest,
    ) -> Result<
        PutDedicatedIpWarmupAttributesResponse,
        RusotoError<PutDedicatedIpWarmupAttributesError>,
    >;

    /// <p>Enable or disable the Deliverability dashboard. When you enable the Deliverability dashboard, you gain access to reputation, deliverability, and other metrics for the domains that you use to send email. You also gain the ability to perform predictive inbox placement tests.</p> <p>When you use the Deliverability dashboard, you pay a monthly subscription charge, in addition to any other fees that you accrue by using Amazon SES and other AWS services. For more information about the features and cost of a Deliverability dashboard subscription, see <a href="http://aws.amazon.com/ses/pricing/">Amazon SES Pricing</a>.</p>
    async fn put_deliverability_dashboard_option(
        &self,
        input: PutDeliverabilityDashboardOptionRequest,
    ) -> Result<
        PutDeliverabilityDashboardOptionResponse,
        RusotoError<PutDeliverabilityDashboardOptionError>,
    >;

    /// <p>Used to enable or disable DKIM authentication for an email identity.</p>
    async fn put_email_identity_dkim_attributes(
        &self,
        input: PutEmailIdentityDkimAttributesRequest,
    ) -> Result<
        PutEmailIdentityDkimAttributesResponse,
        RusotoError<PutEmailIdentityDkimAttributesError>,
    >;

    /// <p><p>Used to configure or change the DKIM authentication settings for an email domain identity. You can use this operation to do any of the following:</p> <ul> <li> <p>Update the signing attributes for an identity that uses Bring Your Own DKIM (BYODKIM).</p> </li> <li> <p>Change from using no DKIM authentication to using Easy DKIM.</p> </li> <li> <p>Change from using no DKIM authentication to using BYODKIM.</p> </li> <li> <p>Change from using Easy DKIM to using BYODKIM.</p> </li> <li> <p>Change from using BYODKIM to using Easy DKIM.</p> </li> </ul></p>
    async fn put_email_identity_dkim_signing_attributes(
        &self,
        input: PutEmailIdentityDkimSigningAttributesRequest,
    ) -> Result<
        PutEmailIdentityDkimSigningAttributesResponse,
        RusotoError<PutEmailIdentityDkimSigningAttributesError>,
    >;

    /// <p>Used to enable or disable feedback forwarding for an identity. This setting determines what happens when an identity is used to send an email that results in a bounce or complaint event.</p> <p>If the value is <code>true</code>, you receive email notifications when bounce or complaint events occur. These notifications are sent to the address that you specified in the <code>Return-Path</code> header of the original email.</p> <p>You're required to have a method of tracking bounces and complaints. If you haven't set up another mechanism for receiving bounce or complaint notifications (for example, by setting up an event destination), you receive an email notification when these events occur (even if this setting is disabled).</p>
    async fn put_email_identity_feedback_attributes(
        &self,
        input: PutEmailIdentityFeedbackAttributesRequest,
    ) -> Result<
        PutEmailIdentityFeedbackAttributesResponse,
        RusotoError<PutEmailIdentityFeedbackAttributesError>,
    >;

    /// <p>Used to enable or disable the custom Mail-From domain configuration for an email identity.</p>
    async fn put_email_identity_mail_from_attributes(
        &self,
        input: PutEmailIdentityMailFromAttributesRequest,
    ) -> Result<
        PutEmailIdentityMailFromAttributesResponse,
        RusotoError<PutEmailIdentityMailFromAttributesError>,
    >;

    /// <p>Adds an email address to the suppression list for your account.</p>
    async fn put_suppressed_destination(
        &self,
        input: PutSuppressedDestinationRequest,
    ) -> Result<PutSuppressedDestinationResponse, RusotoError<PutSuppressedDestinationError>>;

    /// <p>Composes an email message to multiple destinations.</p>
    async fn send_bulk_email(
        &self,
        input: SendBulkEmailRequest,
    ) -> Result<SendBulkEmailResponse, RusotoError<SendBulkEmailError>>;

    /// <p>Adds an email address to the list of identities for your Amazon SES account in the current AWS Region and attempts to verify it. As a result of executing this operation, a customized verification email is sent to the specified address.</p> <p>To use this operation, you must first create a custom verification email template. For more information about creating and using custom verification email templates, see <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/send-email-verify-address-custom.html">Using Custom Verification Email Templates</a> in the <i>Amazon SES Developer Guide</i>.</p> <p>You can execute this operation no more than once per second.</p>
    async fn send_custom_verification_email(
        &self,
        input: SendCustomVerificationEmailRequest,
    ) -> Result<SendCustomVerificationEmailResponse, RusotoError<SendCustomVerificationEmailError>>;

    /// <p><p>Sends an email message. You can use the Amazon SES API v2 to send two types of messages:</p> <ul> <li> <p> <b>Simple</b> – A standard email message. When you create this type of message, you specify the sender, the recipient, and the message body, and Amazon SES assembles the message for you.</p> </li> <li> <p> <b>Raw</b> – A raw, MIME-formatted email message. When you send this type of email, you have to specify all of the message headers, as well as the message body. You can use this message type to send messages that contain attachments. The message that you specify has to be a valid MIME message.</p> </li> <li> <p> <b>Templated</b> – A message that contains personalization tags. When you send this type of email, Amazon SES API v2 automatically replaces the tags with values that you specify.</p> </li> </ul></p>
    async fn send_email(
        &self,
        input: SendEmailRequest,
    ) -> Result<SendEmailResponse, RusotoError<SendEmailError>>;

    /// <p>Add one or more tags (keys and values) to a specified resource. A <i>tag</i> is a label that you optionally define and associate with a resource. Tags can help you categorize and manage resources in different ways, such as by purpose, owner, environment, or other criteria. A resource can have as many as 50 tags.</p> <p>Each tag consists of a required <i>tag key</i> and an associated <i>tag value</i>, both of which you define. A tag key is a general label that acts as a category for more specific tag values. A tag value acts as a descriptor within a tag key.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>>;

    /// <p>Creates a preview of the MIME content of an email when provided with a template and a set of replacement data.</p> <p>You can execute this operation no more than once per second.</p>
    async fn test_render_email_template(
        &self,
        input: TestRenderEmailTemplateRequest,
    ) -> Result<TestRenderEmailTemplateResponse, RusotoError<TestRenderEmailTemplateError>>;

    /// <p>Remove one or more tags (keys and values) from a specified resource.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>>;

    /// <p>Update the configuration of an event destination for a configuration set.</p> <p> <i>Events</i> include message sends, deliveries, opens, clicks, bounces, and complaints. <i>Event destinations</i> are places that you can send information about these events to. For example, you can send event data to Amazon SNS to receive notifications when you receive bounces or complaints, or you can use Amazon Kinesis Data Firehose to stream data to Amazon S3 for long-term storage.</p>
    async fn update_configuration_set_event_destination(
        &self,
        input: UpdateConfigurationSetEventDestinationRequest,
    ) -> Result<
        UpdateConfigurationSetEventDestinationResponse,
        RusotoError<UpdateConfigurationSetEventDestinationError>,
    >;

    /// <p>Updates a contact's preferences for a list. It is not necessary to specify all existing topic preferences in the TopicPreferences object, just the ones that need updating.</p>
    async fn update_contact(
        &self,
        input: UpdateContactRequest,
    ) -> Result<UpdateContactResponse, RusotoError<UpdateContactError>>;

    /// <p>Updates contact list metadata. This operation does a complete replacement.</p>
    async fn update_contact_list(
        &self,
        input: UpdateContactListRequest,
    ) -> Result<UpdateContactListResponse, RusotoError<UpdateContactListError>>;

    /// <p>Updates an existing custom verification email template.</p> <p>For more information about custom verification email templates, see <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/send-email-verify-address-custom.html">Using Custom Verification Email Templates</a> in the <i>Amazon SES Developer Guide</i>.</p> <p>You can execute this operation no more than once per second.</p>
    async fn update_custom_verification_email_template(
        &self,
        input: UpdateCustomVerificationEmailTemplateRequest,
    ) -> Result<
        UpdateCustomVerificationEmailTemplateResponse,
        RusotoError<UpdateCustomVerificationEmailTemplateError>,
    >;

    /// <p>Updates the specified sending authorization policy for the given identity (an email address or a domain). This API returns successfully even if a policy with the specified name does not exist.</p> <note> <p>This API is for the identity owner only. If you have not verified the identity, this API will return an error.</p> </note> <p>Sending authorization is a feature that enables an identity owner to authorize other senders to use its identities. For information about using sending authorization, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/sending-authorization.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    async fn update_email_identity_policy(
        &self,
        input: UpdateEmailIdentityPolicyRequest,
    ) -> Result<UpdateEmailIdentityPolicyResponse, RusotoError<UpdateEmailIdentityPolicyError>>;

    /// <p>Updates an email template. Email templates enable you to send personalized email to one or more destinations in a single API operation. For more information, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/send-personalized-email-api.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    async fn update_email_template(
        &self,
        input: UpdateEmailTemplateRequest,
    ) -> Result<UpdateEmailTemplateResponse, RusotoError<UpdateEmailTemplateError>>;
}
/// A client for the Amazon SES V2 API.
#[derive(Clone)]
pub struct SesV2Client {
    client: Client,
    region: region::Region,
}

impl SesV2Client {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> SesV2Client {
        SesV2Client {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> SesV2Client
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        SesV2Client {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> SesV2Client {
        SesV2Client { client, region }
    }
}

#[async_trait]
impl SesV2 for SesV2Client {
    /// <p>Create a configuration set. <i>Configuration sets</i> are groups of rules that you can apply to the emails that you send. You apply a configuration set to an email by specifying the name of the configuration set when you call the Amazon SES API v2. When you apply a configuration set to an email, all of the rules in that configuration set are applied to the email. </p>
    #[allow(unused_mut)]
    async fn create_configuration_set(
        &self,
        input: CreateConfigurationSetRequest,
    ) -> Result<CreateConfigurationSetResponse, RusotoError<CreateConfigurationSetError>> {
        let request_uri = "/v2/email/configuration-sets";

        let mut request = SignedRequest::new("POST", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateConfigurationSetResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateConfigurationSetError::from_response(response))
        }
    }

    /// <p>Create an event destination. <i>Events</i> include message sends, deliveries, opens, clicks, bounces, and complaints. <i>Event destinations</i> are places that you can send information about these events to. For example, you can send event data to Amazon SNS to receive notifications when you receive bounces or complaints, or you can use Amazon Kinesis Data Firehose to stream data to Amazon S3 for long-term storage.</p> <p>A single configuration set can include more than one event destination.</p>
    #[allow(unused_mut)]
    async fn create_configuration_set_event_destination(
        &self,
        input: CreateConfigurationSetEventDestinationRequest,
    ) -> Result<
        CreateConfigurationSetEventDestinationResponse,
        RusotoError<CreateConfigurationSetEventDestinationError>,
    > {
        let request_uri = format!(
            "/v2/email/configuration-sets/{configuration_set_name}/event-destinations",
            configuration_set_name = input.configuration_set_name
        );

        let mut request = SignedRequest::new("POST", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateConfigurationSetEventDestinationResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateConfigurationSetEventDestinationError::from_response(
                response,
            ))
        }
    }

    /// <p>Creates a contact, which is an end-user who is receiving the email, and adds them to a contact list.</p>
    #[allow(unused_mut)]
    async fn create_contact(
        &self,
        input: CreateContactRequest,
    ) -> Result<CreateContactResponse, RusotoError<CreateContactError>> {
        let request_uri = format!(
            "/v2/email/contact-lists/{contact_list_name}/contacts",
            contact_list_name = input.contact_list_name
        );

        let mut request = SignedRequest::new("POST", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateContactResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateContactError::from_response(response))
        }
    }

    /// <p>Creates a contact list.</p>
    #[allow(unused_mut)]
    async fn create_contact_list(
        &self,
        input: CreateContactListRequest,
    ) -> Result<CreateContactListResponse, RusotoError<CreateContactListError>> {
        let request_uri = "/v2/email/contact-lists";

        let mut request = SignedRequest::new("POST", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateContactListResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateContactListError::from_response(response))
        }
    }

    /// <p>Creates a new custom verification email template.</p> <p>For more information about custom verification email templates, see <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/send-email-verify-address-custom.html">Using Custom Verification Email Templates</a> in the <i>Amazon SES Developer Guide</i>.</p> <p>You can execute this operation no more than once per second.</p>
    #[allow(unused_mut)]
    async fn create_custom_verification_email_template(
        &self,
        input: CreateCustomVerificationEmailTemplateRequest,
    ) -> Result<
        CreateCustomVerificationEmailTemplateResponse,
        RusotoError<CreateCustomVerificationEmailTemplateError>,
    > {
        let request_uri = "/v2/email/custom-verification-email-templates";

        let mut request = SignedRequest::new("POST", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateCustomVerificationEmailTemplateResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateCustomVerificationEmailTemplateError::from_response(
                response,
            ))
        }
    }

    /// <p>Create a new pool of dedicated IP addresses. A pool can include one or more dedicated IP addresses that are associated with your AWS account. You can associate a pool with a configuration set. When you send an email that uses that configuration set, the message is sent from one of the addresses in the associated pool.</p>
    #[allow(unused_mut)]
    async fn create_dedicated_ip_pool(
        &self,
        input: CreateDedicatedIpPoolRequest,
    ) -> Result<CreateDedicatedIpPoolResponse, RusotoError<CreateDedicatedIpPoolError>> {
        let request_uri = "/v2/email/dedicated-ip-pools";

        let mut request = SignedRequest::new("POST", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateDedicatedIpPoolResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateDedicatedIpPoolError::from_response(response))
        }
    }

    /// <p>Create a new predictive inbox placement test. Predictive inbox placement tests can help you predict how your messages will be handled by various email providers around the world. When you perform a predictive inbox placement test, you provide a sample message that contains the content that you plan to send to your customers. Amazon SES then sends that message to special email addresses spread across several major email providers. After about 24 hours, the test is complete, and you can use the <code>GetDeliverabilityTestReport</code> operation to view the results of the test.</p>
    #[allow(unused_mut)]
    async fn create_deliverability_test_report(
        &self,
        input: CreateDeliverabilityTestReportRequest,
    ) -> Result<
        CreateDeliverabilityTestReportResponse,
        RusotoError<CreateDeliverabilityTestReportError>,
    > {
        let request_uri = "/v2/email/deliverability-dashboard/test";

        let mut request = SignedRequest::new("POST", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateDeliverabilityTestReportResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateDeliverabilityTestReportError::from_response(response))
        }
    }

    /// <p>Starts the process of verifying an email identity. An <i>identity</i> is an email address or domain that you use when you send email. Before you can use an identity to send email, you first have to verify it. By verifying an identity, you demonstrate that you're the owner of the identity, and that you've given Amazon SES API v2 permission to send email from the identity.</p> <p>When you verify an email address, Amazon SES sends an email to the address. Your email address is verified as soon as you follow the link in the verification email. </p> <p>When you verify a domain without specifying the <code>DkimSigningAttributes</code> object, this operation provides a set of DKIM tokens. You can convert these tokens into CNAME records, which you then add to the DNS configuration for your domain. Your domain is verified when Amazon SES detects these records in the DNS configuration for your domain. This verification method is known as <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/easy-dkim.html">Easy DKIM</a>.</p> <p>Alternatively, you can perform the verification process by providing your own public-private key pair. This verification method is known as Bring Your Own DKIM (BYODKIM). To use BYODKIM, your call to the <code>CreateEmailIdentity</code> operation has to include the <code>DkimSigningAttributes</code> object. When you specify this object, you provide a selector (a component of the DNS record name that identifies the public key that you want to use for DKIM authentication) and a private key.</p>
    #[allow(unused_mut)]
    async fn create_email_identity(
        &self,
        input: CreateEmailIdentityRequest,
    ) -> Result<CreateEmailIdentityResponse, RusotoError<CreateEmailIdentityError>> {
        let request_uri = "/v2/email/identities";

        let mut request = SignedRequest::new("POST", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateEmailIdentityResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateEmailIdentityError::from_response(response))
        }
    }

    /// <p>Creates the specified sending authorization policy for the given identity (an email address or a domain).</p> <note> <p>This API is for the identity owner only. If you have not verified the identity, this API will return an error.</p> </note> <p>Sending authorization is a feature that enables an identity owner to authorize other senders to use its identities. For information about using sending authorization, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/sending-authorization.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    #[allow(unused_mut)]
    async fn create_email_identity_policy(
        &self,
        input: CreateEmailIdentityPolicyRequest,
    ) -> Result<CreateEmailIdentityPolicyResponse, RusotoError<CreateEmailIdentityPolicyError>>
    {
        let request_uri = format!(
            "/v2/email/identities/{email_identity}/policies/{policy_name}",
            email_identity = input.email_identity,
            policy_name = input.policy_name
        );

        let mut request = SignedRequest::new("POST", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateEmailIdentityPolicyResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateEmailIdentityPolicyError::from_response(response))
        }
    }

    /// <p>Creates an email template. Email templates enable you to send personalized email to one or more destinations in a single API operation. For more information, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/send-personalized-email-api.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    #[allow(unused_mut)]
    async fn create_email_template(
        &self,
        input: CreateEmailTemplateRequest,
    ) -> Result<CreateEmailTemplateResponse, RusotoError<CreateEmailTemplateError>> {
        let request_uri = "/v2/email/templates";

        let mut request = SignedRequest::new("POST", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateEmailTemplateResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateEmailTemplateError::from_response(response))
        }
    }

    /// <p>Creates an import job for a data destination.</p>
    #[allow(unused_mut)]
    async fn create_import_job(
        &self,
        input: CreateImportJobRequest,
    ) -> Result<CreateImportJobResponse, RusotoError<CreateImportJobError>> {
        let request_uri = "/v2/email/import-jobs";

        let mut request = SignedRequest::new("POST", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateImportJobResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateImportJobError::from_response(response))
        }
    }

    /// <p>Delete an existing configuration set.</p> <p> <i>Configuration sets</i> are groups of rules that you can apply to the emails you send. You apply a configuration set to an email by including a reference to the configuration set in the headers of the email. When you apply a configuration set to an email, all of the rules in that configuration set are applied to the email.</p>
    #[allow(unused_mut)]
    async fn delete_configuration_set(
        &self,
        input: DeleteConfigurationSetRequest,
    ) -> Result<DeleteConfigurationSetResponse, RusotoError<DeleteConfigurationSetError>> {
        let request_uri = format!(
            "/v2/email/configuration-sets/{configuration_set_name}",
            configuration_set_name = input.configuration_set_name
        );

        let mut request = SignedRequest::new("DELETE", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteConfigurationSetResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteConfigurationSetError::from_response(response))
        }
    }

    /// <p>Delete an event destination.</p> <p> <i>Events</i> include message sends, deliveries, opens, clicks, bounces, and complaints. <i>Event destinations</i> are places that you can send information about these events to. For example, you can send event data to Amazon SNS to receive notifications when you receive bounces or complaints, or you can use Amazon Kinesis Data Firehose to stream data to Amazon S3 for long-term storage.</p>
    #[allow(unused_mut)]
    async fn delete_configuration_set_event_destination(
        &self,
        input: DeleteConfigurationSetEventDestinationRequest,
    ) -> Result<
        DeleteConfigurationSetEventDestinationResponse,
        RusotoError<DeleteConfigurationSetEventDestinationError>,
    > {
        let request_uri = format!("/v2/email/configuration-sets/{configuration_set_name}/event-destinations/{event_destination_name}", configuration_set_name = input.configuration_set_name, event_destination_name = input.event_destination_name);

        let mut request = SignedRequest::new("DELETE", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteConfigurationSetEventDestinationResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteConfigurationSetEventDestinationError::from_response(
                response,
            ))
        }
    }

    /// <p>Removes a contact from a contact list.</p>
    #[allow(unused_mut)]
    async fn delete_contact(
        &self,
        input: DeleteContactRequest,
    ) -> Result<DeleteContactResponse, RusotoError<DeleteContactError>> {
        let request_uri = format!(
            "/v2/email/contact-lists/{contact_list_name}/contacts/{email_address}",
            contact_list_name = input.contact_list_name,
            email_address = input.email_address
        );

        let mut request = SignedRequest::new("DELETE", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteContactResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteContactError::from_response(response))
        }
    }

    /// <p>Deletes a contact list and all of the contacts on that list.</p>
    #[allow(unused_mut)]
    async fn delete_contact_list(
        &self,
        input: DeleteContactListRequest,
    ) -> Result<DeleteContactListResponse, RusotoError<DeleteContactListError>> {
        let request_uri = format!(
            "/v2/email/contact-lists/{contact_list_name}",
            contact_list_name = input.contact_list_name
        );

        let mut request = SignedRequest::new("DELETE", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteContactListResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteContactListError::from_response(response))
        }
    }

    /// <p>Deletes an existing custom verification email template.</p> <p>For more information about custom verification email templates, see <a href="https://docs.aws.amazon.com/es/latest/DeveloperGuide/send-email-verify-address-custom.html">Using Custom Verification Email Templates</a> in the <i>Amazon SES Developer Guide</i>.</p> <p>You can execute this operation no more than once per second.</p>
    #[allow(unused_mut)]
    async fn delete_custom_verification_email_template(
        &self,
        input: DeleteCustomVerificationEmailTemplateRequest,
    ) -> Result<
        DeleteCustomVerificationEmailTemplateResponse,
        RusotoError<DeleteCustomVerificationEmailTemplateError>,
    > {
        let request_uri = format!(
            "/v2/email/custom-verification-email-templates/{template_name}",
            template_name = input.template_name
        );

        let mut request = SignedRequest::new("DELETE", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteCustomVerificationEmailTemplateResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteCustomVerificationEmailTemplateError::from_response(
                response,
            ))
        }
    }

    /// <p>Delete a dedicated IP pool.</p>
    #[allow(unused_mut)]
    async fn delete_dedicated_ip_pool(
        &self,
        input: DeleteDedicatedIpPoolRequest,
    ) -> Result<DeleteDedicatedIpPoolResponse, RusotoError<DeleteDedicatedIpPoolError>> {
        let request_uri = format!(
            "/v2/email/dedicated-ip-pools/{pool_name}",
            pool_name = input.pool_name
        );

        let mut request = SignedRequest::new("DELETE", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteDedicatedIpPoolResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteDedicatedIpPoolError::from_response(response))
        }
    }

    /// <p>Deletes an email identity. An identity can be either an email address or a domain name.</p>
    #[allow(unused_mut)]
    async fn delete_email_identity(
        &self,
        input: DeleteEmailIdentityRequest,
    ) -> Result<DeleteEmailIdentityResponse, RusotoError<DeleteEmailIdentityError>> {
        let request_uri = format!(
            "/v2/email/identities/{email_identity}",
            email_identity = input.email_identity
        );

        let mut request = SignedRequest::new("DELETE", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteEmailIdentityResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteEmailIdentityError::from_response(response))
        }
    }

    /// <p>Deletes the specified sending authorization policy for the given identity (an email address or a domain). This API returns successfully even if a policy with the specified name does not exist.</p> <note> <p>This API is for the identity owner only. If you have not verified the identity, this API will return an error.</p> </note> <p>Sending authorization is a feature that enables an identity owner to authorize other senders to use its identities. For information about using sending authorization, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/sending-authorization.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    #[allow(unused_mut)]
    async fn delete_email_identity_policy(
        &self,
        input: DeleteEmailIdentityPolicyRequest,
    ) -> Result<DeleteEmailIdentityPolicyResponse, RusotoError<DeleteEmailIdentityPolicyError>>
    {
        let request_uri = format!(
            "/v2/email/identities/{email_identity}/policies/{policy_name}",
            email_identity = input.email_identity,
            policy_name = input.policy_name
        );

        let mut request = SignedRequest::new("DELETE", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteEmailIdentityPolicyResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteEmailIdentityPolicyError::from_response(response))
        }
    }

    /// <p>Deletes an email template.</p> <p>You can execute this operation no more than once per second.</p>
    #[allow(unused_mut)]
    async fn delete_email_template(
        &self,
        input: DeleteEmailTemplateRequest,
    ) -> Result<DeleteEmailTemplateResponse, RusotoError<DeleteEmailTemplateError>> {
        let request_uri = format!(
            "/v2/email/templates/{template_name}",
            template_name = input.template_name
        );

        let mut request = SignedRequest::new("DELETE", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteEmailTemplateResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteEmailTemplateError::from_response(response))
        }
    }

    /// <p>Removes an email address from the suppression list for your account.</p>
    #[allow(unused_mut)]
    async fn delete_suppressed_destination(
        &self,
        input: DeleteSuppressedDestinationRequest,
    ) -> Result<DeleteSuppressedDestinationResponse, RusotoError<DeleteSuppressedDestinationError>>
    {
        let request_uri = format!(
            "/v2/email/suppression/addresses/{email_address}",
            email_address = input.email_address
        );

        let mut request = SignedRequest::new("DELETE", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteSuppressedDestinationResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteSuppressedDestinationError::from_response(response))
        }
    }

    /// <p>Obtain information about the email-sending status and capabilities of your Amazon SES account in the current AWS Region.</p>
    #[allow(unused_mut)]
    async fn get_account(&self) -> Result<GetAccountResponse, RusotoError<GetAccountError>> {
        let request_uri = "/v2/email/account";

        let mut request = SignedRequest::new("GET", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetAccountResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetAccountError::from_response(response))
        }
    }

    /// <p>Retrieve a list of the blacklists that your dedicated IP addresses appear on.</p>
    #[allow(unused_mut)]
    async fn get_blacklist_reports(
        &self,
        input: GetBlacklistReportsRequest,
    ) -> Result<GetBlacklistReportsResponse, RusotoError<GetBlacklistReportsError>> {
        let request_uri = "/v2/email/deliverability-dashboard/blacklist-report";

        let mut request = SignedRequest::new("GET", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());

        let mut params = Params::new();
        for item in input.blacklist_item_names.iter() {
            params.put("BlacklistItemNames", item);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetBlacklistReportsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetBlacklistReportsError::from_response(response))
        }
    }

    /// <p>Get information about an existing configuration set, including the dedicated IP pool that it's associated with, whether or not it's enabled for sending email, and more.</p> <p> <i>Configuration sets</i> are groups of rules that you can apply to the emails you send. You apply a configuration set to an email by including a reference to the configuration set in the headers of the email. When you apply a configuration set to an email, all of the rules in that configuration set are applied to the email.</p>
    #[allow(unused_mut)]
    async fn get_configuration_set(
        &self,
        input: GetConfigurationSetRequest,
    ) -> Result<GetConfigurationSetResponse, RusotoError<GetConfigurationSetError>> {
        let request_uri = format!(
            "/v2/email/configuration-sets/{configuration_set_name}",
            configuration_set_name = input.configuration_set_name
        );

        let mut request = SignedRequest::new("GET", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetConfigurationSetResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetConfigurationSetError::from_response(response))
        }
    }

    /// <p>Retrieve a list of event destinations that are associated with a configuration set.</p> <p> <i>Events</i> include message sends, deliveries, opens, clicks, bounces, and complaints. <i>Event destinations</i> are places that you can send information about these events to. For example, you can send event data to Amazon SNS to receive notifications when you receive bounces or complaints, or you can use Amazon Kinesis Data Firehose to stream data to Amazon S3 for long-term storage.</p>
    #[allow(unused_mut)]
    async fn get_configuration_set_event_destinations(
        &self,
        input: GetConfigurationSetEventDestinationsRequest,
    ) -> Result<
        GetConfigurationSetEventDestinationsResponse,
        RusotoError<GetConfigurationSetEventDestinationsError>,
    > {
        let request_uri = format!(
            "/v2/email/configuration-sets/{configuration_set_name}/event-destinations",
            configuration_set_name = input.configuration_set_name
        );

        let mut request = SignedRequest::new("GET", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetConfigurationSetEventDestinationsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetConfigurationSetEventDestinationsError::from_response(
                response,
            ))
        }
    }

    /// <p>Returns a contact from a contact list.</p>
    #[allow(unused_mut)]
    async fn get_contact(
        &self,
        input: GetContactRequest,
    ) -> Result<GetContactResponse, RusotoError<GetContactError>> {
        let request_uri = format!(
            "/v2/email/contact-lists/{contact_list_name}/contacts/{email_address}",
            contact_list_name = input.contact_list_name,
            email_address = input.email_address
        );

        let mut request = SignedRequest::new("GET", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetContactResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetContactError::from_response(response))
        }
    }

    /// <p>Returns contact list metadata. It does not return any information about the contacts present in the list.</p>
    #[allow(unused_mut)]
    async fn get_contact_list(
        &self,
        input: GetContactListRequest,
    ) -> Result<GetContactListResponse, RusotoError<GetContactListError>> {
        let request_uri = format!(
            "/v2/email/contact-lists/{contact_list_name}",
            contact_list_name = input.contact_list_name
        );

        let mut request = SignedRequest::new("GET", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetContactListResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetContactListError::from_response(response))
        }
    }

    /// <p>Returns the custom email verification template for the template name you specify.</p> <p>For more information about custom verification email templates, see <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/send-email-verify-address-custom.html">Using Custom Verification Email Templates</a> in the <i>Amazon SES Developer Guide</i>.</p> <p>You can execute this operation no more than once per second.</p>
    #[allow(unused_mut)]
    async fn get_custom_verification_email_template(
        &self,
        input: GetCustomVerificationEmailTemplateRequest,
    ) -> Result<
        GetCustomVerificationEmailTemplateResponse,
        RusotoError<GetCustomVerificationEmailTemplateError>,
    > {
        let request_uri = format!(
            "/v2/email/custom-verification-email-templates/{template_name}",
            template_name = input.template_name
        );

        let mut request = SignedRequest::new("GET", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetCustomVerificationEmailTemplateResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetCustomVerificationEmailTemplateError::from_response(
                response,
            ))
        }
    }

    /// <p>Get information about a dedicated IP address, including the name of the dedicated IP pool that it's associated with, as well information about the automatic warm-up process for the address.</p>
    #[allow(unused_mut)]
    async fn get_dedicated_ip(
        &self,
        input: GetDedicatedIpRequest,
    ) -> Result<GetDedicatedIpResponse, RusotoError<GetDedicatedIpError>> {
        let request_uri = format!("/v2/email/dedicated-ips/{ip}", ip = input.ip);

        let mut request = SignedRequest::new("GET", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetDedicatedIpResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetDedicatedIpError::from_response(response))
        }
    }

    /// <p>List the dedicated IP addresses that are associated with your AWS account.</p>
    #[allow(unused_mut)]
    async fn get_dedicated_ips(
        &self,
        input: GetDedicatedIpsRequest,
    ) -> Result<GetDedicatedIpsResponse, RusotoError<GetDedicatedIpsError>> {
        let request_uri = "/v2/email/dedicated-ips";

        let mut request = SignedRequest::new("GET", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.next_token {
            params.put("NextToken", x);
        }
        if let Some(ref x) = input.page_size {
            params.put("PageSize", x);
        }
        if let Some(ref x) = input.pool_name {
            params.put("PoolName", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetDedicatedIpsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetDedicatedIpsError::from_response(response))
        }
    }

    /// <p>Retrieve information about the status of the Deliverability dashboard for your account. When the Deliverability dashboard is enabled, you gain access to reputation, deliverability, and other metrics for the domains that you use to send email. You also gain the ability to perform predictive inbox placement tests.</p> <p>When you use the Deliverability dashboard, you pay a monthly subscription charge, in addition to any other fees that you accrue by using Amazon SES and other AWS services. For more information about the features and cost of a Deliverability dashboard subscription, see <a href="http://aws.amazon.com/ses/pricing/">Amazon SES Pricing</a>.</p>
    #[allow(unused_mut)]
    async fn get_deliverability_dashboard_options(
        &self,
    ) -> Result<
        GetDeliverabilityDashboardOptionsResponse,
        RusotoError<GetDeliverabilityDashboardOptionsError>,
    > {
        let request_uri = "/v2/email/deliverability-dashboard";

        let mut request = SignedRequest::new("GET", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetDeliverabilityDashboardOptionsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetDeliverabilityDashboardOptionsError::from_response(
                response,
            ))
        }
    }

    /// <p>Retrieve the results of a predictive inbox placement test.</p>
    #[allow(unused_mut)]
    async fn get_deliverability_test_report(
        &self,
        input: GetDeliverabilityTestReportRequest,
    ) -> Result<GetDeliverabilityTestReportResponse, RusotoError<GetDeliverabilityTestReportError>>
    {
        let request_uri = format!(
            "/v2/email/deliverability-dashboard/test-reports/{report_id}",
            report_id = input.report_id
        );

        let mut request = SignedRequest::new("GET", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetDeliverabilityTestReportResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetDeliverabilityTestReportError::from_response(response))
        }
    }

    /// <p>Retrieve all the deliverability data for a specific campaign. This data is available for a campaign only if the campaign sent email by using a domain that the Deliverability dashboard is enabled for.</p>
    #[allow(unused_mut)]
    async fn get_domain_deliverability_campaign(
        &self,
        input: GetDomainDeliverabilityCampaignRequest,
    ) -> Result<
        GetDomainDeliverabilityCampaignResponse,
        RusotoError<GetDomainDeliverabilityCampaignError>,
    > {
        let request_uri = format!(
            "/v2/email/deliverability-dashboard/campaigns/{campaign_id}",
            campaign_id = input.campaign_id
        );

        let mut request = SignedRequest::new("GET", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetDomainDeliverabilityCampaignResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetDomainDeliverabilityCampaignError::from_response(
                response,
            ))
        }
    }

    /// <p>Retrieve inbox placement and engagement rates for the domains that you use to send email.</p>
    #[allow(unused_mut)]
    async fn get_domain_statistics_report(
        &self,
        input: GetDomainStatisticsReportRequest,
    ) -> Result<GetDomainStatisticsReportResponse, RusotoError<GetDomainStatisticsReportError>>
    {
        let request_uri = format!(
            "/v2/email/deliverability-dashboard/statistics-report/{domain}",
            domain = input.domain
        );

        let mut request = SignedRequest::new("GET", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());

        let mut params = Params::new();
        params.put("EndDate", &input.end_date);
        params.put("StartDate", &input.start_date);
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetDomainStatisticsReportResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetDomainStatisticsReportError::from_response(response))
        }
    }

    /// <p>Provides information about a specific identity, including the identity's verification status, sending authorization policies, its DKIM authentication status, and its custom Mail-From settings.</p>
    #[allow(unused_mut)]
    async fn get_email_identity(
        &self,
        input: GetEmailIdentityRequest,
    ) -> Result<GetEmailIdentityResponse, RusotoError<GetEmailIdentityError>> {
        let request_uri = format!(
            "/v2/email/identities/{email_identity}",
            email_identity = input.email_identity
        );

        let mut request = SignedRequest::new("GET", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetEmailIdentityResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetEmailIdentityError::from_response(response))
        }
    }

    /// <p>Returns the requested sending authorization policies for the given identity (an email address or a domain). The policies are returned as a map of policy names to policy contents. You can retrieve a maximum of 20 policies at a time.</p> <note> <p>This API is for the identity owner only. If you have not verified the identity, this API will return an error.</p> </note> <p>Sending authorization is a feature that enables an identity owner to authorize other senders to use its identities. For information about using sending authorization, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/sending-authorization.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    #[allow(unused_mut)]
    async fn get_email_identity_policies(
        &self,
        input: GetEmailIdentityPoliciesRequest,
    ) -> Result<GetEmailIdentityPoliciesResponse, RusotoError<GetEmailIdentityPoliciesError>> {
        let request_uri = format!(
            "/v2/email/identities/{email_identity}/policies",
            email_identity = input.email_identity
        );

        let mut request = SignedRequest::new("GET", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetEmailIdentityPoliciesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetEmailIdentityPoliciesError::from_response(response))
        }
    }

    /// <p>Displays the template object (which includes the subject line, HTML part and text part) for the template you specify.</p> <p>You can execute this operation no more than once per second.</p>
    #[allow(unused_mut)]
    async fn get_email_template(
        &self,
        input: GetEmailTemplateRequest,
    ) -> Result<GetEmailTemplateResponse, RusotoError<GetEmailTemplateError>> {
        let request_uri = format!(
            "/v2/email/templates/{template_name}",
            template_name = input.template_name
        );

        let mut request = SignedRequest::new("GET", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetEmailTemplateResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetEmailTemplateError::from_response(response))
        }
    }

    /// <p>Provides information about an import job.</p>
    #[allow(unused_mut)]
    async fn get_import_job(
        &self,
        input: GetImportJobRequest,
    ) -> Result<GetImportJobResponse, RusotoError<GetImportJobError>> {
        let request_uri = format!("/v2/email/import-jobs/{job_id}", job_id = input.job_id);

        let mut request = SignedRequest::new("GET", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetImportJobResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetImportJobError::from_response(response))
        }
    }

    /// <p>Retrieves information about a specific email address that's on the suppression list for your account.</p>
    #[allow(unused_mut)]
    async fn get_suppressed_destination(
        &self,
        input: GetSuppressedDestinationRequest,
    ) -> Result<GetSuppressedDestinationResponse, RusotoError<GetSuppressedDestinationError>> {
        let request_uri = format!(
            "/v2/email/suppression/addresses/{email_address}",
            email_address = input.email_address
        );

        let mut request = SignedRequest::new("GET", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetSuppressedDestinationResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetSuppressedDestinationError::from_response(response))
        }
    }

    /// <p>List all of the configuration sets associated with your account in the current region.</p> <p> <i>Configuration sets</i> are groups of rules that you can apply to the emails you send. You apply a configuration set to an email by including a reference to the configuration set in the headers of the email. When you apply a configuration set to an email, all of the rules in that configuration set are applied to the email.</p>
    #[allow(unused_mut)]
    async fn list_configuration_sets(
        &self,
        input: ListConfigurationSetsRequest,
    ) -> Result<ListConfigurationSetsResponse, RusotoError<ListConfigurationSetsError>> {
        let request_uri = "/v2/email/configuration-sets";

        let mut request = SignedRequest::new("GET", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.next_token {
            params.put("NextToken", x);
        }
        if let Some(ref x) = input.page_size {
            params.put("PageSize", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListConfigurationSetsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListConfigurationSetsError::from_response(response))
        }
    }

    /// <p>Lists all of the contact lists available.</p>
    #[allow(unused_mut)]
    async fn list_contact_lists(
        &self,
        input: ListContactListsRequest,
    ) -> Result<ListContactListsResponse, RusotoError<ListContactListsError>> {
        let request_uri = "/v2/email/contact-lists";

        let mut request = SignedRequest::new("GET", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.next_token {
            params.put("NextToken", x);
        }
        if let Some(ref x) = input.page_size {
            params.put("PageSize", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListContactListsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListContactListsError::from_response(response))
        }
    }

    /// <p>Lists the contacts present in a specific contact list.</p>
    #[allow(unused_mut)]
    async fn list_contacts(
        &self,
        input: ListContactsRequest,
    ) -> Result<ListContactsResponse, RusotoError<ListContactsError>> {
        let request_uri = format!(
            "/v2/email/contact-lists/{contact_list_name}/contacts",
            contact_list_name = input.contact_list_name
        );

        let mut request = SignedRequest::new("GET", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut params = Params::new();
        if let Some(ref x) = input.next_token {
            params.put("NextToken", x);
        }
        if let Some(ref x) = input.page_size {
            params.put("PageSize", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListContactsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListContactsError::from_response(response))
        }
    }

    /// <p>Lists the existing custom verification email templates for your account in the current AWS Region.</p> <p>For more information about custom verification email templates, see <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/send-email-verify-address-custom.html">Using Custom Verification Email Templates</a> in the <i>Amazon SES Developer Guide</i>.</p> <p>You can execute this operation no more than once per second.</p>
    #[allow(unused_mut)]
    async fn list_custom_verification_email_templates(
        &self,
        input: ListCustomVerificationEmailTemplatesRequest,
    ) -> Result<
        ListCustomVerificationEmailTemplatesResponse,
        RusotoError<ListCustomVerificationEmailTemplatesError>,
    > {
        let request_uri = "/v2/email/custom-verification-email-templates";

        let mut request = SignedRequest::new("GET", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.next_token {
            params.put("NextToken", x);
        }
        if let Some(ref x) = input.page_size {
            params.put("PageSize", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListCustomVerificationEmailTemplatesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListCustomVerificationEmailTemplatesError::from_response(
                response,
            ))
        }
    }

    /// <p>List all of the dedicated IP pools that exist in your AWS account in the current Region.</p>
    #[allow(unused_mut)]
    async fn list_dedicated_ip_pools(
        &self,
        input: ListDedicatedIpPoolsRequest,
    ) -> Result<ListDedicatedIpPoolsResponse, RusotoError<ListDedicatedIpPoolsError>> {
        let request_uri = "/v2/email/dedicated-ip-pools";

        let mut request = SignedRequest::new("GET", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.next_token {
            params.put("NextToken", x);
        }
        if let Some(ref x) = input.page_size {
            params.put("PageSize", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListDedicatedIpPoolsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListDedicatedIpPoolsError::from_response(response))
        }
    }

    /// <p>Show a list of the predictive inbox placement tests that you've performed, regardless of their statuses. For predictive inbox placement tests that are complete, you can use the <code>GetDeliverabilityTestReport</code> operation to view the results.</p>
    #[allow(unused_mut)]
    async fn list_deliverability_test_reports(
        &self,
        input: ListDeliverabilityTestReportsRequest,
    ) -> Result<
        ListDeliverabilityTestReportsResponse,
        RusotoError<ListDeliverabilityTestReportsError>,
    > {
        let request_uri = "/v2/email/deliverability-dashboard/test-reports";

        let mut request = SignedRequest::new("GET", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.next_token {
            params.put("NextToken", x);
        }
        if let Some(ref x) = input.page_size {
            params.put("PageSize", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListDeliverabilityTestReportsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListDeliverabilityTestReportsError::from_response(response))
        }
    }

    /// <p>Retrieve deliverability data for all the campaigns that used a specific domain to send email during a specified time range. This data is available for a domain only if you enabled the Deliverability dashboard for the domain.</p>
    #[allow(unused_mut)]
    async fn list_domain_deliverability_campaigns(
        &self,
        input: ListDomainDeliverabilityCampaignsRequest,
    ) -> Result<
        ListDomainDeliverabilityCampaignsResponse,
        RusotoError<ListDomainDeliverabilityCampaignsError>,
    > {
        let request_uri = format!(
            "/v2/email/deliverability-dashboard/domains/{subscribed_domain}/campaigns",
            subscribed_domain = input.subscribed_domain
        );

        let mut request = SignedRequest::new("GET", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());

        let mut params = Params::new();
        params.put("EndDate", &input.end_date);
        if let Some(ref x) = input.next_token {
            params.put("NextToken", x);
        }
        if let Some(ref x) = input.page_size {
            params.put("PageSize", x);
        }
        params.put("StartDate", &input.start_date);
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListDomainDeliverabilityCampaignsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListDomainDeliverabilityCampaignsError::from_response(
                response,
            ))
        }
    }

    /// <p>Returns a list of all of the email identities that are associated with your AWS account. An identity can be either an email address or a domain. This operation returns identities that are verified as well as those that aren't. This operation returns identities that are associated with Amazon SES and Amazon Pinpoint.</p>
    #[allow(unused_mut)]
    async fn list_email_identities(
        &self,
        input: ListEmailIdentitiesRequest,
    ) -> Result<ListEmailIdentitiesResponse, RusotoError<ListEmailIdentitiesError>> {
        let request_uri = "/v2/email/identities";

        let mut request = SignedRequest::new("GET", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.next_token {
            params.put("NextToken", x);
        }
        if let Some(ref x) = input.page_size {
            params.put("PageSize", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListEmailIdentitiesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListEmailIdentitiesError::from_response(response))
        }
    }

    /// <p>Lists the email templates present in your Amazon SES account in the current AWS Region.</p> <p>You can execute this operation no more than once per second.</p>
    #[allow(unused_mut)]
    async fn list_email_templates(
        &self,
        input: ListEmailTemplatesRequest,
    ) -> Result<ListEmailTemplatesResponse, RusotoError<ListEmailTemplatesError>> {
        let request_uri = "/v2/email/templates";

        let mut request = SignedRequest::new("GET", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.next_token {
            params.put("NextToken", x);
        }
        if let Some(ref x) = input.page_size {
            params.put("PageSize", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListEmailTemplatesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListEmailTemplatesError::from_response(response))
        }
    }

    /// <p>Lists all of the import jobs.</p>
    #[allow(unused_mut)]
    async fn list_import_jobs(
        &self,
        input: ListImportJobsRequest,
    ) -> Result<ListImportJobsResponse, RusotoError<ListImportJobsError>> {
        let request_uri = "/v2/email/import-jobs";

        let mut request = SignedRequest::new("GET", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut params = Params::new();
        if let Some(ref x) = input.next_token {
            params.put("NextToken", x);
        }
        if let Some(ref x) = input.page_size {
            params.put("PageSize", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListImportJobsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListImportJobsError::from_response(response))
        }
    }

    /// <p>Retrieves a list of email addresses that are on the suppression list for your account.</p>
    #[allow(unused_mut)]
    async fn list_suppressed_destinations(
        &self,
        input: ListSuppressedDestinationsRequest,
    ) -> Result<ListSuppressedDestinationsResponse, RusotoError<ListSuppressedDestinationsError>>
    {
        let request_uri = "/v2/email/suppression/addresses";

        let mut request = SignedRequest::new("GET", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.end_date {
            params.put("EndDate", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("NextToken", x);
        }
        if let Some(ref x) = input.page_size {
            params.put("PageSize", x);
        }
        if let Some(ref x) = input.reasons {
            for item in x.iter() {
                params.put("Reason", item);
            }
        }
        if let Some(ref x) = input.start_date {
            params.put("StartDate", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListSuppressedDestinationsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListSuppressedDestinationsError::from_response(response))
        }
    }

    /// <p>Retrieve a list of the tags (keys and values) that are associated with a specified resource. A <i>tag</i> is a label that you optionally define and associate with a resource. Each tag consists of a required <i>tag key</i> and an optional associated <i>tag value</i>. A tag key is a general label that acts as a category for more specific tag values. A tag value acts as a descriptor within a tag key.</p>
    #[allow(unused_mut)]
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>> {
        let request_uri = "/v2/email/tags";

        let mut request = SignedRequest::new("GET", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());

        let mut params = Params::new();
        params.put("ResourceArn", &input.resource_arn);
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListTagsForResourceResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListTagsForResourceError::from_response(response))
        }
    }

    /// <p>Enable or disable the automatic warm-up feature for dedicated IP addresses.</p>
    #[allow(unused_mut)]
    async fn put_account_dedicated_ip_warmup_attributes(
        &self,
        input: PutAccountDedicatedIpWarmupAttributesRequest,
    ) -> Result<
        PutAccountDedicatedIpWarmupAttributesResponse,
        RusotoError<PutAccountDedicatedIpWarmupAttributesError>,
    > {
        let request_uri = "/v2/email/account/dedicated-ips/warmup";

        let mut request = SignedRequest::new("PUT", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<PutAccountDedicatedIpWarmupAttributesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(PutAccountDedicatedIpWarmupAttributesError::from_response(
                response,
            ))
        }
    }

    /// <p>Update your Amazon SES account details.</p>
    #[allow(unused_mut)]
    async fn put_account_details(
        &self,
        input: PutAccountDetailsRequest,
    ) -> Result<PutAccountDetailsResponse, RusotoError<PutAccountDetailsError>> {
        let request_uri = "/v2/email/account/details";

        let mut request = SignedRequest::new("POST", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<PutAccountDetailsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(PutAccountDetailsError::from_response(response))
        }
    }

    /// <p>Enable or disable the ability of your account to send email.</p>
    #[allow(unused_mut)]
    async fn put_account_sending_attributes(
        &self,
        input: PutAccountSendingAttributesRequest,
    ) -> Result<PutAccountSendingAttributesResponse, RusotoError<PutAccountSendingAttributesError>>
    {
        let request_uri = "/v2/email/account/sending";

        let mut request = SignedRequest::new("PUT", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<PutAccountSendingAttributesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(PutAccountSendingAttributesError::from_response(response))
        }
    }

    /// <p>Change the settings for the account-level suppression list.</p>
    #[allow(unused_mut)]
    async fn put_account_suppression_attributes(
        &self,
        input: PutAccountSuppressionAttributesRequest,
    ) -> Result<
        PutAccountSuppressionAttributesResponse,
        RusotoError<PutAccountSuppressionAttributesError>,
    > {
        let request_uri = "/v2/email/account/suppression";

        let mut request = SignedRequest::new("PUT", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<PutAccountSuppressionAttributesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(PutAccountSuppressionAttributesError::from_response(
                response,
            ))
        }
    }

    /// <p>Associate a configuration set with a dedicated IP pool. You can use dedicated IP pools to create groups of dedicated IP addresses for sending specific types of email.</p>
    #[allow(unused_mut)]
    async fn put_configuration_set_delivery_options(
        &self,
        input: PutConfigurationSetDeliveryOptionsRequest,
    ) -> Result<
        PutConfigurationSetDeliveryOptionsResponse,
        RusotoError<PutConfigurationSetDeliveryOptionsError>,
    > {
        let request_uri = format!(
            "/v2/email/configuration-sets/{configuration_set_name}/delivery-options",
            configuration_set_name = input.configuration_set_name
        );

        let mut request = SignedRequest::new("PUT", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<PutConfigurationSetDeliveryOptionsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(PutConfigurationSetDeliveryOptionsError::from_response(
                response,
            ))
        }
    }

    /// <p>Enable or disable collection of reputation metrics for emails that you send using a particular configuration set in a specific AWS Region.</p>
    #[allow(unused_mut)]
    async fn put_configuration_set_reputation_options(
        &self,
        input: PutConfigurationSetReputationOptionsRequest,
    ) -> Result<
        PutConfigurationSetReputationOptionsResponse,
        RusotoError<PutConfigurationSetReputationOptionsError>,
    > {
        let request_uri = format!(
            "/v2/email/configuration-sets/{configuration_set_name}/reputation-options",
            configuration_set_name = input.configuration_set_name
        );

        let mut request = SignedRequest::new("PUT", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<PutConfigurationSetReputationOptionsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(PutConfigurationSetReputationOptionsError::from_response(
                response,
            ))
        }
    }

    /// <p>Enable or disable email sending for messages that use a particular configuration set in a specific AWS Region.</p>
    #[allow(unused_mut)]
    async fn put_configuration_set_sending_options(
        &self,
        input: PutConfigurationSetSendingOptionsRequest,
    ) -> Result<
        PutConfigurationSetSendingOptionsResponse,
        RusotoError<PutConfigurationSetSendingOptionsError>,
    > {
        let request_uri = format!(
            "/v2/email/configuration-sets/{configuration_set_name}/sending",
            configuration_set_name = input.configuration_set_name
        );

        let mut request = SignedRequest::new("PUT", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<PutConfigurationSetSendingOptionsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(PutConfigurationSetSendingOptionsError::from_response(
                response,
            ))
        }
    }

    /// <p>Specify the account suppression list preferences for a configuration set.</p>
    #[allow(unused_mut)]
    async fn put_configuration_set_suppression_options(
        &self,
        input: PutConfigurationSetSuppressionOptionsRequest,
    ) -> Result<
        PutConfigurationSetSuppressionOptionsResponse,
        RusotoError<PutConfigurationSetSuppressionOptionsError>,
    > {
        let request_uri = format!(
            "/v2/email/configuration-sets/{configuration_set_name}/suppression-options",
            configuration_set_name = input.configuration_set_name
        );

        let mut request = SignedRequest::new("PUT", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<PutConfigurationSetSuppressionOptionsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(PutConfigurationSetSuppressionOptionsError::from_response(
                response,
            ))
        }
    }

    /// <p>Specify a custom domain to use for open and click tracking elements in email that you send.</p>
    #[allow(unused_mut)]
    async fn put_configuration_set_tracking_options(
        &self,
        input: PutConfigurationSetTrackingOptionsRequest,
    ) -> Result<
        PutConfigurationSetTrackingOptionsResponse,
        RusotoError<PutConfigurationSetTrackingOptionsError>,
    > {
        let request_uri = format!(
            "/v2/email/configuration-sets/{configuration_set_name}/tracking-options",
            configuration_set_name = input.configuration_set_name
        );

        let mut request = SignedRequest::new("PUT", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<PutConfigurationSetTrackingOptionsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(PutConfigurationSetTrackingOptionsError::from_response(
                response,
            ))
        }
    }

    /// <p><p>Move a dedicated IP address to an existing dedicated IP pool.</p> <note> <p>The dedicated IP address that you specify must already exist, and must be associated with your AWS account. </p> <p>The dedicated IP pool you specify must already exist. You can create a new pool by using the <code>CreateDedicatedIpPool</code> operation.</p> </note></p>
    #[allow(unused_mut)]
    async fn put_dedicated_ip_in_pool(
        &self,
        input: PutDedicatedIpInPoolRequest,
    ) -> Result<PutDedicatedIpInPoolResponse, RusotoError<PutDedicatedIpInPoolError>> {
        let request_uri = format!("/v2/email/dedicated-ips/{ip}/pool", ip = input.ip);

        let mut request = SignedRequest::new("PUT", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<PutDedicatedIpInPoolResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(PutDedicatedIpInPoolError::from_response(response))
        }
    }

    /// <p><p/></p>
    #[allow(unused_mut)]
    async fn put_dedicated_ip_warmup_attributes(
        &self,
        input: PutDedicatedIpWarmupAttributesRequest,
    ) -> Result<
        PutDedicatedIpWarmupAttributesResponse,
        RusotoError<PutDedicatedIpWarmupAttributesError>,
    > {
        let request_uri = format!("/v2/email/dedicated-ips/{ip}/warmup", ip = input.ip);

        let mut request = SignedRequest::new("PUT", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<PutDedicatedIpWarmupAttributesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(PutDedicatedIpWarmupAttributesError::from_response(response))
        }
    }

    /// <p>Enable or disable the Deliverability dashboard. When you enable the Deliverability dashboard, you gain access to reputation, deliverability, and other metrics for the domains that you use to send email. You also gain the ability to perform predictive inbox placement tests.</p> <p>When you use the Deliverability dashboard, you pay a monthly subscription charge, in addition to any other fees that you accrue by using Amazon SES and other AWS services. For more information about the features and cost of a Deliverability dashboard subscription, see <a href="http://aws.amazon.com/ses/pricing/">Amazon SES Pricing</a>.</p>
    #[allow(unused_mut)]
    async fn put_deliverability_dashboard_option(
        &self,
        input: PutDeliverabilityDashboardOptionRequest,
    ) -> Result<
        PutDeliverabilityDashboardOptionResponse,
        RusotoError<PutDeliverabilityDashboardOptionError>,
    > {
        let request_uri = "/v2/email/deliverability-dashboard";

        let mut request = SignedRequest::new("PUT", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<PutDeliverabilityDashboardOptionResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(PutDeliverabilityDashboardOptionError::from_response(
                response,
            ))
        }
    }

    /// <p>Used to enable or disable DKIM authentication for an email identity.</p>
    #[allow(unused_mut)]
    async fn put_email_identity_dkim_attributes(
        &self,
        input: PutEmailIdentityDkimAttributesRequest,
    ) -> Result<
        PutEmailIdentityDkimAttributesResponse,
        RusotoError<PutEmailIdentityDkimAttributesError>,
    > {
        let request_uri = format!(
            "/v2/email/identities/{email_identity}/dkim",
            email_identity = input.email_identity
        );

        let mut request = SignedRequest::new("PUT", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<PutEmailIdentityDkimAttributesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(PutEmailIdentityDkimAttributesError::from_response(response))
        }
    }

    /// <p><p>Used to configure or change the DKIM authentication settings for an email domain identity. You can use this operation to do any of the following:</p> <ul> <li> <p>Update the signing attributes for an identity that uses Bring Your Own DKIM (BYODKIM).</p> </li> <li> <p>Change from using no DKIM authentication to using Easy DKIM.</p> </li> <li> <p>Change from using no DKIM authentication to using BYODKIM.</p> </li> <li> <p>Change from using Easy DKIM to using BYODKIM.</p> </li> <li> <p>Change from using BYODKIM to using Easy DKIM.</p> </li> </ul></p>
    #[allow(unused_mut)]
    async fn put_email_identity_dkim_signing_attributes(
        &self,
        input: PutEmailIdentityDkimSigningAttributesRequest,
    ) -> Result<
        PutEmailIdentityDkimSigningAttributesResponse,
        RusotoError<PutEmailIdentityDkimSigningAttributesError>,
    > {
        let request_uri = format!(
            "/v1/email/identities/{email_identity}/dkim/signing",
            email_identity = input.email_identity
        );

        let mut request = SignedRequest::new("PUT", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<PutEmailIdentityDkimSigningAttributesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(PutEmailIdentityDkimSigningAttributesError::from_response(
                response,
            ))
        }
    }

    /// <p>Used to enable or disable feedback forwarding for an identity. This setting determines what happens when an identity is used to send an email that results in a bounce or complaint event.</p> <p>If the value is <code>true</code>, you receive email notifications when bounce or complaint events occur. These notifications are sent to the address that you specified in the <code>Return-Path</code> header of the original email.</p> <p>You're required to have a method of tracking bounces and complaints. If you haven't set up another mechanism for receiving bounce or complaint notifications (for example, by setting up an event destination), you receive an email notification when these events occur (even if this setting is disabled).</p>
    #[allow(unused_mut)]
    async fn put_email_identity_feedback_attributes(
        &self,
        input: PutEmailIdentityFeedbackAttributesRequest,
    ) -> Result<
        PutEmailIdentityFeedbackAttributesResponse,
        RusotoError<PutEmailIdentityFeedbackAttributesError>,
    > {
        let request_uri = format!(
            "/v2/email/identities/{email_identity}/feedback",
            email_identity = input.email_identity
        );

        let mut request = SignedRequest::new("PUT", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<PutEmailIdentityFeedbackAttributesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(PutEmailIdentityFeedbackAttributesError::from_response(
                response,
            ))
        }
    }

    /// <p>Used to enable or disable the custom Mail-From domain configuration for an email identity.</p>
    #[allow(unused_mut)]
    async fn put_email_identity_mail_from_attributes(
        &self,
        input: PutEmailIdentityMailFromAttributesRequest,
    ) -> Result<
        PutEmailIdentityMailFromAttributesResponse,
        RusotoError<PutEmailIdentityMailFromAttributesError>,
    > {
        let request_uri = format!(
            "/v2/email/identities/{email_identity}/mail-from",
            email_identity = input.email_identity
        );

        let mut request = SignedRequest::new("PUT", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<PutEmailIdentityMailFromAttributesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(PutEmailIdentityMailFromAttributesError::from_response(
                response,
            ))
        }
    }

    /// <p>Adds an email address to the suppression list for your account.</p>
    #[allow(unused_mut)]
    async fn put_suppressed_destination(
        &self,
        input: PutSuppressedDestinationRequest,
    ) -> Result<PutSuppressedDestinationResponse, RusotoError<PutSuppressedDestinationError>> {
        let request_uri = "/v2/email/suppression/addresses";

        let mut request = SignedRequest::new("PUT", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<PutSuppressedDestinationResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(PutSuppressedDestinationError::from_response(response))
        }
    }

    /// <p>Composes an email message to multiple destinations.</p>
    #[allow(unused_mut)]
    async fn send_bulk_email(
        &self,
        input: SendBulkEmailRequest,
    ) -> Result<SendBulkEmailResponse, RusotoError<SendBulkEmailError>> {
        let request_uri = "/v2/email/outbound-bulk-emails";

        let mut request = SignedRequest::new("POST", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<SendBulkEmailResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(SendBulkEmailError::from_response(response))
        }
    }

    /// <p>Adds an email address to the list of identities for your Amazon SES account in the current AWS Region and attempts to verify it. As a result of executing this operation, a customized verification email is sent to the specified address.</p> <p>To use this operation, you must first create a custom verification email template. For more information about creating and using custom verification email templates, see <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/send-email-verify-address-custom.html">Using Custom Verification Email Templates</a> in the <i>Amazon SES Developer Guide</i>.</p> <p>You can execute this operation no more than once per second.</p>
    #[allow(unused_mut)]
    async fn send_custom_verification_email(
        &self,
        input: SendCustomVerificationEmailRequest,
    ) -> Result<SendCustomVerificationEmailResponse, RusotoError<SendCustomVerificationEmailError>>
    {
        let request_uri = "/v2/email/outbound-custom-verification-emails";

        let mut request = SignedRequest::new("POST", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<SendCustomVerificationEmailResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(SendCustomVerificationEmailError::from_response(response))
        }
    }

    /// <p><p>Sends an email message. You can use the Amazon SES API v2 to send two types of messages:</p> <ul> <li> <p> <b>Simple</b> – A standard email message. When you create this type of message, you specify the sender, the recipient, and the message body, and Amazon SES assembles the message for you.</p> </li> <li> <p> <b>Raw</b> – A raw, MIME-formatted email message. When you send this type of email, you have to specify all of the message headers, as well as the message body. You can use this message type to send messages that contain attachments. The message that you specify has to be a valid MIME message.</p> </li> <li> <p> <b>Templated</b> – A message that contains personalization tags. When you send this type of email, Amazon SES API v2 automatically replaces the tags with values that you specify.</p> </li> </ul></p>
    #[allow(unused_mut)]
    async fn send_email(
        &self,
        input: SendEmailRequest,
    ) -> Result<SendEmailResponse, RusotoError<SendEmailError>> {
        let request_uri = "/v2/email/outbound-emails";

        let mut request = SignedRequest::new("POST", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<SendEmailResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(SendEmailError::from_response(response))
        }
    }

    /// <p>Add one or more tags (keys and values) to a specified resource. A <i>tag</i> is a label that you optionally define and associate with a resource. Tags can help you categorize and manage resources in different ways, such as by purpose, owner, environment, or other criteria. A resource can have as many as 50 tags.</p> <p>Each tag consists of a required <i>tag key</i> and an associated <i>tag value</i>, both of which you define. A tag key is a general label that acts as a category for more specific tag values. A tag value acts as a descriptor within a tag key.</p>
    #[allow(unused_mut)]
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>> {
        let request_uri = "/v2/email/tags";

        let mut request = SignedRequest::new("POST", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<TagResourceResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(TagResourceError::from_response(response))
        }
    }

    /// <p>Creates a preview of the MIME content of an email when provided with a template and a set of replacement data.</p> <p>You can execute this operation no more than once per second.</p>
    #[allow(unused_mut)]
    async fn test_render_email_template(
        &self,
        input: TestRenderEmailTemplateRequest,
    ) -> Result<TestRenderEmailTemplateResponse, RusotoError<TestRenderEmailTemplateError>> {
        let request_uri = format!(
            "/v2/email/templates/{template_name}/render",
            template_name = input.template_name
        );

        let mut request = SignedRequest::new("POST", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<TestRenderEmailTemplateResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(TestRenderEmailTemplateError::from_response(response))
        }
    }

    /// <p>Remove one or more tags (keys and values) from a specified resource.</p>
    #[allow(unused_mut)]
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>> {
        let request_uri = "/v2/email/tags";

        let mut request = SignedRequest::new("DELETE", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());

        let mut params = Params::new();
        params.put("ResourceArn", &input.resource_arn);
        for item in input.tag_keys.iter() {
            params.put("TagKeys", item);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UntagResourceResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UntagResourceError::from_response(response))
        }
    }

    /// <p>Update the configuration of an event destination for a configuration set.</p> <p> <i>Events</i> include message sends, deliveries, opens, clicks, bounces, and complaints. <i>Event destinations</i> are places that you can send information about these events to. For example, you can send event data to Amazon SNS to receive notifications when you receive bounces or complaints, or you can use Amazon Kinesis Data Firehose to stream data to Amazon S3 for long-term storage.</p>
    #[allow(unused_mut)]
    async fn update_configuration_set_event_destination(
        &self,
        input: UpdateConfigurationSetEventDestinationRequest,
    ) -> Result<
        UpdateConfigurationSetEventDestinationResponse,
        RusotoError<UpdateConfigurationSetEventDestinationError>,
    > {
        let request_uri = format!("/v2/email/configuration-sets/{configuration_set_name}/event-destinations/{event_destination_name}", configuration_set_name = input.configuration_set_name, event_destination_name = input.event_destination_name);

        let mut request = SignedRequest::new("PUT", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateConfigurationSetEventDestinationResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateConfigurationSetEventDestinationError::from_response(
                response,
            ))
        }
    }

    /// <p>Updates a contact's preferences for a list. It is not necessary to specify all existing topic preferences in the TopicPreferences object, just the ones that need updating.</p>
    #[allow(unused_mut)]
    async fn update_contact(
        &self,
        input: UpdateContactRequest,
    ) -> Result<UpdateContactResponse, RusotoError<UpdateContactError>> {
        let request_uri = format!(
            "/v2/email/contact-lists/{contact_list_name}/contacts/{email_address}",
            contact_list_name = input.contact_list_name,
            email_address = input.email_address
        );

        let mut request = SignedRequest::new("PUT", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateContactResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateContactError::from_response(response))
        }
    }

    /// <p>Updates contact list metadata. This operation does a complete replacement.</p>
    #[allow(unused_mut)]
    async fn update_contact_list(
        &self,
        input: UpdateContactListRequest,
    ) -> Result<UpdateContactListResponse, RusotoError<UpdateContactListError>> {
        let request_uri = format!(
            "/v2/email/contact-lists/{contact_list_name}",
            contact_list_name = input.contact_list_name
        );

        let mut request = SignedRequest::new("PUT", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateContactListResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateContactListError::from_response(response))
        }
    }

    /// <p>Updates an existing custom verification email template.</p> <p>For more information about custom verification email templates, see <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/send-email-verify-address-custom.html">Using Custom Verification Email Templates</a> in the <i>Amazon SES Developer Guide</i>.</p> <p>You can execute this operation no more than once per second.</p>
    #[allow(unused_mut)]
    async fn update_custom_verification_email_template(
        &self,
        input: UpdateCustomVerificationEmailTemplateRequest,
    ) -> Result<
        UpdateCustomVerificationEmailTemplateResponse,
        RusotoError<UpdateCustomVerificationEmailTemplateError>,
    > {
        let request_uri = format!(
            "/v2/email/custom-verification-email-templates/{template_name}",
            template_name = input.template_name
        );

        let mut request = SignedRequest::new("PUT", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateCustomVerificationEmailTemplateResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateCustomVerificationEmailTemplateError::from_response(
                response,
            ))
        }
    }

    /// <p>Updates the specified sending authorization policy for the given identity (an email address or a domain). This API returns successfully even if a policy with the specified name does not exist.</p> <note> <p>This API is for the identity owner only. If you have not verified the identity, this API will return an error.</p> </note> <p>Sending authorization is a feature that enables an identity owner to authorize other senders to use its identities. For information about using sending authorization, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/sending-authorization.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    #[allow(unused_mut)]
    async fn update_email_identity_policy(
        &self,
        input: UpdateEmailIdentityPolicyRequest,
    ) -> Result<UpdateEmailIdentityPolicyResponse, RusotoError<UpdateEmailIdentityPolicyError>>
    {
        let request_uri = format!(
            "/v2/email/identities/{email_identity}/policies/{policy_name}",
            email_identity = input.email_identity,
            policy_name = input.policy_name
        );

        let mut request = SignedRequest::new("PUT", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateEmailIdentityPolicyResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateEmailIdentityPolicyError::from_response(response))
        }
    }

    /// <p>Updates an email template. Email templates enable you to send personalized email to one or more destinations in a single API operation. For more information, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/send-personalized-email-api.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    #[allow(unused_mut)]
    async fn update_email_template(
        &self,
        input: UpdateEmailTemplateRequest,
    ) -> Result<UpdateEmailTemplateResponse, RusotoError<UpdateEmailTemplateError>> {
        let request_uri = format!(
            "/v2/email/templates/{template_name}",
            template_name = input.template_name
        );

        let mut request = SignedRequest::new("PUT", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateEmailTemplateResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateEmailTemplateError::from_response(response))
        }
    }
}
