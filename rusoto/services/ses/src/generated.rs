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

use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::proto::xml::error::*;
use rusoto_core::proto::xml::util::{
    characters, deserialize_elements, end_element, find_start_element, peek_at_name, skip_tree,
    start_element,
};
use rusoto_core::proto::xml::util::{Next, Peek, XmlParseError, XmlResponse};
use rusoto_core::signature::SignedRequest;
use serde_urlencoded;
use std::str::FromStr;
use xml::reader::ParserConfig;
use xml::EventReader;

/// <p>When included in a receipt rule, this action adds a header to the received email.</p> <p>For information about adding a header using a receipt rule, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-action-add-header.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AddHeaderAction {
    /// <p>The name of the header to add. Must be between 1 and 50 characters, inclusive, and consist of alphanumeric (a-z, A-Z, 0-9) characters and dashes only.</p>
    pub header_name: String,
    /// <p>Must be less than 2048 characters, and must not contain newline characters ("\r" or "\n").</p>
    pub header_value: String,
}

struct AddHeaderActionDeserializer;
impl AddHeaderActionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AddHeaderAction, XmlParseError> {
        deserialize_elements::<_, AddHeaderAction, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "HeaderName" => {
                    obj.header_name = HeaderNameDeserializer::deserialize("HeaderName", stack)?;
                }
                "HeaderValue" => {
                    obj.header_value = HeaderValueDeserializer::deserialize("HeaderValue", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

/// Serialize `AddHeaderAction` contents to a `SignedRequest`.
struct AddHeaderActionSerializer;
impl AddHeaderActionSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &AddHeaderAction) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "HeaderName"), &obj.header_name);
        params.put(&format!("{}{}", prefix, "HeaderValue"), &obj.header_value);
    }
}

struct AddressDeserializer;
impl AddressDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct AddressListDeserializer;
impl AddressListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(AddressDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}

/// Serialize `AddressList` contents to a `SignedRequest`.
struct AddressListSerializer;
impl AddressListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

struct AmazonResourceNameDeserializer;
impl AmazonResourceNameDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct BehaviorOnMXFailureDeserializer;
impl BehaviorOnMXFailureDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Represents the body of the message. You can specify text, HTML, or both. If you use both, then the message should display correctly in the widest variety of email clients.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Body {
    /// <p>The content of the message, in HTML format. Use this for email clients that can process HTML. You can include clickable links, formatted text, and much more in an HTML message.</p>
    pub html: Option<Content>,
    /// <p>The content of the message, in text format. Use this for text-based email clients, or clients on high-latency networks (such as mobile devices).</p>
    pub text: Option<Content>,
}

/// Serialize `Body` contents to a `SignedRequest`.
struct BodySerializer;
impl BodySerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Body) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.html {
            ContentSerializer::serialize(params, &format!("{}{}", prefix, "Html"), field_value);
        }
        if let Some(ref field_value) = obj.text {
            ContentSerializer::serialize(params, &format!("{}{}", prefix, "Text"), field_value);
        }
    }
}

/// <p>When included in a receipt rule, this action rejects the received email by returning a bounce response to the sender and, optionally, publishes a notification to Amazon Simple Notification Service (Amazon SNS).</p> <p>For information about sending a bounce message in response to a received email, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-action-bounce.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BounceAction {
    /// <p>Human-readable text to include in the bounce message.</p>
    pub message: String,
    /// <p>The email address of the sender of the bounced email. This is the address from which the bounce message will be sent.</p>
    pub sender: String,
    /// <p>The SMTP reply code, as defined by <a href="https://tools.ietf.org/html/rfc5321">RFC 5321</a>.</p>
    pub smtp_reply_code: String,
    /// <p>The SMTP enhanced status code, as defined by <a href="https://tools.ietf.org/html/rfc3463">RFC 3463</a>.</p>
    pub status_code: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the Amazon SNS topic to notify when the bounce action is taken. An example of an Amazon SNS topic ARN is <code>arn:aws:sns:us-west-2:123456789012:MyTopic</code>. For more information about Amazon SNS topics, see the <a href="https://docs.aws.amazon.com/sns/latest/dg/CreateTopic.html">Amazon SNS Developer Guide</a>.</p>
    pub topic_arn: Option<String>,
}

struct BounceActionDeserializer;
impl BounceActionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<BounceAction, XmlParseError> {
        deserialize_elements::<_, BounceAction, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Message" => {
                    obj.message = BounceMessageDeserializer::deserialize("Message", stack)?;
                }
                "Sender" => {
                    obj.sender = AddressDeserializer::deserialize("Sender", stack)?;
                }
                "SmtpReplyCode" => {
                    obj.smtp_reply_code =
                        BounceSmtpReplyCodeDeserializer::deserialize("SmtpReplyCode", stack)?;
                }
                "StatusCode" => {
                    obj.status_code = Some(BounceStatusCodeDeserializer::deserialize(
                        "StatusCode",
                        stack,
                    )?);
                }
                "TopicArn" => {
                    obj.topic_arn = Some(AmazonResourceNameDeserializer::deserialize(
                        "TopicArn", stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

/// Serialize `BounceAction` contents to a `SignedRequest`.
struct BounceActionSerializer;
impl BounceActionSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &BounceAction) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "Message"), &obj.message);
        params.put(&format!("{}{}", prefix, "Sender"), &obj.sender);
        params.put(
            &format!("{}{}", prefix, "SmtpReplyCode"),
            &obj.smtp_reply_code,
        );
        if let Some(ref field_value) = obj.status_code {
            params.put(&format!("{}{}", prefix, "StatusCode"), &field_value);
        }
        if let Some(ref field_value) = obj.topic_arn {
            params.put(&format!("{}{}", prefix, "TopicArn"), &field_value);
        }
    }
}

struct BounceMessageDeserializer;
impl BounceMessageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct BounceSmtpReplyCodeDeserializer;
impl BounceSmtpReplyCodeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct BounceStatusCodeDeserializer;
impl BounceStatusCodeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Recipient-related information to include in the Delivery Status Notification (DSN) when an email that Amazon SES receives on your behalf bounces.</p> <p>For information about receiving email through Amazon SES, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BouncedRecipientInfo {
    /// <p>The reason for the bounce. You must provide either this parameter or <code>RecipientDsnFields</code>.</p>
    pub bounce_type: Option<String>,
    /// <p>The email address of the recipient of the bounced email.</p>
    pub recipient: String,
    /// <p>This parameter is used only for sending authorization. It is the ARN of the identity that is associated with the sending authorization policy that permits you to receive email for the recipient of the bounced email. For more information about sending authorization, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/sending-authorization.html">Amazon SES Developer Guide</a>.</p>
    pub recipient_arn: Option<String>,
    /// <p>Recipient-related DSN fields, most of which would normally be filled in automatically when provided with a <code>BounceType</code>. You must provide either this parameter or <code>BounceType</code>.</p>
    pub recipient_dsn_fields: Option<RecipientDsnFields>,
}

/// Serialize `BouncedRecipientInfo` contents to a `SignedRequest`.
struct BouncedRecipientInfoSerializer;
impl BouncedRecipientInfoSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &BouncedRecipientInfo) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.bounce_type {
            params.put(&format!("{}{}", prefix, "BounceType"), &field_value);
        }
        params.put(&format!("{}{}", prefix, "Recipient"), &obj.recipient);
        if let Some(ref field_value) = obj.recipient_arn {
            params.put(&format!("{}{}", prefix, "RecipientArn"), &field_value);
        }
        if let Some(ref field_value) = obj.recipient_dsn_fields {
            RecipientDsnFieldsSerializer::serialize(
                params,
                &format!("{}{}", prefix, "RecipientDsnFields"),
                field_value,
            );
        }
    }
}

/// Serialize `BouncedRecipientInfoList` contents to a `SignedRequest`.
struct BouncedRecipientInfoListSerializer;
impl BouncedRecipientInfoListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<BouncedRecipientInfo>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            BouncedRecipientInfoSerializer::serialize(params, &key, obj);
        }
    }
}

/// <p>An array that contains one or more Destinations, as well as the tags and replacement data associated with each of those Destinations.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BulkEmailDestination {
    pub destination: Destination,
    /// <p>A list of tags, in the form of name/value pairs, to apply to an email that you send using <code>SendBulkTemplatedEmail</code>. Tags correspond to characteristics of the email that you define, so that you can publish email sending events.</p>
    pub replacement_tags: Option<Vec<MessageTag>>,
    /// <p>A list of replacement values to apply to the template. This parameter is a JSON object, typically consisting of key-value pairs in which the keys correspond to replacement tags in the email template.</p>
    pub replacement_template_data: Option<String>,
}

/// Serialize `BulkEmailDestination` contents to a `SignedRequest`.
struct BulkEmailDestinationSerializer;
impl BulkEmailDestinationSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &BulkEmailDestination) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        DestinationSerializer::serialize(
            params,
            &format!("{}{}", prefix, "Destination"),
            &obj.destination,
        );
        if let Some(ref field_value) = obj.replacement_tags {
            MessageTagListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "ReplacementTags"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.replacement_template_data {
            params.put(
                &format!("{}{}", prefix, "ReplacementTemplateData"),
                &field_value,
            );
        }
    }
}

/// Serialize `BulkEmailDestinationList` contents to a `SignedRequest`.
struct BulkEmailDestinationListSerializer;
impl BulkEmailDestinationListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<BulkEmailDestination>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            BulkEmailDestinationSerializer::serialize(params, &key, obj);
        }
    }
}

/// <p>An object that contains the response from the <code>SendBulkTemplatedEmail</code> operation.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct BulkEmailDestinationStatus {
    /// <p>A description of an error that prevented a message being sent using the <code>SendBulkTemplatedEmail</code> operation.</p>
    pub error: Option<String>,
    /// <p>The unique message identifier returned from the <code>SendBulkTemplatedEmail</code> operation.</p>
    pub message_id: Option<String>,
    /// <p><p>The status of a message sent using the <code>SendBulkTemplatedEmail</code> operation.</p> <p>Possible values for this parameter include:</p> <ul> <li> <p> <code>Success</code>: Amazon SES accepted the message, and will attempt to deliver it to the recipients.</p> </li> <li> <p> <code>MessageRejected</code>: The message was rejected because it contained a virus.</p> </li> <li> <p> <code>MailFromDomainNotVerified</code>: The sender&#39;s email address or domain was not verified.</p> </li> <li> <p> <code>ConfigurationSetDoesNotExist</code>: The configuration set you specified does not exist.</p> </li> <li> <p> <code>TemplateDoesNotExist</code>: The template you specified does not exist.</p> </li> <li> <p> <code>AccountSuspended</code>: Your account has been shut down because of issues related to your email sending practices.</p> </li> <li> <p> <code>AccountThrottled</code>: The number of emails you can send has been reduced because your account has exceeded its allocated sending limit.</p> </li> <li> <p> <code>AccountDailyQuotaExceeded</code>: You have reached or exceeded the maximum number of emails you can send from your account in a 24-hour period.</p> </li> <li> <p> <code>InvalidSendingPoolName</code>: The configuration set you specified refers to an IP pool that does not exist.</p> </li> <li> <p> <code>AccountSendingPaused</code>: Email sending for the Amazon SES account was disabled using the <a>UpdateAccountSendingEnabled</a> operation.</p> </li> <li> <p> <code>ConfigurationSetSendingPaused</code>: Email sending for this configuration set was disabled using the <a>UpdateConfigurationSetSendingEnabled</a> operation.</p> </li> <li> <p> <code>InvalidParameterValue</code>: One or more of the parameters you specified when calling this operation was invalid. See the error message for additional information.</p> </li> <li> <p> <code>TransientFailure</code>: Amazon SES was unable to process your request because of a temporary issue.</p> </li> <li> <p> <code>Failed</code>: Amazon SES was unable to process your request. See the error message for additional information.</p> </li> </ul></p>
    pub status: Option<String>,
}

struct BulkEmailDestinationStatusDeserializer;
impl BulkEmailDestinationStatusDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<BulkEmailDestinationStatus, XmlParseError> {
        deserialize_elements::<_, BulkEmailDestinationStatus, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Error" => {
                        obj.error = Some(SesErrorDeserializer::deserialize("Error", stack)?);
                    }
                    "MessageId" => {
                        obj.message_id =
                            Some(MessageIdDeserializer::deserialize("MessageId", stack)?);
                    }
                    "Status" => {
                        obj.status =
                            Some(BulkEmailStatusDeserializer::deserialize("Status", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
struct BulkEmailDestinationStatusListDeserializer;
impl BulkEmailDestinationStatusListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<BulkEmailDestinationStatus>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(BulkEmailDestinationStatusDeserializer::deserialize(
                    "member", stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
struct BulkEmailStatusDeserializer;
impl BulkEmailStatusDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct CidrDeserializer;
impl CidrDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Represents a request to create a receipt rule set by cloning an existing one. You use receipt rule sets to receive email with Amazon SES. For more information, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-concepts.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CloneReceiptRuleSetRequest {
    /// <p>The name of the rule set to clone.</p>
    pub original_rule_set_name: String,
    /// <p><p>The name of the rule set to create. The name must:</p> <ul> <li> <p>This value can only contain ASCII letters (a-z, A-Z), numbers (0-9), underscores (_), or dashes (-).</p> </li> <li> <p>Start and end with a letter or number.</p> </li> <li> <p>Contain less than 64 characters.</p> </li> </ul></p>
    pub rule_set_name: String,
}

/// Serialize `CloneReceiptRuleSetRequest` contents to a `SignedRequest`.
struct CloneReceiptRuleSetRequestSerializer;
impl CloneReceiptRuleSetRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CloneReceiptRuleSetRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "OriginalRuleSetName"),
            &obj.original_rule_set_name,
        );
        params.put(&format!("{}{}", prefix, "RuleSetName"), &obj.rule_set_name);
    }
}

/// <p>An empty element returned on a successful request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CloneReceiptRuleSetResponse {}

struct CloneReceiptRuleSetResponseDeserializer;
impl CloneReceiptRuleSetResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CloneReceiptRuleSetResponse, XmlParseError> {
        start_element(tag_name, stack)?;

        let obj = CloneReceiptRuleSetResponse::default();

        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Contains information associated with an Amazon CloudWatch event destination to which email sending events are published.</p> <p>Event destinations, such as Amazon CloudWatch, are associated with configuration sets, which enable you to publish email sending events. For information about using configuration sets, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/monitor-sending-activity.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CloudWatchDestination {
    /// <p>A list of dimensions upon which to categorize your emails when you publish email sending events to Amazon CloudWatch.</p>
    pub dimension_configurations: Vec<CloudWatchDimensionConfiguration>,
}

struct CloudWatchDestinationDeserializer;
impl CloudWatchDestinationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CloudWatchDestination, XmlParseError> {
        deserialize_elements::<_, CloudWatchDestination, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "DimensionConfigurations" => {
                    obj.dimension_configurations.extend(
                        CloudWatchDimensionConfigurationsDeserializer::deserialize(
                            "DimensionConfigurations",
                            stack,
                        )?,
                    );
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

/// Serialize `CloudWatchDestination` contents to a `SignedRequest`.
struct CloudWatchDestinationSerializer;
impl CloudWatchDestinationSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CloudWatchDestination) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        CloudWatchDimensionConfigurationsSerializer::serialize(
            params,
            &format!("{}{}", prefix, "DimensionConfigurations"),
            &obj.dimension_configurations,
        );
    }
}

/// <p>Contains the dimension configuration to use when you publish email sending events to Amazon CloudWatch.</p> <p>For information about publishing email sending events to Amazon CloudWatch, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/monitor-sending-activity.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CloudWatchDimensionConfiguration {
    /// <p><p>The default value of the dimension that is published to Amazon CloudWatch if you do not provide the value of the dimension when you send an email. The default value must:</p> <ul> <li> <p>This value can only contain ASCII letters (a-z, A-Z), numbers (0-9), underscores (_), or dashes (-).</p> </li> <li> <p>Contain less than 256 characters.</p> </li> </ul></p>
    pub default_dimension_value: String,
    /// <p><p>The name of an Amazon CloudWatch dimension associated with an email sending metric. The name must:</p> <ul> <li> <p>This value can only contain ASCII letters (a-z, A-Z), numbers (0-9), underscores (_), or dashes (-).</p> </li> <li> <p>Contain less than 256 characters.</p> </li> </ul></p>
    pub dimension_name: String,
    /// <p>The place where Amazon SES finds the value of a dimension to publish to Amazon CloudWatch. If you want Amazon SES to use the message tags that you specify using an <code>X-SES-MESSAGE-TAGS</code> header or a parameter to the <code>SendEmail</code>/<code>SendRawEmail</code> API, choose <code>messageTag</code>. If you want Amazon SES to use your own email headers, choose <code>emailHeader</code>.</p>
    pub dimension_value_source: String,
}

struct CloudWatchDimensionConfigurationDeserializer;
impl CloudWatchDimensionConfigurationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CloudWatchDimensionConfiguration, XmlParseError> {
        deserialize_elements::<_, CloudWatchDimensionConfiguration, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "DefaultDimensionValue" => {
                        obj.default_dimension_value =
                            DefaultDimensionValueDeserializer::deserialize(
                                "DefaultDimensionValue",
                                stack,
                            )?;
                    }
                    "DimensionName" => {
                        obj.dimension_name =
                            DimensionNameDeserializer::deserialize("DimensionName", stack)?;
                    }
                    "DimensionValueSource" => {
                        obj.dimension_value_source = DimensionValueSourceDeserializer::deserialize(
                            "DimensionValueSource",
                            stack,
                        )?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

/// Serialize `CloudWatchDimensionConfiguration` contents to a `SignedRequest`.
struct CloudWatchDimensionConfigurationSerializer;
impl CloudWatchDimensionConfigurationSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CloudWatchDimensionConfiguration) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "DefaultDimensionValue"),
            &obj.default_dimension_value,
        );
        params.put(
            &format!("{}{}", prefix, "DimensionName"),
            &obj.dimension_name,
        );
        params.put(
            &format!("{}{}", prefix, "DimensionValueSource"),
            &obj.dimension_value_source,
        );
    }
}

struct CloudWatchDimensionConfigurationsDeserializer;
impl CloudWatchDimensionConfigurationsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<CloudWatchDimensionConfiguration>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(CloudWatchDimensionConfigurationDeserializer::deserialize(
                    "member", stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}

/// Serialize `CloudWatchDimensionConfigurations` contents to a `SignedRequest`.
struct CloudWatchDimensionConfigurationsSerializer;
impl CloudWatchDimensionConfigurationsSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<CloudWatchDimensionConfiguration>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            CloudWatchDimensionConfigurationSerializer::serialize(params, &key, obj);
        }
    }
}

/// <p>The name of the configuration set.</p> <p>Configuration sets let you create groups of rules that you can apply to the emails you send using Amazon SES. For more information about using configuration sets, see <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/using-configuration-sets.html">Using Amazon SES Configuration Sets</a> in the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ConfigurationSet {
    /// <p><p>The name of the configuration set. The name must meet the following requirements:</p> <ul> <li> <p>Contain only letters (a-z, A-Z), numbers (0-9), underscores (_), or dashes (-).</p> </li> <li> <p>Contain 64 characters or fewer.</p> </li> </ul></p>
    pub name: String,
}

struct ConfigurationSetDeserializer;
impl ConfigurationSetDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ConfigurationSet, XmlParseError> {
        deserialize_elements::<_, ConfigurationSet, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Name" => {
                    obj.name = ConfigurationSetNameDeserializer::deserialize("Name", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

/// Serialize `ConfigurationSet` contents to a `SignedRequest`.
struct ConfigurationSetSerializer;
impl ConfigurationSetSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ConfigurationSet) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "Name"), &obj.name);
    }
}

/// Serialize `ConfigurationSetAttributeList` contents to a `SignedRequest`.
struct ConfigurationSetAttributeListSerializer;
impl ConfigurationSetAttributeListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

struct ConfigurationSetNameDeserializer;
impl ConfigurationSetNameDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct ConfigurationSetsDeserializer;
impl ConfigurationSetsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<ConfigurationSet>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(ConfigurationSetDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
/// <p>Represents textual data, plus an optional character set specification.</p> <p>By default, the text must be 7-bit ASCII, due to the constraints of the SMTP protocol. If the text must contain any other characters, then you must also specify a character set. Examples include UTF-8, ISO-8859-1, and Shift_JIS.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Content {
    /// <p>The character set of the content.</p>
    pub charset: Option<String>,
    /// <p>The textual data of the content.</p>
    pub data: String,
}

/// Serialize `Content` contents to a `SignedRequest`.
struct ContentSerializer;
impl ContentSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Content) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.charset {
            params.put(&format!("{}{}", prefix, "Charset"), &field_value);
        }
        params.put(&format!("{}{}", prefix, "Data"), &obj.data);
    }
}

struct CounterDeserializer;
impl CounterDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = i64::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Represents a request to create a configuration set event destination. A configuration set event destination, which can be either Amazon CloudWatch or Amazon Kinesis Firehose, describes an AWS service in which Amazon SES publishes the email sending events associated with a configuration set. For information about using configuration sets, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/monitor-sending-activity.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateConfigurationSetEventDestinationRequest {
    /// <p>The name of the configuration set that the event destination should be associated with.</p>
    pub configuration_set_name: String,
    /// <p>An object that describes the AWS service that email sending event information will be published to.</p>
    pub event_destination: EventDestination,
}

/// Serialize `CreateConfigurationSetEventDestinationRequest` contents to a `SignedRequest`.
struct CreateConfigurationSetEventDestinationRequestSerializer;
impl CreateConfigurationSetEventDestinationRequestSerializer {
    fn serialize(
        params: &mut Params,
        name: &str,
        obj: &CreateConfigurationSetEventDestinationRequest,
    ) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "ConfigurationSetName"),
            &obj.configuration_set_name,
        );
        EventDestinationSerializer::serialize(
            params,
            &format!("{}{}", prefix, "EventDestination"),
            &obj.event_destination,
        );
    }
}

/// <p>An empty element returned on a successful request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CreateConfigurationSetEventDestinationResponse {}

struct CreateConfigurationSetEventDestinationResponseDeserializer;
impl CreateConfigurationSetEventDestinationResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateConfigurationSetEventDestinationResponse, XmlParseError> {
        start_element(tag_name, stack)?;

        let obj = CreateConfigurationSetEventDestinationResponse::default();

        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Represents a request to create a configuration set. Configuration sets enable you to publish email sending events. For information about using configuration sets, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/monitor-sending-activity.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateConfigurationSetRequest {
    /// <p>A data structure that contains the name of the configuration set.</p>
    pub configuration_set: ConfigurationSet,
}

/// Serialize `CreateConfigurationSetRequest` contents to a `SignedRequest`.
struct CreateConfigurationSetRequestSerializer;
impl CreateConfigurationSetRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CreateConfigurationSetRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        ConfigurationSetSerializer::serialize(
            params,
            &format!("{}{}", prefix, "ConfigurationSet"),
            &obj.configuration_set,
        );
    }
}

/// <p>An empty element returned on a successful request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CreateConfigurationSetResponse {}

struct CreateConfigurationSetResponseDeserializer;
impl CreateConfigurationSetResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateConfigurationSetResponse, XmlParseError> {
        start_element(tag_name, stack)?;

        let obj = CreateConfigurationSetResponse::default();

        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Represents a request to create an open and click tracking option object in a configuration set. </p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateConfigurationSetTrackingOptionsRequest {
    /// <p>The name of the configuration set that the tracking options should be associated with.</p>
    pub configuration_set_name: String,
    pub tracking_options: TrackingOptions,
}

/// Serialize `CreateConfigurationSetTrackingOptionsRequest` contents to a `SignedRequest`.
struct CreateConfigurationSetTrackingOptionsRequestSerializer;
impl CreateConfigurationSetTrackingOptionsRequestSerializer {
    fn serialize(
        params: &mut Params,
        name: &str,
        obj: &CreateConfigurationSetTrackingOptionsRequest,
    ) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "ConfigurationSetName"),
            &obj.configuration_set_name,
        );
        TrackingOptionsSerializer::serialize(
            params,
            &format!("{}{}", prefix, "TrackingOptions"),
            &obj.tracking_options,
        );
    }
}

/// <p>An empty element returned on a successful request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CreateConfigurationSetTrackingOptionsResponse {}

struct CreateConfigurationSetTrackingOptionsResponseDeserializer;
impl CreateConfigurationSetTrackingOptionsResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateConfigurationSetTrackingOptionsResponse, XmlParseError> {
        start_element(tag_name, stack)?;

        let obj = CreateConfigurationSetTrackingOptionsResponse::default();

        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Represents a request to create a custom verification email template.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateCustomVerificationEmailTemplateRequest {
    /// <p>The URL that the recipient of the verification email is sent to if his or her address is not successfully verified.</p>
    pub failure_redirection_url: String,
    /// <p>The email address that the custom verification email is sent from.</p>
    pub from_email_address: String,
    /// <p>The URL that the recipient of the verification email is sent to if his or her address is successfully verified.</p>
    pub success_redirection_url: String,
    /// <p>The content of the custom verification email. The total size of the email must be less than 10 MB. The message body may contain HTML, with some limitations. For more information, see <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/custom-verification-emails.html#custom-verification-emails-faq">Custom Verification Email Frequently Asked Questions</a> in the <i>Amazon SES Developer Guide</i>.</p>
    pub template_content: String,
    /// <p>The name of the custom verification email template.</p>
    pub template_name: String,
    /// <p>The subject line of the custom verification email.</p>
    pub template_subject: String,
}

/// Serialize `CreateCustomVerificationEmailTemplateRequest` contents to a `SignedRequest`.
struct CreateCustomVerificationEmailTemplateRequestSerializer;
impl CreateCustomVerificationEmailTemplateRequestSerializer {
    fn serialize(
        params: &mut Params,
        name: &str,
        obj: &CreateCustomVerificationEmailTemplateRequest,
    ) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "FailureRedirectionURL"),
            &obj.failure_redirection_url,
        );
        params.put(
            &format!("{}{}", prefix, "FromEmailAddress"),
            &obj.from_email_address,
        );
        params.put(
            &format!("{}{}", prefix, "SuccessRedirectionURL"),
            &obj.success_redirection_url,
        );
        params.put(
            &format!("{}{}", prefix, "TemplateContent"),
            &obj.template_content,
        );
        params.put(&format!("{}{}", prefix, "TemplateName"), &obj.template_name);
        params.put(
            &format!("{}{}", prefix, "TemplateSubject"),
            &obj.template_subject,
        );
    }
}

/// <p>Represents a request to create a new IP address filter. You use IP address filters when you receive email with Amazon SES. For more information, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-concepts.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateReceiptFilterRequest {
    /// <p>A data structure that describes the IP address filter to create, which consists of a name, an IP address range, and whether to allow or block mail from it.</p>
    pub filter: ReceiptFilter,
}

/// Serialize `CreateReceiptFilterRequest` contents to a `SignedRequest`.
struct CreateReceiptFilterRequestSerializer;
impl CreateReceiptFilterRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CreateReceiptFilterRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        ReceiptFilterSerializer::serialize(params, &format!("{}{}", prefix, "Filter"), &obj.filter);
    }
}

/// <p>An empty element returned on a successful request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CreateReceiptFilterResponse {}

struct CreateReceiptFilterResponseDeserializer;
impl CreateReceiptFilterResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateReceiptFilterResponse, XmlParseError> {
        start_element(tag_name, stack)?;

        let obj = CreateReceiptFilterResponse::default();

        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Represents a request to create a receipt rule. You use receipt rules to receive email with Amazon SES. For more information, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-concepts.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateReceiptRuleRequest {
    /// <p>The name of an existing rule after which the new rule will be placed. If this parameter is null, the new rule will be inserted at the beginning of the rule list.</p>
    pub after: Option<String>,
    /// <p>A data structure that contains the specified rule's name, actions, recipients, domains, enabled status, scan status, and TLS policy.</p>
    pub rule: ReceiptRule,
    /// <p>The name of the rule set that the receipt rule will be added to.</p>
    pub rule_set_name: String,
}

/// Serialize `CreateReceiptRuleRequest` contents to a `SignedRequest`.
struct CreateReceiptRuleRequestSerializer;
impl CreateReceiptRuleRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CreateReceiptRuleRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.after {
            params.put(&format!("{}{}", prefix, "After"), &field_value);
        }
        ReceiptRuleSerializer::serialize(params, &format!("{}{}", prefix, "Rule"), &obj.rule);
        params.put(&format!("{}{}", prefix, "RuleSetName"), &obj.rule_set_name);
    }
}

/// <p>An empty element returned on a successful request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CreateReceiptRuleResponse {}

struct CreateReceiptRuleResponseDeserializer;
impl CreateReceiptRuleResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateReceiptRuleResponse, XmlParseError> {
        start_element(tag_name, stack)?;

        let obj = CreateReceiptRuleResponse::default();

        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Represents a request to create an empty receipt rule set. You use receipt rule sets to receive email with Amazon SES. For more information, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-concepts.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateReceiptRuleSetRequest {
    /// <p><p>The name of the rule set to create. The name must:</p> <ul> <li> <p>This value can only contain ASCII letters (a-z, A-Z), numbers (0-9), underscores (_), or dashes (-).</p> </li> <li> <p>Start and end with a letter or number.</p> </li> <li> <p>Contain less than 64 characters.</p> </li> </ul></p>
    pub rule_set_name: String,
}

/// Serialize `CreateReceiptRuleSetRequest` contents to a `SignedRequest`.
struct CreateReceiptRuleSetRequestSerializer;
impl CreateReceiptRuleSetRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CreateReceiptRuleSetRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "RuleSetName"), &obj.rule_set_name);
    }
}

/// <p>An empty element returned on a successful request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CreateReceiptRuleSetResponse {}

struct CreateReceiptRuleSetResponseDeserializer;
impl CreateReceiptRuleSetResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateReceiptRuleSetResponse, XmlParseError> {
        start_element(tag_name, stack)?;

        let obj = CreateReceiptRuleSetResponse::default();

        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Represents a request to create an email template. For more information, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/send-personalized-email-api.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateTemplateRequest {
    /// <p>The content of the email, composed of a subject line, an HTML part, and a text-only part.</p>
    pub template: Template,
}

/// Serialize `CreateTemplateRequest` contents to a `SignedRequest`.
struct CreateTemplateRequestSerializer;
impl CreateTemplateRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CreateTemplateRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        TemplateSerializer::serialize(params, &format!("{}{}", prefix, "Template"), &obj.template);
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CreateTemplateResponse {}

struct CreateTemplateResponseDeserializer;
impl CreateTemplateResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateTemplateResponse, XmlParseError> {
        start_element(tag_name, stack)?;

        let obj = CreateTemplateResponse::default();

        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct CustomMailFromStatusDeserializer;
impl CustomMailFromStatusDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct CustomRedirectDomainDeserializer;
impl CustomRedirectDomainDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Contains information about a custom verification email template.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CustomVerificationEmailTemplate {
    /// <p>The URL that the recipient of the verification email is sent to if his or her address is not successfully verified.</p>
    pub failure_redirection_url: Option<String>,
    /// <p>The email address that the custom verification email is sent from.</p>
    pub from_email_address: Option<String>,
    /// <p>The URL that the recipient of the verification email is sent to if his or her address is successfully verified.</p>
    pub success_redirection_url: Option<String>,
    /// <p>The name of the custom verification email template.</p>
    pub template_name: Option<String>,
    /// <p>The subject line of the custom verification email.</p>
    pub template_subject: Option<String>,
}

struct CustomVerificationEmailTemplateDeserializer;
impl CustomVerificationEmailTemplateDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CustomVerificationEmailTemplate, XmlParseError> {
        deserialize_elements::<_, CustomVerificationEmailTemplate, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "FailureRedirectionURL" => {
                        obj.failure_redirection_url =
                            Some(FailureRedirectionURLDeserializer::deserialize(
                                "FailureRedirectionURL",
                                stack,
                            )?);
                    }
                    "FromEmailAddress" => {
                        obj.from_email_address = Some(FromAddressDeserializer::deserialize(
                            "FromEmailAddress",
                            stack,
                        )?);
                    }
                    "SuccessRedirectionURL" => {
                        obj.success_redirection_url =
                            Some(SuccessRedirectionURLDeserializer::deserialize(
                                "SuccessRedirectionURL",
                                stack,
                            )?);
                    }
                    "TemplateName" => {
                        obj.template_name = Some(TemplateNameDeserializer::deserialize(
                            "TemplateName",
                            stack,
                        )?);
                    }
                    "TemplateSubject" => {
                        obj.template_subject =
                            Some(SubjectDeserializer::deserialize("TemplateSubject", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
struct CustomVerificationEmailTemplatesDeserializer;
impl CustomVerificationEmailTemplatesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<CustomVerificationEmailTemplate>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(CustomVerificationEmailTemplateDeserializer::deserialize(
                    "member", stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
struct DefaultDimensionValueDeserializer;
impl DefaultDimensionValueDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Represents a request to delete a configuration set event destination. Configuration set event destinations are associated with configuration sets, which enable you to publish email sending events. For information about using configuration sets, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/monitor-sending-activity.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteConfigurationSetEventDestinationRequest {
    /// <p>The name of the configuration set from which to delete the event destination.</p>
    pub configuration_set_name: String,
    /// <p>The name of the event destination to delete.</p>
    pub event_destination_name: String,
}

/// Serialize `DeleteConfigurationSetEventDestinationRequest` contents to a `SignedRequest`.
struct DeleteConfigurationSetEventDestinationRequestSerializer;
impl DeleteConfigurationSetEventDestinationRequestSerializer {
    fn serialize(
        params: &mut Params,
        name: &str,
        obj: &DeleteConfigurationSetEventDestinationRequest,
    ) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "ConfigurationSetName"),
            &obj.configuration_set_name,
        );
        params.put(
            &format!("{}{}", prefix, "EventDestinationName"),
            &obj.event_destination_name,
        );
    }
}

/// <p>An empty element returned on a successful request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DeleteConfigurationSetEventDestinationResponse {}

struct DeleteConfigurationSetEventDestinationResponseDeserializer;
impl DeleteConfigurationSetEventDestinationResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteConfigurationSetEventDestinationResponse, XmlParseError> {
        start_element(tag_name, stack)?;

        let obj = DeleteConfigurationSetEventDestinationResponse::default();

        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Represents a request to delete a configuration set. Configuration sets enable you to publish email sending events. For information about using configuration sets, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/monitor-sending-activity.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteConfigurationSetRequest {
    /// <p>The name of the configuration set to delete.</p>
    pub configuration_set_name: String,
}

/// Serialize `DeleteConfigurationSetRequest` contents to a `SignedRequest`.
struct DeleteConfigurationSetRequestSerializer;
impl DeleteConfigurationSetRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DeleteConfigurationSetRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "ConfigurationSetName"),
            &obj.configuration_set_name,
        );
    }
}

/// <p>An empty element returned on a successful request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DeleteConfigurationSetResponse {}

struct DeleteConfigurationSetResponseDeserializer;
impl DeleteConfigurationSetResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteConfigurationSetResponse, XmlParseError> {
        start_element(tag_name, stack)?;

        let obj = DeleteConfigurationSetResponse::default();

        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Represents a request to delete open and click tracking options in a configuration set. </p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteConfigurationSetTrackingOptionsRequest {
    /// <p>The name of the configuration set from which you want to delete the tracking options.</p>
    pub configuration_set_name: String,
}

/// Serialize `DeleteConfigurationSetTrackingOptionsRequest` contents to a `SignedRequest`.
struct DeleteConfigurationSetTrackingOptionsRequestSerializer;
impl DeleteConfigurationSetTrackingOptionsRequestSerializer {
    fn serialize(
        params: &mut Params,
        name: &str,
        obj: &DeleteConfigurationSetTrackingOptionsRequest,
    ) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "ConfigurationSetName"),
            &obj.configuration_set_name,
        );
    }
}

/// <p>An empty element returned on a successful request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DeleteConfigurationSetTrackingOptionsResponse {}

struct DeleteConfigurationSetTrackingOptionsResponseDeserializer;
impl DeleteConfigurationSetTrackingOptionsResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteConfigurationSetTrackingOptionsResponse, XmlParseError> {
        start_element(tag_name, stack)?;

        let obj = DeleteConfigurationSetTrackingOptionsResponse::default();

        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Represents a request to delete an existing custom verification email template.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteCustomVerificationEmailTemplateRequest {
    /// <p>The name of the custom verification email template that you want to delete.</p>
    pub template_name: String,
}

/// Serialize `DeleteCustomVerificationEmailTemplateRequest` contents to a `SignedRequest`.
struct DeleteCustomVerificationEmailTemplateRequestSerializer;
impl DeleteCustomVerificationEmailTemplateRequestSerializer {
    fn serialize(
        params: &mut Params,
        name: &str,
        obj: &DeleteCustomVerificationEmailTemplateRequest,
    ) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "TemplateName"), &obj.template_name);
    }
}

/// <p>Represents a request to delete a sending authorization policy for an identity. Sending authorization is an Amazon SES feature that enables you to authorize other senders to use your identities. For information, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/sending-authorization.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteIdentityPolicyRequest {
    /// <p>The identity that is associated with the policy that you want to delete. You can specify the identity by using its name or by using its Amazon Resource Name (ARN). Examples: <code>user@example.com</code>, <code>example.com</code>, <code>arn:aws:ses:us-east-1:123456789012:identity/example.com</code>.</p> <p>To successfully call this API, you must own the identity.</p>
    pub identity: String,
    /// <p>The name of the policy to be deleted.</p>
    pub policy_name: String,
}

/// Serialize `DeleteIdentityPolicyRequest` contents to a `SignedRequest`.
struct DeleteIdentityPolicyRequestSerializer;
impl DeleteIdentityPolicyRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DeleteIdentityPolicyRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "Identity"), &obj.identity);
        params.put(&format!("{}{}", prefix, "PolicyName"), &obj.policy_name);
    }
}

/// <p>An empty element returned on a successful request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DeleteIdentityPolicyResponse {}

struct DeleteIdentityPolicyResponseDeserializer;
impl DeleteIdentityPolicyResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteIdentityPolicyResponse, XmlParseError> {
        start_element(tag_name, stack)?;

        let obj = DeleteIdentityPolicyResponse::default();

        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Represents a request to delete one of your Amazon SES identities (an email address or domain).</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteIdentityRequest {
    /// <p>The identity to be removed from the list of identities for the AWS Account.</p>
    pub identity: String,
}

/// Serialize `DeleteIdentityRequest` contents to a `SignedRequest`.
struct DeleteIdentityRequestSerializer;
impl DeleteIdentityRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DeleteIdentityRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "Identity"), &obj.identity);
    }
}

/// <p>An empty element returned on a successful request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DeleteIdentityResponse {}

struct DeleteIdentityResponseDeserializer;
impl DeleteIdentityResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteIdentityResponse, XmlParseError> {
        start_element(tag_name, stack)?;

        let obj = DeleteIdentityResponse::default();

        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Represents a request to delete an IP address filter. You use IP address filters when you receive email with Amazon SES. For more information, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-concepts.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteReceiptFilterRequest {
    /// <p>The name of the IP address filter to delete.</p>
    pub filter_name: String,
}

/// Serialize `DeleteReceiptFilterRequest` contents to a `SignedRequest`.
struct DeleteReceiptFilterRequestSerializer;
impl DeleteReceiptFilterRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DeleteReceiptFilterRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "FilterName"), &obj.filter_name);
    }
}

/// <p>An empty element returned on a successful request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DeleteReceiptFilterResponse {}

struct DeleteReceiptFilterResponseDeserializer;
impl DeleteReceiptFilterResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteReceiptFilterResponse, XmlParseError> {
        start_element(tag_name, stack)?;

        let obj = DeleteReceiptFilterResponse::default();

        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Represents a request to delete a receipt rule. You use receipt rules to receive email with Amazon SES. For more information, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-concepts.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteReceiptRuleRequest {
    /// <p>The name of the receipt rule to delete.</p>
    pub rule_name: String,
    /// <p>The name of the receipt rule set that contains the receipt rule to delete.</p>
    pub rule_set_name: String,
}

/// Serialize `DeleteReceiptRuleRequest` contents to a `SignedRequest`.
struct DeleteReceiptRuleRequestSerializer;
impl DeleteReceiptRuleRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DeleteReceiptRuleRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "RuleName"), &obj.rule_name);
        params.put(&format!("{}{}", prefix, "RuleSetName"), &obj.rule_set_name);
    }
}

/// <p>An empty element returned on a successful request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DeleteReceiptRuleResponse {}

struct DeleteReceiptRuleResponseDeserializer;
impl DeleteReceiptRuleResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteReceiptRuleResponse, XmlParseError> {
        start_element(tag_name, stack)?;

        let obj = DeleteReceiptRuleResponse::default();

        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Represents a request to delete a receipt rule set and all of the receipt rules it contains. You use receipt rule sets to receive email with Amazon SES. For more information, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-concepts.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteReceiptRuleSetRequest {
    /// <p>The name of the receipt rule set to delete.</p>
    pub rule_set_name: String,
}

/// Serialize `DeleteReceiptRuleSetRequest` contents to a `SignedRequest`.
struct DeleteReceiptRuleSetRequestSerializer;
impl DeleteReceiptRuleSetRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DeleteReceiptRuleSetRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "RuleSetName"), &obj.rule_set_name);
    }
}

/// <p>An empty element returned on a successful request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DeleteReceiptRuleSetResponse {}

struct DeleteReceiptRuleSetResponseDeserializer;
impl DeleteReceiptRuleSetResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteReceiptRuleSetResponse, XmlParseError> {
        start_element(tag_name, stack)?;

        let obj = DeleteReceiptRuleSetResponse::default();

        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Represents a request to delete an email template. For more information, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/send-personalized-email-api.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteTemplateRequest {
    /// <p>The name of the template to be deleted.</p>
    pub template_name: String,
}

/// Serialize `DeleteTemplateRequest` contents to a `SignedRequest`.
struct DeleteTemplateRequestSerializer;
impl DeleteTemplateRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DeleteTemplateRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "TemplateName"), &obj.template_name);
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DeleteTemplateResponse {}

struct DeleteTemplateResponseDeserializer;
impl DeleteTemplateResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteTemplateResponse, XmlParseError> {
        start_element(tag_name, stack)?;

        let obj = DeleteTemplateResponse::default();

        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Represents a request to delete an email address from the list of email addresses you have attempted to verify under your AWS account.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteVerifiedEmailAddressRequest {
    /// <p>An email address to be removed from the list of verified addresses.</p>
    pub email_address: String,
}

/// Serialize `DeleteVerifiedEmailAddressRequest` contents to a `SignedRequest`.
struct DeleteVerifiedEmailAddressRequestSerializer;
impl DeleteVerifiedEmailAddressRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DeleteVerifiedEmailAddressRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "EmailAddress"), &obj.email_address);
    }
}

/// <p>Specifies whether messages that use the configuration set are required to use Transport Layer Security (TLS).</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeliveryOptions {
    /// <p>Specifies whether messages that use the configuration set are required to use Transport Layer Security (TLS). If the value is <code>Require</code>, messages are only delivered if a TLS connection can be established. If the value is <code>Optional</code>, messages can be delivered in plain text if a TLS connection can't be established.</p>
    pub tls_policy: Option<String>,
}

struct DeliveryOptionsDeserializer;
impl DeliveryOptionsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeliveryOptions, XmlParseError> {
        deserialize_elements::<_, DeliveryOptions, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "TlsPolicy" => {
                    obj.tls_policy = Some(TlsPolicyDeserializer::deserialize("TlsPolicy", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

/// Serialize `DeliveryOptions` contents to a `SignedRequest`.
struct DeliveryOptionsSerializer;
impl DeliveryOptionsSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DeliveryOptions) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.tls_policy {
            params.put(&format!("{}{}", prefix, "TlsPolicy"), &field_value);
        }
    }
}

/// <p>Represents a request to return the metadata and receipt rules for the receipt rule set that is currently active. You use receipt rule sets to receive email with Amazon SES. For more information, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-concepts.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeActiveReceiptRuleSetRequest {}

/// Serialize `DescribeActiveReceiptRuleSetRequest` contents to a `SignedRequest`.
struct DescribeActiveReceiptRuleSetRequestSerializer;
impl DescribeActiveReceiptRuleSetRequestSerializer {
    fn serialize(_params: &mut Params, name: &str, _obj: &DescribeActiveReceiptRuleSetRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }
    }
}

/// <p>Represents the metadata and receipt rules for the receipt rule set that is currently active.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DescribeActiveReceiptRuleSetResponse {
    /// <p>The metadata for the currently active receipt rule set. The metadata consists of the rule set name and a timestamp of when the rule set was created.</p>
    pub metadata: Option<ReceiptRuleSetMetadata>,
    /// <p>The receipt rules that belong to the active rule set.</p>
    pub rules: Option<Vec<ReceiptRule>>,
}

struct DescribeActiveReceiptRuleSetResponseDeserializer;
impl DescribeActiveReceiptRuleSetResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeActiveReceiptRuleSetResponse, XmlParseError> {
        deserialize_elements::<_, DescribeActiveReceiptRuleSetResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Metadata" => {
                        obj.metadata = Some(ReceiptRuleSetMetadataDeserializer::deserialize(
                            "Metadata", stack,
                        )?);
                    }
                    "Rules" => {
                        obj.rules
                            .get_or_insert(vec![])
                            .extend(ReceiptRulesListDeserializer::deserialize("Rules", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p>Represents a request to return the details of a configuration set. Configuration sets enable you to publish email sending events. For information about using configuration sets, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/monitor-sending-activity.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeConfigurationSetRequest {
    /// <p>A list of configuration set attributes to return.</p>
    pub configuration_set_attribute_names: Option<Vec<String>>,
    /// <p>The name of the configuration set to describe.</p>
    pub configuration_set_name: String,
}

/// Serialize `DescribeConfigurationSetRequest` contents to a `SignedRequest`.
struct DescribeConfigurationSetRequestSerializer;
impl DescribeConfigurationSetRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeConfigurationSetRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.configuration_set_attribute_names {
            ConfigurationSetAttributeListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "ConfigurationSetAttributeNames"),
                field_value,
            );
        }
        params.put(
            &format!("{}{}", prefix, "ConfigurationSetName"),
            &obj.configuration_set_name,
        );
    }
}

/// <p>Represents the details of a configuration set. Configuration sets enable you to publish email sending events. For information about using configuration sets, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/monitor-sending-activity.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DescribeConfigurationSetResponse {
    /// <p>The configuration set object associated with the specified configuration set.</p>
    pub configuration_set: Option<ConfigurationSet>,
    pub delivery_options: Option<DeliveryOptions>,
    /// <p>A list of event destinations associated with the configuration set. </p>
    pub event_destinations: Option<Vec<EventDestination>>,
    /// <p>An object that represents the reputation settings for the configuration set. </p>
    pub reputation_options: Option<ReputationOptions>,
    /// <p>The name of the custom open and click tracking domain associated with the configuration set.</p>
    pub tracking_options: Option<TrackingOptions>,
}

struct DescribeConfigurationSetResponseDeserializer;
impl DescribeConfigurationSetResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeConfigurationSetResponse, XmlParseError> {
        deserialize_elements::<_, DescribeConfigurationSetResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "ConfigurationSet" => {
                        obj.configuration_set = Some(ConfigurationSetDeserializer::deserialize(
                            "ConfigurationSet",
                            stack,
                        )?);
                    }
                    "DeliveryOptions" => {
                        obj.delivery_options = Some(DeliveryOptionsDeserializer::deserialize(
                            "DeliveryOptions",
                            stack,
                        )?);
                    }
                    "EventDestinations" => {
                        obj.event_destinations.get_or_insert(vec![]).extend(
                            EventDestinationsDeserializer::deserialize("EventDestinations", stack)?,
                        );
                    }
                    "ReputationOptions" => {
                        obj.reputation_options = Some(ReputationOptionsDeserializer::deserialize(
                            "ReputationOptions",
                            stack,
                        )?);
                    }
                    "TrackingOptions" => {
                        obj.tracking_options = Some(TrackingOptionsDeserializer::deserialize(
                            "TrackingOptions",
                            stack,
                        )?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p>Represents a request to return the details of a receipt rule. You use receipt rules to receive email with Amazon SES. For more information, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-concepts.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeReceiptRuleRequest {
    /// <p>The name of the receipt rule.</p>
    pub rule_name: String,
    /// <p>The name of the receipt rule set that the receipt rule belongs to.</p>
    pub rule_set_name: String,
}

/// Serialize `DescribeReceiptRuleRequest` contents to a `SignedRequest`.
struct DescribeReceiptRuleRequestSerializer;
impl DescribeReceiptRuleRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeReceiptRuleRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "RuleName"), &obj.rule_name);
        params.put(&format!("{}{}", prefix, "RuleSetName"), &obj.rule_set_name);
    }
}

/// <p>Represents the details of a receipt rule.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DescribeReceiptRuleResponse {
    /// <p>A data structure that contains the specified receipt rule's name, actions, recipients, domains, enabled status, scan status, and Transport Layer Security (TLS) policy.</p>
    pub rule: Option<ReceiptRule>,
}

struct DescribeReceiptRuleResponseDeserializer;
impl DescribeReceiptRuleResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeReceiptRuleResponse, XmlParseError> {
        deserialize_elements::<_, DescribeReceiptRuleResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Rule" => {
                        obj.rule = Some(ReceiptRuleDeserializer::deserialize("Rule", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p>Represents a request to return the details of a receipt rule set. You use receipt rule sets to receive email with Amazon SES. For more information, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-concepts.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeReceiptRuleSetRequest {
    /// <p>The name of the receipt rule set to describe.</p>
    pub rule_set_name: String,
}

/// Serialize `DescribeReceiptRuleSetRequest` contents to a `SignedRequest`.
struct DescribeReceiptRuleSetRequestSerializer;
impl DescribeReceiptRuleSetRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeReceiptRuleSetRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "RuleSetName"), &obj.rule_set_name);
    }
}

/// <p>Represents the details of the specified receipt rule set.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DescribeReceiptRuleSetResponse {
    /// <p>The metadata for the receipt rule set, which consists of the rule set name and the timestamp of when the rule set was created.</p>
    pub metadata: Option<ReceiptRuleSetMetadata>,
    /// <p>A list of the receipt rules that belong to the specified receipt rule set.</p>
    pub rules: Option<Vec<ReceiptRule>>,
}

struct DescribeReceiptRuleSetResponseDeserializer;
impl DescribeReceiptRuleSetResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeReceiptRuleSetResponse, XmlParseError> {
        deserialize_elements::<_, DescribeReceiptRuleSetResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Metadata" => {
                        obj.metadata = Some(ReceiptRuleSetMetadataDeserializer::deserialize(
                            "Metadata", stack,
                        )?);
                    }
                    "Rules" => {
                        obj.rules
                            .get_or_insert(vec![])
                            .extend(ReceiptRulesListDeserializer::deserialize("Rules", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p><p>Represents the destination of the message, consisting of To:, CC:, and BCC: fields.</p> <note> <p>Amazon SES does not support the SMTPUTF8 extension, as described in <a href="https://tools.ietf.org/html/rfc6531">RFC6531</a>. For this reason, the <i>local part</i> of a destination email address (the part of the email address that precedes the @ sign) may only contain <a href="https://en.wikipedia.org/wiki/Email_address#Local-part">7-bit ASCII characters</a>. If the <i>domain part</i> of an address (the part after the @ sign) contains non-ASCII characters, they must be encoded using Punycode, as described in <a href="https://tools.ietf.org/html/rfc3492.html">RFC3492</a>.</p> </note></p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Destination {
    /// <p>The recipients to place on the BCC: line of the message.</p>
    pub bcc_addresses: Option<Vec<String>>,
    /// <p>The recipients to place on the CC: line of the message.</p>
    pub cc_addresses: Option<Vec<String>>,
    /// <p>The recipients to place on the To: line of the message.</p>
    pub to_addresses: Option<Vec<String>>,
}

/// Serialize `Destination` contents to a `SignedRequest`.
struct DestinationSerializer;
impl DestinationSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Destination) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.bcc_addresses {
            AddressListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "BccAddresses"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.cc_addresses {
            AddressListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "CcAddresses"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.to_addresses {
            AddressListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "ToAddresses"),
                field_value,
            );
        }
    }
}

struct DimensionNameDeserializer;
impl DimensionNameDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct DimensionValueSourceDeserializer;
impl DimensionValueSourceDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct DkimAttributesDeserializer;
impl DkimAttributesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<::std::collections::HashMap<String, IdentityDkimAttributes>, XmlParseError> {
        start_element(tag_name, stack)?;

        let mut obj = ::std::collections::HashMap::new();

        while peek_at_name(stack)? == "entry" {
            start_element("entry", stack)?;
            let key = IdentityDeserializer::deserialize("key", stack)?;
            let value = IdentityDkimAttributesDeserializer::deserialize("value", stack)?;
            obj.insert(key, value);
            end_element("entry", stack)?;
        }

        end_element(tag_name, stack)?;
        Ok(obj)
    }
}
struct EnabledDeserializer;
impl EnabledDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<bool, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = bool::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct SesErrorDeserializer;
impl SesErrorDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Contains information about the event destination that the specified email sending events will be published to.</p> <note> <p>When you create or update an event destination, you must provide one, and only one, destination. The destination can be Amazon CloudWatch, Amazon Kinesis Firehose or Amazon Simple Notification Service (Amazon SNS).</p> </note> <p>Event destinations are associated with configuration sets, which enable you to publish email sending events to Amazon CloudWatch, Amazon Kinesis Firehose, or Amazon Simple Notification Service (Amazon SNS). For information about using configuration sets, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/monitor-sending-activity.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct EventDestination {
    /// <p>An object that contains the names, default values, and sources of the dimensions associated with an Amazon CloudWatch event destination.</p>
    pub cloud_watch_destination: Option<CloudWatchDestination>,
    /// <p>Sets whether Amazon SES publishes events to this destination when you send an email with the associated configuration set. Set to <code>true</code> to enable publishing to this destination; set to <code>false</code> to prevent publishing to this destination. The default value is <code>false</code>.</p>
    pub enabled: Option<bool>,
    /// <p>An object that contains the delivery stream ARN and the IAM role ARN associated with an Amazon Kinesis Firehose event destination.</p>
    pub kinesis_firehose_destination: Option<KinesisFirehoseDestination>,
    /// <p>The type of email sending events to publish to the event destination.</p>
    pub matching_event_types: Vec<String>,
    /// <p><p>The name of the event destination. The name must:</p> <ul> <li> <p>This value can only contain ASCII letters (a-z, A-Z), numbers (0-9), underscores (_), or dashes (-).</p> </li> <li> <p>Contain less than 64 characters.</p> </li> </ul></p>
    pub name: String,
    /// <p>An object that contains the topic ARN associated with an Amazon Simple Notification Service (Amazon SNS) event destination.</p>
    pub sns_destination: Option<SNSDestination>,
}

struct EventDestinationDeserializer;
impl EventDestinationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<EventDestination, XmlParseError> {
        deserialize_elements::<_, EventDestination, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "CloudWatchDestination" => {
                    obj.cloud_watch_destination =
                        Some(CloudWatchDestinationDeserializer::deserialize(
                            "CloudWatchDestination",
                            stack,
                        )?);
                }
                "Enabled" => {
                    obj.enabled = Some(EnabledDeserializer::deserialize("Enabled", stack)?);
                }
                "KinesisFirehoseDestination" => {
                    obj.kinesis_firehose_destination =
                        Some(KinesisFirehoseDestinationDeserializer::deserialize(
                            "KinesisFirehoseDestination",
                            stack,
                        )?);
                }
                "MatchingEventTypes" => {
                    obj.matching_event_types
                        .extend(EventTypesDeserializer::deserialize(
                            "MatchingEventTypes",
                            stack,
                        )?);
                }
                "Name" => {
                    obj.name = EventDestinationNameDeserializer::deserialize("Name", stack)?;
                }
                "SNSDestination" => {
                    obj.sns_destination = Some(SNSDestinationDeserializer::deserialize(
                        "SNSDestination",
                        stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

/// Serialize `EventDestination` contents to a `SignedRequest`.
struct EventDestinationSerializer;
impl EventDestinationSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &EventDestination) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.cloud_watch_destination {
            CloudWatchDestinationSerializer::serialize(
                params,
                &format!("{}{}", prefix, "CloudWatchDestination"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.enabled {
            params.put(&format!("{}{}", prefix, "Enabled"), &field_value);
        }
        if let Some(ref field_value) = obj.kinesis_firehose_destination {
            KinesisFirehoseDestinationSerializer::serialize(
                params,
                &format!("{}{}", prefix, "KinesisFirehoseDestination"),
                field_value,
            );
        }
        EventTypesSerializer::serialize(
            params,
            &format!("{}{}", prefix, "MatchingEventTypes"),
            &obj.matching_event_types,
        );
        params.put(&format!("{}{}", prefix, "Name"), &obj.name);
        if let Some(ref field_value) = obj.sns_destination {
            SNSDestinationSerializer::serialize(
                params,
                &format!("{}{}", prefix, "SNSDestination"),
                field_value,
            );
        }
    }
}

struct EventDestinationNameDeserializer;
impl EventDestinationNameDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct EventDestinationsDeserializer;
impl EventDestinationsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<EventDestination>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(EventDestinationDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
struct EventTypeDeserializer;
impl EventTypeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct EventTypesDeserializer;
impl EventTypesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(EventTypeDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}

/// Serialize `EventTypes` contents to a `SignedRequest`.
struct EventTypesSerializer;
impl EventTypesSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

/// <p>Additional X-headers to include in the Delivery Status Notification (DSN) when an email that Amazon SES receives on your behalf bounces.</p> <p>For information about receiving email through Amazon SES, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ExtensionField {
    /// <p>The name of the header to add. Must be between 1 and 50 characters, inclusive, and consist of alphanumeric (a-z, A-Z, 0-9) characters and dashes only.</p>
    pub name: String,
    /// <p>The value of the header to add. Must be less than 2048 characters, and must not contain newline characters ("\r" or "\n").</p>
    pub value: String,
}

/// Serialize `ExtensionField` contents to a `SignedRequest`.
struct ExtensionFieldSerializer;
impl ExtensionFieldSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ExtensionField) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "Name"), &obj.name);
        params.put(&format!("{}{}", prefix, "Value"), &obj.value);
    }
}

/// Serialize `ExtensionFieldList` contents to a `SignedRequest`.
struct ExtensionFieldListSerializer;
impl ExtensionFieldListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<ExtensionField>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            ExtensionFieldSerializer::serialize(params, &key, obj);
        }
    }
}

struct FailureRedirectionURLDeserializer;
impl FailureRedirectionURLDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct FromAddressDeserializer;
impl FromAddressDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Represents a request to return the email sending status for your Amazon SES account in the current AWS Region.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct GetAccountSendingEnabledResponse {
    /// <p>Describes whether email sending is enabled or disabled for your Amazon SES account in the current AWS Region.</p>
    pub enabled: Option<bool>,
}

struct GetAccountSendingEnabledResponseDeserializer;
impl GetAccountSendingEnabledResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetAccountSendingEnabledResponse, XmlParseError> {
        deserialize_elements::<_, GetAccountSendingEnabledResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Enabled" => {
                        obj.enabled = Some(EnabledDeserializer::deserialize("Enabled", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p>Represents a request to retrieve an existing custom verification email template.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetCustomVerificationEmailTemplateRequest {
    /// <p>The name of the custom verification email template that you want to retrieve.</p>
    pub template_name: String,
}

/// Serialize `GetCustomVerificationEmailTemplateRequest` contents to a `SignedRequest`.
struct GetCustomVerificationEmailTemplateRequestSerializer;
impl GetCustomVerificationEmailTemplateRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &GetCustomVerificationEmailTemplateRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "TemplateName"), &obj.template_name);
    }
}

/// <p>The content of the custom verification email template.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct GetCustomVerificationEmailTemplateResponse {
    /// <p>The URL that the recipient of the verification email is sent to if his or her address is not successfully verified.</p>
    pub failure_redirection_url: Option<String>,
    /// <p>The email address that the custom verification email is sent from.</p>
    pub from_email_address: Option<String>,
    /// <p>The URL that the recipient of the verification email is sent to if his or her address is successfully verified.</p>
    pub success_redirection_url: Option<String>,
    /// <p>The content of the custom verification email.</p>
    pub template_content: Option<String>,
    /// <p>The name of the custom verification email template.</p>
    pub template_name: Option<String>,
    /// <p>The subject line of the custom verification email.</p>
    pub template_subject: Option<String>,
}

struct GetCustomVerificationEmailTemplateResponseDeserializer;
impl GetCustomVerificationEmailTemplateResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetCustomVerificationEmailTemplateResponse, XmlParseError> {
        deserialize_elements::<_, GetCustomVerificationEmailTemplateResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "FailureRedirectionURL" => {
                        obj.failure_redirection_url =
                            Some(FailureRedirectionURLDeserializer::deserialize(
                                "FailureRedirectionURL",
                                stack,
                            )?);
                    }
                    "FromEmailAddress" => {
                        obj.from_email_address = Some(FromAddressDeserializer::deserialize(
                            "FromEmailAddress",
                            stack,
                        )?);
                    }
                    "SuccessRedirectionURL" => {
                        obj.success_redirection_url =
                            Some(SuccessRedirectionURLDeserializer::deserialize(
                                "SuccessRedirectionURL",
                                stack,
                            )?);
                    }
                    "TemplateContent" => {
                        obj.template_content = Some(TemplateContentDeserializer::deserialize(
                            "TemplateContent",
                            stack,
                        )?);
                    }
                    "TemplateName" => {
                        obj.template_name = Some(TemplateNameDeserializer::deserialize(
                            "TemplateName",
                            stack,
                        )?);
                    }
                    "TemplateSubject" => {
                        obj.template_subject =
                            Some(SubjectDeserializer::deserialize("TemplateSubject", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p>Represents a request for the status of Amazon SES Easy DKIM signing for an identity. For domain identities, this request also returns the DKIM tokens that are required for Easy DKIM signing, and whether Amazon SES successfully verified that these tokens were published. For more information about Easy DKIM, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/easy-dkim.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetIdentityDkimAttributesRequest {
    /// <p>A list of one or more verified identities - email addresses, domains, or both.</p>
    pub identities: Vec<String>,
}

/// Serialize `GetIdentityDkimAttributesRequest` contents to a `SignedRequest`.
struct GetIdentityDkimAttributesRequestSerializer;
impl GetIdentityDkimAttributesRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &GetIdentityDkimAttributesRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        IdentityListSerializer::serialize(
            params,
            &format!("{}{}", prefix, "Identities"),
            &obj.identities,
        );
    }
}

/// <p>Represents the status of Amazon SES Easy DKIM signing for an identity. For domain identities, this response also contains the DKIM tokens that are required for Easy DKIM signing, and whether Amazon SES successfully verified that these tokens were published.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct GetIdentityDkimAttributesResponse {
    /// <p>The DKIM attributes for an email address or a domain.</p>
    pub dkim_attributes: ::std::collections::HashMap<String, IdentityDkimAttributes>,
}

struct GetIdentityDkimAttributesResponseDeserializer;
impl GetIdentityDkimAttributesResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetIdentityDkimAttributesResponse, XmlParseError> {
        deserialize_elements::<_, GetIdentityDkimAttributesResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "DkimAttributes" => {
                        obj.dkim_attributes =
                            DkimAttributesDeserializer::deserialize("DkimAttributes", stack)?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p>Represents a request to return the Amazon SES custom MAIL FROM attributes for a list of identities. For information about using a custom MAIL FROM domain, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/mail-from.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetIdentityMailFromDomainAttributesRequest {
    /// <p>A list of one or more identities.</p>
    pub identities: Vec<String>,
}

/// Serialize `GetIdentityMailFromDomainAttributesRequest` contents to a `SignedRequest`.
struct GetIdentityMailFromDomainAttributesRequestSerializer;
impl GetIdentityMailFromDomainAttributesRequestSerializer {
    fn serialize(
        params: &mut Params,
        name: &str,
        obj: &GetIdentityMailFromDomainAttributesRequest,
    ) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        IdentityListSerializer::serialize(
            params,
            &format!("{}{}", prefix, "Identities"),
            &obj.identities,
        );
    }
}

/// <p>Represents the custom MAIL FROM attributes for a list of identities.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct GetIdentityMailFromDomainAttributesResponse {
    /// <p>A map of identities to custom MAIL FROM attributes.</p>
    pub mail_from_domain_attributes:
        ::std::collections::HashMap<String, IdentityMailFromDomainAttributes>,
}

struct GetIdentityMailFromDomainAttributesResponseDeserializer;
impl GetIdentityMailFromDomainAttributesResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetIdentityMailFromDomainAttributesResponse, XmlParseError> {
        deserialize_elements::<_, GetIdentityMailFromDomainAttributesResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "MailFromDomainAttributes" => {
                        obj.mail_from_domain_attributes =
                            MailFromDomainAttributesDeserializer::deserialize(
                                "MailFromDomainAttributes",
                                stack,
                            )?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p>Represents a request to return the notification attributes for a list of identities you verified with Amazon SES. For information about Amazon SES notifications, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/notifications.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetIdentityNotificationAttributesRequest {
    /// <p>A list of one or more identities. You can specify an identity by using its name or by using its Amazon Resource Name (ARN). Examples: <code>user@example.com</code>, <code>example.com</code>, <code>arn:aws:ses:us-east-1:123456789012:identity/example.com</code>.</p>
    pub identities: Vec<String>,
}

/// Serialize `GetIdentityNotificationAttributesRequest` contents to a `SignedRequest`.
struct GetIdentityNotificationAttributesRequestSerializer;
impl GetIdentityNotificationAttributesRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &GetIdentityNotificationAttributesRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        IdentityListSerializer::serialize(
            params,
            &format!("{}{}", prefix, "Identities"),
            &obj.identities,
        );
    }
}

/// <p>Represents the notification attributes for a list of identities.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct GetIdentityNotificationAttributesResponse {
    /// <p>A map of Identity to IdentityNotificationAttributes.</p>
    pub notification_attributes:
        ::std::collections::HashMap<String, IdentityNotificationAttributes>,
}

struct GetIdentityNotificationAttributesResponseDeserializer;
impl GetIdentityNotificationAttributesResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetIdentityNotificationAttributesResponse, XmlParseError> {
        deserialize_elements::<_, GetIdentityNotificationAttributesResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "NotificationAttributes" => {
                        obj.notification_attributes =
                            NotificationAttributesDeserializer::deserialize(
                                "NotificationAttributes",
                                stack,
                            )?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p>Represents a request to return the requested sending authorization policies for an identity. Sending authorization is an Amazon SES feature that enables you to authorize other senders to use your identities. For information, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/sending-authorization.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetIdentityPoliciesRequest {
    /// <p>The identity for which the policies will be retrieved. You can specify an identity by using its name or by using its Amazon Resource Name (ARN). Examples: <code>user@example.com</code>, <code>example.com</code>, <code>arn:aws:ses:us-east-1:123456789012:identity/example.com</code>.</p> <p>To successfully call this API, you must own the identity.</p>
    pub identity: String,
    /// <p>A list of the names of policies to be retrieved. You can retrieve a maximum of 20 policies at a time. If you do not know the names of the policies that are attached to the identity, you can use <code>ListIdentityPolicies</code>.</p>
    pub policy_names: Vec<String>,
}

/// Serialize `GetIdentityPoliciesRequest` contents to a `SignedRequest`.
struct GetIdentityPoliciesRequestSerializer;
impl GetIdentityPoliciesRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &GetIdentityPoliciesRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "Identity"), &obj.identity);
        PolicyNameListSerializer::serialize(
            params,
            &format!("{}{}", prefix, "PolicyNames"),
            &obj.policy_names,
        );
    }
}

/// <p>Represents the requested sending authorization policies.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct GetIdentityPoliciesResponse {
    /// <p>A map of policy names to policies.</p>
    pub policies: ::std::collections::HashMap<String, String>,
}

struct GetIdentityPoliciesResponseDeserializer;
impl GetIdentityPoliciesResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetIdentityPoliciesResponse, XmlParseError> {
        deserialize_elements::<_, GetIdentityPoliciesResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Policies" => {
                        obj.policies = PolicyMapDeserializer::deserialize("Policies", stack)?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p>Represents a request to return the Amazon SES verification status of a list of identities. For domain identities, this request also returns the verification token. For information about verifying identities with Amazon SES, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/verify-addresses-and-domains.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetIdentityVerificationAttributesRequest {
    /// <p>A list of identities.</p>
    pub identities: Vec<String>,
}

/// Serialize `GetIdentityVerificationAttributesRequest` contents to a `SignedRequest`.
struct GetIdentityVerificationAttributesRequestSerializer;
impl GetIdentityVerificationAttributesRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &GetIdentityVerificationAttributesRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        IdentityListSerializer::serialize(
            params,
            &format!("{}{}", prefix, "Identities"),
            &obj.identities,
        );
    }
}

/// <p>The Amazon SES verification status of a list of identities. For domain identities, this response also contains the verification token.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct GetIdentityVerificationAttributesResponse {
    /// <p>A map of Identities to IdentityVerificationAttributes objects.</p>
    pub verification_attributes:
        ::std::collections::HashMap<String, IdentityVerificationAttributes>,
}

struct GetIdentityVerificationAttributesResponseDeserializer;
impl GetIdentityVerificationAttributesResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetIdentityVerificationAttributesResponse, XmlParseError> {
        deserialize_elements::<_, GetIdentityVerificationAttributesResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "VerificationAttributes" => {
                        obj.verification_attributes =
                            VerificationAttributesDeserializer::deserialize(
                                "VerificationAttributes",
                                stack,
                            )?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p>Represents your Amazon SES daily sending quota, maximum send rate, and the number of emails you have sent in the last 24 hours.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct GetSendQuotaResponse {
    /// <p>The maximum number of emails the user is allowed to send in a 24-hour interval. A value of -1 signifies an unlimited quota.</p>
    pub max_24_hour_send: Option<f64>,
    /// <p><p>The maximum number of emails that Amazon SES can accept from the user&#39;s account per second.</p> <note> <p>The rate at which Amazon SES accepts the user&#39;s messages might be less than the maximum send rate.</p> </note></p>
    pub max_send_rate: Option<f64>,
    /// <p>The number of emails sent during the previous 24 hours.</p>
    pub sent_last_24_hours: Option<f64>,
}

struct GetSendQuotaResponseDeserializer;
impl GetSendQuotaResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetSendQuotaResponse, XmlParseError> {
        deserialize_elements::<_, GetSendQuotaResponse, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Max24HourSend" => {
                    obj.max_24_hour_send = Some(Max24HourSendDeserializer::deserialize(
                        "Max24HourSend",
                        stack,
                    )?);
                }
                "MaxSendRate" => {
                    obj.max_send_rate =
                        Some(MaxSendRateDeserializer::deserialize("MaxSendRate", stack)?);
                }
                "SentLast24Hours" => {
                    obj.sent_last_24_hours = Some(SentLast24HoursDeserializer::deserialize(
                        "SentLast24Hours",
                        stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
/// <p>Represents a list of data points. This list contains aggregated data from the previous two weeks of your sending activity with Amazon SES.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct GetSendStatisticsResponse {
    /// <p>A list of data points, each of which represents 15 minutes of activity.</p>
    pub send_data_points: Option<Vec<SendDataPoint>>,
}

struct GetSendStatisticsResponseDeserializer;
impl GetSendStatisticsResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetSendStatisticsResponse, XmlParseError> {
        deserialize_elements::<_, GetSendStatisticsResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "SendDataPoints" => {
                        obj.send_data_points.get_or_insert(vec![]).extend(
                            SendDataPointListDeserializer::deserialize("SendDataPoints", stack)?,
                        );
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetTemplateRequest {
    /// <p>The name of the template you want to retrieve.</p>
    pub template_name: String,
}

/// Serialize `GetTemplateRequest` contents to a `SignedRequest`.
struct GetTemplateRequestSerializer;
impl GetTemplateRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &GetTemplateRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "TemplateName"), &obj.template_name);
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct GetTemplateResponse {
    pub template: Option<Template>,
}

struct GetTemplateResponseDeserializer;
impl GetTemplateResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetTemplateResponse, XmlParseError> {
        deserialize_elements::<_, GetTemplateResponse, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Template" => {
                    obj.template = Some(TemplateDeserializer::deserialize("Template", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct HeaderNameDeserializer;
impl HeaderNameDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct HeaderValueDeserializer;
impl HeaderValueDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct HtmlPartDeserializer;
impl HtmlPartDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct IdentityDeserializer;
impl IdentityDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Represents the DKIM attributes of a verified email address or a domain.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct IdentityDkimAttributes {
    /// <p>Is true if DKIM signing is enabled for email sent from the identity. It's false otherwise. The default value is true.</p>
    pub dkim_enabled: bool,
    /// <p>A set of character strings that represent the domain's identity. Using these tokens, you need to create DNS CNAME records that point to DKIM public keys that are hosted by Amazon SES. Amazon Web Services eventually detects that you've updated your DNS records. This detection process might take up to 72 hours. After successful detection, Amazon SES is able to DKIM-sign email originating from that domain. (This only applies to domain identities, not email address identities.)</p> <p>For more information about creating DNS records using DKIM tokens, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/easy-dkim.html">Amazon SES Developer Guide</a>.</p>
    pub dkim_tokens: Option<Vec<String>>,
    /// <p>Describes whether Amazon SES has successfully verified the DKIM DNS records (tokens) published in the domain name's DNS. (This only applies to domain identities, not email address identities.)</p>
    pub dkim_verification_status: String,
}

struct IdentityDkimAttributesDeserializer;
impl IdentityDkimAttributesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<IdentityDkimAttributes, XmlParseError> {
        deserialize_elements::<_, IdentityDkimAttributes, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "DkimEnabled" => {
                    obj.dkim_enabled = EnabledDeserializer::deserialize("DkimEnabled", stack)?;
                }
                "DkimTokens" => {
                    obj.dkim_tokens.get_or_insert(vec![]).extend(
                        VerificationTokenListDeserializer::deserialize("DkimTokens", stack)?,
                    );
                }
                "DkimVerificationStatus" => {
                    obj.dkim_verification_status = VerificationStatusDeserializer::deserialize(
                        "DkimVerificationStatus",
                        stack,
                    )?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct IdentityListDeserializer;
impl IdentityListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(IdentityDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}

/// Serialize `IdentityList` contents to a `SignedRequest`.
struct IdentityListSerializer;
impl IdentityListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

/// <p>Represents the custom MAIL FROM domain attributes of a verified identity (email address or domain).</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct IdentityMailFromDomainAttributes {
    /// <p>The action that Amazon SES takes if it cannot successfully read the required MX record when you send an email. A value of <code>UseDefaultValue</code> indicates that if Amazon SES cannot read the required MX record, it uses amazonses.com (or a subdomain of that) as the MAIL FROM domain. A value of <code>RejectMessage</code> indicates that if Amazon SES cannot read the required MX record, Amazon SES returns a <code>MailFromDomainNotVerified</code> error and does not send the email.</p> <p>The custom MAIL FROM setup states that result in this behavior are <code>Pending</code>, <code>Failed</code>, and <code>TemporaryFailure</code>.</p>
    pub behavior_on_mx_failure: String,
    /// <p>The custom MAIL FROM domain that the identity is configured to use.</p>
    pub mail_from_domain: String,
    /// <p>The state that indicates whether Amazon SES has successfully read the MX record required for custom MAIL FROM domain setup. If the state is <code>Success</code>, Amazon SES uses the specified custom MAIL FROM domain when the verified identity sends an email. All other states indicate that Amazon SES takes the action described by <code>BehaviorOnMXFailure</code>.</p>
    pub mail_from_domain_status: String,
}

struct IdentityMailFromDomainAttributesDeserializer;
impl IdentityMailFromDomainAttributesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<IdentityMailFromDomainAttributes, XmlParseError> {
        deserialize_elements::<_, IdentityMailFromDomainAttributes, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "BehaviorOnMXFailure" => {
                        obj.behavior_on_mx_failure = BehaviorOnMXFailureDeserializer::deserialize(
                            "BehaviorOnMXFailure",
                            stack,
                        )?;
                    }
                    "MailFromDomain" => {
                        obj.mail_from_domain =
                            MailFromDomainNameDeserializer::deserialize("MailFromDomain", stack)?;
                    }
                    "MailFromDomainStatus" => {
                        obj.mail_from_domain_status =
                            CustomMailFromStatusDeserializer::deserialize(
                                "MailFromDomainStatus",
                                stack,
                            )?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p>Represents the notification attributes of an identity, including whether an identity has Amazon Simple Notification Service (Amazon SNS) topics set for bounce, complaint, and/or delivery notifications, and whether feedback forwarding is enabled for bounce and complaint notifications.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct IdentityNotificationAttributes {
    /// <p>The Amazon Resource Name (ARN) of the Amazon SNS topic where Amazon SES will publish bounce notifications.</p>
    pub bounce_topic: String,
    /// <p>The Amazon Resource Name (ARN) of the Amazon SNS topic where Amazon SES will publish complaint notifications.</p>
    pub complaint_topic: String,
    /// <p>The Amazon Resource Name (ARN) of the Amazon SNS topic where Amazon SES will publish delivery notifications.</p>
    pub delivery_topic: String,
    /// <p>Describes whether Amazon SES will forward bounce and complaint notifications as email. <code>true</code> indicates that Amazon SES will forward bounce and complaint notifications as email, while <code>false</code> indicates that bounce and complaint notifications will be published only to the specified bounce and complaint Amazon SNS topics.</p>
    pub forwarding_enabled: bool,
    /// <p>Describes whether Amazon SES includes the original email headers in Amazon SNS notifications of type <code>Bounce</code>. A value of <code>true</code> specifies that Amazon SES will include headers in bounce notifications, and a value of <code>false</code> specifies that Amazon SES will not include headers in bounce notifications.</p>
    pub headers_in_bounce_notifications_enabled: Option<bool>,
    /// <p>Describes whether Amazon SES includes the original email headers in Amazon SNS notifications of type <code>Complaint</code>. A value of <code>true</code> specifies that Amazon SES will include headers in complaint notifications, and a value of <code>false</code> specifies that Amazon SES will not include headers in complaint notifications.</p>
    pub headers_in_complaint_notifications_enabled: Option<bool>,
    /// <p>Describes whether Amazon SES includes the original email headers in Amazon SNS notifications of type <code>Delivery</code>. A value of <code>true</code> specifies that Amazon SES will include headers in delivery notifications, and a value of <code>false</code> specifies that Amazon SES will not include headers in delivery notifications.</p>
    pub headers_in_delivery_notifications_enabled: Option<bool>,
}

struct IdentityNotificationAttributesDeserializer;
impl IdentityNotificationAttributesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<IdentityNotificationAttributes, XmlParseError> {
        deserialize_elements::<_, IdentityNotificationAttributes, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "BounceTopic" => {
                        obj.bounce_topic =
                            NotificationTopicDeserializer::deserialize("BounceTopic", stack)?;
                    }
                    "ComplaintTopic" => {
                        obj.complaint_topic =
                            NotificationTopicDeserializer::deserialize("ComplaintTopic", stack)?;
                    }
                    "DeliveryTopic" => {
                        obj.delivery_topic =
                            NotificationTopicDeserializer::deserialize("DeliveryTopic", stack)?;
                    }
                    "ForwardingEnabled" => {
                        obj.forwarding_enabled =
                            EnabledDeserializer::deserialize("ForwardingEnabled", stack)?;
                    }
                    "HeadersInBounceNotificationsEnabled" => {
                        obj.headers_in_bounce_notifications_enabled =
                            Some(EnabledDeserializer::deserialize(
                                "HeadersInBounceNotificationsEnabled",
                                stack,
                            )?);
                    }
                    "HeadersInComplaintNotificationsEnabled" => {
                        obj.headers_in_complaint_notifications_enabled =
                            Some(EnabledDeserializer::deserialize(
                                "HeadersInComplaintNotificationsEnabled",
                                stack,
                            )?);
                    }
                    "HeadersInDeliveryNotificationsEnabled" => {
                        obj.headers_in_delivery_notifications_enabled =
                            Some(EnabledDeserializer::deserialize(
                                "HeadersInDeliveryNotificationsEnabled",
                                stack,
                            )?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p>Represents the verification attributes of a single identity.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct IdentityVerificationAttributes {
    /// <p>The verification status of the identity: "Pending", "Success", "Failed", or "TemporaryFailure".</p>
    pub verification_status: String,
    /// <p>The verification token for a domain identity. Null for email address identities.</p>
    pub verification_token: Option<String>,
}

struct IdentityVerificationAttributesDeserializer;
impl IdentityVerificationAttributesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<IdentityVerificationAttributes, XmlParseError> {
        deserialize_elements::<_, IdentityVerificationAttributes, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "VerificationStatus" => {
                        obj.verification_status = VerificationStatusDeserializer::deserialize(
                            "VerificationStatus",
                            stack,
                        )?;
                    }
                    "VerificationToken" => {
                        obj.verification_token = Some(VerificationTokenDeserializer::deserialize(
                            "VerificationToken",
                            stack,
                        )?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
struct InvocationTypeDeserializer;
impl InvocationTypeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Contains the delivery stream ARN and the IAM role ARN associated with an Amazon Kinesis Firehose event destination.</p> <p>Event destinations, such as Amazon Kinesis Firehose, are associated with configuration sets, which enable you to publish email sending events. For information about using configuration sets, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/monitor-sending-activity.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct KinesisFirehoseDestination {
    /// <p>The ARN of the Amazon Kinesis Firehose stream that email sending events should be published to.</p>
    pub delivery_stream_arn: String,
    /// <p>The ARN of the IAM role under which Amazon SES publishes email sending events to the Amazon Kinesis Firehose stream.</p>
    pub iam_role_arn: String,
}

struct KinesisFirehoseDestinationDeserializer;
impl KinesisFirehoseDestinationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<KinesisFirehoseDestination, XmlParseError> {
        deserialize_elements::<_, KinesisFirehoseDestination, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "DeliveryStreamARN" => {
                        obj.delivery_stream_arn = AmazonResourceNameDeserializer::deserialize(
                            "DeliveryStreamARN",
                            stack,
                        )?;
                    }
                    "IAMRoleARN" => {
                        obj.iam_role_arn =
                            AmazonResourceNameDeserializer::deserialize("IAMRoleARN", stack)?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

/// Serialize `KinesisFirehoseDestination` contents to a `SignedRequest`.
struct KinesisFirehoseDestinationSerializer;
impl KinesisFirehoseDestinationSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &KinesisFirehoseDestination) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "DeliveryStreamARN"),
            &obj.delivery_stream_arn,
        );
        params.put(&format!("{}{}", prefix, "IAMRoleARN"), &obj.iam_role_arn);
    }
}

/// <p>When included in a receipt rule, this action calls an AWS Lambda function and, optionally, publishes a notification to Amazon Simple Notification Service (Amazon SNS).</p> <p>To enable Amazon SES to call your AWS Lambda function or to publish to an Amazon SNS topic of another account, Amazon SES must have permission to access those resources. For information about giving permissions, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-permissions.html">Amazon SES Developer Guide</a>.</p> <p>For information about using AWS Lambda actions in receipt rules, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-action-lambda.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct LambdaAction {
    /// <p>The Amazon Resource Name (ARN) of the AWS Lambda function. An example of an AWS Lambda function ARN is <code>arn:aws:lambda:us-west-2:account-id:function:MyFunction</code>. For more information about AWS Lambda, see the <a href="https://docs.aws.amazon.com/lambda/latest/dg/welcome.html">AWS Lambda Developer Guide</a>.</p>
    pub function_arn: String,
    /// <p><p>The invocation type of the AWS Lambda function. An invocation type of <code>RequestResponse</code> means that the execution of the function will immediately result in a response, and a value of <code>Event</code> means that the function will be invoked asynchronously. The default value is <code>Event</code>. For information about AWS Lambda invocation types, see the <a href="https://docs.aws.amazon.com/lambda/latest/dg/API_Invoke.html">AWS Lambda Developer Guide</a>.</p> <important> <p>There is a 30-second timeout on <code>RequestResponse</code> invocations. You should use <code>Event</code> invocation in most cases. Use <code>RequestResponse</code> only when you want to make a mail flow decision, such as whether to stop the receipt rule or the receipt rule set.</p> </important></p>
    pub invocation_type: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the Amazon SNS topic to notify when the Lambda action is taken. An example of an Amazon SNS topic ARN is <code>arn:aws:sns:us-west-2:123456789012:MyTopic</code>. For more information about Amazon SNS topics, see the <a href="https://docs.aws.amazon.com/sns/latest/dg/CreateTopic.html">Amazon SNS Developer Guide</a>.</p>
    pub topic_arn: Option<String>,
}

struct LambdaActionDeserializer;
impl LambdaActionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<LambdaAction, XmlParseError> {
        deserialize_elements::<_, LambdaAction, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "FunctionArn" => {
                    obj.function_arn =
                        AmazonResourceNameDeserializer::deserialize("FunctionArn", stack)?;
                }
                "InvocationType" => {
                    obj.invocation_type = Some(InvocationTypeDeserializer::deserialize(
                        "InvocationType",
                        stack,
                    )?);
                }
                "TopicArn" => {
                    obj.topic_arn = Some(AmazonResourceNameDeserializer::deserialize(
                        "TopicArn", stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

/// Serialize `LambdaAction` contents to a `SignedRequest`.
struct LambdaActionSerializer;
impl LambdaActionSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &LambdaAction) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "FunctionArn"), &obj.function_arn);
        if let Some(ref field_value) = obj.invocation_type {
            params.put(&format!("{}{}", prefix, "InvocationType"), &field_value);
        }
        if let Some(ref field_value) = obj.topic_arn {
            params.put(&format!("{}{}", prefix, "TopicArn"), &field_value);
        }
    }
}

struct LastFreshStartDeserializer;
impl LastFreshStartDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Represents a request to list the configuration sets associated with your AWS account. Configuration sets enable you to publish email sending events. For information about using configuration sets, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/monitor-sending-activity.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListConfigurationSetsRequest {
    /// <p>The number of configuration sets to return.</p>
    pub max_items: Option<i64>,
    /// <p>A token returned from a previous call to <code>ListConfigurationSets</code> to indicate the position of the configuration set in the configuration set list.</p>
    pub next_token: Option<String>,
}

/// Serialize `ListConfigurationSetsRequest` contents to a `SignedRequest`.
struct ListConfigurationSetsRequestSerializer;
impl ListConfigurationSetsRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ListConfigurationSetsRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.max_items {
            params.put(&format!("{}{}", prefix, "MaxItems"), &field_value);
        }
        if let Some(ref field_value) = obj.next_token {
            params.put(&format!("{}{}", prefix, "NextToken"), &field_value);
        }
    }
}

/// <p>A list of configuration sets associated with your AWS account. Configuration sets enable you to publish email sending events. For information about using configuration sets, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/monitor-sending-activity.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ListConfigurationSetsResponse {
    /// <p>A list of configuration sets.</p>
    pub configuration_sets: Option<Vec<ConfigurationSet>>,
    /// <p>A token indicating that there are additional configuration sets available to be listed. Pass this token to successive calls of <code>ListConfigurationSets</code>. </p>
    pub next_token: Option<String>,
}

struct ListConfigurationSetsResponseDeserializer;
impl ListConfigurationSetsResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListConfigurationSetsResponse, XmlParseError> {
        deserialize_elements::<_, ListConfigurationSetsResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "ConfigurationSets" => {
                        obj.configuration_sets.get_or_insert(vec![]).extend(
                            ConfigurationSetsDeserializer::deserialize("ConfigurationSets", stack)?,
                        );
                    }
                    "NextToken" => {
                        obj.next_token =
                            Some(NextTokenDeserializer::deserialize("NextToken", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p>Represents a request to list the existing custom verification email templates for your account.</p> <p>For more information about custom verification email templates, see <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/custom-verification-emails.html">Using Custom Verification Email Templates</a> in the <i>Amazon SES Developer Guide</i>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListCustomVerificationEmailTemplatesRequest {
    /// <p>The maximum number of custom verification email templates to return. This value must be at least 1 and less than or equal to 50. If you do not specify a value, or if you specify a value less than 1 or greater than 50, the operation will return up to 50 results.</p>
    pub max_results: Option<i64>,
    /// <p>An array the contains the name and creation time stamp for each template in your Amazon SES account.</p>
    pub next_token: Option<String>,
}

/// Serialize `ListCustomVerificationEmailTemplatesRequest` contents to a `SignedRequest`.
struct ListCustomVerificationEmailTemplatesRequestSerializer;
impl ListCustomVerificationEmailTemplatesRequestSerializer {
    fn serialize(
        params: &mut Params,
        name: &str,
        obj: &ListCustomVerificationEmailTemplatesRequest,
    ) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.max_results {
            params.put(&format!("{}{}", prefix, "MaxResults"), &field_value);
        }
        if let Some(ref field_value) = obj.next_token {
            params.put(&format!("{}{}", prefix, "NextToken"), &field_value);
        }
    }
}

/// <p>A paginated list of custom verification email templates.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ListCustomVerificationEmailTemplatesResponse {
    /// <p>A list of the custom verification email templates that exist in your account.</p>
    pub custom_verification_email_templates: Option<Vec<CustomVerificationEmailTemplate>>,
    /// <p>A token indicating that there are additional custom verification email templates available to be listed. Pass this token to a subsequent call to <code>ListTemplates</code> to retrieve the next 50 custom verification email templates.</p>
    pub next_token: Option<String>,
}

struct ListCustomVerificationEmailTemplatesResponseDeserializer;
impl ListCustomVerificationEmailTemplatesResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListCustomVerificationEmailTemplatesResponse, XmlParseError> {
        deserialize_elements::<_, ListCustomVerificationEmailTemplatesResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "CustomVerificationEmailTemplates" => {
                        obj.custom_verification_email_templates
                            .get_or_insert(vec![])
                            .extend(CustomVerificationEmailTemplatesDeserializer::deserialize(
                                "CustomVerificationEmailTemplates",
                                stack,
                            )?);
                    }
                    "NextToken" => {
                        obj.next_token =
                            Some(NextTokenDeserializer::deserialize("NextToken", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p>Represents a request to return a list of all identities (email addresses and domains) that you have attempted to verify under your AWS account, regardless of verification status.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListIdentitiesRequest {
    /// <p>The type of the identities to list. Possible values are "EmailAddress" and "Domain". If this parameter is omitted, then all identities will be listed.</p>
    pub identity_type: Option<String>,
    /// <p>The maximum number of identities per page. Possible values are 1-1000 inclusive.</p>
    pub max_items: Option<i64>,
    /// <p>The token to use for pagination.</p>
    pub next_token: Option<String>,
}

/// Serialize `ListIdentitiesRequest` contents to a `SignedRequest`.
struct ListIdentitiesRequestSerializer;
impl ListIdentitiesRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ListIdentitiesRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.identity_type {
            params.put(&format!("{}{}", prefix, "IdentityType"), &field_value);
        }
        if let Some(ref field_value) = obj.max_items {
            params.put(&format!("{}{}", prefix, "MaxItems"), &field_value);
        }
        if let Some(ref field_value) = obj.next_token {
            params.put(&format!("{}{}", prefix, "NextToken"), &field_value);
        }
    }
}

/// <p>A list of all identities that you have attempted to verify under your AWS account, regardless of verification status.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ListIdentitiesResponse {
    /// <p>A list of identities.</p>
    pub identities: Vec<String>,
    /// <p>The token used for pagination.</p>
    pub next_token: Option<String>,
}

struct ListIdentitiesResponseDeserializer;
impl ListIdentitiesResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListIdentitiesResponse, XmlParseError> {
        deserialize_elements::<_, ListIdentitiesResponse, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Identities" => {
                    obj.identities
                        .extend(IdentityListDeserializer::deserialize("Identities", stack)?);
                }
                "NextToken" => {
                    obj.next_token = Some(NextTokenDeserializer::deserialize("NextToken", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
/// <p>Represents a request to return a list of sending authorization policies that are attached to an identity. Sending authorization is an Amazon SES feature that enables you to authorize other senders to use your identities. For information, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/sending-authorization.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListIdentityPoliciesRequest {
    /// <p>The identity that is associated with the policy for which the policies will be listed. You can specify an identity by using its name or by using its Amazon Resource Name (ARN). Examples: <code>user@example.com</code>, <code>example.com</code>, <code>arn:aws:ses:us-east-1:123456789012:identity/example.com</code>.</p> <p>To successfully call this API, you must own the identity.</p>
    pub identity: String,
}

/// Serialize `ListIdentityPoliciesRequest` contents to a `SignedRequest`.
struct ListIdentityPoliciesRequestSerializer;
impl ListIdentityPoliciesRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ListIdentityPoliciesRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "Identity"), &obj.identity);
    }
}

/// <p>A list of names of sending authorization policies that apply to an identity.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ListIdentityPoliciesResponse {
    /// <p>A list of names of policies that apply to the specified identity.</p>
    pub policy_names: Vec<String>,
}

struct ListIdentityPoliciesResponseDeserializer;
impl ListIdentityPoliciesResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListIdentityPoliciesResponse, XmlParseError> {
        deserialize_elements::<_, ListIdentityPoliciesResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "PolicyNames" => {
                        obj.policy_names
                            .extend(PolicyNameListDeserializer::deserialize(
                                "PolicyNames",
                                stack,
                            )?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p>Represents a request to list the IP address filters that exist under your AWS account. You use IP address filters when you receive email with Amazon SES. For more information, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-concepts.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListReceiptFiltersRequest {}

/// Serialize `ListReceiptFiltersRequest` contents to a `SignedRequest`.
struct ListReceiptFiltersRequestSerializer;
impl ListReceiptFiltersRequestSerializer {
    fn serialize(_params: &mut Params, name: &str, _obj: &ListReceiptFiltersRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }
    }
}

/// <p>A list of IP address filters that exist under your AWS account.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ListReceiptFiltersResponse {
    /// <p>A list of IP address filter data structures, which each consist of a name, an IP address range, and whether to allow or block mail from it.</p>
    pub filters: Option<Vec<ReceiptFilter>>,
}

struct ListReceiptFiltersResponseDeserializer;
impl ListReceiptFiltersResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListReceiptFiltersResponse, XmlParseError> {
        deserialize_elements::<_, ListReceiptFiltersResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Filters" => {
                        obj.filters.get_or_insert(vec![]).extend(
                            ReceiptFilterListDeserializer::deserialize("Filters", stack)?,
                        );
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p>Represents a request to list the receipt rule sets that exist under your AWS account. You use receipt rule sets to receive email with Amazon SES. For more information, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-concepts.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListReceiptRuleSetsRequest {
    /// <p>A token returned from a previous call to <code>ListReceiptRuleSets</code> to indicate the position in the receipt rule set list.</p>
    pub next_token: Option<String>,
}

/// Serialize `ListReceiptRuleSetsRequest` contents to a `SignedRequest`.
struct ListReceiptRuleSetsRequestSerializer;
impl ListReceiptRuleSetsRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ListReceiptRuleSetsRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.next_token {
            params.put(&format!("{}{}", prefix, "NextToken"), &field_value);
        }
    }
}

/// <p>A list of receipt rule sets that exist under your AWS account.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ListReceiptRuleSetsResponse {
    /// <p>A token indicating that there are additional receipt rule sets available to be listed. Pass this token to successive calls of <code>ListReceiptRuleSets</code> to retrieve up to 100 receipt rule sets at a time.</p>
    pub next_token: Option<String>,
    /// <p>The metadata for the currently active receipt rule set. The metadata consists of the rule set name and the timestamp of when the rule set was created.</p>
    pub rule_sets: Option<Vec<ReceiptRuleSetMetadata>>,
}

struct ListReceiptRuleSetsResponseDeserializer;
impl ListReceiptRuleSetsResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListReceiptRuleSetsResponse, XmlParseError> {
        deserialize_elements::<_, ListReceiptRuleSetsResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "NextToken" => {
                        obj.next_token =
                            Some(NextTokenDeserializer::deserialize("NextToken", stack)?);
                    }
                    "RuleSets" => {
                        obj.rule_sets.get_or_insert(vec![]).extend(
                            ReceiptRuleSetsListsDeserializer::deserialize("RuleSets", stack)?,
                        );
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTemplatesRequest {
    /// <p>The maximum number of templates to return. This value must be at least 1 and less than or equal to 10. If you do not specify a value, or if you specify a value less than 1 or greater than 10, the operation will return up to 10 results.</p>
    pub max_items: Option<i64>,
    /// <p>A token returned from a previous call to <code>ListTemplates</code> to indicate the position in the list of email templates.</p>
    pub next_token: Option<String>,
}

/// Serialize `ListTemplatesRequest` contents to a `SignedRequest`.
struct ListTemplatesRequestSerializer;
impl ListTemplatesRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ListTemplatesRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.max_items {
            params.put(&format!("{}{}", prefix, "MaxItems"), &field_value);
        }
        if let Some(ref field_value) = obj.next_token {
            params.put(&format!("{}{}", prefix, "NextToken"), &field_value);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ListTemplatesResponse {
    /// <p>A token indicating that there are additional email templates available to be listed. Pass this token to a subsequent call to <code>ListTemplates</code> to retrieve the next 50 email templates.</p>
    pub next_token: Option<String>,
    /// <p>An array the contains the name and creation time stamp for each template in your Amazon SES account.</p>
    pub templates_metadata: Option<Vec<TemplateMetadata>>,
}

struct ListTemplatesResponseDeserializer;
impl ListTemplatesResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListTemplatesResponse, XmlParseError> {
        deserialize_elements::<_, ListTemplatesResponse, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "NextToken" => {
                    obj.next_token = Some(NextTokenDeserializer::deserialize("NextToken", stack)?);
                }
                "TemplatesMetadata" => {
                    obj.templates_metadata.get_or_insert(vec![]).extend(
                        TemplateMetadataListDeserializer::deserialize("TemplatesMetadata", stack)?,
                    );
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
/// <p>A list of email addresses that you have verified with Amazon SES under your AWS account.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ListVerifiedEmailAddressesResponse {
    /// <p>A list of email addresses that have been verified.</p>
    pub verified_email_addresses: Option<Vec<String>>,
}

struct ListVerifiedEmailAddressesResponseDeserializer;
impl ListVerifiedEmailAddressesResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListVerifiedEmailAddressesResponse, XmlParseError> {
        deserialize_elements::<_, ListVerifiedEmailAddressesResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "VerifiedEmailAddresses" => {
                        obj.verified_email_addresses.get_or_insert(vec![]).extend(
                            AddressListDeserializer::deserialize("VerifiedEmailAddresses", stack)?,
                        );
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
struct MailFromDomainAttributesDeserializer;
impl MailFromDomainAttributesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<::std::collections::HashMap<String, IdentityMailFromDomainAttributes>, XmlParseError>
    {
        start_element(tag_name, stack)?;

        let mut obj = ::std::collections::HashMap::new();

        while peek_at_name(stack)? == "entry" {
            start_element("entry", stack)?;
            let key = IdentityDeserializer::deserialize("key", stack)?;
            let value = IdentityMailFromDomainAttributesDeserializer::deserialize("value", stack)?;
            obj.insert(key, value);
            end_element("entry", stack)?;
        }

        end_element(tag_name, stack)?;
        Ok(obj)
    }
}
struct MailFromDomainNameDeserializer;
impl MailFromDomainNameDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct Max24HourSendDeserializer;
impl Max24HourSendDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<f64, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = f64::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct MaxSendRateDeserializer;
impl MaxSendRateDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<f64, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = f64::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Represents the message to be sent, composed of a subject and a body.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Message {
    /// <p>The message body.</p>
    pub body: Body,
    /// <p>The subject of the message: A short summary of the content, which will appear in the recipient's inbox.</p>
    pub subject: Content,
}

/// Serialize `Message` contents to a `SignedRequest`.
struct MessageSerializer;
impl MessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Message) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        BodySerializer::serialize(params, &format!("{}{}", prefix, "Body"), &obj.body);
        ContentSerializer::serialize(params, &format!("{}{}", prefix, "Subject"), &obj.subject);
    }
}

/// <p>Message-related information to include in the Delivery Status Notification (DSN) when an email that Amazon SES receives on your behalf bounces.</p> <p>For information about receiving email through Amazon SES, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct MessageDsn {
    /// <p>When the message was received by the reporting mail transfer agent (MTA), in <a href="https://www.ietf.org/rfc/rfc0822.txt">RFC 822</a> date-time format.</p>
    pub arrival_date: Option<String>,
    /// <p>Additional X-headers to include in the DSN.</p>
    pub extension_fields: Option<Vec<ExtensionField>>,
    /// <p>The reporting MTA that attempted to deliver the message, formatted as specified in <a href="https://tools.ietf.org/html/rfc3464">RFC 3464</a> (<code>mta-name-type; mta-name</code>). The default value is <code>dns; inbound-smtp.[region].amazonaws.com</code>.</p>
    pub reporting_mta: String,
}

/// Serialize `MessageDsn` contents to a `SignedRequest`.
struct MessageDsnSerializer;
impl MessageDsnSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &MessageDsn) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.arrival_date {
            params.put(&format!("{}{}", prefix, "ArrivalDate"), &field_value);
        }
        if let Some(ref field_value) = obj.extension_fields {
            ExtensionFieldListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "ExtensionFields"),
                field_value,
            );
        }
        params.put(&format!("{}{}", prefix, "ReportingMta"), &obj.reporting_mta);
    }
}

struct MessageIdDeserializer;
impl MessageIdDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Contains the name and value of a tag that you can provide to <code>SendEmail</code> or <code>SendRawEmail</code> to apply to an email.</p> <p>Message tags, which you use with configuration sets, enable you to publish email sending events. For information about using configuration sets, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/monitor-sending-activity.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct MessageTag {
    /// <p><p>The name of the tag. The name must:</p> <ul> <li> <p>This value can only contain ASCII letters (a-z, A-Z), numbers (0-9), underscores (_), or dashes (-).</p> </li> <li> <p>Contain less than 256 characters.</p> </li> </ul></p>
    pub name: String,
    /// <p><p>The value of the tag. The value must:</p> <ul> <li> <p>This value can only contain ASCII letters (a-z, A-Z), numbers (0-9), underscores (_), or dashes (-).</p> </li> <li> <p>Contain less than 256 characters.</p> </li> </ul></p>
    pub value: String,
}

/// Serialize `MessageTag` contents to a `SignedRequest`.
struct MessageTagSerializer;
impl MessageTagSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &MessageTag) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "Name"), &obj.name);
        params.put(&format!("{}{}", prefix, "Value"), &obj.value);
    }
}

/// Serialize `MessageTagList` contents to a `SignedRequest`.
struct MessageTagListSerializer;
impl MessageTagListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<MessageTag>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            MessageTagSerializer::serialize(params, &key, obj);
        }
    }
}

struct NextTokenDeserializer;
impl NextTokenDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct NotificationAttributesDeserializer;
impl NotificationAttributesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<::std::collections::HashMap<String, IdentityNotificationAttributes>, XmlParseError>
    {
        start_element(tag_name, stack)?;

        let mut obj = ::std::collections::HashMap::new();

        while peek_at_name(stack)? == "entry" {
            start_element("entry", stack)?;
            let key = IdentityDeserializer::deserialize("key", stack)?;
            let value = IdentityNotificationAttributesDeserializer::deserialize("value", stack)?;
            obj.insert(key, value);
            end_element("entry", stack)?;
        }

        end_element(tag_name, stack)?;
        Ok(obj)
    }
}
struct NotificationTopicDeserializer;
impl NotificationTopicDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct PolicyDeserializer;
impl PolicyDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct PolicyMapDeserializer;
impl PolicyMapDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<::std::collections::HashMap<String, String>, XmlParseError> {
        start_element(tag_name, stack)?;

        let mut obj = ::std::collections::HashMap::new();

        while peek_at_name(stack)? == "entry" {
            start_element("entry", stack)?;
            let key = PolicyNameDeserializer::deserialize("key", stack)?;
            let value = PolicyDeserializer::deserialize("value", stack)?;
            obj.insert(key, value);
            end_element("entry", stack)?;
        }

        end_element(tag_name, stack)?;
        Ok(obj)
    }
}
struct PolicyNameDeserializer;
impl PolicyNameDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct PolicyNameListDeserializer;
impl PolicyNameListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(PolicyNameDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}

/// Serialize `PolicyNameList` contents to a `SignedRequest`.
struct PolicyNameListSerializer;
impl PolicyNameListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

/// <p>A request to modify the delivery options for a configuration set.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutConfigurationSetDeliveryOptionsRequest {
    /// <p>The name of the configuration set that you want to specify the delivery options for.</p>
    pub configuration_set_name: String,
    /// <p>Specifies whether messages that use the configuration set are required to use Transport Layer Security (TLS).</p>
    pub delivery_options: Option<DeliveryOptions>,
}

/// Serialize `PutConfigurationSetDeliveryOptionsRequest` contents to a `SignedRequest`.
struct PutConfigurationSetDeliveryOptionsRequestSerializer;
impl PutConfigurationSetDeliveryOptionsRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &PutConfigurationSetDeliveryOptionsRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "ConfigurationSetName"),
            &obj.configuration_set_name,
        );
        if let Some(ref field_value) = obj.delivery_options {
            DeliveryOptionsSerializer::serialize(
                params,
                &format!("{}{}", prefix, "DeliveryOptions"),
                field_value,
            );
        }
    }
}

/// <p>An HTTP 200 response if the request succeeds, or an error message if the request fails.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct PutConfigurationSetDeliveryOptionsResponse {}

struct PutConfigurationSetDeliveryOptionsResponseDeserializer;
impl PutConfigurationSetDeliveryOptionsResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PutConfigurationSetDeliveryOptionsResponse, XmlParseError> {
        start_element(tag_name, stack)?;

        let obj = PutConfigurationSetDeliveryOptionsResponse::default();

        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Represents a request to add or update a sending authorization policy for an identity. Sending authorization is an Amazon SES feature that enables you to authorize other senders to use your identities. For information, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/sending-authorization.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutIdentityPolicyRequest {
    /// <p>The identity that the policy will apply to. You can specify an identity by using its name or by using its Amazon Resource Name (ARN). Examples: <code>user@example.com</code>, <code>example.com</code>, <code>arn:aws:ses:us-east-1:123456789012:identity/example.com</code>.</p> <p>To successfully call this API, you must own the identity.</p>
    pub identity: String,
    /// <p>The text of the policy in JSON format. The policy cannot exceed 4 KB.</p> <p>For information about the syntax of sending authorization policies, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/sending-authorization-policies.html">Amazon SES Developer Guide</a>. </p>
    pub policy: String,
    /// <p>The name of the policy.</p> <p>The policy name cannot exceed 64 characters and can only include alphanumeric characters, dashes, and underscores.</p>
    pub policy_name: String,
}

/// Serialize `PutIdentityPolicyRequest` contents to a `SignedRequest`.
struct PutIdentityPolicyRequestSerializer;
impl PutIdentityPolicyRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &PutIdentityPolicyRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "Identity"), &obj.identity);
        params.put(&format!("{}{}", prefix, "Policy"), &obj.policy);
        params.put(&format!("{}{}", prefix, "PolicyName"), &obj.policy_name);
    }
}

/// <p>An empty element returned on a successful request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct PutIdentityPolicyResponse {}

struct PutIdentityPolicyResponseDeserializer;
impl PutIdentityPolicyResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PutIdentityPolicyResponse, XmlParseError> {
        start_element(tag_name, stack)?;

        let obj = PutIdentityPolicyResponse::default();

        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Represents the raw data of the message.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RawMessage {
    /// <p>The raw data of the message. This data needs to base64-encoded if you are accessing Amazon SES directly through the HTTPS interface. If you are accessing Amazon SES using an AWS SDK, the SDK takes care of the base 64-encoding for you. In all cases, the client must ensure that the message format complies with Internet email standards regarding email header fields, MIME types, and MIME encoding.</p> <p>The To:, CC:, and BCC: headers in the raw message can contain a group list.</p> <p>If you are using <code>SendRawEmail</code> with sending authorization, you can include X-headers in the raw message to specify the "Source," "From," and "Return-Path" addresses. For more information, see the documentation for <code>SendRawEmail</code>. </p> <important> <p>Do not include these X-headers in the DKIM signature, because they are removed by Amazon SES before sending the email.</p> </important> <p>For more information, go to the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/send-email-raw.html">Amazon SES Developer Guide</a>.</p>
    pub data: bytes::Bytes,
}

/// Serialize `RawMessage` contents to a `SignedRequest`.
struct RawMessageSerializer;
impl RawMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &RawMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "Data"),
            ::std::str::from_utf8(&obj.data).unwrap(),
        );
    }
}

/// <p>An action that Amazon SES can take when it receives an email on behalf of one or more email addresses or domains that you own. An instance of this data type can represent only one action.</p> <p>For information about setting up receipt rules, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-receipt-rules.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ReceiptAction {
    /// <p>Adds a header to the received email.</p>
    pub add_header_action: Option<AddHeaderAction>,
    /// <p>Rejects the received email by returning a bounce response to the sender and, optionally, publishes a notification to Amazon Simple Notification Service (Amazon SNS).</p>
    pub bounce_action: Option<BounceAction>,
    /// <p>Calls an AWS Lambda function, and optionally, publishes a notification to Amazon SNS.</p>
    pub lambda_action: Option<LambdaAction>,
    /// <p>Saves the received message to an Amazon Simple Storage Service (Amazon S3) bucket and, optionally, publishes a notification to Amazon SNS.</p>
    pub s3_action: Option<S3Action>,
    /// <p>Publishes the email content within a notification to Amazon SNS.</p>
    pub sns_action: Option<SNSAction>,
    /// <p>Terminates the evaluation of the receipt rule set and optionally publishes a notification to Amazon SNS.</p>
    pub stop_action: Option<StopAction>,
    /// <p>Calls Amazon WorkMail and, optionally, publishes a notification to Amazon Amazon SNS.</p>
    pub workmail_action: Option<WorkmailAction>,
}

struct ReceiptActionDeserializer;
impl ReceiptActionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ReceiptAction, XmlParseError> {
        deserialize_elements::<_, ReceiptAction, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "AddHeaderAction" => {
                    obj.add_header_action = Some(AddHeaderActionDeserializer::deserialize(
                        "AddHeaderAction",
                        stack,
                    )?);
                }
                "BounceAction" => {
                    obj.bounce_action = Some(BounceActionDeserializer::deserialize(
                        "BounceAction",
                        stack,
                    )?);
                }
                "LambdaAction" => {
                    obj.lambda_action = Some(LambdaActionDeserializer::deserialize(
                        "LambdaAction",
                        stack,
                    )?);
                }
                "S3Action" => {
                    obj.s3_action = Some(S3ActionDeserializer::deserialize("S3Action", stack)?);
                }
                "SNSAction" => {
                    obj.sns_action = Some(SNSActionDeserializer::deserialize("SNSAction", stack)?);
                }
                "StopAction" => {
                    obj.stop_action =
                        Some(StopActionDeserializer::deserialize("StopAction", stack)?);
                }
                "WorkmailAction" => {
                    obj.workmail_action = Some(WorkmailActionDeserializer::deserialize(
                        "WorkmailAction",
                        stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

/// Serialize `ReceiptAction` contents to a `SignedRequest`.
struct ReceiptActionSerializer;
impl ReceiptActionSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ReceiptAction) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.add_header_action {
            AddHeaderActionSerializer::serialize(
                params,
                &format!("{}{}", prefix, "AddHeaderAction"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.bounce_action {
            BounceActionSerializer::serialize(
                params,
                &format!("{}{}", prefix, "BounceAction"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.lambda_action {
            LambdaActionSerializer::serialize(
                params,
                &format!("{}{}", prefix, "LambdaAction"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.s3_action {
            S3ActionSerializer::serialize(
                params,
                &format!("{}{}", prefix, "S3Action"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.sns_action {
            SNSActionSerializer::serialize(
                params,
                &format!("{}{}", prefix, "SNSAction"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.stop_action {
            StopActionSerializer::serialize(
                params,
                &format!("{}{}", prefix, "StopAction"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.workmail_action {
            WorkmailActionSerializer::serialize(
                params,
                &format!("{}{}", prefix, "WorkmailAction"),
                field_value,
            );
        }
    }
}

struct ReceiptActionsListDeserializer;
impl ReceiptActionsListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<ReceiptAction>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(ReceiptActionDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}

/// Serialize `ReceiptActionsList` contents to a `SignedRequest`.
struct ReceiptActionsListSerializer;
impl ReceiptActionsListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<ReceiptAction>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            ReceiptActionSerializer::serialize(params, &key, obj);
        }
    }
}

/// <p>A receipt IP address filter enables you to specify whether to accept or reject mail originating from an IP address or range of IP addresses.</p> <p>For information about setting up IP address filters, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-ip-filters.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ReceiptFilter {
    /// <p>A structure that provides the IP addresses to block or allow, and whether to block or allow incoming mail from them.</p>
    pub ip_filter: ReceiptIpFilter,
    /// <p><p>The name of the IP address filter. The name must:</p> <ul> <li> <p>This value can only contain ASCII letters (a-z, A-Z), numbers (0-9), underscores (_), or dashes (-).</p> </li> <li> <p>Start and end with a letter or number.</p> </li> <li> <p>Contain less than 64 characters.</p> </li> </ul></p>
    pub name: String,
}

struct ReceiptFilterDeserializer;
impl ReceiptFilterDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ReceiptFilter, XmlParseError> {
        deserialize_elements::<_, ReceiptFilter, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "IpFilter" => {
                    obj.ip_filter = ReceiptIpFilterDeserializer::deserialize("IpFilter", stack)?;
                }
                "Name" => {
                    obj.name = ReceiptFilterNameDeserializer::deserialize("Name", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

/// Serialize `ReceiptFilter` contents to a `SignedRequest`.
struct ReceiptFilterSerializer;
impl ReceiptFilterSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ReceiptFilter) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        ReceiptIpFilterSerializer::serialize(
            params,
            &format!("{}{}", prefix, "IpFilter"),
            &obj.ip_filter,
        );
        params.put(&format!("{}{}", prefix, "Name"), &obj.name);
    }
}

struct ReceiptFilterListDeserializer;
impl ReceiptFilterListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<ReceiptFilter>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(ReceiptFilterDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
struct ReceiptFilterNameDeserializer;
impl ReceiptFilterNameDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct ReceiptFilterPolicyDeserializer;
impl ReceiptFilterPolicyDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>A receipt IP address filter enables you to specify whether to accept or reject mail originating from an IP address or range of IP addresses.</p> <p>For information about setting up IP address filters, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-ip-filters.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ReceiptIpFilter {
    /// <p>A single IP address or a range of IP addresses that you want to block or allow, specified in Classless Inter-Domain Routing (CIDR) notation. An example of a single email address is 10.0.0.1. An example of a range of IP addresses is 10.0.0.1/24. For more information about CIDR notation, see <a href="https://tools.ietf.org/html/rfc2317">RFC 2317</a>.</p>
    pub cidr: String,
    /// <p>Indicates whether to block or allow incoming mail from the specified IP addresses.</p>
    pub policy: String,
}

struct ReceiptIpFilterDeserializer;
impl ReceiptIpFilterDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ReceiptIpFilter, XmlParseError> {
        deserialize_elements::<_, ReceiptIpFilter, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Cidr" => {
                    obj.cidr = CidrDeserializer::deserialize("Cidr", stack)?;
                }
                "Policy" => {
                    obj.policy = ReceiptFilterPolicyDeserializer::deserialize("Policy", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

/// Serialize `ReceiptIpFilter` contents to a `SignedRequest`.
struct ReceiptIpFilterSerializer;
impl ReceiptIpFilterSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ReceiptIpFilter) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "Cidr"), &obj.cidr);
        params.put(&format!("{}{}", prefix, "Policy"), &obj.policy);
    }
}

/// <p>Receipt rules enable you to specify which actions Amazon SES should take when it receives mail on behalf of one or more email addresses or domains that you own.</p> <p>Each receipt rule defines a set of email addresses or domains that it applies to. If the email addresses or domains match at least one recipient address of the message, Amazon SES executes all of the receipt rule's actions on the message.</p> <p>For information about setting up receipt rules, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-receipt-rules.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ReceiptRule {
    /// <p>An ordered list of actions to perform on messages that match at least one of the recipient email addresses or domains specified in the receipt rule.</p>
    pub actions: Option<Vec<ReceiptAction>>,
    /// <p>If <code>true</code>, the receipt rule is active. The default value is <code>false</code>.</p>
    pub enabled: Option<bool>,
    /// <p><p>The name of the receipt rule. The name must:</p> <ul> <li> <p>This value can only contain ASCII letters (a-z, A-Z), numbers (0-9), underscores (_), or dashes (-).</p> </li> <li> <p>Start and end with a letter or number.</p> </li> <li> <p>Contain less than 64 characters.</p> </li> </ul></p>
    pub name: String,
    /// <p>The recipient domains and email addresses that the receipt rule applies to. If this field is not specified, this rule will match all recipients under all verified domains.</p>
    pub recipients: Option<Vec<String>>,
    /// <p>If <code>true</code>, then messages that this receipt rule applies to are scanned for spam and viruses. The default value is <code>false</code>.</p>
    pub scan_enabled: Option<bool>,
    /// <p>Specifies whether Amazon SES should require that incoming email is delivered over a connection encrypted with Transport Layer Security (TLS). If this parameter is set to <code>Require</code>, Amazon SES will bounce emails that are not received over TLS. The default is <code>Optional</code>.</p>
    pub tls_policy: Option<String>,
}

struct ReceiptRuleDeserializer;
impl ReceiptRuleDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ReceiptRule, XmlParseError> {
        deserialize_elements::<_, ReceiptRule, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Actions" => {
                    obj.actions.get_or_insert(vec![]).extend(
                        ReceiptActionsListDeserializer::deserialize("Actions", stack)?,
                    );
                }
                "Enabled" => {
                    obj.enabled = Some(EnabledDeserializer::deserialize("Enabled", stack)?);
                }
                "Name" => {
                    obj.name = ReceiptRuleNameDeserializer::deserialize("Name", stack)?;
                }
                "Recipients" => {
                    obj.recipients.get_or_insert(vec![]).extend(
                        RecipientsListDeserializer::deserialize("Recipients", stack)?,
                    );
                }
                "ScanEnabled" => {
                    obj.scan_enabled =
                        Some(EnabledDeserializer::deserialize("ScanEnabled", stack)?);
                }
                "TlsPolicy" => {
                    obj.tls_policy = Some(TlsPolicyDeserializer::deserialize("TlsPolicy", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

/// Serialize `ReceiptRule` contents to a `SignedRequest`.
struct ReceiptRuleSerializer;
impl ReceiptRuleSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ReceiptRule) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.actions {
            ReceiptActionsListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Actions"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.enabled {
            params.put(&format!("{}{}", prefix, "Enabled"), &field_value);
        }
        params.put(&format!("{}{}", prefix, "Name"), &obj.name);
        if let Some(ref field_value) = obj.recipients {
            RecipientsListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Recipients"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.scan_enabled {
            params.put(&format!("{}{}", prefix, "ScanEnabled"), &field_value);
        }
        if let Some(ref field_value) = obj.tls_policy {
            params.put(&format!("{}{}", prefix, "TlsPolicy"), &field_value);
        }
    }
}

struct ReceiptRuleNameDeserializer;
impl ReceiptRuleNameDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

/// Serialize `ReceiptRuleNamesList` contents to a `SignedRequest`.
struct ReceiptRuleNamesListSerializer;
impl ReceiptRuleNamesListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

/// <p>Information about a receipt rule set.</p> <p>A receipt rule set is a collection of rules that specify what Amazon SES should do with mail it receives on behalf of your account's verified domains.</p> <p>For information about setting up receipt rule sets, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-receipt-rule-set.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ReceiptRuleSetMetadata {
    /// <p>The date and time the receipt rule set was created.</p>
    pub created_timestamp: Option<String>,
    /// <p><p>The name of the receipt rule set. The name must:</p> <ul> <li> <p>This value can only contain ASCII letters (a-z, A-Z), numbers (0-9), underscores (_), or dashes (-).</p> </li> <li> <p>Start and end with a letter or number.</p> </li> <li> <p>Contain less than 64 characters.</p> </li> </ul></p>
    pub name: Option<String>,
}

struct ReceiptRuleSetMetadataDeserializer;
impl ReceiptRuleSetMetadataDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ReceiptRuleSetMetadata, XmlParseError> {
        deserialize_elements::<_, ReceiptRuleSetMetadata, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "CreatedTimestamp" => {
                    obj.created_timestamp = Some(TimestampDeserializer::deserialize(
                        "CreatedTimestamp",
                        stack,
                    )?);
                }
                "Name" => {
                    obj.name = Some(ReceiptRuleSetNameDeserializer::deserialize("Name", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct ReceiptRuleSetNameDeserializer;
impl ReceiptRuleSetNameDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct ReceiptRuleSetsListsDeserializer;
impl ReceiptRuleSetsListsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<ReceiptRuleSetMetadata>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(ReceiptRuleSetMetadataDeserializer::deserialize(
                    "member", stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
struct ReceiptRulesListDeserializer;
impl ReceiptRulesListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<ReceiptRule>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(ReceiptRuleDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
struct RecipientDeserializer;
impl RecipientDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Recipient-related information to include in the Delivery Status Notification (DSN) when an email that Amazon SES receives on your behalf bounces.</p> <p>For information about receiving email through Amazon SES, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RecipientDsnFields {
    /// <p>The action performed by the reporting mail transfer agent (MTA) as a result of its attempt to deliver the message to the recipient address. This is required by <a href="https://tools.ietf.org/html/rfc3464">RFC 3464</a>.</p>
    pub action: String,
    /// <p>An extended explanation of what went wrong; this is usually an SMTP response. See <a href="https://tools.ietf.org/html/rfc3463">RFC 3463</a> for the correct formatting of this parameter.</p>
    pub diagnostic_code: Option<String>,
    /// <p>Additional X-headers to include in the DSN.</p>
    pub extension_fields: Option<Vec<ExtensionField>>,
    /// <p><p>The email address that the message was ultimately delivered to. This corresponds to the <code>Final-Recipient</code> in the DSN. If not specified, <code>FinalRecipient</code> will be set to the <code>Recipient</code> specified in the <code>BouncedRecipientInfo</code> structure. Either <code>FinalRecipient</code> or the recipient in <code>BouncedRecipientInfo</code> must be a recipient of the original bounced message.</p> <note> <p>Do not prepend the <code>FinalRecipient</code> email address with <code>rfc 822;</code>, as described in <a href="https://tools.ietf.org/html/rfc3798">RFC 3798</a>.</p> </note></p>
    pub final_recipient: Option<String>,
    /// <p>The time the final delivery attempt was made, in <a href="https://www.ietf.org/rfc/rfc0822.txt">RFC 822</a> date-time format.</p>
    pub last_attempt_date: Option<String>,
    /// <p>The MTA to which the remote MTA attempted to deliver the message, formatted as specified in <a href="https://tools.ietf.org/html/rfc3464">RFC 3464</a> (<code>mta-name-type; mta-name</code>). This parameter typically applies only to propagating synchronous bounces.</p>
    pub remote_mta: Option<String>,
    /// <p>The status code that indicates what went wrong. This is required by <a href="https://tools.ietf.org/html/rfc3464">RFC 3464</a>.</p>
    pub status: String,
}

/// Serialize `RecipientDsnFields` contents to a `SignedRequest`.
struct RecipientDsnFieldsSerializer;
impl RecipientDsnFieldsSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &RecipientDsnFields) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "Action"), &obj.action);
        if let Some(ref field_value) = obj.diagnostic_code {
            params.put(&format!("{}{}", prefix, "DiagnosticCode"), &field_value);
        }
        if let Some(ref field_value) = obj.extension_fields {
            ExtensionFieldListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "ExtensionFields"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.final_recipient {
            params.put(&format!("{}{}", prefix, "FinalRecipient"), &field_value);
        }
        if let Some(ref field_value) = obj.last_attempt_date {
            params.put(&format!("{}{}", prefix, "LastAttemptDate"), &field_value);
        }
        if let Some(ref field_value) = obj.remote_mta {
            params.put(&format!("{}{}", prefix, "RemoteMta"), &field_value);
        }
        params.put(&format!("{}{}", prefix, "Status"), &obj.status);
    }
}

struct RecipientsListDeserializer;
impl RecipientsListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(RecipientDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}

/// Serialize `RecipientsList` contents to a `SignedRequest`.
struct RecipientsListSerializer;
impl RecipientsListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

struct RenderedTemplateDeserializer;
impl RenderedTemplateDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Represents a request to reorder the receipt rules within a receipt rule set. You use receipt rule sets to receive email with Amazon SES. For more information, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-concepts.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ReorderReceiptRuleSetRequest {
    /// <p>A list of the specified receipt rule set's receipt rules in the order that you want to put them.</p>
    pub rule_names: Vec<String>,
    /// <p>The name of the receipt rule set to reorder.</p>
    pub rule_set_name: String,
}

/// Serialize `ReorderReceiptRuleSetRequest` contents to a `SignedRequest`.
struct ReorderReceiptRuleSetRequestSerializer;
impl ReorderReceiptRuleSetRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ReorderReceiptRuleSetRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        ReceiptRuleNamesListSerializer::serialize(
            params,
            &format!("{}{}", prefix, "RuleNames"),
            &obj.rule_names,
        );
        params.put(&format!("{}{}", prefix, "RuleSetName"), &obj.rule_set_name);
    }
}

/// <p>An empty element returned on a successful request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ReorderReceiptRuleSetResponse {}

struct ReorderReceiptRuleSetResponseDeserializer;
impl ReorderReceiptRuleSetResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ReorderReceiptRuleSetResponse, XmlParseError> {
        start_element(tag_name, stack)?;

        let obj = ReorderReceiptRuleSetResponse::default();

        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Contains information about the reputation settings for a configuration set.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ReputationOptions {
    /// <p>The date and time at which the reputation metrics for the configuration set were last reset. Resetting these metrics is known as a <i>fresh start</i>.</p> <p>When you disable email sending for a configuration set using <a>UpdateConfigurationSetSendingEnabled</a> and later re-enable it, the reputation metrics for the configuration set (but not for the entire Amazon SES account) are reset.</p> <p>If email sending for the configuration set has never been disabled and later re-enabled, the value of this attribute is <code>null</code>.</p>
    pub last_fresh_start: Option<String>,
    /// <p>Describes whether or not Amazon SES publishes reputation metrics for the configuration set, such as bounce and complaint rates, to Amazon CloudWatch.</p> <p>If the value is <code>true</code>, reputation metrics are published. If the value is <code>false</code>, reputation metrics are not published. The default value is <code>false</code>.</p>
    pub reputation_metrics_enabled: Option<bool>,
    /// <p>Describes whether email sending is enabled or disabled for the configuration set. If the value is <code>true</code>, then Amazon SES will send emails that use the configuration set. If the value is <code>false</code>, Amazon SES will not send emails that use the configuration set. The default value is <code>true</code>. You can change this setting using <a>UpdateConfigurationSetSendingEnabled</a>.</p>
    pub sending_enabled: Option<bool>,
}

struct ReputationOptionsDeserializer;
impl ReputationOptionsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ReputationOptions, XmlParseError> {
        deserialize_elements::<_, ReputationOptions, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "LastFreshStart" => {
                    obj.last_fresh_start = Some(LastFreshStartDeserializer::deserialize(
                        "LastFreshStart",
                        stack,
                    )?);
                }
                "ReputationMetricsEnabled" => {
                    obj.reputation_metrics_enabled = Some(EnabledDeserializer::deserialize(
                        "ReputationMetricsEnabled",
                        stack,
                    )?);
                }
                "SendingEnabled" => {
                    obj.sending_enabled =
                        Some(EnabledDeserializer::deserialize("SendingEnabled", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
/// <p>When included in a receipt rule, this action saves the received message to an Amazon Simple Storage Service (Amazon S3) bucket and, optionally, publishes a notification to Amazon Simple Notification Service (Amazon SNS).</p> <p>To enable Amazon SES to write emails to your Amazon S3 bucket, use an AWS KMS key to encrypt your emails, or publish to an Amazon SNS topic of another account, Amazon SES must have permission to access those resources. For information about giving permissions, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-permissions.html">Amazon SES Developer Guide</a>.</p> <note> <p>When you save your emails to an Amazon S3 bucket, the maximum email size (including headers) is 30 MB. Emails larger than that will bounce.</p> </note> <p>For information about specifying Amazon S3 actions in receipt rules, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-action-s3.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct S3Action {
    /// <p>The name of the Amazon S3 bucket that incoming email will be saved to.</p>
    pub bucket_name: String,
    /// <p><p>The customer master key that Amazon SES should use to encrypt your emails before saving them to the Amazon S3 bucket. You can use the default master key or a custom master key you created in AWS KMS as follows:</p> <ul> <li> <p>To use the default master key, provide an ARN in the form of <code>arn:aws:kms:REGION:ACCOUNT-ID-WITHOUT-HYPHENS:alias/aws/ses</code>. For example, if your AWS account ID is 123456789012 and you want to use the default master key in the US West (Oregon) region, the ARN of the default master key would be <code>arn:aws:kms:us-west-2:123456789012:alias/aws/ses</code>. If you use the default master key, you don&#39;t need to perform any extra steps to give Amazon SES permission to use the key.</p> </li> <li> <p>To use a custom master key you created in AWS KMS, provide the ARN of the master key and ensure that you add a statement to your key&#39;s policy to give Amazon SES permission to use it. For more information about giving permissions, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-permissions.html">Amazon SES Developer Guide</a>.</p> </li> </ul> <p>For more information about key policies, see the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html">AWS KMS Developer Guide</a>. If you do not specify a master key, Amazon SES will not encrypt your emails.</p> <important> <p>Your mail is encrypted by Amazon SES using the Amazon S3 encryption client before the mail is submitted to Amazon S3 for storage. It is not encrypted using Amazon S3 server-side encryption. This means that you must use the Amazon S3 encryption client to decrypt the email after retrieving it from Amazon S3, as the service has no access to use your AWS KMS keys for decryption. This encryption client is currently available with the <a href="http://aws.amazon.com/sdk-for-java/">AWS SDK for Java</a> and <a href="http://aws.amazon.com/sdk-for-ruby/">AWS SDK for Ruby</a> only. For more information about client-side encryption using AWS KMS master keys, see the <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/UsingClientSideEncryption.html">Amazon S3 Developer Guide</a>.</p> </important></p>
    pub kms_key_arn: Option<String>,
    /// <p>The key prefix of the Amazon S3 bucket. The key prefix is similar to a directory name that enables you to store similar data under the same directory in a bucket.</p>
    pub object_key_prefix: Option<String>,
    /// <p>The ARN of the Amazon SNS topic to notify when the message is saved to the Amazon S3 bucket. An example of an Amazon SNS topic ARN is <code>arn:aws:sns:us-west-2:123456789012:MyTopic</code>. For more information about Amazon SNS topics, see the <a href="https://docs.aws.amazon.com/sns/latest/dg/CreateTopic.html">Amazon SNS Developer Guide</a>.</p>
    pub topic_arn: Option<String>,
}

struct S3ActionDeserializer;
impl S3ActionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<S3Action, XmlParseError> {
        deserialize_elements::<_, S3Action, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "BucketName" => {
                    obj.bucket_name = S3BucketNameDeserializer::deserialize("BucketName", stack)?;
                }
                "KmsKeyArn" => {
                    obj.kms_key_arn = Some(AmazonResourceNameDeserializer::deserialize(
                        "KmsKeyArn",
                        stack,
                    )?);
                }
                "ObjectKeyPrefix" => {
                    obj.object_key_prefix = Some(S3KeyPrefixDeserializer::deserialize(
                        "ObjectKeyPrefix",
                        stack,
                    )?);
                }
                "TopicArn" => {
                    obj.topic_arn = Some(AmazonResourceNameDeserializer::deserialize(
                        "TopicArn", stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

/// Serialize `S3Action` contents to a `SignedRequest`.
struct S3ActionSerializer;
impl S3ActionSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &S3Action) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "BucketName"), &obj.bucket_name);
        if let Some(ref field_value) = obj.kms_key_arn {
            params.put(&format!("{}{}", prefix, "KmsKeyArn"), &field_value);
        }
        if let Some(ref field_value) = obj.object_key_prefix {
            params.put(&format!("{}{}", prefix, "ObjectKeyPrefix"), &field_value);
        }
        if let Some(ref field_value) = obj.topic_arn {
            params.put(&format!("{}{}", prefix, "TopicArn"), &field_value);
        }
    }
}

struct S3BucketNameDeserializer;
impl S3BucketNameDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct S3KeyPrefixDeserializer;
impl S3KeyPrefixDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>When included in a receipt rule, this action publishes a notification to Amazon Simple Notification Service (Amazon SNS). This action includes a complete copy of the email content in the Amazon SNS notifications. Amazon SNS notifications for all other actions simply provide information about the email. They do not include the email content itself.</p> <p>If you own the Amazon SNS topic, you don't need to do anything to give Amazon SES permission to publish emails to it. However, if you don't own the Amazon SNS topic, you need to attach a policy to the topic to give Amazon SES permissions to access it. For information about giving permissions, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-permissions.html">Amazon SES Developer Guide</a>.</p> <important> <p>You can only publish emails that are 150 KB or less (including the header) to Amazon SNS. Larger emails will bounce. If you anticipate emails larger than 150 KB, use the S3 action instead.</p> </important> <p>For information about using a receipt rule to publish an Amazon SNS notification, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-action-sns.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SNSAction {
    /// <p>The encoding to use for the email within the Amazon SNS notification. UTF-8 is easier to use, but may not preserve all special characters when a message was encoded with a different encoding format. Base64 preserves all special characters. The default value is UTF-8.</p>
    pub encoding: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the Amazon SNS topic to notify. An example of an Amazon SNS topic ARN is <code>arn:aws:sns:us-west-2:123456789012:MyTopic</code>. For more information about Amazon SNS topics, see the <a href="https://docs.aws.amazon.com/sns/latest/dg/CreateTopic.html">Amazon SNS Developer Guide</a>.</p>
    pub topic_arn: String,
}

struct SNSActionDeserializer;
impl SNSActionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<SNSAction, XmlParseError> {
        deserialize_elements::<_, SNSAction, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Encoding" => {
                    obj.encoding = Some(SNSActionEncodingDeserializer::deserialize(
                        "Encoding", stack,
                    )?);
                }
                "TopicArn" => {
                    obj.topic_arn = AmazonResourceNameDeserializer::deserialize("TopicArn", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

/// Serialize `SNSAction` contents to a `SignedRequest`.
struct SNSActionSerializer;
impl SNSActionSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &SNSAction) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.encoding {
            params.put(&format!("{}{}", prefix, "Encoding"), &field_value);
        }
        params.put(&format!("{}{}", prefix, "TopicArn"), &obj.topic_arn);
    }
}

struct SNSActionEncodingDeserializer;
impl SNSActionEncodingDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Contains the topic ARN associated with an Amazon Simple Notification Service (Amazon SNS) event destination.</p> <p>Event destinations, such as Amazon SNS, are associated with configuration sets, which enable you to publish email sending events. For information about using configuration sets, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/monitor-sending-activity.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SNSDestination {
    /// <p>The ARN of the Amazon SNS topic that email sending events will be published to. An example of an Amazon SNS topic ARN is <code>arn:aws:sns:us-west-2:123456789012:MyTopic</code>. For more information about Amazon SNS topics, see the <a href="https://docs.aws.amazon.com/sns/latest/dg/CreateTopic.html">Amazon SNS Developer Guide</a>.</p>
    pub topic_arn: String,
}

struct SNSDestinationDeserializer;
impl SNSDestinationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<SNSDestination, XmlParseError> {
        deserialize_elements::<_, SNSDestination, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "TopicARN" => {
                    obj.topic_arn = AmazonResourceNameDeserializer::deserialize("TopicARN", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

/// Serialize `SNSDestination` contents to a `SignedRequest`.
struct SNSDestinationSerializer;
impl SNSDestinationSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &SNSDestination) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "TopicARN"), &obj.topic_arn);
    }
}

/// <p>Represents a request to send a bounce message to the sender of an email you received through Amazon SES.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SendBounceRequest {
    /// <p>The address to use in the "From" header of the bounce message. This must be an identity that you have verified with Amazon SES.</p>
    pub bounce_sender: String,
    /// <p>This parameter is used only for sending authorization. It is the ARN of the identity that is associated with the sending authorization policy that permits you to use the address in the "From" header of the bounce. For more information about sending authorization, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/sending-authorization.html">Amazon SES Developer Guide</a>.</p>
    pub bounce_sender_arn: Option<String>,
    /// <p>A list of recipients of the bounced message, including the information required to create the Delivery Status Notifications (DSNs) for the recipients. You must specify at least one <code>BouncedRecipientInfo</code> in the list.</p>
    pub bounced_recipient_info_list: Vec<BouncedRecipientInfo>,
    /// <p>Human-readable text for the bounce message to explain the failure. If not specified, the text will be auto-generated based on the bounced recipient information.</p>
    pub explanation: Option<String>,
    /// <p>Message-related DSN fields. If not specified, Amazon SES will choose the values.</p>
    pub message_dsn: Option<MessageDsn>,
    /// <p>The message ID of the message to be bounced.</p>
    pub original_message_id: String,
}

/// Serialize `SendBounceRequest` contents to a `SignedRequest`.
struct SendBounceRequestSerializer;
impl SendBounceRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &SendBounceRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "BounceSender"), &obj.bounce_sender);
        if let Some(ref field_value) = obj.bounce_sender_arn {
            params.put(&format!("{}{}", prefix, "BounceSenderArn"), &field_value);
        }
        BouncedRecipientInfoListSerializer::serialize(
            params,
            &format!("{}{}", prefix, "BouncedRecipientInfoList"),
            &obj.bounced_recipient_info_list,
        );
        if let Some(ref field_value) = obj.explanation {
            params.put(&format!("{}{}", prefix, "Explanation"), &field_value);
        }
        if let Some(ref field_value) = obj.message_dsn {
            MessageDsnSerializer::serialize(
                params,
                &format!("{}{}", prefix, "MessageDsn"),
                field_value,
            );
        }
        params.put(
            &format!("{}{}", prefix, "OriginalMessageId"),
            &obj.original_message_id,
        );
    }
}

/// <p>Represents a unique message ID.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct SendBounceResponse {
    /// <p>The message ID of the bounce message.</p>
    pub message_id: Option<String>,
}

struct SendBounceResponseDeserializer;
impl SendBounceResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<SendBounceResponse, XmlParseError> {
        deserialize_elements::<_, SendBounceResponse, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "MessageId" => {
                    obj.message_id = Some(MessageIdDeserializer::deserialize("MessageId", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
/// <p>Represents a request to send a templated email to multiple destinations using Amazon SES. For more information, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/send-personalized-email-api.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SendBulkTemplatedEmailRequest {
    /// <p>The name of the configuration set to use when you send an email using <code>SendBulkTemplatedEmail</code>.</p>
    pub configuration_set_name: Option<String>,
    /// <p>A list of tags, in the form of name/value pairs, to apply to an email that you send to a destination using <code>SendBulkTemplatedEmail</code>.</p>
    pub default_tags: Option<Vec<MessageTag>>,
    /// <p>A list of replacement values to apply to the template when replacement data is not specified in a Destination object. These values act as a default or fallback option when no other data is available.</p> <p>The template data is a JSON object, typically consisting of key-value pairs in which the keys correspond to replacement tags in the email template.</p>
    pub default_template_data: Option<String>,
    /// <p>One or more <code>Destination</code> objects. All of the recipients in a <code>Destination</code> will receive the same version of the email. You can specify up to 50 <code>Destination</code> objects within a <code>Destinations</code> array.</p>
    pub destinations: Vec<BulkEmailDestination>,
    /// <p>The reply-to email address(es) for the message. If the recipient replies to the message, each reply-to address will receive the reply.</p>
    pub reply_to_addresses: Option<Vec<String>>,
    /// <p>The email address that bounces and complaints will be forwarded to when feedback forwarding is enabled. If the message cannot be delivered to the recipient, then an error message will be returned from the recipient's ISP; this message will then be forwarded to the email address specified by the <code>ReturnPath</code> parameter. The <code>ReturnPath</code> parameter is never overwritten. This email address must be either individually verified with Amazon SES, or from a domain that has been verified with Amazon SES. </p>
    pub return_path: Option<String>,
    /// <p>This parameter is used only for sending authorization. It is the ARN of the identity that is associated with the sending authorization policy that permits you to use the email address specified in the <code>ReturnPath</code> parameter.</p> <p>For example, if the owner of <code>example.com</code> (which has ARN <code>arn:aws:ses:us-east-1:123456789012:identity/example.com</code>) attaches a policy to it that authorizes you to use <code>feedback@example.com</code>, then you would specify the <code>ReturnPathArn</code> to be <code>arn:aws:ses:us-east-1:123456789012:identity/example.com</code>, and the <code>ReturnPath</code> to be <code>feedback@example.com</code>.</p> <p>For more information about sending authorization, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/sending-authorization.html">Amazon SES Developer Guide</a>.</p>
    pub return_path_arn: Option<String>,
    /// <p><p>The email address that is sending the email. This email address must be either individually verified with Amazon SES, or from a domain that has been verified with Amazon SES. For information about verifying identities, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/verify-addresses-and-domains.html">Amazon SES Developer Guide</a>.</p> <p>If you are sending on behalf of another user and have been permitted to do so by a sending authorization policy, then you must also specify the <code>SourceArn</code> parameter. For more information about sending authorization, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/sending-authorization.html">Amazon SES Developer Guide</a>.</p> <note> <p>Amazon SES does not support the SMTPUTF8 extension, as described in <a href="https://tools.ietf.org/html/rfc6531">RFC6531</a>. For this reason, the <i>local part</i> of a source email address (the part of the email address that precedes the @ sign) may only contain <a href="https://en.wikipedia.org/wiki/Email_address#Local-part">7-bit ASCII characters</a>. If the <i>domain part</i> of an address (the part after the @ sign) contains non-ASCII characters, they must be encoded using Punycode, as described in <a href="https://tools.ietf.org/html/rfc3492.html">RFC3492</a>. The sender name (also known as the <i>friendly name</i>) may contain non-ASCII characters. These characters must be encoded using MIME encoded-word syntax, as described in <a href="https://tools.ietf.org/html/rfc2047">RFC 2047</a>. MIME encoded-word syntax uses the following form: <code>=?charset?encoding?encoded-text?=</code>.</p> </note></p>
    pub source: String,
    /// <p>This parameter is used only for sending authorization. It is the ARN of the identity that is associated with the sending authorization policy that permits you to send for the email address specified in the <code>Source</code> parameter.</p> <p>For example, if the owner of <code>example.com</code> (which has ARN <code>arn:aws:ses:us-east-1:123456789012:identity/example.com</code>) attaches a policy to it that authorizes you to send from <code>user@example.com</code>, then you would specify the <code>SourceArn</code> to be <code>arn:aws:ses:us-east-1:123456789012:identity/example.com</code>, and the <code>Source</code> to be <code>user@example.com</code>.</p> <p>For more information about sending authorization, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/sending-authorization.html">Amazon SES Developer Guide</a>.</p>
    pub source_arn: Option<String>,
    /// <p>The template to use when sending this email.</p>
    pub template: String,
    /// <p>The ARN of the template to use when sending this email.</p>
    pub template_arn: Option<String>,
}

/// Serialize `SendBulkTemplatedEmailRequest` contents to a `SignedRequest`.
struct SendBulkTemplatedEmailRequestSerializer;
impl SendBulkTemplatedEmailRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &SendBulkTemplatedEmailRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.configuration_set_name {
            params.put(
                &format!("{}{}", prefix, "ConfigurationSetName"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.default_tags {
            MessageTagListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "DefaultTags"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.default_template_data {
            params.put(
                &format!("{}{}", prefix, "DefaultTemplateData"),
                &field_value,
            );
        }
        BulkEmailDestinationListSerializer::serialize(
            params,
            &format!("{}{}", prefix, "Destinations"),
            &obj.destinations,
        );
        if let Some(ref field_value) = obj.reply_to_addresses {
            AddressListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "ReplyToAddresses"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.return_path {
            params.put(&format!("{}{}", prefix, "ReturnPath"), &field_value);
        }
        if let Some(ref field_value) = obj.return_path_arn {
            params.put(&format!("{}{}", prefix, "ReturnPathArn"), &field_value);
        }
        params.put(&format!("{}{}", prefix, "Source"), &obj.source);
        if let Some(ref field_value) = obj.source_arn {
            params.put(&format!("{}{}", prefix, "SourceArn"), &field_value);
        }
        params.put(&format!("{}{}", prefix, "Template"), &obj.template);
        if let Some(ref field_value) = obj.template_arn {
            params.put(&format!("{}{}", prefix, "TemplateArn"), &field_value);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct SendBulkTemplatedEmailResponse {
    /// <p>The unique message identifier returned from the <code>SendBulkTemplatedEmail</code> action.</p>
    pub status: Vec<BulkEmailDestinationStatus>,
}

struct SendBulkTemplatedEmailResponseDeserializer;
impl SendBulkTemplatedEmailResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<SendBulkTemplatedEmailResponse, XmlParseError> {
        deserialize_elements::<_, SendBulkTemplatedEmailResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Status" => {
                        obj.status
                            .extend(BulkEmailDestinationStatusListDeserializer::deserialize(
                                "Status", stack,
                            )?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p>Represents a request to send a custom verification email to a specified recipient.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SendCustomVerificationEmailRequest {
    /// <p>Name of a configuration set to use when sending the verification email.</p>
    pub configuration_set_name: Option<String>,
    /// <p>The email address to verify.</p>
    pub email_address: String,
    /// <p>The name of the custom verification email template to use when sending the verification email.</p>
    pub template_name: String,
}

/// Serialize `SendCustomVerificationEmailRequest` contents to a `SignedRequest`.
struct SendCustomVerificationEmailRequestSerializer;
impl SendCustomVerificationEmailRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &SendCustomVerificationEmailRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.configuration_set_name {
            params.put(
                &format!("{}{}", prefix, "ConfigurationSetName"),
                &field_value,
            );
        }
        params.put(&format!("{}{}", prefix, "EmailAddress"), &obj.email_address);
        params.put(&format!("{}{}", prefix, "TemplateName"), &obj.template_name);
    }
}

/// <p>The response received when attempting to send the custom verification email.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct SendCustomVerificationEmailResponse {
    /// <p>The unique message identifier returned from the <code>SendCustomVerificationEmail</code> operation.</p>
    pub message_id: Option<String>,
}

struct SendCustomVerificationEmailResponseDeserializer;
impl SendCustomVerificationEmailResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<SendCustomVerificationEmailResponse, XmlParseError> {
        deserialize_elements::<_, SendCustomVerificationEmailResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "MessageId" => {
                        obj.message_id =
                            Some(MessageIdDeserializer::deserialize("MessageId", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p>Represents sending statistics data. Each <code>SendDataPoint</code> contains statistics for a 15-minute period of sending activity. </p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct SendDataPoint {
    /// <p>Number of emails that have bounced.</p>
    pub bounces: Option<i64>,
    /// <p>Number of unwanted emails that were rejected by recipients.</p>
    pub complaints: Option<i64>,
    /// <p>Number of emails that have been sent.</p>
    pub delivery_attempts: Option<i64>,
    /// <p>Number of emails rejected by Amazon SES.</p>
    pub rejects: Option<i64>,
    /// <p>Time of the data point.</p>
    pub timestamp: Option<String>,
}

struct SendDataPointDeserializer;
impl SendDataPointDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<SendDataPoint, XmlParseError> {
        deserialize_elements::<_, SendDataPoint, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Bounces" => {
                    obj.bounces = Some(CounterDeserializer::deserialize("Bounces", stack)?);
                }
                "Complaints" => {
                    obj.complaints = Some(CounterDeserializer::deserialize("Complaints", stack)?);
                }
                "DeliveryAttempts" => {
                    obj.delivery_attempts =
                        Some(CounterDeserializer::deserialize("DeliveryAttempts", stack)?);
                }
                "Rejects" => {
                    obj.rejects = Some(CounterDeserializer::deserialize("Rejects", stack)?);
                }
                "Timestamp" => {
                    obj.timestamp = Some(TimestampDeserializer::deserialize("Timestamp", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct SendDataPointListDeserializer;
impl SendDataPointListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<SendDataPoint>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(SendDataPointDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
/// <p>Represents a request to send a single formatted email using Amazon SES. For more information, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/send-email-formatted.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SendEmailRequest {
    /// <p>The name of the configuration set to use when you send an email using <code>SendEmail</code>.</p>
    pub configuration_set_name: Option<String>,
    /// <p>The destination for this email, composed of To:, CC:, and BCC: fields.</p>
    pub destination: Destination,
    /// <p>The message to be sent.</p>
    pub message: Message,
    /// <p>The reply-to email address(es) for the message. If the recipient replies to the message, each reply-to address will receive the reply.</p>
    pub reply_to_addresses: Option<Vec<String>>,
    /// <p>The email address that bounces and complaints will be forwarded to when feedback forwarding is enabled. If the message cannot be delivered to the recipient, then an error message will be returned from the recipient's ISP; this message will then be forwarded to the email address specified by the <code>ReturnPath</code> parameter. The <code>ReturnPath</code> parameter is never overwritten. This email address must be either individually verified with Amazon SES, or from a domain that has been verified with Amazon SES. </p>
    pub return_path: Option<String>,
    /// <p>This parameter is used only for sending authorization. It is the ARN of the identity that is associated with the sending authorization policy that permits you to use the email address specified in the <code>ReturnPath</code> parameter.</p> <p>For example, if the owner of <code>example.com</code> (which has ARN <code>arn:aws:ses:us-east-1:123456789012:identity/example.com</code>) attaches a policy to it that authorizes you to use <code>feedback@example.com</code>, then you would specify the <code>ReturnPathArn</code> to be <code>arn:aws:ses:us-east-1:123456789012:identity/example.com</code>, and the <code>ReturnPath</code> to be <code>feedback@example.com</code>.</p> <p>For more information about sending authorization, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/sending-authorization.html">Amazon SES Developer Guide</a>.</p>
    pub return_path_arn: Option<String>,
    /// <p><p>The email address that is sending the email. This email address must be either individually verified with Amazon SES, or from a domain that has been verified with Amazon SES. For information about verifying identities, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/verify-addresses-and-domains.html">Amazon SES Developer Guide</a>.</p> <p>If you are sending on behalf of another user and have been permitted to do so by a sending authorization policy, then you must also specify the <code>SourceArn</code> parameter. For more information about sending authorization, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/sending-authorization.html">Amazon SES Developer Guide</a>.</p> <note> <p>Amazon SES does not support the SMTPUTF8 extension, as described in <a href="https://tools.ietf.org/html/rfc6531">RFC6531</a>. For this reason, the <i>local part</i> of a source email address (the part of the email address that precedes the @ sign) may only contain <a href="https://en.wikipedia.org/wiki/Email_address#Local-part">7-bit ASCII characters</a>. If the <i>domain part</i> of an address (the part after the @ sign) contains non-ASCII characters, they must be encoded using Punycode, as described in <a href="https://tools.ietf.org/html/rfc3492.html">RFC3492</a>. The sender name (also known as the <i>friendly name</i>) may contain non-ASCII characters. These characters must be encoded using MIME encoded-word syntax, as described in <a href="https://tools.ietf.org/html/rfc2047">RFC 2047</a>. MIME encoded-word syntax uses the following form: <code>=?charset?encoding?encoded-text?=</code>.</p> </note></p>
    pub source: String,
    /// <p>This parameter is used only for sending authorization. It is the ARN of the identity that is associated with the sending authorization policy that permits you to send for the email address specified in the <code>Source</code> parameter.</p> <p>For example, if the owner of <code>example.com</code> (which has ARN <code>arn:aws:ses:us-east-1:123456789012:identity/example.com</code>) attaches a policy to it that authorizes you to send from <code>user@example.com</code>, then you would specify the <code>SourceArn</code> to be <code>arn:aws:ses:us-east-1:123456789012:identity/example.com</code>, and the <code>Source</code> to be <code>user@example.com</code>.</p> <p>For more information about sending authorization, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/sending-authorization.html">Amazon SES Developer Guide</a>.</p>
    pub source_arn: Option<String>,
    /// <p>A list of tags, in the form of name/value pairs, to apply to an email that you send using <code>SendEmail</code>. Tags correspond to characteristics of the email that you define, so that you can publish email sending events.</p>
    pub tags: Option<Vec<MessageTag>>,
}

/// Serialize `SendEmailRequest` contents to a `SignedRequest`.
struct SendEmailRequestSerializer;
impl SendEmailRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &SendEmailRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.configuration_set_name {
            params.put(
                &format!("{}{}", prefix, "ConfigurationSetName"),
                &field_value,
            );
        }
        DestinationSerializer::serialize(
            params,
            &format!("{}{}", prefix, "Destination"),
            &obj.destination,
        );
        MessageSerializer::serialize(params, &format!("{}{}", prefix, "Message"), &obj.message);
        if let Some(ref field_value) = obj.reply_to_addresses {
            AddressListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "ReplyToAddresses"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.return_path {
            params.put(&format!("{}{}", prefix, "ReturnPath"), &field_value);
        }
        if let Some(ref field_value) = obj.return_path_arn {
            params.put(&format!("{}{}", prefix, "ReturnPathArn"), &field_value);
        }
        params.put(&format!("{}{}", prefix, "Source"), &obj.source);
        if let Some(ref field_value) = obj.source_arn {
            params.put(&format!("{}{}", prefix, "SourceArn"), &field_value);
        }
        if let Some(ref field_value) = obj.tags {
            MessageTagListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Tags"),
                field_value,
            );
        }
    }
}

/// <p>Represents a unique message ID.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct SendEmailResponse {
    /// <p>The unique message identifier returned from the <code>SendEmail</code> action. </p>
    pub message_id: String,
}

struct SendEmailResponseDeserializer;
impl SendEmailResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<SendEmailResponse, XmlParseError> {
        deserialize_elements::<_, SendEmailResponse, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "MessageId" => {
                    obj.message_id = MessageIdDeserializer::deserialize("MessageId", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
/// <p>Represents a request to send a single raw email using Amazon SES. For more information, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/send-email-raw.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SendRawEmailRequest {
    /// <p>The name of the configuration set to use when you send an email using <code>SendRawEmail</code>.</p>
    pub configuration_set_name: Option<String>,
    /// <p>A list of destinations for the message, consisting of To:, CC:, and BCC: addresses.</p>
    pub destinations: Option<Vec<String>>,
    /// <p><p>This parameter is used only for sending authorization. It is the ARN of the identity that is associated with the sending authorization policy that permits you to specify a particular &quot;From&quot; address in the header of the raw email.</p> <p>Instead of using this parameter, you can use the X-header <code>X-SES-FROM-ARN</code> in the raw message of the email. If you use both the <code>FromArn</code> parameter and the corresponding X-header, Amazon SES uses the value of the <code>FromArn</code> parameter.</p> <note> <p>For information about when to use this parameter, see the description of <code>SendRawEmail</code> in this guide, or see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/sending-authorization-delegate-sender-tasks-email.html">Amazon SES Developer Guide</a>.</p> </note></p>
    pub from_arn: Option<String>,
    /// <p><p>The raw email message itself. The message has to meet the following criteria:</p> <ul> <li> <p>The message has to contain a header and a body, separated by a blank line.</p> </li> <li> <p>All of the required header fields must be present in the message.</p> </li> <li> <p>Each part of a multipart MIME message must be formatted properly.</p> </li> <li> <p>Attachments must be of a content type that Amazon SES supports. For a list on unsupported content types, see <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/mime-types.html">Unsupported Attachment Types</a> in the <i>Amazon SES Developer Guide</i>.</p> </li> <li> <p>The entire message must be base64-encoded.</p> </li> <li> <p>If any of the MIME parts in your message contain content that is outside of the 7-bit ASCII character range, we highly recommend that you encode that content. For more information, see <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/send-email-raw.html">Sending Raw Email</a> in the <i>Amazon SES Developer Guide</i>.</p> </li> <li> <p>Per <a href="https://tools.ietf.org/html/rfc5321#section-4.5.3.1.6">RFC 5321</a>, the maximum length of each line of text, including the &lt;CRLF&gt;, must not exceed 1,000 characters.</p> </li> </ul></p>
    pub raw_message: RawMessage,
    /// <p><p>This parameter is used only for sending authorization. It is the ARN of the identity that is associated with the sending authorization policy that permits you to use the email address specified in the <code>ReturnPath</code> parameter.</p> <p>For example, if the owner of <code>example.com</code> (which has ARN <code>arn:aws:ses:us-east-1:123456789012:identity/example.com</code>) attaches a policy to it that authorizes you to use <code>feedback@example.com</code>, then you would specify the <code>ReturnPathArn</code> to be <code>arn:aws:ses:us-east-1:123456789012:identity/example.com</code>, and the <code>ReturnPath</code> to be <code>feedback@example.com</code>.</p> <p>Instead of using this parameter, you can use the X-header <code>X-SES-RETURN-PATH-ARN</code> in the raw message of the email. If you use both the <code>ReturnPathArn</code> parameter and the corresponding X-header, Amazon SES uses the value of the <code>ReturnPathArn</code> parameter.</p> <note> <p>For information about when to use this parameter, see the description of <code>SendRawEmail</code> in this guide, or see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/sending-authorization-delegate-sender-tasks-email.html">Amazon SES Developer Guide</a>.</p> </note></p>
    pub return_path_arn: Option<String>,
    /// <p>The identity's email address. If you do not provide a value for this parameter, you must specify a "From" address in the raw text of the message. (You can also specify both.)</p> <note> <p>Amazon SES does not support the SMTPUTF8 extension, as described in<a href="https://tools.ietf.org/html/rfc6531">RFC6531</a>. For this reason, the <i>local part</i> of a source email address (the part of the email address that precedes the @ sign) may only contain <a href="https://en.wikipedia.org/wiki/Email_address#Local-part">7-bit ASCII characters</a>. If the <i>domain part</i> of an address (the part after the @ sign) contains non-ASCII characters, they must be encoded using Punycode, as described in <a href="https://tools.ietf.org/html/rfc3492.html">RFC3492</a>. The sender name (also known as the <i>friendly name</i>) may contain non-ASCII characters. These characters must be encoded using MIME encoded-word syntax, as described in <a href="https://tools.ietf.org/html/rfc2047">RFC 2047</a>. MIME encoded-word syntax uses the following form: <code>=?charset?encoding?encoded-text?=</code>.</p> </note> <p>If you specify the <code>Source</code> parameter and have feedback forwarding enabled, then bounces and complaints will be sent to this email address. This takes precedence over any Return-Path header that you might include in the raw text of the message.</p>
    pub source: Option<String>,
    /// <p><p>This parameter is used only for sending authorization. It is the ARN of the identity that is associated with the sending authorization policy that permits you to send for the email address specified in the <code>Source</code> parameter.</p> <p>For example, if the owner of <code>example.com</code> (which has ARN <code>arn:aws:ses:us-east-1:123456789012:identity/example.com</code>) attaches a policy to it that authorizes you to send from <code>user@example.com</code>, then you would specify the <code>SourceArn</code> to be <code>arn:aws:ses:us-east-1:123456789012:identity/example.com</code>, and the <code>Source</code> to be <code>user@example.com</code>.</p> <p>Instead of using this parameter, you can use the X-header <code>X-SES-SOURCE-ARN</code> in the raw message of the email. If you use both the <code>SourceArn</code> parameter and the corresponding X-header, Amazon SES uses the value of the <code>SourceArn</code> parameter.</p> <note> <p>For information about when to use this parameter, see the description of <code>SendRawEmail</code> in this guide, or see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/sending-authorization-delegate-sender-tasks-email.html">Amazon SES Developer Guide</a>.</p> </note></p>
    pub source_arn: Option<String>,
    /// <p>A list of tags, in the form of name/value pairs, to apply to an email that you send using <code>SendRawEmail</code>. Tags correspond to characteristics of the email that you define, so that you can publish email sending events.</p>
    pub tags: Option<Vec<MessageTag>>,
}

/// Serialize `SendRawEmailRequest` contents to a `SignedRequest`.
struct SendRawEmailRequestSerializer;
impl SendRawEmailRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &SendRawEmailRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.configuration_set_name {
            params.put(
                &format!("{}{}", prefix, "ConfigurationSetName"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.destinations {
            AddressListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Destinations"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.from_arn {
            params.put(&format!("{}{}", prefix, "FromArn"), &field_value);
        }
        RawMessageSerializer::serialize(
            params,
            &format!("{}{}", prefix, "RawMessage"),
            &obj.raw_message,
        );
        if let Some(ref field_value) = obj.return_path_arn {
            params.put(&format!("{}{}", prefix, "ReturnPathArn"), &field_value);
        }
        if let Some(ref field_value) = obj.source {
            params.put(&format!("{}{}", prefix, "Source"), &field_value);
        }
        if let Some(ref field_value) = obj.source_arn {
            params.put(&format!("{}{}", prefix, "SourceArn"), &field_value);
        }
        if let Some(ref field_value) = obj.tags {
            MessageTagListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Tags"),
                field_value,
            );
        }
    }
}

/// <p>Represents a unique message ID.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct SendRawEmailResponse {
    /// <p>The unique message identifier returned from the <code>SendRawEmail</code> action. </p>
    pub message_id: String,
}

struct SendRawEmailResponseDeserializer;
impl SendRawEmailResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<SendRawEmailResponse, XmlParseError> {
        deserialize_elements::<_, SendRawEmailResponse, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "MessageId" => {
                    obj.message_id = MessageIdDeserializer::deserialize("MessageId", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
/// <p>Represents a request to send a templated email using Amazon SES. For more information, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/send-personalized-email-api.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SendTemplatedEmailRequest {
    /// <p>The name of the configuration set to use when you send an email using <code>SendTemplatedEmail</code>.</p>
    pub configuration_set_name: Option<String>,
    /// <p>The destination for this email, composed of To:, CC:, and BCC: fields. A Destination can include up to 50 recipients across these three fields.</p>
    pub destination: Destination,
    /// <p>The reply-to email address(es) for the message. If the recipient replies to the message, each reply-to address will receive the reply.</p>
    pub reply_to_addresses: Option<Vec<String>>,
    /// <p>The email address that bounces and complaints will be forwarded to when feedback forwarding is enabled. If the message cannot be delivered to the recipient, then an error message will be returned from the recipient's ISP; this message will then be forwarded to the email address specified by the <code>ReturnPath</code> parameter. The <code>ReturnPath</code> parameter is never overwritten. This email address must be either individually verified with Amazon SES, or from a domain that has been verified with Amazon SES. </p>
    pub return_path: Option<String>,
    /// <p>This parameter is used only for sending authorization. It is the ARN of the identity that is associated with the sending authorization policy that permits you to use the email address specified in the <code>ReturnPath</code> parameter.</p> <p>For example, if the owner of <code>example.com</code> (which has ARN <code>arn:aws:ses:us-east-1:123456789012:identity/example.com</code>) attaches a policy to it that authorizes you to use <code>feedback@example.com</code>, then you would specify the <code>ReturnPathArn</code> to be <code>arn:aws:ses:us-east-1:123456789012:identity/example.com</code>, and the <code>ReturnPath</code> to be <code>feedback@example.com</code>.</p> <p>For more information about sending authorization, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/sending-authorization.html">Amazon SES Developer Guide</a>.</p>
    pub return_path_arn: Option<String>,
    /// <p><p>The email address that is sending the email. This email address must be either individually verified with Amazon SES, or from a domain that has been verified with Amazon SES. For information about verifying identities, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/verify-addresses-and-domains.html">Amazon SES Developer Guide</a>.</p> <p>If you are sending on behalf of another user and have been permitted to do so by a sending authorization policy, then you must also specify the <code>SourceArn</code> parameter. For more information about sending authorization, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/sending-authorization.html">Amazon SES Developer Guide</a>.</p> <note> <p>Amazon SES does not support the SMTPUTF8 extension, as described in <a href="https://tools.ietf.org/html/rfc6531">RFC6531</a>. For this reason, the <i>local part</i> of a source email address (the part of the email address that precedes the @ sign) may only contain <a href="https://en.wikipedia.org/wiki/Email_address#Local-part">7-bit ASCII characters</a>. If the <i>domain part</i> of an address (the part after the @ sign) contains non-ASCII characters, they must be encoded using Punycode, as described in <a href="https://tools.ietf.org/html/rfc3492.html">RFC3492</a>. The sender name (also known as the <i>friendly name</i>) may contain non-ASCII characters. These characters must be encoded using MIME encoded-word syntax, as described in<a href="https://tools.ietf.org/html/rfc2047">RFC 2047</a>. MIME encoded-word syntax uses the following form: <code>=?charset?encoding?encoded-text?=</code>.</p> </note></p>
    pub source: String,
    /// <p>This parameter is used only for sending authorization. It is the ARN of the identity that is associated with the sending authorization policy that permits you to send for the email address specified in the <code>Source</code> parameter.</p> <p>For example, if the owner of <code>example.com</code> (which has ARN <code>arn:aws:ses:us-east-1:123456789012:identity/example.com</code>) attaches a policy to it that authorizes you to send from <code>user@example.com</code>, then you would specify the <code>SourceArn</code> to be <code>arn:aws:ses:us-east-1:123456789012:identity/example.com</code>, and the <code>Source</code> to be <code>user@example.com</code>.</p> <p>For more information about sending authorization, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/sending-authorization.html">Amazon SES Developer Guide</a>.</p>
    pub source_arn: Option<String>,
    /// <p>A list of tags, in the form of name/value pairs, to apply to an email that you send using <code>SendTemplatedEmail</code>. Tags correspond to characteristics of the email that you define, so that you can publish email sending events.</p>
    pub tags: Option<Vec<MessageTag>>,
    /// <p>The template to use when sending this email.</p>
    pub template: String,
    /// <p>The ARN of the template to use when sending this email.</p>
    pub template_arn: Option<String>,
    /// <p>A list of replacement values to apply to the template. This parameter is a JSON object, typically consisting of key-value pairs in which the keys correspond to replacement tags in the email template.</p>
    pub template_data: String,
}

/// Serialize `SendTemplatedEmailRequest` contents to a `SignedRequest`.
struct SendTemplatedEmailRequestSerializer;
impl SendTemplatedEmailRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &SendTemplatedEmailRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.configuration_set_name {
            params.put(
                &format!("{}{}", prefix, "ConfigurationSetName"),
                &field_value,
            );
        }
        DestinationSerializer::serialize(
            params,
            &format!("{}{}", prefix, "Destination"),
            &obj.destination,
        );
        if let Some(ref field_value) = obj.reply_to_addresses {
            AddressListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "ReplyToAddresses"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.return_path {
            params.put(&format!("{}{}", prefix, "ReturnPath"), &field_value);
        }
        if let Some(ref field_value) = obj.return_path_arn {
            params.put(&format!("{}{}", prefix, "ReturnPathArn"), &field_value);
        }
        params.put(&format!("{}{}", prefix, "Source"), &obj.source);
        if let Some(ref field_value) = obj.source_arn {
            params.put(&format!("{}{}", prefix, "SourceArn"), &field_value);
        }
        if let Some(ref field_value) = obj.tags {
            MessageTagListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Tags"),
                field_value,
            );
        }
        params.put(&format!("{}{}", prefix, "Template"), &obj.template);
        if let Some(ref field_value) = obj.template_arn {
            params.put(&format!("{}{}", prefix, "TemplateArn"), &field_value);
        }
        params.put(&format!("{}{}", prefix, "TemplateData"), &obj.template_data);
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct SendTemplatedEmailResponse {
    /// <p>The unique message identifier returned from the <code>SendTemplatedEmail</code> action. </p>
    pub message_id: String,
}

struct SendTemplatedEmailResponseDeserializer;
impl SendTemplatedEmailResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<SendTemplatedEmailResponse, XmlParseError> {
        deserialize_elements::<_, SendTemplatedEmailResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "MessageId" => {
                        obj.message_id = MessageIdDeserializer::deserialize("MessageId", stack)?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
struct SentLast24HoursDeserializer;
impl SentLast24HoursDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<f64, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = f64::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Represents a request to set a receipt rule set as the active receipt rule set. You use receipt rule sets to receive email with Amazon SES. For more information, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-concepts.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SetActiveReceiptRuleSetRequest {
    /// <p>The name of the receipt rule set to make active. Setting this value to null disables all email receiving.</p>
    pub rule_set_name: Option<String>,
}

/// Serialize `SetActiveReceiptRuleSetRequest` contents to a `SignedRequest`.
struct SetActiveReceiptRuleSetRequestSerializer;
impl SetActiveReceiptRuleSetRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &SetActiveReceiptRuleSetRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.rule_set_name {
            params.put(&format!("{}{}", prefix, "RuleSetName"), &field_value);
        }
    }
}

/// <p>An empty element returned on a successful request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct SetActiveReceiptRuleSetResponse {}

struct SetActiveReceiptRuleSetResponseDeserializer;
impl SetActiveReceiptRuleSetResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<SetActiveReceiptRuleSetResponse, XmlParseError> {
        start_element(tag_name, stack)?;

        let obj = SetActiveReceiptRuleSetResponse::default();

        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Represents a request to enable or disable Amazon SES Easy DKIM signing for an identity. For more information about setting up Easy DKIM, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/easy-dkim.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SetIdentityDkimEnabledRequest {
    /// <p>Sets whether DKIM signing is enabled for an identity. Set to <code>true</code> to enable DKIM signing for this identity; <code>false</code> to disable it. </p>
    pub dkim_enabled: bool,
    /// <p>The identity for which DKIM signing should be enabled or disabled.</p>
    pub identity: String,
}

/// Serialize `SetIdentityDkimEnabledRequest` contents to a `SignedRequest`.
struct SetIdentityDkimEnabledRequestSerializer;
impl SetIdentityDkimEnabledRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &SetIdentityDkimEnabledRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "DkimEnabled"), &obj.dkim_enabled);
        params.put(&format!("{}{}", prefix, "Identity"), &obj.identity);
    }
}

/// <p>An empty element returned on a successful request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct SetIdentityDkimEnabledResponse {}

struct SetIdentityDkimEnabledResponseDeserializer;
impl SetIdentityDkimEnabledResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<SetIdentityDkimEnabledResponse, XmlParseError> {
        start_element(tag_name, stack)?;

        let obj = SetIdentityDkimEnabledResponse::default();

        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Represents a request to enable or disable whether Amazon SES forwards you bounce and complaint notifications through email. For information about email feedback forwarding, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/notifications-via-email.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SetIdentityFeedbackForwardingEnabledRequest {
    /// <p>Sets whether Amazon SES will forward bounce and complaint notifications as email. <code>true</code> specifies that Amazon SES will forward bounce and complaint notifications as email, in addition to any Amazon SNS topic publishing otherwise specified. <code>false</code> specifies that Amazon SES will publish bounce and complaint notifications only through Amazon SNS. This value can only be set to <code>false</code> when Amazon SNS topics are set for both <code>Bounce</code> and <code>Complaint</code> notification types.</p>
    pub forwarding_enabled: bool,
    /// <p>The identity for which to set bounce and complaint notification forwarding. Examples: <code>user@example.com</code>, <code>example.com</code>.</p>
    pub identity: String,
}

/// Serialize `SetIdentityFeedbackForwardingEnabledRequest` contents to a `SignedRequest`.
struct SetIdentityFeedbackForwardingEnabledRequestSerializer;
impl SetIdentityFeedbackForwardingEnabledRequestSerializer {
    fn serialize(
        params: &mut Params,
        name: &str,
        obj: &SetIdentityFeedbackForwardingEnabledRequest,
    ) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "ForwardingEnabled"),
            &obj.forwarding_enabled,
        );
        params.put(&format!("{}{}", prefix, "Identity"), &obj.identity);
    }
}

/// <p>An empty element returned on a successful request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct SetIdentityFeedbackForwardingEnabledResponse {}

struct SetIdentityFeedbackForwardingEnabledResponseDeserializer;
impl SetIdentityFeedbackForwardingEnabledResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<SetIdentityFeedbackForwardingEnabledResponse, XmlParseError> {
        start_element(tag_name, stack)?;

        let obj = SetIdentityFeedbackForwardingEnabledResponse::default();

        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Represents a request to set whether Amazon SES includes the original email headers in the Amazon SNS notifications of a specified type. For information about notifications, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/notifications-via-sns.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SetIdentityHeadersInNotificationsEnabledRequest {
    /// <p>Sets whether Amazon SES includes the original email headers in Amazon SNS notifications of the specified notification type. A value of <code>true</code> specifies that Amazon SES will include headers in notifications, and a value of <code>false</code> specifies that Amazon SES will not include headers in notifications.</p> <p>This value can only be set when <code>NotificationType</code> is already set to use a particular Amazon SNS topic.</p>
    pub enabled: bool,
    /// <p>The identity for which to enable or disable headers in notifications. Examples: <code>user@example.com</code>, <code>example.com</code>.</p>
    pub identity: String,
    /// <p>The notification type for which to enable or disable headers in notifications. </p>
    pub notification_type: String,
}

/// Serialize `SetIdentityHeadersInNotificationsEnabledRequest` contents to a `SignedRequest`.
struct SetIdentityHeadersInNotificationsEnabledRequestSerializer;
impl SetIdentityHeadersInNotificationsEnabledRequestSerializer {
    fn serialize(
        params: &mut Params,
        name: &str,
        obj: &SetIdentityHeadersInNotificationsEnabledRequest,
    ) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "Enabled"), &obj.enabled);
        params.put(&format!("{}{}", prefix, "Identity"), &obj.identity);
        params.put(
            &format!("{}{}", prefix, "NotificationType"),
            &obj.notification_type,
        );
    }
}

/// <p>An empty element returned on a successful request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct SetIdentityHeadersInNotificationsEnabledResponse {}

struct SetIdentityHeadersInNotificationsEnabledResponseDeserializer;
impl SetIdentityHeadersInNotificationsEnabledResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<SetIdentityHeadersInNotificationsEnabledResponse, XmlParseError> {
        start_element(tag_name, stack)?;

        let obj = SetIdentityHeadersInNotificationsEnabledResponse::default();

        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Represents a request to enable or disable the Amazon SES custom MAIL FROM domain setup for a verified identity. For information about using a custom MAIL FROM domain, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/mail-from.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SetIdentityMailFromDomainRequest {
    /// <p>The action that you want Amazon SES to take if it cannot successfully read the required MX record when you send an email. If you choose <code>UseDefaultValue</code>, Amazon SES will use amazonses.com (or a subdomain of that) as the MAIL FROM domain. If you choose <code>RejectMessage</code>, Amazon SES will return a <code>MailFromDomainNotVerified</code> error and not send the email.</p> <p>The action specified in <code>BehaviorOnMXFailure</code> is taken when the custom MAIL FROM domain setup is in the <code>Pending</code>, <code>Failed</code>, and <code>TemporaryFailure</code> states.</p>
    pub behavior_on_mx_failure: Option<String>,
    /// <p>The verified identity for which you want to enable or disable the specified custom MAIL FROM domain.</p>
    pub identity: String,
    /// <p>The custom MAIL FROM domain that you want the verified identity to use. The MAIL FROM domain must 1) be a subdomain of the verified identity, 2) not be used in a "From" address if the MAIL FROM domain is the destination of email feedback forwarding (for more information, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/mail-from.html">Amazon SES Developer Guide</a>), and 3) not be used to receive emails. A value of <code>null</code> disables the custom MAIL FROM setting for the identity.</p>
    pub mail_from_domain: Option<String>,
}

/// Serialize `SetIdentityMailFromDomainRequest` contents to a `SignedRequest`.
struct SetIdentityMailFromDomainRequestSerializer;
impl SetIdentityMailFromDomainRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &SetIdentityMailFromDomainRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.behavior_on_mx_failure {
            params.put(
                &format!("{}{}", prefix, "BehaviorOnMXFailure"),
                &field_value,
            );
        }
        params.put(&format!("{}{}", prefix, "Identity"), &obj.identity);
        if let Some(ref field_value) = obj.mail_from_domain {
            params.put(&format!("{}{}", prefix, "MailFromDomain"), &field_value);
        }
    }
}

/// <p>An empty element returned on a successful request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct SetIdentityMailFromDomainResponse {}

struct SetIdentityMailFromDomainResponseDeserializer;
impl SetIdentityMailFromDomainResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<SetIdentityMailFromDomainResponse, XmlParseError> {
        start_element(tag_name, stack)?;

        let obj = SetIdentityMailFromDomainResponse::default();

        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Represents a request to specify the Amazon SNS topic to which Amazon SES will publish bounce, complaint, or delivery notifications for emails sent with that identity as the Source. For information about Amazon SES notifications, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/notifications-via-sns.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SetIdentityNotificationTopicRequest {
    /// <p>The identity (email address or domain) that you want to set the Amazon SNS topic for.</p> <important> <p>You can only specify a verified identity for this parameter.</p> </important> <p>You can specify an identity by using its name or by using its Amazon Resource Name (ARN). The following examples are all valid identities: <code>sender@example.com</code>, <code>example.com</code>, <code>arn:aws:ses:us-east-1:123456789012:identity/example.com</code>.</p>
    pub identity: String,
    /// <p>The type of notifications that will be published to the specified Amazon SNS topic.</p>
    pub notification_type: String,
    /// <p>The Amazon Resource Name (ARN) of the Amazon SNS topic. If the parameter is omitted from the request or a null value is passed, <code>SnsTopic</code> is cleared and publishing is disabled.</p>
    pub sns_topic: Option<String>,
}

/// Serialize `SetIdentityNotificationTopicRequest` contents to a `SignedRequest`.
struct SetIdentityNotificationTopicRequestSerializer;
impl SetIdentityNotificationTopicRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &SetIdentityNotificationTopicRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "Identity"), &obj.identity);
        params.put(
            &format!("{}{}", prefix, "NotificationType"),
            &obj.notification_type,
        );
        if let Some(ref field_value) = obj.sns_topic {
            params.put(&format!("{}{}", prefix, "SnsTopic"), &field_value);
        }
    }
}

/// <p>An empty element returned on a successful request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct SetIdentityNotificationTopicResponse {}

struct SetIdentityNotificationTopicResponseDeserializer;
impl SetIdentityNotificationTopicResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<SetIdentityNotificationTopicResponse, XmlParseError> {
        start_element(tag_name, stack)?;

        let obj = SetIdentityNotificationTopicResponse::default();

        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Represents a request to set the position of a receipt rule in a receipt rule set. You use receipt rule sets to receive email with Amazon SES. For more information, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-concepts.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SetReceiptRulePositionRequest {
    /// <p>The name of the receipt rule after which to place the specified receipt rule.</p>
    pub after: Option<String>,
    /// <p>The name of the receipt rule to reposition.</p>
    pub rule_name: String,
    /// <p>The name of the receipt rule set that contains the receipt rule to reposition.</p>
    pub rule_set_name: String,
}

/// Serialize `SetReceiptRulePositionRequest` contents to a `SignedRequest`.
struct SetReceiptRulePositionRequestSerializer;
impl SetReceiptRulePositionRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &SetReceiptRulePositionRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.after {
            params.put(&format!("{}{}", prefix, "After"), &field_value);
        }
        params.put(&format!("{}{}", prefix, "RuleName"), &obj.rule_name);
        params.put(&format!("{}{}", prefix, "RuleSetName"), &obj.rule_set_name);
    }
}

/// <p>An empty element returned on a successful request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct SetReceiptRulePositionResponse {}

struct SetReceiptRulePositionResponseDeserializer;
impl SetReceiptRulePositionResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<SetReceiptRulePositionResponse, XmlParseError> {
        start_element(tag_name, stack)?;

        let obj = SetReceiptRulePositionResponse::default();

        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>When included in a receipt rule, this action terminates the evaluation of the receipt rule set and, optionally, publishes a notification to Amazon Simple Notification Service (Amazon SNS).</p> <p>For information about setting a stop action in a receipt rule, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-action-stop.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StopAction {
    /// <p>The scope of the StopAction. The only acceptable value is <code>RuleSet</code>.</p>
    pub scope: String,
    /// <p>The Amazon Resource Name (ARN) of the Amazon SNS topic to notify when the stop action is taken. An example of an Amazon SNS topic ARN is <code>arn:aws:sns:us-west-2:123456789012:MyTopic</code>. For more information about Amazon SNS topics, see the <a href="https://docs.aws.amazon.com/sns/latest/dg/CreateTopic.html">Amazon SNS Developer Guide</a>.</p>
    pub topic_arn: Option<String>,
}

struct StopActionDeserializer;
impl StopActionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<StopAction, XmlParseError> {
        deserialize_elements::<_, StopAction, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Scope" => {
                    obj.scope = StopScopeDeserializer::deserialize("Scope", stack)?;
                }
                "TopicArn" => {
                    obj.topic_arn = Some(AmazonResourceNameDeserializer::deserialize(
                        "TopicArn", stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

/// Serialize `StopAction` contents to a `SignedRequest`.
struct StopActionSerializer;
impl StopActionSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &StopAction) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "Scope"), &obj.scope);
        if let Some(ref field_value) = obj.topic_arn {
            params.put(&format!("{}{}", prefix, "TopicArn"), &field_value);
        }
    }
}

struct StopScopeDeserializer;
impl StopScopeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct SubjectDeserializer;
impl SubjectDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct SubjectPartDeserializer;
impl SubjectPartDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct SuccessRedirectionURLDeserializer;
impl SuccessRedirectionURLDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>The content of the email, composed of a subject line, an HTML part, and a text-only part.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Template {
    /// <p>The HTML body of the email.</p>
    pub html_part: Option<String>,
    /// <p>The subject line of the email.</p>
    pub subject_part: Option<String>,
    /// <p>The name of the template. You will refer to this name when you send email using the <code>SendTemplatedEmail</code> or <code>SendBulkTemplatedEmail</code> operations.</p>
    pub template_name: String,
    /// <p>The email body that will be visible to recipients whose email clients do not display HTML.</p>
    pub text_part: Option<String>,
}

struct TemplateDeserializer;
impl TemplateDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Template, XmlParseError> {
        deserialize_elements::<_, Template, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "HtmlPart" => {
                    obj.html_part = Some(HtmlPartDeserializer::deserialize("HtmlPart", stack)?);
                }
                "SubjectPart" => {
                    obj.subject_part =
                        Some(SubjectPartDeserializer::deserialize("SubjectPart", stack)?);
                }
                "TemplateName" => {
                    obj.template_name =
                        TemplateNameDeserializer::deserialize("TemplateName", stack)?;
                }
                "TextPart" => {
                    obj.text_part = Some(TextPartDeserializer::deserialize("TextPart", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

/// Serialize `Template` contents to a `SignedRequest`.
struct TemplateSerializer;
impl TemplateSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Template) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.html_part {
            params.put(&format!("{}{}", prefix, "HtmlPart"), &field_value);
        }
        if let Some(ref field_value) = obj.subject_part {
            params.put(&format!("{}{}", prefix, "SubjectPart"), &field_value);
        }
        params.put(&format!("{}{}", prefix, "TemplateName"), &obj.template_name);
        if let Some(ref field_value) = obj.text_part {
            params.put(&format!("{}{}", prefix, "TextPart"), &field_value);
        }
    }
}

struct TemplateContentDeserializer;
impl TemplateContentDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Contains information about an email template.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct TemplateMetadata {
    /// <p>The time and date the template was created.</p>
    pub created_timestamp: Option<String>,
    /// <p>The name of the template.</p>
    pub name: Option<String>,
}

struct TemplateMetadataDeserializer;
impl TemplateMetadataDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<TemplateMetadata, XmlParseError> {
        deserialize_elements::<_, TemplateMetadata, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "CreatedTimestamp" => {
                    obj.created_timestamp = Some(TimestampDeserializer::deserialize(
                        "CreatedTimestamp",
                        stack,
                    )?);
                }
                "Name" => {
                    obj.name = Some(TemplateNameDeserializer::deserialize("Name", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct TemplateMetadataListDeserializer;
impl TemplateMetadataListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<TemplateMetadata>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(TemplateMetadataDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
struct TemplateNameDeserializer;
impl TemplateNameDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TestRenderTemplateRequest {
    /// <p>A list of replacement values to apply to the template. This parameter is a JSON object, typically consisting of key-value pairs in which the keys correspond to replacement tags in the email template.</p>
    pub template_data: String,
    /// <p>The name of the template that you want to render.</p>
    pub template_name: String,
}

/// Serialize `TestRenderTemplateRequest` contents to a `SignedRequest`.
struct TestRenderTemplateRequestSerializer;
impl TestRenderTemplateRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &TestRenderTemplateRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "TemplateData"), &obj.template_data);
        params.put(&format!("{}{}", prefix, "TemplateName"), &obj.template_name);
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct TestRenderTemplateResponse {
    /// <p>The complete MIME message rendered by applying the data in the TemplateData parameter to the template specified in the TemplateName parameter.</p>
    pub rendered_template: Option<String>,
}

struct TestRenderTemplateResponseDeserializer;
impl TestRenderTemplateResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<TestRenderTemplateResponse, XmlParseError> {
        deserialize_elements::<_, TestRenderTemplateResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "RenderedTemplate" => {
                        obj.rendered_template = Some(RenderedTemplateDeserializer::deserialize(
                            "RenderedTemplate",
                            stack,
                        )?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
struct TextPartDeserializer;
impl TextPartDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct TimestampDeserializer;
impl TimestampDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct TlsPolicyDeserializer;
impl TlsPolicyDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>A domain that is used to redirect email recipients to an Amazon SES-operated domain. This domain captures open and click events generated by Amazon SES emails.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/configure-custom-open-click-domains.html">Configuring Custom Domains to Handle Open and Click Tracking</a> in the <i>Amazon SES Developer Guide</i>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TrackingOptions {
    /// <p>The custom subdomain that will be used to redirect email recipients to the Amazon SES event tracking domain.</p>
    pub custom_redirect_domain: Option<String>,
}

struct TrackingOptionsDeserializer;
impl TrackingOptionsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<TrackingOptions, XmlParseError> {
        deserialize_elements::<_, TrackingOptions, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "CustomRedirectDomain" => {
                    obj.custom_redirect_domain =
                        Some(CustomRedirectDomainDeserializer::deserialize(
                            "CustomRedirectDomain",
                            stack,
                        )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

/// Serialize `TrackingOptions` contents to a `SignedRequest`.
struct TrackingOptionsSerializer;
impl TrackingOptionsSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &TrackingOptions) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.custom_redirect_domain {
            params.put(
                &format!("{}{}", prefix, "CustomRedirectDomain"),
                &field_value,
            );
        }
    }
}

/// <p>Represents a request to enable or disable the email sending capabilities for your entire Amazon SES account.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateAccountSendingEnabledRequest {
    /// <p>Describes whether email sending is enabled or disabled for your Amazon SES account in the current AWS Region.</p>
    pub enabled: Option<bool>,
}

/// Serialize `UpdateAccountSendingEnabledRequest` contents to a `SignedRequest`.
struct UpdateAccountSendingEnabledRequestSerializer;
impl UpdateAccountSendingEnabledRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &UpdateAccountSendingEnabledRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.enabled {
            params.put(&format!("{}{}", prefix, "Enabled"), &field_value);
        }
    }
}

/// <p>Represents a request to update the event destination of a configuration set. Configuration sets enable you to publish email sending events. For information about using configuration sets, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/monitor-sending-activity.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateConfigurationSetEventDestinationRequest {
    /// <p>The name of the configuration set that contains the event destination that you want to update.</p>
    pub configuration_set_name: String,
    /// <p>The event destination object that you want to apply to the specified configuration set.</p>
    pub event_destination: EventDestination,
}

/// Serialize `UpdateConfigurationSetEventDestinationRequest` contents to a `SignedRequest`.
struct UpdateConfigurationSetEventDestinationRequestSerializer;
impl UpdateConfigurationSetEventDestinationRequestSerializer {
    fn serialize(
        params: &mut Params,
        name: &str,
        obj: &UpdateConfigurationSetEventDestinationRequest,
    ) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "ConfigurationSetName"),
            &obj.configuration_set_name,
        );
        EventDestinationSerializer::serialize(
            params,
            &format!("{}{}", prefix, "EventDestination"),
            &obj.event_destination,
        );
    }
}

/// <p>An empty element returned on a successful request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct UpdateConfigurationSetEventDestinationResponse {}

struct UpdateConfigurationSetEventDestinationResponseDeserializer;
impl UpdateConfigurationSetEventDestinationResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<UpdateConfigurationSetEventDestinationResponse, XmlParseError> {
        start_element(tag_name, stack)?;

        let obj = UpdateConfigurationSetEventDestinationResponse::default();

        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Represents a request to modify the reputation metric publishing settings for a configuration set.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateConfigurationSetReputationMetricsEnabledRequest {
    /// <p>The name of the configuration set that you want to update.</p>
    pub configuration_set_name: String,
    /// <p>Describes whether or not Amazon SES will publish reputation metrics for the configuration set, such as bounce and complaint rates, to Amazon CloudWatch.</p>
    pub enabled: bool,
}

/// Serialize `UpdateConfigurationSetReputationMetricsEnabledRequest` contents to a `SignedRequest`.
struct UpdateConfigurationSetReputationMetricsEnabledRequestSerializer;
impl UpdateConfigurationSetReputationMetricsEnabledRequestSerializer {
    fn serialize(
        params: &mut Params,
        name: &str,
        obj: &UpdateConfigurationSetReputationMetricsEnabledRequest,
    ) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "ConfigurationSetName"),
            &obj.configuration_set_name,
        );
        params.put(&format!("{}{}", prefix, "Enabled"), &obj.enabled);
    }
}

/// <p>Represents a request to enable or disable the email sending capabilities for a specific configuration set.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateConfigurationSetSendingEnabledRequest {
    /// <p>The name of the configuration set that you want to update.</p>
    pub configuration_set_name: String,
    /// <p>Describes whether email sending is enabled or disabled for the configuration set. </p>
    pub enabled: bool,
}

/// Serialize `UpdateConfigurationSetSendingEnabledRequest` contents to a `SignedRequest`.
struct UpdateConfigurationSetSendingEnabledRequestSerializer;
impl UpdateConfigurationSetSendingEnabledRequestSerializer {
    fn serialize(
        params: &mut Params,
        name: &str,
        obj: &UpdateConfigurationSetSendingEnabledRequest,
    ) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "ConfigurationSetName"),
            &obj.configuration_set_name,
        );
        params.put(&format!("{}{}", prefix, "Enabled"), &obj.enabled);
    }
}

/// <p>Represents a request to update the tracking options for a configuration set. </p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateConfigurationSetTrackingOptionsRequest {
    /// <p>The name of the configuration set for which you want to update the custom tracking domain.</p>
    pub configuration_set_name: String,
    pub tracking_options: TrackingOptions,
}

/// Serialize `UpdateConfigurationSetTrackingOptionsRequest` contents to a `SignedRequest`.
struct UpdateConfigurationSetTrackingOptionsRequestSerializer;
impl UpdateConfigurationSetTrackingOptionsRequestSerializer {
    fn serialize(
        params: &mut Params,
        name: &str,
        obj: &UpdateConfigurationSetTrackingOptionsRequest,
    ) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "ConfigurationSetName"),
            &obj.configuration_set_name,
        );
        TrackingOptionsSerializer::serialize(
            params,
            &format!("{}{}", prefix, "TrackingOptions"),
            &obj.tracking_options,
        );
    }
}

/// <p>An empty element returned on a successful request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct UpdateConfigurationSetTrackingOptionsResponse {}

struct UpdateConfigurationSetTrackingOptionsResponseDeserializer;
impl UpdateConfigurationSetTrackingOptionsResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<UpdateConfigurationSetTrackingOptionsResponse, XmlParseError> {
        start_element(tag_name, stack)?;

        let obj = UpdateConfigurationSetTrackingOptionsResponse::default();

        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Represents a request to update an existing custom verification email template.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateCustomVerificationEmailTemplateRequest {
    /// <p>The URL that the recipient of the verification email is sent to if his or her address is not successfully verified.</p>
    pub failure_redirection_url: Option<String>,
    /// <p>The email address that the custom verification email is sent from.</p>
    pub from_email_address: Option<String>,
    /// <p>The URL that the recipient of the verification email is sent to if his or her address is successfully verified.</p>
    pub success_redirection_url: Option<String>,
    /// <p>The content of the custom verification email. The total size of the email must be less than 10 MB. The message body may contain HTML, with some limitations. For more information, see <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/custom-verification-emails.html#custom-verification-emails-faq">Custom Verification Email Frequently Asked Questions</a> in the <i>Amazon SES Developer Guide</i>.</p>
    pub template_content: Option<String>,
    /// <p>The name of the custom verification email template that you want to update.</p>
    pub template_name: String,
    /// <p>The subject line of the custom verification email.</p>
    pub template_subject: Option<String>,
}

/// Serialize `UpdateCustomVerificationEmailTemplateRequest` contents to a `SignedRequest`.
struct UpdateCustomVerificationEmailTemplateRequestSerializer;
impl UpdateCustomVerificationEmailTemplateRequestSerializer {
    fn serialize(
        params: &mut Params,
        name: &str,
        obj: &UpdateCustomVerificationEmailTemplateRequest,
    ) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.failure_redirection_url {
            params.put(
                &format!("{}{}", prefix, "FailureRedirectionURL"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.from_email_address {
            params.put(&format!("{}{}", prefix, "FromEmailAddress"), &field_value);
        }
        if let Some(ref field_value) = obj.success_redirection_url {
            params.put(
                &format!("{}{}", prefix, "SuccessRedirectionURL"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.template_content {
            params.put(&format!("{}{}", prefix, "TemplateContent"), &field_value);
        }
        params.put(&format!("{}{}", prefix, "TemplateName"), &obj.template_name);
        if let Some(ref field_value) = obj.template_subject {
            params.put(&format!("{}{}", prefix, "TemplateSubject"), &field_value);
        }
    }
}

/// <p>Represents a request to update a receipt rule. You use receipt rules to receive email with Amazon SES. For more information, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-concepts.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateReceiptRuleRequest {
    /// <p>A data structure that contains the updated receipt rule information.</p>
    pub rule: ReceiptRule,
    /// <p>The name of the receipt rule set that the receipt rule belongs to.</p>
    pub rule_set_name: String,
}

/// Serialize `UpdateReceiptRuleRequest` contents to a `SignedRequest`.
struct UpdateReceiptRuleRequestSerializer;
impl UpdateReceiptRuleRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &UpdateReceiptRuleRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        ReceiptRuleSerializer::serialize(params, &format!("{}{}", prefix, "Rule"), &obj.rule);
        params.put(&format!("{}{}", prefix, "RuleSetName"), &obj.rule_set_name);
    }
}

/// <p>An empty element returned on a successful request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct UpdateReceiptRuleResponse {}

struct UpdateReceiptRuleResponseDeserializer;
impl UpdateReceiptRuleResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<UpdateReceiptRuleResponse, XmlParseError> {
        start_element(tag_name, stack)?;

        let obj = UpdateReceiptRuleResponse::default();

        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateTemplateRequest {
    pub template: Template,
}

/// Serialize `UpdateTemplateRequest` contents to a `SignedRequest`.
struct UpdateTemplateRequestSerializer;
impl UpdateTemplateRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &UpdateTemplateRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        TemplateSerializer::serialize(params, &format!("{}{}", prefix, "Template"), &obj.template);
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct UpdateTemplateResponse {}

struct UpdateTemplateResponseDeserializer;
impl UpdateTemplateResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<UpdateTemplateResponse, XmlParseError> {
        start_element(tag_name, stack)?;

        let obj = UpdateTemplateResponse::default();

        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct VerificationAttributesDeserializer;
impl VerificationAttributesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<::std::collections::HashMap<String, IdentityVerificationAttributes>, XmlParseError>
    {
        start_element(tag_name, stack)?;

        let mut obj = ::std::collections::HashMap::new();

        while peek_at_name(stack)? == "entry" {
            start_element("entry", stack)?;
            let key = IdentityDeserializer::deserialize("key", stack)?;
            let value = IdentityVerificationAttributesDeserializer::deserialize("value", stack)?;
            obj.insert(key, value);
            end_element("entry", stack)?;
        }

        end_element(tag_name, stack)?;
        Ok(obj)
    }
}
struct VerificationStatusDeserializer;
impl VerificationStatusDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct VerificationTokenDeserializer;
impl VerificationTokenDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct VerificationTokenListDeserializer;
impl VerificationTokenListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(VerificationTokenDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
/// <p>Represents a request to generate the CNAME records needed to set up Easy DKIM with Amazon SES. For more information about setting up Easy DKIM, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/easy-dkim.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct VerifyDomainDkimRequest {
    /// <p>The name of the domain to be verified for Easy DKIM signing.</p>
    pub domain: String,
}

/// Serialize `VerifyDomainDkimRequest` contents to a `SignedRequest`.
struct VerifyDomainDkimRequestSerializer;
impl VerifyDomainDkimRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &VerifyDomainDkimRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "Domain"), &obj.domain);
    }
}

/// <p>Returns CNAME records that you must publish to the DNS server of your domain to set up Easy DKIM with Amazon SES.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct VerifyDomainDkimResponse {
    /// <p>A set of character strings that represent the domain's identity. If the identity is an email address, the tokens represent the domain of that address.</p> <p>Using these tokens, you need to create DNS CNAME records that point to DKIM public keys that are hosted by Amazon SES. Amazon Web Services eventually detects that you've updated your DNS records. This detection process might take up to 72 hours. After successful detection, Amazon SES is able to DKIM-sign email originating from that domain. (This only applies to domain identities, not email address identities.)</p> <p>For more information about creating DNS records using DKIM tokens, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/easy-dkim.html">Amazon SES Developer Guide</a>.</p>
    pub dkim_tokens: Vec<String>,
}

struct VerifyDomainDkimResponseDeserializer;
impl VerifyDomainDkimResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<VerifyDomainDkimResponse, XmlParseError> {
        deserialize_elements::<_, VerifyDomainDkimResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "DkimTokens" => {
                        obj.dkim_tokens
                            .extend(VerificationTokenListDeserializer::deserialize(
                                "DkimTokens",
                                stack,
                            )?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p>Represents a request to begin Amazon SES domain verification and to generate the TXT records that you must publish to the DNS server of your domain to complete the verification. For information about domain verification, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/verify-domains.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct VerifyDomainIdentityRequest {
    /// <p>The domain to be verified.</p>
    pub domain: String,
}

/// Serialize `VerifyDomainIdentityRequest` contents to a `SignedRequest`.
struct VerifyDomainIdentityRequestSerializer;
impl VerifyDomainIdentityRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &VerifyDomainIdentityRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "Domain"), &obj.domain);
    }
}

/// <p>Returns a TXT record that you must publish to the DNS server of your domain to complete domain verification with Amazon SES.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct VerifyDomainIdentityResponse {
    /// <p>A TXT record that you must place in the DNS settings of the domain to complete domain verification with Amazon SES.</p> <p>As Amazon SES searches for the TXT record, the domain's verification status is "Pending". When Amazon SES detects the record, the domain's verification status changes to "Success". If Amazon SES is unable to detect the record within 72 hours, the domain's verification status changes to "Failed." In that case, if you still want to verify the domain, you must restart the verification process from the beginning.</p>
    pub verification_token: String,
}

struct VerifyDomainIdentityResponseDeserializer;
impl VerifyDomainIdentityResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<VerifyDomainIdentityResponse, XmlParseError> {
        deserialize_elements::<_, VerifyDomainIdentityResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "VerificationToken" => {
                        obj.verification_token =
                            VerificationTokenDeserializer::deserialize("VerificationToken", stack)?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p>Represents a request to begin email address verification with Amazon SES. For information about email address verification, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/verify-email-addresses.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct VerifyEmailAddressRequest {
    /// <p>The email address to be verified.</p>
    pub email_address: String,
}

/// Serialize `VerifyEmailAddressRequest` contents to a `SignedRequest`.
struct VerifyEmailAddressRequestSerializer;
impl VerifyEmailAddressRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &VerifyEmailAddressRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "EmailAddress"), &obj.email_address);
    }
}

/// <p>Represents a request to begin email address verification with Amazon SES. For information about email address verification, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/verify-email-addresses.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct VerifyEmailIdentityRequest {
    /// <p>The email address to be verified.</p>
    pub email_address: String,
}

/// Serialize `VerifyEmailIdentityRequest` contents to a `SignedRequest`.
struct VerifyEmailIdentityRequestSerializer;
impl VerifyEmailIdentityRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &VerifyEmailIdentityRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "EmailAddress"), &obj.email_address);
    }
}

/// <p>An empty element returned on a successful request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct VerifyEmailIdentityResponse {}

struct VerifyEmailIdentityResponseDeserializer;
impl VerifyEmailIdentityResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<VerifyEmailIdentityResponse, XmlParseError> {
        start_element(tag_name, stack)?;

        let obj = VerifyEmailIdentityResponse::default();

        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>When included in a receipt rule, this action calls Amazon WorkMail and, optionally, publishes a notification to Amazon Simple Notification Service (Amazon SNS). You will typically not use this action directly because Amazon WorkMail adds the rule automatically during its setup procedure.</p> <p>For information using a receipt rule to call Amazon WorkMail, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-action-workmail.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct WorkmailAction {
    /// <p>The ARN of the Amazon WorkMail organization. An example of an Amazon WorkMail organization ARN is <code>arn:aws:workmail:us-west-2:123456789012:organization/m-68755160c4cb4e29a2b2f8fb58f359d7</code>. For information about Amazon WorkMail organizations, see the <a href="https://docs.aws.amazon.com/workmail/latest/adminguide/organizations_overview.html">Amazon WorkMail Administrator Guide</a>.</p>
    pub organization_arn: String,
    /// <p>The Amazon Resource Name (ARN) of the Amazon SNS topic to notify when the WorkMail action is called. An example of an Amazon SNS topic ARN is <code>arn:aws:sns:us-west-2:123456789012:MyTopic</code>. For more information about Amazon SNS topics, see the <a href="https://docs.aws.amazon.com/sns/latest/dg/CreateTopic.html">Amazon SNS Developer Guide</a>.</p>
    pub topic_arn: Option<String>,
}

struct WorkmailActionDeserializer;
impl WorkmailActionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<WorkmailAction, XmlParseError> {
        deserialize_elements::<_, WorkmailAction, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "OrganizationArn" => {
                    obj.organization_arn =
                        AmazonResourceNameDeserializer::deserialize("OrganizationArn", stack)?;
                }
                "TopicArn" => {
                    obj.topic_arn = Some(AmazonResourceNameDeserializer::deserialize(
                        "TopicArn", stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

/// Serialize `WorkmailAction` contents to a `SignedRequest`.
struct WorkmailActionSerializer;
impl WorkmailActionSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &WorkmailAction) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "OrganizationArn"),
            &obj.organization_arn,
        );
        if let Some(ref field_value) = obj.topic_arn {
            params.put(&format!("{}{}", prefix, "TopicArn"), &field_value);
        }
    }
}

/// Errors returned by CloneReceiptRuleSet
#[derive(Debug, PartialEq)]
pub enum CloneReceiptRuleSetError {
    /// <p>Indicates that a resource could not be created because of a naming conflict.</p>
    AlreadyExists(String),
    /// <p>Indicates that a resource could not be created because of service limits. For a list of Amazon SES limits, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/limits.html">Amazon SES Developer Guide</a>.</p>
    LimitExceeded(String),
    /// <p>Indicates that the provided receipt rule set does not exist.</p>
    RuleSetDoesNotExist(String),
}

impl CloneReceiptRuleSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CloneReceiptRuleSetError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AlreadyExists" => {
                        return RusotoError::Service(CloneReceiptRuleSetError::AlreadyExists(
                            parsed_error.message,
                        ))
                    }
                    "LimitExceeded" => {
                        return RusotoError::Service(CloneReceiptRuleSetError::LimitExceeded(
                            parsed_error.message,
                        ))
                    }
                    "RuleSetDoesNotExist" => {
                        return RusotoError::Service(CloneReceiptRuleSetError::RuleSetDoesNotExist(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for CloneReceiptRuleSetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CloneReceiptRuleSetError::AlreadyExists(ref cause) => write!(f, "{}", cause),
            CloneReceiptRuleSetError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CloneReceiptRuleSetError::RuleSetDoesNotExist(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CloneReceiptRuleSetError {}
/// Errors returned by CreateConfigurationSet
#[derive(Debug, PartialEq)]
pub enum CreateConfigurationSetError {
    /// <p>Indicates that the configuration set could not be created because of a naming conflict.</p>
    ConfigurationSetAlreadyExists(String),
    /// <p>Indicates that the configuration set is invalid. See the error message for details.</p>
    InvalidConfigurationSet(String),
    /// <p>Indicates that a resource could not be created because of service limits. For a list of Amazon SES limits, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/limits.html">Amazon SES Developer Guide</a>.</p>
    LimitExceeded(String),
}

impl CreateConfigurationSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateConfigurationSetError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "ConfigurationSetAlreadyExists" => {
                        return RusotoError::Service(
                            CreateConfigurationSetError::ConfigurationSetAlreadyExists(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidConfigurationSet" => {
                        return RusotoError::Service(
                            CreateConfigurationSetError::InvalidConfigurationSet(
                                parsed_error.message,
                            ),
                        )
                    }
                    "LimitExceeded" => {
                        return RusotoError::Service(CreateConfigurationSetError::LimitExceeded(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for CreateConfigurationSetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateConfigurationSetError::ConfigurationSetAlreadyExists(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateConfigurationSetError::InvalidConfigurationSet(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateConfigurationSetError::LimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateConfigurationSetError {}
/// Errors returned by CreateConfigurationSetEventDestination
#[derive(Debug, PartialEq)]
pub enum CreateConfigurationSetEventDestinationError {
    /// <p>Indicates that the configuration set does not exist.</p>
    ConfigurationSetDoesNotExist(String),
    /// <p>Indicates that the event destination could not be created because of a naming conflict.</p>
    EventDestinationAlreadyExists(String),
    /// <p>Indicates that the Amazon CloudWatch destination is invalid. See the error message for details.</p>
    InvalidCloudWatchDestination(String),
    /// <p>Indicates that the Amazon Kinesis Firehose destination is invalid. See the error message for details.</p>
    InvalidFirehoseDestination(String),
    /// <p>Indicates that the Amazon Simple Notification Service (Amazon SNS) destination is invalid. See the error message for details.</p>
    InvalidSNSDestination(String),
    /// <p>Indicates that a resource could not be created because of service limits. For a list of Amazon SES limits, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/limits.html">Amazon SES Developer Guide</a>.</p>
    LimitExceeded(String),
}

impl CreateConfigurationSetEventDestinationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateConfigurationSetEventDestinationError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "ConfigurationSetDoesNotExist" => return RusotoError::Service(
                        CreateConfigurationSetEventDestinationError::ConfigurationSetDoesNotExist(
                            parsed_error.message,
                        ),
                    ),
                    "EventDestinationAlreadyExists" => return RusotoError::Service(
                        CreateConfigurationSetEventDestinationError::EventDestinationAlreadyExists(
                            parsed_error.message,
                        ),
                    ),
                    "InvalidCloudWatchDestination" => return RusotoError::Service(
                        CreateConfigurationSetEventDestinationError::InvalidCloudWatchDestination(
                            parsed_error.message,
                        ),
                    ),
                    "InvalidFirehoseDestination" => {
                        return RusotoError::Service(
                            CreateConfigurationSetEventDestinationError::InvalidFirehoseDestination(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidSNSDestination" => {
                        return RusotoError::Service(
                            CreateConfigurationSetEventDestinationError::InvalidSNSDestination(
                                parsed_error.message,
                            ),
                        )
                    }
                    "LimitExceeded" => {
                        return RusotoError::Service(
                            CreateConfigurationSetEventDestinationError::LimitExceeded(
                                parsed_error.message,
                            ),
                        )
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for CreateConfigurationSetEventDestinationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateConfigurationSetEventDestinationError::ConfigurationSetDoesNotExist(
                ref cause,
            ) => write!(f, "{}", cause),
            CreateConfigurationSetEventDestinationError::EventDestinationAlreadyExists(
                ref cause,
            ) => write!(f, "{}", cause),
            CreateConfigurationSetEventDestinationError::InvalidCloudWatchDestination(
                ref cause,
            ) => write!(f, "{}", cause),
            CreateConfigurationSetEventDestinationError::InvalidFirehoseDestination(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateConfigurationSetEventDestinationError::InvalidSNSDestination(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateConfigurationSetEventDestinationError::LimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for CreateConfigurationSetEventDestinationError {}
/// Errors returned by CreateConfigurationSetTrackingOptions
#[derive(Debug, PartialEq)]
pub enum CreateConfigurationSetTrackingOptionsError {
    /// <p>Indicates that the configuration set does not exist.</p>
    ConfigurationSetDoesNotExist(String),
    /// <p><p>Indicates that the custom domain to be used for open and click tracking redirects is invalid. This error appears most often in the following situations:</p> <ul> <li> <p>When the tracking domain you specified is not verified in Amazon SES.</p> </li> <li> <p>When the tracking domain you specified is not a valid domain or subdomain.</p> </li> </ul></p>
    InvalidTrackingOptions(String),
    /// <p>Indicates that the configuration set you specified already contains a TrackingOptions object.</p>
    TrackingOptionsAlreadyExists(String),
}

impl CreateConfigurationSetTrackingOptionsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateConfigurationSetTrackingOptionsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "ConfigurationSetDoesNotExist" => return RusotoError::Service(
                        CreateConfigurationSetTrackingOptionsError::ConfigurationSetDoesNotExist(
                            parsed_error.message,
                        ),
                    ),
                    "InvalidTrackingOptions" => {
                        return RusotoError::Service(
                            CreateConfigurationSetTrackingOptionsError::InvalidTrackingOptions(
                                parsed_error.message,
                            ),
                        )
                    }
                    "TrackingOptionsAlreadyExistsException" => return RusotoError::Service(
                        CreateConfigurationSetTrackingOptionsError::TrackingOptionsAlreadyExists(
                            parsed_error.message,
                        ),
                    ),
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for CreateConfigurationSetTrackingOptionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateConfigurationSetTrackingOptionsError::ConfigurationSetDoesNotExist(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateConfigurationSetTrackingOptionsError::InvalidTrackingOptions(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateConfigurationSetTrackingOptionsError::TrackingOptionsAlreadyExists(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for CreateConfigurationSetTrackingOptionsError {}
/// Errors returned by CreateCustomVerificationEmailTemplate
#[derive(Debug, PartialEq)]
pub enum CreateCustomVerificationEmailTemplateError {
    /// <p>Indicates that custom verification email template provided content is invalid.</p>
    CustomVerificationEmailInvalidContent(String),
    /// <p>Indicates that a custom verification email template with the name you specified already exists.</p>
    CustomVerificationEmailTemplateAlreadyExists(String),
    /// <p>Indicates that the sender address specified for a custom verification email is not verified, and is therefore not eligible to send the custom verification email. </p>
    FromEmailAddressNotVerified(String),
    /// <p>Indicates that a resource could not be created because of service limits. For a list of Amazon SES limits, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/limits.html">Amazon SES Developer Guide</a>.</p>
    LimitExceeded(String),
}

impl CreateCustomVerificationEmailTemplateError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateCustomVerificationEmailTemplateError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                                    "CustomVerificationEmailInvalidContent" => return RusotoError::Service(CreateCustomVerificationEmailTemplateError::CustomVerificationEmailInvalidContent(parsed_error.message)),"CustomVerificationEmailTemplateAlreadyExists" => return RusotoError::Service(CreateCustomVerificationEmailTemplateError::CustomVerificationEmailTemplateAlreadyExists(parsed_error.message)),"FromEmailAddressNotVerified" => return RusotoError::Service(CreateCustomVerificationEmailTemplateError::FromEmailAddressNotVerified(parsed_error.message)),"LimitExceeded" => return RusotoError::Service(CreateCustomVerificationEmailTemplateError::LimitExceeded(parsed_error.message)),_ => {}
                                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for CreateCustomVerificationEmailTemplateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
                            CreateCustomVerificationEmailTemplateError::CustomVerificationEmailInvalidContent(ref cause) => write!(f, "{}", cause),
CreateCustomVerificationEmailTemplateError::CustomVerificationEmailTemplateAlreadyExists(ref cause) => write!(f, "{}", cause),
CreateCustomVerificationEmailTemplateError::FromEmailAddressNotVerified(ref cause) => write!(f, "{}", cause),
CreateCustomVerificationEmailTemplateError::LimitExceeded(ref cause) => write!(f, "{}", cause)
                        }
    }
}
impl Error for CreateCustomVerificationEmailTemplateError {}
/// Errors returned by CreateReceiptFilter
#[derive(Debug, PartialEq)]
pub enum CreateReceiptFilterError {
    /// <p>Indicates that a resource could not be created because of a naming conflict.</p>
    AlreadyExists(String),
    /// <p>Indicates that a resource could not be created because of service limits. For a list of Amazon SES limits, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/limits.html">Amazon SES Developer Guide</a>.</p>
    LimitExceeded(String),
}

impl CreateReceiptFilterError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateReceiptFilterError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AlreadyExists" => {
                        return RusotoError::Service(CreateReceiptFilterError::AlreadyExists(
                            parsed_error.message,
                        ))
                    }
                    "LimitExceeded" => {
                        return RusotoError::Service(CreateReceiptFilterError::LimitExceeded(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for CreateReceiptFilterError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateReceiptFilterError::AlreadyExists(ref cause) => write!(f, "{}", cause),
            CreateReceiptFilterError::LimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateReceiptFilterError {}
/// Errors returned by CreateReceiptRule
#[derive(Debug, PartialEq)]
pub enum CreateReceiptRuleError {
    /// <p>Indicates that a resource could not be created because of a naming conflict.</p>
    AlreadyExists(String),
    /// <p>Indicates that the provided AWS Lambda function is invalid, or that Amazon SES could not execute the provided function, possibly due to permissions issues. For information about giving permissions, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-permissions.html">Amazon SES Developer Guide</a>.</p>
    InvalidLambdaFunction(String),
    /// <p>Indicates that the provided Amazon S3 bucket or AWS KMS encryption key is invalid, or that Amazon SES could not publish to the bucket, possibly due to permissions issues. For information about giving permissions, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-permissions.html">Amazon SES Developer Guide</a>.</p>
    InvalidS3Configuration(String),
    /// <p>Indicates that the provided Amazon SNS topic is invalid, or that Amazon SES could not publish to the topic, possibly due to permissions issues. For information about giving permissions, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-permissions.html">Amazon SES Developer Guide</a>.</p>
    InvalidSnsTopic(String),
    /// <p>Indicates that a resource could not be created because of service limits. For a list of Amazon SES limits, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/limits.html">Amazon SES Developer Guide</a>.</p>
    LimitExceeded(String),
    /// <p>Indicates that the provided receipt rule does not exist.</p>
    RuleDoesNotExist(String),
    /// <p>Indicates that the provided receipt rule set does not exist.</p>
    RuleSetDoesNotExist(String),
}

impl CreateReceiptRuleError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateReceiptRuleError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AlreadyExists" => {
                        return RusotoError::Service(CreateReceiptRuleError::AlreadyExists(
                            parsed_error.message,
                        ))
                    }
                    "InvalidLambdaFunction" => {
                        return RusotoError::Service(CreateReceiptRuleError::InvalidLambdaFunction(
                            parsed_error.message,
                        ))
                    }
                    "InvalidS3Configuration" => {
                        return RusotoError::Service(
                            CreateReceiptRuleError::InvalidS3Configuration(parsed_error.message),
                        )
                    }
                    "InvalidSnsTopic" => {
                        return RusotoError::Service(CreateReceiptRuleError::InvalidSnsTopic(
                            parsed_error.message,
                        ))
                    }
                    "LimitExceeded" => {
                        return RusotoError::Service(CreateReceiptRuleError::LimitExceeded(
                            parsed_error.message,
                        ))
                    }
                    "RuleDoesNotExist" => {
                        return RusotoError::Service(CreateReceiptRuleError::RuleDoesNotExist(
                            parsed_error.message,
                        ))
                    }
                    "RuleSetDoesNotExist" => {
                        return RusotoError::Service(CreateReceiptRuleError::RuleSetDoesNotExist(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for CreateReceiptRuleError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateReceiptRuleError::AlreadyExists(ref cause) => write!(f, "{}", cause),
            CreateReceiptRuleError::InvalidLambdaFunction(ref cause) => write!(f, "{}", cause),
            CreateReceiptRuleError::InvalidS3Configuration(ref cause) => write!(f, "{}", cause),
            CreateReceiptRuleError::InvalidSnsTopic(ref cause) => write!(f, "{}", cause),
            CreateReceiptRuleError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateReceiptRuleError::RuleDoesNotExist(ref cause) => write!(f, "{}", cause),
            CreateReceiptRuleError::RuleSetDoesNotExist(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateReceiptRuleError {}
/// Errors returned by CreateReceiptRuleSet
#[derive(Debug, PartialEq)]
pub enum CreateReceiptRuleSetError {
    /// <p>Indicates that a resource could not be created because of a naming conflict.</p>
    AlreadyExists(String),
    /// <p>Indicates that a resource could not be created because of service limits. For a list of Amazon SES limits, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/limits.html">Amazon SES Developer Guide</a>.</p>
    LimitExceeded(String),
}

impl CreateReceiptRuleSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateReceiptRuleSetError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AlreadyExists" => {
                        return RusotoError::Service(CreateReceiptRuleSetError::AlreadyExists(
                            parsed_error.message,
                        ))
                    }
                    "LimitExceeded" => {
                        return RusotoError::Service(CreateReceiptRuleSetError::LimitExceeded(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for CreateReceiptRuleSetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateReceiptRuleSetError::AlreadyExists(ref cause) => write!(f, "{}", cause),
            CreateReceiptRuleSetError::LimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateReceiptRuleSetError {}
/// Errors returned by CreateTemplate
#[derive(Debug, PartialEq)]
pub enum CreateTemplateError {
    /// <p>Indicates that a resource could not be created because of a naming conflict.</p>
    AlreadyExists(String),
    /// <p>Indicates that the template that you specified could not be rendered. This issue may occur when a template refers to a partial that does not exist.</p>
    InvalidTemplate(String),
    /// <p>Indicates that a resource could not be created because of service limits. For a list of Amazon SES limits, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/limits.html">Amazon SES Developer Guide</a>.</p>
    LimitExceeded(String),
}

impl CreateTemplateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateTemplateError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AlreadyExists" => {
                        return RusotoError::Service(CreateTemplateError::AlreadyExists(
                            parsed_error.message,
                        ))
                    }
                    "InvalidTemplate" => {
                        return RusotoError::Service(CreateTemplateError::InvalidTemplate(
                            parsed_error.message,
                        ))
                    }
                    "LimitExceeded" => {
                        return RusotoError::Service(CreateTemplateError::LimitExceeded(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for CreateTemplateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateTemplateError::AlreadyExists(ref cause) => write!(f, "{}", cause),
            CreateTemplateError::InvalidTemplate(ref cause) => write!(f, "{}", cause),
            CreateTemplateError::LimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateTemplateError {}
/// Errors returned by DeleteConfigurationSet
#[derive(Debug, PartialEq)]
pub enum DeleteConfigurationSetError {
    /// <p>Indicates that the configuration set does not exist.</p>
    ConfigurationSetDoesNotExist(String),
}

impl DeleteConfigurationSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteConfigurationSetError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "ConfigurationSetDoesNotExist" => {
                        return RusotoError::Service(
                            DeleteConfigurationSetError::ConfigurationSetDoesNotExist(
                                parsed_error.message,
                            ),
                        )
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DeleteConfigurationSetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteConfigurationSetError::ConfigurationSetDoesNotExist(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DeleteConfigurationSetError {}
/// Errors returned by DeleteConfigurationSetEventDestination
#[derive(Debug, PartialEq)]
pub enum DeleteConfigurationSetEventDestinationError {
    /// <p>Indicates that the configuration set does not exist.</p>
    ConfigurationSetDoesNotExist(String),
    /// <p>Indicates that the event destination does not exist.</p>
    EventDestinationDoesNotExist(String),
}

impl DeleteConfigurationSetEventDestinationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteConfigurationSetEventDestinationError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "ConfigurationSetDoesNotExist" => return RusotoError::Service(
                        DeleteConfigurationSetEventDestinationError::ConfigurationSetDoesNotExist(
                            parsed_error.message,
                        ),
                    ),
                    "EventDestinationDoesNotExist" => return RusotoError::Service(
                        DeleteConfigurationSetEventDestinationError::EventDestinationDoesNotExist(
                            parsed_error.message,
                        ),
                    ),
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DeleteConfigurationSetEventDestinationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteConfigurationSetEventDestinationError::ConfigurationSetDoesNotExist(
                ref cause,
            ) => write!(f, "{}", cause),
            DeleteConfigurationSetEventDestinationError::EventDestinationDoesNotExist(
                ref cause,
            ) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteConfigurationSetEventDestinationError {}
/// Errors returned by DeleteConfigurationSetTrackingOptions
#[derive(Debug, PartialEq)]
pub enum DeleteConfigurationSetTrackingOptionsError {
    /// <p>Indicates that the configuration set does not exist.</p>
    ConfigurationSetDoesNotExist(String),
    /// <p>Indicates that the TrackingOptions object you specified does not exist.</p>
    TrackingOptionsDoesNotExist(String),
}

impl DeleteConfigurationSetTrackingOptionsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteConfigurationSetTrackingOptionsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "ConfigurationSetDoesNotExist" => return RusotoError::Service(
                        DeleteConfigurationSetTrackingOptionsError::ConfigurationSetDoesNotExist(
                            parsed_error.message,
                        ),
                    ),
                    "TrackingOptionsDoesNotExistException" => {
                        return RusotoError::Service(
                            DeleteConfigurationSetTrackingOptionsError::TrackingOptionsDoesNotExist(
                                parsed_error.message,
                            ),
                        )
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DeleteConfigurationSetTrackingOptionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteConfigurationSetTrackingOptionsError::ConfigurationSetDoesNotExist(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteConfigurationSetTrackingOptionsError::TrackingOptionsDoesNotExist(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DeleteConfigurationSetTrackingOptionsError {}
/// Errors returned by DeleteCustomVerificationEmailTemplate
#[derive(Debug, PartialEq)]
pub enum DeleteCustomVerificationEmailTemplateError {}

impl DeleteCustomVerificationEmailTemplateError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteCustomVerificationEmailTemplateError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DeleteCustomVerificationEmailTemplateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for DeleteCustomVerificationEmailTemplateError {}
/// Errors returned by DeleteIdentity
#[derive(Debug, PartialEq)]
pub enum DeleteIdentityError {}

impl DeleteIdentityError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteIdentityError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DeleteIdentityError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for DeleteIdentityError {}
/// Errors returned by DeleteIdentityPolicy
#[derive(Debug, PartialEq)]
pub enum DeleteIdentityPolicyError {}

impl DeleteIdentityPolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteIdentityPolicyError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DeleteIdentityPolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for DeleteIdentityPolicyError {}
/// Errors returned by DeleteReceiptFilter
#[derive(Debug, PartialEq)]
pub enum DeleteReceiptFilterError {}

impl DeleteReceiptFilterError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteReceiptFilterError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DeleteReceiptFilterError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for DeleteReceiptFilterError {}
/// Errors returned by DeleteReceiptRule
#[derive(Debug, PartialEq)]
pub enum DeleteReceiptRuleError {
    /// <p>Indicates that the provided receipt rule set does not exist.</p>
    RuleSetDoesNotExist(String),
}

impl DeleteReceiptRuleError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteReceiptRuleError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "RuleSetDoesNotExist" => {
                        return RusotoError::Service(DeleteReceiptRuleError::RuleSetDoesNotExist(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DeleteReceiptRuleError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteReceiptRuleError::RuleSetDoesNotExist(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteReceiptRuleError {}
/// Errors returned by DeleteReceiptRuleSet
#[derive(Debug, PartialEq)]
pub enum DeleteReceiptRuleSetError {
    /// <p>Indicates that the delete operation could not be completed.</p>
    CannotDelete(String),
}

impl DeleteReceiptRuleSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteReceiptRuleSetError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "CannotDelete" => {
                        return RusotoError::Service(DeleteReceiptRuleSetError::CannotDelete(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DeleteReceiptRuleSetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteReceiptRuleSetError::CannotDelete(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteReceiptRuleSetError {}
/// Errors returned by DeleteTemplate
#[derive(Debug, PartialEq)]
pub enum DeleteTemplateError {}

impl DeleteTemplateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteTemplateError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DeleteTemplateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for DeleteTemplateError {}
/// Errors returned by DeleteVerifiedEmailAddress
#[derive(Debug, PartialEq)]
pub enum DeleteVerifiedEmailAddressError {}

impl DeleteVerifiedEmailAddressError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteVerifiedEmailAddressError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DeleteVerifiedEmailAddressError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for DeleteVerifiedEmailAddressError {}
/// Errors returned by DescribeActiveReceiptRuleSet
#[derive(Debug, PartialEq)]
pub enum DescribeActiveReceiptRuleSetError {}

impl DescribeActiveReceiptRuleSetError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeActiveReceiptRuleSetError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DescribeActiveReceiptRuleSetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for DescribeActiveReceiptRuleSetError {}
/// Errors returned by DescribeConfigurationSet
#[derive(Debug, PartialEq)]
pub enum DescribeConfigurationSetError {
    /// <p>Indicates that the configuration set does not exist.</p>
    ConfigurationSetDoesNotExist(String),
}

impl DescribeConfigurationSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeConfigurationSetError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "ConfigurationSetDoesNotExist" => {
                        return RusotoError::Service(
                            DescribeConfigurationSetError::ConfigurationSetDoesNotExist(
                                parsed_error.message,
                            ),
                        )
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DescribeConfigurationSetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeConfigurationSetError::ConfigurationSetDoesNotExist(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeConfigurationSetError {}
/// Errors returned by DescribeReceiptRule
#[derive(Debug, PartialEq)]
pub enum DescribeReceiptRuleError {
    /// <p>Indicates that the provided receipt rule does not exist.</p>
    RuleDoesNotExist(String),
    /// <p>Indicates that the provided receipt rule set does not exist.</p>
    RuleSetDoesNotExist(String),
}

impl DescribeReceiptRuleError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeReceiptRuleError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "RuleDoesNotExist" => {
                        return RusotoError::Service(DescribeReceiptRuleError::RuleDoesNotExist(
                            parsed_error.message,
                        ))
                    }
                    "RuleSetDoesNotExist" => {
                        return RusotoError::Service(DescribeReceiptRuleError::RuleSetDoesNotExist(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DescribeReceiptRuleError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeReceiptRuleError::RuleDoesNotExist(ref cause) => write!(f, "{}", cause),
            DescribeReceiptRuleError::RuleSetDoesNotExist(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeReceiptRuleError {}
/// Errors returned by DescribeReceiptRuleSet
#[derive(Debug, PartialEq)]
pub enum DescribeReceiptRuleSetError {
    /// <p>Indicates that the provided receipt rule set does not exist.</p>
    RuleSetDoesNotExist(String),
}

impl DescribeReceiptRuleSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeReceiptRuleSetError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "RuleSetDoesNotExist" => {
                        return RusotoError::Service(
                            DescribeReceiptRuleSetError::RuleSetDoesNotExist(parsed_error.message),
                        )
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DescribeReceiptRuleSetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeReceiptRuleSetError::RuleSetDoesNotExist(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeReceiptRuleSetError {}
/// Errors returned by GetAccountSendingEnabled
#[derive(Debug, PartialEq)]
pub enum GetAccountSendingEnabledError {}

impl GetAccountSendingEnabledError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetAccountSendingEnabledError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for GetAccountSendingEnabledError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for GetAccountSendingEnabledError {}
/// Errors returned by GetCustomVerificationEmailTemplate
#[derive(Debug, PartialEq)]
pub enum GetCustomVerificationEmailTemplateError {
    /// <p>Indicates that a custom verification email template with the name you specified does not exist.</p>
    CustomVerificationEmailTemplateDoesNotExist(String),
}

impl GetCustomVerificationEmailTemplateError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetCustomVerificationEmailTemplateError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                                    "CustomVerificationEmailTemplateDoesNotExist" => return RusotoError::Service(GetCustomVerificationEmailTemplateError::CustomVerificationEmailTemplateDoesNotExist(parsed_error.message)),_ => {}
                                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for GetCustomVerificationEmailTemplateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
                            GetCustomVerificationEmailTemplateError::CustomVerificationEmailTemplateDoesNotExist(ref cause) => write!(f, "{}", cause)
                        }
    }
}
impl Error for GetCustomVerificationEmailTemplateError {}
/// Errors returned by GetIdentityDkimAttributes
#[derive(Debug, PartialEq)]
pub enum GetIdentityDkimAttributesError {}

impl GetIdentityDkimAttributesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetIdentityDkimAttributesError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for GetIdentityDkimAttributesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for GetIdentityDkimAttributesError {}
/// Errors returned by GetIdentityMailFromDomainAttributes
#[derive(Debug, PartialEq)]
pub enum GetIdentityMailFromDomainAttributesError {}

impl GetIdentityMailFromDomainAttributesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetIdentityMailFromDomainAttributesError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for GetIdentityMailFromDomainAttributesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for GetIdentityMailFromDomainAttributesError {}
/// Errors returned by GetIdentityNotificationAttributes
#[derive(Debug, PartialEq)]
pub enum GetIdentityNotificationAttributesError {}

impl GetIdentityNotificationAttributesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetIdentityNotificationAttributesError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for GetIdentityNotificationAttributesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for GetIdentityNotificationAttributesError {}
/// Errors returned by GetIdentityPolicies
#[derive(Debug, PartialEq)]
pub enum GetIdentityPoliciesError {}

impl GetIdentityPoliciesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetIdentityPoliciesError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for GetIdentityPoliciesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for GetIdentityPoliciesError {}
/// Errors returned by GetIdentityVerificationAttributes
#[derive(Debug, PartialEq)]
pub enum GetIdentityVerificationAttributesError {}

impl GetIdentityVerificationAttributesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetIdentityVerificationAttributesError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for GetIdentityVerificationAttributesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for GetIdentityVerificationAttributesError {}
/// Errors returned by GetSendQuota
#[derive(Debug, PartialEq)]
pub enum GetSendQuotaError {}

impl GetSendQuotaError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetSendQuotaError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for GetSendQuotaError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for GetSendQuotaError {}
/// Errors returned by GetSendStatistics
#[derive(Debug, PartialEq)]
pub enum GetSendStatisticsError {}

impl GetSendStatisticsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetSendStatisticsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for GetSendStatisticsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for GetSendStatisticsError {}
/// Errors returned by GetTemplate
#[derive(Debug, PartialEq)]
pub enum GetTemplateError {
    /// <p>Indicates that the Template object you specified does not exist in your Amazon SES account.</p>
    TemplateDoesNotExist(String),
}

impl GetTemplateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetTemplateError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "TemplateDoesNotExist" => {
                        return RusotoError::Service(GetTemplateError::TemplateDoesNotExist(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for GetTemplateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetTemplateError::TemplateDoesNotExist(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetTemplateError {}
/// Errors returned by ListConfigurationSets
#[derive(Debug, PartialEq)]
pub enum ListConfigurationSetsError {}

impl ListConfigurationSetsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListConfigurationSetsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for ListConfigurationSetsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for ListConfigurationSetsError {}
/// Errors returned by ListCustomVerificationEmailTemplates
#[derive(Debug, PartialEq)]
pub enum ListCustomVerificationEmailTemplatesError {}

impl ListCustomVerificationEmailTemplatesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListCustomVerificationEmailTemplatesError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for ListCustomVerificationEmailTemplatesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for ListCustomVerificationEmailTemplatesError {}
/// Errors returned by ListIdentities
#[derive(Debug, PartialEq)]
pub enum ListIdentitiesError {}

impl ListIdentitiesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListIdentitiesError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for ListIdentitiesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for ListIdentitiesError {}
/// Errors returned by ListIdentityPolicies
#[derive(Debug, PartialEq)]
pub enum ListIdentityPoliciesError {}

impl ListIdentityPoliciesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListIdentityPoliciesError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for ListIdentityPoliciesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for ListIdentityPoliciesError {}
/// Errors returned by ListReceiptFilters
#[derive(Debug, PartialEq)]
pub enum ListReceiptFiltersError {}

impl ListReceiptFiltersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListReceiptFiltersError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for ListReceiptFiltersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for ListReceiptFiltersError {}
/// Errors returned by ListReceiptRuleSets
#[derive(Debug, PartialEq)]
pub enum ListReceiptRuleSetsError {}

impl ListReceiptRuleSetsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListReceiptRuleSetsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for ListReceiptRuleSetsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for ListReceiptRuleSetsError {}
/// Errors returned by ListTemplates
#[derive(Debug, PartialEq)]
pub enum ListTemplatesError {}

impl ListTemplatesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTemplatesError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for ListTemplatesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for ListTemplatesError {}
/// Errors returned by ListVerifiedEmailAddresses
#[derive(Debug, PartialEq)]
pub enum ListVerifiedEmailAddressesError {}

impl ListVerifiedEmailAddressesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListVerifiedEmailAddressesError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for ListVerifiedEmailAddressesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for ListVerifiedEmailAddressesError {}
/// Errors returned by PutConfigurationSetDeliveryOptions
#[derive(Debug, PartialEq)]
pub enum PutConfigurationSetDeliveryOptionsError {
    /// <p>Indicates that the configuration set does not exist.</p>
    ConfigurationSetDoesNotExist(String),
    /// <p>Indicates that provided delivery option is invalid.</p>
    InvalidDeliveryOptions(String),
}

impl PutConfigurationSetDeliveryOptionsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<PutConfigurationSetDeliveryOptionsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "ConfigurationSetDoesNotExist" => {
                        return RusotoError::Service(
                            PutConfigurationSetDeliveryOptionsError::ConfigurationSetDoesNotExist(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidDeliveryOptions" => {
                        return RusotoError::Service(
                            PutConfigurationSetDeliveryOptionsError::InvalidDeliveryOptions(
                                parsed_error.message,
                            ),
                        )
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for PutConfigurationSetDeliveryOptionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutConfigurationSetDeliveryOptionsError::ConfigurationSetDoesNotExist(ref cause) => {
                write!(f, "{}", cause)
            }
            PutConfigurationSetDeliveryOptionsError::InvalidDeliveryOptions(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for PutConfigurationSetDeliveryOptionsError {}
/// Errors returned by PutIdentityPolicy
#[derive(Debug, PartialEq)]
pub enum PutIdentityPolicyError {
    /// <p>Indicates that the provided policy is invalid. Check the error stack for more information about what caused the error.</p>
    InvalidPolicy(String),
}

impl PutIdentityPolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutIdentityPolicyError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidPolicy" => {
                        return RusotoError::Service(PutIdentityPolicyError::InvalidPolicy(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for PutIdentityPolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutIdentityPolicyError::InvalidPolicy(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutIdentityPolicyError {}
/// Errors returned by ReorderReceiptRuleSet
#[derive(Debug, PartialEq)]
pub enum ReorderReceiptRuleSetError {
    /// <p>Indicates that the provided receipt rule does not exist.</p>
    RuleDoesNotExist(String),
    /// <p>Indicates that the provided receipt rule set does not exist.</p>
    RuleSetDoesNotExist(String),
}

impl ReorderReceiptRuleSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ReorderReceiptRuleSetError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "RuleDoesNotExist" => {
                        return RusotoError::Service(ReorderReceiptRuleSetError::RuleDoesNotExist(
                            parsed_error.message,
                        ))
                    }
                    "RuleSetDoesNotExist" => {
                        return RusotoError::Service(
                            ReorderReceiptRuleSetError::RuleSetDoesNotExist(parsed_error.message),
                        )
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for ReorderReceiptRuleSetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ReorderReceiptRuleSetError::RuleDoesNotExist(ref cause) => write!(f, "{}", cause),
            ReorderReceiptRuleSetError::RuleSetDoesNotExist(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ReorderReceiptRuleSetError {}
/// Errors returned by SendBounce
#[derive(Debug, PartialEq)]
pub enum SendBounceError {
    /// <p>Indicates that the action failed, and the message could not be sent. Check the error stack for more information about what caused the error.</p>
    MessageRejected(String),
}

impl SendBounceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SendBounceError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "MessageRejected" => {
                        return RusotoError::Service(SendBounceError::MessageRejected(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for SendBounceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SendBounceError::MessageRejected(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for SendBounceError {}
/// Errors returned by SendBulkTemplatedEmail
#[derive(Debug, PartialEq)]
pub enum SendBulkTemplatedEmailError {
    /// <p>Indicates that email sending is disabled for your entire Amazon SES account.</p> <p>You can enable or disable email sending for your Amazon SES account using <a>UpdateAccountSendingEnabled</a>.</p>
    AccountSendingPaused(String),
    /// <p>Indicates that the configuration set does not exist.</p>
    ConfigurationSetDoesNotExist(String),
    /// <p>Indicates that email sending is disabled for the configuration set.</p> <p>You can enable or disable email sending for a configuration set using <a>UpdateConfigurationSetSendingEnabled</a>.</p>
    ConfigurationSetSendingPaused(String),
    /// <p> Indicates that the message could not be sent because Amazon SES could not read the MX record required to use the specified MAIL FROM domain. For information about editing the custom MAIL FROM domain settings for an identity, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/mail-from-edit.html">Amazon SES Developer Guide</a>.</p>
    MailFromDomainNotVerified(String),
    /// <p>Indicates that the action failed, and the message could not be sent. Check the error stack for more information about what caused the error.</p>
    MessageRejected(String),
    /// <p>Indicates that the Template object you specified does not exist in your Amazon SES account.</p>
    TemplateDoesNotExist(String),
}

impl SendBulkTemplatedEmailError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SendBulkTemplatedEmailError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AccountSendingPausedException" => {
                        return RusotoError::Service(
                            SendBulkTemplatedEmailError::AccountSendingPaused(parsed_error.message),
                        )
                    }
                    "ConfigurationSetDoesNotExist" => {
                        return RusotoError::Service(
                            SendBulkTemplatedEmailError::ConfigurationSetDoesNotExist(
                                parsed_error.message,
                            ),
                        )
                    }
                    "ConfigurationSetSendingPausedException" => {
                        return RusotoError::Service(
                            SendBulkTemplatedEmailError::ConfigurationSetSendingPaused(
                                parsed_error.message,
                            ),
                        )
                    }
                    "MailFromDomainNotVerifiedException" => {
                        return RusotoError::Service(
                            SendBulkTemplatedEmailError::MailFromDomainNotVerified(
                                parsed_error.message,
                            ),
                        )
                    }
                    "MessageRejected" => {
                        return RusotoError::Service(SendBulkTemplatedEmailError::MessageRejected(
                            parsed_error.message,
                        ))
                    }
                    "TemplateDoesNotExist" => {
                        return RusotoError::Service(
                            SendBulkTemplatedEmailError::TemplateDoesNotExist(parsed_error.message),
                        )
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for SendBulkTemplatedEmailError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SendBulkTemplatedEmailError::AccountSendingPaused(ref cause) => write!(f, "{}", cause),
            SendBulkTemplatedEmailError::ConfigurationSetDoesNotExist(ref cause) => {
                write!(f, "{}", cause)
            }
            SendBulkTemplatedEmailError::ConfigurationSetSendingPaused(ref cause) => {
                write!(f, "{}", cause)
            }
            SendBulkTemplatedEmailError::MailFromDomainNotVerified(ref cause) => {
                write!(f, "{}", cause)
            }
            SendBulkTemplatedEmailError::MessageRejected(ref cause) => write!(f, "{}", cause),
            SendBulkTemplatedEmailError::TemplateDoesNotExist(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for SendBulkTemplatedEmailError {}
/// Errors returned by SendCustomVerificationEmail
#[derive(Debug, PartialEq)]
pub enum SendCustomVerificationEmailError {
    /// <p>Indicates that the configuration set does not exist.</p>
    ConfigurationSetDoesNotExist(String),
    /// <p>Indicates that a custom verification email template with the name you specified does not exist.</p>
    CustomVerificationEmailTemplateDoesNotExist(String),
    /// <p>Indicates that the sender address specified for a custom verification email is not verified, and is therefore not eligible to send the custom verification email. </p>
    FromEmailAddressNotVerified(String),
    /// <p>Indicates that the action failed, and the message could not be sent. Check the error stack for more information about what caused the error.</p>
    MessageRejected(String),
    /// <p>Indicates that the account has not been granted production access.</p>
    ProductionAccessNotGranted(String),
}

impl SendCustomVerificationEmailError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<SendCustomVerificationEmailError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                                    "ConfigurationSetDoesNotExist" => return RusotoError::Service(SendCustomVerificationEmailError::ConfigurationSetDoesNotExist(parsed_error.message)),"CustomVerificationEmailTemplateDoesNotExist" => return RusotoError::Service(SendCustomVerificationEmailError::CustomVerificationEmailTemplateDoesNotExist(parsed_error.message)),"FromEmailAddressNotVerified" => return RusotoError::Service(SendCustomVerificationEmailError::FromEmailAddressNotVerified(parsed_error.message)),"MessageRejected" => return RusotoError::Service(SendCustomVerificationEmailError::MessageRejected(parsed_error.message)),"ProductionAccessNotGranted" => return RusotoError::Service(SendCustomVerificationEmailError::ProductionAccessNotGranted(parsed_error.message)),_ => {}
                                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for SendCustomVerificationEmailError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SendCustomVerificationEmailError::ConfigurationSetDoesNotExist(ref cause) => {
                write!(f, "{}", cause)
            }
            SendCustomVerificationEmailError::CustomVerificationEmailTemplateDoesNotExist(
                ref cause,
            ) => write!(f, "{}", cause),
            SendCustomVerificationEmailError::FromEmailAddressNotVerified(ref cause) => {
                write!(f, "{}", cause)
            }
            SendCustomVerificationEmailError::MessageRejected(ref cause) => write!(f, "{}", cause),
            SendCustomVerificationEmailError::ProductionAccessNotGranted(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for SendCustomVerificationEmailError {}
/// Errors returned by SendEmail
#[derive(Debug, PartialEq)]
pub enum SendEmailError {
    /// <p>Indicates that email sending is disabled for your entire Amazon SES account.</p> <p>You can enable or disable email sending for your Amazon SES account using <a>UpdateAccountSendingEnabled</a>.</p>
    AccountSendingPaused(String),
    /// <p>Indicates that the configuration set does not exist.</p>
    ConfigurationSetDoesNotExist(String),
    /// <p>Indicates that email sending is disabled for the configuration set.</p> <p>You can enable or disable email sending for a configuration set using <a>UpdateConfigurationSetSendingEnabled</a>.</p>
    ConfigurationSetSendingPaused(String),
    /// <p> Indicates that the message could not be sent because Amazon SES could not read the MX record required to use the specified MAIL FROM domain. For information about editing the custom MAIL FROM domain settings for an identity, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/mail-from-edit.html">Amazon SES Developer Guide</a>.</p>
    MailFromDomainNotVerified(String),
    /// <p>Indicates that the action failed, and the message could not be sent. Check the error stack for more information about what caused the error.</p>
    MessageRejected(String),
}

impl SendEmailError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SendEmailError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AccountSendingPausedException" => {
                        return RusotoError::Service(SendEmailError::AccountSendingPaused(
                            parsed_error.message,
                        ))
                    }
                    "ConfigurationSetDoesNotExist" => {
                        return RusotoError::Service(SendEmailError::ConfigurationSetDoesNotExist(
                            parsed_error.message,
                        ))
                    }
                    "ConfigurationSetSendingPausedException" => {
                        return RusotoError::Service(SendEmailError::ConfigurationSetSendingPaused(
                            parsed_error.message,
                        ))
                    }
                    "MailFromDomainNotVerifiedException" => {
                        return RusotoError::Service(SendEmailError::MailFromDomainNotVerified(
                            parsed_error.message,
                        ))
                    }
                    "MessageRejected" => {
                        return RusotoError::Service(SendEmailError::MessageRejected(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for SendEmailError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SendEmailError::AccountSendingPaused(ref cause) => write!(f, "{}", cause),
            SendEmailError::ConfigurationSetDoesNotExist(ref cause) => write!(f, "{}", cause),
            SendEmailError::ConfigurationSetSendingPaused(ref cause) => write!(f, "{}", cause),
            SendEmailError::MailFromDomainNotVerified(ref cause) => write!(f, "{}", cause),
            SendEmailError::MessageRejected(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for SendEmailError {}
/// Errors returned by SendRawEmail
#[derive(Debug, PartialEq)]
pub enum SendRawEmailError {
    /// <p>Indicates that email sending is disabled for your entire Amazon SES account.</p> <p>You can enable or disable email sending for your Amazon SES account using <a>UpdateAccountSendingEnabled</a>.</p>
    AccountSendingPaused(String),
    /// <p>Indicates that the configuration set does not exist.</p>
    ConfigurationSetDoesNotExist(String),
    /// <p>Indicates that email sending is disabled for the configuration set.</p> <p>You can enable or disable email sending for a configuration set using <a>UpdateConfigurationSetSendingEnabled</a>.</p>
    ConfigurationSetSendingPaused(String),
    /// <p> Indicates that the message could not be sent because Amazon SES could not read the MX record required to use the specified MAIL FROM domain. For information about editing the custom MAIL FROM domain settings for an identity, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/mail-from-edit.html">Amazon SES Developer Guide</a>.</p>
    MailFromDomainNotVerified(String),
    /// <p>Indicates that the action failed, and the message could not be sent. Check the error stack for more information about what caused the error.</p>
    MessageRejected(String),
}

impl SendRawEmailError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SendRawEmailError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AccountSendingPausedException" => {
                        return RusotoError::Service(SendRawEmailError::AccountSendingPaused(
                            parsed_error.message,
                        ))
                    }
                    "ConfigurationSetDoesNotExist" => {
                        return RusotoError::Service(
                            SendRawEmailError::ConfigurationSetDoesNotExist(parsed_error.message),
                        )
                    }
                    "ConfigurationSetSendingPausedException" => {
                        return RusotoError::Service(
                            SendRawEmailError::ConfigurationSetSendingPaused(parsed_error.message),
                        )
                    }
                    "MailFromDomainNotVerifiedException" => {
                        return RusotoError::Service(SendRawEmailError::MailFromDomainNotVerified(
                            parsed_error.message,
                        ))
                    }
                    "MessageRejected" => {
                        return RusotoError::Service(SendRawEmailError::MessageRejected(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for SendRawEmailError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SendRawEmailError::AccountSendingPaused(ref cause) => write!(f, "{}", cause),
            SendRawEmailError::ConfigurationSetDoesNotExist(ref cause) => write!(f, "{}", cause),
            SendRawEmailError::ConfigurationSetSendingPaused(ref cause) => write!(f, "{}", cause),
            SendRawEmailError::MailFromDomainNotVerified(ref cause) => write!(f, "{}", cause),
            SendRawEmailError::MessageRejected(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for SendRawEmailError {}
/// Errors returned by SendTemplatedEmail
#[derive(Debug, PartialEq)]
pub enum SendTemplatedEmailError {
    /// <p>Indicates that email sending is disabled for your entire Amazon SES account.</p> <p>You can enable or disable email sending for your Amazon SES account using <a>UpdateAccountSendingEnabled</a>.</p>
    AccountSendingPaused(String),
    /// <p>Indicates that the configuration set does not exist.</p>
    ConfigurationSetDoesNotExist(String),
    /// <p>Indicates that email sending is disabled for the configuration set.</p> <p>You can enable or disable email sending for a configuration set using <a>UpdateConfigurationSetSendingEnabled</a>.</p>
    ConfigurationSetSendingPaused(String),
    /// <p> Indicates that the message could not be sent because Amazon SES could not read the MX record required to use the specified MAIL FROM domain. For information about editing the custom MAIL FROM domain settings for an identity, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/mail-from-edit.html">Amazon SES Developer Guide</a>.</p>
    MailFromDomainNotVerified(String),
    /// <p>Indicates that the action failed, and the message could not be sent. Check the error stack for more information about what caused the error.</p>
    MessageRejected(String),
    /// <p>Indicates that the Template object you specified does not exist in your Amazon SES account.</p>
    TemplateDoesNotExist(String),
}

impl SendTemplatedEmailError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SendTemplatedEmailError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AccountSendingPausedException" => {
                        return RusotoError::Service(SendTemplatedEmailError::AccountSendingPaused(
                            parsed_error.message,
                        ))
                    }
                    "ConfigurationSetDoesNotExist" => {
                        return RusotoError::Service(
                            SendTemplatedEmailError::ConfigurationSetDoesNotExist(
                                parsed_error.message,
                            ),
                        )
                    }
                    "ConfigurationSetSendingPausedException" => {
                        return RusotoError::Service(
                            SendTemplatedEmailError::ConfigurationSetSendingPaused(
                                parsed_error.message,
                            ),
                        )
                    }
                    "MailFromDomainNotVerifiedException" => {
                        return RusotoError::Service(
                            SendTemplatedEmailError::MailFromDomainNotVerified(
                                parsed_error.message,
                            ),
                        )
                    }
                    "MessageRejected" => {
                        return RusotoError::Service(SendTemplatedEmailError::MessageRejected(
                            parsed_error.message,
                        ))
                    }
                    "TemplateDoesNotExist" => {
                        return RusotoError::Service(SendTemplatedEmailError::TemplateDoesNotExist(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for SendTemplatedEmailError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SendTemplatedEmailError::AccountSendingPaused(ref cause) => write!(f, "{}", cause),
            SendTemplatedEmailError::ConfigurationSetDoesNotExist(ref cause) => {
                write!(f, "{}", cause)
            }
            SendTemplatedEmailError::ConfigurationSetSendingPaused(ref cause) => {
                write!(f, "{}", cause)
            }
            SendTemplatedEmailError::MailFromDomainNotVerified(ref cause) => write!(f, "{}", cause),
            SendTemplatedEmailError::MessageRejected(ref cause) => write!(f, "{}", cause),
            SendTemplatedEmailError::TemplateDoesNotExist(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for SendTemplatedEmailError {}
/// Errors returned by SetActiveReceiptRuleSet
#[derive(Debug, PartialEq)]
pub enum SetActiveReceiptRuleSetError {
    /// <p>Indicates that the provided receipt rule set does not exist.</p>
    RuleSetDoesNotExist(String),
}

impl SetActiveReceiptRuleSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SetActiveReceiptRuleSetError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "RuleSetDoesNotExist" => {
                        return RusotoError::Service(
                            SetActiveReceiptRuleSetError::RuleSetDoesNotExist(parsed_error.message),
                        )
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for SetActiveReceiptRuleSetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SetActiveReceiptRuleSetError::RuleSetDoesNotExist(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for SetActiveReceiptRuleSetError {}
/// Errors returned by SetIdentityDkimEnabled
#[derive(Debug, PartialEq)]
pub enum SetIdentityDkimEnabledError {}

impl SetIdentityDkimEnabledError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SetIdentityDkimEnabledError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for SetIdentityDkimEnabledError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for SetIdentityDkimEnabledError {}
/// Errors returned by SetIdentityFeedbackForwardingEnabled
#[derive(Debug, PartialEq)]
pub enum SetIdentityFeedbackForwardingEnabledError {}

impl SetIdentityFeedbackForwardingEnabledError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<SetIdentityFeedbackForwardingEnabledError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for SetIdentityFeedbackForwardingEnabledError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for SetIdentityFeedbackForwardingEnabledError {}
/// Errors returned by SetIdentityHeadersInNotificationsEnabled
#[derive(Debug, PartialEq)]
pub enum SetIdentityHeadersInNotificationsEnabledError {}

impl SetIdentityHeadersInNotificationsEnabledError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<SetIdentityHeadersInNotificationsEnabledError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for SetIdentityHeadersInNotificationsEnabledError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for SetIdentityHeadersInNotificationsEnabledError {}
/// Errors returned by SetIdentityMailFromDomain
#[derive(Debug, PartialEq)]
pub enum SetIdentityMailFromDomainError {}

impl SetIdentityMailFromDomainError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SetIdentityMailFromDomainError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for SetIdentityMailFromDomainError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for SetIdentityMailFromDomainError {}
/// Errors returned by SetIdentityNotificationTopic
#[derive(Debug, PartialEq)]
pub enum SetIdentityNotificationTopicError {}

impl SetIdentityNotificationTopicError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<SetIdentityNotificationTopicError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for SetIdentityNotificationTopicError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for SetIdentityNotificationTopicError {}
/// Errors returned by SetReceiptRulePosition
#[derive(Debug, PartialEq)]
pub enum SetReceiptRulePositionError {
    /// <p>Indicates that the provided receipt rule does not exist.</p>
    RuleDoesNotExist(String),
    /// <p>Indicates that the provided receipt rule set does not exist.</p>
    RuleSetDoesNotExist(String),
}

impl SetReceiptRulePositionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SetReceiptRulePositionError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "RuleDoesNotExist" => {
                        return RusotoError::Service(SetReceiptRulePositionError::RuleDoesNotExist(
                            parsed_error.message,
                        ))
                    }
                    "RuleSetDoesNotExist" => {
                        return RusotoError::Service(
                            SetReceiptRulePositionError::RuleSetDoesNotExist(parsed_error.message),
                        )
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for SetReceiptRulePositionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SetReceiptRulePositionError::RuleDoesNotExist(ref cause) => write!(f, "{}", cause),
            SetReceiptRulePositionError::RuleSetDoesNotExist(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for SetReceiptRulePositionError {}
/// Errors returned by TestRenderTemplate
#[derive(Debug, PartialEq)]
pub enum TestRenderTemplateError {
    /// <p>Indicates that one or more of the replacement values you provided is invalid. This error may occur when the TemplateData object contains invalid JSON.</p>
    InvalidRenderingParameter(String),
    /// <p>Indicates that one or more of the replacement values for the specified template was not specified. Ensure that the TemplateData object contains references to all of the replacement tags in the specified template.</p>
    MissingRenderingAttribute(String),
    /// <p>Indicates that the Template object you specified does not exist in your Amazon SES account.</p>
    TemplateDoesNotExist(String),
}

impl TestRenderTemplateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TestRenderTemplateError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidRenderingParameter" => {
                        return RusotoError::Service(
                            TestRenderTemplateError::InvalidRenderingParameter(
                                parsed_error.message,
                            ),
                        )
                    }
                    "MissingRenderingAttribute" => {
                        return RusotoError::Service(
                            TestRenderTemplateError::MissingRenderingAttribute(
                                parsed_error.message,
                            ),
                        )
                    }
                    "TemplateDoesNotExist" => {
                        return RusotoError::Service(TestRenderTemplateError::TemplateDoesNotExist(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for TestRenderTemplateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TestRenderTemplateError::InvalidRenderingParameter(ref cause) => write!(f, "{}", cause),
            TestRenderTemplateError::MissingRenderingAttribute(ref cause) => write!(f, "{}", cause),
            TestRenderTemplateError::TemplateDoesNotExist(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TestRenderTemplateError {}
/// Errors returned by UpdateAccountSendingEnabled
#[derive(Debug, PartialEq)]
pub enum UpdateAccountSendingEnabledError {}

impl UpdateAccountSendingEnabledError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateAccountSendingEnabledError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for UpdateAccountSendingEnabledError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for UpdateAccountSendingEnabledError {}
/// Errors returned by UpdateConfigurationSetEventDestination
#[derive(Debug, PartialEq)]
pub enum UpdateConfigurationSetEventDestinationError {
    /// <p>Indicates that the configuration set does not exist.</p>
    ConfigurationSetDoesNotExist(String),
    /// <p>Indicates that the event destination does not exist.</p>
    EventDestinationDoesNotExist(String),
    /// <p>Indicates that the Amazon CloudWatch destination is invalid. See the error message for details.</p>
    InvalidCloudWatchDestination(String),
    /// <p>Indicates that the Amazon Kinesis Firehose destination is invalid. See the error message for details.</p>
    InvalidFirehoseDestination(String),
    /// <p>Indicates that the Amazon Simple Notification Service (Amazon SNS) destination is invalid. See the error message for details.</p>
    InvalidSNSDestination(String),
}

impl UpdateConfigurationSetEventDestinationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateConfigurationSetEventDestinationError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "ConfigurationSetDoesNotExist" => return RusotoError::Service(
                        UpdateConfigurationSetEventDestinationError::ConfigurationSetDoesNotExist(
                            parsed_error.message,
                        ),
                    ),
                    "EventDestinationDoesNotExist" => return RusotoError::Service(
                        UpdateConfigurationSetEventDestinationError::EventDestinationDoesNotExist(
                            parsed_error.message,
                        ),
                    ),
                    "InvalidCloudWatchDestination" => return RusotoError::Service(
                        UpdateConfigurationSetEventDestinationError::InvalidCloudWatchDestination(
                            parsed_error.message,
                        ),
                    ),
                    "InvalidFirehoseDestination" => {
                        return RusotoError::Service(
                            UpdateConfigurationSetEventDestinationError::InvalidFirehoseDestination(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidSNSDestination" => {
                        return RusotoError::Service(
                            UpdateConfigurationSetEventDestinationError::InvalidSNSDestination(
                                parsed_error.message,
                            ),
                        )
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for UpdateConfigurationSetEventDestinationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateConfigurationSetEventDestinationError::ConfigurationSetDoesNotExist(
                ref cause,
            ) => write!(f, "{}", cause),
            UpdateConfigurationSetEventDestinationError::EventDestinationDoesNotExist(
                ref cause,
            ) => write!(f, "{}", cause),
            UpdateConfigurationSetEventDestinationError::InvalidCloudWatchDestination(
                ref cause,
            ) => write!(f, "{}", cause),
            UpdateConfigurationSetEventDestinationError::InvalidFirehoseDestination(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateConfigurationSetEventDestinationError::InvalidSNSDestination(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for UpdateConfigurationSetEventDestinationError {}
/// Errors returned by UpdateConfigurationSetReputationMetricsEnabled
#[derive(Debug, PartialEq)]
pub enum UpdateConfigurationSetReputationMetricsEnabledError {
    /// <p>Indicates that the configuration set does not exist.</p>
    ConfigurationSetDoesNotExist(String),
}

impl UpdateConfigurationSetReputationMetricsEnabledError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateConfigurationSetReputationMetricsEnabledError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                                    "ConfigurationSetDoesNotExist" => return RusotoError::Service(UpdateConfigurationSetReputationMetricsEnabledError::ConfigurationSetDoesNotExist(parsed_error.message)),_ => {}
                                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for UpdateConfigurationSetReputationMetricsEnabledError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateConfigurationSetReputationMetricsEnabledError::ConfigurationSetDoesNotExist(
                ref cause,
            ) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateConfigurationSetReputationMetricsEnabledError {}
/// Errors returned by UpdateConfigurationSetSendingEnabled
#[derive(Debug, PartialEq)]
pub enum UpdateConfigurationSetSendingEnabledError {
    /// <p>Indicates that the configuration set does not exist.</p>
    ConfigurationSetDoesNotExist(String),
}

impl UpdateConfigurationSetSendingEnabledError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateConfigurationSetSendingEnabledError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "ConfigurationSetDoesNotExist" => {
                        return RusotoError::Service(
                            UpdateConfigurationSetSendingEnabledError::ConfigurationSetDoesNotExist(
                                parsed_error.message,
                            ),
                        )
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for UpdateConfigurationSetSendingEnabledError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateConfigurationSetSendingEnabledError::ConfigurationSetDoesNotExist(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for UpdateConfigurationSetSendingEnabledError {}
/// Errors returned by UpdateConfigurationSetTrackingOptions
#[derive(Debug, PartialEq)]
pub enum UpdateConfigurationSetTrackingOptionsError {
    /// <p>Indicates that the configuration set does not exist.</p>
    ConfigurationSetDoesNotExist(String),
    /// <p><p>Indicates that the custom domain to be used for open and click tracking redirects is invalid. This error appears most often in the following situations:</p> <ul> <li> <p>When the tracking domain you specified is not verified in Amazon SES.</p> </li> <li> <p>When the tracking domain you specified is not a valid domain or subdomain.</p> </li> </ul></p>
    InvalidTrackingOptions(String),
    /// <p>Indicates that the TrackingOptions object you specified does not exist.</p>
    TrackingOptionsDoesNotExist(String),
}

impl UpdateConfigurationSetTrackingOptionsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateConfigurationSetTrackingOptionsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "ConfigurationSetDoesNotExist" => return RusotoError::Service(
                        UpdateConfigurationSetTrackingOptionsError::ConfigurationSetDoesNotExist(
                            parsed_error.message,
                        ),
                    ),
                    "InvalidTrackingOptions" => {
                        return RusotoError::Service(
                            UpdateConfigurationSetTrackingOptionsError::InvalidTrackingOptions(
                                parsed_error.message,
                            ),
                        )
                    }
                    "TrackingOptionsDoesNotExistException" => {
                        return RusotoError::Service(
                            UpdateConfigurationSetTrackingOptionsError::TrackingOptionsDoesNotExist(
                                parsed_error.message,
                            ),
                        )
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for UpdateConfigurationSetTrackingOptionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateConfigurationSetTrackingOptionsError::ConfigurationSetDoesNotExist(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateConfigurationSetTrackingOptionsError::InvalidTrackingOptions(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateConfigurationSetTrackingOptionsError::TrackingOptionsDoesNotExist(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for UpdateConfigurationSetTrackingOptionsError {}
/// Errors returned by UpdateCustomVerificationEmailTemplate
#[derive(Debug, PartialEq)]
pub enum UpdateCustomVerificationEmailTemplateError {
    /// <p>Indicates that custom verification email template provided content is invalid.</p>
    CustomVerificationEmailInvalidContent(String),
    /// <p>Indicates that a custom verification email template with the name you specified does not exist.</p>
    CustomVerificationEmailTemplateDoesNotExist(String),
    /// <p>Indicates that the sender address specified for a custom verification email is not verified, and is therefore not eligible to send the custom verification email. </p>
    FromEmailAddressNotVerified(String),
}

impl UpdateCustomVerificationEmailTemplateError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateCustomVerificationEmailTemplateError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                                    "CustomVerificationEmailInvalidContent" => return RusotoError::Service(UpdateCustomVerificationEmailTemplateError::CustomVerificationEmailInvalidContent(parsed_error.message)),"CustomVerificationEmailTemplateDoesNotExist" => return RusotoError::Service(UpdateCustomVerificationEmailTemplateError::CustomVerificationEmailTemplateDoesNotExist(parsed_error.message)),"FromEmailAddressNotVerified" => return RusotoError::Service(UpdateCustomVerificationEmailTemplateError::FromEmailAddressNotVerified(parsed_error.message)),_ => {}
                                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for UpdateCustomVerificationEmailTemplateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
                            UpdateCustomVerificationEmailTemplateError::CustomVerificationEmailInvalidContent(ref cause) => write!(f, "{}", cause),
UpdateCustomVerificationEmailTemplateError::CustomVerificationEmailTemplateDoesNotExist(ref cause) => write!(f, "{}", cause),
UpdateCustomVerificationEmailTemplateError::FromEmailAddressNotVerified(ref cause) => write!(f, "{}", cause)
                        }
    }
}
impl Error for UpdateCustomVerificationEmailTemplateError {}
/// Errors returned by UpdateReceiptRule
#[derive(Debug, PartialEq)]
pub enum UpdateReceiptRuleError {
    /// <p>Indicates that the provided AWS Lambda function is invalid, or that Amazon SES could not execute the provided function, possibly due to permissions issues. For information about giving permissions, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-permissions.html">Amazon SES Developer Guide</a>.</p>
    InvalidLambdaFunction(String),
    /// <p>Indicates that the provided Amazon S3 bucket or AWS KMS encryption key is invalid, or that Amazon SES could not publish to the bucket, possibly due to permissions issues. For information about giving permissions, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-permissions.html">Amazon SES Developer Guide</a>.</p>
    InvalidS3Configuration(String),
    /// <p>Indicates that the provided Amazon SNS topic is invalid, or that Amazon SES could not publish to the topic, possibly due to permissions issues. For information about giving permissions, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-permissions.html">Amazon SES Developer Guide</a>.</p>
    InvalidSnsTopic(String),
    /// <p>Indicates that a resource could not be created because of service limits. For a list of Amazon SES limits, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/limits.html">Amazon SES Developer Guide</a>.</p>
    LimitExceeded(String),
    /// <p>Indicates that the provided receipt rule does not exist.</p>
    RuleDoesNotExist(String),
    /// <p>Indicates that the provided receipt rule set does not exist.</p>
    RuleSetDoesNotExist(String),
}

impl UpdateReceiptRuleError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateReceiptRuleError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidLambdaFunction" => {
                        return RusotoError::Service(UpdateReceiptRuleError::InvalidLambdaFunction(
                            parsed_error.message,
                        ))
                    }
                    "InvalidS3Configuration" => {
                        return RusotoError::Service(
                            UpdateReceiptRuleError::InvalidS3Configuration(parsed_error.message),
                        )
                    }
                    "InvalidSnsTopic" => {
                        return RusotoError::Service(UpdateReceiptRuleError::InvalidSnsTopic(
                            parsed_error.message,
                        ))
                    }
                    "LimitExceeded" => {
                        return RusotoError::Service(UpdateReceiptRuleError::LimitExceeded(
                            parsed_error.message,
                        ))
                    }
                    "RuleDoesNotExist" => {
                        return RusotoError::Service(UpdateReceiptRuleError::RuleDoesNotExist(
                            parsed_error.message,
                        ))
                    }
                    "RuleSetDoesNotExist" => {
                        return RusotoError::Service(UpdateReceiptRuleError::RuleSetDoesNotExist(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for UpdateReceiptRuleError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateReceiptRuleError::InvalidLambdaFunction(ref cause) => write!(f, "{}", cause),
            UpdateReceiptRuleError::InvalidS3Configuration(ref cause) => write!(f, "{}", cause),
            UpdateReceiptRuleError::InvalidSnsTopic(ref cause) => write!(f, "{}", cause),
            UpdateReceiptRuleError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            UpdateReceiptRuleError::RuleDoesNotExist(ref cause) => write!(f, "{}", cause),
            UpdateReceiptRuleError::RuleSetDoesNotExist(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateReceiptRuleError {}
/// Errors returned by UpdateTemplate
#[derive(Debug, PartialEq)]
pub enum UpdateTemplateError {
    /// <p>Indicates that the template that you specified could not be rendered. This issue may occur when a template refers to a partial that does not exist.</p>
    InvalidTemplate(String),
    /// <p>Indicates that the Template object you specified does not exist in your Amazon SES account.</p>
    TemplateDoesNotExist(String),
}

impl UpdateTemplateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateTemplateError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidTemplate" => {
                        return RusotoError::Service(UpdateTemplateError::InvalidTemplate(
                            parsed_error.message,
                        ))
                    }
                    "TemplateDoesNotExist" => {
                        return RusotoError::Service(UpdateTemplateError::TemplateDoesNotExist(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for UpdateTemplateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateTemplateError::InvalidTemplate(ref cause) => write!(f, "{}", cause),
            UpdateTemplateError::TemplateDoesNotExist(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateTemplateError {}
/// Errors returned by VerifyDomainDkim
#[derive(Debug, PartialEq)]
pub enum VerifyDomainDkimError {}

impl VerifyDomainDkimError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<VerifyDomainDkimError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for VerifyDomainDkimError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for VerifyDomainDkimError {}
/// Errors returned by VerifyDomainIdentity
#[derive(Debug, PartialEq)]
pub enum VerifyDomainIdentityError {}

impl VerifyDomainIdentityError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<VerifyDomainIdentityError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for VerifyDomainIdentityError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for VerifyDomainIdentityError {}
/// Errors returned by VerifyEmailAddress
#[derive(Debug, PartialEq)]
pub enum VerifyEmailAddressError {}

impl VerifyEmailAddressError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<VerifyEmailAddressError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for VerifyEmailAddressError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for VerifyEmailAddressError {}
/// Errors returned by VerifyEmailIdentity
#[derive(Debug, PartialEq)]
pub enum VerifyEmailIdentityError {}

impl VerifyEmailIdentityError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<VerifyEmailIdentityError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for VerifyEmailIdentityError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for VerifyEmailIdentityError {}
/// Trait representing the capabilities of the Amazon SES API. Amazon SES clients implement this trait.
#[async_trait]
pub trait Ses {
    /// <p>Creates a receipt rule set by cloning an existing one. All receipt rules and configurations are copied to the new receipt rule set and are completely independent of the source rule set.</p> <p>For information about setting up rule sets, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-receipt-rule-set.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    async fn clone_receipt_rule_set(
        &self,
        input: CloneReceiptRuleSetRequest,
    ) -> Result<CloneReceiptRuleSetResponse, RusotoError<CloneReceiptRuleSetError>>;

    /// <p>Creates a configuration set.</p> <p>Configuration sets enable you to publish email sending events. For information about using configuration sets, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/monitor-sending-activity.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    async fn create_configuration_set(
        &self,
        input: CreateConfigurationSetRequest,
    ) -> Result<CreateConfigurationSetResponse, RusotoError<CreateConfigurationSetError>>;

    /// <p>Creates a configuration set event destination.</p> <note> <p>When you create or update an event destination, you must provide one, and only one, destination. The destination can be CloudWatch, Amazon Kinesis Firehose, or Amazon Simple Notification Service (Amazon SNS).</p> </note> <p>An event destination is the AWS service to which Amazon SES publishes the email sending events associated with a configuration set. For information about using configuration sets, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/monitor-sending-activity.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    async fn create_configuration_set_event_destination(
        &self,
        input: CreateConfigurationSetEventDestinationRequest,
    ) -> Result<
        CreateConfigurationSetEventDestinationResponse,
        RusotoError<CreateConfigurationSetEventDestinationError>,
    >;

    /// <p>Creates an association between a configuration set and a custom domain for open and click event tracking. </p> <p>By default, images and links used for tracking open and click events are hosted on domains operated by Amazon SES. You can configure a subdomain of your own to handle these events. For information about using custom domains, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/configure-custom-open-click-domains.html">Amazon SES Developer Guide</a>.</p>
    async fn create_configuration_set_tracking_options(
        &self,
        input: CreateConfigurationSetTrackingOptionsRequest,
    ) -> Result<
        CreateConfigurationSetTrackingOptionsResponse,
        RusotoError<CreateConfigurationSetTrackingOptionsError>,
    >;

    /// <p>Creates a new custom verification email template.</p> <p>For more information about custom verification email templates, see <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/custom-verification-emails.html">Using Custom Verification Email Templates</a> in the <i>Amazon SES Developer Guide</i>.</p> <p>You can execute this operation no more than once per second.</p>
    async fn create_custom_verification_email_template(
        &self,
        input: CreateCustomVerificationEmailTemplateRequest,
    ) -> Result<(), RusotoError<CreateCustomVerificationEmailTemplateError>>;

    /// <p>Creates a new IP address filter.</p> <p>For information about setting up IP address filters, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-ip-filters.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    async fn create_receipt_filter(
        &self,
        input: CreateReceiptFilterRequest,
    ) -> Result<CreateReceiptFilterResponse, RusotoError<CreateReceiptFilterError>>;

    /// <p>Creates a receipt rule.</p> <p>For information about setting up receipt rules, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-receipt-rules.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    async fn create_receipt_rule(
        &self,
        input: CreateReceiptRuleRequest,
    ) -> Result<CreateReceiptRuleResponse, RusotoError<CreateReceiptRuleError>>;

    /// <p>Creates an empty receipt rule set.</p> <p>For information about setting up receipt rule sets, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-receipt-rule-set.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    async fn create_receipt_rule_set(
        &self,
        input: CreateReceiptRuleSetRequest,
    ) -> Result<CreateReceiptRuleSetResponse, RusotoError<CreateReceiptRuleSetError>>;

    /// <p>Creates an email template. Email templates enable you to send personalized email to one or more destinations in a single API operation. For more information, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/send-personalized-email-api.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    async fn create_template(
        &self,
        input: CreateTemplateRequest,
    ) -> Result<CreateTemplateResponse, RusotoError<CreateTemplateError>>;

    /// <p>Deletes a configuration set. Configuration sets enable you to publish email sending events. For information about using configuration sets, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/monitor-sending-activity.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    async fn delete_configuration_set(
        &self,
        input: DeleteConfigurationSetRequest,
    ) -> Result<DeleteConfigurationSetResponse, RusotoError<DeleteConfigurationSetError>>;

    /// <p>Deletes a configuration set event destination. Configuration set event destinations are associated with configuration sets, which enable you to publish email sending events. For information about using configuration sets, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/monitor-sending-activity.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    async fn delete_configuration_set_event_destination(
        &self,
        input: DeleteConfigurationSetEventDestinationRequest,
    ) -> Result<
        DeleteConfigurationSetEventDestinationResponse,
        RusotoError<DeleteConfigurationSetEventDestinationError>,
    >;

    /// <p><p>Deletes an association between a configuration set and a custom domain for open and click event tracking.</p> <p>By default, images and links used for tracking open and click events are hosted on domains operated by Amazon SES. You can configure a subdomain of your own to handle these events. For information about using custom domains, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/configure-custom-open-click-domains.html">Amazon SES Developer Guide</a>.</p> <note> <p>Deleting this kind of association will result in emails sent using the specified configuration set to capture open and click events using the standard, Amazon SES-operated domains.</p> </note></p>
    async fn delete_configuration_set_tracking_options(
        &self,
        input: DeleteConfigurationSetTrackingOptionsRequest,
    ) -> Result<
        DeleteConfigurationSetTrackingOptionsResponse,
        RusotoError<DeleteConfigurationSetTrackingOptionsError>,
    >;

    /// <p>Deletes an existing custom verification email template. </p> <p>For more information about custom verification email templates, see <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/custom-verification-emails.html">Using Custom Verification Email Templates</a> in the <i>Amazon SES Developer Guide</i>.</p> <p>You can execute this operation no more than once per second.</p>
    async fn delete_custom_verification_email_template(
        &self,
        input: DeleteCustomVerificationEmailTemplateRequest,
    ) -> Result<(), RusotoError<DeleteCustomVerificationEmailTemplateError>>;

    /// <p>Deletes the specified identity (an email address or a domain) from the list of verified identities.</p> <p>You can execute this operation no more than once per second.</p>
    async fn delete_identity(
        &self,
        input: DeleteIdentityRequest,
    ) -> Result<DeleteIdentityResponse, RusotoError<DeleteIdentityError>>;

    /// <p>Deletes the specified sending authorization policy for the given identity (an email address or a domain). This API returns successfully even if a policy with the specified name does not exist.</p> <note> <p>This API is for the identity owner only. If you have not verified the identity, this API will return an error.</p> </note> <p>Sending authorization is a feature that enables an identity owner to authorize other senders to use its identities. For information about using sending authorization, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/sending-authorization.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    async fn delete_identity_policy(
        &self,
        input: DeleteIdentityPolicyRequest,
    ) -> Result<DeleteIdentityPolicyResponse, RusotoError<DeleteIdentityPolicyError>>;

    /// <p>Deletes the specified IP address filter.</p> <p>For information about managing IP address filters, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-managing-ip-filters.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    async fn delete_receipt_filter(
        &self,
        input: DeleteReceiptFilterRequest,
    ) -> Result<DeleteReceiptFilterResponse, RusotoError<DeleteReceiptFilterError>>;

    /// <p>Deletes the specified receipt rule.</p> <p>For information about managing receipt rules, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-managing-receipt-rules.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    async fn delete_receipt_rule(
        &self,
        input: DeleteReceiptRuleRequest,
    ) -> Result<DeleteReceiptRuleResponse, RusotoError<DeleteReceiptRuleError>>;

    /// <p>Deletes the specified receipt rule set and all of the receipt rules it contains.</p> <note> <p>The currently active rule set cannot be deleted.</p> </note> <p>For information about managing receipt rule sets, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-managing-receipt-rule-sets.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    async fn delete_receipt_rule_set(
        &self,
        input: DeleteReceiptRuleSetRequest,
    ) -> Result<DeleteReceiptRuleSetResponse, RusotoError<DeleteReceiptRuleSetError>>;

    /// <p>Deletes an email template.</p> <p>You can execute this operation no more than once per second.</p>
    async fn delete_template(
        &self,
        input: DeleteTemplateRequest,
    ) -> Result<DeleteTemplateResponse, RusotoError<DeleteTemplateError>>;

    /// <p>Deprecated. Use the <code>DeleteIdentity</code> operation to delete email addresses and domains.</p>
    async fn delete_verified_email_address(
        &self,
        input: DeleteVerifiedEmailAddressRequest,
    ) -> Result<(), RusotoError<DeleteVerifiedEmailAddressError>>;

    /// <p>Returns the metadata and receipt rules for the receipt rule set that is currently active.</p> <p>For information about setting up receipt rule sets, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-receipt-rule-set.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    async fn describe_active_receipt_rule_set(
        &self,
        input: DescribeActiveReceiptRuleSetRequest,
    ) -> Result<DescribeActiveReceiptRuleSetResponse, RusotoError<DescribeActiveReceiptRuleSetError>>;

    /// <p>Returns the details of the specified configuration set. For information about using configuration sets, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/monitor-sending-activity.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    async fn describe_configuration_set(
        &self,
        input: DescribeConfigurationSetRequest,
    ) -> Result<DescribeConfigurationSetResponse, RusotoError<DescribeConfigurationSetError>>;

    /// <p>Returns the details of the specified receipt rule.</p> <p>For information about setting up receipt rules, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-receipt-rules.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    async fn describe_receipt_rule(
        &self,
        input: DescribeReceiptRuleRequest,
    ) -> Result<DescribeReceiptRuleResponse, RusotoError<DescribeReceiptRuleError>>;

    /// <p>Returns the details of the specified receipt rule set.</p> <p>For information about managing receipt rule sets, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-managing-receipt-rule-sets.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    async fn describe_receipt_rule_set(
        &self,
        input: DescribeReceiptRuleSetRequest,
    ) -> Result<DescribeReceiptRuleSetResponse, RusotoError<DescribeReceiptRuleSetError>>;

    /// <p>Returns the email sending status of the Amazon SES account for the current region.</p> <p>You can execute this operation no more than once per second.</p>
    async fn get_account_sending_enabled(
        &self,
    ) -> Result<GetAccountSendingEnabledResponse, RusotoError<GetAccountSendingEnabledError>>;

    /// <p>Returns the custom email verification template for the template name you specify.</p> <p>For more information about custom verification email templates, see <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/custom-verification-emails.html">Using Custom Verification Email Templates</a> in the <i>Amazon SES Developer Guide</i>.</p> <p>You can execute this operation no more than once per second.</p>
    async fn get_custom_verification_email_template(
        &self,
        input: GetCustomVerificationEmailTemplateRequest,
    ) -> Result<
        GetCustomVerificationEmailTemplateResponse,
        RusotoError<GetCustomVerificationEmailTemplateError>,
    >;

    /// <p>Returns the current status of Easy DKIM signing for an entity. For domain name identities, this operation also returns the DKIM tokens that are required for Easy DKIM signing, and whether Amazon SES has successfully verified that these tokens have been published.</p> <p>This operation takes a list of identities as input and returns the following information for each:</p> <ul> <li> <p>Whether Easy DKIM signing is enabled or disabled.</p> </li> <li> <p>A set of DKIM tokens that represent the identity. If the identity is an email address, the tokens represent the domain of that address.</p> </li> <li> <p>Whether Amazon SES has successfully verified the DKIM tokens published in the domain's DNS. This information is only returned for domain name identities, not for email addresses.</p> </li> </ul> <p>This operation is throttled at one request per second and can only get DKIM attributes for up to 100 identities at a time.</p> <p>For more information about creating DNS records using DKIM tokens, go to the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/easy-dkim-dns-records.html">Amazon SES Developer Guide</a>.</p>
    async fn get_identity_dkim_attributes(
        &self,
        input: GetIdentityDkimAttributesRequest,
    ) -> Result<GetIdentityDkimAttributesResponse, RusotoError<GetIdentityDkimAttributesError>>;

    /// <p>Returns the custom MAIL FROM attributes for a list of identities (email addresses : domains).</p> <p>This operation is throttled at one request per second and can only get custom MAIL FROM attributes for up to 100 identities at a time.</p>
    async fn get_identity_mail_from_domain_attributes(
        &self,
        input: GetIdentityMailFromDomainAttributesRequest,
    ) -> Result<
        GetIdentityMailFromDomainAttributesResponse,
        RusotoError<GetIdentityMailFromDomainAttributesError>,
    >;

    /// <p>Given a list of verified identities (email addresses and/or domains), returns a structure describing identity notification attributes.</p> <p>This operation is throttled at one request per second and can only get notification attributes for up to 100 identities at a time.</p> <p>For more information about using notifications with Amazon SES, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/notifications.html">Amazon SES Developer Guide</a>.</p>
    async fn get_identity_notification_attributes(
        &self,
        input: GetIdentityNotificationAttributesRequest,
    ) -> Result<
        GetIdentityNotificationAttributesResponse,
        RusotoError<GetIdentityNotificationAttributesError>,
    >;

    /// <p>Returns the requested sending authorization policies for the given identity (an email address or a domain). The policies are returned as a map of policy names to policy contents. You can retrieve a maximum of 20 policies at a time.</p> <note> <p>This API is for the identity owner only. If you have not verified the identity, this API will return an error.</p> </note> <p>Sending authorization is a feature that enables an identity owner to authorize other senders to use its identities. For information about using sending authorization, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/sending-authorization.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    async fn get_identity_policies(
        &self,
        input: GetIdentityPoliciesRequest,
    ) -> Result<GetIdentityPoliciesResponse, RusotoError<GetIdentityPoliciesError>>;

    /// <p>Given a list of identities (email addresses and/or domains), returns the verification status and (for domain identities) the verification token for each identity.</p> <p>The verification status of an email address is "Pending" until the email address owner clicks the link within the verification email that Amazon SES sent to that address. If the email address owner clicks the link within 24 hours, the verification status of the email address changes to "Success". If the link is not clicked within 24 hours, the verification status changes to "Failed." In that case, if you still want to verify the email address, you must restart the verification process from the beginning.</p> <p>For domain identities, the domain's verification status is "Pending" as Amazon SES searches for the required TXT record in the DNS settings of the domain. When Amazon SES detects the record, the domain's verification status changes to "Success". If Amazon SES is unable to detect the record within 72 hours, the domain's verification status changes to "Failed." In that case, if you still want to verify the domain, you must restart the verification process from the beginning.</p> <p>This operation is throttled at one request per second and can only get verification attributes for up to 100 identities at a time.</p>
    async fn get_identity_verification_attributes(
        &self,
        input: GetIdentityVerificationAttributesRequest,
    ) -> Result<
        GetIdentityVerificationAttributesResponse,
        RusotoError<GetIdentityVerificationAttributesError>,
    >;

    /// <p>Provides the sending limits for the Amazon SES account. </p> <p>You can execute this operation no more than once per second.</p>
    async fn get_send_quota(&self) -> Result<GetSendQuotaResponse, RusotoError<GetSendQuotaError>>;

    /// <p>Provides sending statistics for the current AWS Region. The result is a list of data points, representing the last two weeks of sending activity. Each data point in the list contains statistics for a 15-minute period of time.</p> <p>You can execute this operation no more than once per second.</p>
    async fn get_send_statistics(
        &self,
    ) -> Result<GetSendStatisticsResponse, RusotoError<GetSendStatisticsError>>;

    /// <p>Displays the template object (which includes the Subject line, HTML part and text part) for the template you specify.</p> <p>You can execute this operation no more than once per second.</p>
    async fn get_template(
        &self,
        input: GetTemplateRequest,
    ) -> Result<GetTemplateResponse, RusotoError<GetTemplateError>>;

    /// <p>Provides a list of the configuration sets associated with your Amazon SES account in the current AWS Region. For information about using configuration sets, see <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/monitor-sending-activity.html">Monitoring Your Amazon SES Sending Activity</a> in the <i>Amazon SES Developer Guide.</i> </p> <p>You can execute this operation no more than once per second. This operation will return up to 1,000 configuration sets each time it is run. If your Amazon SES account has more than 1,000 configuration sets, this operation will also return a NextToken element. You can then execute the <code>ListConfigurationSets</code> operation again, passing the <code>NextToken</code> parameter and the value of the NextToken element to retrieve additional results.</p>
    async fn list_configuration_sets(
        &self,
        input: ListConfigurationSetsRequest,
    ) -> Result<ListConfigurationSetsResponse, RusotoError<ListConfigurationSetsError>>;

    /// <p>Lists the existing custom verification email templates for your account in the current AWS Region.</p> <p>For more information about custom verification email templates, see <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/custom-verification-emails.html">Using Custom Verification Email Templates</a> in the <i>Amazon SES Developer Guide</i>.</p> <p>You can execute this operation no more than once per second.</p>
    async fn list_custom_verification_email_templates(
        &self,
        input: ListCustomVerificationEmailTemplatesRequest,
    ) -> Result<
        ListCustomVerificationEmailTemplatesResponse,
        RusotoError<ListCustomVerificationEmailTemplatesError>,
    >;

    /// <p>Returns a list containing all of the identities (email addresses and domains) for your AWS account in the current AWS Region, regardless of verification status.</p> <p>You can execute this operation no more than once per second.</p>
    async fn list_identities(
        &self,
        input: ListIdentitiesRequest,
    ) -> Result<ListIdentitiesResponse, RusotoError<ListIdentitiesError>>;

    /// <p>Returns a list of sending authorization policies that are attached to the given identity (an email address or a domain). This API returns only a list. If you want the actual policy content, you can use <code>GetIdentityPolicies</code>.</p> <note> <p>This API is for the identity owner only. If you have not verified the identity, this API will return an error.</p> </note> <p>Sending authorization is a feature that enables an identity owner to authorize other senders to use its identities. For information about using sending authorization, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/sending-authorization.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    async fn list_identity_policies(
        &self,
        input: ListIdentityPoliciesRequest,
    ) -> Result<ListIdentityPoliciesResponse, RusotoError<ListIdentityPoliciesError>>;

    /// <p>Lists the IP address filters associated with your AWS account in the current AWS Region.</p> <p>For information about managing IP address filters, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-managing-ip-filters.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    async fn list_receipt_filters(
        &self,
        input: ListReceiptFiltersRequest,
    ) -> Result<ListReceiptFiltersResponse, RusotoError<ListReceiptFiltersError>>;

    /// <p>Lists the receipt rule sets that exist under your AWS account in the current AWS Region. If there are additional receipt rule sets to be retrieved, you will receive a <code>NextToken</code> that you can provide to the next call to <code>ListReceiptRuleSets</code> to retrieve the additional entries.</p> <p>For information about managing receipt rule sets, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-managing-receipt-rule-sets.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    async fn list_receipt_rule_sets(
        &self,
        input: ListReceiptRuleSetsRequest,
    ) -> Result<ListReceiptRuleSetsResponse, RusotoError<ListReceiptRuleSetsError>>;

    /// <p>Lists the email templates present in your Amazon SES account in the current AWS Region.</p> <p>You can execute this operation no more than once per second.</p>
    async fn list_templates(
        &self,
        input: ListTemplatesRequest,
    ) -> Result<ListTemplatesResponse, RusotoError<ListTemplatesError>>;

    /// <p>Deprecated. Use the <code>ListIdentities</code> operation to list the email addresses and domains associated with your account.</p>
    async fn list_verified_email_addresses(
        &self,
    ) -> Result<ListVerifiedEmailAddressesResponse, RusotoError<ListVerifiedEmailAddressesError>>;

    /// <p>Adds or updates the delivery options for a configuration set.</p>
    async fn put_configuration_set_delivery_options(
        &self,
        input: PutConfigurationSetDeliveryOptionsRequest,
    ) -> Result<
        PutConfigurationSetDeliveryOptionsResponse,
        RusotoError<PutConfigurationSetDeliveryOptionsError>,
    >;

    /// <p>Adds or updates a sending authorization policy for the specified identity (an email address or a domain).</p> <note> <p>This API is for the identity owner only. If you have not verified the identity, this API will return an error.</p> </note> <p>Sending authorization is a feature that enables an identity owner to authorize other senders to use its identities. For information about using sending authorization, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/sending-authorization.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    async fn put_identity_policy(
        &self,
        input: PutIdentityPolicyRequest,
    ) -> Result<PutIdentityPolicyResponse, RusotoError<PutIdentityPolicyError>>;

    /// <p>Reorders the receipt rules within a receipt rule set.</p> <note> <p>All of the rules in the rule set must be represented in this request. That is, this API will return an error if the reorder request doesn't explicitly position all of the rules.</p> </note> <p>For information about managing receipt rule sets, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-managing-receipt-rule-sets.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    async fn reorder_receipt_rule_set(
        &self,
        input: ReorderReceiptRuleSetRequest,
    ) -> Result<ReorderReceiptRuleSetResponse, RusotoError<ReorderReceiptRuleSetError>>;

    /// <p>Generates and sends a bounce message to the sender of an email you received through Amazon SES. You can only use this API on an email up to 24 hours after you receive it.</p> <note> <p>You cannot use this API to send generic bounces for mail that was not received by Amazon SES.</p> </note> <p>For information about receiving email through Amazon SES, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    async fn send_bounce(
        &self,
        input: SendBounceRequest,
    ) -> Result<SendBounceResponse, RusotoError<SendBounceError>>;

    /// <p><p>Composes an email message to multiple destinations. The message body is created using an email template.</p> <p>In order to send email using the <code>SendBulkTemplatedEmail</code> operation, your call to the API must meet the following requirements:</p> <ul> <li> <p>The call must refer to an existing email template. You can create email templates using the <a>CreateTemplate</a> operation.</p> </li> <li> <p>The message must be sent from a verified email address or domain.</p> </li> <li> <p>If your account is still in the Amazon SES sandbox, you may only send to verified addresses or domains, or to email addresses associated with the Amazon SES Mailbox Simulator. For more information, see <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/verify-addresses-and-domains.html">Verifying Email Addresses and Domains</a> in the <i>Amazon SES Developer Guide.</i> </p> </li> <li> <p>The maximum message size is 10 MB.</p> </li> <li> <p>Each <code>Destination</code> parameter must include at least one recipient email address. The recipient address can be a To: address, a CC: address, or a BCC: address. If a recipient email address is invalid (that is, it is not in the format <i>UserName@[SubDomain.]Domain.TopLevelDomain</i>), the entire message will be rejected, even if the message contains other recipients that are valid.</p> </li> <li> <p>The message may not include more than 50 recipients, across the To:, CC: and BCC: fields. If you need to send an email message to a larger audience, you can divide your recipient list into groups of 50 or fewer, and then call the <code>SendBulkTemplatedEmail</code> operation several times to send the message to each group.</p> </li> <li> <p>The number of destinations you can contact in a single call to the API may be limited by your account&#39;s maximum sending rate.</p> </li> </ul></p>
    async fn send_bulk_templated_email(
        &self,
        input: SendBulkTemplatedEmailRequest,
    ) -> Result<SendBulkTemplatedEmailResponse, RusotoError<SendBulkTemplatedEmailError>>;

    /// <p>Adds an email address to the list of identities for your Amazon SES account in the current AWS Region and attempts to verify it. As a result of executing this operation, a customized verification email is sent to the specified address.</p> <p>To use this operation, you must first create a custom verification email template. For more information about creating and using custom verification email templates, see <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/custom-verification-emails.html">Using Custom Verification Email Templates</a> in the <i>Amazon SES Developer Guide</i>.</p> <p>You can execute this operation no more than once per second.</p>
    async fn send_custom_verification_email(
        &self,
        input: SendCustomVerificationEmailRequest,
    ) -> Result<SendCustomVerificationEmailResponse, RusotoError<SendCustomVerificationEmailError>>;

    /// <p><p>Composes an email message and immediately queues it for sending. In order to send email using the <code>SendEmail</code> operation, your message must meet the following requirements:</p> <ul> <li> <p>The message must be sent from a verified email address or domain. If you attempt to send email using a non-verified address or domain, the operation will result in an &quot;Email address not verified&quot; error. </p> </li> <li> <p>If your account is still in the Amazon SES sandbox, you may only send to verified addresses or domains, or to email addresses associated with the Amazon SES Mailbox Simulator. For more information, see <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/verify-addresses-and-domains.html">Verifying Email Addresses and Domains</a> in the <i>Amazon SES Developer Guide.</i> </p> </li> <li> <p>The maximum message size is 10 MB.</p> </li> <li> <p>The message must include at least one recipient email address. The recipient address can be a To: address, a CC: address, or a BCC: address. If a recipient email address is invalid (that is, it is not in the format <i>UserName@[SubDomain.]Domain.TopLevelDomain</i>), the entire message will be rejected, even if the message contains other recipients that are valid.</p> </li> <li> <p>The message may not include more than 50 recipients, across the To:, CC: and BCC: fields. If you need to send an email message to a larger audience, you can divide your recipient list into groups of 50 or fewer, and then call the <code>SendEmail</code> operation several times to send the message to each group.</p> </li> </ul> <important> <p>For every message that you send, the total number of recipients (including each recipient in the To:, CC: and BCC: fields) is counted against the maximum number of emails you can send in a 24-hour period (your <i>sending quota</i>). For more information about sending quotas in Amazon SES, see <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/manage-sending-limits.html">Managing Your Amazon SES Sending Limits</a> in the <i>Amazon SES Developer Guide.</i> </p> </important></p>
    async fn send_email(
        &self,
        input: SendEmailRequest,
    ) -> Result<SendEmailResponse, RusotoError<SendEmailError>>;

    /// <p><p>Composes an email message and immediately queues it for sending.</p> <p>This operation is more flexible than the <code>SendEmail</code> API operation. When you use the <code>SendRawEmail</code> operation, you can specify the headers of the message as well as its content. This flexibility is useful, for example, when you want to send a multipart MIME email (such a message that contains both a text and an HTML version). You can also use this operation to send messages that include attachments.</p> <p>The <code>SendRawEmail</code> operation has the following requirements:</p> <ul> <li> <p>You can only send email from <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/verify-addresses-and-domains.html">verified email addresses or domains</a>. If you try to send email from an address that isn&#39;t verified, the operation results in an &quot;Email address not verified&quot; error.</p> </li> <li> <p>If your account is still in the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/request-production-access.html">Amazon SES sandbox</a>, you can only send email to other verified addresses in your account, or to addresses that are associated with the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/mailbox-simulator.html">Amazon SES mailbox simulator</a>.</p> </li> <li> <p>The maximum message size, including attachments, is 10 MB.</p> </li> <li> <p>Each message has to include at least one recipient address. A recipient address includes any address on the To:, CC:, or BCC: lines.</p> </li> <li> <p>If you send a single message to more than one recipient address, and one of the recipient addresses isn&#39;t in a valid format (that is, it&#39;s not in the format <i>UserName@[SubDomain.]Domain.TopLevelDomain</i>), Amazon SES rejects the entire message, even if the other addresses are valid.</p> </li> <li> <p>Each message can include up to 50 recipient addresses across the To:, CC:, or BCC: lines. If you need to send a single message to more than 50 recipients, you have to split the list of recipient addresses into groups of less than 50 recipients, and send separate messages to each group.</p> </li> <li> <p>Amazon SES allows you to specify 8-bit Content-Transfer-Encoding for MIME message parts. However, if Amazon SES has to modify the contents of your message (for example, if you use open and click tracking), 8-bit content isn&#39;t preserved. For this reason, we highly recommend that you encode all content that isn&#39;t 7-bit ASCII. For more information, see <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/send-email-raw.html#send-email-mime-encoding">MIME Encoding</a> in the <i>Amazon SES Developer Guide</i>.</p> </li> </ul> <p>Additionally, keep the following considerations in mind when using the <code>SendRawEmail</code> operation:</p> <ul> <li> <p>Although you can customize the message headers when using the <code>SendRawEmail</code> operation, Amazon SES will automatically apply its own <code>Message-ID</code> and <code>Date</code> headers; if you passed these headers when creating the message, they will be overwritten by the values that Amazon SES provides.</p> </li> <li> <p>If you are using sending authorization to send on behalf of another user, <code>SendRawEmail</code> enables you to specify the cross-account identity for the email&#39;s Source, From, and Return-Path parameters in one of two ways: you can pass optional parameters <code>SourceArn</code>, <code>FromArn</code>, and/or <code>ReturnPathArn</code> to the API, or you can include the following X-headers in the header of your raw email:</p> <ul> <li> <p> <code>X-SES-SOURCE-ARN</code> </p> </li> <li> <p> <code>X-SES-FROM-ARN</code> </p> </li> <li> <p> <code>X-SES-RETURN-PATH-ARN</code> </p> </li> </ul> <important> <p>Don&#39;t include these X-headers in the DKIM signature. Amazon SES removes these before it sends the email.</p> </important> <p>If you only specify the <code>SourceIdentityArn</code> parameter, Amazon SES sets the From and Return-Path addresses to the same identity that you specified.</p> <p>For more information about sending authorization, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/sending-authorization.html">Using Sending Authorization with Amazon SES</a> in the <i>Amazon SES Developer Guide.</i> </p> </li> <li> <p>For every message that you send, the total number of recipients (including each recipient in the To:, CC: and BCC: fields) is counted against the maximum number of emails you can send in a 24-hour period (your <i>sending quota</i>). For more information about sending quotas in Amazon SES, see <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/manage-sending-limits.html">Managing Your Amazon SES Sending Limits</a> in the <i>Amazon SES Developer Guide.</i> </p> </li> </ul></p>
    async fn send_raw_email(
        &self,
        input: SendRawEmailRequest,
    ) -> Result<SendRawEmailResponse, RusotoError<SendRawEmailError>>;

    /// <p><p>Composes an email message using an email template and immediately queues it for sending.</p> <p>In order to send email using the <code>SendTemplatedEmail</code> operation, your call to the API must meet the following requirements:</p> <ul> <li> <p>The call must refer to an existing email template. You can create email templates using the <a>CreateTemplate</a> operation.</p> </li> <li> <p>The message must be sent from a verified email address or domain.</p> </li> <li> <p>If your account is still in the Amazon SES sandbox, you may only send to verified addresses or domains, or to email addresses associated with the Amazon SES Mailbox Simulator. For more information, see <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/verify-addresses-and-domains.html">Verifying Email Addresses and Domains</a> in the <i>Amazon SES Developer Guide.</i> </p> </li> <li> <p>The maximum message size is 10 MB.</p> </li> <li> <p>Calls to the <code>SendTemplatedEmail</code> operation may only include one <code>Destination</code> parameter. A destination is a set of recipients who will receive the same version of the email. The <code>Destination</code> parameter can include up to 50 recipients, across the To:, CC: and BCC: fields.</p> </li> <li> <p>The <code>Destination</code> parameter must include at least one recipient email address. The recipient address can be a To: address, a CC: address, or a BCC: address. If a recipient email address is invalid (that is, it is not in the format <i>UserName@[SubDomain.]Domain.TopLevelDomain</i>), the entire message will be rejected, even if the message contains other recipients that are valid.</p> </li> </ul> <important> <p>If your call to the <code>SendTemplatedEmail</code> operation includes all of the required parameters, Amazon SES accepts it and returns a Message ID. However, if Amazon SES can&#39;t render the email because the template contains errors, it doesn&#39;t send the email. Additionally, because it already accepted the message, Amazon SES doesn&#39;t return a message stating that it was unable to send the email.</p> <p>For these reasons, we highly recommend that you set up Amazon SES to send you notifications when Rendering Failure events occur. For more information, see <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/send-personalized-email-api.html">Sending Personalized Email Using the Amazon SES API</a> in the <i>Amazon Simple Email Service Developer Guide</i>.</p> </important></p>
    async fn send_templated_email(
        &self,
        input: SendTemplatedEmailRequest,
    ) -> Result<SendTemplatedEmailResponse, RusotoError<SendTemplatedEmailError>>;

    /// <p>Sets the specified receipt rule set as the active receipt rule set.</p> <note> <p>To disable your email-receiving through Amazon SES completely, you can call this API with RuleSetName set to null.</p> </note> <p>For information about managing receipt rule sets, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-managing-receipt-rule-sets.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    async fn set_active_receipt_rule_set(
        &self,
        input: SetActiveReceiptRuleSetRequest,
    ) -> Result<SetActiveReceiptRuleSetResponse, RusotoError<SetActiveReceiptRuleSetError>>;

    /// <p>Enables or disables Easy DKIM signing of email sent from an identity. If Easy DKIM signing is enabled for a domain, then Amazon SES uses DKIM to sign all email that it sends from addresses on that domain. If Easy DKIM signing is enabled for an email address, then Amazon SES uses DKIM to sign all email it sends from that address.</p> <note> <p>For email addresses (for example, <code>user@example.com</code>), you can only enable DKIM signing if the corresponding domain (in this case, <code>example.com</code>) has been set up to use Easy DKIM.</p> </note> <p>You can enable DKIM signing for an identity at any time after you start the verification process for the identity, even if the verification process isn't complete. </p> <p>You can execute this operation no more than once per second.</p> <p>For more information about Easy DKIM signing, go to the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/easy-dkim.html">Amazon SES Developer Guide</a>.</p>
    async fn set_identity_dkim_enabled(
        &self,
        input: SetIdentityDkimEnabledRequest,
    ) -> Result<SetIdentityDkimEnabledResponse, RusotoError<SetIdentityDkimEnabledError>>;

    /// <p>Given an identity (an email address or a domain), enables or disables whether Amazon SES forwards bounce and complaint notifications as email. Feedback forwarding can only be disabled when Amazon Simple Notification Service (Amazon SNS) topics are specified for both bounces and complaints.</p> <note> <p>Feedback forwarding does not apply to delivery notifications. Delivery notifications are only available through Amazon SNS.</p> </note> <p>You can execute this operation no more than once per second.</p> <p>For more information about using notifications with Amazon SES, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/notifications.html">Amazon SES Developer Guide</a>.</p>
    async fn set_identity_feedback_forwarding_enabled(
        &self,
        input: SetIdentityFeedbackForwardingEnabledRequest,
    ) -> Result<
        SetIdentityFeedbackForwardingEnabledResponse,
        RusotoError<SetIdentityFeedbackForwardingEnabledError>,
    >;

    /// <p>Given an identity (an email address or a domain), sets whether Amazon SES includes the original email headers in the Amazon Simple Notification Service (Amazon SNS) notifications of a specified type.</p> <p>You can execute this operation no more than once per second.</p> <p>For more information about using notifications with Amazon SES, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/notifications.html">Amazon SES Developer Guide</a>.</p>
    async fn set_identity_headers_in_notifications_enabled(
        &self,
        input: SetIdentityHeadersInNotificationsEnabledRequest,
    ) -> Result<
        SetIdentityHeadersInNotificationsEnabledResponse,
        RusotoError<SetIdentityHeadersInNotificationsEnabledError>,
    >;

    /// <p>Enables or disables the custom MAIL FROM domain setup for a verified identity (an email address or a domain).</p> <important> <p>To send emails using the specified MAIL FROM domain, you must add an MX record to your MAIL FROM domain's DNS settings. If you want your emails to pass Sender Policy Framework (SPF) checks, you must also add or update an SPF record. For more information, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/mail-from-set.html">Amazon SES Developer Guide</a>.</p> </important> <p>You can execute this operation no more than once per second.</p>
    async fn set_identity_mail_from_domain(
        &self,
        input: SetIdentityMailFromDomainRequest,
    ) -> Result<SetIdentityMailFromDomainResponse, RusotoError<SetIdentityMailFromDomainError>>;

    /// <p>Sets an Amazon Simple Notification Service (Amazon SNS) topic to use when delivering notifications. When you use this operation, you specify a verified identity, such as an email address or domain. When you send an email that uses the chosen identity in the Source field, Amazon SES sends notifications to the topic you specified. You can send bounce, complaint, or delivery notifications (or any combination of the three) to the Amazon SNS topic that you specify.</p> <p>You can execute this operation no more than once per second.</p> <p>For more information about feedback notification, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/notifications.html">Amazon SES Developer Guide</a>.</p>
    async fn set_identity_notification_topic(
        &self,
        input: SetIdentityNotificationTopicRequest,
    ) -> Result<SetIdentityNotificationTopicResponse, RusotoError<SetIdentityNotificationTopicError>>;

    /// <p>Sets the position of the specified receipt rule in the receipt rule set.</p> <p>For information about managing receipt rules, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-managing-receipt-rules.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    async fn set_receipt_rule_position(
        &self,
        input: SetReceiptRulePositionRequest,
    ) -> Result<SetReceiptRulePositionResponse, RusotoError<SetReceiptRulePositionError>>;

    /// <p>Creates a preview of the MIME content of an email when provided with a template and a set of replacement data.</p> <p>You can execute this operation no more than once per second.</p>
    async fn test_render_template(
        &self,
        input: TestRenderTemplateRequest,
    ) -> Result<TestRenderTemplateResponse, RusotoError<TestRenderTemplateError>>;

    /// <p>Enables or disables email sending across your entire Amazon SES account in the current AWS Region. You can use this operation in conjunction with Amazon CloudWatch alarms to temporarily pause email sending across your Amazon SES account in a given AWS Region when reputation metrics (such as your bounce or complaint rates) reach certain thresholds.</p> <p>You can execute this operation no more than once per second.</p>
    async fn update_account_sending_enabled(
        &self,
        input: UpdateAccountSendingEnabledRequest,
    ) -> Result<(), RusotoError<UpdateAccountSendingEnabledError>>;

    /// <p>Updates the event destination of a configuration set. Event destinations are associated with configuration sets, which enable you to publish email sending events to Amazon CloudWatch, Amazon Kinesis Firehose, or Amazon Simple Notification Service (Amazon SNS). For information about using configuration sets, see <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/monitor-sending-activity.html">Monitoring Your Amazon SES Sending Activity</a> in the <i>Amazon SES Developer Guide.</i> </p> <note> <p>When you create or update an event destination, you must provide one, and only one, destination. The destination can be Amazon CloudWatch, Amazon Kinesis Firehose, or Amazon Simple Notification Service (Amazon SNS).</p> </note> <p>You can execute this operation no more than once per second.</p>
    async fn update_configuration_set_event_destination(
        &self,
        input: UpdateConfigurationSetEventDestinationRequest,
    ) -> Result<
        UpdateConfigurationSetEventDestinationResponse,
        RusotoError<UpdateConfigurationSetEventDestinationError>,
    >;

    /// <p>Enables or disables the publishing of reputation metrics for emails sent using a specific configuration set in a given AWS Region. Reputation metrics include bounce and complaint rates. These metrics are published to Amazon CloudWatch. By using CloudWatch, you can create alarms when bounce or complaint rates exceed certain thresholds.</p> <p>You can execute this operation no more than once per second.</p>
    async fn update_configuration_set_reputation_metrics_enabled(
        &self,
        input: UpdateConfigurationSetReputationMetricsEnabledRequest,
    ) -> Result<(), RusotoError<UpdateConfigurationSetReputationMetricsEnabledError>>;

    /// <p>Enables or disables email sending for messages sent using a specific configuration set in a given AWS Region. You can use this operation in conjunction with Amazon CloudWatch alarms to temporarily pause email sending for a configuration set when the reputation metrics for that configuration set (such as your bounce on complaint rate) exceed certain thresholds.</p> <p>You can execute this operation no more than once per second.</p>
    async fn update_configuration_set_sending_enabled(
        &self,
        input: UpdateConfigurationSetSendingEnabledRequest,
    ) -> Result<(), RusotoError<UpdateConfigurationSetSendingEnabledError>>;

    /// <p>Modifies an association between a configuration set and a custom domain for open and click event tracking. </p> <p>By default, images and links used for tracking open and click events are hosted on domains operated by Amazon SES. You can configure a subdomain of your own to handle these events. For information about using custom domains, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/configure-custom-open-click-domains.html">Amazon SES Developer Guide</a>.</p>
    async fn update_configuration_set_tracking_options(
        &self,
        input: UpdateConfigurationSetTrackingOptionsRequest,
    ) -> Result<
        UpdateConfigurationSetTrackingOptionsResponse,
        RusotoError<UpdateConfigurationSetTrackingOptionsError>,
    >;

    /// <p>Updates an existing custom verification email template.</p> <p>For more information about custom verification email templates, see <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/custom-verification-emails.html">Using Custom Verification Email Templates</a> in the <i>Amazon SES Developer Guide</i>.</p> <p>You can execute this operation no more than once per second.</p>
    async fn update_custom_verification_email_template(
        &self,
        input: UpdateCustomVerificationEmailTemplateRequest,
    ) -> Result<(), RusotoError<UpdateCustomVerificationEmailTemplateError>>;

    /// <p>Updates a receipt rule.</p> <p>For information about managing receipt rules, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-managing-receipt-rules.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    async fn update_receipt_rule(
        &self,
        input: UpdateReceiptRuleRequest,
    ) -> Result<UpdateReceiptRuleResponse, RusotoError<UpdateReceiptRuleError>>;

    /// <p>Updates an email template. Email templates enable you to send personalized email to one or more destinations in a single API operation. For more information, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/send-personalized-email-api.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    async fn update_template(
        &self,
        input: UpdateTemplateRequest,
    ) -> Result<UpdateTemplateResponse, RusotoError<UpdateTemplateError>>;

    /// <p>Returns a set of DKIM tokens for a domain identity.</p> <important> <p>When you execute the <code>VerifyDomainDkim</code> operation, the domain that you specify is added to the list of identities that are associated with your account. This is true even if you haven't already associated the domain with your account by using the <code>VerifyDomainIdentity</code> operation. However, you can't send email from the domain until you either successfully <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/verify-domains.html">verify it</a> or you successfully <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/easy-dkim.html">set up DKIM for it</a>.</p> </important> <p>You use the tokens that are generated by this operation to create CNAME records. When Amazon SES detects that you've added these records to the DNS configuration for a domain, you can start sending email from that domain. You can start sending email even if you haven't added the TXT record provided by the VerifyDomainIdentity operation to the DNS configuration for your domain. All email that you send from the domain is authenticated using DKIM.</p> <p>To create the CNAME records for DKIM authentication, use the following values:</p> <ul> <li> <p> <b>Name</b>: <i>token</i>._domainkey.<i>example.com</i> </p> </li> <li> <p> <b>Type</b>: CNAME</p> </li> <li> <p> <b>Value</b>: <i>token</i>.dkim.amazonses.com</p> </li> </ul> <p>In the preceding example, replace <i>token</i> with one of the tokens that are generated when you execute this operation. Replace <i>example.com</i> with your domain. Repeat this process for each token that's generated by this operation.</p> <p>You can execute this operation no more than once per second.</p>
    async fn verify_domain_dkim(
        &self,
        input: VerifyDomainDkimRequest,
    ) -> Result<VerifyDomainDkimResponse, RusotoError<VerifyDomainDkimError>>;

    /// <p>Adds a domain to the list of identities for your Amazon SES account in the current AWS Region and attempts to verify it. For more information about verifying domains, see <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/verify-addresses-and-domains.html">Verifying Email Addresses and Domains</a> in the <i>Amazon SES Developer Guide.</i> </p> <p>You can execute this operation no more than once per second.</p>
    async fn verify_domain_identity(
        &self,
        input: VerifyDomainIdentityRequest,
    ) -> Result<VerifyDomainIdentityResponse, RusotoError<VerifyDomainIdentityError>>;

    /// <p>Deprecated. Use the <code>VerifyEmailIdentity</code> operation to verify a new email address.</p>
    async fn verify_email_address(
        &self,
        input: VerifyEmailAddressRequest,
    ) -> Result<(), RusotoError<VerifyEmailAddressError>>;

    /// <p>Adds an email address to the list of identities for your Amazon SES account in the current AWS region and attempts to verify it. As a result of executing this operation, a verification email is sent to the specified address.</p> <p>You can execute this operation no more than once per second.</p>
    async fn verify_email_identity(
        &self,
        input: VerifyEmailIdentityRequest,
    ) -> Result<VerifyEmailIdentityResponse, RusotoError<VerifyEmailIdentityError>>;
}
/// A client for the Amazon SES API.
#[derive(Clone)]
pub struct SesClient {
    client: Client,
    region: region::Region,
}

impl SesClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> SesClient {
        SesClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> SesClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        SesClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> SesClient {
        SesClient { client, region }
    }
}

#[async_trait]
impl Ses for SesClient {
    /// <p>Creates a receipt rule set by cloning an existing one. All receipt rules and configurations are copied to the new receipt rule set and are completely independent of the source rule set.</p> <p>For information about setting up rule sets, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-receipt-rule-set.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    async fn clone_receipt_rule_set(
        &self,
        input: CloneReceiptRuleSetRequest,
    ) -> Result<CloneReceiptRuleSetResponse, RusotoError<CloneReceiptRuleSetError>> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CloneReceiptRuleSet");
        params.put("Version", "2010-12-01");
        CloneReceiptRuleSetRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(CloneReceiptRuleSetError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = CloneReceiptRuleSetResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = CloneReceiptRuleSetResponseDeserializer::deserialize(
                "CloneReceiptRuleSetResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Creates a configuration set.</p> <p>Configuration sets enable you to publish email sending events. For information about using configuration sets, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/monitor-sending-activity.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    async fn create_configuration_set(
        &self,
        input: CreateConfigurationSetRequest,
    ) -> Result<CreateConfigurationSetResponse, RusotoError<CreateConfigurationSetError>> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CreateConfigurationSet");
        params.put("Version", "2010-12-01");
        CreateConfigurationSetRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(CreateConfigurationSetError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = CreateConfigurationSetResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = CreateConfigurationSetResponseDeserializer::deserialize(
                "CreateConfigurationSetResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Creates a configuration set event destination.</p> <note> <p>When you create or update an event destination, you must provide one, and only one, destination. The destination can be CloudWatch, Amazon Kinesis Firehose, or Amazon Simple Notification Service (Amazon SNS).</p> </note> <p>An event destination is the AWS service to which Amazon SES publishes the email sending events associated with a configuration set. For information about using configuration sets, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/monitor-sending-activity.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    async fn create_configuration_set_event_destination(
        &self,
        input: CreateConfigurationSetEventDestinationRequest,
    ) -> Result<
        CreateConfigurationSetEventDestinationResponse,
        RusotoError<CreateConfigurationSetEventDestinationError>,
    > {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CreateConfigurationSetEventDestination");
        params.put("Version", "2010-12-01");
        CreateConfigurationSetEventDestinationRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(CreateConfigurationSetEventDestinationError::from_response(
                response,
            ));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = CreateConfigurationSetEventDestinationResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = CreateConfigurationSetEventDestinationResponseDeserializer::deserialize(
                "CreateConfigurationSetEventDestinationResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Creates an association between a configuration set and a custom domain for open and click event tracking. </p> <p>By default, images and links used for tracking open and click events are hosted on domains operated by Amazon SES. You can configure a subdomain of your own to handle these events. For information about using custom domains, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/configure-custom-open-click-domains.html">Amazon SES Developer Guide</a>.</p>
    async fn create_configuration_set_tracking_options(
        &self,
        input: CreateConfigurationSetTrackingOptionsRequest,
    ) -> Result<
        CreateConfigurationSetTrackingOptionsResponse,
        RusotoError<CreateConfigurationSetTrackingOptionsError>,
    > {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CreateConfigurationSetTrackingOptions");
        params.put("Version", "2010-12-01");
        CreateConfigurationSetTrackingOptionsRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(CreateConfigurationSetTrackingOptionsError::from_response(
                response,
            ));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = CreateConfigurationSetTrackingOptionsResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = CreateConfigurationSetTrackingOptionsResponseDeserializer::deserialize(
                "CreateConfigurationSetTrackingOptionsResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Creates a new custom verification email template.</p> <p>For more information about custom verification email templates, see <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/custom-verification-emails.html">Using Custom Verification Email Templates</a> in the <i>Amazon SES Developer Guide</i>.</p> <p>You can execute this operation no more than once per second.</p>
    async fn create_custom_verification_email_template(
        &self,
        input: CreateCustomVerificationEmailTemplateRequest,
    ) -> Result<(), RusotoError<CreateCustomVerificationEmailTemplateError>> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CreateCustomVerificationEmailTemplate");
        params.put("Version", "2010-12-01");
        CreateCustomVerificationEmailTemplateRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(CreateCustomVerificationEmailTemplateError::from_response(
                response,
            ));
        }

        std::mem::drop(response);
        Ok(())
    }

    /// <p>Creates a new IP address filter.</p> <p>For information about setting up IP address filters, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-ip-filters.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    async fn create_receipt_filter(
        &self,
        input: CreateReceiptFilterRequest,
    ) -> Result<CreateReceiptFilterResponse, RusotoError<CreateReceiptFilterError>> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CreateReceiptFilter");
        params.put("Version", "2010-12-01");
        CreateReceiptFilterRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(CreateReceiptFilterError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = CreateReceiptFilterResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = CreateReceiptFilterResponseDeserializer::deserialize(
                "CreateReceiptFilterResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Creates a receipt rule.</p> <p>For information about setting up receipt rules, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-receipt-rules.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    async fn create_receipt_rule(
        &self,
        input: CreateReceiptRuleRequest,
    ) -> Result<CreateReceiptRuleResponse, RusotoError<CreateReceiptRuleError>> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CreateReceiptRule");
        params.put("Version", "2010-12-01");
        CreateReceiptRuleRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(CreateReceiptRuleError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = CreateReceiptRuleResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = CreateReceiptRuleResponseDeserializer::deserialize(
                "CreateReceiptRuleResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Creates an empty receipt rule set.</p> <p>For information about setting up receipt rule sets, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-receipt-rule-set.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    async fn create_receipt_rule_set(
        &self,
        input: CreateReceiptRuleSetRequest,
    ) -> Result<CreateReceiptRuleSetResponse, RusotoError<CreateReceiptRuleSetError>> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CreateReceiptRuleSet");
        params.put("Version", "2010-12-01");
        CreateReceiptRuleSetRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(CreateReceiptRuleSetError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = CreateReceiptRuleSetResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = CreateReceiptRuleSetResponseDeserializer::deserialize(
                "CreateReceiptRuleSetResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Creates an email template. Email templates enable you to send personalized email to one or more destinations in a single API operation. For more information, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/send-personalized-email-api.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    async fn create_template(
        &self,
        input: CreateTemplateRequest,
    ) -> Result<CreateTemplateResponse, RusotoError<CreateTemplateError>> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CreateTemplate");
        params.put("Version", "2010-12-01");
        CreateTemplateRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(CreateTemplateError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = CreateTemplateResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = CreateTemplateResponseDeserializer::deserialize(
                "CreateTemplateResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Deletes a configuration set. Configuration sets enable you to publish email sending events. For information about using configuration sets, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/monitor-sending-activity.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    async fn delete_configuration_set(
        &self,
        input: DeleteConfigurationSetRequest,
    ) -> Result<DeleteConfigurationSetResponse, RusotoError<DeleteConfigurationSetError>> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteConfigurationSet");
        params.put("Version", "2010-12-01");
        DeleteConfigurationSetRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DeleteConfigurationSetError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = DeleteConfigurationSetResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = DeleteConfigurationSetResponseDeserializer::deserialize(
                "DeleteConfigurationSetResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Deletes a configuration set event destination. Configuration set event destinations are associated with configuration sets, which enable you to publish email sending events. For information about using configuration sets, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/monitor-sending-activity.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    async fn delete_configuration_set_event_destination(
        &self,
        input: DeleteConfigurationSetEventDestinationRequest,
    ) -> Result<
        DeleteConfigurationSetEventDestinationResponse,
        RusotoError<DeleteConfigurationSetEventDestinationError>,
    > {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteConfigurationSetEventDestination");
        params.put("Version", "2010-12-01");
        DeleteConfigurationSetEventDestinationRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DeleteConfigurationSetEventDestinationError::from_response(
                response,
            ));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = DeleteConfigurationSetEventDestinationResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = DeleteConfigurationSetEventDestinationResponseDeserializer::deserialize(
                "DeleteConfigurationSetEventDestinationResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p><p>Deletes an association between a configuration set and a custom domain for open and click event tracking.</p> <p>By default, images and links used for tracking open and click events are hosted on domains operated by Amazon SES. You can configure a subdomain of your own to handle these events. For information about using custom domains, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/configure-custom-open-click-domains.html">Amazon SES Developer Guide</a>.</p> <note> <p>Deleting this kind of association will result in emails sent using the specified configuration set to capture open and click events using the standard, Amazon SES-operated domains.</p> </note></p>
    async fn delete_configuration_set_tracking_options(
        &self,
        input: DeleteConfigurationSetTrackingOptionsRequest,
    ) -> Result<
        DeleteConfigurationSetTrackingOptionsResponse,
        RusotoError<DeleteConfigurationSetTrackingOptionsError>,
    > {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteConfigurationSetTrackingOptions");
        params.put("Version", "2010-12-01");
        DeleteConfigurationSetTrackingOptionsRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DeleteConfigurationSetTrackingOptionsError::from_response(
                response,
            ));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = DeleteConfigurationSetTrackingOptionsResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = DeleteConfigurationSetTrackingOptionsResponseDeserializer::deserialize(
                "DeleteConfigurationSetTrackingOptionsResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Deletes an existing custom verification email template. </p> <p>For more information about custom verification email templates, see <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/custom-verification-emails.html">Using Custom Verification Email Templates</a> in the <i>Amazon SES Developer Guide</i>.</p> <p>You can execute this operation no more than once per second.</p>
    async fn delete_custom_verification_email_template(
        &self,
        input: DeleteCustomVerificationEmailTemplateRequest,
    ) -> Result<(), RusotoError<DeleteCustomVerificationEmailTemplateError>> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteCustomVerificationEmailTemplate");
        params.put("Version", "2010-12-01");
        DeleteCustomVerificationEmailTemplateRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DeleteCustomVerificationEmailTemplateError::from_response(
                response,
            ));
        }

        std::mem::drop(response);
        Ok(())
    }

    /// <p>Deletes the specified identity (an email address or a domain) from the list of verified identities.</p> <p>You can execute this operation no more than once per second.</p>
    async fn delete_identity(
        &self,
        input: DeleteIdentityRequest,
    ) -> Result<DeleteIdentityResponse, RusotoError<DeleteIdentityError>> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteIdentity");
        params.put("Version", "2010-12-01");
        DeleteIdentityRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DeleteIdentityError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = DeleteIdentityResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = DeleteIdentityResponseDeserializer::deserialize(
                "DeleteIdentityResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Deletes the specified sending authorization policy for the given identity (an email address or a domain). This API returns successfully even if a policy with the specified name does not exist.</p> <note> <p>This API is for the identity owner only. If you have not verified the identity, this API will return an error.</p> </note> <p>Sending authorization is a feature that enables an identity owner to authorize other senders to use its identities. For information about using sending authorization, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/sending-authorization.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    async fn delete_identity_policy(
        &self,
        input: DeleteIdentityPolicyRequest,
    ) -> Result<DeleteIdentityPolicyResponse, RusotoError<DeleteIdentityPolicyError>> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteIdentityPolicy");
        params.put("Version", "2010-12-01");
        DeleteIdentityPolicyRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DeleteIdentityPolicyError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = DeleteIdentityPolicyResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = DeleteIdentityPolicyResponseDeserializer::deserialize(
                "DeleteIdentityPolicyResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Deletes the specified IP address filter.</p> <p>For information about managing IP address filters, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-managing-ip-filters.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    async fn delete_receipt_filter(
        &self,
        input: DeleteReceiptFilterRequest,
    ) -> Result<DeleteReceiptFilterResponse, RusotoError<DeleteReceiptFilterError>> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteReceiptFilter");
        params.put("Version", "2010-12-01");
        DeleteReceiptFilterRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DeleteReceiptFilterError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = DeleteReceiptFilterResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = DeleteReceiptFilterResponseDeserializer::deserialize(
                "DeleteReceiptFilterResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Deletes the specified receipt rule.</p> <p>For information about managing receipt rules, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-managing-receipt-rules.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    async fn delete_receipt_rule(
        &self,
        input: DeleteReceiptRuleRequest,
    ) -> Result<DeleteReceiptRuleResponse, RusotoError<DeleteReceiptRuleError>> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteReceiptRule");
        params.put("Version", "2010-12-01");
        DeleteReceiptRuleRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DeleteReceiptRuleError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = DeleteReceiptRuleResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = DeleteReceiptRuleResponseDeserializer::deserialize(
                "DeleteReceiptRuleResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Deletes the specified receipt rule set and all of the receipt rules it contains.</p> <note> <p>The currently active rule set cannot be deleted.</p> </note> <p>For information about managing receipt rule sets, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-managing-receipt-rule-sets.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    async fn delete_receipt_rule_set(
        &self,
        input: DeleteReceiptRuleSetRequest,
    ) -> Result<DeleteReceiptRuleSetResponse, RusotoError<DeleteReceiptRuleSetError>> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteReceiptRuleSet");
        params.put("Version", "2010-12-01");
        DeleteReceiptRuleSetRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DeleteReceiptRuleSetError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = DeleteReceiptRuleSetResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = DeleteReceiptRuleSetResponseDeserializer::deserialize(
                "DeleteReceiptRuleSetResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Deletes an email template.</p> <p>You can execute this operation no more than once per second.</p>
    async fn delete_template(
        &self,
        input: DeleteTemplateRequest,
    ) -> Result<DeleteTemplateResponse, RusotoError<DeleteTemplateError>> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteTemplate");
        params.put("Version", "2010-12-01");
        DeleteTemplateRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DeleteTemplateError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = DeleteTemplateResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = DeleteTemplateResponseDeserializer::deserialize(
                "DeleteTemplateResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Deprecated. Use the <code>DeleteIdentity</code> operation to delete email addresses and domains.</p>
    async fn delete_verified_email_address(
        &self,
        input: DeleteVerifiedEmailAddressRequest,
    ) -> Result<(), RusotoError<DeleteVerifiedEmailAddressError>> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteVerifiedEmailAddress");
        params.put("Version", "2010-12-01");
        DeleteVerifiedEmailAddressRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DeleteVerifiedEmailAddressError::from_response(response));
        }

        std::mem::drop(response);
        Ok(())
    }

    /// <p>Returns the metadata and receipt rules for the receipt rule set that is currently active.</p> <p>For information about setting up receipt rule sets, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-receipt-rule-set.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    async fn describe_active_receipt_rule_set(
        &self,
        input: DescribeActiveReceiptRuleSetRequest,
    ) -> Result<DescribeActiveReceiptRuleSetResponse, RusotoError<DescribeActiveReceiptRuleSetError>>
    {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeActiveReceiptRuleSet");
        params.put("Version", "2010-12-01");
        DescribeActiveReceiptRuleSetRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DescribeActiveReceiptRuleSetError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = DescribeActiveReceiptRuleSetResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = DescribeActiveReceiptRuleSetResponseDeserializer::deserialize(
                "DescribeActiveReceiptRuleSetResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Returns the details of the specified configuration set. For information about using configuration sets, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/monitor-sending-activity.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    async fn describe_configuration_set(
        &self,
        input: DescribeConfigurationSetRequest,
    ) -> Result<DescribeConfigurationSetResponse, RusotoError<DescribeConfigurationSetError>> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeConfigurationSet");
        params.put("Version", "2010-12-01");
        DescribeConfigurationSetRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DescribeConfigurationSetError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = DescribeConfigurationSetResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = DescribeConfigurationSetResponseDeserializer::deserialize(
                "DescribeConfigurationSetResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Returns the details of the specified receipt rule.</p> <p>For information about setting up receipt rules, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-receipt-rules.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    async fn describe_receipt_rule(
        &self,
        input: DescribeReceiptRuleRequest,
    ) -> Result<DescribeReceiptRuleResponse, RusotoError<DescribeReceiptRuleError>> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeReceiptRule");
        params.put("Version", "2010-12-01");
        DescribeReceiptRuleRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DescribeReceiptRuleError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = DescribeReceiptRuleResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = DescribeReceiptRuleResponseDeserializer::deserialize(
                "DescribeReceiptRuleResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Returns the details of the specified receipt rule set.</p> <p>For information about managing receipt rule sets, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-managing-receipt-rule-sets.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    async fn describe_receipt_rule_set(
        &self,
        input: DescribeReceiptRuleSetRequest,
    ) -> Result<DescribeReceiptRuleSetResponse, RusotoError<DescribeReceiptRuleSetError>> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeReceiptRuleSet");
        params.put("Version", "2010-12-01");
        DescribeReceiptRuleSetRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DescribeReceiptRuleSetError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = DescribeReceiptRuleSetResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = DescribeReceiptRuleSetResponseDeserializer::deserialize(
                "DescribeReceiptRuleSetResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Returns the email sending status of the Amazon SES account for the current region.</p> <p>You can execute this operation no more than once per second.</p>
    async fn get_account_sending_enabled(
        &self,
    ) -> Result<GetAccountSendingEnabledResponse, RusotoError<GetAccountSendingEnabledError>> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "GetAccountSendingEnabled");
        params.put("Version", "2010-12-01");

        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(GetAccountSendingEnabledError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = GetAccountSendingEnabledResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = GetAccountSendingEnabledResponseDeserializer::deserialize(
                "GetAccountSendingEnabledResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Returns the custom email verification template for the template name you specify.</p> <p>For more information about custom verification email templates, see <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/custom-verification-emails.html">Using Custom Verification Email Templates</a> in the <i>Amazon SES Developer Guide</i>.</p> <p>You can execute this operation no more than once per second.</p>
    async fn get_custom_verification_email_template(
        &self,
        input: GetCustomVerificationEmailTemplateRequest,
    ) -> Result<
        GetCustomVerificationEmailTemplateResponse,
        RusotoError<GetCustomVerificationEmailTemplateError>,
    > {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "GetCustomVerificationEmailTemplate");
        params.put("Version", "2010-12-01");
        GetCustomVerificationEmailTemplateRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(GetCustomVerificationEmailTemplateError::from_response(
                response,
            ));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = GetCustomVerificationEmailTemplateResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = GetCustomVerificationEmailTemplateResponseDeserializer::deserialize(
                "GetCustomVerificationEmailTemplateResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Returns the current status of Easy DKIM signing for an entity. For domain name identities, this operation also returns the DKIM tokens that are required for Easy DKIM signing, and whether Amazon SES has successfully verified that these tokens have been published.</p> <p>This operation takes a list of identities as input and returns the following information for each:</p> <ul> <li> <p>Whether Easy DKIM signing is enabled or disabled.</p> </li> <li> <p>A set of DKIM tokens that represent the identity. If the identity is an email address, the tokens represent the domain of that address.</p> </li> <li> <p>Whether Amazon SES has successfully verified the DKIM tokens published in the domain's DNS. This information is only returned for domain name identities, not for email addresses.</p> </li> </ul> <p>This operation is throttled at one request per second and can only get DKIM attributes for up to 100 identities at a time.</p> <p>For more information about creating DNS records using DKIM tokens, go to the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/easy-dkim-dns-records.html">Amazon SES Developer Guide</a>.</p>
    async fn get_identity_dkim_attributes(
        &self,
        input: GetIdentityDkimAttributesRequest,
    ) -> Result<GetIdentityDkimAttributesResponse, RusotoError<GetIdentityDkimAttributesError>>
    {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "GetIdentityDkimAttributes");
        params.put("Version", "2010-12-01");
        GetIdentityDkimAttributesRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(GetIdentityDkimAttributesError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = GetIdentityDkimAttributesResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = GetIdentityDkimAttributesResponseDeserializer::deserialize(
                "GetIdentityDkimAttributesResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Returns the custom MAIL FROM attributes for a list of identities (email addresses : domains).</p> <p>This operation is throttled at one request per second and can only get custom MAIL FROM attributes for up to 100 identities at a time.</p>
    async fn get_identity_mail_from_domain_attributes(
        &self,
        input: GetIdentityMailFromDomainAttributesRequest,
    ) -> Result<
        GetIdentityMailFromDomainAttributesResponse,
        RusotoError<GetIdentityMailFromDomainAttributesError>,
    > {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "GetIdentityMailFromDomainAttributes");
        params.put("Version", "2010-12-01");
        GetIdentityMailFromDomainAttributesRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(GetIdentityMailFromDomainAttributesError::from_response(
                response,
            ));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = GetIdentityMailFromDomainAttributesResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = GetIdentityMailFromDomainAttributesResponseDeserializer::deserialize(
                "GetIdentityMailFromDomainAttributesResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Given a list of verified identities (email addresses and/or domains), returns a structure describing identity notification attributes.</p> <p>This operation is throttled at one request per second and can only get notification attributes for up to 100 identities at a time.</p> <p>For more information about using notifications with Amazon SES, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/notifications.html">Amazon SES Developer Guide</a>.</p>
    async fn get_identity_notification_attributes(
        &self,
        input: GetIdentityNotificationAttributesRequest,
    ) -> Result<
        GetIdentityNotificationAttributesResponse,
        RusotoError<GetIdentityNotificationAttributesError>,
    > {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "GetIdentityNotificationAttributes");
        params.put("Version", "2010-12-01");
        GetIdentityNotificationAttributesRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(GetIdentityNotificationAttributesError::from_response(
                response,
            ));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = GetIdentityNotificationAttributesResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = GetIdentityNotificationAttributesResponseDeserializer::deserialize(
                "GetIdentityNotificationAttributesResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Returns the requested sending authorization policies for the given identity (an email address or a domain). The policies are returned as a map of policy names to policy contents. You can retrieve a maximum of 20 policies at a time.</p> <note> <p>This API is for the identity owner only. If you have not verified the identity, this API will return an error.</p> </note> <p>Sending authorization is a feature that enables an identity owner to authorize other senders to use its identities. For information about using sending authorization, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/sending-authorization.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    async fn get_identity_policies(
        &self,
        input: GetIdentityPoliciesRequest,
    ) -> Result<GetIdentityPoliciesResponse, RusotoError<GetIdentityPoliciesError>> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "GetIdentityPolicies");
        params.put("Version", "2010-12-01");
        GetIdentityPoliciesRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(GetIdentityPoliciesError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = GetIdentityPoliciesResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = GetIdentityPoliciesResponseDeserializer::deserialize(
                "GetIdentityPoliciesResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Given a list of identities (email addresses and/or domains), returns the verification status and (for domain identities) the verification token for each identity.</p> <p>The verification status of an email address is "Pending" until the email address owner clicks the link within the verification email that Amazon SES sent to that address. If the email address owner clicks the link within 24 hours, the verification status of the email address changes to "Success". If the link is not clicked within 24 hours, the verification status changes to "Failed." In that case, if you still want to verify the email address, you must restart the verification process from the beginning.</p> <p>For domain identities, the domain's verification status is "Pending" as Amazon SES searches for the required TXT record in the DNS settings of the domain. When Amazon SES detects the record, the domain's verification status changes to "Success". If Amazon SES is unable to detect the record within 72 hours, the domain's verification status changes to "Failed." In that case, if you still want to verify the domain, you must restart the verification process from the beginning.</p> <p>This operation is throttled at one request per second and can only get verification attributes for up to 100 identities at a time.</p>
    async fn get_identity_verification_attributes(
        &self,
        input: GetIdentityVerificationAttributesRequest,
    ) -> Result<
        GetIdentityVerificationAttributesResponse,
        RusotoError<GetIdentityVerificationAttributesError>,
    > {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "GetIdentityVerificationAttributes");
        params.put("Version", "2010-12-01");
        GetIdentityVerificationAttributesRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(GetIdentityVerificationAttributesError::from_response(
                response,
            ));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = GetIdentityVerificationAttributesResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = GetIdentityVerificationAttributesResponseDeserializer::deserialize(
                "GetIdentityVerificationAttributesResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Provides the sending limits for the Amazon SES account. </p> <p>You can execute this operation no more than once per second.</p>
    async fn get_send_quota(&self) -> Result<GetSendQuotaResponse, RusotoError<GetSendQuotaError>> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "GetSendQuota");
        params.put("Version", "2010-12-01");

        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(GetSendQuotaError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = GetSendQuotaResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result =
                GetSendQuotaResponseDeserializer::deserialize("GetSendQuotaResult", &mut stack)?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Provides sending statistics for the current AWS Region. The result is a list of data points, representing the last two weeks of sending activity. Each data point in the list contains statistics for a 15-minute period of time.</p> <p>You can execute this operation no more than once per second.</p>
    async fn get_send_statistics(
        &self,
    ) -> Result<GetSendStatisticsResponse, RusotoError<GetSendStatisticsError>> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "GetSendStatistics");
        params.put("Version", "2010-12-01");

        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(GetSendStatisticsError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = GetSendStatisticsResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = GetSendStatisticsResponseDeserializer::deserialize(
                "GetSendStatisticsResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Displays the template object (which includes the Subject line, HTML part and text part) for the template you specify.</p> <p>You can execute this operation no more than once per second.</p>
    async fn get_template(
        &self,
        input: GetTemplateRequest,
    ) -> Result<GetTemplateResponse, RusotoError<GetTemplateError>> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "GetTemplate");
        params.put("Version", "2010-12-01");
        GetTemplateRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(GetTemplateError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = GetTemplateResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = GetTemplateResponseDeserializer::deserialize("GetTemplateResult", &mut stack)?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Provides a list of the configuration sets associated with your Amazon SES account in the current AWS Region. For information about using configuration sets, see <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/monitor-sending-activity.html">Monitoring Your Amazon SES Sending Activity</a> in the <i>Amazon SES Developer Guide.</i> </p> <p>You can execute this operation no more than once per second. This operation will return up to 1,000 configuration sets each time it is run. If your Amazon SES account has more than 1,000 configuration sets, this operation will also return a NextToken element. You can then execute the <code>ListConfigurationSets</code> operation again, passing the <code>NextToken</code> parameter and the value of the NextToken element to retrieve additional results.</p>
    async fn list_configuration_sets(
        &self,
        input: ListConfigurationSetsRequest,
    ) -> Result<ListConfigurationSetsResponse, RusotoError<ListConfigurationSetsError>> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ListConfigurationSets");
        params.put("Version", "2010-12-01");
        ListConfigurationSetsRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(ListConfigurationSetsError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = ListConfigurationSetsResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = ListConfigurationSetsResponseDeserializer::deserialize(
                "ListConfigurationSetsResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Lists the existing custom verification email templates for your account in the current AWS Region.</p> <p>For more information about custom verification email templates, see <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/custom-verification-emails.html">Using Custom Verification Email Templates</a> in the <i>Amazon SES Developer Guide</i>.</p> <p>You can execute this operation no more than once per second.</p>
    async fn list_custom_verification_email_templates(
        &self,
        input: ListCustomVerificationEmailTemplatesRequest,
    ) -> Result<
        ListCustomVerificationEmailTemplatesResponse,
        RusotoError<ListCustomVerificationEmailTemplatesError>,
    > {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ListCustomVerificationEmailTemplates");
        params.put("Version", "2010-12-01");
        ListCustomVerificationEmailTemplatesRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(ListCustomVerificationEmailTemplatesError::from_response(
                response,
            ));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = ListCustomVerificationEmailTemplatesResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = ListCustomVerificationEmailTemplatesResponseDeserializer::deserialize(
                "ListCustomVerificationEmailTemplatesResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Returns a list containing all of the identities (email addresses and domains) for your AWS account in the current AWS Region, regardless of verification status.</p> <p>You can execute this operation no more than once per second.</p>
    async fn list_identities(
        &self,
        input: ListIdentitiesRequest,
    ) -> Result<ListIdentitiesResponse, RusotoError<ListIdentitiesError>> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ListIdentities");
        params.put("Version", "2010-12-01");
        ListIdentitiesRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(ListIdentitiesError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = ListIdentitiesResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = ListIdentitiesResponseDeserializer::deserialize(
                "ListIdentitiesResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Returns a list of sending authorization policies that are attached to the given identity (an email address or a domain). This API returns only a list. If you want the actual policy content, you can use <code>GetIdentityPolicies</code>.</p> <note> <p>This API is for the identity owner only. If you have not verified the identity, this API will return an error.</p> </note> <p>Sending authorization is a feature that enables an identity owner to authorize other senders to use its identities. For information about using sending authorization, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/sending-authorization.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    async fn list_identity_policies(
        &self,
        input: ListIdentityPoliciesRequest,
    ) -> Result<ListIdentityPoliciesResponse, RusotoError<ListIdentityPoliciesError>> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ListIdentityPolicies");
        params.put("Version", "2010-12-01");
        ListIdentityPoliciesRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(ListIdentityPoliciesError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = ListIdentityPoliciesResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = ListIdentityPoliciesResponseDeserializer::deserialize(
                "ListIdentityPoliciesResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Lists the IP address filters associated with your AWS account in the current AWS Region.</p> <p>For information about managing IP address filters, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-managing-ip-filters.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    async fn list_receipt_filters(
        &self,
        input: ListReceiptFiltersRequest,
    ) -> Result<ListReceiptFiltersResponse, RusotoError<ListReceiptFiltersError>> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ListReceiptFilters");
        params.put("Version", "2010-12-01");
        ListReceiptFiltersRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(ListReceiptFiltersError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = ListReceiptFiltersResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = ListReceiptFiltersResponseDeserializer::deserialize(
                "ListReceiptFiltersResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Lists the receipt rule sets that exist under your AWS account in the current AWS Region. If there are additional receipt rule sets to be retrieved, you will receive a <code>NextToken</code> that you can provide to the next call to <code>ListReceiptRuleSets</code> to retrieve the additional entries.</p> <p>For information about managing receipt rule sets, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-managing-receipt-rule-sets.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    async fn list_receipt_rule_sets(
        &self,
        input: ListReceiptRuleSetsRequest,
    ) -> Result<ListReceiptRuleSetsResponse, RusotoError<ListReceiptRuleSetsError>> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ListReceiptRuleSets");
        params.put("Version", "2010-12-01");
        ListReceiptRuleSetsRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(ListReceiptRuleSetsError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = ListReceiptRuleSetsResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = ListReceiptRuleSetsResponseDeserializer::deserialize(
                "ListReceiptRuleSetsResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Lists the email templates present in your Amazon SES account in the current AWS Region.</p> <p>You can execute this operation no more than once per second.</p>
    async fn list_templates(
        &self,
        input: ListTemplatesRequest,
    ) -> Result<ListTemplatesResponse, RusotoError<ListTemplatesError>> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ListTemplates");
        params.put("Version", "2010-12-01");
        ListTemplatesRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(ListTemplatesError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = ListTemplatesResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result =
                ListTemplatesResponseDeserializer::deserialize("ListTemplatesResult", &mut stack)?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Deprecated. Use the <code>ListIdentities</code> operation to list the email addresses and domains associated with your account.</p>
    async fn list_verified_email_addresses(
        &self,
    ) -> Result<ListVerifiedEmailAddressesResponse, RusotoError<ListVerifiedEmailAddressesError>>
    {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ListVerifiedEmailAddresses");
        params.put("Version", "2010-12-01");

        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(ListVerifiedEmailAddressesError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = ListVerifiedEmailAddressesResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = ListVerifiedEmailAddressesResponseDeserializer::deserialize(
                "ListVerifiedEmailAddressesResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Adds or updates the delivery options for a configuration set.</p>
    async fn put_configuration_set_delivery_options(
        &self,
        input: PutConfigurationSetDeliveryOptionsRequest,
    ) -> Result<
        PutConfigurationSetDeliveryOptionsResponse,
        RusotoError<PutConfigurationSetDeliveryOptionsError>,
    > {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "PutConfigurationSetDeliveryOptions");
        params.put("Version", "2010-12-01");
        PutConfigurationSetDeliveryOptionsRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(PutConfigurationSetDeliveryOptionsError::from_response(
                response,
            ));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = PutConfigurationSetDeliveryOptionsResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = PutConfigurationSetDeliveryOptionsResponseDeserializer::deserialize(
                "PutConfigurationSetDeliveryOptionsResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Adds or updates a sending authorization policy for the specified identity (an email address or a domain).</p> <note> <p>This API is for the identity owner only. If you have not verified the identity, this API will return an error.</p> </note> <p>Sending authorization is a feature that enables an identity owner to authorize other senders to use its identities. For information about using sending authorization, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/sending-authorization.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    async fn put_identity_policy(
        &self,
        input: PutIdentityPolicyRequest,
    ) -> Result<PutIdentityPolicyResponse, RusotoError<PutIdentityPolicyError>> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "PutIdentityPolicy");
        params.put("Version", "2010-12-01");
        PutIdentityPolicyRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(PutIdentityPolicyError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = PutIdentityPolicyResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = PutIdentityPolicyResponseDeserializer::deserialize(
                "PutIdentityPolicyResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Reorders the receipt rules within a receipt rule set.</p> <note> <p>All of the rules in the rule set must be represented in this request. That is, this API will return an error if the reorder request doesn't explicitly position all of the rules.</p> </note> <p>For information about managing receipt rule sets, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-managing-receipt-rule-sets.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    async fn reorder_receipt_rule_set(
        &self,
        input: ReorderReceiptRuleSetRequest,
    ) -> Result<ReorderReceiptRuleSetResponse, RusotoError<ReorderReceiptRuleSetError>> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ReorderReceiptRuleSet");
        params.put("Version", "2010-12-01");
        ReorderReceiptRuleSetRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(ReorderReceiptRuleSetError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = ReorderReceiptRuleSetResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = ReorderReceiptRuleSetResponseDeserializer::deserialize(
                "ReorderReceiptRuleSetResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Generates and sends a bounce message to the sender of an email you received through Amazon SES. You can only use this API on an email up to 24 hours after you receive it.</p> <note> <p>You cannot use this API to send generic bounces for mail that was not received by Amazon SES.</p> </note> <p>For information about receiving email through Amazon SES, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    async fn send_bounce(
        &self,
        input: SendBounceRequest,
    ) -> Result<SendBounceResponse, RusotoError<SendBounceError>> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "SendBounce");
        params.put("Version", "2010-12-01");
        SendBounceRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(SendBounceError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = SendBounceResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = SendBounceResponseDeserializer::deserialize("SendBounceResult", &mut stack)?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p><p>Composes an email message to multiple destinations. The message body is created using an email template.</p> <p>In order to send email using the <code>SendBulkTemplatedEmail</code> operation, your call to the API must meet the following requirements:</p> <ul> <li> <p>The call must refer to an existing email template. You can create email templates using the <a>CreateTemplate</a> operation.</p> </li> <li> <p>The message must be sent from a verified email address or domain.</p> </li> <li> <p>If your account is still in the Amazon SES sandbox, you may only send to verified addresses or domains, or to email addresses associated with the Amazon SES Mailbox Simulator. For more information, see <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/verify-addresses-and-domains.html">Verifying Email Addresses and Domains</a> in the <i>Amazon SES Developer Guide.</i> </p> </li> <li> <p>The maximum message size is 10 MB.</p> </li> <li> <p>Each <code>Destination</code> parameter must include at least one recipient email address. The recipient address can be a To: address, a CC: address, or a BCC: address. If a recipient email address is invalid (that is, it is not in the format <i>UserName@[SubDomain.]Domain.TopLevelDomain</i>), the entire message will be rejected, even if the message contains other recipients that are valid.</p> </li> <li> <p>The message may not include more than 50 recipients, across the To:, CC: and BCC: fields. If you need to send an email message to a larger audience, you can divide your recipient list into groups of 50 or fewer, and then call the <code>SendBulkTemplatedEmail</code> operation several times to send the message to each group.</p> </li> <li> <p>The number of destinations you can contact in a single call to the API may be limited by your account&#39;s maximum sending rate.</p> </li> </ul></p>
    async fn send_bulk_templated_email(
        &self,
        input: SendBulkTemplatedEmailRequest,
    ) -> Result<SendBulkTemplatedEmailResponse, RusotoError<SendBulkTemplatedEmailError>> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "SendBulkTemplatedEmail");
        params.put("Version", "2010-12-01");
        SendBulkTemplatedEmailRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(SendBulkTemplatedEmailError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = SendBulkTemplatedEmailResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = SendBulkTemplatedEmailResponseDeserializer::deserialize(
                "SendBulkTemplatedEmailResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Adds an email address to the list of identities for your Amazon SES account in the current AWS Region and attempts to verify it. As a result of executing this operation, a customized verification email is sent to the specified address.</p> <p>To use this operation, you must first create a custom verification email template. For more information about creating and using custom verification email templates, see <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/custom-verification-emails.html">Using Custom Verification Email Templates</a> in the <i>Amazon SES Developer Guide</i>.</p> <p>You can execute this operation no more than once per second.</p>
    async fn send_custom_verification_email(
        &self,
        input: SendCustomVerificationEmailRequest,
    ) -> Result<SendCustomVerificationEmailResponse, RusotoError<SendCustomVerificationEmailError>>
    {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "SendCustomVerificationEmail");
        params.put("Version", "2010-12-01");
        SendCustomVerificationEmailRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(SendCustomVerificationEmailError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = SendCustomVerificationEmailResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = SendCustomVerificationEmailResponseDeserializer::deserialize(
                "SendCustomVerificationEmailResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p><p>Composes an email message and immediately queues it for sending. In order to send email using the <code>SendEmail</code> operation, your message must meet the following requirements:</p> <ul> <li> <p>The message must be sent from a verified email address or domain. If you attempt to send email using a non-verified address or domain, the operation will result in an &quot;Email address not verified&quot; error. </p> </li> <li> <p>If your account is still in the Amazon SES sandbox, you may only send to verified addresses or domains, or to email addresses associated with the Amazon SES Mailbox Simulator. For more information, see <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/verify-addresses-and-domains.html">Verifying Email Addresses and Domains</a> in the <i>Amazon SES Developer Guide.</i> </p> </li> <li> <p>The maximum message size is 10 MB.</p> </li> <li> <p>The message must include at least one recipient email address. The recipient address can be a To: address, a CC: address, or a BCC: address. If a recipient email address is invalid (that is, it is not in the format <i>UserName@[SubDomain.]Domain.TopLevelDomain</i>), the entire message will be rejected, even if the message contains other recipients that are valid.</p> </li> <li> <p>The message may not include more than 50 recipients, across the To:, CC: and BCC: fields. If you need to send an email message to a larger audience, you can divide your recipient list into groups of 50 or fewer, and then call the <code>SendEmail</code> operation several times to send the message to each group.</p> </li> </ul> <important> <p>For every message that you send, the total number of recipients (including each recipient in the To:, CC: and BCC: fields) is counted against the maximum number of emails you can send in a 24-hour period (your <i>sending quota</i>). For more information about sending quotas in Amazon SES, see <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/manage-sending-limits.html">Managing Your Amazon SES Sending Limits</a> in the <i>Amazon SES Developer Guide.</i> </p> </important></p>
    async fn send_email(
        &self,
        input: SendEmailRequest,
    ) -> Result<SendEmailResponse, RusotoError<SendEmailError>> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "SendEmail");
        params.put("Version", "2010-12-01");
        SendEmailRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(SendEmailError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = SendEmailResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = SendEmailResponseDeserializer::deserialize("SendEmailResult", &mut stack)?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p><p>Composes an email message and immediately queues it for sending.</p> <p>This operation is more flexible than the <code>SendEmail</code> API operation. When you use the <code>SendRawEmail</code> operation, you can specify the headers of the message as well as its content. This flexibility is useful, for example, when you want to send a multipart MIME email (such a message that contains both a text and an HTML version). You can also use this operation to send messages that include attachments.</p> <p>The <code>SendRawEmail</code> operation has the following requirements:</p> <ul> <li> <p>You can only send email from <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/verify-addresses-and-domains.html">verified email addresses or domains</a>. If you try to send email from an address that isn&#39;t verified, the operation results in an &quot;Email address not verified&quot; error.</p> </li> <li> <p>If your account is still in the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/request-production-access.html">Amazon SES sandbox</a>, you can only send email to other verified addresses in your account, or to addresses that are associated with the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/mailbox-simulator.html">Amazon SES mailbox simulator</a>.</p> </li> <li> <p>The maximum message size, including attachments, is 10 MB.</p> </li> <li> <p>Each message has to include at least one recipient address. A recipient address includes any address on the To:, CC:, or BCC: lines.</p> </li> <li> <p>If you send a single message to more than one recipient address, and one of the recipient addresses isn&#39;t in a valid format (that is, it&#39;s not in the format <i>UserName@[SubDomain.]Domain.TopLevelDomain</i>), Amazon SES rejects the entire message, even if the other addresses are valid.</p> </li> <li> <p>Each message can include up to 50 recipient addresses across the To:, CC:, or BCC: lines. If you need to send a single message to more than 50 recipients, you have to split the list of recipient addresses into groups of less than 50 recipients, and send separate messages to each group.</p> </li> <li> <p>Amazon SES allows you to specify 8-bit Content-Transfer-Encoding for MIME message parts. However, if Amazon SES has to modify the contents of your message (for example, if you use open and click tracking), 8-bit content isn&#39;t preserved. For this reason, we highly recommend that you encode all content that isn&#39;t 7-bit ASCII. For more information, see <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/send-email-raw.html#send-email-mime-encoding">MIME Encoding</a> in the <i>Amazon SES Developer Guide</i>.</p> </li> </ul> <p>Additionally, keep the following considerations in mind when using the <code>SendRawEmail</code> operation:</p> <ul> <li> <p>Although you can customize the message headers when using the <code>SendRawEmail</code> operation, Amazon SES will automatically apply its own <code>Message-ID</code> and <code>Date</code> headers; if you passed these headers when creating the message, they will be overwritten by the values that Amazon SES provides.</p> </li> <li> <p>If you are using sending authorization to send on behalf of another user, <code>SendRawEmail</code> enables you to specify the cross-account identity for the email&#39;s Source, From, and Return-Path parameters in one of two ways: you can pass optional parameters <code>SourceArn</code>, <code>FromArn</code>, and/or <code>ReturnPathArn</code> to the API, or you can include the following X-headers in the header of your raw email:</p> <ul> <li> <p> <code>X-SES-SOURCE-ARN</code> </p> </li> <li> <p> <code>X-SES-FROM-ARN</code> </p> </li> <li> <p> <code>X-SES-RETURN-PATH-ARN</code> </p> </li> </ul> <important> <p>Don&#39;t include these X-headers in the DKIM signature. Amazon SES removes these before it sends the email.</p> </important> <p>If you only specify the <code>SourceIdentityArn</code> parameter, Amazon SES sets the From and Return-Path addresses to the same identity that you specified.</p> <p>For more information about sending authorization, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/sending-authorization.html">Using Sending Authorization with Amazon SES</a> in the <i>Amazon SES Developer Guide.</i> </p> </li> <li> <p>For every message that you send, the total number of recipients (including each recipient in the To:, CC: and BCC: fields) is counted against the maximum number of emails you can send in a 24-hour period (your <i>sending quota</i>). For more information about sending quotas in Amazon SES, see <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/manage-sending-limits.html">Managing Your Amazon SES Sending Limits</a> in the <i>Amazon SES Developer Guide.</i> </p> </li> </ul></p>
    async fn send_raw_email(
        &self,
        input: SendRawEmailRequest,
    ) -> Result<SendRawEmailResponse, RusotoError<SendRawEmailError>> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "SendRawEmail");
        params.put("Version", "2010-12-01");
        SendRawEmailRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(SendRawEmailError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = SendRawEmailResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result =
                SendRawEmailResponseDeserializer::deserialize("SendRawEmailResult", &mut stack)?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p><p>Composes an email message using an email template and immediately queues it for sending.</p> <p>In order to send email using the <code>SendTemplatedEmail</code> operation, your call to the API must meet the following requirements:</p> <ul> <li> <p>The call must refer to an existing email template. You can create email templates using the <a>CreateTemplate</a> operation.</p> </li> <li> <p>The message must be sent from a verified email address or domain.</p> </li> <li> <p>If your account is still in the Amazon SES sandbox, you may only send to verified addresses or domains, or to email addresses associated with the Amazon SES Mailbox Simulator. For more information, see <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/verify-addresses-and-domains.html">Verifying Email Addresses and Domains</a> in the <i>Amazon SES Developer Guide.</i> </p> </li> <li> <p>The maximum message size is 10 MB.</p> </li> <li> <p>Calls to the <code>SendTemplatedEmail</code> operation may only include one <code>Destination</code> parameter. A destination is a set of recipients who will receive the same version of the email. The <code>Destination</code> parameter can include up to 50 recipients, across the To:, CC: and BCC: fields.</p> </li> <li> <p>The <code>Destination</code> parameter must include at least one recipient email address. The recipient address can be a To: address, a CC: address, or a BCC: address. If a recipient email address is invalid (that is, it is not in the format <i>UserName@[SubDomain.]Domain.TopLevelDomain</i>), the entire message will be rejected, even if the message contains other recipients that are valid.</p> </li> </ul> <important> <p>If your call to the <code>SendTemplatedEmail</code> operation includes all of the required parameters, Amazon SES accepts it and returns a Message ID. However, if Amazon SES can&#39;t render the email because the template contains errors, it doesn&#39;t send the email. Additionally, because it already accepted the message, Amazon SES doesn&#39;t return a message stating that it was unable to send the email.</p> <p>For these reasons, we highly recommend that you set up Amazon SES to send you notifications when Rendering Failure events occur. For more information, see <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/send-personalized-email-api.html">Sending Personalized Email Using the Amazon SES API</a> in the <i>Amazon Simple Email Service Developer Guide</i>.</p> </important></p>
    async fn send_templated_email(
        &self,
        input: SendTemplatedEmailRequest,
    ) -> Result<SendTemplatedEmailResponse, RusotoError<SendTemplatedEmailError>> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "SendTemplatedEmail");
        params.put("Version", "2010-12-01");
        SendTemplatedEmailRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(SendTemplatedEmailError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = SendTemplatedEmailResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = SendTemplatedEmailResponseDeserializer::deserialize(
                "SendTemplatedEmailResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Sets the specified receipt rule set as the active receipt rule set.</p> <note> <p>To disable your email-receiving through Amazon SES completely, you can call this API with RuleSetName set to null.</p> </note> <p>For information about managing receipt rule sets, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-managing-receipt-rule-sets.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    async fn set_active_receipt_rule_set(
        &self,
        input: SetActiveReceiptRuleSetRequest,
    ) -> Result<SetActiveReceiptRuleSetResponse, RusotoError<SetActiveReceiptRuleSetError>> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "SetActiveReceiptRuleSet");
        params.put("Version", "2010-12-01");
        SetActiveReceiptRuleSetRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(SetActiveReceiptRuleSetError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = SetActiveReceiptRuleSetResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = SetActiveReceiptRuleSetResponseDeserializer::deserialize(
                "SetActiveReceiptRuleSetResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Enables or disables Easy DKIM signing of email sent from an identity. If Easy DKIM signing is enabled for a domain, then Amazon SES uses DKIM to sign all email that it sends from addresses on that domain. If Easy DKIM signing is enabled for an email address, then Amazon SES uses DKIM to sign all email it sends from that address.</p> <note> <p>For email addresses (for example, <code>user@example.com</code>), you can only enable DKIM signing if the corresponding domain (in this case, <code>example.com</code>) has been set up to use Easy DKIM.</p> </note> <p>You can enable DKIM signing for an identity at any time after you start the verification process for the identity, even if the verification process isn't complete. </p> <p>You can execute this operation no more than once per second.</p> <p>For more information about Easy DKIM signing, go to the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/easy-dkim.html">Amazon SES Developer Guide</a>.</p>
    async fn set_identity_dkim_enabled(
        &self,
        input: SetIdentityDkimEnabledRequest,
    ) -> Result<SetIdentityDkimEnabledResponse, RusotoError<SetIdentityDkimEnabledError>> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "SetIdentityDkimEnabled");
        params.put("Version", "2010-12-01");
        SetIdentityDkimEnabledRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(SetIdentityDkimEnabledError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = SetIdentityDkimEnabledResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = SetIdentityDkimEnabledResponseDeserializer::deserialize(
                "SetIdentityDkimEnabledResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Given an identity (an email address or a domain), enables or disables whether Amazon SES forwards bounce and complaint notifications as email. Feedback forwarding can only be disabled when Amazon Simple Notification Service (Amazon SNS) topics are specified for both bounces and complaints.</p> <note> <p>Feedback forwarding does not apply to delivery notifications. Delivery notifications are only available through Amazon SNS.</p> </note> <p>You can execute this operation no more than once per second.</p> <p>For more information about using notifications with Amazon SES, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/notifications.html">Amazon SES Developer Guide</a>.</p>
    async fn set_identity_feedback_forwarding_enabled(
        &self,
        input: SetIdentityFeedbackForwardingEnabledRequest,
    ) -> Result<
        SetIdentityFeedbackForwardingEnabledResponse,
        RusotoError<SetIdentityFeedbackForwardingEnabledError>,
    > {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "SetIdentityFeedbackForwardingEnabled");
        params.put("Version", "2010-12-01");
        SetIdentityFeedbackForwardingEnabledRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(SetIdentityFeedbackForwardingEnabledError::from_response(
                response,
            ));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = SetIdentityFeedbackForwardingEnabledResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = SetIdentityFeedbackForwardingEnabledResponseDeserializer::deserialize(
                "SetIdentityFeedbackForwardingEnabledResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Given an identity (an email address or a domain), sets whether Amazon SES includes the original email headers in the Amazon Simple Notification Service (Amazon SNS) notifications of a specified type.</p> <p>You can execute this operation no more than once per second.</p> <p>For more information about using notifications with Amazon SES, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/notifications.html">Amazon SES Developer Guide</a>.</p>
    async fn set_identity_headers_in_notifications_enabled(
        &self,
        input: SetIdentityHeadersInNotificationsEnabledRequest,
    ) -> Result<
        SetIdentityHeadersInNotificationsEnabledResponse,
        RusotoError<SetIdentityHeadersInNotificationsEnabledError>,
    > {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "SetIdentityHeadersInNotificationsEnabled");
        params.put("Version", "2010-12-01");
        SetIdentityHeadersInNotificationsEnabledRequestSerializer::serialize(
            &mut params,
            "",
            &input,
        );
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(SetIdentityHeadersInNotificationsEnabledError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = SetIdentityHeadersInNotificationsEnabledResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = SetIdentityHeadersInNotificationsEnabledResponseDeserializer::deserialize(
                "SetIdentityHeadersInNotificationsEnabledResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Enables or disables the custom MAIL FROM domain setup for a verified identity (an email address or a domain).</p> <important> <p>To send emails using the specified MAIL FROM domain, you must add an MX record to your MAIL FROM domain's DNS settings. If you want your emails to pass Sender Policy Framework (SPF) checks, you must also add or update an SPF record. For more information, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/mail-from-set.html">Amazon SES Developer Guide</a>.</p> </important> <p>You can execute this operation no more than once per second.</p>
    async fn set_identity_mail_from_domain(
        &self,
        input: SetIdentityMailFromDomainRequest,
    ) -> Result<SetIdentityMailFromDomainResponse, RusotoError<SetIdentityMailFromDomainError>>
    {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "SetIdentityMailFromDomain");
        params.put("Version", "2010-12-01");
        SetIdentityMailFromDomainRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(SetIdentityMailFromDomainError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = SetIdentityMailFromDomainResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = SetIdentityMailFromDomainResponseDeserializer::deserialize(
                "SetIdentityMailFromDomainResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Sets an Amazon Simple Notification Service (Amazon SNS) topic to use when delivering notifications. When you use this operation, you specify a verified identity, such as an email address or domain. When you send an email that uses the chosen identity in the Source field, Amazon SES sends notifications to the topic you specified. You can send bounce, complaint, or delivery notifications (or any combination of the three) to the Amazon SNS topic that you specify.</p> <p>You can execute this operation no more than once per second.</p> <p>For more information about feedback notification, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/notifications.html">Amazon SES Developer Guide</a>.</p>
    async fn set_identity_notification_topic(
        &self,
        input: SetIdentityNotificationTopicRequest,
    ) -> Result<SetIdentityNotificationTopicResponse, RusotoError<SetIdentityNotificationTopicError>>
    {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "SetIdentityNotificationTopic");
        params.put("Version", "2010-12-01");
        SetIdentityNotificationTopicRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(SetIdentityNotificationTopicError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = SetIdentityNotificationTopicResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = SetIdentityNotificationTopicResponseDeserializer::deserialize(
                "SetIdentityNotificationTopicResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Sets the position of the specified receipt rule in the receipt rule set.</p> <p>For information about managing receipt rules, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-managing-receipt-rules.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    async fn set_receipt_rule_position(
        &self,
        input: SetReceiptRulePositionRequest,
    ) -> Result<SetReceiptRulePositionResponse, RusotoError<SetReceiptRulePositionError>> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "SetReceiptRulePosition");
        params.put("Version", "2010-12-01");
        SetReceiptRulePositionRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(SetReceiptRulePositionError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = SetReceiptRulePositionResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = SetReceiptRulePositionResponseDeserializer::deserialize(
                "SetReceiptRulePositionResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Creates a preview of the MIME content of an email when provided with a template and a set of replacement data.</p> <p>You can execute this operation no more than once per second.</p>
    async fn test_render_template(
        &self,
        input: TestRenderTemplateRequest,
    ) -> Result<TestRenderTemplateResponse, RusotoError<TestRenderTemplateError>> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "TestRenderTemplate");
        params.put("Version", "2010-12-01");
        TestRenderTemplateRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(TestRenderTemplateError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = TestRenderTemplateResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = TestRenderTemplateResponseDeserializer::deserialize(
                "TestRenderTemplateResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Enables or disables email sending across your entire Amazon SES account in the current AWS Region. You can use this operation in conjunction with Amazon CloudWatch alarms to temporarily pause email sending across your Amazon SES account in a given AWS Region when reputation metrics (such as your bounce or complaint rates) reach certain thresholds.</p> <p>You can execute this operation no more than once per second.</p>
    async fn update_account_sending_enabled(
        &self,
        input: UpdateAccountSendingEnabledRequest,
    ) -> Result<(), RusotoError<UpdateAccountSendingEnabledError>> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "UpdateAccountSendingEnabled");
        params.put("Version", "2010-12-01");
        UpdateAccountSendingEnabledRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(UpdateAccountSendingEnabledError::from_response(response));
        }

        std::mem::drop(response);
        Ok(())
    }

    /// <p>Updates the event destination of a configuration set. Event destinations are associated with configuration sets, which enable you to publish email sending events to Amazon CloudWatch, Amazon Kinesis Firehose, or Amazon Simple Notification Service (Amazon SNS). For information about using configuration sets, see <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/monitor-sending-activity.html">Monitoring Your Amazon SES Sending Activity</a> in the <i>Amazon SES Developer Guide.</i> </p> <note> <p>When you create or update an event destination, you must provide one, and only one, destination. The destination can be Amazon CloudWatch, Amazon Kinesis Firehose, or Amazon Simple Notification Service (Amazon SNS).</p> </note> <p>You can execute this operation no more than once per second.</p>
    async fn update_configuration_set_event_destination(
        &self,
        input: UpdateConfigurationSetEventDestinationRequest,
    ) -> Result<
        UpdateConfigurationSetEventDestinationResponse,
        RusotoError<UpdateConfigurationSetEventDestinationError>,
    > {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "UpdateConfigurationSetEventDestination");
        params.put("Version", "2010-12-01");
        UpdateConfigurationSetEventDestinationRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(UpdateConfigurationSetEventDestinationError::from_response(
                response,
            ));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = UpdateConfigurationSetEventDestinationResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = UpdateConfigurationSetEventDestinationResponseDeserializer::deserialize(
                "UpdateConfigurationSetEventDestinationResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Enables or disables the publishing of reputation metrics for emails sent using a specific configuration set in a given AWS Region. Reputation metrics include bounce and complaint rates. These metrics are published to Amazon CloudWatch. By using CloudWatch, you can create alarms when bounce or complaint rates exceed certain thresholds.</p> <p>You can execute this operation no more than once per second.</p>
    async fn update_configuration_set_reputation_metrics_enabled(
        &self,
        input: UpdateConfigurationSetReputationMetricsEnabledRequest,
    ) -> Result<(), RusotoError<UpdateConfigurationSetReputationMetricsEnabledError>> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "UpdateConfigurationSetReputationMetricsEnabled");
        params.put("Version", "2010-12-01");
        UpdateConfigurationSetReputationMetricsEnabledRequestSerializer::serialize(
            &mut params,
            "",
            &input,
        );
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(
                UpdateConfigurationSetReputationMetricsEnabledError::from_response(response),
            );
        }

        std::mem::drop(response);
        Ok(())
    }

    /// <p>Enables or disables email sending for messages sent using a specific configuration set in a given AWS Region. You can use this operation in conjunction with Amazon CloudWatch alarms to temporarily pause email sending for a configuration set when the reputation metrics for that configuration set (such as your bounce on complaint rate) exceed certain thresholds.</p> <p>You can execute this operation no more than once per second.</p>
    async fn update_configuration_set_sending_enabled(
        &self,
        input: UpdateConfigurationSetSendingEnabledRequest,
    ) -> Result<(), RusotoError<UpdateConfigurationSetSendingEnabledError>> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "UpdateConfigurationSetSendingEnabled");
        params.put("Version", "2010-12-01");
        UpdateConfigurationSetSendingEnabledRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(UpdateConfigurationSetSendingEnabledError::from_response(
                response,
            ));
        }

        std::mem::drop(response);
        Ok(())
    }

    /// <p>Modifies an association between a configuration set and a custom domain for open and click event tracking. </p> <p>By default, images and links used for tracking open and click events are hosted on domains operated by Amazon SES. You can configure a subdomain of your own to handle these events. For information about using custom domains, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/configure-custom-open-click-domains.html">Amazon SES Developer Guide</a>.</p>
    async fn update_configuration_set_tracking_options(
        &self,
        input: UpdateConfigurationSetTrackingOptionsRequest,
    ) -> Result<
        UpdateConfigurationSetTrackingOptionsResponse,
        RusotoError<UpdateConfigurationSetTrackingOptionsError>,
    > {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "UpdateConfigurationSetTrackingOptions");
        params.put("Version", "2010-12-01");
        UpdateConfigurationSetTrackingOptionsRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(UpdateConfigurationSetTrackingOptionsError::from_response(
                response,
            ));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = UpdateConfigurationSetTrackingOptionsResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = UpdateConfigurationSetTrackingOptionsResponseDeserializer::deserialize(
                "UpdateConfigurationSetTrackingOptionsResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Updates an existing custom verification email template.</p> <p>For more information about custom verification email templates, see <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/custom-verification-emails.html">Using Custom Verification Email Templates</a> in the <i>Amazon SES Developer Guide</i>.</p> <p>You can execute this operation no more than once per second.</p>
    async fn update_custom_verification_email_template(
        &self,
        input: UpdateCustomVerificationEmailTemplateRequest,
    ) -> Result<(), RusotoError<UpdateCustomVerificationEmailTemplateError>> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "UpdateCustomVerificationEmailTemplate");
        params.put("Version", "2010-12-01");
        UpdateCustomVerificationEmailTemplateRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(UpdateCustomVerificationEmailTemplateError::from_response(
                response,
            ));
        }

        std::mem::drop(response);
        Ok(())
    }

    /// <p>Updates a receipt rule.</p> <p>For information about managing receipt rules, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-managing-receipt-rules.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    async fn update_receipt_rule(
        &self,
        input: UpdateReceiptRuleRequest,
    ) -> Result<UpdateReceiptRuleResponse, RusotoError<UpdateReceiptRuleError>> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "UpdateReceiptRule");
        params.put("Version", "2010-12-01");
        UpdateReceiptRuleRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(UpdateReceiptRuleError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = UpdateReceiptRuleResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = UpdateReceiptRuleResponseDeserializer::deserialize(
                "UpdateReceiptRuleResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Updates an email template. Email templates enable you to send personalized email to one or more destinations in a single API operation. For more information, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/send-personalized-email-api.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    async fn update_template(
        &self,
        input: UpdateTemplateRequest,
    ) -> Result<UpdateTemplateResponse, RusotoError<UpdateTemplateError>> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "UpdateTemplate");
        params.put("Version", "2010-12-01");
        UpdateTemplateRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(UpdateTemplateError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = UpdateTemplateResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = UpdateTemplateResponseDeserializer::deserialize(
                "UpdateTemplateResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Returns a set of DKIM tokens for a domain identity.</p> <important> <p>When you execute the <code>VerifyDomainDkim</code> operation, the domain that you specify is added to the list of identities that are associated with your account. This is true even if you haven't already associated the domain with your account by using the <code>VerifyDomainIdentity</code> operation. However, you can't send email from the domain until you either successfully <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/verify-domains.html">verify it</a> or you successfully <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/easy-dkim.html">set up DKIM for it</a>.</p> </important> <p>You use the tokens that are generated by this operation to create CNAME records. When Amazon SES detects that you've added these records to the DNS configuration for a domain, you can start sending email from that domain. You can start sending email even if you haven't added the TXT record provided by the VerifyDomainIdentity operation to the DNS configuration for your domain. All email that you send from the domain is authenticated using DKIM.</p> <p>To create the CNAME records for DKIM authentication, use the following values:</p> <ul> <li> <p> <b>Name</b>: <i>token</i>._domainkey.<i>example.com</i> </p> </li> <li> <p> <b>Type</b>: CNAME</p> </li> <li> <p> <b>Value</b>: <i>token</i>.dkim.amazonses.com</p> </li> </ul> <p>In the preceding example, replace <i>token</i> with one of the tokens that are generated when you execute this operation. Replace <i>example.com</i> with your domain. Repeat this process for each token that's generated by this operation.</p> <p>You can execute this operation no more than once per second.</p>
    async fn verify_domain_dkim(
        &self,
        input: VerifyDomainDkimRequest,
    ) -> Result<VerifyDomainDkimResponse, RusotoError<VerifyDomainDkimError>> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "VerifyDomainDkim");
        params.put("Version", "2010-12-01");
        VerifyDomainDkimRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(VerifyDomainDkimError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = VerifyDomainDkimResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = VerifyDomainDkimResponseDeserializer::deserialize(
                "VerifyDomainDkimResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Adds a domain to the list of identities for your Amazon SES account in the current AWS Region and attempts to verify it. For more information about verifying domains, see <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/verify-addresses-and-domains.html">Verifying Email Addresses and Domains</a> in the <i>Amazon SES Developer Guide.</i> </p> <p>You can execute this operation no more than once per second.</p>
    async fn verify_domain_identity(
        &self,
        input: VerifyDomainIdentityRequest,
    ) -> Result<VerifyDomainIdentityResponse, RusotoError<VerifyDomainIdentityError>> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "VerifyDomainIdentity");
        params.put("Version", "2010-12-01");
        VerifyDomainIdentityRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(VerifyDomainIdentityError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = VerifyDomainIdentityResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = VerifyDomainIdentityResponseDeserializer::deserialize(
                "VerifyDomainIdentityResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Deprecated. Use the <code>VerifyEmailIdentity</code> operation to verify a new email address.</p>
    async fn verify_email_address(
        &self,
        input: VerifyEmailAddressRequest,
    ) -> Result<(), RusotoError<VerifyEmailAddressError>> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "VerifyEmailAddress");
        params.put("Version", "2010-12-01");
        VerifyEmailAddressRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(VerifyEmailAddressError::from_response(response));
        }

        std::mem::drop(response);
        Ok(())
    }

    /// <p>Adds an email address to the list of identities for your Amazon SES account in the current AWS region and attempts to verify it. As a result of executing this operation, a verification email is sent to the specified address.</p> <p>You can execute this operation no more than once per second.</p>
    async fn verify_email_identity(
        &self,
        input: VerifyEmailIdentityRequest,
    ) -> Result<VerifyEmailIdentityResponse, RusotoError<VerifyEmailIdentityError>> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "VerifyEmailIdentity");
        params.put("Version", "2010-12-01");
        VerifyEmailIdentityRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(VerifyEmailIdentityError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = VerifyEmailIdentityResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = VerifyEmailIdentityResponseDeserializer::deserialize(
                "VerifyEmailIdentityResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }
}

#[cfg(test)]
mod protocol_tests {

    extern crate rusoto_mock;

    use self::rusoto_mock::*;
    use super::*;
    use rusoto_core::Region as rusoto_region;

    #[tokio::test]
    async fn test_parse_error_ses_delete_identity() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/error",
            "ses-delete-identity.xml",
        );
        let mock = MockRequestDispatcher::with_status(400).with_body(&mock_response);
        let client = SesClient::new_with(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = DeleteIdentityRequest::default();
        let result = client.delete_identity(request).await;
        assert!(!result.is_ok(), "parse error: {:?}", result);
    }

    #[tokio::test]
    async fn test_parse_valid_ses_delete_identity() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "ses-delete-identity.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = SesClient::new_with(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = DeleteIdentityRequest::default();
        let result = client.delete_identity(request).await;
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[tokio::test]
    async fn test_parse_valid_ses_get_identity_dkim_attributes() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "ses-get-identity-dkim-attributes.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = SesClient::new_with(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = GetIdentityDkimAttributesRequest::default();
        let result = client.get_identity_dkim_attributes(request).await;
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[tokio::test]
    async fn test_parse_valid_ses_get_identity_notification_attributes() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "ses-get-identity-notification-attributes.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = SesClient::new_with(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = GetIdentityNotificationAttributesRequest::default();
        let result = client.get_identity_notification_attributes(request).await;
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[tokio::test]
    async fn test_parse_valid_ses_get_identity_verification_attributes() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "ses-get-identity-verification-attributes.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = SesClient::new_with(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = GetIdentityVerificationAttributesRequest::default();
        let result = client.get_identity_verification_attributes(request).await;
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[tokio::test]
    async fn test_parse_valid_ses_get_send_quota() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "ses-get-send-quota.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = SesClient::new_with(mock, MockCredentialsProvider, rusoto_region::UsEast1);

        let result = client.get_send_quota().await;
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[tokio::test]
    async fn test_parse_valid_ses_get_send_statistics() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "ses-get-send-statistics.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = SesClient::new_with(mock, MockCredentialsProvider, rusoto_region::UsEast1);

        let result = client.get_send_statistics().await;
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[tokio::test]
    async fn test_parse_valid_ses_list_identities() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "ses-list-identities.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = SesClient::new_with(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = ListIdentitiesRequest::default();
        let result = client.list_identities(request).await;
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[tokio::test]
    async fn test_parse_valid_ses_send_email() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "ses-send-email.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = SesClient::new_with(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = SendEmailRequest::default();
        let result = client.send_email(request).await;
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[tokio::test]
    async fn test_parse_valid_ses_send_raw_email() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "ses-send-raw-email.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = SesClient::new_with(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = SendRawEmailRequest::default();
        let result = client.send_raw_email(request).await;
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[tokio::test]
    async fn test_parse_valid_ses_set_identity_dkim_enabled() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "ses-set-identity-dkim-enabled.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = SesClient::new_with(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = SetIdentityDkimEnabledRequest::default();
        let result = client.set_identity_dkim_enabled(request).await;
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[tokio::test]
    async fn test_parse_valid_ses_verify_domain_dkim() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "ses-verify-domain-dkim.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = SesClient::new_with(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = VerifyDomainDkimRequest::default();
        let result = client.verify_domain_dkim(request).await;
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[tokio::test]
    async fn test_parse_valid_ses_verify_domain_identity() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "ses-verify-domain-identity.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = SesClient::new_with(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = VerifyDomainIdentityRequest::default();
        let result = client.verify_domain_identity(request).await;
        assert!(result.is_ok(), "parse error: {:?}", result);
    }
}
