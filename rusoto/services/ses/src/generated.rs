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
use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::signature::SignedRequest;
use rusoto_core::xmlerror::*;
use rusoto_core::xmlutil::{
    characters, end_element, find_start_element, peek_at_name, skip_tree, start_element,
};
use rusoto_core::xmlutil::{Next, Peek, XmlParseError, XmlResponse};
use std::str::FromStr;
use xml::reader::ParserConfig;
use xml::reader::XmlEvent;
use xml::EventReader;

enum DeserializerNext {
    Close,
    Skip,
    Element(String),
}
/// <p>When included in a receipt rule, this action adds a header to the received email.</p> <p>For information about adding a header using a receipt rule, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-action-add-header.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct AddHeaderAction {
    /// <p>The name of the header to add. Must be between 1 and 50 characters, inclusive, and consist of alphanumeric (a-z, A-Z, 0-9) characters and dashes only.</p>
    pub header_name: String,
    /// <p>Must be less than 2048 characters, and must not contain newline characters ("\r" or "\n").</p>
    pub header_value: String,
}

struct AddHeaderActionDeserializer;
impl AddHeaderActionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AddHeaderAction, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = AddHeaderAction::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "HeaderName" => {
                        obj.header_name =
                            try!(HeaderNameDeserializer::deserialize("HeaderName", stack));
                    }
                    "HeaderValue" => {
                        obj.header_value =
                            try!(HeaderValueDeserializer::deserialize("HeaderValue", stack));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
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

        params.put(
            &format!("{}{}", prefix, "HeaderName"),
            &obj.header_name.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "HeaderValue"),
            &obj.header_value.replace("+", "%2B"),
        );
    }
}

struct AddressDeserializer;
impl AddressDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct AddressListDeserializer;
impl AddressListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "member" {
                        obj.push(try!(AddressDeserializer::deserialize("member", stack)));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
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
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct BehaviorOnMXFailureDeserializer;
impl BehaviorOnMXFailureDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Represents the body of the message. You can specify text, HTML, or both. If you use both, then the message should display correctly in the widest variety of email clients.</p>
#[derive(Default, Debug, Clone, PartialEq)]
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

/// <p>When included in a receipt rule, this action rejects the received email by returning a bounce response to the sender and, optionally, publishes a notification to Amazon Simple Notification Service (Amazon SNS).</p> <p>For information about sending a bounce message in response to a received email, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-action-bounce.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct BounceAction {
    /// <p>Human-readable text to include in the bounce message.</p>
    pub message: String,
    /// <p>The email address of the sender of the bounced email. This is the address from which the bounce message will be sent.</p>
    pub sender: String,
    /// <p>The SMTP reply code, as defined by <a href="https://tools.ietf.org/html/rfc5321">RFC 5321</a>.</p>
    pub smtp_reply_code: String,
    /// <p>The SMTP enhanced status code, as defined by <a href="https://tools.ietf.org/html/rfc3463">RFC 3463</a>.</p>
    pub status_code: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the Amazon SNS topic to notify when the bounce action is taken. An example of an Amazon SNS topic ARN is <code>arn:aws:sns:us-west-2:123456789012:MyTopic</code>. For more information about Amazon SNS topics, see the <a href="http://docs.aws.amazon.com/sns/latest/dg/CreateTopic.html">Amazon SNS Developer Guide</a>.</p>
    pub topic_arn: Option<String>,
}

struct BounceActionDeserializer;
impl BounceActionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<BounceAction, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = BounceAction::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Message" => {
                        obj.message =
                            try!(BounceMessageDeserializer::deserialize("Message", stack));
                    }
                    "Sender" => {
                        obj.sender = try!(AddressDeserializer::deserialize("Sender", stack));
                    }
                    "SmtpReplyCode" => {
                        obj.smtp_reply_code = try!(BounceSmtpReplyCodeDeserializer::deserialize(
                            "SmtpReplyCode",
                            stack
                        ));
                    }
                    "StatusCode" => {
                        obj.status_code = Some(try!(BounceStatusCodeDeserializer::deserialize(
                            "StatusCode",
                            stack
                        )));
                    }
                    "TopicArn" => {
                        obj.topic_arn = Some(try!(AmazonResourceNameDeserializer::deserialize(
                            "TopicArn", stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
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

        params.put(
            &format!("{}{}", prefix, "Message"),
            &obj.message.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "Sender"),
            &obj.sender.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "SmtpReplyCode"),
            &obj.smtp_reply_code.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.status_code {
            params.put(
                &format!("{}{}", prefix, "StatusCode"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.topic_arn {
            params.put(
                &format!("{}{}", prefix, "TopicArn"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

struct BounceMessageDeserializer;
impl BounceMessageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct BounceSmtpReplyCodeDeserializer;
impl BounceSmtpReplyCodeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct BounceStatusCodeDeserializer;
impl BounceStatusCodeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Recipient-related information to include in the Delivery Status Notification (DSN) when an email that Amazon SES receives on your behalf bounces.</p> <p>For information about receiving email through Amazon SES, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct BouncedRecipientInfo {
    /// <p>The reason for the bounce. You must provide either this parameter or <code>RecipientDsnFields</code>.</p>
    pub bounce_type: Option<String>,
    /// <p>The email address of the recipient of the bounced email.</p>
    pub recipient: String,
    /// <p>This parameter is used only for sending authorization. It is the ARN of the identity that is associated with the sending authorization policy that permits you to receive email for the recipient of the bounced email. For more information about sending authorization, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/sending-authorization.html">Amazon SES Developer Guide</a>.</p>
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
            params.put(
                &format!("{}{}", prefix, "BounceType"),
                &field_value.replace("+", "%2B"),
            );
        }
        params.put(
            &format!("{}{}", prefix, "Recipient"),
            &obj.recipient.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.recipient_arn {
            params.put(
                &format!("{}{}", prefix, "RecipientArn"),
                &field_value.replace("+", "%2B"),
            );
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
                &field_value.replace("+", "%2B"),
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
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<BulkEmailDestinationStatus, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = BulkEmailDestinationStatus::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Error" => {
                        obj.error = Some(try!(SesErrorDeserializer::deserialize("Error", stack)));
                    }
                    "MessageId" => {
                        obj.message_id =
                            Some(try!(MessageIdDeserializer::deserialize("MessageId", stack)));
                    }
                    "Status" => {
                        obj.status = Some(try!(BulkEmailStatusDeserializer::deserialize(
                            "Status", stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct BulkEmailDestinationStatusListDeserializer;
impl BulkEmailDestinationStatusListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<BulkEmailDestinationStatus>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "member" {
                        obj.push(try!(BulkEmailDestinationStatusDeserializer::deserialize(
                            "member", stack
                        )));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}
struct BulkEmailStatusDeserializer;
impl BulkEmailStatusDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct CidrDeserializer;
impl CidrDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Represents a request to create a receipt rule set by cloning an existing one. You use receipt rule sets to receive email with Amazon SES. For more information, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-concepts.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
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
            &obj.original_rule_set_name.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "RuleSetName"),
            &obj.rule_set_name.replace("+", "%2B"),
        );
    }
}

/// <p>An empty element returned on a successful request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CloneReceiptRuleSetResponse {}

struct CloneReceiptRuleSetResponseDeserializer;
impl CloneReceiptRuleSetResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CloneReceiptRuleSetResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let obj = CloneReceiptRuleSetResponse::default();

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Contains information associated with an Amazon CloudWatch event destination to which email sending events are published.</p> <p>Event destinations, such as Amazon CloudWatch, are associated with configuration sets, which enable you to publish email sending events. For information about using configuration sets, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/monitor-sending-activity.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CloudWatchDestination {
    /// <p>A list of dimensions upon which to categorize your emails when you publish email sending events to Amazon CloudWatch.</p>
    pub dimension_configurations: Vec<CloudWatchDimensionConfiguration>,
}

struct CloudWatchDestinationDeserializer;
impl CloudWatchDestinationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CloudWatchDestination, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CloudWatchDestination::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "DimensionConfigurations" => {
                        obj.dimension_configurations =
                            try!(CloudWatchDimensionConfigurationsDeserializer::deserialize(
                                "DimensionConfigurations",
                                stack
                            ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
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

/// <p>Contains the dimension configuration to use when you publish email sending events to Amazon CloudWatch.</p> <p>For information about publishing email sending events to Amazon CloudWatch, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/monitor-sending-activity.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
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
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CloudWatchDimensionConfiguration, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CloudWatchDimensionConfiguration::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "DefaultDimensionValue" => {
                        obj.default_dimension_value =
                            try!(DefaultDimensionValueDeserializer::deserialize(
                                "DefaultDimensionValue",
                                stack
                            ));
                    }
                    "DimensionName" => {
                        obj.dimension_name = try!(DimensionNameDeserializer::deserialize(
                            "DimensionName",
                            stack
                        ));
                    }
                    "DimensionValueSource" => {
                        obj.dimension_value_source =
                            try!(DimensionValueSourceDeserializer::deserialize(
                                "DimensionValueSource",
                                stack
                            ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
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
            &obj.default_dimension_value.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "DimensionName"),
            &obj.dimension_name.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "DimensionValueSource"),
            &obj.dimension_value_source.replace("+", "%2B"),
        );
    }
}

struct CloudWatchDimensionConfigurationsDeserializer;
impl CloudWatchDimensionConfigurationsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<CloudWatchDimensionConfiguration>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "member" {
                        obj.push(try!(
                            CloudWatchDimensionConfigurationDeserializer::deserialize(
                                "member", stack
                            )
                        ));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
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

/// <p>The name of the configuration set.</p> <p>Configuration sets let you create groups of rules that you can apply to the emails you send using Amazon SES. For more information about using configuration sets, see <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/using-configuration-sets.html">Using Amazon SES Configuration Sets</a> in the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ConfigurationSet {
    /// <p><p>The name of the configuration set. The name must meet the following requirements:</p> <ul> <li> <p>Contain only letters (a-z, A-Z), numbers (0-9), underscores (_), or dashes (-).</p> </li> <li> <p>Contain 64 characters or fewer.</p> </li> </ul></p>
    pub name: String,
}

struct ConfigurationSetDeserializer;
impl ConfigurationSetDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ConfigurationSet, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ConfigurationSet::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Name" => {
                        obj.name =
                            try!(ConfigurationSetNameDeserializer::deserialize("Name", stack));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
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

        params.put(
            &format!("{}{}", prefix, "Name"),
            &obj.name.replace("+", "%2B"),
        );
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
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct ConfigurationSetsDeserializer;
impl ConfigurationSetsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<ConfigurationSet>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "member" {
                        obj.push(try!(ConfigurationSetDeserializer::deserialize(
                            "member", stack
                        )));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}
/// <p>Represents textual data, plus an optional character set specification.</p> <p>By default, the text must be 7-bit ASCII, due to the constraints of the SMTP protocol. If the text must contain any other characters, then you must also specify a character set. Examples include UTF-8, ISO-8859-1, and Shift_JIS.</p>
#[derive(Default, Debug, Clone, PartialEq)]
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
            params.put(
                &format!("{}{}", prefix, "Charset"),
                &field_value.replace("+", "%2B"),
            );
        }
        params.put(
            &format!("{}{}", prefix, "Data"),
            &obj.data.replace("+", "%2B"),
        );
    }
}

struct CounterDeserializer;
impl CounterDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<i64, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = i64::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Represents a request to create a configuration set event destination. A configuration set event destination, which can be either Amazon CloudWatch or Amazon Kinesis Firehose, describes an AWS service in which Amazon SES publishes the email sending events associated with a configuration set. For information about using configuration sets, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/monitor-sending-activity.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
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
            &obj.configuration_set_name.replace("+", "%2B"),
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
pub struct CreateConfigurationSetEventDestinationResponse {}

struct CreateConfigurationSetEventDestinationResponseDeserializer;
impl CreateConfigurationSetEventDestinationResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateConfigurationSetEventDestinationResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let obj = CreateConfigurationSetEventDestinationResponse::default();

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Represents a request to create a configuration set. Configuration sets enable you to publish email sending events. For information about using configuration sets, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/monitor-sending-activity.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
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
pub struct CreateConfigurationSetResponse {}

struct CreateConfigurationSetResponseDeserializer;
impl CreateConfigurationSetResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateConfigurationSetResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let obj = CreateConfigurationSetResponse::default();

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Represents a request to create an open and click tracking option object in a configuration set. </p>
#[derive(Default, Debug, Clone, PartialEq)]
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
            &obj.configuration_set_name.replace("+", "%2B"),
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
pub struct CreateConfigurationSetTrackingOptionsResponse {}

struct CreateConfigurationSetTrackingOptionsResponseDeserializer;
impl CreateConfigurationSetTrackingOptionsResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateConfigurationSetTrackingOptionsResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let obj = CreateConfigurationSetTrackingOptionsResponse::default();

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Represents a request to create a custom verification email template.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateCustomVerificationEmailTemplateRequest {
    /// <p>The URL that the recipient of the verification email is sent to if his or her address is not successfully verified.</p>
    pub failure_redirection_url: String,
    /// <p>The email address that the custom verification email is sent from.</p>
    pub from_email_address: String,
    /// <p>The URL that the recipient of the verification email is sent to if his or her address is successfully verified.</p>
    pub success_redirection_url: String,
    /// <p>The content of the custom verification email. The total size of the email must be less than 10 MB. The message body may contain HTML, with some limitations. For more information, see <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/custom-verification-emails.html#custom-verification-emails-faq">Custom Verification Email Frequently Asked Questions</a> in the <i>Amazon SES Developer Guide</i>.</p>
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
            &obj.failure_redirection_url.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "FromEmailAddress"),
            &obj.from_email_address.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "SuccessRedirectionURL"),
            &obj.success_redirection_url.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "TemplateContent"),
            &obj.template_content.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "TemplateName"),
            &obj.template_name.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "TemplateSubject"),
            &obj.template_subject.replace("+", "%2B"),
        );
    }
}

/// <p>Represents a request to create a new IP address filter. You use IP address filters when you receive email with Amazon SES. For more information, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-concepts.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
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
pub struct CreateReceiptFilterResponse {}

struct CreateReceiptFilterResponseDeserializer;
impl CreateReceiptFilterResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateReceiptFilterResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let obj = CreateReceiptFilterResponse::default();

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Represents a request to create a receipt rule. You use receipt rules to receive email with Amazon SES. For more information, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-concepts.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
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
            params.put(
                &format!("{}{}", prefix, "After"),
                &field_value.replace("+", "%2B"),
            );
        }
        ReceiptRuleSerializer::serialize(params, &format!("{}{}", prefix, "Rule"), &obj.rule);
        params.put(
            &format!("{}{}", prefix, "RuleSetName"),
            &obj.rule_set_name.replace("+", "%2B"),
        );
    }
}

/// <p>An empty element returned on a successful request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateReceiptRuleResponse {}

struct CreateReceiptRuleResponseDeserializer;
impl CreateReceiptRuleResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateReceiptRuleResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let obj = CreateReceiptRuleResponse::default();

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Represents a request to create an empty receipt rule set. You use receipt rule sets to receive email with Amazon SES. For more information, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-concepts.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
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

        params.put(
            &format!("{}{}", prefix, "RuleSetName"),
            &obj.rule_set_name.replace("+", "%2B"),
        );
    }
}

/// <p>An empty element returned on a successful request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateReceiptRuleSetResponse {}

struct CreateReceiptRuleSetResponseDeserializer;
impl CreateReceiptRuleSetResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateReceiptRuleSetResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let obj = CreateReceiptRuleSetResponse::default();

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Represents a request to create an email template. For more information, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/send-personalized-email-api.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
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
pub struct CreateTemplateResponse {}

struct CreateTemplateResponseDeserializer;
impl CreateTemplateResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateTemplateResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let obj = CreateTemplateResponse::default();

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct CustomMailFromStatusDeserializer;
impl CustomMailFromStatusDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct CustomRedirectDomainDeserializer;
impl CustomRedirectDomainDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Contains information about a custom verification email template.</p>
#[derive(Default, Debug, Clone, PartialEq)]
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
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CustomVerificationEmailTemplate, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CustomVerificationEmailTemplate::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "FailureRedirectionURL" => {
                        obj.failure_redirection_url =
                            Some(try!(FailureRedirectionURLDeserializer::deserialize(
                                "FailureRedirectionURL",
                                stack
                            )));
                    }
                    "FromEmailAddress" => {
                        obj.from_email_address = Some(try!(FromAddressDeserializer::deserialize(
                            "FromEmailAddress",
                            stack
                        )));
                    }
                    "SuccessRedirectionURL" => {
                        obj.success_redirection_url =
                            Some(try!(SuccessRedirectionURLDeserializer::deserialize(
                                "SuccessRedirectionURL",
                                stack
                            )));
                    }
                    "TemplateName" => {
                        obj.template_name = Some(try!(TemplateNameDeserializer::deserialize(
                            "TemplateName",
                            stack
                        )));
                    }
                    "TemplateSubject" => {
                        obj.template_subject = Some(try!(SubjectDeserializer::deserialize(
                            "TemplateSubject",
                            stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct CustomVerificationEmailTemplatesDeserializer;
impl CustomVerificationEmailTemplatesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<CustomVerificationEmailTemplate>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "member" {
                        obj.push(try!(
                            CustomVerificationEmailTemplateDeserializer::deserialize(
                                "member", stack
                            )
                        ));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}
struct DefaultDimensionValueDeserializer;
impl DefaultDimensionValueDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Represents a request to delete a configuration set event destination. Configuration set event destinations are associated with configuration sets, which enable you to publish email sending events. For information about using configuration sets, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/monitor-sending-activity.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
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
            &obj.configuration_set_name.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "EventDestinationName"),
            &obj.event_destination_name.replace("+", "%2B"),
        );
    }
}

/// <p>An empty element returned on a successful request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteConfigurationSetEventDestinationResponse {}

struct DeleteConfigurationSetEventDestinationResponseDeserializer;
impl DeleteConfigurationSetEventDestinationResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteConfigurationSetEventDestinationResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let obj = DeleteConfigurationSetEventDestinationResponse::default();

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Represents a request to delete a configuration set. Configuration sets enable you to publish email sending events. For information about using configuration sets, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/monitor-sending-activity.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
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
            &obj.configuration_set_name.replace("+", "%2B"),
        );
    }
}

/// <p>An empty element returned on a successful request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteConfigurationSetResponse {}

struct DeleteConfigurationSetResponseDeserializer;
impl DeleteConfigurationSetResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteConfigurationSetResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let obj = DeleteConfigurationSetResponse::default();

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Represents a request to delete open and click tracking options in a configuration set. </p>
#[derive(Default, Debug, Clone, PartialEq)]
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
            &obj.configuration_set_name.replace("+", "%2B"),
        );
    }
}

/// <p>An empty element returned on a successful request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteConfigurationSetTrackingOptionsResponse {}

struct DeleteConfigurationSetTrackingOptionsResponseDeserializer;
impl DeleteConfigurationSetTrackingOptionsResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteConfigurationSetTrackingOptionsResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let obj = DeleteConfigurationSetTrackingOptionsResponse::default();

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Represents a request to delete an existing custom verification email template.</p>
#[derive(Default, Debug, Clone, PartialEq)]
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

        params.put(
            &format!("{}{}", prefix, "TemplateName"),
            &obj.template_name.replace("+", "%2B"),
        );
    }
}

/// <p>Represents a request to delete a sending authorization policy for an identity. Sending authorization is an Amazon SES feature that enables you to authorize other senders to use your identities. For information, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/sending-authorization.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
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

        params.put(
            &format!("{}{}", prefix, "Identity"),
            &obj.identity.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "PolicyName"),
            &obj.policy_name.replace("+", "%2B"),
        );
    }
}

/// <p>An empty element returned on a successful request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteIdentityPolicyResponse {}

struct DeleteIdentityPolicyResponseDeserializer;
impl DeleteIdentityPolicyResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteIdentityPolicyResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let obj = DeleteIdentityPolicyResponse::default();

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Represents a request to delete one of your Amazon SES identities (an email address or domain).</p>
#[derive(Default, Debug, Clone, PartialEq)]
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

        params.put(
            &format!("{}{}", prefix, "Identity"),
            &obj.identity.replace("+", "%2B"),
        );
    }
}

/// <p>An empty element returned on a successful request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteIdentityResponse {}

struct DeleteIdentityResponseDeserializer;
impl DeleteIdentityResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteIdentityResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let obj = DeleteIdentityResponse::default();

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Represents a request to delete an IP address filter. You use IP address filters when you receive email with Amazon SES. For more information, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-concepts.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
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

        params.put(
            &format!("{}{}", prefix, "FilterName"),
            &obj.filter_name.replace("+", "%2B"),
        );
    }
}

/// <p>An empty element returned on a successful request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteReceiptFilterResponse {}

struct DeleteReceiptFilterResponseDeserializer;
impl DeleteReceiptFilterResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteReceiptFilterResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let obj = DeleteReceiptFilterResponse::default();

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Represents a request to delete a receipt rule. You use receipt rules to receive email with Amazon SES. For more information, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-concepts.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
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

        params.put(
            &format!("{}{}", prefix, "RuleName"),
            &obj.rule_name.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "RuleSetName"),
            &obj.rule_set_name.replace("+", "%2B"),
        );
    }
}

/// <p>An empty element returned on a successful request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteReceiptRuleResponse {}

struct DeleteReceiptRuleResponseDeserializer;
impl DeleteReceiptRuleResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteReceiptRuleResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let obj = DeleteReceiptRuleResponse::default();

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Represents a request to delete a receipt rule set and all of the receipt rules it contains. You use receipt rule sets to receive email with Amazon SES. For more information, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-concepts.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
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

        params.put(
            &format!("{}{}", prefix, "RuleSetName"),
            &obj.rule_set_name.replace("+", "%2B"),
        );
    }
}

/// <p>An empty element returned on a successful request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteReceiptRuleSetResponse {}

struct DeleteReceiptRuleSetResponseDeserializer;
impl DeleteReceiptRuleSetResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteReceiptRuleSetResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let obj = DeleteReceiptRuleSetResponse::default();

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Represents a request to delete an email template. For more information, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/send-personalized-email-api.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
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

        params.put(
            &format!("{}{}", prefix, "TemplateName"),
            &obj.template_name.replace("+", "%2B"),
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteTemplateResponse {}

struct DeleteTemplateResponseDeserializer;
impl DeleteTemplateResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteTemplateResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let obj = DeleteTemplateResponse::default();

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Represents a request to delete an email address from the list of email addresses you have attempted to verify under your AWS account.</p>
#[derive(Default, Debug, Clone, PartialEq)]
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

        params.put(
            &format!("{}{}", prefix, "EmailAddress"),
            &obj.email_address.replace("+", "%2B"),
        );
    }
}

/// <p>Represents a request to return the metadata and receipt rules for the receipt rule set that is currently active. You use receipt rule sets to receive email with Amazon SES. For more information, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-concepts.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
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
pub struct DescribeActiveReceiptRuleSetResponse {
    /// <p>The metadata for the currently active receipt rule set. The metadata consists of the rule set name and a timestamp of when the rule set was created.</p>
    pub metadata: Option<ReceiptRuleSetMetadata>,
    /// <p>The receipt rules that belong to the active rule set.</p>
    pub rules: Option<Vec<ReceiptRule>>,
}

struct DescribeActiveReceiptRuleSetResponseDeserializer;
impl DescribeActiveReceiptRuleSetResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeActiveReceiptRuleSetResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DescribeActiveReceiptRuleSetResponse::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Metadata" => {
                        obj.metadata = Some(try!(ReceiptRuleSetMetadataDeserializer::deserialize(
                            "Metadata", stack
                        )));
                    }
                    "Rules" => {
                        obj.rules = Some(try!(ReceiptRulesListDeserializer::deserialize(
                            "Rules", stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Represents a request to return the details of a configuration set. Configuration sets enable you to publish email sending events. For information about using configuration sets, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/monitor-sending-activity.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
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
            &obj.configuration_set_name.replace("+", "%2B"),
        );
    }
}

/// <p>Represents the details of a configuration set. Configuration sets enable you to publish email sending events. For information about using configuration sets, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/monitor-sending-activity.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeConfigurationSetResponse {
    /// <p>The configuration set object associated with the specified configuration set.</p>
    pub configuration_set: Option<ConfigurationSet>,
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
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeConfigurationSetResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DescribeConfigurationSetResponse::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "ConfigurationSet" => {
                        obj.configuration_set = Some(try!(
                            ConfigurationSetDeserializer::deserialize("ConfigurationSet", stack)
                        ));
                    }
                    "EventDestinations" => {
                        obj.event_destinations = Some(try!(
                            EventDestinationsDeserializer::deserialize("EventDestinations", stack)
                        ));
                    }
                    "ReputationOptions" => {
                        obj.reputation_options = Some(try!(
                            ReputationOptionsDeserializer::deserialize("ReputationOptions", stack)
                        ));
                    }
                    "TrackingOptions" => {
                        obj.tracking_options = Some(try!(
                            TrackingOptionsDeserializer::deserialize("TrackingOptions", stack)
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Represents a request to return the details of a receipt rule. You use receipt rules to receive email with Amazon SES. For more information, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-concepts.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
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

        params.put(
            &format!("{}{}", prefix, "RuleName"),
            &obj.rule_name.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "RuleSetName"),
            &obj.rule_set_name.replace("+", "%2B"),
        );
    }
}

/// <p>Represents the details of a receipt rule.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeReceiptRuleResponse {
    /// <p>A data structure that contains the specified receipt rule's name, actions, recipients, domains, enabled status, scan status, and Transport Layer Security (TLS) policy.</p>
    pub rule: Option<ReceiptRule>,
}

struct DescribeReceiptRuleResponseDeserializer;
impl DescribeReceiptRuleResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeReceiptRuleResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DescribeReceiptRuleResponse::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Rule" => {
                        obj.rule = Some(try!(ReceiptRuleDeserializer::deserialize("Rule", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Represents a request to return the details of a receipt rule set. You use receipt rule sets to receive email with Amazon SES. For more information, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-concepts.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
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

        params.put(
            &format!("{}{}", prefix, "RuleSetName"),
            &obj.rule_set_name.replace("+", "%2B"),
        );
    }
}

/// <p>Represents the details of the specified receipt rule set.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeReceiptRuleSetResponse {
    /// <p>The metadata for the receipt rule set, which consists of the rule set name and the timestamp of when the rule set was created.</p>
    pub metadata: Option<ReceiptRuleSetMetadata>,
    /// <p>A list of the receipt rules that belong to the specified receipt rule set.</p>
    pub rules: Option<Vec<ReceiptRule>>,
}

struct DescribeReceiptRuleSetResponseDeserializer;
impl DescribeReceiptRuleSetResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeReceiptRuleSetResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DescribeReceiptRuleSetResponse::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Metadata" => {
                        obj.metadata = Some(try!(ReceiptRuleSetMetadataDeserializer::deserialize(
                            "Metadata", stack
                        )));
                    }
                    "Rules" => {
                        obj.rules = Some(try!(ReceiptRulesListDeserializer::deserialize(
                            "Rules", stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p><p>Represents the destination of the message, consisting of To:, CC:, and BCC: fields.</p> <note> <p>Amazon SES does not support the SMTPUTF8 extension, as described in <a href="https://tools.ietf.org/html/rfc6531">RFC6531</a>. For this reason, the <i>local part</i> of a destination email address (the part of the email address that precedes the @ sign) may only contain <a href="https://en.wikipedia.org/wiki/Email_address#Local-part">7-bit ASCII characters</a>. If the <i>domain part</i> of an address (the part after the @ sign) contains non-ASCII characters, they must be encoded using Punycode, as described in <a href="https://tools.ietf.org/html/rfc3492.html">RFC3492</a>.</p> </note></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Destination {
    /// <p>The BCC: field(s) of the message.</p>
    pub bcc_addresses: Option<Vec<String>>,
    /// <p>The CC: field(s) of the message.</p>
    pub cc_addresses: Option<Vec<String>>,
    /// <p>The To: field(s) of the message.</p>
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
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct DimensionValueSourceDeserializer;
impl DimensionValueSourceDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct DkimAttributesDeserializer;
impl DkimAttributesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<::std::collections::HashMap<String, IdentityDkimAttributes>, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ::std::collections::HashMap::new();

        while try!(peek_at_name(stack)) == "entry" {
            try!(start_element("entry", stack));
            let key = try!(IdentityDeserializer::deserialize("key", stack));
            let value = try!(IdentityDkimAttributesDeserializer::deserialize(
                "value", stack
            ));
            obj.insert(key, value);
            try!(end_element("entry", stack));
        }

        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
struct EnabledDeserializer;
impl EnabledDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<bool, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = bool::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct SesErrorDeserializer;
impl SesErrorDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Contains information about the event destination that the specified email sending events will be published to.</p> <note> <p>When you create or update an event destination, you must provide one, and only one, destination. The destination can be Amazon CloudWatch, Amazon Kinesis Firehose or Amazon Simple Notification Service (Amazon SNS).</p> </note> <p>Event destinations are associated with configuration sets, which enable you to publish email sending events to Amazon CloudWatch, Amazon Kinesis Firehose, or Amazon Simple Notification Service (Amazon SNS). For information about using configuration sets, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/monitor-sending-activity.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
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
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<EventDestination, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = EventDestination::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "CloudWatchDestination" => {
                        obj.cloud_watch_destination =
                            Some(try!(CloudWatchDestinationDeserializer::deserialize(
                                "CloudWatchDestination",
                                stack
                            )));
                    }
                    "Enabled" => {
                        obj.enabled =
                            Some(try!(EnabledDeserializer::deserialize("Enabled", stack)));
                    }
                    "KinesisFirehoseDestination" => {
                        obj.kinesis_firehose_destination =
                            Some(try!(KinesisFirehoseDestinationDeserializer::deserialize(
                                "KinesisFirehoseDestination",
                                stack
                            )));
                    }
                    "MatchingEventTypes" => {
                        obj.matching_event_types = try!(EventTypesDeserializer::deserialize(
                            "MatchingEventTypes",
                            stack
                        ));
                    }
                    "Name" => {
                        obj.name =
                            try!(EventDestinationNameDeserializer::deserialize("Name", stack));
                    }
                    "SNSDestination" => {
                        obj.sns_destination = Some(try!(SNSDestinationDeserializer::deserialize(
                            "SNSDestination",
                            stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
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
            params.put(
                &format!("{}{}", prefix, "Enabled"),
                &field_value.to_string().replace("+", "%2B"),
            );
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
        params.put(
            &format!("{}{}", prefix, "Name"),
            &obj.name.replace("+", "%2B"),
        );
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
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct EventDestinationsDeserializer;
impl EventDestinationsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<EventDestination>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "member" {
                        obj.push(try!(EventDestinationDeserializer::deserialize(
                            "member", stack
                        )));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}
struct EventTypeDeserializer;
impl EventTypeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct EventTypesDeserializer;
impl EventTypesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "member" {
                        obj.push(try!(EventTypeDeserializer::deserialize("member", stack)));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
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

/// <p>Additional X-headers to include in the Delivery Status Notification (DSN) when an email that Amazon SES receives on your behalf bounces.</p> <p>For information about receiving email through Amazon SES, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
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

        params.put(
            &format!("{}{}", prefix, "Name"),
            &obj.name.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "Value"),
            &obj.value.replace("+", "%2B"),
        );
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
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct FromAddressDeserializer;
impl FromAddressDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Represents a request to return the email sending status for your Amazon SES account.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetAccountSendingEnabledResponse {
    /// <p>Describes whether email sending is enabled or disabled for your Amazon SES account.</p>
    pub enabled: Option<bool>,
}

struct GetAccountSendingEnabledResponseDeserializer;
impl GetAccountSendingEnabledResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetAccountSendingEnabledResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = GetAccountSendingEnabledResponse::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Enabled" => {
                        obj.enabled =
                            Some(try!(EnabledDeserializer::deserialize("Enabled", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Represents a request to retrieve an existing custom verification email template.</p>
#[derive(Default, Debug, Clone, PartialEq)]
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

        params.put(
            &format!("{}{}", prefix, "TemplateName"),
            &obj.template_name.replace("+", "%2B"),
        );
    }
}

/// <p>The content of the custom verification email template.</p>
#[derive(Default, Debug, Clone, PartialEq)]
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
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetCustomVerificationEmailTemplateResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = GetCustomVerificationEmailTemplateResponse::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "FailureRedirectionURL" => {
                        obj.failure_redirection_url =
                            Some(try!(FailureRedirectionURLDeserializer::deserialize(
                                "FailureRedirectionURL",
                                stack
                            )));
                    }
                    "FromEmailAddress" => {
                        obj.from_email_address = Some(try!(FromAddressDeserializer::deserialize(
                            "FromEmailAddress",
                            stack
                        )));
                    }
                    "SuccessRedirectionURL" => {
                        obj.success_redirection_url =
                            Some(try!(SuccessRedirectionURLDeserializer::deserialize(
                                "SuccessRedirectionURL",
                                stack
                            )));
                    }
                    "TemplateContent" => {
                        obj.template_content = Some(try!(
                            TemplateContentDeserializer::deserialize("TemplateContent", stack)
                        ));
                    }
                    "TemplateName" => {
                        obj.template_name = Some(try!(TemplateNameDeserializer::deserialize(
                            "TemplateName",
                            stack
                        )));
                    }
                    "TemplateSubject" => {
                        obj.template_subject = Some(try!(SubjectDeserializer::deserialize(
                            "TemplateSubject",
                            stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Represents a request for the status of Amazon SES Easy DKIM signing for an identity. For domain identities, this request also returns the DKIM tokens that are required for Easy DKIM signing, and whether Amazon SES successfully verified that these tokens were published. For more information about Easy DKIM, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/easy-dkim.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
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
pub struct GetIdentityDkimAttributesResponse {
    /// <p>The DKIM attributes for an email address or a domain.</p>
    pub dkim_attributes: ::std::collections::HashMap<String, IdentityDkimAttributes>,
}

struct GetIdentityDkimAttributesResponseDeserializer;
impl GetIdentityDkimAttributesResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetIdentityDkimAttributesResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = GetIdentityDkimAttributesResponse::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "DkimAttributes" => {
                        obj.dkim_attributes = try!(DkimAttributesDeserializer::deserialize(
                            "DkimAttributes",
                            stack
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Represents a request to return the Amazon SES custom MAIL FROM attributes for a list of identities. For information about using a custom MAIL FROM domain, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/mail-from.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
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
pub struct GetIdentityMailFromDomainAttributesResponse {
    /// <p>A map of identities to custom MAIL FROM attributes.</p>
    pub mail_from_domain_attributes:
        ::std::collections::HashMap<String, IdentityMailFromDomainAttributes>,
}

struct GetIdentityMailFromDomainAttributesResponseDeserializer;
impl GetIdentityMailFromDomainAttributesResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetIdentityMailFromDomainAttributesResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = GetIdentityMailFromDomainAttributesResponse::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "MailFromDomainAttributes" => {
                        obj.mail_from_domain_attributes =
                            try!(MailFromDomainAttributesDeserializer::deserialize(
                                "MailFromDomainAttributes",
                                stack
                            ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Represents a request to return the notification attributes for a list of identities you verified with Amazon SES. For information about Amazon SES notifications, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/notifications.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
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
pub struct GetIdentityNotificationAttributesResponse {
    /// <p>A map of Identity to IdentityNotificationAttributes.</p>
    pub notification_attributes:
        ::std::collections::HashMap<String, IdentityNotificationAttributes>,
}

struct GetIdentityNotificationAttributesResponseDeserializer;
impl GetIdentityNotificationAttributesResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetIdentityNotificationAttributesResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = GetIdentityNotificationAttributesResponse::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "NotificationAttributes" => {
                        obj.notification_attributes =
                            try!(NotificationAttributesDeserializer::deserialize(
                                "NotificationAttributes",
                                stack
                            ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Represents a request to return the requested sending authorization policies for an identity. Sending authorization is an Amazon SES feature that enables you to authorize other senders to use your identities. For information, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/sending-authorization.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
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

        params.put(
            &format!("{}{}", prefix, "Identity"),
            &obj.identity.replace("+", "%2B"),
        );
        PolicyNameListSerializer::serialize(
            params,
            &format!("{}{}", prefix, "PolicyNames"),
            &obj.policy_names,
        );
    }
}

/// <p>Represents the requested sending authorization policies.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetIdentityPoliciesResponse {
    /// <p>A map of policy names to policies.</p>
    pub policies: ::std::collections::HashMap<String, String>,
}

struct GetIdentityPoliciesResponseDeserializer;
impl GetIdentityPoliciesResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetIdentityPoliciesResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = GetIdentityPoliciesResponse::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Policies" => {
                        obj.policies = try!(PolicyMapDeserializer::deserialize("Policies", stack));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Represents a request to return the Amazon SES verification status of a list of identities. For domain identities, this request also returns the verification token. For information about verifying identities with Amazon SES, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/verify-addresses-and-domains.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
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
pub struct GetIdentityVerificationAttributesResponse {
    /// <p>A map of Identities to IdentityVerificationAttributes objects.</p>
    pub verification_attributes:
        ::std::collections::HashMap<String, IdentityVerificationAttributes>,
}

struct GetIdentityVerificationAttributesResponseDeserializer;
impl GetIdentityVerificationAttributesResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetIdentityVerificationAttributesResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = GetIdentityVerificationAttributesResponse::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "VerificationAttributes" => {
                        obj.verification_attributes =
                            try!(VerificationAttributesDeserializer::deserialize(
                                "VerificationAttributes",
                                stack
                            ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Represents your Amazon SES daily sending quota, maximum send rate, and the number of emails you have sent in the last 24 hours.</p>
#[derive(Default, Debug, Clone, PartialEq)]
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
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetSendQuotaResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = GetSendQuotaResponse::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Max24HourSend" => {
                        obj.max_24_hour_send = Some(try!(Max24HourSendDeserializer::deserialize(
                            "Max24HourSend",
                            stack
                        )));
                    }
                    "MaxSendRate" => {
                        obj.max_send_rate = Some(try!(MaxSendRateDeserializer::deserialize(
                            "MaxSendRate",
                            stack
                        )));
                    }
                    "SentLast24Hours" => {
                        obj.sent_last_24_hours = Some(try!(
                            SentLast24HoursDeserializer::deserialize("SentLast24Hours", stack)
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Represents a list of data points. This list contains aggregated data from the previous two weeks of your sending activity with Amazon SES.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetSendStatisticsResponse {
    /// <p>A list of data points, each of which represents 15 minutes of activity.</p>
    pub send_data_points: Option<Vec<SendDataPoint>>,
}

struct GetSendStatisticsResponseDeserializer;
impl GetSendStatisticsResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetSendStatisticsResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = GetSendStatisticsResponse::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "SendDataPoints" => {
                        obj.send_data_points = Some(try!(
                            SendDataPointListDeserializer::deserialize("SendDataPoints", stack)
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
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

        params.put(
            &format!("{}{}", prefix, "TemplateName"),
            &obj.template_name.replace("+", "%2B"),
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetTemplateResponse {
    pub template: Option<Template>,
}

struct GetTemplateResponseDeserializer;
impl GetTemplateResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetTemplateResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = GetTemplateResponse::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Template" => {
                        obj.template =
                            Some(try!(TemplateDeserializer::deserialize("Template", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct HeaderNameDeserializer;
impl HeaderNameDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct HeaderValueDeserializer;
impl HeaderValueDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct HtmlPartDeserializer;
impl HtmlPartDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct IdentityDeserializer;
impl IdentityDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Represents the DKIM attributes of a verified email address or a domain.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct IdentityDkimAttributes {
    /// <p>True if DKIM signing is enabled for email sent from the identity; false otherwise. The default value is true.</p>
    pub dkim_enabled: bool,
    /// <p>A set of character strings that represent the domain's identity. Using these tokens, you will need to create DNS CNAME records that point to DKIM public keys hosted by Amazon SES. Amazon Web Services will eventually detect that you have updated your DNS records; this detection process may take up to 72 hours. Upon successful detection, Amazon SES will be able to DKIM-sign email originating from that domain. (This only applies to domain identities, not email address identities.)</p> <p>For more information about creating DNS records using DKIM tokens, go to the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/easy-dkim-dns-records.html">Amazon SES Developer Guide</a>.</p>
    pub dkim_tokens: Option<Vec<String>>,
    /// <p>Describes whether Amazon SES has successfully verified the DKIM DNS records (tokens) published in the domain name's DNS. (This only applies to domain identities, not email address identities.)</p>
    pub dkim_verification_status: String,
}

struct IdentityDkimAttributesDeserializer;
impl IdentityDkimAttributesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<IdentityDkimAttributes, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = IdentityDkimAttributes::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "DkimEnabled" => {
                        obj.dkim_enabled =
                            try!(EnabledDeserializer::deserialize("DkimEnabled", stack));
                    }
                    "DkimTokens" => {
                        obj.dkim_tokens = Some(try!(
                            VerificationTokenListDeserializer::deserialize("DkimTokens", stack)
                        ));
                    }
                    "DkimVerificationStatus" => {
                        obj.dkim_verification_status =
                            try!(VerificationStatusDeserializer::deserialize(
                                "DkimVerificationStatus",
                                stack
                            ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct IdentityListDeserializer;
impl IdentityListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "member" {
                        obj.push(try!(IdentityDeserializer::deserialize("member", stack)));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
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
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<IdentityMailFromDomainAttributes, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = IdentityMailFromDomainAttributes::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "BehaviorOnMXFailure" => {
                        obj.behavior_on_mx_failure =
                            try!(BehaviorOnMXFailureDeserializer::deserialize(
                                "BehaviorOnMXFailure",
                                stack
                            ));
                    }
                    "MailFromDomain" => {
                        obj.mail_from_domain = try!(MailFromDomainNameDeserializer::deserialize(
                            "MailFromDomain",
                            stack
                        ));
                    }
                    "MailFromDomainStatus" => {
                        obj.mail_from_domain_status =
                            try!(CustomMailFromStatusDeserializer::deserialize(
                                "MailFromDomainStatus",
                                stack
                            ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Represents the notification attributes of an identity, including whether an identity has Amazon Simple Notification Service (Amazon SNS) topics set for bounce, complaint, and/or delivery notifications, and whether feedback forwarding is enabled for bounce and complaint notifications.</p>
#[derive(Default, Debug, Clone, PartialEq)]
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
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<IdentityNotificationAttributes, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = IdentityNotificationAttributes::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "BounceTopic" => {
                        obj.bounce_topic = try!(NotificationTopicDeserializer::deserialize(
                            "BounceTopic",
                            stack
                        ));
                    }
                    "ComplaintTopic" => {
                        obj.complaint_topic = try!(NotificationTopicDeserializer::deserialize(
                            "ComplaintTopic",
                            stack
                        ));
                    }
                    "DeliveryTopic" => {
                        obj.delivery_topic = try!(NotificationTopicDeserializer::deserialize(
                            "DeliveryTopic",
                            stack
                        ));
                    }
                    "ForwardingEnabled" => {
                        obj.forwarding_enabled =
                            try!(EnabledDeserializer::deserialize("ForwardingEnabled", stack));
                    }
                    "HeadersInBounceNotificationsEnabled" => {
                        obj.headers_in_bounce_notifications_enabled =
                            Some(try!(EnabledDeserializer::deserialize(
                                "HeadersInBounceNotificationsEnabled",
                                stack
                            )));
                    }
                    "HeadersInComplaintNotificationsEnabled" => {
                        obj.headers_in_complaint_notifications_enabled =
                            Some(try!(EnabledDeserializer::deserialize(
                                "HeadersInComplaintNotificationsEnabled",
                                stack
                            )));
                    }
                    "HeadersInDeliveryNotificationsEnabled" => {
                        obj.headers_in_delivery_notifications_enabled =
                            Some(try!(EnabledDeserializer::deserialize(
                                "HeadersInDeliveryNotificationsEnabled",
                                stack
                            )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Represents the verification attributes of a single identity.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct IdentityVerificationAttributes {
    /// <p>The verification status of the identity: "Pending", "Success", "Failed", or "TemporaryFailure".</p>
    pub verification_status: String,
    /// <p>The verification token for a domain identity. Null for email address identities.</p>
    pub verification_token: Option<String>,
}

struct IdentityVerificationAttributesDeserializer;
impl IdentityVerificationAttributesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<IdentityVerificationAttributes, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = IdentityVerificationAttributes::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "VerificationStatus" => {
                        obj.verification_status =
                            try!(VerificationStatusDeserializer::deserialize(
                                "VerificationStatus",
                                stack
                            ));
                    }
                    "VerificationToken" => {
                        obj.verification_token = Some(try!(
                            VerificationTokenDeserializer::deserialize("VerificationToken", stack)
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct InvocationTypeDeserializer;
impl InvocationTypeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Contains the delivery stream ARN and the IAM role ARN associated with an Amazon Kinesis Firehose event destination.</p> <p>Event destinations, such as Amazon Kinesis Firehose, are associated with configuration sets, which enable you to publish email sending events. For information about using configuration sets, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/monitor-sending-activity.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct KinesisFirehoseDestination {
    /// <p>The ARN of the Amazon Kinesis Firehose stream that email sending events should be published to.</p>
    pub delivery_stream_arn: String,
    /// <p>The ARN of the IAM role under which Amazon SES publishes email sending events to the Amazon Kinesis Firehose stream.</p>
    pub iam_role_arn: String,
}

struct KinesisFirehoseDestinationDeserializer;
impl KinesisFirehoseDestinationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<KinesisFirehoseDestination, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = KinesisFirehoseDestination::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "DeliveryStreamARN" => {
                        obj.delivery_stream_arn = try!(
                            AmazonResourceNameDeserializer::deserialize("DeliveryStreamARN", stack)
                        );
                    }
                    "IAMRoleARN" => {
                        obj.iam_role_arn = try!(AmazonResourceNameDeserializer::deserialize(
                            "IAMRoleARN",
                            stack
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
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
            &obj.delivery_stream_arn.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "IAMRoleARN"),
            &obj.iam_role_arn.replace("+", "%2B"),
        );
    }
}

/// <p>When included in a receipt rule, this action calls an AWS Lambda function and, optionally, publishes a notification to Amazon Simple Notification Service (Amazon SNS).</p> <p>To enable Amazon SES to call your AWS Lambda function or to publish to an Amazon SNS topic of another account, Amazon SES must have permission to access those resources. For information about giving permissions, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-permissions.html">Amazon SES Developer Guide</a>.</p> <p>For information about using AWS Lambda actions in receipt rules, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-action-lambda.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct LambdaAction {
    /// <p>The Amazon Resource Name (ARN) of the AWS Lambda function. An example of an AWS Lambda function ARN is <code>arn:aws:lambda:us-west-2:account-id:function:MyFunction</code>. For more information about AWS Lambda, see the <a href="http://docs.aws.amazon.com/lambda/latest/dg/welcome.html">AWS Lambda Developer Guide</a>.</p>
    pub function_arn: String,
    /// <p><p>The invocation type of the AWS Lambda function. An invocation type of <code>RequestResponse</code> means that the execution of the function will immediately result in a response, and a value of <code>Event</code> means that the function will be invoked asynchronously. The default value is <code>Event</code>. For information about AWS Lambda invocation types, see the <a href="http://docs.aws.amazon.com/lambda/latest/dg/API_Invoke.html">AWS Lambda Developer Guide</a>.</p> <important> <p>There is a 30-second timeout on <code>RequestResponse</code> invocations. You should use <code>Event</code> invocation in most cases. Use <code>RequestResponse</code> only when you want to make a mail flow decision, such as whether to stop the receipt rule or the receipt rule set.</p> </important></p>
    pub invocation_type: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the Amazon SNS topic to notify when the Lambda action is taken. An example of an Amazon SNS topic ARN is <code>arn:aws:sns:us-west-2:123456789012:MyTopic</code>. For more information about Amazon SNS topics, see the <a href="http://docs.aws.amazon.com/sns/latest/dg/CreateTopic.html">Amazon SNS Developer Guide</a>.</p>
    pub topic_arn: Option<String>,
}

struct LambdaActionDeserializer;
impl LambdaActionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<LambdaAction, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = LambdaAction::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "FunctionArn" => {
                        obj.function_arn = try!(AmazonResourceNameDeserializer::deserialize(
                            "FunctionArn",
                            stack
                        ));
                    }
                    "InvocationType" => {
                        obj.invocation_type = Some(try!(InvocationTypeDeserializer::deserialize(
                            "InvocationType",
                            stack
                        )));
                    }
                    "TopicArn" => {
                        obj.topic_arn = Some(try!(AmazonResourceNameDeserializer::deserialize(
                            "TopicArn", stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
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

        params.put(
            &format!("{}{}", prefix, "FunctionArn"),
            &obj.function_arn.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.invocation_type {
            params.put(
                &format!("{}{}", prefix, "InvocationType"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.topic_arn {
            params.put(
                &format!("{}{}", prefix, "TopicArn"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

struct LastFreshStartDeserializer;
impl LastFreshStartDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Represents a request to list the configuration sets associated with your AWS account. Configuration sets enable you to publish email sending events. For information about using configuration sets, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/monitor-sending-activity.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
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
            params.put(
                &format!("{}{}", prefix, "MaxItems"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.next_token {
            params.put(
                &format!("{}{}", prefix, "NextToken"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

/// <p>A list of configuration sets associated with your AWS account. Configuration sets enable you to publish email sending events. For information about using configuration sets, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/monitor-sending-activity.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListConfigurationSetsResponse {
    /// <p>A list of configuration sets.</p>
    pub configuration_sets: Option<Vec<ConfigurationSet>>,
    /// <p>A token indicating that there are additional configuration sets available to be listed. Pass this token to successive calls of <code>ListConfigurationSets</code>. </p>
    pub next_token: Option<String>,
}

struct ListConfigurationSetsResponseDeserializer;
impl ListConfigurationSetsResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListConfigurationSetsResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ListConfigurationSetsResponse::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "ConfigurationSets" => {
                        obj.configuration_sets = Some(try!(
                            ConfigurationSetsDeserializer::deserialize("ConfigurationSets", stack)
                        ));
                    }
                    "NextToken" => {
                        obj.next_token =
                            Some(try!(NextTokenDeserializer::deserialize("NextToken", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Represents a request to list the existing custom verification email templates for your account.</p> <p>For more information about custom verification email templates, see <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/custom-verification-emails.html">Using Custom Verification Email Templates</a> in the <i>Amazon SES Developer Guide</i>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
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
            params.put(
                &format!("{}{}", prefix, "MaxResults"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.next_token {
            params.put(
                &format!("{}{}", prefix, "NextToken"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

/// <p>A paginated list of custom verification email templates.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListCustomVerificationEmailTemplatesResponse {
    /// <p>A list of the custom verification email templates that exist in your account.</p>
    pub custom_verification_email_templates: Option<Vec<CustomVerificationEmailTemplate>>,
    /// <p>A token indicating that there are additional custom verification email templates available to be listed. Pass this token to a subsequent call to <code>ListTemplates</code> to retrieve the next 50 custom verification email templates.</p>
    pub next_token: Option<String>,
}

struct ListCustomVerificationEmailTemplatesResponseDeserializer;
impl ListCustomVerificationEmailTemplatesResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListCustomVerificationEmailTemplatesResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ListCustomVerificationEmailTemplatesResponse::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "CustomVerificationEmailTemplates" => {
                        obj.custom_verification_email_templates = Some(try!(
                            CustomVerificationEmailTemplatesDeserializer::deserialize(
                                "CustomVerificationEmailTemplates",
                                stack
                            )
                        ));
                    }
                    "NextToken" => {
                        obj.next_token =
                            Some(try!(NextTokenDeserializer::deserialize("NextToken", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Represents a request to return a list of all identities (email addresses and domains) that you have attempted to verify under your AWS account, regardless of verification status.</p>
#[derive(Default, Debug, Clone, PartialEq)]
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
            params.put(
                &format!("{}{}", prefix, "IdentityType"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.max_items {
            params.put(
                &format!("{}{}", prefix, "MaxItems"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.next_token {
            params.put(
                &format!("{}{}", prefix, "NextToken"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

/// <p>A list of all identities that you have attempted to verify under your AWS account, regardless of verification status.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListIdentitiesResponse {
    /// <p>A list of identities.</p>
    pub identities: Vec<String>,
    /// <p>The token used for pagination.</p>
    pub next_token: Option<String>,
}

struct ListIdentitiesResponseDeserializer;
impl ListIdentitiesResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListIdentitiesResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ListIdentitiesResponse::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Identities" => {
                        obj.identities =
                            try!(IdentityListDeserializer::deserialize("Identities", stack));
                    }
                    "NextToken" => {
                        obj.next_token =
                            Some(try!(NextTokenDeserializer::deserialize("NextToken", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Represents a request to return a list of sending authorization policies that are attached to an identity. Sending authorization is an Amazon SES feature that enables you to authorize other senders to use your identities. For information, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/sending-authorization.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
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

        params.put(
            &format!("{}{}", prefix, "Identity"),
            &obj.identity.replace("+", "%2B"),
        );
    }
}

/// <p>A list of names of sending authorization policies that apply to an identity.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListIdentityPoliciesResponse {
    /// <p>A list of names of policies that apply to the specified identity.</p>
    pub policy_names: Vec<String>,
}

struct ListIdentityPoliciesResponseDeserializer;
impl ListIdentityPoliciesResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListIdentityPoliciesResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ListIdentityPoliciesResponse::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "PolicyNames" => {
                        obj.policy_names = try!(PolicyNameListDeserializer::deserialize(
                            "PolicyNames",
                            stack
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Represents a request to list the IP address filters that exist under your AWS account. You use IP address filters when you receive email with Amazon SES. For more information, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-concepts.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
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
pub struct ListReceiptFiltersResponse {
    /// <p>A list of IP address filter data structures, which each consist of a name, an IP address range, and whether to allow or block mail from it.</p>
    pub filters: Option<Vec<ReceiptFilter>>,
}

struct ListReceiptFiltersResponseDeserializer;
impl ListReceiptFiltersResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListReceiptFiltersResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ListReceiptFiltersResponse::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Filters" => {
                        obj.filters = Some(try!(ReceiptFilterListDeserializer::deserialize(
                            "Filters", stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Represents a request to list the receipt rule sets that exist under your AWS account. You use receipt rule sets to receive email with Amazon SES. For more information, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-concepts.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
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
            params.put(
                &format!("{}{}", prefix, "NextToken"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

/// <p>A list of receipt rule sets that exist under your AWS account.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListReceiptRuleSetsResponse {
    /// <p>A token indicating that there are additional receipt rule sets available to be listed. Pass this token to successive calls of <code>ListReceiptRuleSets</code> to retrieve up to 100 receipt rule sets at a time.</p>
    pub next_token: Option<String>,
    /// <p>The metadata for the currently active receipt rule set. The metadata consists of the rule set name and the timestamp of when the rule set was created.</p>
    pub rule_sets: Option<Vec<ReceiptRuleSetMetadata>>,
}

struct ListReceiptRuleSetsResponseDeserializer;
impl ListReceiptRuleSetsResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListReceiptRuleSetsResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ListReceiptRuleSetsResponse::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "NextToken" => {
                        obj.next_token =
                            Some(try!(NextTokenDeserializer::deserialize("NextToken", stack)));
                    }
                    "RuleSets" => {
                        obj.rule_sets = Some(try!(ReceiptRuleSetsListsDeserializer::deserialize(
                            "RuleSets", stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
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
            params.put(
                &format!("{}{}", prefix, "MaxItems"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.next_token {
            params.put(
                &format!("{}{}", prefix, "NextToken"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListTemplatesResponse {
    /// <p>A token indicating that there are additional email templates available to be listed. Pass this token to a subsequent call to <code>ListTemplates</code> to retrieve the next 50 email templates.</p>
    pub next_token: Option<String>,
    /// <p>An array the contains the name and creation time stamp for each template in your Amazon SES account.</p>
    pub templates_metadata: Option<Vec<TemplateMetadata>>,
}

struct ListTemplatesResponseDeserializer;
impl ListTemplatesResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListTemplatesResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ListTemplatesResponse::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "NextToken" => {
                        obj.next_token =
                            Some(try!(NextTokenDeserializer::deserialize("NextToken", stack)));
                    }
                    "TemplatesMetadata" => {
                        obj.templates_metadata =
                            Some(try!(TemplateMetadataListDeserializer::deserialize(
                                "TemplatesMetadata",
                                stack
                            )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>A list of email addresses that you have verified with Amazon SES under your AWS account.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListVerifiedEmailAddressesResponse {
    /// <p>A list of email addresses that have been verified.</p>
    pub verified_email_addresses: Option<Vec<String>>,
}

struct ListVerifiedEmailAddressesResponseDeserializer;
impl ListVerifiedEmailAddressesResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListVerifiedEmailAddressesResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ListVerifiedEmailAddressesResponse::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "VerifiedEmailAddresses" => {
                        obj.verified_email_addresses = Some(try!(
                            AddressListDeserializer::deserialize("VerifiedEmailAddresses", stack)
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct MailFromDomainAttributesDeserializer;
impl MailFromDomainAttributesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<::std::collections::HashMap<String, IdentityMailFromDomainAttributes>, XmlParseError>
    {
        try!(start_element(tag_name, stack));

        let mut obj = ::std::collections::HashMap::new();

        while try!(peek_at_name(stack)) == "entry" {
            try!(start_element("entry", stack));
            let key = try!(IdentityDeserializer::deserialize("key", stack));
            let value = try!(IdentityMailFromDomainAttributesDeserializer::deserialize(
                "value", stack
            ));
            obj.insert(key, value);
            try!(end_element("entry", stack));
        }

        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
struct MailFromDomainNameDeserializer;
impl MailFromDomainNameDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct Max24HourSendDeserializer;
impl Max24HourSendDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<f64, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = f64::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct MaxSendRateDeserializer;
impl MaxSendRateDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<f64, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = f64::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Represents the message to be sent, composed of a subject and a body.</p>
#[derive(Default, Debug, Clone, PartialEq)]
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

/// <p>Message-related information to include in the Delivery Status Notification (DSN) when an email that Amazon SES receives on your behalf bounces.</p> <p>For information about receiving email through Amazon SES, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
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
            params.put(
                &format!("{}{}", prefix, "ArrivalDate"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.extension_fields {
            ExtensionFieldListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "ExtensionFields"),
                field_value,
            );
        }
        params.put(
            &format!("{}{}", prefix, "ReportingMta"),
            &obj.reporting_mta.replace("+", "%2B"),
        );
    }
}

struct MessageIdDeserializer;
impl MessageIdDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Contains the name and value of a tag that you can provide to <code>SendEmail</code> or <code>SendRawEmail</code> to apply to an email.</p> <p>Message tags, which you use with configuration sets, enable you to publish email sending events. For information about using configuration sets, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/monitor-sending-activity.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
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

        params.put(
            &format!("{}{}", prefix, "Name"),
            &obj.name.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "Value"),
            &obj.value.replace("+", "%2B"),
        );
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
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct NotificationAttributesDeserializer;
impl NotificationAttributesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<::std::collections::HashMap<String, IdentityNotificationAttributes>, XmlParseError>
    {
        try!(start_element(tag_name, stack));

        let mut obj = ::std::collections::HashMap::new();

        while try!(peek_at_name(stack)) == "entry" {
            try!(start_element("entry", stack));
            let key = try!(IdentityDeserializer::deserialize("key", stack));
            let value = try!(IdentityNotificationAttributesDeserializer::deserialize(
                "value", stack
            ));
            obj.insert(key, value);
            try!(end_element("entry", stack));
        }

        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
struct NotificationTopicDeserializer;
impl NotificationTopicDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct PolicyDeserializer;
impl PolicyDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct PolicyMapDeserializer;
impl PolicyMapDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<::std::collections::HashMap<String, String>, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ::std::collections::HashMap::new();

        while try!(peek_at_name(stack)) == "entry" {
            try!(start_element("entry", stack));
            let key = try!(PolicyNameDeserializer::deserialize("key", stack));
            let value = try!(PolicyDeserializer::deserialize("value", stack));
            obj.insert(key, value);
            try!(end_element("entry", stack));
        }

        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
struct PolicyNameDeserializer;
impl PolicyNameDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct PolicyNameListDeserializer;
impl PolicyNameListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "member" {
                        obj.push(try!(PolicyNameDeserializer::deserialize("member", stack)));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
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

/// <p>Represents a request to add or update a sending authorization policy for an identity. Sending authorization is an Amazon SES feature that enables you to authorize other senders to use your identities. For information, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/sending-authorization.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct PutIdentityPolicyRequest {
    /// <p>The identity that the policy will apply to. You can specify an identity by using its name or by using its Amazon Resource Name (ARN). Examples: <code>user@example.com</code>, <code>example.com</code>, <code>arn:aws:ses:us-east-1:123456789012:identity/example.com</code>.</p> <p>To successfully call this API, you must own the identity.</p>
    pub identity: String,
    /// <p>The text of the policy in JSON format. The policy cannot exceed 4 KB.</p> <p>For information about the syntax of sending authorization policies, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/sending-authorization-policies.html">Amazon SES Developer Guide</a>. </p>
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

        params.put(
            &format!("{}{}", prefix, "Identity"),
            &obj.identity.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "Policy"),
            &obj.policy.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "PolicyName"),
            &obj.policy_name.replace("+", "%2B"),
        );
    }
}

/// <p>An empty element returned on a successful request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct PutIdentityPolicyResponse {}

struct PutIdentityPolicyResponseDeserializer;
impl PutIdentityPolicyResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PutIdentityPolicyResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let obj = PutIdentityPolicyResponse::default();

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Represents the raw data of the message.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct RawMessage {
    /// <p>The raw data of the message. This data needs to base64-encoded if you are accessing Amazon SES directly through the HTTPS interface. If you are accessing Amazon SES using an AWS SDK, the SDK takes care of the base 64-encoding for you. In all cases, the client must ensure that the message format complies with Internet email standards regarding email header fields, MIME types, and MIME encoding.</p> <p>The To:, CC:, and BCC: headers in the raw message can contain a group list.</p> <p>If you are using <code>SendRawEmail</code> with sending authorization, you can include X-headers in the raw message to specify the "Source," "From," and "Return-Path" addresses. For more information, see the documentation for <code>SendRawEmail</code>. </p> <important> <p>Do not include these X-headers in the DKIM signature, because they are removed by Amazon SES before sending the email.</p> </important> <p>For more information, go to the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/send-email-raw.html">Amazon SES Developer Guide</a>.</p>
    pub data: Vec<u8>,
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
            ::std::str::from_utf8(&obj.data)
                .unwrap()
                .replace("+", "%2B"),
        );
    }
}

/// <p>An action that Amazon SES can take when it receives an email on behalf of one or more email addresses or domains that you own. An instance of this data type can represent only one action.</p> <p>For information about setting up receipt rules, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-receipt-rules.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
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
    /// <p>Calls Amazon WorkMail and, optionally, publishes a notification to Amazon SNS.</p>
    pub workmail_action: Option<WorkmailAction>,
}

struct ReceiptActionDeserializer;
impl ReceiptActionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ReceiptAction, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ReceiptAction::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "AddHeaderAction" => {
                        obj.add_header_action = Some(try!(
                            AddHeaderActionDeserializer::deserialize("AddHeaderAction", stack)
                        ));
                    }
                    "BounceAction" => {
                        obj.bounce_action = Some(try!(BounceActionDeserializer::deserialize(
                            "BounceAction",
                            stack
                        )));
                    }
                    "LambdaAction" => {
                        obj.lambda_action = Some(try!(LambdaActionDeserializer::deserialize(
                            "LambdaAction",
                            stack
                        )));
                    }
                    "S3Action" => {
                        obj.s3_action =
                            Some(try!(S3ActionDeserializer::deserialize("S3Action", stack)));
                    }
                    "SNSAction" => {
                        obj.sns_action =
                            Some(try!(SNSActionDeserializer::deserialize("SNSAction", stack)));
                    }
                    "StopAction" => {
                        obj.stop_action = Some(try!(StopActionDeserializer::deserialize(
                            "StopAction",
                            stack
                        )));
                    }
                    "WorkmailAction" => {
                        obj.workmail_action = Some(try!(WorkmailActionDeserializer::deserialize(
                            "WorkmailAction",
                            stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
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
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<ReceiptAction>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "member" {
                        obj.push(try!(ReceiptActionDeserializer::deserialize(
                            "member", stack
                        )));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
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

/// <p>A receipt IP address filter enables you to specify whether to accept or reject mail originating from an IP address or range of IP addresses.</p> <p>For information about setting up IP address filters, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-ip-filters.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ReceiptFilter {
    /// <p>A structure that provides the IP addresses to block or allow, and whether to block or allow incoming mail from them.</p>
    pub ip_filter: ReceiptIpFilter,
    /// <p><p>The name of the IP address filter. The name must:</p> <ul> <li> <p>This value can only contain ASCII letters (a-z, A-Z), numbers (0-9), underscores (_), or dashes (-).</p> </li> <li> <p>Start and end with a letter or number.</p> </li> <li> <p>Contain less than 64 characters.</p> </li> </ul></p>
    pub name: String,
}

struct ReceiptFilterDeserializer;
impl ReceiptFilterDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ReceiptFilter, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ReceiptFilter::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "IpFilter" => {
                        obj.ip_filter =
                            try!(ReceiptIpFilterDeserializer::deserialize("IpFilter", stack));
                    }
                    "Name" => {
                        obj.name = try!(ReceiptFilterNameDeserializer::deserialize("Name", stack));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
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
        params.put(
            &format!("{}{}", prefix, "Name"),
            &obj.name.replace("+", "%2B"),
        );
    }
}

struct ReceiptFilterListDeserializer;
impl ReceiptFilterListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<ReceiptFilter>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "member" {
                        obj.push(try!(ReceiptFilterDeserializer::deserialize(
                            "member", stack
                        )));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}
struct ReceiptFilterNameDeserializer;
impl ReceiptFilterNameDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct ReceiptFilterPolicyDeserializer;
impl ReceiptFilterPolicyDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>A receipt IP address filter enables you to specify whether to accept or reject mail originating from an IP address or range of IP addresses.</p> <p>For information about setting up IP address filters, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-ip-filters.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ReceiptIpFilter {
    /// <p>A single IP address or a range of IP addresses that you want to block or allow, specified in Classless Inter-Domain Routing (CIDR) notation. An example of a single email address is 10.0.0.1. An example of a range of IP addresses is 10.0.0.1/24. For more information about CIDR notation, see <a href="https://tools.ietf.org/html/rfc2317">RFC 2317</a>.</p>
    pub cidr: String,
    /// <p>Indicates whether to block or allow incoming mail from the specified IP addresses.</p>
    pub policy: String,
}

struct ReceiptIpFilterDeserializer;
impl ReceiptIpFilterDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ReceiptIpFilter, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ReceiptIpFilter::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Cidr" => {
                        obj.cidr = try!(CidrDeserializer::deserialize("Cidr", stack));
                    }
                    "Policy" => {
                        obj.policy = try!(ReceiptFilterPolicyDeserializer::deserialize(
                            "Policy", stack
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
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

        params.put(
            &format!("{}{}", prefix, "Cidr"),
            &obj.cidr.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "Policy"),
            &obj.policy.replace("+", "%2B"),
        );
    }
}

/// <p>Receipt rules enable you to specify which actions Amazon SES should take when it receives mail on behalf of one or more email addresses or domains that you own.</p> <p>Each receipt rule defines a set of email addresses or domains that it applies to. If the email addresses or domains match at least one recipient address of the message, Amazon SES executes all of the receipt rule's actions on the message.</p> <p>For information about setting up receipt rules, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-receipt-rules.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
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
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ReceiptRule, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ReceiptRule::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Actions" => {
                        obj.actions = Some(try!(ReceiptActionsListDeserializer::deserialize(
                            "Actions", stack
                        )));
                    }
                    "Enabled" => {
                        obj.enabled =
                            Some(try!(EnabledDeserializer::deserialize("Enabled", stack)));
                    }
                    "Name" => {
                        obj.name = try!(ReceiptRuleNameDeserializer::deserialize("Name", stack));
                    }
                    "Recipients" => {
                        obj.recipients = Some(try!(RecipientsListDeserializer::deserialize(
                            "Recipients",
                            stack
                        )));
                    }
                    "ScanEnabled" => {
                        obj.scan_enabled =
                            Some(try!(EnabledDeserializer::deserialize("ScanEnabled", stack)));
                    }
                    "TlsPolicy" => {
                        obj.tls_policy =
                            Some(try!(TlsPolicyDeserializer::deserialize("TlsPolicy", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
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
            params.put(
                &format!("{}{}", prefix, "Enabled"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        params.put(
            &format!("{}{}", prefix, "Name"),
            &obj.name.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.recipients {
            RecipientsListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Recipients"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.scan_enabled {
            params.put(
                &format!("{}{}", prefix, "ScanEnabled"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.tls_policy {
            params.put(
                &format!("{}{}", prefix, "TlsPolicy"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

struct ReceiptRuleNameDeserializer;
impl ReceiptRuleNameDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

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

/// <p>Information about a receipt rule set.</p> <p>A receipt rule set is a collection of rules that specify what Amazon SES should do with mail it receives on behalf of your account's verified domains.</p> <p>For information about setting up receipt rule sets, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-receipt-rule-set.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ReceiptRuleSetMetadata {
    /// <p>The date and time the receipt rule set was created.</p>
    pub created_timestamp: Option<String>,
    /// <p><p>The name of the receipt rule set. The name must:</p> <ul> <li> <p>This value can only contain ASCII letters (a-z, A-Z), numbers (0-9), underscores (_), or dashes (-).</p> </li> <li> <p>Start and end with a letter or number.</p> </li> <li> <p>Contain less than 64 characters.</p> </li> </ul></p>
    pub name: Option<String>,
}

struct ReceiptRuleSetMetadataDeserializer;
impl ReceiptRuleSetMetadataDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ReceiptRuleSetMetadata, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ReceiptRuleSetMetadata::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "CreatedTimestamp" => {
                        obj.created_timestamp = Some(try!(TimestampDeserializer::deserialize(
                            "CreatedTimestamp",
                            stack
                        )));
                    }
                    "Name" => {
                        obj.name = Some(try!(ReceiptRuleSetNameDeserializer::deserialize(
                            "Name", stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct ReceiptRuleSetNameDeserializer;
impl ReceiptRuleSetNameDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct ReceiptRuleSetsListsDeserializer;
impl ReceiptRuleSetsListsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<ReceiptRuleSetMetadata>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "member" {
                        obj.push(try!(ReceiptRuleSetMetadataDeserializer::deserialize(
                            "member", stack
                        )));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}
struct ReceiptRulesListDeserializer;
impl ReceiptRulesListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<ReceiptRule>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "member" {
                        obj.push(try!(ReceiptRuleDeserializer::deserialize("member", stack)));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}
struct RecipientDeserializer;
impl RecipientDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Recipient-related information to include in the Delivery Status Notification (DSN) when an email that Amazon SES receives on your behalf bounces.</p> <p>For information about receiving email through Amazon SES, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
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

        params.put(
            &format!("{}{}", prefix, "Action"),
            &obj.action.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.diagnostic_code {
            params.put(
                &format!("{}{}", prefix, "DiagnosticCode"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.extension_fields {
            ExtensionFieldListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "ExtensionFields"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.final_recipient {
            params.put(
                &format!("{}{}", prefix, "FinalRecipient"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.last_attempt_date {
            params.put(
                &format!("{}{}", prefix, "LastAttemptDate"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.remote_mta {
            params.put(
                &format!("{}{}", prefix, "RemoteMta"),
                &field_value.replace("+", "%2B"),
            );
        }
        params.put(
            &format!("{}{}", prefix, "Status"),
            &obj.status.replace("+", "%2B"),
        );
    }
}

struct RecipientsListDeserializer;
impl RecipientsListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "member" {
                        obj.push(try!(RecipientDeserializer::deserialize("member", stack)));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
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
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Represents a request to reorder the receipt rules within a receipt rule set. You use receipt rule sets to receive email with Amazon SES. For more information, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-concepts.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
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
        params.put(
            &format!("{}{}", prefix, "RuleSetName"),
            &obj.rule_set_name.replace("+", "%2B"),
        );
    }
}

/// <p>An empty element returned on a successful request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ReorderReceiptRuleSetResponse {}

struct ReorderReceiptRuleSetResponseDeserializer;
impl ReorderReceiptRuleSetResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ReorderReceiptRuleSetResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let obj = ReorderReceiptRuleSetResponse::default();

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Contains information about the reputation settings for a configuration set.</p>
#[derive(Default, Debug, Clone, PartialEq)]
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
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ReputationOptions, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ReputationOptions::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "LastFreshStart" => {
                        obj.last_fresh_start = Some(try!(LastFreshStartDeserializer::deserialize(
                            "LastFreshStart",
                            stack
                        )));
                    }
                    "ReputationMetricsEnabled" => {
                        obj.reputation_metrics_enabled = Some(try!(
                            EnabledDeserializer::deserialize("ReputationMetricsEnabled", stack)
                        ));
                    }
                    "SendingEnabled" => {
                        obj.sending_enabled = Some(try!(EnabledDeserializer::deserialize(
                            "SendingEnabled",
                            stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>When included in a receipt rule, this action saves the received message to an Amazon Simple Storage Service (Amazon S3) bucket and, optionally, publishes a notification to Amazon Simple Notification Service (Amazon SNS).</p> <p>To enable Amazon SES to write emails to your Amazon S3 bucket, use an AWS KMS key to encrypt your emails, or publish to an Amazon SNS topic of another account, Amazon SES must have permission to access those resources. For information about giving permissions, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-permissions.html">Amazon SES Developer Guide</a>.</p> <note> <p>When you save your emails to an Amazon S3 bucket, the maximum email size (including headers) is 30 MB. Emails larger than that will bounce.</p> </note> <p>For information about specifying Amazon S3 actions in receipt rules, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-action-s3.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct S3Action {
    /// <p>The name of the Amazon S3 bucket that incoming email will be saved to.</p>
    pub bucket_name: String,
    /// <p><p>The customer master key that Amazon SES should use to encrypt your emails before saving them to the Amazon S3 bucket. You can use the default master key or a custom master key you created in AWS KMS as follows:</p> <ul> <li> <p>To use the default master key, provide an ARN in the form of <code>arn:aws:kms:REGION:ACCOUNT-ID-WITHOUT-HYPHENS:alias/aws/ses</code>. For example, if your AWS account ID is 123456789012 and you want to use the default master key in the US West (Oregon) region, the ARN of the default master key would be <code>arn:aws:kms:us-west-2:123456789012:alias/aws/ses</code>. If you use the default master key, you don&#39;t need to perform any extra steps to give Amazon SES permission to use the key.</p> </li> <li> <p>To use a custom master key you created in AWS KMS, provide the ARN of the master key and ensure that you add a statement to your key&#39;s policy to give Amazon SES permission to use it. For more information about giving permissions, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-permissions.html">Amazon SES Developer Guide</a>.</p> </li> </ul> <p>For more information about key policies, see the <a href="http://docs.aws.amazon.com/kms/latest/developerguide/concepts.html">AWS KMS Developer Guide</a>. If you do not specify a master key, Amazon SES will not encrypt your emails.</p> <important> <p>Your mail is encrypted by Amazon SES using the Amazon S3 encryption client before the mail is submitted to Amazon S3 for storage. It is not encrypted using Amazon S3 server-side encryption. This means that you must use the Amazon S3 encryption client to decrypt the email after retrieving it from Amazon S3, as the service has no access to use your AWS KMS keys for decryption. This encryption client is currently available with the <a href="http://aws.amazon.com/sdk-for-java/">AWS Java SDK</a> and <a href="http://aws.amazon.com/sdk-for-ruby/">AWS Ruby SDK</a> only. For more information about client-side encryption using AWS KMS master keys, see the <a href="AmazonS3/latest/dev/UsingClientSideEncryption.html">Amazon S3 Developer Guide</a>.</p> </important></p>
    pub kms_key_arn: Option<String>,
    /// <p>The key prefix of the Amazon S3 bucket. The key prefix is similar to a directory name that enables you to store similar data under the same directory in a bucket.</p>
    pub object_key_prefix: Option<String>,
    /// <p>The ARN of the Amazon SNS topic to notify when the message is saved to the Amazon S3 bucket. An example of an Amazon SNS topic ARN is <code>arn:aws:sns:us-west-2:123456789012:MyTopic</code>. For more information about Amazon SNS topics, see the <a href="http://docs.aws.amazon.com/sns/latest/dg/CreateTopic.html">Amazon SNS Developer Guide</a>.</p>
    pub topic_arn: Option<String>,
}

struct S3ActionDeserializer;
impl S3ActionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<S3Action, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = S3Action::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "BucketName" => {
                            obj.bucket_name =
                                try!(S3BucketNameDeserializer::deserialize("BucketName", stack));
                        }
                        "KmsKeyArn" => {
                            obj.kms_key_arn = Some(try!(
                                AmazonResourceNameDeserializer::deserialize("KmsKeyArn", stack)
                            ));
                        }
                        "ObjectKeyPrefix" => {
                            obj.object_key_prefix = Some(try!(
                                S3KeyPrefixDeserializer::deserialize("ObjectKeyPrefix", stack)
                            ));
                        }
                        "TopicArn" => {
                            obj.topic_arn = Some(try!(
                                AmazonResourceNameDeserializer::deserialize("TopicArn", stack)
                            ));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
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

        params.put(
            &format!("{}{}", prefix, "BucketName"),
            &obj.bucket_name.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.kms_key_arn {
            params.put(
                &format!("{}{}", prefix, "KmsKeyArn"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.object_key_prefix {
            params.put(
                &format!("{}{}", prefix, "ObjectKeyPrefix"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.topic_arn {
            params.put(
                &format!("{}{}", prefix, "TopicArn"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

struct S3BucketNameDeserializer;
impl S3BucketNameDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct S3KeyPrefixDeserializer;
impl S3KeyPrefixDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>When included in a receipt rule, this action publishes a notification to Amazon Simple Notification Service (Amazon SNS). This action includes a complete copy of the email content in the Amazon SNS notifications. Amazon SNS notifications for all other actions simply provide information about the email. They do not include the email content itself.</p> <p>If you own the Amazon SNS topic, you don't need to do anything to give Amazon SES permission to publish emails to it. However, if you don't own the Amazon SNS topic, you need to attach a policy to the topic to give Amazon SES permissions to access it. For information about giving permissions, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-permissions.html">Amazon SES Developer Guide</a>.</p> <important> <p>You can only publish emails that are 150 KB or less (including the header) to Amazon SNS. Larger emails will bounce. If you anticipate emails larger than 150 KB, use the S3 action instead.</p> </important> <p>For information about using a receipt rule to publish an Amazon SNS notification, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-action-sns.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SNSAction {
    /// <p>The encoding to use for the email within the Amazon SNS notification. UTF-8 is easier to use, but may not preserve all special characters when a message was encoded with a different encoding format. Base64 preserves all special characters. The default value is UTF-8.</p>
    pub encoding: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the Amazon SNS topic to notify. An example of an Amazon SNS topic ARN is <code>arn:aws:sns:us-west-2:123456789012:MyTopic</code>. For more information about Amazon SNS topics, see the <a href="http://docs.aws.amazon.com/sns/latest/dg/CreateTopic.html">Amazon SNS Developer Guide</a>.</p>
    pub topic_arn: String,
}

struct SNSActionDeserializer;
impl SNSActionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<SNSAction, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = SNSAction::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Encoding" => {
                        obj.encoding = Some(try!(SNSActionEncodingDeserializer::deserialize(
                            "Encoding", stack
                        )));
                    }
                    "TopicArn" => {
                        obj.topic_arn = try!(AmazonResourceNameDeserializer::deserialize(
                            "TopicArn", stack
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
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
            params.put(
                &format!("{}{}", prefix, "Encoding"),
                &field_value.replace("+", "%2B"),
            );
        }
        params.put(
            &format!("{}{}", prefix, "TopicArn"),
            &obj.topic_arn.replace("+", "%2B"),
        );
    }
}

struct SNSActionEncodingDeserializer;
impl SNSActionEncodingDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Contains the topic ARN associated with an Amazon Simple Notification Service (Amazon SNS) event destination.</p> <p>Event destinations, such as Amazon SNS, are associated with configuration sets, which enable you to publish email sending events. For information about using configuration sets, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/monitor-sending-activity.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SNSDestination {
    /// <p>The ARN of the Amazon SNS topic that email sending events will be published to. An example of an Amazon SNS topic ARN is <code>arn:aws:sns:us-west-2:123456789012:MyTopic</code>. For more information about Amazon SNS topics, see the <a href="http://docs.aws.amazon.com/sns/latest/dg/CreateTopic.html">Amazon SNS Developer Guide</a>.</p>
    pub topic_arn: String,
}

struct SNSDestinationDeserializer;
impl SNSDestinationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<SNSDestination, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = SNSDestination::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "TopicARN" => {
                        obj.topic_arn = try!(AmazonResourceNameDeserializer::deserialize(
                            "TopicARN", stack
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
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

        params.put(
            &format!("{}{}", prefix, "TopicARN"),
            &obj.topic_arn.replace("+", "%2B"),
        );
    }
}

/// <p>Represents a request to send a bounce message to the sender of an email you received through Amazon SES.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SendBounceRequest {
    /// <p>The address to use in the "From" header of the bounce message. This must be an identity that you have verified with Amazon SES.</p>
    pub bounce_sender: String,
    /// <p>This parameter is used only for sending authorization. It is the ARN of the identity that is associated with the sending authorization policy that permits you to use the address in the "From" header of the bounce. For more information about sending authorization, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/sending-authorization.html">Amazon SES Developer Guide</a>.</p>
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

        params.put(
            &format!("{}{}", prefix, "BounceSender"),
            &obj.bounce_sender.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.bounce_sender_arn {
            params.put(
                &format!("{}{}", prefix, "BounceSenderArn"),
                &field_value.replace("+", "%2B"),
            );
        }
        BouncedRecipientInfoListSerializer::serialize(
            params,
            &format!("{}{}", prefix, "BouncedRecipientInfoList"),
            &obj.bounced_recipient_info_list,
        );
        if let Some(ref field_value) = obj.explanation {
            params.put(
                &format!("{}{}", prefix, "Explanation"),
                &field_value.replace("+", "%2B"),
            );
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
            &obj.original_message_id.replace("+", "%2B"),
        );
    }
}

/// <p>Represents a unique message ID.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SendBounceResponse {
    /// <p>The message ID of the bounce message.</p>
    pub message_id: Option<String>,
}

struct SendBounceResponseDeserializer;
impl SendBounceResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<SendBounceResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = SendBounceResponse::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "MessageId" => {
                        obj.message_id =
                            Some(try!(MessageIdDeserializer::deserialize("MessageId", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Represents a request to send a templated email to multiple destinations using Amazon SES. For more information, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/send-personalized-email-api.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
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
    /// <p>This parameter is used only for sending authorization. It is the ARN of the identity that is associated with the sending authorization policy that permits you to use the email address specified in the <code>ReturnPath</code> parameter.</p> <p>For example, if the owner of <code>example.com</code> (which has ARN <code>arn:aws:ses:us-east-1:123456789012:identity/example.com</code>) attaches a policy to it that authorizes you to use <code>feedback@example.com</code>, then you would specify the <code>ReturnPathArn</code> to be <code>arn:aws:ses:us-east-1:123456789012:identity/example.com</code>, and the <code>ReturnPath</code> to be <code>feedback@example.com</code>.</p> <p>For more information about sending authorization, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/sending-authorization.html">Amazon SES Developer Guide</a>.</p>
    pub return_path_arn: Option<String>,
    /// <p><p>The email address that is sending the email. This email address must be either individually verified with Amazon SES, or from a domain that has been verified with Amazon SES. For information about verifying identities, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/verify-addresses-and-domains.html">Amazon SES Developer Guide</a>.</p> <p>If you are sending on behalf of another user and have been permitted to do so by a sending authorization policy, then you must also specify the <code>SourceArn</code> parameter. For more information about sending authorization, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/sending-authorization.html">Amazon SES Developer Guide</a>.</p> <note> <p>Amazon SES does not support the SMTPUTF8 extension, as described in <a href="https://tools.ietf.org/html/rfc6531">RFC6531</a>. For this reason, the <i>local part</i> of a source email address (the part of the email address that precedes the @ sign) may only contain <a href="https://en.wikipedia.org/wiki/Email_address#Local-part">7-bit ASCII characters</a>. If the <i>domain part</i> of an address (the part after the @ sign) contains non-ASCII characters, they must be encoded using Punycode, as described in <a href="https://tools.ietf.org/html/rfc3492.html">RFC3492</a>. The sender name (also known as the <i>friendly name</i>) may contain non-ASCII characters. These characters must be encoded using MIME encoded-word syntax, as described in <a href="https://tools.ietf.org/html/rfc2047">RFC 2047</a>. MIME encoded-word syntax uses the following form: <code>=?charset?encoding?encoded-text?=</code>.</p> </note></p>
    pub source: String,
    /// <p>This parameter is used only for sending authorization. It is the ARN of the identity that is associated with the sending authorization policy that permits you to send for the email address specified in the <code>Source</code> parameter.</p> <p>For example, if the owner of <code>example.com</code> (which has ARN <code>arn:aws:ses:us-east-1:123456789012:identity/example.com</code>) attaches a policy to it that authorizes you to send from <code>user@example.com</code>, then you would specify the <code>SourceArn</code> to be <code>arn:aws:ses:us-east-1:123456789012:identity/example.com</code>, and the <code>Source</code> to be <code>user@example.com</code>.</p> <p>For more information about sending authorization, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/sending-authorization.html">Amazon SES Developer Guide</a>.</p>
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
                &field_value.replace("+", "%2B"),
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
                &field_value.replace("+", "%2B"),
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
            params.put(
                &format!("{}{}", prefix, "ReturnPath"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.return_path_arn {
            params.put(
                &format!("{}{}", prefix, "ReturnPathArn"),
                &field_value.replace("+", "%2B"),
            );
        }
        params.put(
            &format!("{}{}", prefix, "Source"),
            &obj.source.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.source_arn {
            params.put(
                &format!("{}{}", prefix, "SourceArn"),
                &field_value.replace("+", "%2B"),
            );
        }
        params.put(
            &format!("{}{}", prefix, "Template"),
            &obj.template.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.template_arn {
            params.put(
                &format!("{}{}", prefix, "TemplateArn"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct SendBulkTemplatedEmailResponse {
    /// <p>The unique message identifier returned from the <code>SendBulkTemplatedEmail</code> action.</p>
    pub status: Vec<BulkEmailDestinationStatus>,
}

struct SendBulkTemplatedEmailResponseDeserializer;
impl SendBulkTemplatedEmailResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<SendBulkTemplatedEmailResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = SendBulkTemplatedEmailResponse::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Status" => {
                        obj.status = try!(BulkEmailDestinationStatusListDeserializer::deserialize(
                            "Status", stack
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Represents a request to send a custom verification email to a specified recipient.</p>
#[derive(Default, Debug, Clone, PartialEq)]
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
                &field_value.replace("+", "%2B"),
            );
        }
        params.put(
            &format!("{}{}", prefix, "EmailAddress"),
            &obj.email_address.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "TemplateName"),
            &obj.template_name.replace("+", "%2B"),
        );
    }
}

/// <p>The response received when attempting to send the custom verification email.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SendCustomVerificationEmailResponse {
    /// <p>The unique message identifier returned from the <code>SendCustomVerificationEmail</code> operation.</p>
    pub message_id: Option<String>,
}

struct SendCustomVerificationEmailResponseDeserializer;
impl SendCustomVerificationEmailResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<SendCustomVerificationEmailResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = SendCustomVerificationEmailResponse::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "MessageId" => {
                        obj.message_id =
                            Some(try!(MessageIdDeserializer::deserialize("MessageId", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Represents sending statistics data. Each <code>SendDataPoint</code> contains statistics for a 15-minute period of sending activity. </p>
#[derive(Default, Debug, Clone, PartialEq)]
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
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<SendDataPoint, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = SendDataPoint::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Bounces" => {
                        obj.bounces =
                            Some(try!(CounterDeserializer::deserialize("Bounces", stack)));
                    }
                    "Complaints" => {
                        obj.complaints =
                            Some(try!(CounterDeserializer::deserialize("Complaints", stack)));
                    }
                    "DeliveryAttempts" => {
                        obj.delivery_attempts = Some(try!(CounterDeserializer::deserialize(
                            "DeliveryAttempts",
                            stack
                        )));
                    }
                    "Rejects" => {
                        obj.rejects =
                            Some(try!(CounterDeserializer::deserialize("Rejects", stack)));
                    }
                    "Timestamp" => {
                        obj.timestamp =
                            Some(try!(TimestampDeserializer::deserialize("Timestamp", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct SendDataPointListDeserializer;
impl SendDataPointListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<SendDataPoint>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "member" {
                        obj.push(try!(SendDataPointDeserializer::deserialize(
                            "member", stack
                        )));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}
/// <p>Represents a request to send a single formatted email using Amazon SES. For more information, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/send-email-formatted.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
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
    /// <p>This parameter is used only for sending authorization. It is the ARN of the identity that is associated with the sending authorization policy that permits you to use the email address specified in the <code>ReturnPath</code> parameter.</p> <p>For example, if the owner of <code>example.com</code> (which has ARN <code>arn:aws:ses:us-east-1:123456789012:identity/example.com</code>) attaches a policy to it that authorizes you to use <code>feedback@example.com</code>, then you would specify the <code>ReturnPathArn</code> to be <code>arn:aws:ses:us-east-1:123456789012:identity/example.com</code>, and the <code>ReturnPath</code> to be <code>feedback@example.com</code>.</p> <p>For more information about sending authorization, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/sending-authorization.html">Amazon SES Developer Guide</a>.</p>
    pub return_path_arn: Option<String>,
    /// <p><p>The email address that is sending the email. This email address must be either individually verified with Amazon SES, or from a domain that has been verified with Amazon SES. For information about verifying identities, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/verify-addresses-and-domains.html">Amazon SES Developer Guide</a>.</p> <p>If you are sending on behalf of another user and have been permitted to do so by a sending authorization policy, then you must also specify the <code>SourceArn</code> parameter. For more information about sending authorization, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/sending-authorization.html">Amazon SES Developer Guide</a>.</p> <note> <p>Amazon SES does not support the SMTPUTF8 extension, as described in <a href="https://tools.ietf.org/html/rfc6531">RFC6531</a>. For this reason, the <i>local part</i> of a source email address (the part of the email address that precedes the @ sign) may only contain <a href="https://en.wikipedia.org/wiki/Email_address#Local-part">7-bit ASCII characters</a>. If the <i>domain part</i> of an address (the part after the @ sign) contains non-ASCII characters, they must be encoded using Punycode, as described in <a href="https://tools.ietf.org/html/rfc3492.html">RFC3492</a>. The sender name (also known as the <i>friendly name</i>) may contain non-ASCII characters. These characters must be encoded using MIME encoded-word syntax, as described in <a href="https://tools.ietf.org/html/rfc2047">RFC 2047</a>. MIME encoded-word syntax uses the following form: <code>=?charset?encoding?encoded-text?=</code>.</p> </note></p>
    pub source: String,
    /// <p>This parameter is used only for sending authorization. It is the ARN of the identity that is associated with the sending authorization policy that permits you to send for the email address specified in the <code>Source</code> parameter.</p> <p>For example, if the owner of <code>example.com</code> (which has ARN <code>arn:aws:ses:us-east-1:123456789012:identity/example.com</code>) attaches a policy to it that authorizes you to send from <code>user@example.com</code>, then you would specify the <code>SourceArn</code> to be <code>arn:aws:ses:us-east-1:123456789012:identity/example.com</code>, and the <code>Source</code> to be <code>user@example.com</code>.</p> <p>For more information about sending authorization, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/sending-authorization.html">Amazon SES Developer Guide</a>.</p>
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
                &field_value.replace("+", "%2B"),
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
            params.put(
                &format!("{}{}", prefix, "ReturnPath"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.return_path_arn {
            params.put(
                &format!("{}{}", prefix, "ReturnPathArn"),
                &field_value.replace("+", "%2B"),
            );
        }
        params.put(
            &format!("{}{}", prefix, "Source"),
            &obj.source.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.source_arn {
            params.put(
                &format!("{}{}", prefix, "SourceArn"),
                &field_value.replace("+", "%2B"),
            );
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
pub struct SendEmailResponse {
    /// <p>The unique message identifier returned from the <code>SendEmail</code> action. </p>
    pub message_id: String,
}

struct SendEmailResponseDeserializer;
impl SendEmailResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<SendEmailResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = SendEmailResponse::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "MessageId" => {
                        obj.message_id =
                            try!(MessageIdDeserializer::deserialize("MessageId", stack));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Represents a request to send a single raw email using Amazon SES. For more information, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/send-email-raw.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SendRawEmailRequest {
    /// <p>The name of the configuration set to use when you send an email using <code>SendRawEmail</code>.</p>
    pub configuration_set_name: Option<String>,
    /// <p>A list of destinations for the message, consisting of To:, CC:, and BCC: addresses.</p>
    pub destinations: Option<Vec<String>>,
    /// <p><p>This parameter is used only for sending authorization. It is the ARN of the identity that is associated with the sending authorization policy that permits you to specify a particular &quot;From&quot; address in the header of the raw email.</p> <p>Instead of using this parameter, you can use the X-header <code>X-SES-FROM-ARN</code> in the raw message of the email. If you use both the <code>FromArn</code> parameter and the corresponding X-header, Amazon SES uses the value of the <code>FromArn</code> parameter.</p> <note> <p>For information about when to use this parameter, see the description of <code>SendRawEmail</code> in this guide, or see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/sending-authorization-delegate-sender-tasks-email.html">Amazon SES Developer Guide</a>.</p> </note></p>
    pub from_arn: Option<String>,
    /// <p><p>The raw text of the message. The client is responsible for ensuring the following:</p> <ul> <li> <p>Message must contain a header and a body, separated by a blank line.</p> </li> <li> <p>All required header fields must be present.</p> </li> <li> <p>Each part of a multipart MIME message must be formatted properly.</p> </li> <li> <p>MIME content types must be among those supported by Amazon SES. For more information, go to the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/mime-types.html">Amazon SES Developer Guide</a>.</p> </li> <li> <p>Must be base64-encoded.</p> </li> <li> <p>Per <a href="https://tools.ietf.org/html/rfc5321#section-4.5.3.1.6">RFC 5321</a>, the maximum length of each line of text, including the &lt;CRLF&gt;, must not exceed 1,000 characters.</p> </li> </ul></p>
    pub raw_message: RawMessage,
    /// <p><p>This parameter is used only for sending authorization. It is the ARN of the identity that is associated with the sending authorization policy that permits you to use the email address specified in the <code>ReturnPath</code> parameter.</p> <p>For example, if the owner of <code>example.com</code> (which has ARN <code>arn:aws:ses:us-east-1:123456789012:identity/example.com</code>) attaches a policy to it that authorizes you to use <code>feedback@example.com</code>, then you would specify the <code>ReturnPathArn</code> to be <code>arn:aws:ses:us-east-1:123456789012:identity/example.com</code>, and the <code>ReturnPath</code> to be <code>feedback@example.com</code>.</p> <p>Instead of using this parameter, you can use the X-header <code>X-SES-RETURN-PATH-ARN</code> in the raw message of the email. If you use both the <code>ReturnPathArn</code> parameter and the corresponding X-header, Amazon SES uses the value of the <code>ReturnPathArn</code> parameter.</p> <note> <p>For information about when to use this parameter, see the description of <code>SendRawEmail</code> in this guide, or see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/sending-authorization-delegate-sender-tasks-email.html">Amazon SES Developer Guide</a>.</p> </note></p>
    pub return_path_arn: Option<String>,
    /// <p>The identity's email address. If you do not provide a value for this parameter, you must specify a "From" address in the raw text of the message. (You can also specify both.)</p> <note> <p>Amazon SES does not support the SMTPUTF8 extension, as described in<a href="https://tools.ietf.org/html/rfc6531">RFC6531</a>. For this reason, the <i>local part</i> of a source email address (the part of the email address that precedes the @ sign) may only contain <a href="https://en.wikipedia.org/wiki/Email_address#Local-part">7-bit ASCII characters</a>. If the <i>domain part</i> of an address (the part after the @ sign) contains non-ASCII characters, they must be encoded using Punycode, as described in <a href="https://tools.ietf.org/html/rfc3492.html">RFC3492</a>. The sender name (also known as the <i>friendly name</i>) may contain non-ASCII characters. These characters must be encoded using MIME encoded-word syntax, as described in <a href="https://tools.ietf.org/html/rfc2047">RFC 2047</a>. MIME encoded-word syntax uses the following form: <code>=?charset?encoding?encoded-text?=</code>.</p> </note> <p>If you specify the <code>Source</code> parameter and have feedback forwarding enabled, then bounces and complaints will be sent to this email address. This takes precedence over any Return-Path header that you might include in the raw text of the message.</p>
    pub source: Option<String>,
    /// <p><p>This parameter is used only for sending authorization. It is the ARN of the identity that is associated with the sending authorization policy that permits you to send for the email address specified in the <code>Source</code> parameter.</p> <p>For example, if the owner of <code>example.com</code> (which has ARN <code>arn:aws:ses:us-east-1:123456789012:identity/example.com</code>) attaches a policy to it that authorizes you to send from <code>user@example.com</code>, then you would specify the <code>SourceArn</code> to be <code>arn:aws:ses:us-east-1:123456789012:identity/example.com</code>, and the <code>Source</code> to be <code>user@example.com</code>.</p> <p>Instead of using this parameter, you can use the X-header <code>X-SES-SOURCE-ARN</code> in the raw message of the email. If you use both the <code>SourceArn</code> parameter and the corresponding X-header, Amazon SES uses the value of the <code>SourceArn</code> parameter.</p> <note> <p>For information about when to use this parameter, see the description of <code>SendRawEmail</code> in this guide, or see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/sending-authorization-delegate-sender-tasks-email.html">Amazon SES Developer Guide</a>.</p> </note></p>
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
                &field_value.replace("+", "%2B"),
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
            params.put(
                &format!("{}{}", prefix, "FromArn"),
                &field_value.replace("+", "%2B"),
            );
        }
        RawMessageSerializer::serialize(
            params,
            &format!("{}{}", prefix, "RawMessage"),
            &obj.raw_message,
        );
        if let Some(ref field_value) = obj.return_path_arn {
            params.put(
                &format!("{}{}", prefix, "ReturnPathArn"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.source {
            params.put(
                &format!("{}{}", prefix, "Source"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.source_arn {
            params.put(
                &format!("{}{}", prefix, "SourceArn"),
                &field_value.replace("+", "%2B"),
            );
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
pub struct SendRawEmailResponse {
    /// <p>The unique message identifier returned from the <code>SendRawEmail</code> action. </p>
    pub message_id: String,
}

struct SendRawEmailResponseDeserializer;
impl SendRawEmailResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<SendRawEmailResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = SendRawEmailResponse::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "MessageId" => {
                        obj.message_id =
                            try!(MessageIdDeserializer::deserialize("MessageId", stack));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Represents a request to send a templated email using Amazon SES. For more information, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/send-personalized-email-api.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SendTemplatedEmailRequest {
    /// <p>The name of the configuration set to use when you send an email using <code>SendTemplatedEmail</code>.</p>
    pub configuration_set_name: Option<String>,
    /// <p>The destination for this email, composed of To:, CC:, and BCC: fields. A Destination can include up to 50 recipients across these three fields.</p>
    pub destination: Destination,
    /// <p>The reply-to email address(es) for the message. If the recipient replies to the message, each reply-to address will receive the reply.</p>
    pub reply_to_addresses: Option<Vec<String>>,
    /// <p>The email address that bounces and complaints will be forwarded to when feedback forwarding is enabled. If the message cannot be delivered to the recipient, then an error message will be returned from the recipient's ISP; this message will then be forwarded to the email address specified by the <code>ReturnPath</code> parameter. The <code>ReturnPath</code> parameter is never overwritten. This email address must be either individually verified with Amazon SES, or from a domain that has been verified with Amazon SES. </p>
    pub return_path: Option<String>,
    /// <p>This parameter is used only for sending authorization. It is the ARN of the identity that is associated with the sending authorization policy that permits you to use the email address specified in the <code>ReturnPath</code> parameter.</p> <p>For example, if the owner of <code>example.com</code> (which has ARN <code>arn:aws:ses:us-east-1:123456789012:identity/example.com</code>) attaches a policy to it that authorizes you to use <code>feedback@example.com</code>, then you would specify the <code>ReturnPathArn</code> to be <code>arn:aws:ses:us-east-1:123456789012:identity/example.com</code>, and the <code>ReturnPath</code> to be <code>feedback@example.com</code>.</p> <p>For more information about sending authorization, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/sending-authorization.html">Amazon SES Developer Guide</a>.</p>
    pub return_path_arn: Option<String>,
    /// <p><p>The email address that is sending the email. This email address must be either individually verified with Amazon SES, or from a domain that has been verified with Amazon SES. For information about verifying identities, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/verify-addresses-and-domains.html">Amazon SES Developer Guide</a>.</p> <p>If you are sending on behalf of another user and have been permitted to do so by a sending authorization policy, then you must also specify the <code>SourceArn</code> parameter. For more information about sending authorization, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/sending-authorization.html">Amazon SES Developer Guide</a>.</p> <note> <p>Amazon SES does not support the SMTPUTF8 extension, as described in <a href="https://tools.ietf.org/html/rfc6531">RFC6531</a>. For this reason, the <i>local part</i> of a source email address (the part of the email address that precedes the @ sign) may only contain <a href="https://en.wikipedia.org/wiki/Email_address#Local-part">7-bit ASCII characters</a>. If the <i>domain part</i> of an address (the part after the @ sign) contains non-ASCII characters, they must be encoded using Punycode, as described in <a href="https://tools.ietf.org/html/rfc3492.html">RFC3492</a>. The sender name (also known as the <i>friendly name</i>) may contain non-ASCII characters. These characters must be encoded using MIME encoded-word syntax, as described in<a href="https://tools.ietf.org/html/rfc2047">RFC 2047</a>. MIME encoded-word syntax uses the following form: <code>=?charset?encoding?encoded-text?=</code>.</p> </note></p>
    pub source: String,
    /// <p>This parameter is used only for sending authorization. It is the ARN of the identity that is associated with the sending authorization policy that permits you to send for the email address specified in the <code>Source</code> parameter.</p> <p>For example, if the owner of <code>example.com</code> (which has ARN <code>arn:aws:ses:us-east-1:123456789012:identity/example.com</code>) attaches a policy to it that authorizes you to send from <code>user@example.com</code>, then you would specify the <code>SourceArn</code> to be <code>arn:aws:ses:us-east-1:123456789012:identity/example.com</code>, and the <code>Source</code> to be <code>user@example.com</code>.</p> <p>For more information about sending authorization, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/sending-authorization.html">Amazon SES Developer Guide</a>.</p>
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
                &field_value.replace("+", "%2B"),
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
            params.put(
                &format!("{}{}", prefix, "ReturnPath"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.return_path_arn {
            params.put(
                &format!("{}{}", prefix, "ReturnPathArn"),
                &field_value.replace("+", "%2B"),
            );
        }
        params.put(
            &format!("{}{}", prefix, "Source"),
            &obj.source.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.source_arn {
            params.put(
                &format!("{}{}", prefix, "SourceArn"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.tags {
            MessageTagListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Tags"),
                field_value,
            );
        }
        params.put(
            &format!("{}{}", prefix, "Template"),
            &obj.template.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.template_arn {
            params.put(
                &format!("{}{}", prefix, "TemplateArn"),
                &field_value.replace("+", "%2B"),
            );
        }
        params.put(
            &format!("{}{}", prefix, "TemplateData"),
            &obj.template_data.replace("+", "%2B"),
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct SendTemplatedEmailResponse {
    /// <p>The unique message identifier returned from the <code>SendTemplatedEmail</code> action. </p>
    pub message_id: String,
}

struct SendTemplatedEmailResponseDeserializer;
impl SendTemplatedEmailResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<SendTemplatedEmailResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = SendTemplatedEmailResponse::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "MessageId" => {
                        obj.message_id =
                            try!(MessageIdDeserializer::deserialize("MessageId", stack));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct SentLast24HoursDeserializer;
impl SentLast24HoursDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<f64, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = f64::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Represents a request to set a receipt rule set as the active receipt rule set. You use receipt rule sets to receive email with Amazon SES. For more information, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-concepts.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
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
            params.put(
                &format!("{}{}", prefix, "RuleSetName"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

/// <p>An empty element returned on a successful request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SetActiveReceiptRuleSetResponse {}

struct SetActiveReceiptRuleSetResponseDeserializer;
impl SetActiveReceiptRuleSetResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<SetActiveReceiptRuleSetResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let obj = SetActiveReceiptRuleSetResponse::default();

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Represents a request to enable or disable Amazon SES Easy DKIM signing for an identity. For more information about setting up Easy DKIM, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/easy-dkim.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
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

        params.put(
            &format!("{}{}", prefix, "DkimEnabled"),
            &obj.dkim_enabled.to_string().replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "Identity"),
            &obj.identity.replace("+", "%2B"),
        );
    }
}

/// <p>An empty element returned on a successful request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SetIdentityDkimEnabledResponse {}

struct SetIdentityDkimEnabledResponseDeserializer;
impl SetIdentityDkimEnabledResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<SetIdentityDkimEnabledResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let obj = SetIdentityDkimEnabledResponse::default();

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Represents a request to enable or disable whether Amazon SES forwards you bounce and complaint notifications through email. For information about email feedback forwarding, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/notifications-via-email.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
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
            &obj.forwarding_enabled.to_string().replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "Identity"),
            &obj.identity.replace("+", "%2B"),
        );
    }
}

/// <p>An empty element returned on a successful request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SetIdentityFeedbackForwardingEnabledResponse {}

struct SetIdentityFeedbackForwardingEnabledResponseDeserializer;
impl SetIdentityFeedbackForwardingEnabledResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<SetIdentityFeedbackForwardingEnabledResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let obj = SetIdentityFeedbackForwardingEnabledResponse::default();

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Represents a request to set whether Amazon SES includes the original email headers in the Amazon SNS notifications of a specified type. For information about notifications, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/notifications-via-sns.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
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

        params.put(
            &format!("{}{}", prefix, "Enabled"),
            &obj.enabled.to_string().replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "Identity"),
            &obj.identity.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "NotificationType"),
            &obj.notification_type.replace("+", "%2B"),
        );
    }
}

/// <p>An empty element returned on a successful request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SetIdentityHeadersInNotificationsEnabledResponse {}

struct SetIdentityHeadersInNotificationsEnabledResponseDeserializer;
impl SetIdentityHeadersInNotificationsEnabledResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<SetIdentityHeadersInNotificationsEnabledResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let obj = SetIdentityHeadersInNotificationsEnabledResponse::default();

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Represents a request to enable or disable the Amazon SES custom MAIL FROM domain setup for a verified identity. For information about using a custom MAIL FROM domain, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/mail-from.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SetIdentityMailFromDomainRequest {
    /// <p>The action that you want Amazon SES to take if it cannot successfully read the required MX record when you send an email. If you choose <code>UseDefaultValue</code>, Amazon SES will use amazonses.com (or a subdomain of that) as the MAIL FROM domain. If you choose <code>RejectMessage</code>, Amazon SES will return a <code>MailFromDomainNotVerified</code> error and not send the email.</p> <p>The action specified in <code>BehaviorOnMXFailure</code> is taken when the custom MAIL FROM domain setup is in the <code>Pending</code>, <code>Failed</code>, and <code>TemporaryFailure</code> states.</p>
    pub behavior_on_mx_failure: Option<String>,
    /// <p>The verified identity for which you want to enable or disable the specified custom MAIL FROM domain.</p>
    pub identity: String,
    /// <p>The custom MAIL FROM domain that you want the verified identity to use. The MAIL FROM domain must 1) be a subdomain of the verified identity, 2) not be used in a "From" address if the MAIL FROM domain is the destination of email feedback forwarding (for more information, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/mail-from.html">Amazon SES Developer Guide</a>), and 3) not be used to receive emails. A value of <code>null</code> disables the custom MAIL FROM setting for the identity.</p>
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
                &field_value.replace("+", "%2B"),
            );
        }
        params.put(
            &format!("{}{}", prefix, "Identity"),
            &obj.identity.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.mail_from_domain {
            params.put(
                &format!("{}{}", prefix, "MailFromDomain"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

/// <p>An empty element returned on a successful request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SetIdentityMailFromDomainResponse {}

struct SetIdentityMailFromDomainResponseDeserializer;
impl SetIdentityMailFromDomainResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<SetIdentityMailFromDomainResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let obj = SetIdentityMailFromDomainResponse::default();

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Represents a request to specify the Amazon SNS topic to which Amazon SES will publish bounce, complaint, or delivery notifications for emails sent with that identity as the Source. For information about Amazon SES notifications, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/notifications-via-sns.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SetIdentityNotificationTopicRequest {
    /// <p>The identity for which the Amazon SNS topic will be set. You can specify an identity by using its name or by using its Amazon Resource Name (ARN). Examples: <code>user@example.com</code>, <code>example.com</code>, <code>arn:aws:ses:us-east-1:123456789012:identity/example.com</code>.</p>
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

        params.put(
            &format!("{}{}", prefix, "Identity"),
            &obj.identity.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "NotificationType"),
            &obj.notification_type.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.sns_topic {
            params.put(
                &format!("{}{}", prefix, "SnsTopic"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

/// <p>An empty element returned on a successful request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SetIdentityNotificationTopicResponse {}

struct SetIdentityNotificationTopicResponseDeserializer;
impl SetIdentityNotificationTopicResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<SetIdentityNotificationTopicResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let obj = SetIdentityNotificationTopicResponse::default();

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Represents a request to set the position of a receipt rule in a receipt rule set. You use receipt rule sets to receive email with Amazon SES. For more information, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-concepts.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
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
            params.put(
                &format!("{}{}", prefix, "After"),
                &field_value.replace("+", "%2B"),
            );
        }
        params.put(
            &format!("{}{}", prefix, "RuleName"),
            &obj.rule_name.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "RuleSetName"),
            &obj.rule_set_name.replace("+", "%2B"),
        );
    }
}

/// <p>An empty element returned on a successful request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SetReceiptRulePositionResponse {}

struct SetReceiptRulePositionResponseDeserializer;
impl SetReceiptRulePositionResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<SetReceiptRulePositionResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let obj = SetReceiptRulePositionResponse::default();

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>When included in a receipt rule, this action terminates the evaluation of the receipt rule set and, optionally, publishes a notification to Amazon Simple Notification Service (Amazon SNS).</p> <p>For information about setting a stop action in a receipt rule, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-action-stop.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct StopAction {
    /// <p>The name of the RuleSet that is being stopped.</p>
    pub scope: String,
    /// <p>The Amazon Resource Name (ARN) of the Amazon SNS topic to notify when the stop action is taken. An example of an Amazon SNS topic ARN is <code>arn:aws:sns:us-west-2:123456789012:MyTopic</code>. For more information about Amazon SNS topics, see the <a href="http://docs.aws.amazon.com/sns/latest/dg/CreateTopic.html">Amazon SNS Developer Guide</a>.</p>
    pub topic_arn: Option<String>,
}

struct StopActionDeserializer;
impl StopActionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<StopAction, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = StopAction::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Scope" => {
                        obj.scope = try!(StopScopeDeserializer::deserialize("Scope", stack));
                    }
                    "TopicArn" => {
                        obj.topic_arn = Some(try!(AmazonResourceNameDeserializer::deserialize(
                            "TopicArn", stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
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

        params.put(
            &format!("{}{}", prefix, "Scope"),
            &obj.scope.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.topic_arn {
            params.put(
                &format!("{}{}", prefix, "TopicArn"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

struct StopScopeDeserializer;
impl StopScopeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct SubjectDeserializer;
impl SubjectDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct SubjectPartDeserializer;
impl SubjectPartDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct SuccessRedirectionURLDeserializer;
impl SuccessRedirectionURLDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>The content of the email, composed of a subject line, an HTML part, and a text-only part.</p>
#[derive(Default, Debug, Clone, PartialEq)]
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
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Template, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = Template::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "HtmlPart" => {
                        obj.html_part =
                            Some(try!(HtmlPartDeserializer::deserialize("HtmlPart", stack)));
                    }
                    "SubjectPart" => {
                        obj.subject_part = Some(try!(SubjectPartDeserializer::deserialize(
                            "SubjectPart",
                            stack
                        )));
                    }
                    "TemplateName" => {
                        obj.template_name =
                            try!(TemplateNameDeserializer::deserialize("TemplateName", stack));
                    }
                    "TextPart" => {
                        obj.text_part =
                            Some(try!(TextPartDeserializer::deserialize("TextPart", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
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
            params.put(
                &format!("{}{}", prefix, "HtmlPart"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.subject_part {
            params.put(
                &format!("{}{}", prefix, "SubjectPart"),
                &field_value.replace("+", "%2B"),
            );
        }
        params.put(
            &format!("{}{}", prefix, "TemplateName"),
            &obj.template_name.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.text_part {
            params.put(
                &format!("{}{}", prefix, "TextPart"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

struct TemplateContentDeserializer;
impl TemplateContentDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Contains information about an email template.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct TemplateMetadata {
    /// <p>The time and date the template was created.</p>
    pub created_timestamp: Option<String>,
    /// <p>The name of the template.</p>
    pub name: Option<String>,
}

struct TemplateMetadataDeserializer;
impl TemplateMetadataDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<TemplateMetadata, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = TemplateMetadata::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "CreatedTimestamp" => {
                        obj.created_timestamp = Some(try!(TimestampDeserializer::deserialize(
                            "CreatedTimestamp",
                            stack
                        )));
                    }
                    "Name" => {
                        obj.name = Some(try!(TemplateNameDeserializer::deserialize("Name", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct TemplateMetadataListDeserializer;
impl TemplateMetadataListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<TemplateMetadata>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "member" {
                        obj.push(try!(TemplateMetadataDeserializer::deserialize(
                            "member", stack
                        )));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}
struct TemplateNameDeserializer;
impl TemplateNameDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
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

        params.put(
            &format!("{}{}", prefix, "TemplateData"),
            &obj.template_data.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "TemplateName"),
            &obj.template_name.replace("+", "%2B"),
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct TestRenderTemplateResponse {
    /// <p>The complete MIME message rendered by applying the data in the TemplateData parameter to the template specified in the TemplateName parameter.</p>
    pub rendered_template: Option<String>,
}

struct TestRenderTemplateResponseDeserializer;
impl TestRenderTemplateResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<TestRenderTemplateResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = TestRenderTemplateResponse::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "RenderedTemplate" => {
                        obj.rendered_template = Some(try!(
                            RenderedTemplateDeserializer::deserialize("RenderedTemplate", stack)
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct TextPartDeserializer;
impl TextPartDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct TimestampDeserializer;
impl TimestampDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct TlsPolicyDeserializer;
impl TlsPolicyDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>A domain that is used to redirect email recipients to an Amazon SES-operated domain. This domain captures open and click events generated by Amazon SES emails.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/configure-custom-open-click-domains.html">Configuring Custom Domains to Handle Open and Click Tracking</a> in the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/Welcome.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct TrackingOptions {
    /// <p>The custom subdomain that will be used to redirect email recipients to the Amazon SES event tracking domain.</p>
    pub custom_redirect_domain: Option<String>,
}

struct TrackingOptionsDeserializer;
impl TrackingOptionsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<TrackingOptions, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = TrackingOptions::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "CustomRedirectDomain" => {
                        obj.custom_redirect_domain =
                            Some(try!(CustomRedirectDomainDeserializer::deserialize(
                                "CustomRedirectDomain",
                                stack
                            )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
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
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

/// <p>Represents a request to enable or disable the email sending capabilities for your entire Amazon SES account.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct UpdateAccountSendingEnabledRequest {
    /// <p>Describes whether email sending is enabled or disabled for your Amazon SES account.</p>
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
            params.put(
                &format!("{}{}", prefix, "Enabled"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
    }
}

/// <p>Represents a request to update the event destination of a configuration set. Configuration sets enable you to publish email sending events. For information about using configuration sets, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/monitor-sending-activity.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
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
            &obj.configuration_set_name.replace("+", "%2B"),
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
pub struct UpdateConfigurationSetEventDestinationResponse {}

struct UpdateConfigurationSetEventDestinationResponseDeserializer;
impl UpdateConfigurationSetEventDestinationResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<UpdateConfigurationSetEventDestinationResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let obj = UpdateConfigurationSetEventDestinationResponse::default();

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Represents a request to modify the reputation metric publishing settings for a configuration set.</p>
#[derive(Default, Debug, Clone, PartialEq)]
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
            &obj.configuration_set_name.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "Enabled"),
            &obj.enabled.to_string().replace("+", "%2B"),
        );
    }
}

/// <p>Represents a request to enable or disable the email sending capabilities for a specific configuration set.</p>
#[derive(Default, Debug, Clone, PartialEq)]
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
            &obj.configuration_set_name.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "Enabled"),
            &obj.enabled.to_string().replace("+", "%2B"),
        );
    }
}

/// <p>Represents a request to update the tracking options for a configuration set. </p>
#[derive(Default, Debug, Clone, PartialEq)]
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
            &obj.configuration_set_name.replace("+", "%2B"),
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
pub struct UpdateConfigurationSetTrackingOptionsResponse {}

struct UpdateConfigurationSetTrackingOptionsResponseDeserializer;
impl UpdateConfigurationSetTrackingOptionsResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<UpdateConfigurationSetTrackingOptionsResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let obj = UpdateConfigurationSetTrackingOptionsResponse::default();

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Represents a request to update an existing custom verification email template.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct UpdateCustomVerificationEmailTemplateRequest {
    /// <p>The URL that the recipient of the verification email is sent to if his or her address is not successfully verified.</p>
    pub failure_redirection_url: Option<String>,
    /// <p>The email address that the custom verification email is sent from.</p>
    pub from_email_address: Option<String>,
    /// <p>The URL that the recipient of the verification email is sent to if his or her address is successfully verified.</p>
    pub success_redirection_url: Option<String>,
    /// <p>The content of the custom verification email. The total size of the email must be less than 10 MB. The message body may contain HTML, with some limitations. For more information, see <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/custom-verification-emails.html#custom-verification-emails-faq">Custom Verification Email Frequently Asked Questions</a> in the <i>Amazon SES Developer Guide</i>.</p>
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
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.from_email_address {
            params.put(
                &format!("{}{}", prefix, "FromEmailAddress"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.success_redirection_url {
            params.put(
                &format!("{}{}", prefix, "SuccessRedirectionURL"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.template_content {
            params.put(
                &format!("{}{}", prefix, "TemplateContent"),
                &field_value.replace("+", "%2B"),
            );
        }
        params.put(
            &format!("{}{}", prefix, "TemplateName"),
            &obj.template_name.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.template_subject {
            params.put(
                &format!("{}{}", prefix, "TemplateSubject"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

/// <p>Represents a request to update a receipt rule. You use receipt rules to receive email with Amazon SES. For more information, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-concepts.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
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
        params.put(
            &format!("{}{}", prefix, "RuleSetName"),
            &obj.rule_set_name.replace("+", "%2B"),
        );
    }
}

/// <p>An empty element returned on a successful request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct UpdateReceiptRuleResponse {}

struct UpdateReceiptRuleResponseDeserializer;
impl UpdateReceiptRuleResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<UpdateReceiptRuleResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let obj = UpdateReceiptRuleResponse::default();

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
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
pub struct UpdateTemplateResponse {}

struct UpdateTemplateResponseDeserializer;
impl UpdateTemplateResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<UpdateTemplateResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let obj = UpdateTemplateResponse::default();

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct VerificationAttributesDeserializer;
impl VerificationAttributesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<::std::collections::HashMap<String, IdentityVerificationAttributes>, XmlParseError>
    {
        try!(start_element(tag_name, stack));

        let mut obj = ::std::collections::HashMap::new();

        while try!(peek_at_name(stack)) == "entry" {
            try!(start_element("entry", stack));
            let key = try!(IdentityDeserializer::deserialize("key", stack));
            let value = try!(IdentityVerificationAttributesDeserializer::deserialize(
                "value", stack
            ));
            obj.insert(key, value);
            try!(end_element("entry", stack));
        }

        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
struct VerificationStatusDeserializer;
impl VerificationStatusDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct VerificationTokenDeserializer;
impl VerificationTokenDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct VerificationTokenListDeserializer;
impl VerificationTokenListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "member" {
                        obj.push(try!(VerificationTokenDeserializer::deserialize(
                            "member", stack
                        )));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}
/// <p>Represents a request to generate the CNAME records needed to set up Easy DKIM with Amazon SES. For more information about setting up Easy DKIM, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/easy-dkim.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
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

        params.put(
            &format!("{}{}", prefix, "Domain"),
            &obj.domain.replace("+", "%2B"),
        );
    }
}

/// <p>Returns CNAME records that you must publish to the DNS server of your domain to set up Easy DKIM with Amazon SES.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct VerifyDomainDkimResponse {
    /// <p>A set of character strings that represent the domain's identity. If the identity is an email address, the tokens represent the domain of that address.</p> <p>Using these tokens, you will need to create DNS CNAME records that point to DKIM public keys hosted by Amazon SES. Amazon Web Services will eventually detect that you have updated your DNS records; this detection process may take up to 72 hours. Upon successful detection, Amazon SES will be able to DKIM-sign emails originating from that domain.</p> <p>For more information about creating DNS records using DKIM tokens, go to the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/easy-dkim-dns-records.html">Amazon SES Developer Guide</a>.</p>
    pub dkim_tokens: Vec<String>,
}

struct VerifyDomainDkimResponseDeserializer;
impl VerifyDomainDkimResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<VerifyDomainDkimResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = VerifyDomainDkimResponse::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "DkimTokens" => {
                        obj.dkim_tokens = try!(VerificationTokenListDeserializer::deserialize(
                            "DkimTokens",
                            stack
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Represents a request to begin Amazon SES domain verification and to generate the TXT records that you must publish to the DNS server of your domain to complete the verification. For information about domain verification, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/verify-domains.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
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

        params.put(
            &format!("{}{}", prefix, "Domain"),
            &obj.domain.replace("+", "%2B"),
        );
    }
}

/// <p>Returns a TXT record that you must publish to the DNS server of your domain to complete domain verification with Amazon SES.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct VerifyDomainIdentityResponse {
    /// <p>A TXT record that you must place in the DNS settings of the domain to complete domain verification with Amazon SES.</p> <p>As Amazon SES searches for the TXT record, the domain's verification status is "Pending". When Amazon SES detects the record, the domain's verification status changes to "Success". If Amazon SES is unable to detect the record within 72 hours, the domain's verification status changes to "Failed." In that case, if you still want to verify the domain, you must restart the verification process from the beginning.</p>
    pub verification_token: String,
}

struct VerifyDomainIdentityResponseDeserializer;
impl VerifyDomainIdentityResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<VerifyDomainIdentityResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = VerifyDomainIdentityResponse::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "VerificationToken" => {
                        obj.verification_token = try!(VerificationTokenDeserializer::deserialize(
                            "VerificationToken",
                            stack
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Represents a request to begin email address verification with Amazon SES. For information about email address verification, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/verify-email-addresses.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
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

        params.put(
            &format!("{}{}", prefix, "EmailAddress"),
            &obj.email_address.replace("+", "%2B"),
        );
    }
}

/// <p>Represents a request to begin email address verification with Amazon SES. For information about email address verification, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/verify-email-addresses.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
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

        params.put(
            &format!("{}{}", prefix, "EmailAddress"),
            &obj.email_address.replace("+", "%2B"),
        );
    }
}

/// <p>An empty element returned on a successful request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct VerifyEmailIdentityResponse {}

struct VerifyEmailIdentityResponseDeserializer;
impl VerifyEmailIdentityResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<VerifyEmailIdentityResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let obj = VerifyEmailIdentityResponse::default();

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>When included in a receipt rule, this action calls Amazon WorkMail and, optionally, publishes a notification to Amazon Simple Notification Service (Amazon SNS). You will typically not use this action directly because Amazon WorkMail adds the rule automatically during its setup procedure.</p> <p>For information using a receipt rule to call Amazon WorkMail, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-action-workmail.html">Amazon SES Developer Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct WorkmailAction {
    /// <p>The ARN of the Amazon WorkMail organization. An example of an Amazon WorkMail organization ARN is <code>arn:aws:workmail:us-west-2:123456789012:organization/m-68755160c4cb4e29a2b2f8fb58f359d7</code>. For information about Amazon WorkMail organizations, see the <a href="http://docs.aws.amazon.com/workmail/latest/adminguide/organizations_overview.html">Amazon WorkMail Administrator Guide</a>.</p>
    pub organization_arn: String,
    /// <p>The Amazon Resource Name (ARN) of the Amazon SNS topic to notify when the WorkMail action is called. An example of an Amazon SNS topic ARN is <code>arn:aws:sns:us-west-2:123456789012:MyTopic</code>. For more information about Amazon SNS topics, see the <a href="http://docs.aws.amazon.com/sns/latest/dg/CreateTopic.html">Amazon SNS Developer Guide</a>.</p>
    pub topic_arn: Option<String>,
}

struct WorkmailActionDeserializer;
impl WorkmailActionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<WorkmailAction, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = WorkmailAction::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "OrganizationArn" => {
                        obj.organization_arn = try!(AmazonResourceNameDeserializer::deserialize(
                            "OrganizationArn",
                            stack
                        ));
                    }
                    "TopicArn" => {
                        obj.topic_arn = Some(try!(AmazonResourceNameDeserializer::deserialize(
                            "TopicArn", stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
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
            &obj.organization_arn.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.topic_arn {
            params.put(
                &format!("{}{}", prefix, "TopicArn"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

/// Errors returned by CloneReceiptRuleSet
#[derive(Debug, PartialEq)]
pub enum CloneReceiptRuleSetError {
    /// <p>Indicates that a resource could not be created because of a naming conflict.</p>
    AlreadyExists(String),
    /// <p>Indicates that a resource could not be created because of service limits. For a list of Amazon SES limits, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/limits.html">Amazon SES Developer Guide</a>.</p>
    LimitExceeded(String),
    /// <p>Indicates that the provided receipt rule set does not exist.</p>
    RuleSetDoesNotExist(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CloneReceiptRuleSetError {
    pub fn from_body(body: &str) -> CloneReceiptRuleSetError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "AlreadyExists" => {
                    CloneReceiptRuleSetError::AlreadyExists(String::from(parsed_error.message))
                }
                "LimitExceeded" => {
                    CloneReceiptRuleSetError::LimitExceeded(String::from(parsed_error.message))
                }
                "RuleSetDoesNotExist" => CloneReceiptRuleSetError::RuleSetDoesNotExist(
                    String::from(parsed_error.message),
                ),
                _ => CloneReceiptRuleSetError::Unknown(String::from(body)),
            },
            Err(_) => CloneReceiptRuleSetError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for CloneReceiptRuleSetError {
    fn from(err: XmlParseError) -> CloneReceiptRuleSetError {
        let XmlParseError(message) = err;
        CloneReceiptRuleSetError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for CloneReceiptRuleSetError {
    fn from(err: CredentialsError) -> CloneReceiptRuleSetError {
        CloneReceiptRuleSetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CloneReceiptRuleSetError {
    fn from(err: HttpDispatchError) -> CloneReceiptRuleSetError {
        CloneReceiptRuleSetError::HttpDispatch(err)
    }
}
impl From<io::Error> for CloneReceiptRuleSetError {
    fn from(err: io::Error) -> CloneReceiptRuleSetError {
        CloneReceiptRuleSetError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CloneReceiptRuleSetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CloneReceiptRuleSetError {
    fn description(&self) -> &str {
        match *self {
            CloneReceiptRuleSetError::AlreadyExists(ref cause) => cause,
            CloneReceiptRuleSetError::LimitExceeded(ref cause) => cause,
            CloneReceiptRuleSetError::RuleSetDoesNotExist(ref cause) => cause,
            CloneReceiptRuleSetError::Validation(ref cause) => cause,
            CloneReceiptRuleSetError::Credentials(ref err) => err.description(),
            CloneReceiptRuleSetError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CloneReceiptRuleSetError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateConfigurationSet
#[derive(Debug, PartialEq)]
pub enum CreateConfigurationSetError {
    /// <p>Indicates that the configuration set could not be created because of a naming conflict.</p>
    ConfigurationSetAlreadyExists(String),
    /// <p>Indicates that the configuration set is invalid. See the error message for details.</p>
    InvalidConfigurationSet(String),
    /// <p>Indicates that a resource could not be created because of service limits. For a list of Amazon SES limits, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/limits.html">Amazon SES Developer Guide</a>.</p>
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

impl CreateConfigurationSetError {
    pub fn from_body(body: &str) -> CreateConfigurationSetError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ConfigurationSetAlreadyExists" => {
                    CreateConfigurationSetError::ConfigurationSetAlreadyExists(String::from(
                        parsed_error.message,
                    ))
                }
                "InvalidConfigurationSet" => CreateConfigurationSetError::InvalidConfigurationSet(
                    String::from(parsed_error.message),
                ),
                "LimitExceeded" => {
                    CreateConfigurationSetError::LimitExceeded(String::from(parsed_error.message))
                }
                _ => CreateConfigurationSetError::Unknown(String::from(body)),
            },
            Err(_) => CreateConfigurationSetError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for CreateConfigurationSetError {
    fn from(err: XmlParseError) -> CreateConfigurationSetError {
        let XmlParseError(message) = err;
        CreateConfigurationSetError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for CreateConfigurationSetError {
    fn from(err: CredentialsError) -> CreateConfigurationSetError {
        CreateConfigurationSetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateConfigurationSetError {
    fn from(err: HttpDispatchError) -> CreateConfigurationSetError {
        CreateConfigurationSetError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateConfigurationSetError {
    fn from(err: io::Error) -> CreateConfigurationSetError {
        CreateConfigurationSetError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateConfigurationSetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateConfigurationSetError {
    fn description(&self) -> &str {
        match *self {
            CreateConfigurationSetError::ConfigurationSetAlreadyExists(ref cause) => cause,
            CreateConfigurationSetError::InvalidConfigurationSet(ref cause) => cause,
            CreateConfigurationSetError::LimitExceeded(ref cause) => cause,
            CreateConfigurationSetError::Validation(ref cause) => cause,
            CreateConfigurationSetError::Credentials(ref err) => err.description(),
            CreateConfigurationSetError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateConfigurationSetError::Unknown(ref cause) => cause,
        }
    }
}
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
    /// <p>Indicates that a resource could not be created because of service limits. For a list of Amazon SES limits, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/limits.html">Amazon SES Developer Guide</a>.</p>
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

impl CreateConfigurationSetEventDestinationError {
    pub fn from_body(body: &str) -> CreateConfigurationSetEventDestinationError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ConfigurationSetDoesNotExist" => {
                    CreateConfigurationSetEventDestinationError::ConfigurationSetDoesNotExist(
                        String::from(parsed_error.message),
                    )
                }
                "EventDestinationAlreadyExists" => {
                    CreateConfigurationSetEventDestinationError::EventDestinationAlreadyExists(
                        String::from(parsed_error.message),
                    )
                }
                "InvalidCloudWatchDestination" => {
                    CreateConfigurationSetEventDestinationError::InvalidCloudWatchDestination(
                        String::from(parsed_error.message),
                    )
                }
                "InvalidFirehoseDestination" => {
                    CreateConfigurationSetEventDestinationError::InvalidFirehoseDestination(
                        String::from(parsed_error.message),
                    )
                }
                "InvalidSNSDestination" => {
                    CreateConfigurationSetEventDestinationError::InvalidSNSDestination(
                        String::from(parsed_error.message),
                    )
                }
                "LimitExceeded" => CreateConfigurationSetEventDestinationError::LimitExceeded(
                    String::from(parsed_error.message),
                ),
                _ => CreateConfigurationSetEventDestinationError::Unknown(String::from(body)),
            },
            Err(_) => CreateConfigurationSetEventDestinationError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for CreateConfigurationSetEventDestinationError {
    fn from(err: XmlParseError) -> CreateConfigurationSetEventDestinationError {
        let XmlParseError(message) = err;
        CreateConfigurationSetEventDestinationError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for CreateConfigurationSetEventDestinationError {
    fn from(err: CredentialsError) -> CreateConfigurationSetEventDestinationError {
        CreateConfigurationSetEventDestinationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateConfigurationSetEventDestinationError {
    fn from(err: HttpDispatchError) -> CreateConfigurationSetEventDestinationError {
        CreateConfigurationSetEventDestinationError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateConfigurationSetEventDestinationError {
    fn from(err: io::Error) -> CreateConfigurationSetEventDestinationError {
        CreateConfigurationSetEventDestinationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateConfigurationSetEventDestinationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateConfigurationSetEventDestinationError {
    fn description(&self) -> &str {
        match *self {
            CreateConfigurationSetEventDestinationError::ConfigurationSetDoesNotExist(
                ref cause,
            ) => cause,
            CreateConfigurationSetEventDestinationError::EventDestinationAlreadyExists(
                ref cause,
            ) => cause,
            CreateConfigurationSetEventDestinationError::InvalidCloudWatchDestination(
                ref cause,
            ) => cause,
            CreateConfigurationSetEventDestinationError::InvalidFirehoseDestination(ref cause) => {
                cause
            }
            CreateConfigurationSetEventDestinationError::InvalidSNSDestination(ref cause) => cause,
            CreateConfigurationSetEventDestinationError::LimitExceeded(ref cause) => cause,
            CreateConfigurationSetEventDestinationError::Validation(ref cause) => cause,
            CreateConfigurationSetEventDestinationError::Credentials(ref err) => err.description(),
            CreateConfigurationSetEventDestinationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateConfigurationSetEventDestinationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateConfigurationSetTrackingOptions
#[derive(Debug, PartialEq)]
pub enum CreateConfigurationSetTrackingOptionsError {
    /// <p>Indicates that the configuration set does not exist.</p>
    ConfigurationSetDoesNotExist(String),
    /// <p><p>Indicates that the custom domain to be used for open and click tracking redirects is invalid. This error appears most often in the following situations:</p> <ul> <li> <p>When the tracking domain you specified is not verified in Amazon SES.</p> </li> <li> <p>When the tracking domain you specified is not a valid domain or subdomain.</p> </li> </ul></p>
    InvalidTrackingOptions(String),
    /// <p>Indicates that the configuration set you specified already contains a TrackingOptions object.</p>
    TrackingOptionsAlreadyExists(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateConfigurationSetTrackingOptionsError {
    pub fn from_body(body: &str) -> CreateConfigurationSetTrackingOptionsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ConfigurationSetDoesNotExist" => {
                    CreateConfigurationSetTrackingOptionsError::ConfigurationSetDoesNotExist(
                        String::from(parsed_error.message),
                    )
                }
                "InvalidTrackingOptions" => {
                    CreateConfigurationSetTrackingOptionsError::InvalidTrackingOptions(
                        String::from(parsed_error.message),
                    )
                }
                "TrackingOptionsAlreadyExistsException" => {
                    CreateConfigurationSetTrackingOptionsError::TrackingOptionsAlreadyExists(
                        String::from(parsed_error.message),
                    )
                }
                _ => CreateConfigurationSetTrackingOptionsError::Unknown(String::from(body)),
            },
            Err(_) => CreateConfigurationSetTrackingOptionsError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for CreateConfigurationSetTrackingOptionsError {
    fn from(err: XmlParseError) -> CreateConfigurationSetTrackingOptionsError {
        let XmlParseError(message) = err;
        CreateConfigurationSetTrackingOptionsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for CreateConfigurationSetTrackingOptionsError {
    fn from(err: CredentialsError) -> CreateConfigurationSetTrackingOptionsError {
        CreateConfigurationSetTrackingOptionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateConfigurationSetTrackingOptionsError {
    fn from(err: HttpDispatchError) -> CreateConfigurationSetTrackingOptionsError {
        CreateConfigurationSetTrackingOptionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateConfigurationSetTrackingOptionsError {
    fn from(err: io::Error) -> CreateConfigurationSetTrackingOptionsError {
        CreateConfigurationSetTrackingOptionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateConfigurationSetTrackingOptionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateConfigurationSetTrackingOptionsError {
    fn description(&self) -> &str {
        match *self {
            CreateConfigurationSetTrackingOptionsError::ConfigurationSetDoesNotExist(ref cause) => {
                cause
            }
            CreateConfigurationSetTrackingOptionsError::InvalidTrackingOptions(ref cause) => cause,
            CreateConfigurationSetTrackingOptionsError::TrackingOptionsAlreadyExists(ref cause) => {
                cause
            }
            CreateConfigurationSetTrackingOptionsError::Validation(ref cause) => cause,
            CreateConfigurationSetTrackingOptionsError::Credentials(ref err) => err.description(),
            CreateConfigurationSetTrackingOptionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateConfigurationSetTrackingOptionsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateCustomVerificationEmailTemplate
#[derive(Debug, PartialEq)]
pub enum CreateCustomVerificationEmailTemplateError {
    /// <p>Indicates that custom verification email template provided content is invalid.</p>
    CustomVerificationEmailInvalidContent(String),
    /// <p>Indicates that a custom verification email template with the name you specified already exists.</p>
    CustomVerificationEmailTemplateAlreadyExists(String),
    /// <p>Indicates that the sender address specified for a custom verification email is not verified, and is therefore not eligible to send the custom verification email. </p>
    FromEmailAddressNotVerified(String),
    /// <p>Indicates that a resource could not be created because of service limits. For a list of Amazon SES limits, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/limits.html">Amazon SES Developer Guide</a>.</p>
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

impl CreateCustomVerificationEmailTemplateError {
    pub fn from_body(body: &str) -> CreateCustomVerificationEmailTemplateError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
                            Ok(parsed_error) => {
                                match &parsed_error.code[..] {
                                    "CustomVerificationEmailInvalidContent" => CreateCustomVerificationEmailTemplateError::CustomVerificationEmailInvalidContent(String::from(parsed_error.message)),"CustomVerificationEmailTemplateAlreadyExists" => CreateCustomVerificationEmailTemplateError::CustomVerificationEmailTemplateAlreadyExists(String::from(parsed_error.message)),"FromEmailAddressNotVerified" => CreateCustomVerificationEmailTemplateError::FromEmailAddressNotVerified(String::from(parsed_error.message)),"LimitExceeded" => CreateCustomVerificationEmailTemplateError::LimitExceeded(String::from(parsed_error.message)),_ => CreateCustomVerificationEmailTemplateError::Unknown(String::from(body))
                                }
                           },
                           Err(_) => CreateCustomVerificationEmailTemplateError::Unknown(body.to_string())
                       }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for CreateCustomVerificationEmailTemplateError {
    fn from(err: XmlParseError) -> CreateCustomVerificationEmailTemplateError {
        let XmlParseError(message) = err;
        CreateCustomVerificationEmailTemplateError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for CreateCustomVerificationEmailTemplateError {
    fn from(err: CredentialsError) -> CreateCustomVerificationEmailTemplateError {
        CreateCustomVerificationEmailTemplateError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateCustomVerificationEmailTemplateError {
    fn from(err: HttpDispatchError) -> CreateCustomVerificationEmailTemplateError {
        CreateCustomVerificationEmailTemplateError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateCustomVerificationEmailTemplateError {
    fn from(err: io::Error) -> CreateCustomVerificationEmailTemplateError {
        CreateCustomVerificationEmailTemplateError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateCustomVerificationEmailTemplateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateCustomVerificationEmailTemplateError {
    fn description(&self) -> &str {
        match *self {
                            CreateCustomVerificationEmailTemplateError::CustomVerificationEmailInvalidContent(ref cause) => cause,
CreateCustomVerificationEmailTemplateError::CustomVerificationEmailTemplateAlreadyExists(ref cause) => cause,
CreateCustomVerificationEmailTemplateError::FromEmailAddressNotVerified(ref cause) => cause,
CreateCustomVerificationEmailTemplateError::LimitExceeded(ref cause) => cause,
CreateCustomVerificationEmailTemplateError::Validation(ref cause) => cause,
CreateCustomVerificationEmailTemplateError::Credentials(ref err) => err.description(),
CreateCustomVerificationEmailTemplateError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
CreateCustomVerificationEmailTemplateError::Unknown(ref cause) => cause
                        }
    }
}
/// Errors returned by CreateReceiptFilter
#[derive(Debug, PartialEq)]
pub enum CreateReceiptFilterError {
    /// <p>Indicates that a resource could not be created because of a naming conflict.</p>
    AlreadyExists(String),
    /// <p>Indicates that a resource could not be created because of service limits. For a list of Amazon SES limits, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/limits.html">Amazon SES Developer Guide</a>.</p>
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

impl CreateReceiptFilterError {
    pub fn from_body(body: &str) -> CreateReceiptFilterError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "AlreadyExists" => {
                    CreateReceiptFilterError::AlreadyExists(String::from(parsed_error.message))
                }
                "LimitExceeded" => {
                    CreateReceiptFilterError::LimitExceeded(String::from(parsed_error.message))
                }
                _ => CreateReceiptFilterError::Unknown(String::from(body)),
            },
            Err(_) => CreateReceiptFilterError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for CreateReceiptFilterError {
    fn from(err: XmlParseError) -> CreateReceiptFilterError {
        let XmlParseError(message) = err;
        CreateReceiptFilterError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for CreateReceiptFilterError {
    fn from(err: CredentialsError) -> CreateReceiptFilterError {
        CreateReceiptFilterError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateReceiptFilterError {
    fn from(err: HttpDispatchError) -> CreateReceiptFilterError {
        CreateReceiptFilterError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateReceiptFilterError {
    fn from(err: io::Error) -> CreateReceiptFilterError {
        CreateReceiptFilterError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateReceiptFilterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateReceiptFilterError {
    fn description(&self) -> &str {
        match *self {
            CreateReceiptFilterError::AlreadyExists(ref cause) => cause,
            CreateReceiptFilterError::LimitExceeded(ref cause) => cause,
            CreateReceiptFilterError::Validation(ref cause) => cause,
            CreateReceiptFilterError::Credentials(ref err) => err.description(),
            CreateReceiptFilterError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateReceiptFilterError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateReceiptRule
#[derive(Debug, PartialEq)]
pub enum CreateReceiptRuleError {
    /// <p>Indicates that a resource could not be created because of a naming conflict.</p>
    AlreadyExists(String),
    /// <p>Indicates that the provided AWS Lambda function is invalid, or that Amazon SES could not execute the provided function, possibly due to permissions issues. For information about giving permissions, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-permissions.html">Amazon SES Developer Guide</a>.</p>
    InvalidLambdaFunction(String),
    /// <p>Indicates that the provided Amazon S3 bucket or AWS KMS encryption key is invalid, or that Amazon SES could not publish to the bucket, possibly due to permissions issues. For information about giving permissions, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-permissions.html">Amazon SES Developer Guide</a>.</p>
    InvalidS3Configuration(String),
    /// <p>Indicates that the provided Amazon SNS topic is invalid, or that Amazon SES could not publish to the topic, possibly due to permissions issues. For information about giving permissions, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-permissions.html">Amazon SES Developer Guide</a>.</p>
    InvalidSnsTopic(String),
    /// <p>Indicates that a resource could not be created because of service limits. For a list of Amazon SES limits, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/limits.html">Amazon SES Developer Guide</a>.</p>
    LimitExceeded(String),
    /// <p>Indicates that the provided receipt rule does not exist.</p>
    RuleDoesNotExist(String),
    /// <p>Indicates that the provided receipt rule set does not exist.</p>
    RuleSetDoesNotExist(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateReceiptRuleError {
    pub fn from_body(body: &str) -> CreateReceiptRuleError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "AlreadyExists" => {
                    CreateReceiptRuleError::AlreadyExists(String::from(parsed_error.message))
                }
                "InvalidLambdaFunction" => CreateReceiptRuleError::InvalidLambdaFunction(
                    String::from(parsed_error.message),
                ),
                "InvalidS3Configuration" => CreateReceiptRuleError::InvalidS3Configuration(
                    String::from(parsed_error.message),
                ),
                "InvalidSnsTopic" => {
                    CreateReceiptRuleError::InvalidSnsTopic(String::from(parsed_error.message))
                }
                "LimitExceeded" => {
                    CreateReceiptRuleError::LimitExceeded(String::from(parsed_error.message))
                }
                "RuleDoesNotExist" => {
                    CreateReceiptRuleError::RuleDoesNotExist(String::from(parsed_error.message))
                }
                "RuleSetDoesNotExist" => {
                    CreateReceiptRuleError::RuleSetDoesNotExist(String::from(parsed_error.message))
                }
                _ => CreateReceiptRuleError::Unknown(String::from(body)),
            },
            Err(_) => CreateReceiptRuleError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for CreateReceiptRuleError {
    fn from(err: XmlParseError) -> CreateReceiptRuleError {
        let XmlParseError(message) = err;
        CreateReceiptRuleError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for CreateReceiptRuleError {
    fn from(err: CredentialsError) -> CreateReceiptRuleError {
        CreateReceiptRuleError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateReceiptRuleError {
    fn from(err: HttpDispatchError) -> CreateReceiptRuleError {
        CreateReceiptRuleError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateReceiptRuleError {
    fn from(err: io::Error) -> CreateReceiptRuleError {
        CreateReceiptRuleError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateReceiptRuleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateReceiptRuleError {
    fn description(&self) -> &str {
        match *self {
            CreateReceiptRuleError::AlreadyExists(ref cause) => cause,
            CreateReceiptRuleError::InvalidLambdaFunction(ref cause) => cause,
            CreateReceiptRuleError::InvalidS3Configuration(ref cause) => cause,
            CreateReceiptRuleError::InvalidSnsTopic(ref cause) => cause,
            CreateReceiptRuleError::LimitExceeded(ref cause) => cause,
            CreateReceiptRuleError::RuleDoesNotExist(ref cause) => cause,
            CreateReceiptRuleError::RuleSetDoesNotExist(ref cause) => cause,
            CreateReceiptRuleError::Validation(ref cause) => cause,
            CreateReceiptRuleError::Credentials(ref err) => err.description(),
            CreateReceiptRuleError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateReceiptRuleError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateReceiptRuleSet
#[derive(Debug, PartialEq)]
pub enum CreateReceiptRuleSetError {
    /// <p>Indicates that a resource could not be created because of a naming conflict.</p>
    AlreadyExists(String),
    /// <p>Indicates that a resource could not be created because of service limits. For a list of Amazon SES limits, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/limits.html">Amazon SES Developer Guide</a>.</p>
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

impl CreateReceiptRuleSetError {
    pub fn from_body(body: &str) -> CreateReceiptRuleSetError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "AlreadyExists" => {
                    CreateReceiptRuleSetError::AlreadyExists(String::from(parsed_error.message))
                }
                "LimitExceeded" => {
                    CreateReceiptRuleSetError::LimitExceeded(String::from(parsed_error.message))
                }
                _ => CreateReceiptRuleSetError::Unknown(String::from(body)),
            },
            Err(_) => CreateReceiptRuleSetError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for CreateReceiptRuleSetError {
    fn from(err: XmlParseError) -> CreateReceiptRuleSetError {
        let XmlParseError(message) = err;
        CreateReceiptRuleSetError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for CreateReceiptRuleSetError {
    fn from(err: CredentialsError) -> CreateReceiptRuleSetError {
        CreateReceiptRuleSetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateReceiptRuleSetError {
    fn from(err: HttpDispatchError) -> CreateReceiptRuleSetError {
        CreateReceiptRuleSetError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateReceiptRuleSetError {
    fn from(err: io::Error) -> CreateReceiptRuleSetError {
        CreateReceiptRuleSetError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateReceiptRuleSetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateReceiptRuleSetError {
    fn description(&self) -> &str {
        match *self {
            CreateReceiptRuleSetError::AlreadyExists(ref cause) => cause,
            CreateReceiptRuleSetError::LimitExceeded(ref cause) => cause,
            CreateReceiptRuleSetError::Validation(ref cause) => cause,
            CreateReceiptRuleSetError::Credentials(ref err) => err.description(),
            CreateReceiptRuleSetError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateReceiptRuleSetError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateTemplate
#[derive(Debug, PartialEq)]
pub enum CreateTemplateError {
    /// <p>Indicates that a resource could not be created because of a naming conflict.</p>
    AlreadyExists(String),
    /// <p>Indicates that a template could not be created because it contained invalid JSON.</p>
    InvalidTemplate(String),
    /// <p>Indicates that a resource could not be created because of service limits. For a list of Amazon SES limits, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/limits.html">Amazon SES Developer Guide</a>.</p>
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

impl CreateTemplateError {
    pub fn from_body(body: &str) -> CreateTemplateError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "AlreadyExists" => {
                    CreateTemplateError::AlreadyExists(String::from(parsed_error.message))
                }
                "InvalidTemplate" => {
                    CreateTemplateError::InvalidTemplate(String::from(parsed_error.message))
                }
                "LimitExceeded" => {
                    CreateTemplateError::LimitExceeded(String::from(parsed_error.message))
                }
                _ => CreateTemplateError::Unknown(String::from(body)),
            },
            Err(_) => CreateTemplateError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for CreateTemplateError {
    fn from(err: XmlParseError) -> CreateTemplateError {
        let XmlParseError(message) = err;
        CreateTemplateError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for CreateTemplateError {
    fn from(err: CredentialsError) -> CreateTemplateError {
        CreateTemplateError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateTemplateError {
    fn from(err: HttpDispatchError) -> CreateTemplateError {
        CreateTemplateError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateTemplateError {
    fn from(err: io::Error) -> CreateTemplateError {
        CreateTemplateError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateTemplateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateTemplateError {
    fn description(&self) -> &str {
        match *self {
            CreateTemplateError::AlreadyExists(ref cause) => cause,
            CreateTemplateError::InvalidTemplate(ref cause) => cause,
            CreateTemplateError::LimitExceeded(ref cause) => cause,
            CreateTemplateError::Validation(ref cause) => cause,
            CreateTemplateError::Credentials(ref err) => err.description(),
            CreateTemplateError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateTemplateError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteConfigurationSet
#[derive(Debug, PartialEq)]
pub enum DeleteConfigurationSetError {
    /// <p>Indicates that the configuration set does not exist.</p>
    ConfigurationSetDoesNotExist(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteConfigurationSetError {
    pub fn from_body(body: &str) -> DeleteConfigurationSetError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ConfigurationSetDoesNotExist" => {
                    DeleteConfigurationSetError::ConfigurationSetDoesNotExist(String::from(
                        parsed_error.message,
                    ))
                }
                _ => DeleteConfigurationSetError::Unknown(String::from(body)),
            },
            Err(_) => DeleteConfigurationSetError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DeleteConfigurationSetError {
    fn from(err: XmlParseError) -> DeleteConfigurationSetError {
        let XmlParseError(message) = err;
        DeleteConfigurationSetError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeleteConfigurationSetError {
    fn from(err: CredentialsError) -> DeleteConfigurationSetError {
        DeleteConfigurationSetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteConfigurationSetError {
    fn from(err: HttpDispatchError) -> DeleteConfigurationSetError {
        DeleteConfigurationSetError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteConfigurationSetError {
    fn from(err: io::Error) -> DeleteConfigurationSetError {
        DeleteConfigurationSetError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteConfigurationSetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteConfigurationSetError {
    fn description(&self) -> &str {
        match *self {
            DeleteConfigurationSetError::ConfigurationSetDoesNotExist(ref cause) => cause,
            DeleteConfigurationSetError::Validation(ref cause) => cause,
            DeleteConfigurationSetError::Credentials(ref err) => err.description(),
            DeleteConfigurationSetError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteConfigurationSetError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteConfigurationSetEventDestination
#[derive(Debug, PartialEq)]
pub enum DeleteConfigurationSetEventDestinationError {
    /// <p>Indicates that the configuration set does not exist.</p>
    ConfigurationSetDoesNotExist(String),
    /// <p>Indicates that the event destination does not exist.</p>
    EventDestinationDoesNotExist(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteConfigurationSetEventDestinationError {
    pub fn from_body(body: &str) -> DeleteConfigurationSetEventDestinationError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ConfigurationSetDoesNotExist" => {
                    DeleteConfigurationSetEventDestinationError::ConfigurationSetDoesNotExist(
                        String::from(parsed_error.message),
                    )
                }
                "EventDestinationDoesNotExist" => {
                    DeleteConfigurationSetEventDestinationError::EventDestinationDoesNotExist(
                        String::from(parsed_error.message),
                    )
                }
                _ => DeleteConfigurationSetEventDestinationError::Unknown(String::from(body)),
            },
            Err(_) => DeleteConfigurationSetEventDestinationError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DeleteConfigurationSetEventDestinationError {
    fn from(err: XmlParseError) -> DeleteConfigurationSetEventDestinationError {
        let XmlParseError(message) = err;
        DeleteConfigurationSetEventDestinationError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeleteConfigurationSetEventDestinationError {
    fn from(err: CredentialsError) -> DeleteConfigurationSetEventDestinationError {
        DeleteConfigurationSetEventDestinationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteConfigurationSetEventDestinationError {
    fn from(err: HttpDispatchError) -> DeleteConfigurationSetEventDestinationError {
        DeleteConfigurationSetEventDestinationError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteConfigurationSetEventDestinationError {
    fn from(err: io::Error) -> DeleteConfigurationSetEventDestinationError {
        DeleteConfigurationSetEventDestinationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteConfigurationSetEventDestinationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteConfigurationSetEventDestinationError {
    fn description(&self) -> &str {
        match *self {
            DeleteConfigurationSetEventDestinationError::ConfigurationSetDoesNotExist(
                ref cause,
            ) => cause,
            DeleteConfigurationSetEventDestinationError::EventDestinationDoesNotExist(
                ref cause,
            ) => cause,
            DeleteConfigurationSetEventDestinationError::Validation(ref cause) => cause,
            DeleteConfigurationSetEventDestinationError::Credentials(ref err) => err.description(),
            DeleteConfigurationSetEventDestinationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteConfigurationSetEventDestinationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteConfigurationSetTrackingOptions
#[derive(Debug, PartialEq)]
pub enum DeleteConfigurationSetTrackingOptionsError {
    /// <p>Indicates that the configuration set does not exist.</p>
    ConfigurationSetDoesNotExist(String),
    /// <p>Indicates that the TrackingOptions object you specified does not exist.</p>
    TrackingOptionsDoesNotExist(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteConfigurationSetTrackingOptionsError {
    pub fn from_body(body: &str) -> DeleteConfigurationSetTrackingOptionsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ConfigurationSetDoesNotExist" => {
                    DeleteConfigurationSetTrackingOptionsError::ConfigurationSetDoesNotExist(
                        String::from(parsed_error.message),
                    )
                }
                "TrackingOptionsDoesNotExistException" => {
                    DeleteConfigurationSetTrackingOptionsError::TrackingOptionsDoesNotExist(
                        String::from(parsed_error.message),
                    )
                }
                _ => DeleteConfigurationSetTrackingOptionsError::Unknown(String::from(body)),
            },
            Err(_) => DeleteConfigurationSetTrackingOptionsError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DeleteConfigurationSetTrackingOptionsError {
    fn from(err: XmlParseError) -> DeleteConfigurationSetTrackingOptionsError {
        let XmlParseError(message) = err;
        DeleteConfigurationSetTrackingOptionsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeleteConfigurationSetTrackingOptionsError {
    fn from(err: CredentialsError) -> DeleteConfigurationSetTrackingOptionsError {
        DeleteConfigurationSetTrackingOptionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteConfigurationSetTrackingOptionsError {
    fn from(err: HttpDispatchError) -> DeleteConfigurationSetTrackingOptionsError {
        DeleteConfigurationSetTrackingOptionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteConfigurationSetTrackingOptionsError {
    fn from(err: io::Error) -> DeleteConfigurationSetTrackingOptionsError {
        DeleteConfigurationSetTrackingOptionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteConfigurationSetTrackingOptionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteConfigurationSetTrackingOptionsError {
    fn description(&self) -> &str {
        match *self {
            DeleteConfigurationSetTrackingOptionsError::ConfigurationSetDoesNotExist(ref cause) => {
                cause
            }
            DeleteConfigurationSetTrackingOptionsError::TrackingOptionsDoesNotExist(ref cause) => {
                cause
            }
            DeleteConfigurationSetTrackingOptionsError::Validation(ref cause) => cause,
            DeleteConfigurationSetTrackingOptionsError::Credentials(ref err) => err.description(),
            DeleteConfigurationSetTrackingOptionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteConfigurationSetTrackingOptionsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteCustomVerificationEmailTemplate
#[derive(Debug, PartialEq)]
pub enum DeleteCustomVerificationEmailTemplateError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteCustomVerificationEmailTemplateError {
    pub fn from_body(body: &str) -> DeleteCustomVerificationEmailTemplateError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => DeleteCustomVerificationEmailTemplateError::Unknown(String::from(body)),
            },
            Err(_) => DeleteCustomVerificationEmailTemplateError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DeleteCustomVerificationEmailTemplateError {
    fn from(err: XmlParseError) -> DeleteCustomVerificationEmailTemplateError {
        let XmlParseError(message) = err;
        DeleteCustomVerificationEmailTemplateError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeleteCustomVerificationEmailTemplateError {
    fn from(err: CredentialsError) -> DeleteCustomVerificationEmailTemplateError {
        DeleteCustomVerificationEmailTemplateError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteCustomVerificationEmailTemplateError {
    fn from(err: HttpDispatchError) -> DeleteCustomVerificationEmailTemplateError {
        DeleteCustomVerificationEmailTemplateError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteCustomVerificationEmailTemplateError {
    fn from(err: io::Error) -> DeleteCustomVerificationEmailTemplateError {
        DeleteCustomVerificationEmailTemplateError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteCustomVerificationEmailTemplateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteCustomVerificationEmailTemplateError {
    fn description(&self) -> &str {
        match *self {
            DeleteCustomVerificationEmailTemplateError::Validation(ref cause) => cause,
            DeleteCustomVerificationEmailTemplateError::Credentials(ref err) => err.description(),
            DeleteCustomVerificationEmailTemplateError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteCustomVerificationEmailTemplateError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteIdentity
#[derive(Debug, PartialEq)]
pub enum DeleteIdentityError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteIdentityError {
    pub fn from_body(body: &str) -> DeleteIdentityError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => DeleteIdentityError::Unknown(String::from(body)),
            },
            Err(_) => DeleteIdentityError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DeleteIdentityError {
    fn from(err: XmlParseError) -> DeleteIdentityError {
        let XmlParseError(message) = err;
        DeleteIdentityError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeleteIdentityError {
    fn from(err: CredentialsError) -> DeleteIdentityError {
        DeleteIdentityError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteIdentityError {
    fn from(err: HttpDispatchError) -> DeleteIdentityError {
        DeleteIdentityError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteIdentityError {
    fn from(err: io::Error) -> DeleteIdentityError {
        DeleteIdentityError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteIdentityError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteIdentityError {
    fn description(&self) -> &str {
        match *self {
            DeleteIdentityError::Validation(ref cause) => cause,
            DeleteIdentityError::Credentials(ref err) => err.description(),
            DeleteIdentityError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteIdentityError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteIdentityPolicy
#[derive(Debug, PartialEq)]
pub enum DeleteIdentityPolicyError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteIdentityPolicyError {
    pub fn from_body(body: &str) -> DeleteIdentityPolicyError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => DeleteIdentityPolicyError::Unknown(String::from(body)),
            },
            Err(_) => DeleteIdentityPolicyError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DeleteIdentityPolicyError {
    fn from(err: XmlParseError) -> DeleteIdentityPolicyError {
        let XmlParseError(message) = err;
        DeleteIdentityPolicyError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeleteIdentityPolicyError {
    fn from(err: CredentialsError) -> DeleteIdentityPolicyError {
        DeleteIdentityPolicyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteIdentityPolicyError {
    fn from(err: HttpDispatchError) -> DeleteIdentityPolicyError {
        DeleteIdentityPolicyError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteIdentityPolicyError {
    fn from(err: io::Error) -> DeleteIdentityPolicyError {
        DeleteIdentityPolicyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteIdentityPolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteIdentityPolicyError {
    fn description(&self) -> &str {
        match *self {
            DeleteIdentityPolicyError::Validation(ref cause) => cause,
            DeleteIdentityPolicyError::Credentials(ref err) => err.description(),
            DeleteIdentityPolicyError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteIdentityPolicyError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteReceiptFilter
#[derive(Debug, PartialEq)]
pub enum DeleteReceiptFilterError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteReceiptFilterError {
    pub fn from_body(body: &str) -> DeleteReceiptFilterError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => DeleteReceiptFilterError::Unknown(String::from(body)),
            },
            Err(_) => DeleteReceiptFilterError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DeleteReceiptFilterError {
    fn from(err: XmlParseError) -> DeleteReceiptFilterError {
        let XmlParseError(message) = err;
        DeleteReceiptFilterError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeleteReceiptFilterError {
    fn from(err: CredentialsError) -> DeleteReceiptFilterError {
        DeleteReceiptFilterError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteReceiptFilterError {
    fn from(err: HttpDispatchError) -> DeleteReceiptFilterError {
        DeleteReceiptFilterError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteReceiptFilterError {
    fn from(err: io::Error) -> DeleteReceiptFilterError {
        DeleteReceiptFilterError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteReceiptFilterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteReceiptFilterError {
    fn description(&self) -> &str {
        match *self {
            DeleteReceiptFilterError::Validation(ref cause) => cause,
            DeleteReceiptFilterError::Credentials(ref err) => err.description(),
            DeleteReceiptFilterError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteReceiptFilterError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteReceiptRule
#[derive(Debug, PartialEq)]
pub enum DeleteReceiptRuleError {
    /// <p>Indicates that the provided receipt rule set does not exist.</p>
    RuleSetDoesNotExist(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteReceiptRuleError {
    pub fn from_body(body: &str) -> DeleteReceiptRuleError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "RuleSetDoesNotExist" => {
                    DeleteReceiptRuleError::RuleSetDoesNotExist(String::from(parsed_error.message))
                }
                _ => DeleteReceiptRuleError::Unknown(String::from(body)),
            },
            Err(_) => DeleteReceiptRuleError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DeleteReceiptRuleError {
    fn from(err: XmlParseError) -> DeleteReceiptRuleError {
        let XmlParseError(message) = err;
        DeleteReceiptRuleError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeleteReceiptRuleError {
    fn from(err: CredentialsError) -> DeleteReceiptRuleError {
        DeleteReceiptRuleError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteReceiptRuleError {
    fn from(err: HttpDispatchError) -> DeleteReceiptRuleError {
        DeleteReceiptRuleError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteReceiptRuleError {
    fn from(err: io::Error) -> DeleteReceiptRuleError {
        DeleteReceiptRuleError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteReceiptRuleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteReceiptRuleError {
    fn description(&self) -> &str {
        match *self {
            DeleteReceiptRuleError::RuleSetDoesNotExist(ref cause) => cause,
            DeleteReceiptRuleError::Validation(ref cause) => cause,
            DeleteReceiptRuleError::Credentials(ref err) => err.description(),
            DeleteReceiptRuleError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteReceiptRuleError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteReceiptRuleSet
#[derive(Debug, PartialEq)]
pub enum DeleteReceiptRuleSetError {
    /// <p>Indicates that the delete operation could not be completed.</p>
    CannotDelete(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteReceiptRuleSetError {
    pub fn from_body(body: &str) -> DeleteReceiptRuleSetError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "CannotDelete" => {
                    DeleteReceiptRuleSetError::CannotDelete(String::from(parsed_error.message))
                }
                _ => DeleteReceiptRuleSetError::Unknown(String::from(body)),
            },
            Err(_) => DeleteReceiptRuleSetError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DeleteReceiptRuleSetError {
    fn from(err: XmlParseError) -> DeleteReceiptRuleSetError {
        let XmlParseError(message) = err;
        DeleteReceiptRuleSetError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeleteReceiptRuleSetError {
    fn from(err: CredentialsError) -> DeleteReceiptRuleSetError {
        DeleteReceiptRuleSetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteReceiptRuleSetError {
    fn from(err: HttpDispatchError) -> DeleteReceiptRuleSetError {
        DeleteReceiptRuleSetError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteReceiptRuleSetError {
    fn from(err: io::Error) -> DeleteReceiptRuleSetError {
        DeleteReceiptRuleSetError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteReceiptRuleSetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteReceiptRuleSetError {
    fn description(&self) -> &str {
        match *self {
            DeleteReceiptRuleSetError::CannotDelete(ref cause) => cause,
            DeleteReceiptRuleSetError::Validation(ref cause) => cause,
            DeleteReceiptRuleSetError::Credentials(ref err) => err.description(),
            DeleteReceiptRuleSetError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteReceiptRuleSetError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteTemplate
#[derive(Debug, PartialEq)]
pub enum DeleteTemplateError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteTemplateError {
    pub fn from_body(body: &str) -> DeleteTemplateError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => DeleteTemplateError::Unknown(String::from(body)),
            },
            Err(_) => DeleteTemplateError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DeleteTemplateError {
    fn from(err: XmlParseError) -> DeleteTemplateError {
        let XmlParseError(message) = err;
        DeleteTemplateError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeleteTemplateError {
    fn from(err: CredentialsError) -> DeleteTemplateError {
        DeleteTemplateError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteTemplateError {
    fn from(err: HttpDispatchError) -> DeleteTemplateError {
        DeleteTemplateError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteTemplateError {
    fn from(err: io::Error) -> DeleteTemplateError {
        DeleteTemplateError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteTemplateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteTemplateError {
    fn description(&self) -> &str {
        match *self {
            DeleteTemplateError::Validation(ref cause) => cause,
            DeleteTemplateError::Credentials(ref err) => err.description(),
            DeleteTemplateError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteTemplateError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteVerifiedEmailAddress
#[derive(Debug, PartialEq)]
pub enum DeleteVerifiedEmailAddressError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteVerifiedEmailAddressError {
    pub fn from_body(body: &str) -> DeleteVerifiedEmailAddressError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => DeleteVerifiedEmailAddressError::Unknown(String::from(body)),
            },
            Err(_) => DeleteVerifiedEmailAddressError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DeleteVerifiedEmailAddressError {
    fn from(err: XmlParseError) -> DeleteVerifiedEmailAddressError {
        let XmlParseError(message) = err;
        DeleteVerifiedEmailAddressError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeleteVerifiedEmailAddressError {
    fn from(err: CredentialsError) -> DeleteVerifiedEmailAddressError {
        DeleteVerifiedEmailAddressError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteVerifiedEmailAddressError {
    fn from(err: HttpDispatchError) -> DeleteVerifiedEmailAddressError {
        DeleteVerifiedEmailAddressError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteVerifiedEmailAddressError {
    fn from(err: io::Error) -> DeleteVerifiedEmailAddressError {
        DeleteVerifiedEmailAddressError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteVerifiedEmailAddressError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteVerifiedEmailAddressError {
    fn description(&self) -> &str {
        match *self {
            DeleteVerifiedEmailAddressError::Validation(ref cause) => cause,
            DeleteVerifiedEmailAddressError::Credentials(ref err) => err.description(),
            DeleteVerifiedEmailAddressError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteVerifiedEmailAddressError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeActiveReceiptRuleSet
#[derive(Debug, PartialEq)]
pub enum DescribeActiveReceiptRuleSetError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeActiveReceiptRuleSetError {
    pub fn from_body(body: &str) -> DescribeActiveReceiptRuleSetError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => DescribeActiveReceiptRuleSetError::Unknown(String::from(body)),
            },
            Err(_) => DescribeActiveReceiptRuleSetError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DescribeActiveReceiptRuleSetError {
    fn from(err: XmlParseError) -> DescribeActiveReceiptRuleSetError {
        let XmlParseError(message) = err;
        DescribeActiveReceiptRuleSetError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DescribeActiveReceiptRuleSetError {
    fn from(err: CredentialsError) -> DescribeActiveReceiptRuleSetError {
        DescribeActiveReceiptRuleSetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeActiveReceiptRuleSetError {
    fn from(err: HttpDispatchError) -> DescribeActiveReceiptRuleSetError {
        DescribeActiveReceiptRuleSetError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeActiveReceiptRuleSetError {
    fn from(err: io::Error) -> DescribeActiveReceiptRuleSetError {
        DescribeActiveReceiptRuleSetError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeActiveReceiptRuleSetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeActiveReceiptRuleSetError {
    fn description(&self) -> &str {
        match *self {
            DescribeActiveReceiptRuleSetError::Validation(ref cause) => cause,
            DescribeActiveReceiptRuleSetError::Credentials(ref err) => err.description(),
            DescribeActiveReceiptRuleSetError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeActiveReceiptRuleSetError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeConfigurationSet
#[derive(Debug, PartialEq)]
pub enum DescribeConfigurationSetError {
    /// <p>Indicates that the configuration set does not exist.</p>
    ConfigurationSetDoesNotExist(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeConfigurationSetError {
    pub fn from_body(body: &str) -> DescribeConfigurationSetError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ConfigurationSetDoesNotExist" => {
                    DescribeConfigurationSetError::ConfigurationSetDoesNotExist(String::from(
                        parsed_error.message,
                    ))
                }
                _ => DescribeConfigurationSetError::Unknown(String::from(body)),
            },
            Err(_) => DescribeConfigurationSetError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DescribeConfigurationSetError {
    fn from(err: XmlParseError) -> DescribeConfigurationSetError {
        let XmlParseError(message) = err;
        DescribeConfigurationSetError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DescribeConfigurationSetError {
    fn from(err: CredentialsError) -> DescribeConfigurationSetError {
        DescribeConfigurationSetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeConfigurationSetError {
    fn from(err: HttpDispatchError) -> DescribeConfigurationSetError {
        DescribeConfigurationSetError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeConfigurationSetError {
    fn from(err: io::Error) -> DescribeConfigurationSetError {
        DescribeConfigurationSetError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeConfigurationSetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeConfigurationSetError {
    fn description(&self) -> &str {
        match *self {
            DescribeConfigurationSetError::ConfigurationSetDoesNotExist(ref cause) => cause,
            DescribeConfigurationSetError::Validation(ref cause) => cause,
            DescribeConfigurationSetError::Credentials(ref err) => err.description(),
            DescribeConfigurationSetError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeConfigurationSetError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeReceiptRule
#[derive(Debug, PartialEq)]
pub enum DescribeReceiptRuleError {
    /// <p>Indicates that the provided receipt rule does not exist.</p>
    RuleDoesNotExist(String),
    /// <p>Indicates that the provided receipt rule set does not exist.</p>
    RuleSetDoesNotExist(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeReceiptRuleError {
    pub fn from_body(body: &str) -> DescribeReceiptRuleError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "RuleDoesNotExist" => {
                    DescribeReceiptRuleError::RuleDoesNotExist(String::from(parsed_error.message))
                }
                "RuleSetDoesNotExist" => DescribeReceiptRuleError::RuleSetDoesNotExist(
                    String::from(parsed_error.message),
                ),
                _ => DescribeReceiptRuleError::Unknown(String::from(body)),
            },
            Err(_) => DescribeReceiptRuleError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DescribeReceiptRuleError {
    fn from(err: XmlParseError) -> DescribeReceiptRuleError {
        let XmlParseError(message) = err;
        DescribeReceiptRuleError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DescribeReceiptRuleError {
    fn from(err: CredentialsError) -> DescribeReceiptRuleError {
        DescribeReceiptRuleError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeReceiptRuleError {
    fn from(err: HttpDispatchError) -> DescribeReceiptRuleError {
        DescribeReceiptRuleError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeReceiptRuleError {
    fn from(err: io::Error) -> DescribeReceiptRuleError {
        DescribeReceiptRuleError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeReceiptRuleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeReceiptRuleError {
    fn description(&self) -> &str {
        match *self {
            DescribeReceiptRuleError::RuleDoesNotExist(ref cause) => cause,
            DescribeReceiptRuleError::RuleSetDoesNotExist(ref cause) => cause,
            DescribeReceiptRuleError::Validation(ref cause) => cause,
            DescribeReceiptRuleError::Credentials(ref err) => err.description(),
            DescribeReceiptRuleError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeReceiptRuleError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeReceiptRuleSet
#[derive(Debug, PartialEq)]
pub enum DescribeReceiptRuleSetError {
    /// <p>Indicates that the provided receipt rule set does not exist.</p>
    RuleSetDoesNotExist(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeReceiptRuleSetError {
    pub fn from_body(body: &str) -> DescribeReceiptRuleSetError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "RuleSetDoesNotExist" => DescribeReceiptRuleSetError::RuleSetDoesNotExist(
                    String::from(parsed_error.message),
                ),
                _ => DescribeReceiptRuleSetError::Unknown(String::from(body)),
            },
            Err(_) => DescribeReceiptRuleSetError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DescribeReceiptRuleSetError {
    fn from(err: XmlParseError) -> DescribeReceiptRuleSetError {
        let XmlParseError(message) = err;
        DescribeReceiptRuleSetError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DescribeReceiptRuleSetError {
    fn from(err: CredentialsError) -> DescribeReceiptRuleSetError {
        DescribeReceiptRuleSetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeReceiptRuleSetError {
    fn from(err: HttpDispatchError) -> DescribeReceiptRuleSetError {
        DescribeReceiptRuleSetError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeReceiptRuleSetError {
    fn from(err: io::Error) -> DescribeReceiptRuleSetError {
        DescribeReceiptRuleSetError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeReceiptRuleSetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeReceiptRuleSetError {
    fn description(&self) -> &str {
        match *self {
            DescribeReceiptRuleSetError::RuleSetDoesNotExist(ref cause) => cause,
            DescribeReceiptRuleSetError::Validation(ref cause) => cause,
            DescribeReceiptRuleSetError::Credentials(ref err) => err.description(),
            DescribeReceiptRuleSetError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeReceiptRuleSetError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetAccountSendingEnabled
#[derive(Debug, PartialEq)]
pub enum GetAccountSendingEnabledError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetAccountSendingEnabledError {
    pub fn from_body(body: &str) -> GetAccountSendingEnabledError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => GetAccountSendingEnabledError::Unknown(String::from(body)),
            },
            Err(_) => GetAccountSendingEnabledError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for GetAccountSendingEnabledError {
    fn from(err: XmlParseError) -> GetAccountSendingEnabledError {
        let XmlParseError(message) = err;
        GetAccountSendingEnabledError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for GetAccountSendingEnabledError {
    fn from(err: CredentialsError) -> GetAccountSendingEnabledError {
        GetAccountSendingEnabledError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetAccountSendingEnabledError {
    fn from(err: HttpDispatchError) -> GetAccountSendingEnabledError {
        GetAccountSendingEnabledError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetAccountSendingEnabledError {
    fn from(err: io::Error) -> GetAccountSendingEnabledError {
        GetAccountSendingEnabledError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetAccountSendingEnabledError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetAccountSendingEnabledError {
    fn description(&self) -> &str {
        match *self {
            GetAccountSendingEnabledError::Validation(ref cause) => cause,
            GetAccountSendingEnabledError::Credentials(ref err) => err.description(),
            GetAccountSendingEnabledError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetAccountSendingEnabledError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetCustomVerificationEmailTemplate
#[derive(Debug, PartialEq)]
pub enum GetCustomVerificationEmailTemplateError {
    /// <p>Indicates that a custom verification email template with the name you specified does not exist.</p>
    CustomVerificationEmailTemplateDoesNotExist(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetCustomVerificationEmailTemplateError {
    pub fn from_body(body: &str) -> GetCustomVerificationEmailTemplateError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
                            Ok(parsed_error) => {
                                match &parsed_error.code[..] {
                                    "CustomVerificationEmailTemplateDoesNotExist" => GetCustomVerificationEmailTemplateError::CustomVerificationEmailTemplateDoesNotExist(String::from(parsed_error.message)),_ => GetCustomVerificationEmailTemplateError::Unknown(String::from(body))
                                }
                           },
                           Err(_) => GetCustomVerificationEmailTemplateError::Unknown(body.to_string())
                       }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for GetCustomVerificationEmailTemplateError {
    fn from(err: XmlParseError) -> GetCustomVerificationEmailTemplateError {
        let XmlParseError(message) = err;
        GetCustomVerificationEmailTemplateError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for GetCustomVerificationEmailTemplateError {
    fn from(err: CredentialsError) -> GetCustomVerificationEmailTemplateError {
        GetCustomVerificationEmailTemplateError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetCustomVerificationEmailTemplateError {
    fn from(err: HttpDispatchError) -> GetCustomVerificationEmailTemplateError {
        GetCustomVerificationEmailTemplateError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetCustomVerificationEmailTemplateError {
    fn from(err: io::Error) -> GetCustomVerificationEmailTemplateError {
        GetCustomVerificationEmailTemplateError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetCustomVerificationEmailTemplateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetCustomVerificationEmailTemplateError {
    fn description(&self) -> &str {
        match *self {
                            GetCustomVerificationEmailTemplateError::CustomVerificationEmailTemplateDoesNotExist(ref cause) => cause,
GetCustomVerificationEmailTemplateError::Validation(ref cause) => cause,
GetCustomVerificationEmailTemplateError::Credentials(ref err) => err.description(),
GetCustomVerificationEmailTemplateError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
GetCustomVerificationEmailTemplateError::Unknown(ref cause) => cause
                        }
    }
}
/// Errors returned by GetIdentityDkimAttributes
#[derive(Debug, PartialEq)]
pub enum GetIdentityDkimAttributesError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetIdentityDkimAttributesError {
    pub fn from_body(body: &str) -> GetIdentityDkimAttributesError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => GetIdentityDkimAttributesError::Unknown(String::from(body)),
            },
            Err(_) => GetIdentityDkimAttributesError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for GetIdentityDkimAttributesError {
    fn from(err: XmlParseError) -> GetIdentityDkimAttributesError {
        let XmlParseError(message) = err;
        GetIdentityDkimAttributesError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for GetIdentityDkimAttributesError {
    fn from(err: CredentialsError) -> GetIdentityDkimAttributesError {
        GetIdentityDkimAttributesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetIdentityDkimAttributesError {
    fn from(err: HttpDispatchError) -> GetIdentityDkimAttributesError {
        GetIdentityDkimAttributesError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetIdentityDkimAttributesError {
    fn from(err: io::Error) -> GetIdentityDkimAttributesError {
        GetIdentityDkimAttributesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetIdentityDkimAttributesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetIdentityDkimAttributesError {
    fn description(&self) -> &str {
        match *self {
            GetIdentityDkimAttributesError::Validation(ref cause) => cause,
            GetIdentityDkimAttributesError::Credentials(ref err) => err.description(),
            GetIdentityDkimAttributesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetIdentityDkimAttributesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetIdentityMailFromDomainAttributes
#[derive(Debug, PartialEq)]
pub enum GetIdentityMailFromDomainAttributesError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetIdentityMailFromDomainAttributesError {
    pub fn from_body(body: &str) -> GetIdentityMailFromDomainAttributesError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => GetIdentityMailFromDomainAttributesError::Unknown(String::from(body)),
            },
            Err(_) => GetIdentityMailFromDomainAttributesError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for GetIdentityMailFromDomainAttributesError {
    fn from(err: XmlParseError) -> GetIdentityMailFromDomainAttributesError {
        let XmlParseError(message) = err;
        GetIdentityMailFromDomainAttributesError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for GetIdentityMailFromDomainAttributesError {
    fn from(err: CredentialsError) -> GetIdentityMailFromDomainAttributesError {
        GetIdentityMailFromDomainAttributesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetIdentityMailFromDomainAttributesError {
    fn from(err: HttpDispatchError) -> GetIdentityMailFromDomainAttributesError {
        GetIdentityMailFromDomainAttributesError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetIdentityMailFromDomainAttributesError {
    fn from(err: io::Error) -> GetIdentityMailFromDomainAttributesError {
        GetIdentityMailFromDomainAttributesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetIdentityMailFromDomainAttributesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetIdentityMailFromDomainAttributesError {
    fn description(&self) -> &str {
        match *self {
            GetIdentityMailFromDomainAttributesError::Validation(ref cause) => cause,
            GetIdentityMailFromDomainAttributesError::Credentials(ref err) => err.description(),
            GetIdentityMailFromDomainAttributesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetIdentityMailFromDomainAttributesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetIdentityNotificationAttributes
#[derive(Debug, PartialEq)]
pub enum GetIdentityNotificationAttributesError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetIdentityNotificationAttributesError {
    pub fn from_body(body: &str) -> GetIdentityNotificationAttributesError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => GetIdentityNotificationAttributesError::Unknown(String::from(body)),
            },
            Err(_) => GetIdentityNotificationAttributesError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for GetIdentityNotificationAttributesError {
    fn from(err: XmlParseError) -> GetIdentityNotificationAttributesError {
        let XmlParseError(message) = err;
        GetIdentityNotificationAttributesError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for GetIdentityNotificationAttributesError {
    fn from(err: CredentialsError) -> GetIdentityNotificationAttributesError {
        GetIdentityNotificationAttributesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetIdentityNotificationAttributesError {
    fn from(err: HttpDispatchError) -> GetIdentityNotificationAttributesError {
        GetIdentityNotificationAttributesError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetIdentityNotificationAttributesError {
    fn from(err: io::Error) -> GetIdentityNotificationAttributesError {
        GetIdentityNotificationAttributesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetIdentityNotificationAttributesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetIdentityNotificationAttributesError {
    fn description(&self) -> &str {
        match *self {
            GetIdentityNotificationAttributesError::Validation(ref cause) => cause,
            GetIdentityNotificationAttributesError::Credentials(ref err) => err.description(),
            GetIdentityNotificationAttributesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetIdentityNotificationAttributesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetIdentityPolicies
#[derive(Debug, PartialEq)]
pub enum GetIdentityPoliciesError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetIdentityPoliciesError {
    pub fn from_body(body: &str) -> GetIdentityPoliciesError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => GetIdentityPoliciesError::Unknown(String::from(body)),
            },
            Err(_) => GetIdentityPoliciesError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for GetIdentityPoliciesError {
    fn from(err: XmlParseError) -> GetIdentityPoliciesError {
        let XmlParseError(message) = err;
        GetIdentityPoliciesError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for GetIdentityPoliciesError {
    fn from(err: CredentialsError) -> GetIdentityPoliciesError {
        GetIdentityPoliciesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetIdentityPoliciesError {
    fn from(err: HttpDispatchError) -> GetIdentityPoliciesError {
        GetIdentityPoliciesError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetIdentityPoliciesError {
    fn from(err: io::Error) -> GetIdentityPoliciesError {
        GetIdentityPoliciesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetIdentityPoliciesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetIdentityPoliciesError {
    fn description(&self) -> &str {
        match *self {
            GetIdentityPoliciesError::Validation(ref cause) => cause,
            GetIdentityPoliciesError::Credentials(ref err) => err.description(),
            GetIdentityPoliciesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetIdentityPoliciesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetIdentityVerificationAttributes
#[derive(Debug, PartialEq)]
pub enum GetIdentityVerificationAttributesError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetIdentityVerificationAttributesError {
    pub fn from_body(body: &str) -> GetIdentityVerificationAttributesError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => GetIdentityVerificationAttributesError::Unknown(String::from(body)),
            },
            Err(_) => GetIdentityVerificationAttributesError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for GetIdentityVerificationAttributesError {
    fn from(err: XmlParseError) -> GetIdentityVerificationAttributesError {
        let XmlParseError(message) = err;
        GetIdentityVerificationAttributesError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for GetIdentityVerificationAttributesError {
    fn from(err: CredentialsError) -> GetIdentityVerificationAttributesError {
        GetIdentityVerificationAttributesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetIdentityVerificationAttributesError {
    fn from(err: HttpDispatchError) -> GetIdentityVerificationAttributesError {
        GetIdentityVerificationAttributesError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetIdentityVerificationAttributesError {
    fn from(err: io::Error) -> GetIdentityVerificationAttributesError {
        GetIdentityVerificationAttributesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetIdentityVerificationAttributesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetIdentityVerificationAttributesError {
    fn description(&self) -> &str {
        match *self {
            GetIdentityVerificationAttributesError::Validation(ref cause) => cause,
            GetIdentityVerificationAttributesError::Credentials(ref err) => err.description(),
            GetIdentityVerificationAttributesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetIdentityVerificationAttributesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetSendQuota
#[derive(Debug, PartialEq)]
pub enum GetSendQuotaError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetSendQuotaError {
    pub fn from_body(body: &str) -> GetSendQuotaError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => GetSendQuotaError::Unknown(String::from(body)),
            },
            Err(_) => GetSendQuotaError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for GetSendQuotaError {
    fn from(err: XmlParseError) -> GetSendQuotaError {
        let XmlParseError(message) = err;
        GetSendQuotaError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for GetSendQuotaError {
    fn from(err: CredentialsError) -> GetSendQuotaError {
        GetSendQuotaError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetSendQuotaError {
    fn from(err: HttpDispatchError) -> GetSendQuotaError {
        GetSendQuotaError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetSendQuotaError {
    fn from(err: io::Error) -> GetSendQuotaError {
        GetSendQuotaError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetSendQuotaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetSendQuotaError {
    fn description(&self) -> &str {
        match *self {
            GetSendQuotaError::Validation(ref cause) => cause,
            GetSendQuotaError::Credentials(ref err) => err.description(),
            GetSendQuotaError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetSendQuotaError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetSendStatistics
#[derive(Debug, PartialEq)]
pub enum GetSendStatisticsError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetSendStatisticsError {
    pub fn from_body(body: &str) -> GetSendStatisticsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => GetSendStatisticsError::Unknown(String::from(body)),
            },
            Err(_) => GetSendStatisticsError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for GetSendStatisticsError {
    fn from(err: XmlParseError) -> GetSendStatisticsError {
        let XmlParseError(message) = err;
        GetSendStatisticsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for GetSendStatisticsError {
    fn from(err: CredentialsError) -> GetSendStatisticsError {
        GetSendStatisticsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetSendStatisticsError {
    fn from(err: HttpDispatchError) -> GetSendStatisticsError {
        GetSendStatisticsError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetSendStatisticsError {
    fn from(err: io::Error) -> GetSendStatisticsError {
        GetSendStatisticsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetSendStatisticsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetSendStatisticsError {
    fn description(&self) -> &str {
        match *self {
            GetSendStatisticsError::Validation(ref cause) => cause,
            GetSendStatisticsError::Credentials(ref err) => err.description(),
            GetSendStatisticsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetSendStatisticsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetTemplate
#[derive(Debug, PartialEq)]
pub enum GetTemplateError {
    /// <p>Indicates that the Template object you specified does not exist in your Amazon SES account.</p>
    TemplateDoesNotExist(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetTemplateError {
    pub fn from_body(body: &str) -> GetTemplateError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "TemplateDoesNotExist" => {
                    GetTemplateError::TemplateDoesNotExist(String::from(parsed_error.message))
                }
                _ => GetTemplateError::Unknown(String::from(body)),
            },
            Err(_) => GetTemplateError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for GetTemplateError {
    fn from(err: XmlParseError) -> GetTemplateError {
        let XmlParseError(message) = err;
        GetTemplateError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for GetTemplateError {
    fn from(err: CredentialsError) -> GetTemplateError {
        GetTemplateError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetTemplateError {
    fn from(err: HttpDispatchError) -> GetTemplateError {
        GetTemplateError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetTemplateError {
    fn from(err: io::Error) -> GetTemplateError {
        GetTemplateError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetTemplateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetTemplateError {
    fn description(&self) -> &str {
        match *self {
            GetTemplateError::TemplateDoesNotExist(ref cause) => cause,
            GetTemplateError::Validation(ref cause) => cause,
            GetTemplateError::Credentials(ref err) => err.description(),
            GetTemplateError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetTemplateError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListConfigurationSets
#[derive(Debug, PartialEq)]
pub enum ListConfigurationSetsError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListConfigurationSetsError {
    pub fn from_body(body: &str) -> ListConfigurationSetsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => ListConfigurationSetsError::Unknown(String::from(body)),
            },
            Err(_) => ListConfigurationSetsError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for ListConfigurationSetsError {
    fn from(err: XmlParseError) -> ListConfigurationSetsError {
        let XmlParseError(message) = err;
        ListConfigurationSetsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ListConfigurationSetsError {
    fn from(err: CredentialsError) -> ListConfigurationSetsError {
        ListConfigurationSetsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListConfigurationSetsError {
    fn from(err: HttpDispatchError) -> ListConfigurationSetsError {
        ListConfigurationSetsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListConfigurationSetsError {
    fn from(err: io::Error) -> ListConfigurationSetsError {
        ListConfigurationSetsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListConfigurationSetsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListConfigurationSetsError {
    fn description(&self) -> &str {
        match *self {
            ListConfigurationSetsError::Validation(ref cause) => cause,
            ListConfigurationSetsError::Credentials(ref err) => err.description(),
            ListConfigurationSetsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListConfigurationSetsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListCustomVerificationEmailTemplates
#[derive(Debug, PartialEq)]
pub enum ListCustomVerificationEmailTemplatesError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListCustomVerificationEmailTemplatesError {
    pub fn from_body(body: &str) -> ListCustomVerificationEmailTemplatesError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => ListCustomVerificationEmailTemplatesError::Unknown(String::from(body)),
            },
            Err(_) => ListCustomVerificationEmailTemplatesError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for ListCustomVerificationEmailTemplatesError {
    fn from(err: XmlParseError) -> ListCustomVerificationEmailTemplatesError {
        let XmlParseError(message) = err;
        ListCustomVerificationEmailTemplatesError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ListCustomVerificationEmailTemplatesError {
    fn from(err: CredentialsError) -> ListCustomVerificationEmailTemplatesError {
        ListCustomVerificationEmailTemplatesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListCustomVerificationEmailTemplatesError {
    fn from(err: HttpDispatchError) -> ListCustomVerificationEmailTemplatesError {
        ListCustomVerificationEmailTemplatesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListCustomVerificationEmailTemplatesError {
    fn from(err: io::Error) -> ListCustomVerificationEmailTemplatesError {
        ListCustomVerificationEmailTemplatesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListCustomVerificationEmailTemplatesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListCustomVerificationEmailTemplatesError {
    fn description(&self) -> &str {
        match *self {
            ListCustomVerificationEmailTemplatesError::Validation(ref cause) => cause,
            ListCustomVerificationEmailTemplatesError::Credentials(ref err) => err.description(),
            ListCustomVerificationEmailTemplatesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListCustomVerificationEmailTemplatesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListIdentities
#[derive(Debug, PartialEq)]
pub enum ListIdentitiesError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListIdentitiesError {
    pub fn from_body(body: &str) -> ListIdentitiesError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => ListIdentitiesError::Unknown(String::from(body)),
            },
            Err(_) => ListIdentitiesError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for ListIdentitiesError {
    fn from(err: XmlParseError) -> ListIdentitiesError {
        let XmlParseError(message) = err;
        ListIdentitiesError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ListIdentitiesError {
    fn from(err: CredentialsError) -> ListIdentitiesError {
        ListIdentitiesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListIdentitiesError {
    fn from(err: HttpDispatchError) -> ListIdentitiesError {
        ListIdentitiesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListIdentitiesError {
    fn from(err: io::Error) -> ListIdentitiesError {
        ListIdentitiesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListIdentitiesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListIdentitiesError {
    fn description(&self) -> &str {
        match *self {
            ListIdentitiesError::Validation(ref cause) => cause,
            ListIdentitiesError::Credentials(ref err) => err.description(),
            ListIdentitiesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListIdentitiesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListIdentityPolicies
#[derive(Debug, PartialEq)]
pub enum ListIdentityPoliciesError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListIdentityPoliciesError {
    pub fn from_body(body: &str) -> ListIdentityPoliciesError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => ListIdentityPoliciesError::Unknown(String::from(body)),
            },
            Err(_) => ListIdentityPoliciesError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for ListIdentityPoliciesError {
    fn from(err: XmlParseError) -> ListIdentityPoliciesError {
        let XmlParseError(message) = err;
        ListIdentityPoliciesError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ListIdentityPoliciesError {
    fn from(err: CredentialsError) -> ListIdentityPoliciesError {
        ListIdentityPoliciesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListIdentityPoliciesError {
    fn from(err: HttpDispatchError) -> ListIdentityPoliciesError {
        ListIdentityPoliciesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListIdentityPoliciesError {
    fn from(err: io::Error) -> ListIdentityPoliciesError {
        ListIdentityPoliciesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListIdentityPoliciesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListIdentityPoliciesError {
    fn description(&self) -> &str {
        match *self {
            ListIdentityPoliciesError::Validation(ref cause) => cause,
            ListIdentityPoliciesError::Credentials(ref err) => err.description(),
            ListIdentityPoliciesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListIdentityPoliciesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListReceiptFilters
#[derive(Debug, PartialEq)]
pub enum ListReceiptFiltersError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListReceiptFiltersError {
    pub fn from_body(body: &str) -> ListReceiptFiltersError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => ListReceiptFiltersError::Unknown(String::from(body)),
            },
            Err(_) => ListReceiptFiltersError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for ListReceiptFiltersError {
    fn from(err: XmlParseError) -> ListReceiptFiltersError {
        let XmlParseError(message) = err;
        ListReceiptFiltersError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ListReceiptFiltersError {
    fn from(err: CredentialsError) -> ListReceiptFiltersError {
        ListReceiptFiltersError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListReceiptFiltersError {
    fn from(err: HttpDispatchError) -> ListReceiptFiltersError {
        ListReceiptFiltersError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListReceiptFiltersError {
    fn from(err: io::Error) -> ListReceiptFiltersError {
        ListReceiptFiltersError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListReceiptFiltersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListReceiptFiltersError {
    fn description(&self) -> &str {
        match *self {
            ListReceiptFiltersError::Validation(ref cause) => cause,
            ListReceiptFiltersError::Credentials(ref err) => err.description(),
            ListReceiptFiltersError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListReceiptFiltersError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListReceiptRuleSets
#[derive(Debug, PartialEq)]
pub enum ListReceiptRuleSetsError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListReceiptRuleSetsError {
    pub fn from_body(body: &str) -> ListReceiptRuleSetsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => ListReceiptRuleSetsError::Unknown(String::from(body)),
            },
            Err(_) => ListReceiptRuleSetsError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for ListReceiptRuleSetsError {
    fn from(err: XmlParseError) -> ListReceiptRuleSetsError {
        let XmlParseError(message) = err;
        ListReceiptRuleSetsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ListReceiptRuleSetsError {
    fn from(err: CredentialsError) -> ListReceiptRuleSetsError {
        ListReceiptRuleSetsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListReceiptRuleSetsError {
    fn from(err: HttpDispatchError) -> ListReceiptRuleSetsError {
        ListReceiptRuleSetsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListReceiptRuleSetsError {
    fn from(err: io::Error) -> ListReceiptRuleSetsError {
        ListReceiptRuleSetsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListReceiptRuleSetsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListReceiptRuleSetsError {
    fn description(&self) -> &str {
        match *self {
            ListReceiptRuleSetsError::Validation(ref cause) => cause,
            ListReceiptRuleSetsError::Credentials(ref err) => err.description(),
            ListReceiptRuleSetsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListReceiptRuleSetsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListTemplates
#[derive(Debug, PartialEq)]
pub enum ListTemplatesError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListTemplatesError {
    pub fn from_body(body: &str) -> ListTemplatesError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => ListTemplatesError::Unknown(String::from(body)),
            },
            Err(_) => ListTemplatesError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for ListTemplatesError {
    fn from(err: XmlParseError) -> ListTemplatesError {
        let XmlParseError(message) = err;
        ListTemplatesError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ListTemplatesError {
    fn from(err: CredentialsError) -> ListTemplatesError {
        ListTemplatesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListTemplatesError {
    fn from(err: HttpDispatchError) -> ListTemplatesError {
        ListTemplatesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListTemplatesError {
    fn from(err: io::Error) -> ListTemplatesError {
        ListTemplatesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListTemplatesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListTemplatesError {
    fn description(&self) -> &str {
        match *self {
            ListTemplatesError::Validation(ref cause) => cause,
            ListTemplatesError::Credentials(ref err) => err.description(),
            ListTemplatesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListTemplatesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListVerifiedEmailAddresses
#[derive(Debug, PartialEq)]
pub enum ListVerifiedEmailAddressesError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListVerifiedEmailAddressesError {
    pub fn from_body(body: &str) -> ListVerifiedEmailAddressesError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => ListVerifiedEmailAddressesError::Unknown(String::from(body)),
            },
            Err(_) => ListVerifiedEmailAddressesError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for ListVerifiedEmailAddressesError {
    fn from(err: XmlParseError) -> ListVerifiedEmailAddressesError {
        let XmlParseError(message) = err;
        ListVerifiedEmailAddressesError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ListVerifiedEmailAddressesError {
    fn from(err: CredentialsError) -> ListVerifiedEmailAddressesError {
        ListVerifiedEmailAddressesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListVerifiedEmailAddressesError {
    fn from(err: HttpDispatchError) -> ListVerifiedEmailAddressesError {
        ListVerifiedEmailAddressesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListVerifiedEmailAddressesError {
    fn from(err: io::Error) -> ListVerifiedEmailAddressesError {
        ListVerifiedEmailAddressesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListVerifiedEmailAddressesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListVerifiedEmailAddressesError {
    fn description(&self) -> &str {
        match *self {
            ListVerifiedEmailAddressesError::Validation(ref cause) => cause,
            ListVerifiedEmailAddressesError::Credentials(ref err) => err.description(),
            ListVerifiedEmailAddressesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListVerifiedEmailAddressesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PutIdentityPolicy
#[derive(Debug, PartialEq)]
pub enum PutIdentityPolicyError {
    /// <p>Indicates that the provided policy is invalid. Check the error stack for more information about what caused the error.</p>
    InvalidPolicy(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl PutIdentityPolicyError {
    pub fn from_body(body: &str) -> PutIdentityPolicyError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "InvalidPolicy" => {
                    PutIdentityPolicyError::InvalidPolicy(String::from(parsed_error.message))
                }
                _ => PutIdentityPolicyError::Unknown(String::from(body)),
            },
            Err(_) => PutIdentityPolicyError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for PutIdentityPolicyError {
    fn from(err: XmlParseError) -> PutIdentityPolicyError {
        let XmlParseError(message) = err;
        PutIdentityPolicyError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for PutIdentityPolicyError {
    fn from(err: CredentialsError) -> PutIdentityPolicyError {
        PutIdentityPolicyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutIdentityPolicyError {
    fn from(err: HttpDispatchError) -> PutIdentityPolicyError {
        PutIdentityPolicyError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutIdentityPolicyError {
    fn from(err: io::Error) -> PutIdentityPolicyError {
        PutIdentityPolicyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutIdentityPolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutIdentityPolicyError {
    fn description(&self) -> &str {
        match *self {
            PutIdentityPolicyError::InvalidPolicy(ref cause) => cause,
            PutIdentityPolicyError::Validation(ref cause) => cause,
            PutIdentityPolicyError::Credentials(ref err) => err.description(),
            PutIdentityPolicyError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            PutIdentityPolicyError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ReorderReceiptRuleSet
#[derive(Debug, PartialEq)]
pub enum ReorderReceiptRuleSetError {
    /// <p>Indicates that the provided receipt rule does not exist.</p>
    RuleDoesNotExist(String),
    /// <p>Indicates that the provided receipt rule set does not exist.</p>
    RuleSetDoesNotExist(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ReorderReceiptRuleSetError {
    pub fn from_body(body: &str) -> ReorderReceiptRuleSetError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "RuleDoesNotExist" => {
                    ReorderReceiptRuleSetError::RuleDoesNotExist(String::from(parsed_error.message))
                }
                "RuleSetDoesNotExist" => ReorderReceiptRuleSetError::RuleSetDoesNotExist(
                    String::from(parsed_error.message),
                ),
                _ => ReorderReceiptRuleSetError::Unknown(String::from(body)),
            },
            Err(_) => ReorderReceiptRuleSetError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for ReorderReceiptRuleSetError {
    fn from(err: XmlParseError) -> ReorderReceiptRuleSetError {
        let XmlParseError(message) = err;
        ReorderReceiptRuleSetError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ReorderReceiptRuleSetError {
    fn from(err: CredentialsError) -> ReorderReceiptRuleSetError {
        ReorderReceiptRuleSetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ReorderReceiptRuleSetError {
    fn from(err: HttpDispatchError) -> ReorderReceiptRuleSetError {
        ReorderReceiptRuleSetError::HttpDispatch(err)
    }
}
impl From<io::Error> for ReorderReceiptRuleSetError {
    fn from(err: io::Error) -> ReorderReceiptRuleSetError {
        ReorderReceiptRuleSetError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ReorderReceiptRuleSetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ReorderReceiptRuleSetError {
    fn description(&self) -> &str {
        match *self {
            ReorderReceiptRuleSetError::RuleDoesNotExist(ref cause) => cause,
            ReorderReceiptRuleSetError::RuleSetDoesNotExist(ref cause) => cause,
            ReorderReceiptRuleSetError::Validation(ref cause) => cause,
            ReorderReceiptRuleSetError::Credentials(ref err) => err.description(),
            ReorderReceiptRuleSetError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ReorderReceiptRuleSetError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by SendBounce
#[derive(Debug, PartialEq)]
pub enum SendBounceError {
    /// <p>Indicates that the action failed, and the message could not be sent. Check the error stack for more information about what caused the error.</p>
    MessageRejected(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl SendBounceError {
    pub fn from_body(body: &str) -> SendBounceError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "MessageRejected" => {
                    SendBounceError::MessageRejected(String::from(parsed_error.message))
                }
                _ => SendBounceError::Unknown(String::from(body)),
            },
            Err(_) => SendBounceError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for SendBounceError {
    fn from(err: XmlParseError) -> SendBounceError {
        let XmlParseError(message) = err;
        SendBounceError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for SendBounceError {
    fn from(err: CredentialsError) -> SendBounceError {
        SendBounceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for SendBounceError {
    fn from(err: HttpDispatchError) -> SendBounceError {
        SendBounceError::HttpDispatch(err)
    }
}
impl From<io::Error> for SendBounceError {
    fn from(err: io::Error) -> SendBounceError {
        SendBounceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for SendBounceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SendBounceError {
    fn description(&self) -> &str {
        match *self {
            SendBounceError::MessageRejected(ref cause) => cause,
            SendBounceError::Validation(ref cause) => cause,
            SendBounceError::Credentials(ref err) => err.description(),
            SendBounceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            SendBounceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by SendBulkTemplatedEmail
#[derive(Debug, PartialEq)]
pub enum SendBulkTemplatedEmailError {
    /// <p>Indicates that email sending is disabled for your entire Amazon SES account.</p> <p>You can enable or disable email sending for your Amazon SES account using <a>UpdateAccountSendingEnabled</a>.</p>
    AccountSendingPaused(String),
    /// <p>Indicates that the configuration set does not exist.</p>
    ConfigurationSetDoesNotExist(String),
    /// <p>Indicates that email sending is disabled for the configuration set.</p> <p>You can enable or disable email sending for a configuration set using <a>UpdateConfigurationSetSendingEnabled</a>.</p>
    ConfigurationSetSendingPaused(String),
    /// <p> Indicates that the message could not be sent because Amazon SES could not read the MX record required to use the specified MAIL FROM domain. For information about editing the custom MAIL FROM domain settings for an identity, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/mail-from-edit.html">Amazon SES Developer Guide</a>.</p>
    MailFromDomainNotVerified(String),
    /// <p>Indicates that the action failed, and the message could not be sent. Check the error stack for more information about what caused the error.</p>
    MessageRejected(String),
    /// <p>Indicates that the Template object you specified does not exist in your Amazon SES account.</p>
    TemplateDoesNotExist(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl SendBulkTemplatedEmailError {
    pub fn from_body(body: &str) -> SendBulkTemplatedEmailError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "AccountSendingPausedException" => {
                    SendBulkTemplatedEmailError::AccountSendingPaused(String::from(
                        parsed_error.message,
                    ))
                }
                "ConfigurationSetDoesNotExist" => {
                    SendBulkTemplatedEmailError::ConfigurationSetDoesNotExist(String::from(
                        parsed_error.message,
                    ))
                }
                "ConfigurationSetSendingPausedException" => {
                    SendBulkTemplatedEmailError::ConfigurationSetSendingPaused(String::from(
                        parsed_error.message,
                    ))
                }
                "MailFromDomainNotVerifiedException" => {
                    SendBulkTemplatedEmailError::MailFromDomainNotVerified(String::from(
                        parsed_error.message,
                    ))
                }
                "MessageRejected" => {
                    SendBulkTemplatedEmailError::MessageRejected(String::from(parsed_error.message))
                }
                "TemplateDoesNotExist" => SendBulkTemplatedEmailError::TemplateDoesNotExist(
                    String::from(parsed_error.message),
                ),
                _ => SendBulkTemplatedEmailError::Unknown(String::from(body)),
            },
            Err(_) => SendBulkTemplatedEmailError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for SendBulkTemplatedEmailError {
    fn from(err: XmlParseError) -> SendBulkTemplatedEmailError {
        let XmlParseError(message) = err;
        SendBulkTemplatedEmailError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for SendBulkTemplatedEmailError {
    fn from(err: CredentialsError) -> SendBulkTemplatedEmailError {
        SendBulkTemplatedEmailError::Credentials(err)
    }
}
impl From<HttpDispatchError> for SendBulkTemplatedEmailError {
    fn from(err: HttpDispatchError) -> SendBulkTemplatedEmailError {
        SendBulkTemplatedEmailError::HttpDispatch(err)
    }
}
impl From<io::Error> for SendBulkTemplatedEmailError {
    fn from(err: io::Error) -> SendBulkTemplatedEmailError {
        SendBulkTemplatedEmailError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for SendBulkTemplatedEmailError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SendBulkTemplatedEmailError {
    fn description(&self) -> &str {
        match *self {
            SendBulkTemplatedEmailError::AccountSendingPaused(ref cause) => cause,
            SendBulkTemplatedEmailError::ConfigurationSetDoesNotExist(ref cause) => cause,
            SendBulkTemplatedEmailError::ConfigurationSetSendingPaused(ref cause) => cause,
            SendBulkTemplatedEmailError::MailFromDomainNotVerified(ref cause) => cause,
            SendBulkTemplatedEmailError::MessageRejected(ref cause) => cause,
            SendBulkTemplatedEmailError::TemplateDoesNotExist(ref cause) => cause,
            SendBulkTemplatedEmailError::Validation(ref cause) => cause,
            SendBulkTemplatedEmailError::Credentials(ref err) => err.description(),
            SendBulkTemplatedEmailError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            SendBulkTemplatedEmailError::Unknown(ref cause) => cause,
        }
    }
}
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl SendCustomVerificationEmailError {
    pub fn from_body(body: &str) -> SendCustomVerificationEmailError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ConfigurationSetDoesNotExist" => {
                    SendCustomVerificationEmailError::ConfigurationSetDoesNotExist(String::from(
                        parsed_error.message,
                    ))
                }
                "CustomVerificationEmailTemplateDoesNotExist" => {
                    SendCustomVerificationEmailError::CustomVerificationEmailTemplateDoesNotExist(
                        String::from(parsed_error.message),
                    )
                }
                "FromEmailAddressNotVerified" => {
                    SendCustomVerificationEmailError::FromEmailAddressNotVerified(String::from(
                        parsed_error.message,
                    ))
                }
                "MessageRejected" => SendCustomVerificationEmailError::MessageRejected(
                    String::from(parsed_error.message),
                ),
                "ProductionAccessNotGranted" => {
                    SendCustomVerificationEmailError::ProductionAccessNotGranted(String::from(
                        parsed_error.message,
                    ))
                }
                _ => SendCustomVerificationEmailError::Unknown(String::from(body)),
            },
            Err(_) => SendCustomVerificationEmailError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for SendCustomVerificationEmailError {
    fn from(err: XmlParseError) -> SendCustomVerificationEmailError {
        let XmlParseError(message) = err;
        SendCustomVerificationEmailError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for SendCustomVerificationEmailError {
    fn from(err: CredentialsError) -> SendCustomVerificationEmailError {
        SendCustomVerificationEmailError::Credentials(err)
    }
}
impl From<HttpDispatchError> for SendCustomVerificationEmailError {
    fn from(err: HttpDispatchError) -> SendCustomVerificationEmailError {
        SendCustomVerificationEmailError::HttpDispatch(err)
    }
}
impl From<io::Error> for SendCustomVerificationEmailError {
    fn from(err: io::Error) -> SendCustomVerificationEmailError {
        SendCustomVerificationEmailError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for SendCustomVerificationEmailError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SendCustomVerificationEmailError {
    fn description(&self) -> &str {
        match *self {
            SendCustomVerificationEmailError::ConfigurationSetDoesNotExist(ref cause) => cause,
            SendCustomVerificationEmailError::CustomVerificationEmailTemplateDoesNotExist(
                ref cause,
            ) => cause,
            SendCustomVerificationEmailError::FromEmailAddressNotVerified(ref cause) => cause,
            SendCustomVerificationEmailError::MessageRejected(ref cause) => cause,
            SendCustomVerificationEmailError::ProductionAccessNotGranted(ref cause) => cause,
            SendCustomVerificationEmailError::Validation(ref cause) => cause,
            SendCustomVerificationEmailError::Credentials(ref err) => err.description(),
            SendCustomVerificationEmailError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            SendCustomVerificationEmailError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by SendEmail
#[derive(Debug, PartialEq)]
pub enum SendEmailError {
    /// <p>Indicates that email sending is disabled for your entire Amazon SES account.</p> <p>You can enable or disable email sending for your Amazon SES account using <a>UpdateAccountSendingEnabled</a>.</p>
    AccountSendingPaused(String),
    /// <p>Indicates that the configuration set does not exist.</p>
    ConfigurationSetDoesNotExist(String),
    /// <p>Indicates that email sending is disabled for the configuration set.</p> <p>You can enable or disable email sending for a configuration set using <a>UpdateConfigurationSetSendingEnabled</a>.</p>
    ConfigurationSetSendingPaused(String),
    /// <p> Indicates that the message could not be sent because Amazon SES could not read the MX record required to use the specified MAIL FROM domain. For information about editing the custom MAIL FROM domain settings for an identity, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/mail-from-edit.html">Amazon SES Developer Guide</a>.</p>
    MailFromDomainNotVerified(String),
    /// <p>Indicates that the action failed, and the message could not be sent. Check the error stack for more information about what caused the error.</p>
    MessageRejected(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl SendEmailError {
    pub fn from_body(body: &str) -> SendEmailError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "AccountSendingPausedException" => {
                    SendEmailError::AccountSendingPaused(String::from(parsed_error.message))
                }
                "ConfigurationSetDoesNotExist" => {
                    SendEmailError::ConfigurationSetDoesNotExist(String::from(parsed_error.message))
                }
                "ConfigurationSetSendingPausedException" => {
                    SendEmailError::ConfigurationSetSendingPaused(String::from(
                        parsed_error.message,
                    ))
                }
                "MailFromDomainNotVerifiedException" => {
                    SendEmailError::MailFromDomainNotVerified(String::from(parsed_error.message))
                }
                "MessageRejected" => {
                    SendEmailError::MessageRejected(String::from(parsed_error.message))
                }
                _ => SendEmailError::Unknown(String::from(body)),
            },
            Err(_) => SendEmailError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for SendEmailError {
    fn from(err: XmlParseError) -> SendEmailError {
        let XmlParseError(message) = err;
        SendEmailError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for SendEmailError {
    fn from(err: CredentialsError) -> SendEmailError {
        SendEmailError::Credentials(err)
    }
}
impl From<HttpDispatchError> for SendEmailError {
    fn from(err: HttpDispatchError) -> SendEmailError {
        SendEmailError::HttpDispatch(err)
    }
}
impl From<io::Error> for SendEmailError {
    fn from(err: io::Error) -> SendEmailError {
        SendEmailError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for SendEmailError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SendEmailError {
    fn description(&self) -> &str {
        match *self {
            SendEmailError::AccountSendingPaused(ref cause) => cause,
            SendEmailError::ConfigurationSetDoesNotExist(ref cause) => cause,
            SendEmailError::ConfigurationSetSendingPaused(ref cause) => cause,
            SendEmailError::MailFromDomainNotVerified(ref cause) => cause,
            SendEmailError::MessageRejected(ref cause) => cause,
            SendEmailError::Validation(ref cause) => cause,
            SendEmailError::Credentials(ref err) => err.description(),
            SendEmailError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            SendEmailError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by SendRawEmail
#[derive(Debug, PartialEq)]
pub enum SendRawEmailError {
    /// <p>Indicates that email sending is disabled for your entire Amazon SES account.</p> <p>You can enable or disable email sending for your Amazon SES account using <a>UpdateAccountSendingEnabled</a>.</p>
    AccountSendingPaused(String),
    /// <p>Indicates that the configuration set does not exist.</p>
    ConfigurationSetDoesNotExist(String),
    /// <p>Indicates that email sending is disabled for the configuration set.</p> <p>You can enable or disable email sending for a configuration set using <a>UpdateConfigurationSetSendingEnabled</a>.</p>
    ConfigurationSetSendingPaused(String),
    /// <p> Indicates that the message could not be sent because Amazon SES could not read the MX record required to use the specified MAIL FROM domain. For information about editing the custom MAIL FROM domain settings for an identity, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/mail-from-edit.html">Amazon SES Developer Guide</a>.</p>
    MailFromDomainNotVerified(String),
    /// <p>Indicates that the action failed, and the message could not be sent. Check the error stack for more information about what caused the error.</p>
    MessageRejected(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl SendRawEmailError {
    pub fn from_body(body: &str) -> SendRawEmailError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "AccountSendingPausedException" => {
                    SendRawEmailError::AccountSendingPaused(String::from(parsed_error.message))
                }
                "ConfigurationSetDoesNotExist" => SendRawEmailError::ConfigurationSetDoesNotExist(
                    String::from(parsed_error.message),
                ),
                "ConfigurationSetSendingPausedException" => {
                    SendRawEmailError::ConfigurationSetSendingPaused(String::from(
                        parsed_error.message,
                    ))
                }
                "MailFromDomainNotVerifiedException" => {
                    SendRawEmailError::MailFromDomainNotVerified(String::from(parsed_error.message))
                }
                "MessageRejected" => {
                    SendRawEmailError::MessageRejected(String::from(parsed_error.message))
                }
                _ => SendRawEmailError::Unknown(String::from(body)),
            },
            Err(_) => SendRawEmailError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for SendRawEmailError {
    fn from(err: XmlParseError) -> SendRawEmailError {
        let XmlParseError(message) = err;
        SendRawEmailError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for SendRawEmailError {
    fn from(err: CredentialsError) -> SendRawEmailError {
        SendRawEmailError::Credentials(err)
    }
}
impl From<HttpDispatchError> for SendRawEmailError {
    fn from(err: HttpDispatchError) -> SendRawEmailError {
        SendRawEmailError::HttpDispatch(err)
    }
}
impl From<io::Error> for SendRawEmailError {
    fn from(err: io::Error) -> SendRawEmailError {
        SendRawEmailError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for SendRawEmailError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SendRawEmailError {
    fn description(&self) -> &str {
        match *self {
            SendRawEmailError::AccountSendingPaused(ref cause) => cause,
            SendRawEmailError::ConfigurationSetDoesNotExist(ref cause) => cause,
            SendRawEmailError::ConfigurationSetSendingPaused(ref cause) => cause,
            SendRawEmailError::MailFromDomainNotVerified(ref cause) => cause,
            SendRawEmailError::MessageRejected(ref cause) => cause,
            SendRawEmailError::Validation(ref cause) => cause,
            SendRawEmailError::Credentials(ref err) => err.description(),
            SendRawEmailError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            SendRawEmailError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by SendTemplatedEmail
#[derive(Debug, PartialEq)]
pub enum SendTemplatedEmailError {
    /// <p>Indicates that email sending is disabled for your entire Amazon SES account.</p> <p>You can enable or disable email sending for your Amazon SES account using <a>UpdateAccountSendingEnabled</a>.</p>
    AccountSendingPaused(String),
    /// <p>Indicates that the configuration set does not exist.</p>
    ConfigurationSetDoesNotExist(String),
    /// <p>Indicates that email sending is disabled for the configuration set.</p> <p>You can enable or disable email sending for a configuration set using <a>UpdateConfigurationSetSendingEnabled</a>.</p>
    ConfigurationSetSendingPaused(String),
    /// <p> Indicates that the message could not be sent because Amazon SES could not read the MX record required to use the specified MAIL FROM domain. For information about editing the custom MAIL FROM domain settings for an identity, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/mail-from-edit.html">Amazon SES Developer Guide</a>.</p>
    MailFromDomainNotVerified(String),
    /// <p>Indicates that the action failed, and the message could not be sent. Check the error stack for more information about what caused the error.</p>
    MessageRejected(String),
    /// <p>Indicates that the Template object you specified does not exist in your Amazon SES account.</p>
    TemplateDoesNotExist(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl SendTemplatedEmailError {
    pub fn from_body(body: &str) -> SendTemplatedEmailError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "AccountSendingPausedException" => SendTemplatedEmailError::AccountSendingPaused(
                    String::from(parsed_error.message),
                ),
                "ConfigurationSetDoesNotExist" => {
                    SendTemplatedEmailError::ConfigurationSetDoesNotExist(String::from(
                        parsed_error.message,
                    ))
                }
                "ConfigurationSetSendingPausedException" => {
                    SendTemplatedEmailError::ConfigurationSetSendingPaused(String::from(
                        parsed_error.message,
                    ))
                }
                "MailFromDomainNotVerifiedException" => {
                    SendTemplatedEmailError::MailFromDomainNotVerified(String::from(
                        parsed_error.message,
                    ))
                }
                "MessageRejected" => {
                    SendTemplatedEmailError::MessageRejected(String::from(parsed_error.message))
                }
                "TemplateDoesNotExist" => SendTemplatedEmailError::TemplateDoesNotExist(
                    String::from(parsed_error.message),
                ),
                _ => SendTemplatedEmailError::Unknown(String::from(body)),
            },
            Err(_) => SendTemplatedEmailError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for SendTemplatedEmailError {
    fn from(err: XmlParseError) -> SendTemplatedEmailError {
        let XmlParseError(message) = err;
        SendTemplatedEmailError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for SendTemplatedEmailError {
    fn from(err: CredentialsError) -> SendTemplatedEmailError {
        SendTemplatedEmailError::Credentials(err)
    }
}
impl From<HttpDispatchError> for SendTemplatedEmailError {
    fn from(err: HttpDispatchError) -> SendTemplatedEmailError {
        SendTemplatedEmailError::HttpDispatch(err)
    }
}
impl From<io::Error> for SendTemplatedEmailError {
    fn from(err: io::Error) -> SendTemplatedEmailError {
        SendTemplatedEmailError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for SendTemplatedEmailError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SendTemplatedEmailError {
    fn description(&self) -> &str {
        match *self {
            SendTemplatedEmailError::AccountSendingPaused(ref cause) => cause,
            SendTemplatedEmailError::ConfigurationSetDoesNotExist(ref cause) => cause,
            SendTemplatedEmailError::ConfigurationSetSendingPaused(ref cause) => cause,
            SendTemplatedEmailError::MailFromDomainNotVerified(ref cause) => cause,
            SendTemplatedEmailError::MessageRejected(ref cause) => cause,
            SendTemplatedEmailError::TemplateDoesNotExist(ref cause) => cause,
            SendTemplatedEmailError::Validation(ref cause) => cause,
            SendTemplatedEmailError::Credentials(ref err) => err.description(),
            SendTemplatedEmailError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            SendTemplatedEmailError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by SetActiveReceiptRuleSet
#[derive(Debug, PartialEq)]
pub enum SetActiveReceiptRuleSetError {
    /// <p>Indicates that the provided receipt rule set does not exist.</p>
    RuleSetDoesNotExist(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl SetActiveReceiptRuleSetError {
    pub fn from_body(body: &str) -> SetActiveReceiptRuleSetError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "RuleSetDoesNotExist" => SetActiveReceiptRuleSetError::RuleSetDoesNotExist(
                    String::from(parsed_error.message),
                ),
                _ => SetActiveReceiptRuleSetError::Unknown(String::from(body)),
            },
            Err(_) => SetActiveReceiptRuleSetError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for SetActiveReceiptRuleSetError {
    fn from(err: XmlParseError) -> SetActiveReceiptRuleSetError {
        let XmlParseError(message) = err;
        SetActiveReceiptRuleSetError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for SetActiveReceiptRuleSetError {
    fn from(err: CredentialsError) -> SetActiveReceiptRuleSetError {
        SetActiveReceiptRuleSetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for SetActiveReceiptRuleSetError {
    fn from(err: HttpDispatchError) -> SetActiveReceiptRuleSetError {
        SetActiveReceiptRuleSetError::HttpDispatch(err)
    }
}
impl From<io::Error> for SetActiveReceiptRuleSetError {
    fn from(err: io::Error) -> SetActiveReceiptRuleSetError {
        SetActiveReceiptRuleSetError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for SetActiveReceiptRuleSetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SetActiveReceiptRuleSetError {
    fn description(&self) -> &str {
        match *self {
            SetActiveReceiptRuleSetError::RuleSetDoesNotExist(ref cause) => cause,
            SetActiveReceiptRuleSetError::Validation(ref cause) => cause,
            SetActiveReceiptRuleSetError::Credentials(ref err) => err.description(),
            SetActiveReceiptRuleSetError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            SetActiveReceiptRuleSetError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by SetIdentityDkimEnabled
#[derive(Debug, PartialEq)]
pub enum SetIdentityDkimEnabledError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl SetIdentityDkimEnabledError {
    pub fn from_body(body: &str) -> SetIdentityDkimEnabledError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => SetIdentityDkimEnabledError::Unknown(String::from(body)),
            },
            Err(_) => SetIdentityDkimEnabledError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for SetIdentityDkimEnabledError {
    fn from(err: XmlParseError) -> SetIdentityDkimEnabledError {
        let XmlParseError(message) = err;
        SetIdentityDkimEnabledError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for SetIdentityDkimEnabledError {
    fn from(err: CredentialsError) -> SetIdentityDkimEnabledError {
        SetIdentityDkimEnabledError::Credentials(err)
    }
}
impl From<HttpDispatchError> for SetIdentityDkimEnabledError {
    fn from(err: HttpDispatchError) -> SetIdentityDkimEnabledError {
        SetIdentityDkimEnabledError::HttpDispatch(err)
    }
}
impl From<io::Error> for SetIdentityDkimEnabledError {
    fn from(err: io::Error) -> SetIdentityDkimEnabledError {
        SetIdentityDkimEnabledError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for SetIdentityDkimEnabledError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SetIdentityDkimEnabledError {
    fn description(&self) -> &str {
        match *self {
            SetIdentityDkimEnabledError::Validation(ref cause) => cause,
            SetIdentityDkimEnabledError::Credentials(ref err) => err.description(),
            SetIdentityDkimEnabledError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            SetIdentityDkimEnabledError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by SetIdentityFeedbackForwardingEnabled
#[derive(Debug, PartialEq)]
pub enum SetIdentityFeedbackForwardingEnabledError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl SetIdentityFeedbackForwardingEnabledError {
    pub fn from_body(body: &str) -> SetIdentityFeedbackForwardingEnabledError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => SetIdentityFeedbackForwardingEnabledError::Unknown(String::from(body)),
            },
            Err(_) => SetIdentityFeedbackForwardingEnabledError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for SetIdentityFeedbackForwardingEnabledError {
    fn from(err: XmlParseError) -> SetIdentityFeedbackForwardingEnabledError {
        let XmlParseError(message) = err;
        SetIdentityFeedbackForwardingEnabledError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for SetIdentityFeedbackForwardingEnabledError {
    fn from(err: CredentialsError) -> SetIdentityFeedbackForwardingEnabledError {
        SetIdentityFeedbackForwardingEnabledError::Credentials(err)
    }
}
impl From<HttpDispatchError> for SetIdentityFeedbackForwardingEnabledError {
    fn from(err: HttpDispatchError) -> SetIdentityFeedbackForwardingEnabledError {
        SetIdentityFeedbackForwardingEnabledError::HttpDispatch(err)
    }
}
impl From<io::Error> for SetIdentityFeedbackForwardingEnabledError {
    fn from(err: io::Error) -> SetIdentityFeedbackForwardingEnabledError {
        SetIdentityFeedbackForwardingEnabledError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for SetIdentityFeedbackForwardingEnabledError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SetIdentityFeedbackForwardingEnabledError {
    fn description(&self) -> &str {
        match *self {
            SetIdentityFeedbackForwardingEnabledError::Validation(ref cause) => cause,
            SetIdentityFeedbackForwardingEnabledError::Credentials(ref err) => err.description(),
            SetIdentityFeedbackForwardingEnabledError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            SetIdentityFeedbackForwardingEnabledError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by SetIdentityHeadersInNotificationsEnabled
#[derive(Debug, PartialEq)]
pub enum SetIdentityHeadersInNotificationsEnabledError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl SetIdentityHeadersInNotificationsEnabledError {
    pub fn from_body(body: &str) -> SetIdentityHeadersInNotificationsEnabledError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => SetIdentityHeadersInNotificationsEnabledError::Unknown(String::from(body)),
            },
            Err(_) => SetIdentityHeadersInNotificationsEnabledError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for SetIdentityHeadersInNotificationsEnabledError {
    fn from(err: XmlParseError) -> SetIdentityHeadersInNotificationsEnabledError {
        let XmlParseError(message) = err;
        SetIdentityHeadersInNotificationsEnabledError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for SetIdentityHeadersInNotificationsEnabledError {
    fn from(err: CredentialsError) -> SetIdentityHeadersInNotificationsEnabledError {
        SetIdentityHeadersInNotificationsEnabledError::Credentials(err)
    }
}
impl From<HttpDispatchError> for SetIdentityHeadersInNotificationsEnabledError {
    fn from(err: HttpDispatchError) -> SetIdentityHeadersInNotificationsEnabledError {
        SetIdentityHeadersInNotificationsEnabledError::HttpDispatch(err)
    }
}
impl From<io::Error> for SetIdentityHeadersInNotificationsEnabledError {
    fn from(err: io::Error) -> SetIdentityHeadersInNotificationsEnabledError {
        SetIdentityHeadersInNotificationsEnabledError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for SetIdentityHeadersInNotificationsEnabledError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SetIdentityHeadersInNotificationsEnabledError {
    fn description(&self) -> &str {
        match *self {
            SetIdentityHeadersInNotificationsEnabledError::Validation(ref cause) => cause,
            SetIdentityHeadersInNotificationsEnabledError::Credentials(ref err) => {
                err.description()
            }
            SetIdentityHeadersInNotificationsEnabledError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            SetIdentityHeadersInNotificationsEnabledError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by SetIdentityMailFromDomain
#[derive(Debug, PartialEq)]
pub enum SetIdentityMailFromDomainError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl SetIdentityMailFromDomainError {
    pub fn from_body(body: &str) -> SetIdentityMailFromDomainError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => SetIdentityMailFromDomainError::Unknown(String::from(body)),
            },
            Err(_) => SetIdentityMailFromDomainError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for SetIdentityMailFromDomainError {
    fn from(err: XmlParseError) -> SetIdentityMailFromDomainError {
        let XmlParseError(message) = err;
        SetIdentityMailFromDomainError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for SetIdentityMailFromDomainError {
    fn from(err: CredentialsError) -> SetIdentityMailFromDomainError {
        SetIdentityMailFromDomainError::Credentials(err)
    }
}
impl From<HttpDispatchError> for SetIdentityMailFromDomainError {
    fn from(err: HttpDispatchError) -> SetIdentityMailFromDomainError {
        SetIdentityMailFromDomainError::HttpDispatch(err)
    }
}
impl From<io::Error> for SetIdentityMailFromDomainError {
    fn from(err: io::Error) -> SetIdentityMailFromDomainError {
        SetIdentityMailFromDomainError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for SetIdentityMailFromDomainError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SetIdentityMailFromDomainError {
    fn description(&self) -> &str {
        match *self {
            SetIdentityMailFromDomainError::Validation(ref cause) => cause,
            SetIdentityMailFromDomainError::Credentials(ref err) => err.description(),
            SetIdentityMailFromDomainError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            SetIdentityMailFromDomainError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by SetIdentityNotificationTopic
#[derive(Debug, PartialEq)]
pub enum SetIdentityNotificationTopicError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl SetIdentityNotificationTopicError {
    pub fn from_body(body: &str) -> SetIdentityNotificationTopicError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => SetIdentityNotificationTopicError::Unknown(String::from(body)),
            },
            Err(_) => SetIdentityNotificationTopicError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for SetIdentityNotificationTopicError {
    fn from(err: XmlParseError) -> SetIdentityNotificationTopicError {
        let XmlParseError(message) = err;
        SetIdentityNotificationTopicError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for SetIdentityNotificationTopicError {
    fn from(err: CredentialsError) -> SetIdentityNotificationTopicError {
        SetIdentityNotificationTopicError::Credentials(err)
    }
}
impl From<HttpDispatchError> for SetIdentityNotificationTopicError {
    fn from(err: HttpDispatchError) -> SetIdentityNotificationTopicError {
        SetIdentityNotificationTopicError::HttpDispatch(err)
    }
}
impl From<io::Error> for SetIdentityNotificationTopicError {
    fn from(err: io::Error) -> SetIdentityNotificationTopicError {
        SetIdentityNotificationTopicError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for SetIdentityNotificationTopicError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SetIdentityNotificationTopicError {
    fn description(&self) -> &str {
        match *self {
            SetIdentityNotificationTopicError::Validation(ref cause) => cause,
            SetIdentityNotificationTopicError::Credentials(ref err) => err.description(),
            SetIdentityNotificationTopicError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            SetIdentityNotificationTopicError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by SetReceiptRulePosition
#[derive(Debug, PartialEq)]
pub enum SetReceiptRulePositionError {
    /// <p>Indicates that the provided receipt rule does not exist.</p>
    RuleDoesNotExist(String),
    /// <p>Indicates that the provided receipt rule set does not exist.</p>
    RuleSetDoesNotExist(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl SetReceiptRulePositionError {
    pub fn from_body(body: &str) -> SetReceiptRulePositionError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "RuleDoesNotExist" => SetReceiptRulePositionError::RuleDoesNotExist(String::from(
                    parsed_error.message,
                )),
                "RuleSetDoesNotExist" => SetReceiptRulePositionError::RuleSetDoesNotExist(
                    String::from(parsed_error.message),
                ),
                _ => SetReceiptRulePositionError::Unknown(String::from(body)),
            },
            Err(_) => SetReceiptRulePositionError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for SetReceiptRulePositionError {
    fn from(err: XmlParseError) -> SetReceiptRulePositionError {
        let XmlParseError(message) = err;
        SetReceiptRulePositionError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for SetReceiptRulePositionError {
    fn from(err: CredentialsError) -> SetReceiptRulePositionError {
        SetReceiptRulePositionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for SetReceiptRulePositionError {
    fn from(err: HttpDispatchError) -> SetReceiptRulePositionError {
        SetReceiptRulePositionError::HttpDispatch(err)
    }
}
impl From<io::Error> for SetReceiptRulePositionError {
    fn from(err: io::Error) -> SetReceiptRulePositionError {
        SetReceiptRulePositionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for SetReceiptRulePositionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SetReceiptRulePositionError {
    fn description(&self) -> &str {
        match *self {
            SetReceiptRulePositionError::RuleDoesNotExist(ref cause) => cause,
            SetReceiptRulePositionError::RuleSetDoesNotExist(ref cause) => cause,
            SetReceiptRulePositionError::Validation(ref cause) => cause,
            SetReceiptRulePositionError::Credentials(ref err) => err.description(),
            SetReceiptRulePositionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            SetReceiptRulePositionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by TestRenderTemplate
#[derive(Debug, PartialEq)]
pub enum TestRenderTemplateError {
    /// <p>Indicates that one or more of the replacement values you provided is invalid. This error may occur when the TemplateData object contains invalid JSON.</p>
    InvalidRenderingParameter(String),
    /// <p>Indicates that one or more of the replacement values for the specified template was not specified. Ensure that the TemplateData object contains references to all of the replacement tags in the specified template.</p>
    MissingRenderingAttribute(String),
    /// <p>Indicates that the Template object you specified does not exist in your Amazon SES account.</p>
    TemplateDoesNotExist(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl TestRenderTemplateError {
    pub fn from_body(body: &str) -> TestRenderTemplateError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "InvalidRenderingParameter" => TestRenderTemplateError::InvalidRenderingParameter(
                    String::from(parsed_error.message),
                ),
                "MissingRenderingAttribute" => TestRenderTemplateError::MissingRenderingAttribute(
                    String::from(parsed_error.message),
                ),
                "TemplateDoesNotExist" => TestRenderTemplateError::TemplateDoesNotExist(
                    String::from(parsed_error.message),
                ),
                _ => TestRenderTemplateError::Unknown(String::from(body)),
            },
            Err(_) => TestRenderTemplateError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for TestRenderTemplateError {
    fn from(err: XmlParseError) -> TestRenderTemplateError {
        let XmlParseError(message) = err;
        TestRenderTemplateError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for TestRenderTemplateError {
    fn from(err: CredentialsError) -> TestRenderTemplateError {
        TestRenderTemplateError::Credentials(err)
    }
}
impl From<HttpDispatchError> for TestRenderTemplateError {
    fn from(err: HttpDispatchError) -> TestRenderTemplateError {
        TestRenderTemplateError::HttpDispatch(err)
    }
}
impl From<io::Error> for TestRenderTemplateError {
    fn from(err: io::Error) -> TestRenderTemplateError {
        TestRenderTemplateError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for TestRenderTemplateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for TestRenderTemplateError {
    fn description(&self) -> &str {
        match *self {
            TestRenderTemplateError::InvalidRenderingParameter(ref cause) => cause,
            TestRenderTemplateError::MissingRenderingAttribute(ref cause) => cause,
            TestRenderTemplateError::TemplateDoesNotExist(ref cause) => cause,
            TestRenderTemplateError::Validation(ref cause) => cause,
            TestRenderTemplateError::Credentials(ref err) => err.description(),
            TestRenderTemplateError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            TestRenderTemplateError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateAccountSendingEnabled
#[derive(Debug, PartialEq)]
pub enum UpdateAccountSendingEnabledError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateAccountSendingEnabledError {
    pub fn from_body(body: &str) -> UpdateAccountSendingEnabledError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => UpdateAccountSendingEnabledError::Unknown(String::from(body)),
            },
            Err(_) => UpdateAccountSendingEnabledError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for UpdateAccountSendingEnabledError {
    fn from(err: XmlParseError) -> UpdateAccountSendingEnabledError {
        let XmlParseError(message) = err;
        UpdateAccountSendingEnabledError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for UpdateAccountSendingEnabledError {
    fn from(err: CredentialsError) -> UpdateAccountSendingEnabledError {
        UpdateAccountSendingEnabledError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateAccountSendingEnabledError {
    fn from(err: HttpDispatchError) -> UpdateAccountSendingEnabledError {
        UpdateAccountSendingEnabledError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateAccountSendingEnabledError {
    fn from(err: io::Error) -> UpdateAccountSendingEnabledError {
        UpdateAccountSendingEnabledError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateAccountSendingEnabledError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateAccountSendingEnabledError {
    fn description(&self) -> &str {
        match *self {
            UpdateAccountSendingEnabledError::Validation(ref cause) => cause,
            UpdateAccountSendingEnabledError::Credentials(ref err) => err.description(),
            UpdateAccountSendingEnabledError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateAccountSendingEnabledError::Unknown(ref cause) => cause,
        }
    }
}
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateConfigurationSetEventDestinationError {
    pub fn from_body(body: &str) -> UpdateConfigurationSetEventDestinationError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ConfigurationSetDoesNotExist" => {
                    UpdateConfigurationSetEventDestinationError::ConfigurationSetDoesNotExist(
                        String::from(parsed_error.message),
                    )
                }
                "EventDestinationDoesNotExist" => {
                    UpdateConfigurationSetEventDestinationError::EventDestinationDoesNotExist(
                        String::from(parsed_error.message),
                    )
                }
                "InvalidCloudWatchDestination" => {
                    UpdateConfigurationSetEventDestinationError::InvalidCloudWatchDestination(
                        String::from(parsed_error.message),
                    )
                }
                "InvalidFirehoseDestination" => {
                    UpdateConfigurationSetEventDestinationError::InvalidFirehoseDestination(
                        String::from(parsed_error.message),
                    )
                }
                "InvalidSNSDestination" => {
                    UpdateConfigurationSetEventDestinationError::InvalidSNSDestination(
                        String::from(parsed_error.message),
                    )
                }
                _ => UpdateConfigurationSetEventDestinationError::Unknown(String::from(body)),
            },
            Err(_) => UpdateConfigurationSetEventDestinationError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for UpdateConfigurationSetEventDestinationError {
    fn from(err: XmlParseError) -> UpdateConfigurationSetEventDestinationError {
        let XmlParseError(message) = err;
        UpdateConfigurationSetEventDestinationError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for UpdateConfigurationSetEventDestinationError {
    fn from(err: CredentialsError) -> UpdateConfigurationSetEventDestinationError {
        UpdateConfigurationSetEventDestinationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateConfigurationSetEventDestinationError {
    fn from(err: HttpDispatchError) -> UpdateConfigurationSetEventDestinationError {
        UpdateConfigurationSetEventDestinationError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateConfigurationSetEventDestinationError {
    fn from(err: io::Error) -> UpdateConfigurationSetEventDestinationError {
        UpdateConfigurationSetEventDestinationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateConfigurationSetEventDestinationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateConfigurationSetEventDestinationError {
    fn description(&self) -> &str {
        match *self {
            UpdateConfigurationSetEventDestinationError::ConfigurationSetDoesNotExist(
                ref cause,
            ) => cause,
            UpdateConfigurationSetEventDestinationError::EventDestinationDoesNotExist(
                ref cause,
            ) => cause,
            UpdateConfigurationSetEventDestinationError::InvalidCloudWatchDestination(
                ref cause,
            ) => cause,
            UpdateConfigurationSetEventDestinationError::InvalidFirehoseDestination(ref cause) => {
                cause
            }
            UpdateConfigurationSetEventDestinationError::InvalidSNSDestination(ref cause) => cause,
            UpdateConfigurationSetEventDestinationError::Validation(ref cause) => cause,
            UpdateConfigurationSetEventDestinationError::Credentials(ref err) => err.description(),
            UpdateConfigurationSetEventDestinationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateConfigurationSetEventDestinationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateConfigurationSetReputationMetricsEnabled
#[derive(Debug, PartialEq)]
pub enum UpdateConfigurationSetReputationMetricsEnabledError {
    /// <p>Indicates that the configuration set does not exist.</p>
    ConfigurationSetDoesNotExist(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateConfigurationSetReputationMetricsEnabledError {
    pub fn from_body(body: &str) -> UpdateConfigurationSetReputationMetricsEnabledError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
                            Ok(parsed_error) => {
                                match &parsed_error.code[..] {
                                    "ConfigurationSetDoesNotExist" => UpdateConfigurationSetReputationMetricsEnabledError::ConfigurationSetDoesNotExist(String::from(parsed_error.message)),_ => UpdateConfigurationSetReputationMetricsEnabledError::Unknown(String::from(body))
                                }
                           },
                           Err(_) => UpdateConfigurationSetReputationMetricsEnabledError::Unknown(body.to_string())
                       }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for UpdateConfigurationSetReputationMetricsEnabledError {
    fn from(err: XmlParseError) -> UpdateConfigurationSetReputationMetricsEnabledError {
        let XmlParseError(message) = err;
        UpdateConfigurationSetReputationMetricsEnabledError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for UpdateConfigurationSetReputationMetricsEnabledError {
    fn from(err: CredentialsError) -> UpdateConfigurationSetReputationMetricsEnabledError {
        UpdateConfigurationSetReputationMetricsEnabledError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateConfigurationSetReputationMetricsEnabledError {
    fn from(err: HttpDispatchError) -> UpdateConfigurationSetReputationMetricsEnabledError {
        UpdateConfigurationSetReputationMetricsEnabledError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateConfigurationSetReputationMetricsEnabledError {
    fn from(err: io::Error) -> UpdateConfigurationSetReputationMetricsEnabledError {
        UpdateConfigurationSetReputationMetricsEnabledError::HttpDispatch(HttpDispatchError::from(
            err,
        ))
    }
}
impl fmt::Display for UpdateConfigurationSetReputationMetricsEnabledError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateConfigurationSetReputationMetricsEnabledError {
    fn description(&self) -> &str {
        match *self {
            UpdateConfigurationSetReputationMetricsEnabledError::ConfigurationSetDoesNotExist(
                ref cause,
            ) => cause,
            UpdateConfigurationSetReputationMetricsEnabledError::Validation(ref cause) => cause,
            UpdateConfigurationSetReputationMetricsEnabledError::Credentials(ref err) => {
                err.description()
            }
            UpdateConfigurationSetReputationMetricsEnabledError::HttpDispatch(
                ref dispatch_error,
            ) => dispatch_error.description(),
            UpdateConfigurationSetReputationMetricsEnabledError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateConfigurationSetSendingEnabled
#[derive(Debug, PartialEq)]
pub enum UpdateConfigurationSetSendingEnabledError {
    /// <p>Indicates that the configuration set does not exist.</p>
    ConfigurationSetDoesNotExist(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateConfigurationSetSendingEnabledError {
    pub fn from_body(body: &str) -> UpdateConfigurationSetSendingEnabledError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ConfigurationSetDoesNotExist" => {
                    UpdateConfigurationSetSendingEnabledError::ConfigurationSetDoesNotExist(
                        String::from(parsed_error.message),
                    )
                }
                _ => UpdateConfigurationSetSendingEnabledError::Unknown(String::from(body)),
            },
            Err(_) => UpdateConfigurationSetSendingEnabledError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for UpdateConfigurationSetSendingEnabledError {
    fn from(err: XmlParseError) -> UpdateConfigurationSetSendingEnabledError {
        let XmlParseError(message) = err;
        UpdateConfigurationSetSendingEnabledError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for UpdateConfigurationSetSendingEnabledError {
    fn from(err: CredentialsError) -> UpdateConfigurationSetSendingEnabledError {
        UpdateConfigurationSetSendingEnabledError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateConfigurationSetSendingEnabledError {
    fn from(err: HttpDispatchError) -> UpdateConfigurationSetSendingEnabledError {
        UpdateConfigurationSetSendingEnabledError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateConfigurationSetSendingEnabledError {
    fn from(err: io::Error) -> UpdateConfigurationSetSendingEnabledError {
        UpdateConfigurationSetSendingEnabledError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateConfigurationSetSendingEnabledError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateConfigurationSetSendingEnabledError {
    fn description(&self) -> &str {
        match *self {
            UpdateConfigurationSetSendingEnabledError::ConfigurationSetDoesNotExist(ref cause) => {
                cause
            }
            UpdateConfigurationSetSendingEnabledError::Validation(ref cause) => cause,
            UpdateConfigurationSetSendingEnabledError::Credentials(ref err) => err.description(),
            UpdateConfigurationSetSendingEnabledError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateConfigurationSetSendingEnabledError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateConfigurationSetTrackingOptions
#[derive(Debug, PartialEq)]
pub enum UpdateConfigurationSetTrackingOptionsError {
    /// <p>Indicates that the configuration set does not exist.</p>
    ConfigurationSetDoesNotExist(String),
    /// <p><p>Indicates that the custom domain to be used for open and click tracking redirects is invalid. This error appears most often in the following situations:</p> <ul> <li> <p>When the tracking domain you specified is not verified in Amazon SES.</p> </li> <li> <p>When the tracking domain you specified is not a valid domain or subdomain.</p> </li> </ul></p>
    InvalidTrackingOptions(String),
    /// <p>Indicates that the TrackingOptions object you specified does not exist.</p>
    TrackingOptionsDoesNotExist(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateConfigurationSetTrackingOptionsError {
    pub fn from_body(body: &str) -> UpdateConfigurationSetTrackingOptionsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ConfigurationSetDoesNotExist" => {
                    UpdateConfigurationSetTrackingOptionsError::ConfigurationSetDoesNotExist(
                        String::from(parsed_error.message),
                    )
                }
                "InvalidTrackingOptions" => {
                    UpdateConfigurationSetTrackingOptionsError::InvalidTrackingOptions(
                        String::from(parsed_error.message),
                    )
                }
                "TrackingOptionsDoesNotExistException" => {
                    UpdateConfigurationSetTrackingOptionsError::TrackingOptionsDoesNotExist(
                        String::from(parsed_error.message),
                    )
                }
                _ => UpdateConfigurationSetTrackingOptionsError::Unknown(String::from(body)),
            },
            Err(_) => UpdateConfigurationSetTrackingOptionsError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for UpdateConfigurationSetTrackingOptionsError {
    fn from(err: XmlParseError) -> UpdateConfigurationSetTrackingOptionsError {
        let XmlParseError(message) = err;
        UpdateConfigurationSetTrackingOptionsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for UpdateConfigurationSetTrackingOptionsError {
    fn from(err: CredentialsError) -> UpdateConfigurationSetTrackingOptionsError {
        UpdateConfigurationSetTrackingOptionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateConfigurationSetTrackingOptionsError {
    fn from(err: HttpDispatchError) -> UpdateConfigurationSetTrackingOptionsError {
        UpdateConfigurationSetTrackingOptionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateConfigurationSetTrackingOptionsError {
    fn from(err: io::Error) -> UpdateConfigurationSetTrackingOptionsError {
        UpdateConfigurationSetTrackingOptionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateConfigurationSetTrackingOptionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateConfigurationSetTrackingOptionsError {
    fn description(&self) -> &str {
        match *self {
            UpdateConfigurationSetTrackingOptionsError::ConfigurationSetDoesNotExist(ref cause) => {
                cause
            }
            UpdateConfigurationSetTrackingOptionsError::InvalidTrackingOptions(ref cause) => cause,
            UpdateConfigurationSetTrackingOptionsError::TrackingOptionsDoesNotExist(ref cause) => {
                cause
            }
            UpdateConfigurationSetTrackingOptionsError::Validation(ref cause) => cause,
            UpdateConfigurationSetTrackingOptionsError::Credentials(ref err) => err.description(),
            UpdateConfigurationSetTrackingOptionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateConfigurationSetTrackingOptionsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateCustomVerificationEmailTemplate
#[derive(Debug, PartialEq)]
pub enum UpdateCustomVerificationEmailTemplateError {
    /// <p>Indicates that custom verification email template provided content is invalid.</p>
    CustomVerificationEmailInvalidContent(String),
    /// <p>Indicates that a custom verification email template with the name you specified does not exist.</p>
    CustomVerificationEmailTemplateDoesNotExist(String),
    /// <p>Indicates that the sender address specified for a custom verification email is not verified, and is therefore not eligible to send the custom verification email. </p>
    FromEmailAddressNotVerified(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateCustomVerificationEmailTemplateError {
    pub fn from_body(body: &str) -> UpdateCustomVerificationEmailTemplateError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
                            Ok(parsed_error) => {
                                match &parsed_error.code[..] {
                                    "CustomVerificationEmailInvalidContent" => UpdateCustomVerificationEmailTemplateError::CustomVerificationEmailInvalidContent(String::from(parsed_error.message)),"CustomVerificationEmailTemplateDoesNotExist" => UpdateCustomVerificationEmailTemplateError::CustomVerificationEmailTemplateDoesNotExist(String::from(parsed_error.message)),"FromEmailAddressNotVerified" => UpdateCustomVerificationEmailTemplateError::FromEmailAddressNotVerified(String::from(parsed_error.message)),_ => UpdateCustomVerificationEmailTemplateError::Unknown(String::from(body))
                                }
                           },
                           Err(_) => UpdateCustomVerificationEmailTemplateError::Unknown(body.to_string())
                       }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for UpdateCustomVerificationEmailTemplateError {
    fn from(err: XmlParseError) -> UpdateCustomVerificationEmailTemplateError {
        let XmlParseError(message) = err;
        UpdateCustomVerificationEmailTemplateError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for UpdateCustomVerificationEmailTemplateError {
    fn from(err: CredentialsError) -> UpdateCustomVerificationEmailTemplateError {
        UpdateCustomVerificationEmailTemplateError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateCustomVerificationEmailTemplateError {
    fn from(err: HttpDispatchError) -> UpdateCustomVerificationEmailTemplateError {
        UpdateCustomVerificationEmailTemplateError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateCustomVerificationEmailTemplateError {
    fn from(err: io::Error) -> UpdateCustomVerificationEmailTemplateError {
        UpdateCustomVerificationEmailTemplateError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateCustomVerificationEmailTemplateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateCustomVerificationEmailTemplateError {
    fn description(&self) -> &str {
        match *self {
                            UpdateCustomVerificationEmailTemplateError::CustomVerificationEmailInvalidContent(ref cause) => cause,
UpdateCustomVerificationEmailTemplateError::CustomVerificationEmailTemplateDoesNotExist(ref cause) => cause,
UpdateCustomVerificationEmailTemplateError::FromEmailAddressNotVerified(ref cause) => cause,
UpdateCustomVerificationEmailTemplateError::Validation(ref cause) => cause,
UpdateCustomVerificationEmailTemplateError::Credentials(ref err) => err.description(),
UpdateCustomVerificationEmailTemplateError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
UpdateCustomVerificationEmailTemplateError::Unknown(ref cause) => cause
                        }
    }
}
/// Errors returned by UpdateReceiptRule
#[derive(Debug, PartialEq)]
pub enum UpdateReceiptRuleError {
    /// <p>Indicates that the provided AWS Lambda function is invalid, or that Amazon SES could not execute the provided function, possibly due to permissions issues. For information about giving permissions, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-permissions.html">Amazon SES Developer Guide</a>.</p>
    InvalidLambdaFunction(String),
    /// <p>Indicates that the provided Amazon S3 bucket or AWS KMS encryption key is invalid, or that Amazon SES could not publish to the bucket, possibly due to permissions issues. For information about giving permissions, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-permissions.html">Amazon SES Developer Guide</a>.</p>
    InvalidS3Configuration(String),
    /// <p>Indicates that the provided Amazon SNS topic is invalid, or that Amazon SES could not publish to the topic, possibly due to permissions issues. For information about giving permissions, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-permissions.html">Amazon SES Developer Guide</a>.</p>
    InvalidSnsTopic(String),
    /// <p>Indicates that a resource could not be created because of service limits. For a list of Amazon SES limits, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/limits.html">Amazon SES Developer Guide</a>.</p>
    LimitExceeded(String),
    /// <p>Indicates that the provided receipt rule does not exist.</p>
    RuleDoesNotExist(String),
    /// <p>Indicates that the provided receipt rule set does not exist.</p>
    RuleSetDoesNotExist(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateReceiptRuleError {
    pub fn from_body(body: &str) -> UpdateReceiptRuleError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "InvalidLambdaFunction" => UpdateReceiptRuleError::InvalidLambdaFunction(
                    String::from(parsed_error.message),
                ),
                "InvalidS3Configuration" => UpdateReceiptRuleError::InvalidS3Configuration(
                    String::from(parsed_error.message),
                ),
                "InvalidSnsTopic" => {
                    UpdateReceiptRuleError::InvalidSnsTopic(String::from(parsed_error.message))
                }
                "LimitExceeded" => {
                    UpdateReceiptRuleError::LimitExceeded(String::from(parsed_error.message))
                }
                "RuleDoesNotExist" => {
                    UpdateReceiptRuleError::RuleDoesNotExist(String::from(parsed_error.message))
                }
                "RuleSetDoesNotExist" => {
                    UpdateReceiptRuleError::RuleSetDoesNotExist(String::from(parsed_error.message))
                }
                _ => UpdateReceiptRuleError::Unknown(String::from(body)),
            },
            Err(_) => UpdateReceiptRuleError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for UpdateReceiptRuleError {
    fn from(err: XmlParseError) -> UpdateReceiptRuleError {
        let XmlParseError(message) = err;
        UpdateReceiptRuleError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for UpdateReceiptRuleError {
    fn from(err: CredentialsError) -> UpdateReceiptRuleError {
        UpdateReceiptRuleError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateReceiptRuleError {
    fn from(err: HttpDispatchError) -> UpdateReceiptRuleError {
        UpdateReceiptRuleError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateReceiptRuleError {
    fn from(err: io::Error) -> UpdateReceiptRuleError {
        UpdateReceiptRuleError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateReceiptRuleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateReceiptRuleError {
    fn description(&self) -> &str {
        match *self {
            UpdateReceiptRuleError::InvalidLambdaFunction(ref cause) => cause,
            UpdateReceiptRuleError::InvalidS3Configuration(ref cause) => cause,
            UpdateReceiptRuleError::InvalidSnsTopic(ref cause) => cause,
            UpdateReceiptRuleError::LimitExceeded(ref cause) => cause,
            UpdateReceiptRuleError::RuleDoesNotExist(ref cause) => cause,
            UpdateReceiptRuleError::RuleSetDoesNotExist(ref cause) => cause,
            UpdateReceiptRuleError::Validation(ref cause) => cause,
            UpdateReceiptRuleError::Credentials(ref err) => err.description(),
            UpdateReceiptRuleError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateReceiptRuleError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateTemplate
#[derive(Debug, PartialEq)]
pub enum UpdateTemplateError {
    /// <p>Indicates that a template could not be created because it contained invalid JSON.</p>
    InvalidTemplate(String),
    /// <p>Indicates that the Template object you specified does not exist in your Amazon SES account.</p>
    TemplateDoesNotExist(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateTemplateError {
    pub fn from_body(body: &str) -> UpdateTemplateError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "InvalidTemplate" => {
                    UpdateTemplateError::InvalidTemplate(String::from(parsed_error.message))
                }
                "TemplateDoesNotExist" => {
                    UpdateTemplateError::TemplateDoesNotExist(String::from(parsed_error.message))
                }
                _ => UpdateTemplateError::Unknown(String::from(body)),
            },
            Err(_) => UpdateTemplateError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for UpdateTemplateError {
    fn from(err: XmlParseError) -> UpdateTemplateError {
        let XmlParseError(message) = err;
        UpdateTemplateError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for UpdateTemplateError {
    fn from(err: CredentialsError) -> UpdateTemplateError {
        UpdateTemplateError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateTemplateError {
    fn from(err: HttpDispatchError) -> UpdateTemplateError {
        UpdateTemplateError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateTemplateError {
    fn from(err: io::Error) -> UpdateTemplateError {
        UpdateTemplateError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateTemplateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateTemplateError {
    fn description(&self) -> &str {
        match *self {
            UpdateTemplateError::InvalidTemplate(ref cause) => cause,
            UpdateTemplateError::TemplateDoesNotExist(ref cause) => cause,
            UpdateTemplateError::Validation(ref cause) => cause,
            UpdateTemplateError::Credentials(ref err) => err.description(),
            UpdateTemplateError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateTemplateError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by VerifyDomainDkim
#[derive(Debug, PartialEq)]
pub enum VerifyDomainDkimError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl VerifyDomainDkimError {
    pub fn from_body(body: &str) -> VerifyDomainDkimError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => VerifyDomainDkimError::Unknown(String::from(body)),
            },
            Err(_) => VerifyDomainDkimError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for VerifyDomainDkimError {
    fn from(err: XmlParseError) -> VerifyDomainDkimError {
        let XmlParseError(message) = err;
        VerifyDomainDkimError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for VerifyDomainDkimError {
    fn from(err: CredentialsError) -> VerifyDomainDkimError {
        VerifyDomainDkimError::Credentials(err)
    }
}
impl From<HttpDispatchError> for VerifyDomainDkimError {
    fn from(err: HttpDispatchError) -> VerifyDomainDkimError {
        VerifyDomainDkimError::HttpDispatch(err)
    }
}
impl From<io::Error> for VerifyDomainDkimError {
    fn from(err: io::Error) -> VerifyDomainDkimError {
        VerifyDomainDkimError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for VerifyDomainDkimError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for VerifyDomainDkimError {
    fn description(&self) -> &str {
        match *self {
            VerifyDomainDkimError::Validation(ref cause) => cause,
            VerifyDomainDkimError::Credentials(ref err) => err.description(),
            VerifyDomainDkimError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            VerifyDomainDkimError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by VerifyDomainIdentity
#[derive(Debug, PartialEq)]
pub enum VerifyDomainIdentityError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl VerifyDomainIdentityError {
    pub fn from_body(body: &str) -> VerifyDomainIdentityError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => VerifyDomainIdentityError::Unknown(String::from(body)),
            },
            Err(_) => VerifyDomainIdentityError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for VerifyDomainIdentityError {
    fn from(err: XmlParseError) -> VerifyDomainIdentityError {
        let XmlParseError(message) = err;
        VerifyDomainIdentityError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for VerifyDomainIdentityError {
    fn from(err: CredentialsError) -> VerifyDomainIdentityError {
        VerifyDomainIdentityError::Credentials(err)
    }
}
impl From<HttpDispatchError> for VerifyDomainIdentityError {
    fn from(err: HttpDispatchError) -> VerifyDomainIdentityError {
        VerifyDomainIdentityError::HttpDispatch(err)
    }
}
impl From<io::Error> for VerifyDomainIdentityError {
    fn from(err: io::Error) -> VerifyDomainIdentityError {
        VerifyDomainIdentityError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for VerifyDomainIdentityError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for VerifyDomainIdentityError {
    fn description(&self) -> &str {
        match *self {
            VerifyDomainIdentityError::Validation(ref cause) => cause,
            VerifyDomainIdentityError::Credentials(ref err) => err.description(),
            VerifyDomainIdentityError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            VerifyDomainIdentityError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by VerifyEmailAddress
#[derive(Debug, PartialEq)]
pub enum VerifyEmailAddressError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl VerifyEmailAddressError {
    pub fn from_body(body: &str) -> VerifyEmailAddressError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => VerifyEmailAddressError::Unknown(String::from(body)),
            },
            Err(_) => VerifyEmailAddressError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for VerifyEmailAddressError {
    fn from(err: XmlParseError) -> VerifyEmailAddressError {
        let XmlParseError(message) = err;
        VerifyEmailAddressError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for VerifyEmailAddressError {
    fn from(err: CredentialsError) -> VerifyEmailAddressError {
        VerifyEmailAddressError::Credentials(err)
    }
}
impl From<HttpDispatchError> for VerifyEmailAddressError {
    fn from(err: HttpDispatchError) -> VerifyEmailAddressError {
        VerifyEmailAddressError::HttpDispatch(err)
    }
}
impl From<io::Error> for VerifyEmailAddressError {
    fn from(err: io::Error) -> VerifyEmailAddressError {
        VerifyEmailAddressError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for VerifyEmailAddressError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for VerifyEmailAddressError {
    fn description(&self) -> &str {
        match *self {
            VerifyEmailAddressError::Validation(ref cause) => cause,
            VerifyEmailAddressError::Credentials(ref err) => err.description(),
            VerifyEmailAddressError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            VerifyEmailAddressError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by VerifyEmailIdentity
#[derive(Debug, PartialEq)]
pub enum VerifyEmailIdentityError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl VerifyEmailIdentityError {
    pub fn from_body(body: &str) -> VerifyEmailIdentityError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => VerifyEmailIdentityError::Unknown(String::from(body)),
            },
            Err(_) => VerifyEmailIdentityError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for VerifyEmailIdentityError {
    fn from(err: XmlParseError) -> VerifyEmailIdentityError {
        let XmlParseError(message) = err;
        VerifyEmailIdentityError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for VerifyEmailIdentityError {
    fn from(err: CredentialsError) -> VerifyEmailIdentityError {
        VerifyEmailIdentityError::Credentials(err)
    }
}
impl From<HttpDispatchError> for VerifyEmailIdentityError {
    fn from(err: HttpDispatchError) -> VerifyEmailIdentityError {
        VerifyEmailIdentityError::HttpDispatch(err)
    }
}
impl From<io::Error> for VerifyEmailIdentityError {
    fn from(err: io::Error) -> VerifyEmailIdentityError {
        VerifyEmailIdentityError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for VerifyEmailIdentityError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for VerifyEmailIdentityError {
    fn description(&self) -> &str {
        match *self {
            VerifyEmailIdentityError::Validation(ref cause) => cause,
            VerifyEmailIdentityError::Credentials(ref err) => err.description(),
            VerifyEmailIdentityError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            VerifyEmailIdentityError::Unknown(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the Amazon SES API. Amazon SES clients implement this trait.
pub trait Ses {
    /// <p>Creates a receipt rule set by cloning an existing one. All receipt rules and configurations are copied to the new receipt rule set and are completely independent of the source rule set.</p> <p>For information about setting up rule sets, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-receipt-rule-set.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    fn clone_receipt_rule_set(
        &self,
        input: CloneReceiptRuleSetRequest,
    ) -> RusotoFuture<CloneReceiptRuleSetResponse, CloneReceiptRuleSetError>;

    /// <p>Creates a configuration set.</p> <p>Configuration sets enable you to publish email sending events. For information about using configuration sets, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/monitor-sending-activity.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    fn create_configuration_set(
        &self,
        input: CreateConfigurationSetRequest,
    ) -> RusotoFuture<CreateConfigurationSetResponse, CreateConfigurationSetError>;

    /// <p>Creates a configuration set event destination.</p> <note> <p>When you create or update an event destination, you must provide one, and only one, destination. The destination can be Amazon CloudWatch, Amazon Kinesis Firehose, or Amazon Simple Notification Service (Amazon SNS).</p> </note> <p>An event destination is the AWS service to which Amazon SES publishes the email sending events associated with a configuration set. For information about using configuration sets, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/monitor-sending-activity.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    fn create_configuration_set_event_destination(
        &self,
        input: CreateConfigurationSetEventDestinationRequest,
    ) -> RusotoFuture<
        CreateConfigurationSetEventDestinationResponse,
        CreateConfigurationSetEventDestinationError,
    >;

    /// <p>Creates an association between a configuration set and a custom domain for open and click event tracking. </p> <p>By default, images and links used for tracking open and click events are hosted on domains operated by Amazon SES. You can configure a subdomain of your own to handle these events. For information about using configuration sets, see <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/configure-custom-open-click-domains.html">Configuring Custom Domains to Handle Open and Click Tracking</a> in the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/Welcome.html">Amazon SES Developer Guide</a>.</p>
    fn create_configuration_set_tracking_options(
        &self,
        input: CreateConfigurationSetTrackingOptionsRequest,
    ) -> RusotoFuture<
        CreateConfigurationSetTrackingOptionsResponse,
        CreateConfigurationSetTrackingOptionsError,
    >;

    /// <p>Creates a new custom verification email template.</p> <p>For more information about custom verification email templates, see <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/custom-verification-emails.html">Using Custom Verification Email Templates</a> in the <i>Amazon SES Developer Guide</i>.</p> <p>You can execute this operation no more than once per second.</p>
    fn create_custom_verification_email_template(
        &self,
        input: CreateCustomVerificationEmailTemplateRequest,
    ) -> RusotoFuture<(), CreateCustomVerificationEmailTemplateError>;

    /// <p>Creates a new IP address filter.</p> <p>For information about setting up IP address filters, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-ip-filters.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    fn create_receipt_filter(
        &self,
        input: CreateReceiptFilterRequest,
    ) -> RusotoFuture<CreateReceiptFilterResponse, CreateReceiptFilterError>;

    /// <p>Creates a receipt rule.</p> <p>For information about setting up receipt rules, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-receipt-rules.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    fn create_receipt_rule(
        &self,
        input: CreateReceiptRuleRequest,
    ) -> RusotoFuture<CreateReceiptRuleResponse, CreateReceiptRuleError>;

    /// <p>Creates an empty receipt rule set.</p> <p>For information about setting up receipt rule sets, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-receipt-rule-set.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    fn create_receipt_rule_set(
        &self,
        input: CreateReceiptRuleSetRequest,
    ) -> RusotoFuture<CreateReceiptRuleSetResponse, CreateReceiptRuleSetError>;

    /// <p>Creates an email template. Email templates enable you to send personalized email to one or more destinations in a single API operation. For more information, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/send-personalized-email-api.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    fn create_template(
        &self,
        input: CreateTemplateRequest,
    ) -> RusotoFuture<CreateTemplateResponse, CreateTemplateError>;

    /// <p>Deletes a configuration set. Configuration sets enable you to publish email sending events. For information about using configuration sets, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/monitor-sending-activity.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    fn delete_configuration_set(
        &self,
        input: DeleteConfigurationSetRequest,
    ) -> RusotoFuture<DeleteConfigurationSetResponse, DeleteConfigurationSetError>;

    /// <p>Deletes a configuration set event destination. Configuration set event destinations are associated with configuration sets, which enable you to publish email sending events. For information about using configuration sets, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/monitor-sending-activity.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    fn delete_configuration_set_event_destination(
        &self,
        input: DeleteConfigurationSetEventDestinationRequest,
    ) -> RusotoFuture<
        DeleteConfigurationSetEventDestinationResponse,
        DeleteConfigurationSetEventDestinationError,
    >;

    /// <p><p>Deletes an association between a configuration set and a custom domain for open and click event tracking.</p> <p>By default, images and links used for tracking open and click events are hosted on domains operated by Amazon SES. You can configure a subdomain of your own to handle these events. For information about using configuration sets, see <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/configure-custom-open-click-domains.html">Configuring Custom Domains to Handle Open and Click Tracking</a> in the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/Welcome.html">Amazon SES Developer Guide</a>.</p> <note> <p>Deleting this kind of association will result in emails sent using the specified configuration set to capture open and click events using the standard, Amazon SES-operated domains.</p> </note></p>
    fn delete_configuration_set_tracking_options(
        &self,
        input: DeleteConfigurationSetTrackingOptionsRequest,
    ) -> RusotoFuture<
        DeleteConfigurationSetTrackingOptionsResponse,
        DeleteConfigurationSetTrackingOptionsError,
    >;

    /// <p>Deletes an existing custom verification email template. </p> <p>For more information about custom verification email templates, see <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/custom-verification-emails.html">Using Custom Verification Email Templates</a> in the <i>Amazon SES Developer Guide</i>.</p> <p>You can execute this operation no more than once per second.</p>
    fn delete_custom_verification_email_template(
        &self,
        input: DeleteCustomVerificationEmailTemplateRequest,
    ) -> RusotoFuture<(), DeleteCustomVerificationEmailTemplateError>;

    /// <p>Deletes the specified identity (an email address or a domain) from the list of verified identities.</p> <p>You can execute this operation no more than once per second.</p>
    fn delete_identity(
        &self,
        input: DeleteIdentityRequest,
    ) -> RusotoFuture<DeleteIdentityResponse, DeleteIdentityError>;

    /// <p>Deletes the specified sending authorization policy for the given identity (an email address or a domain). This API returns successfully even if a policy with the specified name does not exist.</p> <note> <p>This API is for the identity owner only. If you have not verified the identity, this API will return an error.</p> </note> <p>Sending authorization is a feature that enables an identity owner to authorize other senders to use its identities. For information about using sending authorization, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/sending-authorization.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    fn delete_identity_policy(
        &self,
        input: DeleteIdentityPolicyRequest,
    ) -> RusotoFuture<DeleteIdentityPolicyResponse, DeleteIdentityPolicyError>;

    /// <p>Deletes the specified IP address filter.</p> <p>For information about managing IP address filters, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-managing-ip-filters.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    fn delete_receipt_filter(
        &self,
        input: DeleteReceiptFilterRequest,
    ) -> RusotoFuture<DeleteReceiptFilterResponse, DeleteReceiptFilterError>;

    /// <p>Deletes the specified receipt rule.</p> <p>For information about managing receipt rules, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-managing-receipt-rules.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    fn delete_receipt_rule(
        &self,
        input: DeleteReceiptRuleRequest,
    ) -> RusotoFuture<DeleteReceiptRuleResponse, DeleteReceiptRuleError>;

    /// <p>Deletes the specified receipt rule set and all of the receipt rules it contains.</p> <note> <p>The currently active rule set cannot be deleted.</p> </note> <p>For information about managing receipt rule sets, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-managing-receipt-rule-sets.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    fn delete_receipt_rule_set(
        &self,
        input: DeleteReceiptRuleSetRequest,
    ) -> RusotoFuture<DeleteReceiptRuleSetResponse, DeleteReceiptRuleSetError>;

    /// <p>Deletes an email template.</p> <p>You can execute this operation no more than once per second.</p>
    fn delete_template(
        &self,
        input: DeleteTemplateRequest,
    ) -> RusotoFuture<DeleteTemplateResponse, DeleteTemplateError>;

    /// <p>Deprecated. Use the <code>DeleteIdentity</code> operation to delete email addresses and domains.</p>
    fn delete_verified_email_address(
        &self,
        input: DeleteVerifiedEmailAddressRequest,
    ) -> RusotoFuture<(), DeleteVerifiedEmailAddressError>;

    /// <p>Returns the metadata and receipt rules for the receipt rule set that is currently active.</p> <p>For information about setting up receipt rule sets, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-receipt-rule-set.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    fn describe_active_receipt_rule_set(
        &self,
        input: DescribeActiveReceiptRuleSetRequest,
    ) -> RusotoFuture<DescribeActiveReceiptRuleSetResponse, DescribeActiveReceiptRuleSetError>;

    /// <p>Returns the details of the specified configuration set. For information about using configuration sets, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/monitor-sending-activity.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    fn describe_configuration_set(
        &self,
        input: DescribeConfigurationSetRequest,
    ) -> RusotoFuture<DescribeConfigurationSetResponse, DescribeConfigurationSetError>;

    /// <p>Returns the details of the specified receipt rule.</p> <p>For information about setting up receipt rules, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-receipt-rules.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    fn describe_receipt_rule(
        &self,
        input: DescribeReceiptRuleRequest,
    ) -> RusotoFuture<DescribeReceiptRuleResponse, DescribeReceiptRuleError>;

    /// <p>Returns the details of the specified receipt rule set.</p> <p>For information about managing receipt rule sets, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-managing-receipt-rule-sets.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    fn describe_receipt_rule_set(
        &self,
        input: DescribeReceiptRuleSetRequest,
    ) -> RusotoFuture<DescribeReceiptRuleSetResponse, DescribeReceiptRuleSetError>;

    /// <p>Returns the email sending status of the Amazon SES account.</p> <p>You can execute this operation no more than once per second.</p>
    fn get_account_sending_enabled(
        &self,
    ) -> RusotoFuture<GetAccountSendingEnabledResponse, GetAccountSendingEnabledError>;

    /// <p>Returns the custom email verification template for the template name you specify.</p> <p>For more information about custom verification email templates, see <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/custom-verification-emails.html">Using Custom Verification Email Templates</a> in the <i>Amazon SES Developer Guide</i>.</p> <p>You can execute this operation no more than once per second.</p>
    fn get_custom_verification_email_template(
        &self,
        input: GetCustomVerificationEmailTemplateRequest,
    ) -> RusotoFuture<
        GetCustomVerificationEmailTemplateResponse,
        GetCustomVerificationEmailTemplateError,
    >;

    /// <p>Returns the current status of Easy DKIM signing for an entity. For domain name identities, this operation also returns the DKIM tokens that are required for Easy DKIM signing, and whether Amazon SES has successfully verified that these tokens have been published.</p> <p>This operation takes a list of identities as input and returns the following information for each:</p> <ul> <li> <p>Whether Easy DKIM signing is enabled or disabled.</p> </li> <li> <p>A set of DKIM tokens that represent the identity. If the identity is an email address, the tokens represent the domain of that address.</p> </li> <li> <p>Whether Amazon SES has successfully verified the DKIM tokens published in the domain's DNS. This information is only returned for domain name identities, not for email addresses.</p> </li> </ul> <p>This operation is throttled at one request per second and can only get DKIM attributes for up to 100 identities at a time.</p> <p>For more information about creating DNS records using DKIM tokens, go to the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/easy-dkim-dns-records.html">Amazon SES Developer Guide</a>.</p>
    fn get_identity_dkim_attributes(
        &self,
        input: GetIdentityDkimAttributesRequest,
    ) -> RusotoFuture<GetIdentityDkimAttributesResponse, GetIdentityDkimAttributesError>;

    /// <p>Returns the custom MAIL FROM attributes for a list of identities (email addresses : domains).</p> <p>This operation is throttled at one request per second and can only get custom MAIL FROM attributes for up to 100 identities at a time.</p>
    fn get_identity_mail_from_domain_attributes(
        &self,
        input: GetIdentityMailFromDomainAttributesRequest,
    ) -> RusotoFuture<
        GetIdentityMailFromDomainAttributesResponse,
        GetIdentityMailFromDomainAttributesError,
    >;

    /// <p>Given a list of verified identities (email addresses and/or domains), returns a structure describing identity notification attributes.</p> <p>This operation is throttled at one request per second and can only get notification attributes for up to 100 identities at a time.</p> <p>For more information about using notifications with Amazon SES, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/notifications.html">Amazon SES Developer Guide</a>.</p>
    fn get_identity_notification_attributes(
        &self,
        input: GetIdentityNotificationAttributesRequest,
    ) -> RusotoFuture<
        GetIdentityNotificationAttributesResponse,
        GetIdentityNotificationAttributesError,
    >;

    /// <p>Returns the requested sending authorization policies for the given identity (an email address or a domain). The policies are returned as a map of policy names to policy contents. You can retrieve a maximum of 20 policies at a time.</p> <note> <p>This API is for the identity owner only. If you have not verified the identity, this API will return an error.</p> </note> <p>Sending authorization is a feature that enables an identity owner to authorize other senders to use its identities. For information about using sending authorization, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/sending-authorization.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    fn get_identity_policies(
        &self,
        input: GetIdentityPoliciesRequest,
    ) -> RusotoFuture<GetIdentityPoliciesResponse, GetIdentityPoliciesError>;

    /// <p>Given a list of identities (email addresses and/or domains), returns the verification status and (for domain identities) the verification token for each identity.</p> <p>The verification status of an email address is "Pending" until the email address owner clicks the link within the verification email that Amazon SES sent to that address. If the email address owner clicks the link within 24 hours, the verification status of the email address changes to "Success". If the link is not clicked within 24 hours, the verification status changes to "Failed." In that case, if you still want to verify the email address, you must restart the verification process from the beginning.</p> <p>For domain identities, the domain's verification status is "Pending" as Amazon SES searches for the required TXT record in the DNS settings of the domain. When Amazon SES detects the record, the domain's verification status changes to "Success". If Amazon SES is unable to detect the record within 72 hours, the domain's verification status changes to "Failed." In that case, if you still want to verify the domain, you must restart the verification process from the beginning.</p> <p>This operation is throttled at one request per second and can only get verification attributes for up to 100 identities at a time.</p>
    fn get_identity_verification_attributes(
        &self,
        input: GetIdentityVerificationAttributesRequest,
    ) -> RusotoFuture<
        GetIdentityVerificationAttributesResponse,
        GetIdentityVerificationAttributesError,
    >;

    /// <p>Provides the sending limits for the Amazon SES account. </p> <p>You can execute this operation no more than once per second.</p>
    fn get_send_quota(&self) -> RusotoFuture<GetSendQuotaResponse, GetSendQuotaError>;

    /// <p>Provides sending statistics for the Amazon SES account. The result is a list of data points, representing the last two weeks of sending activity. Each data point in the list contains statistics for a 15-minute period of time.</p> <p>You can execute this operation no more than once per second.</p>
    fn get_send_statistics(
        &self,
    ) -> RusotoFuture<GetSendStatisticsResponse, GetSendStatisticsError>;

    /// <p>Displays the template object (which includes the Subject line, HTML part and text part) for the template you specify.</p> <p>You can execute this operation no more than once per second.</p>
    fn get_template(
        &self,
        input: GetTemplateRequest,
    ) -> RusotoFuture<GetTemplateResponse, GetTemplateError>;

    /// <p>Provides a list of the configuration sets associated with your Amazon SES account. For information about using configuration sets, see <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/monitor-sending-activity.html">Monitoring Your Amazon SES Sending Activity</a> in the <i>Amazon SES Developer Guide.</i> </p> <p>You can execute this operation no more than once per second. This operation will return up to 1,000 configuration sets each time it is run. If your Amazon SES account has more than 1,000 configuration sets, this operation will also return a NextToken element. You can then execute the <code>ListConfigurationSets</code> operation again, passing the <code>NextToken</code> parameter and the value of the NextToken element to retrieve additional results.</p>
    fn list_configuration_sets(
        &self,
        input: ListConfigurationSetsRequest,
    ) -> RusotoFuture<ListConfigurationSetsResponse, ListConfigurationSetsError>;

    /// <p>Lists the existing custom verification email templates for your account.</p> <p>For more information about custom verification email templates, see <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/custom-verification-emails.html">Using Custom Verification Email Templates</a> in the <i>Amazon SES Developer Guide</i>.</p> <p>You can execute this operation no more than once per second.</p>
    fn list_custom_verification_email_templates(
        &self,
        input: ListCustomVerificationEmailTemplatesRequest,
    ) -> RusotoFuture<
        ListCustomVerificationEmailTemplatesResponse,
        ListCustomVerificationEmailTemplatesError,
    >;

    /// <p>Returns a list containing all of the identities (email addresses and domains) for your AWS account, regardless of verification status.</p> <p>You can execute this operation no more than once per second.</p>
    fn list_identities(
        &self,
        input: ListIdentitiesRequest,
    ) -> RusotoFuture<ListIdentitiesResponse, ListIdentitiesError>;

    /// <p>Returns a list of sending authorization policies that are attached to the given identity (an email address or a domain). This API returns only a list. If you want the actual policy content, you can use <code>GetIdentityPolicies</code>.</p> <note> <p>This API is for the identity owner only. If you have not verified the identity, this API will return an error.</p> </note> <p>Sending authorization is a feature that enables an identity owner to authorize other senders to use its identities. For information about using sending authorization, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/sending-authorization.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    fn list_identity_policies(
        &self,
        input: ListIdentityPoliciesRequest,
    ) -> RusotoFuture<ListIdentityPoliciesResponse, ListIdentityPoliciesError>;

    /// <p>Lists the IP address filters associated with your AWS account.</p> <p>For information about managing IP address filters, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-managing-ip-filters.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    fn list_receipt_filters(
        &self,
        input: ListReceiptFiltersRequest,
    ) -> RusotoFuture<ListReceiptFiltersResponse, ListReceiptFiltersError>;

    /// <p>Lists the receipt rule sets that exist under your AWS account. If there are additional receipt rule sets to be retrieved, you will receive a <code>NextToken</code> that you can provide to the next call to <code>ListReceiptRuleSets</code> to retrieve the additional entries.</p> <p>For information about managing receipt rule sets, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-managing-receipt-rule-sets.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    fn list_receipt_rule_sets(
        &self,
        input: ListReceiptRuleSetsRequest,
    ) -> RusotoFuture<ListReceiptRuleSetsResponse, ListReceiptRuleSetsError>;

    /// <p>Lists the email templates present in your Amazon SES account.</p> <p>You can execute this operation no more than once per second.</p>
    fn list_templates(
        &self,
        input: ListTemplatesRequest,
    ) -> RusotoFuture<ListTemplatesResponse, ListTemplatesError>;

    /// <p>Deprecated. Use the <code>ListIdentities</code> operation to list the email addresses and domains associated with your account.</p>
    fn list_verified_email_addresses(
        &self,
    ) -> RusotoFuture<ListVerifiedEmailAddressesResponse, ListVerifiedEmailAddressesError>;

    /// <p>Adds or updates a sending authorization policy for the specified identity (an email address or a domain).</p> <note> <p>This API is for the identity owner only. If you have not verified the identity, this API will return an error.</p> </note> <p>Sending authorization is a feature that enables an identity owner to authorize other senders to use its identities. For information about using sending authorization, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/sending-authorization.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    fn put_identity_policy(
        &self,
        input: PutIdentityPolicyRequest,
    ) -> RusotoFuture<PutIdentityPolicyResponse, PutIdentityPolicyError>;

    /// <p>Reorders the receipt rules within a receipt rule set.</p> <note> <p>All of the rules in the rule set must be represented in this request. That is, this API will return an error if the reorder request doesn't explicitly position all of the rules.</p> </note> <p>For information about managing receipt rule sets, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-managing-receipt-rule-sets.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    fn reorder_receipt_rule_set(
        &self,
        input: ReorderReceiptRuleSetRequest,
    ) -> RusotoFuture<ReorderReceiptRuleSetResponse, ReorderReceiptRuleSetError>;

    /// <p>Generates and sends a bounce message to the sender of an email you received through Amazon SES. You can only use this API on an email up to 24 hours after you receive it.</p> <note> <p>You cannot use this API to send generic bounces for mail that was not received by Amazon SES.</p> </note> <p>For information about receiving email through Amazon SES, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    fn send_bounce(
        &self,
        input: SendBounceRequest,
    ) -> RusotoFuture<SendBounceResponse, SendBounceError>;

    /// <p><p>Composes an email message to multiple destinations. The message body is created using an email template.</p> <p>In order to send email using the <code>SendBulkTemplatedEmail</code> operation, your call to the API must meet the following requirements:</p> <ul> <li> <p>The call must refer to an existing email template. You can create email templates using the <a>CreateTemplate</a> operation.</p> </li> <li> <p>The message must be sent from a verified email address or domain.</p> </li> <li> <p>If your account is still in the Amazon SES sandbox, you may only send to verified addresses or domains, or to email addresses associated with the Amazon SES Mailbox Simulator. For more information, see <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/verify-addresses-and-domains.html">Verifying Email Addresses and Domains</a> in the <i>Amazon SES Developer Guide.</i> </p> </li> <li> <p>The total size of the message, including attachments, must be less than 10 MB.</p> </li> <li> <p>Each <code>Destination</code> parameter must include at least one recipient email address. The recipient address can be a To: address, a CC: address, or a BCC: address. If a recipient email address is invalid (that is, it is not in the format <i>UserName@[SubDomain.]Domain.TopLevelDomain</i>), the entire message will be rejected, even if the message contains other recipients that are valid.</p> </li> </ul></p>
    fn send_bulk_templated_email(
        &self,
        input: SendBulkTemplatedEmailRequest,
    ) -> RusotoFuture<SendBulkTemplatedEmailResponse, SendBulkTemplatedEmailError>;

    /// <p>Adds an email address to the list of identities for your Amazon SES account and attempts to verify it. As a result of executing this operation, a customized verification email is sent to the specified address.</p> <p>To use this operation, you must first create a custom verification email template. For more information about creating and using custom verification email templates, see <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/custom-verification-emails.html">Using Custom Verification Email Templates</a> in the <i>Amazon SES Developer Guide</i>.</p> <p>You can execute this operation no more than once per second.</p>
    fn send_custom_verification_email(
        &self,
        input: SendCustomVerificationEmailRequest,
    ) -> RusotoFuture<SendCustomVerificationEmailResponse, SendCustomVerificationEmailError>;

    /// <p><p>Composes an email message and immediately queues it for sending. In order to send email using the <code>SendEmail</code> operation, your message must meet the following requirements:</p> <ul> <li> <p>The message must be sent from a verified email address or domain. If you attempt to send email using a non-verified address or domain, the operation will result in an &quot;Email address not verified&quot; error. </p> </li> <li> <p>If your account is still in the Amazon SES sandbox, you may only send to verified addresses or domains, or to email addresses associated with the Amazon SES Mailbox Simulator. For more information, see <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/verify-addresses-and-domains.html">Verifying Email Addresses and Domains</a> in the <i>Amazon SES Developer Guide.</i> </p> </li> <li> <p>The total size of the message, including attachments, must be smaller than 10 MB.</p> </li> <li> <p>The message must include at least one recipient email address. The recipient address can be a To: address, a CC: address, or a BCC: address. If a recipient email address is invalid (that is, it is not in the format <i>UserName@[SubDomain.]Domain.TopLevelDomain</i>), the entire message will be rejected, even if the message contains other recipients that are valid.</p> </li> <li> <p>The message may not include more than 50 recipients, across the To:, CC: and BCC: fields. If you need to send an email message to a larger audience, you can divide your recipient list into groups of 50 or fewer, and then call the <code>SendEmail</code> operation several times to send the message to each group.</p> </li> </ul> <important> <p>For every message that you send, the total number of recipients (including each recipient in the To:, CC: and BCC: fields) is counted against the maximum number of emails you can send in a 24-hour period (your <i>sending quota</i>). For more information about sending quotas in Amazon SES, see <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/manage-sending-limits.html">Managing Your Amazon SES Sending Limits</a> in the <i>Amazon SES Developer Guide.</i> </p> </important></p>
    fn send_email(
        &self,
        input: SendEmailRequest,
    ) -> RusotoFuture<SendEmailResponse, SendEmailError>;

    /// <p><p>Composes an email message and immediately queues it for sending. When calling this operation, you may specify the message headers as well as the content. The <code>SendRawEmail</code> operation is particularly useful for sending multipart MIME emails (such as those that contain both a plain-text and an HTML version). </p> <p>In order to send email using the <code>SendRawEmail</code> operation, your message must meet the following requirements:</p> <ul> <li> <p>The message must be sent from a verified email address or domain. If you attempt to send email using a non-verified address or domain, the operation will result in an &quot;Email address not verified&quot; error. </p> </li> <li> <p>If your account is still in the Amazon SES sandbox, you may only send to verified addresses or domains, or to email addresses associated with the Amazon SES Mailbox Simulator. For more information, see <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/verify-addresses-and-domains.html">Verifying Email Addresses and Domains</a> in the <i>Amazon SES Developer Guide.</i> </p> </li> <li> <p>The total size of the message, including attachments, must be smaller than 10 MB.</p> </li> <li> <p>The message must include at least one recipient email address. The recipient address can be a To: address, a CC: address, or a BCC: address. If a recipient email address is invalid (that is, it is not in the format <i>UserName@[SubDomain.]Domain.TopLevelDomain</i>), the entire message will be rejected, even if the message contains other recipients that are valid.</p> </li> <li> <p>The message may not include more than 50 recipients, across the To:, CC: and BCC: fields. If you need to send an email message to a larger audience, you can divide your recipient list into groups of 50 or fewer, and then call the <code>SendRawEmail</code> operation several times to send the message to each group.</p> </li> </ul> <important> <p>For every message that you send, the total number of recipients (including each recipient in the To:, CC: and BCC: fields) is counted against the maximum number of emails you can send in a 24-hour period (your <i>sending quota</i>). For more information about sending quotas in Amazon SES, see <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/manage-sending-limits.html">Managing Your Amazon SES Sending Limits</a> in the <i>Amazon SES Developer Guide.</i> </p> </important> <p>Additionally, keep the following considerations in mind when using the <code>SendRawEmail</code> operation:</p> <ul> <li> <p>Although you can customize the message headers when using the <code>SendRawEmail</code> operation, Amazon SES will automatically apply its own <code>Message-ID</code> and <code>Date</code> headers; if you passed these headers when creating the message, they will be overwritten by the values that Amazon SES provides.</p> </li> <li> <p>If you are using sending authorization to send on behalf of another user, <code>SendRawEmail</code> enables you to specify the cross-account identity for the email&#39;s Source, From, and Return-Path parameters in one of two ways: you can pass optional parameters <code>SourceArn</code>, <code>FromArn</code>, and/or <code>ReturnPathArn</code> to the API, or you can include the following X-headers in the header of your raw email:</p> <ul> <li> <p> <code>X-SES-SOURCE-ARN</code> </p> </li> <li> <p> <code>X-SES-FROM-ARN</code> </p> </li> <li> <p> <code>X-SES-RETURN-PATH-ARN</code> </p> </li> </ul> <important> <p>Do not include these X-headers in the DKIM signature; Amazon SES will remove them before sending the email.</p> </important> <p>For most common sending authorization scenarios, we recommend that you specify the <code>SourceIdentityArn</code> parameter and not the <code>FromIdentityArn</code> or <code>ReturnPathIdentityArn</code> parameters. If you only specify the <code>SourceIdentityArn</code> parameter, Amazon SES will set the From and Return Path addresses to the identity specified in <code>SourceIdentityArn</code>. For more information about sending authorization, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/sending-authorization.html">Using Sending Authorization with Amazon SES</a> in the <i>Amazon SES Developer Guide.</i> </p> </li> </ul></p>
    fn send_raw_email(
        &self,
        input: SendRawEmailRequest,
    ) -> RusotoFuture<SendRawEmailResponse, SendRawEmailError>;

    /// <p><p>Composes an email message using an email template and immediately queues it for sending.</p> <p>In order to send email using the <code>SendTemplatedEmail</code> operation, your call to the API must meet the following requirements:</p> <ul> <li> <p>The call must refer to an existing email template. You can create email templates using the <a>CreateTemplate</a> operation.</p> </li> <li> <p>The message must be sent from a verified email address or domain.</p> </li> <li> <p>If your account is still in the Amazon SES sandbox, you may only send to verified addresses or domains, or to email addresses associated with the Amazon SES Mailbox Simulator. For more information, see <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/verify-addresses-and-domains.html">Verifying Email Addresses and Domains</a> in the <i>Amazon SES Developer Guide.</i> </p> </li> <li> <p>The total size of the message, including attachments, must be less than 10 MB.</p> </li> <li> <p>Calls to the <code>SendTemplatedEmail</code> operation may only include one <code>Destination</code> parameter. A destination is a set of recipients who will receive the same version of the email. The <code>Destination</code> parameter can include up to 50 recipients, across the To:, CC: and BCC: fields.</p> </li> <li> <p>The <code>Destination</code> parameter must include at least one recipient email address. The recipient address can be a To: address, a CC: address, or a BCC: address. If a recipient email address is invalid (that is, it is not in the format <i>UserName@[SubDomain.]Domain.TopLevelDomain</i>), the entire message will be rejected, even if the message contains other recipients that are valid.</p> </li> </ul></p>
    fn send_templated_email(
        &self,
        input: SendTemplatedEmailRequest,
    ) -> RusotoFuture<SendTemplatedEmailResponse, SendTemplatedEmailError>;

    /// <p>Sets the specified receipt rule set as the active receipt rule set.</p> <note> <p>To disable your email-receiving through Amazon SES completely, you can call this API with RuleSetName set to null.</p> </note> <p>For information about managing receipt rule sets, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-managing-receipt-rule-sets.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    fn set_active_receipt_rule_set(
        &self,
        input: SetActiveReceiptRuleSetRequest,
    ) -> RusotoFuture<SetActiveReceiptRuleSetResponse, SetActiveReceiptRuleSetError>;

    /// <p>Enables or disables Easy DKIM signing of email sent from an identity:</p> <ul> <li> <p>If Easy DKIM signing is enabled for a domain name identity (such as <code>example.com</code>), then Amazon SES will DKIM-sign all email sent by addresses under that domain name (for example, <code>user@example.com</code>).</p> </li> <li> <p>If Easy DKIM signing is enabled for an email address, then Amazon SES will DKIM-sign all email sent by that email address.</p> </li> </ul> <p>For email addresses (for example, <code>user@example.com</code>), you can only enable Easy DKIM signing if the corresponding domain (in this case, <code>example.com</code>) has been set up for Easy DKIM using the AWS Console or the <code>VerifyDomainDkim</code> operation.</p> <p>You can execute this operation no more than once per second.</p> <p>For more information about Easy DKIM signing, go to the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/easy-dkim.html">Amazon SES Developer Guide</a>.</p>
    fn set_identity_dkim_enabled(
        &self,
        input: SetIdentityDkimEnabledRequest,
    ) -> RusotoFuture<SetIdentityDkimEnabledResponse, SetIdentityDkimEnabledError>;

    /// <p>Given an identity (an email address or a domain), enables or disables whether Amazon SES forwards bounce and complaint notifications as email. Feedback forwarding can only be disabled when Amazon Simple Notification Service (Amazon SNS) topics are specified for both bounces and complaints.</p> <note> <p>Feedback forwarding does not apply to delivery notifications. Delivery notifications are only available through Amazon SNS.</p> </note> <p>You can execute this operation no more than once per second.</p> <p>For more information about using notifications with Amazon SES, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/notifications.html">Amazon SES Developer Guide</a>.</p>
    fn set_identity_feedback_forwarding_enabled(
        &self,
        input: SetIdentityFeedbackForwardingEnabledRequest,
    ) -> RusotoFuture<
        SetIdentityFeedbackForwardingEnabledResponse,
        SetIdentityFeedbackForwardingEnabledError,
    >;

    /// <p>Given an identity (an email address or a domain), sets whether Amazon SES includes the original email headers in the Amazon Simple Notification Service (Amazon SNS) notifications of a specified type.</p> <p>You can execute this operation no more than once per second.</p> <p>For more information about using notifications with Amazon SES, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/notifications.html">Amazon SES Developer Guide</a>.</p>
    fn set_identity_headers_in_notifications_enabled(
        &self,
        input: SetIdentityHeadersInNotificationsEnabledRequest,
    ) -> RusotoFuture<
        SetIdentityHeadersInNotificationsEnabledResponse,
        SetIdentityHeadersInNotificationsEnabledError,
    >;

    /// <p>Enables or disables the custom MAIL FROM domain setup for a verified identity (an email address or a domain).</p> <important> <p>To send emails using the specified MAIL FROM domain, you must add an MX record to your MAIL FROM domain's DNS settings. If you want your emails to pass Sender Policy Framework (SPF) checks, you must also add or update an SPF record. For more information, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/mail-from-set.html">Amazon SES Developer Guide</a>.</p> </important> <p>You can execute this operation no more than once per second.</p>
    fn set_identity_mail_from_domain(
        &self,
        input: SetIdentityMailFromDomainRequest,
    ) -> RusotoFuture<SetIdentityMailFromDomainResponse, SetIdentityMailFromDomainError>;

    /// <p>Given an identity (an email address or a domain), sets the Amazon Simple Notification Service (Amazon SNS) topic to which Amazon SES will publish bounce, complaint, and/or delivery notifications for emails sent with that identity as the <code>Source</code>.</p> <note> <p>Unless feedback forwarding is enabled, you must specify Amazon SNS topics for bounce and complaint notifications. For more information, see <code>SetIdentityFeedbackForwardingEnabled</code>.</p> </note> <p>You can execute this operation no more than once per second.</p> <p>For more information about feedback notification, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/notifications.html">Amazon SES Developer Guide</a>.</p>
    fn set_identity_notification_topic(
        &self,
        input: SetIdentityNotificationTopicRequest,
    ) -> RusotoFuture<SetIdentityNotificationTopicResponse, SetIdentityNotificationTopicError>;

    /// <p>Sets the position of the specified receipt rule in the receipt rule set.</p> <p>For information about managing receipt rules, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-managing-receipt-rules.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    fn set_receipt_rule_position(
        &self,
        input: SetReceiptRulePositionRequest,
    ) -> RusotoFuture<SetReceiptRulePositionResponse, SetReceiptRulePositionError>;

    /// <p>Creates a preview of the MIME content of an email when provided with a template and a set of replacement data.</p> <p>You can execute this operation no more than once per second.</p>
    fn test_render_template(
        &self,
        input: TestRenderTemplateRequest,
    ) -> RusotoFuture<TestRenderTemplateResponse, TestRenderTemplateError>;

    /// <p>Enables or disables email sending across your entire Amazon SES account. You can use this operation in conjunction with Amazon CloudWatch alarms to temporarily pause email sending across your Amazon SES account when reputation metrics (such as your bounce on complaint rate) reach certain thresholds.</p> <p>You can execute this operation no more than once per second.</p>
    fn update_account_sending_enabled(
        &self,
        input: UpdateAccountSendingEnabledRequest,
    ) -> RusotoFuture<(), UpdateAccountSendingEnabledError>;

    /// <p>Updates the event destination of a configuration set. Event destinations are associated with configuration sets, which enable you to publish email sending events to Amazon CloudWatch, Amazon Kinesis Firehose, or Amazon Simple Notification Service (Amazon SNS). For information about using configuration sets, see <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/monitor-sending-activity.html">Monitoring Your Amazon SES Sending Activity</a> in the <i>Amazon SES Developer Guide.</i> </p> <note> <p>When you create or update an event destination, you must provide one, and only one, destination. The destination can be Amazon CloudWatch, Amazon Kinesis Firehose, or Amazon Simple Notification Service (Amazon SNS).</p> </note> <p>You can execute this operation no more than once per second.</p>
    fn update_configuration_set_event_destination(
        &self,
        input: UpdateConfigurationSetEventDestinationRequest,
    ) -> RusotoFuture<
        UpdateConfigurationSetEventDestinationResponse,
        UpdateConfigurationSetEventDestinationError,
    >;

    /// <p>Enables or disables the publishing of reputation metrics for emails sent using a specific configuration set. Reputation metrics include bounce and complaint rates. These metrics are published to Amazon CloudWatch. By using Amazon CloudWatch, you can create alarms when bounce or complaint rates exceed a certain threshold.</p> <p>You can execute this operation no more than once per second.</p>
    fn update_configuration_set_reputation_metrics_enabled(
        &self,
        input: UpdateConfigurationSetReputationMetricsEnabledRequest,
    ) -> RusotoFuture<(), UpdateConfigurationSetReputationMetricsEnabledError>;

    /// <p>Enables or disables email sending for messages sent using a specific configuration set. You can use this operation in conjunction with Amazon CloudWatch alarms to temporarily pause email sending for a configuration set when the reputation metrics for that configuration set (such as your bounce on complaint rate) reach certain thresholds.</p> <p>You can execute this operation no more than once per second.</p>
    fn update_configuration_set_sending_enabled(
        &self,
        input: UpdateConfigurationSetSendingEnabledRequest,
    ) -> RusotoFuture<(), UpdateConfigurationSetSendingEnabledError>;

    /// <p>Modifies an association between a configuration set and a custom domain for open and click event tracking. </p> <p>By default, images and links used for tracking open and click events are hosted on domains operated by Amazon SES. You can configure a subdomain of your own to handle these events. For information about using configuration sets, see <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/configure-custom-open-click-domains.html">Configuring Custom Domains to Handle Open and Click Tracking</a> in the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/Welcome.html">Amazon SES Developer Guide</a>.</p>
    fn update_configuration_set_tracking_options(
        &self,
        input: UpdateConfigurationSetTrackingOptionsRequest,
    ) -> RusotoFuture<
        UpdateConfigurationSetTrackingOptionsResponse,
        UpdateConfigurationSetTrackingOptionsError,
    >;

    /// <p>Updates an existing custom verification email template.</p> <p>For more information about custom verification email templates, see <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/custom-verification-emails.html">Using Custom Verification Email Templates</a> in the <i>Amazon SES Developer Guide</i>.</p> <p>You can execute this operation no more than once per second.</p>
    fn update_custom_verification_email_template(
        &self,
        input: UpdateCustomVerificationEmailTemplateRequest,
    ) -> RusotoFuture<(), UpdateCustomVerificationEmailTemplateError>;

    /// <p>Updates a receipt rule.</p> <p>For information about managing receipt rules, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-managing-receipt-rules.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    fn update_receipt_rule(
        &self,
        input: UpdateReceiptRuleRequest,
    ) -> RusotoFuture<UpdateReceiptRuleResponse, UpdateReceiptRuleError>;

    /// <p>Updates an email template. Email templates enable you to send personalized email to one or more destinations in a single API operation. For more information, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/send-personalized-email-api.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    fn update_template(
        &self,
        input: UpdateTemplateRequest,
    ) -> RusotoFuture<UpdateTemplateResponse, UpdateTemplateError>;

    /// <p>Returns a set of DKIM tokens for a domain. DKIM <i>tokens</i> are character strings that represent your domain's identity. Using these tokens, you will need to create DNS CNAME records that point to DKIM public keys hosted by Amazon SES. Amazon Web Services will eventually detect that you have updated your DNS records; this detection process may take up to 72 hours. Upon successful detection, Amazon SES will be able to DKIM-sign email originating from that domain.</p> <p>You can execute this operation no more than once per second.</p> <p>To enable or disable Easy DKIM signing for a domain, use the <code>SetIdentityDkimEnabled</code> operation.</p> <p>For more information about creating DNS records using DKIM tokens, go to the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/easy-dkim-dns-records.html">Amazon SES Developer Guide</a>.</p>
    fn verify_domain_dkim(
        &self,
        input: VerifyDomainDkimRequest,
    ) -> RusotoFuture<VerifyDomainDkimResponse, VerifyDomainDkimError>;

    /// <p>Adds a domain to the list of identities for your Amazon SES account and attempts to verify it. For more information about verifying domains, see <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/verify-addresses-and-domains.html">Verifying Email Addresses and Domains</a> in the <i>Amazon SES Developer Guide.</i> </p> <p>You can execute this operation no more than once per second.</p>
    fn verify_domain_identity(
        &self,
        input: VerifyDomainIdentityRequest,
    ) -> RusotoFuture<VerifyDomainIdentityResponse, VerifyDomainIdentityError>;

    /// <p>Deprecated. Use the <code>VerifyEmailIdentity</code> operation to verify a new email address.</p>
    fn verify_email_address(
        &self,
        input: VerifyEmailAddressRequest,
    ) -> RusotoFuture<(), VerifyEmailAddressError>;

    /// <p>Adds an email address to the list of identities for your Amazon SES account and attempts to verify it. As a result of executing this operation, a verification email is sent to the specified address.</p> <p>You can execute this operation no more than once per second.</p>
    fn verify_email_identity(
        &self,
        input: VerifyEmailIdentityRequest,
    ) -> RusotoFuture<VerifyEmailIdentityResponse, VerifyEmailIdentityError>;
}
/// A client for the Amazon SES API.
pub struct SesClient<P = CredentialsProvider, D = RequestDispatcher>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    inner: ClientInner<P, D>,
    region: region::Region,
}

impl SesClient {
    /// Creates a simple client backed by an implicit event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    ///
    /// See the `rusoto_core::reactor` module for more details.
    pub fn simple(region: region::Region) -> SesClient {
        SesClient::new(
            RequestDispatcher::default(),
            CredentialsProvider::default(),
            region,
        )
    }
}

impl<P, D> SesClient<P, D>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
        SesClient {
            inner: ClientInner::new(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl<P, D> Ses for SesClient<P, D>
where
    P: ProvideAwsCredentials + 'static,
    D: DispatchSignedRequest + 'static,
{
    /// <p>Creates a receipt rule set by cloning an existing one. All receipt rules and configurations are copied to the new receipt rule set and are completely independent of the source rule set.</p> <p>For information about setting up rule sets, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-receipt-rule-set.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    fn clone_receipt_rule_set(
        &self,
        input: CloneReceiptRuleSetRequest,
    ) -> RusotoFuture<CloneReceiptRuleSetResponse, CloneReceiptRuleSetError> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CloneReceiptRuleSet");
        params.put("Version", "2010-12-01");
        CloneReceiptRuleSetRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CloneReceiptRuleSetError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = CloneReceiptRuleSetResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(CloneReceiptRuleSetResponseDeserializer::deserialize(
                        "CloneReceiptRuleSetResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates a configuration set.</p> <p>Configuration sets enable you to publish email sending events. For information about using configuration sets, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/monitor-sending-activity.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    fn create_configuration_set(
        &self,
        input: CreateConfigurationSetRequest,
    ) -> RusotoFuture<CreateConfigurationSetResponse, CreateConfigurationSetError> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CreateConfigurationSet");
        params.put("Version", "2010-12-01");
        CreateConfigurationSetRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateConfigurationSetError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = CreateConfigurationSetResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(CreateConfigurationSetResponseDeserializer::deserialize(
                        "CreateConfigurationSetResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates a configuration set event destination.</p> <note> <p>When you create or update an event destination, you must provide one, and only one, destination. The destination can be Amazon CloudWatch, Amazon Kinesis Firehose, or Amazon Simple Notification Service (Amazon SNS).</p> </note> <p>An event destination is the AWS service to which Amazon SES publishes the email sending events associated with a configuration set. For information about using configuration sets, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/monitor-sending-activity.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    fn create_configuration_set_event_destination(
        &self,
        input: CreateConfigurationSetEventDestinationRequest,
    ) -> RusotoFuture<
        CreateConfigurationSetEventDestinationResponse,
        CreateConfigurationSetEventDestinationError,
    > {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CreateConfigurationSetEventDestination");
        params.put("Version", "2010-12-01");
        CreateConfigurationSetEventDestinationRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateConfigurationSetEventDestinationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = CreateConfigurationSetEventDestinationResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(
                        CreateConfigurationSetEventDestinationResponseDeserializer::deserialize(
                            "CreateConfigurationSetEventDestinationResult",
                            &mut stack
                        )
                    );
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates an association between a configuration set and a custom domain for open and click event tracking. </p> <p>By default, images and links used for tracking open and click events are hosted on domains operated by Amazon SES. You can configure a subdomain of your own to handle these events. For information about using configuration sets, see <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/configure-custom-open-click-domains.html">Configuring Custom Domains to Handle Open and Click Tracking</a> in the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/Welcome.html">Amazon SES Developer Guide</a>.</p>
    fn create_configuration_set_tracking_options(
        &self,
        input: CreateConfigurationSetTrackingOptionsRequest,
    ) -> RusotoFuture<
        CreateConfigurationSetTrackingOptionsResponse,
        CreateConfigurationSetTrackingOptionsError,
    > {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CreateConfigurationSetTrackingOptions");
        params.put("Version", "2010-12-01");
        CreateConfigurationSetTrackingOptionsRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateConfigurationSetTrackingOptionsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = CreateConfigurationSetTrackingOptionsResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(
                        CreateConfigurationSetTrackingOptionsResponseDeserializer::deserialize(
                            "CreateConfigurationSetTrackingOptionsResult",
                            &mut stack
                        )
                    );
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates a new custom verification email template.</p> <p>For more information about custom verification email templates, see <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/custom-verification-emails.html">Using Custom Verification Email Templates</a> in the <i>Amazon SES Developer Guide</i>.</p> <p>You can execute this operation no more than once per second.</p>
    fn create_custom_verification_email_template(
        &self,
        input: CreateCustomVerificationEmailTemplateRequest,
    ) -> RusotoFuture<(), CreateCustomVerificationEmailTemplateError> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CreateCustomVerificationEmailTemplate");
        params.put("Version", "2010-12-01");
        CreateCustomVerificationEmailTemplateRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateCustomVerificationEmailTemplateError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(future::ok(::std::mem::drop(response)))
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates a new IP address filter.</p> <p>For information about setting up IP address filters, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-ip-filters.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    fn create_receipt_filter(
        &self,
        input: CreateReceiptFilterRequest,
    ) -> RusotoFuture<CreateReceiptFilterResponse, CreateReceiptFilterError> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CreateReceiptFilter");
        params.put("Version", "2010-12-01");
        CreateReceiptFilterRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateReceiptFilterError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = CreateReceiptFilterResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(CreateReceiptFilterResponseDeserializer::deserialize(
                        "CreateReceiptFilterResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates a receipt rule.</p> <p>For information about setting up receipt rules, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-receipt-rules.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    fn create_receipt_rule(
        &self,
        input: CreateReceiptRuleRequest,
    ) -> RusotoFuture<CreateReceiptRuleResponse, CreateReceiptRuleError> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CreateReceiptRule");
        params.put("Version", "2010-12-01");
        CreateReceiptRuleRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateReceiptRuleError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = CreateReceiptRuleResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(CreateReceiptRuleResponseDeserializer::deserialize(
                        "CreateReceiptRuleResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates an empty receipt rule set.</p> <p>For information about setting up receipt rule sets, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-receipt-rule-set.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    fn create_receipt_rule_set(
        &self,
        input: CreateReceiptRuleSetRequest,
    ) -> RusotoFuture<CreateReceiptRuleSetResponse, CreateReceiptRuleSetError> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CreateReceiptRuleSet");
        params.put("Version", "2010-12-01");
        CreateReceiptRuleSetRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateReceiptRuleSetError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = CreateReceiptRuleSetResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(CreateReceiptRuleSetResponseDeserializer::deserialize(
                        "CreateReceiptRuleSetResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates an email template. Email templates enable you to send personalized email to one or more destinations in a single API operation. For more information, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/send-personalized-email-api.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    fn create_template(
        &self,
        input: CreateTemplateRequest,
    ) -> RusotoFuture<CreateTemplateResponse, CreateTemplateError> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CreateTemplate");
        params.put("Version", "2010-12-01");
        CreateTemplateRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateTemplateError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = CreateTemplateResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(CreateTemplateResponseDeserializer::deserialize(
                        "CreateTemplateResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes a configuration set. Configuration sets enable you to publish email sending events. For information about using configuration sets, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/monitor-sending-activity.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    fn delete_configuration_set(
        &self,
        input: DeleteConfigurationSetRequest,
    ) -> RusotoFuture<DeleteConfigurationSetResponse, DeleteConfigurationSetError> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteConfigurationSet");
        params.put("Version", "2010-12-01");
        DeleteConfigurationSetRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteConfigurationSetError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DeleteConfigurationSetResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DeleteConfigurationSetResponseDeserializer::deserialize(
                        "DeleteConfigurationSetResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes a configuration set event destination. Configuration set event destinations are associated with configuration sets, which enable you to publish email sending events. For information about using configuration sets, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/monitor-sending-activity.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    fn delete_configuration_set_event_destination(
        &self,
        input: DeleteConfigurationSetEventDestinationRequest,
    ) -> RusotoFuture<
        DeleteConfigurationSetEventDestinationResponse,
        DeleteConfigurationSetEventDestinationError,
    > {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteConfigurationSetEventDestination");
        params.put("Version", "2010-12-01");
        DeleteConfigurationSetEventDestinationRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteConfigurationSetEventDestinationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DeleteConfigurationSetEventDestinationResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(
                        DeleteConfigurationSetEventDestinationResponseDeserializer::deserialize(
                            "DeleteConfigurationSetEventDestinationResult",
                            &mut stack
                        )
                    );
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Deletes an association between a configuration set and a custom domain for open and click event tracking.</p> <p>By default, images and links used for tracking open and click events are hosted on domains operated by Amazon SES. You can configure a subdomain of your own to handle these events. For information about using configuration sets, see <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/configure-custom-open-click-domains.html">Configuring Custom Domains to Handle Open and Click Tracking</a> in the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/Welcome.html">Amazon SES Developer Guide</a>.</p> <note> <p>Deleting this kind of association will result in emails sent using the specified configuration set to capture open and click events using the standard, Amazon SES-operated domains.</p> </note></p>
    fn delete_configuration_set_tracking_options(
        &self,
        input: DeleteConfigurationSetTrackingOptionsRequest,
    ) -> RusotoFuture<
        DeleteConfigurationSetTrackingOptionsResponse,
        DeleteConfigurationSetTrackingOptionsError,
    > {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteConfigurationSetTrackingOptions");
        params.put("Version", "2010-12-01");
        DeleteConfigurationSetTrackingOptionsRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteConfigurationSetTrackingOptionsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DeleteConfigurationSetTrackingOptionsResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(
                        DeleteConfigurationSetTrackingOptionsResponseDeserializer::deserialize(
                            "DeleteConfigurationSetTrackingOptionsResult",
                            &mut stack
                        )
                    );
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes an existing custom verification email template. </p> <p>For more information about custom verification email templates, see <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/custom-verification-emails.html">Using Custom Verification Email Templates</a> in the <i>Amazon SES Developer Guide</i>.</p> <p>You can execute this operation no more than once per second.</p>
    fn delete_custom_verification_email_template(
        &self,
        input: DeleteCustomVerificationEmailTemplateRequest,
    ) -> RusotoFuture<(), DeleteCustomVerificationEmailTemplateError> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteCustomVerificationEmailTemplate");
        params.put("Version", "2010-12-01");
        DeleteCustomVerificationEmailTemplateRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteCustomVerificationEmailTemplateError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(future::ok(::std::mem::drop(response)))
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes the specified identity (an email address or a domain) from the list of verified identities.</p> <p>You can execute this operation no more than once per second.</p>
    fn delete_identity(
        &self,
        input: DeleteIdentityRequest,
    ) -> RusotoFuture<DeleteIdentityResponse, DeleteIdentityError> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteIdentity");
        params.put("Version", "2010-12-01");
        DeleteIdentityRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteIdentityError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DeleteIdentityResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DeleteIdentityResponseDeserializer::deserialize(
                        "DeleteIdentityResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes the specified sending authorization policy for the given identity (an email address or a domain). This API returns successfully even if a policy with the specified name does not exist.</p> <note> <p>This API is for the identity owner only. If you have not verified the identity, this API will return an error.</p> </note> <p>Sending authorization is a feature that enables an identity owner to authorize other senders to use its identities. For information about using sending authorization, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/sending-authorization.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    fn delete_identity_policy(
        &self,
        input: DeleteIdentityPolicyRequest,
    ) -> RusotoFuture<DeleteIdentityPolicyResponse, DeleteIdentityPolicyError> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteIdentityPolicy");
        params.put("Version", "2010-12-01");
        DeleteIdentityPolicyRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteIdentityPolicyError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DeleteIdentityPolicyResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DeleteIdentityPolicyResponseDeserializer::deserialize(
                        "DeleteIdentityPolicyResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes the specified IP address filter.</p> <p>For information about managing IP address filters, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-managing-ip-filters.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    fn delete_receipt_filter(
        &self,
        input: DeleteReceiptFilterRequest,
    ) -> RusotoFuture<DeleteReceiptFilterResponse, DeleteReceiptFilterError> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteReceiptFilter");
        params.put("Version", "2010-12-01");
        DeleteReceiptFilterRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteReceiptFilterError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DeleteReceiptFilterResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DeleteReceiptFilterResponseDeserializer::deserialize(
                        "DeleteReceiptFilterResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes the specified receipt rule.</p> <p>For information about managing receipt rules, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-managing-receipt-rules.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    fn delete_receipt_rule(
        &self,
        input: DeleteReceiptRuleRequest,
    ) -> RusotoFuture<DeleteReceiptRuleResponse, DeleteReceiptRuleError> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteReceiptRule");
        params.put("Version", "2010-12-01");
        DeleteReceiptRuleRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteReceiptRuleError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DeleteReceiptRuleResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DeleteReceiptRuleResponseDeserializer::deserialize(
                        "DeleteReceiptRuleResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes the specified receipt rule set and all of the receipt rules it contains.</p> <note> <p>The currently active rule set cannot be deleted.</p> </note> <p>For information about managing receipt rule sets, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-managing-receipt-rule-sets.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    fn delete_receipt_rule_set(
        &self,
        input: DeleteReceiptRuleSetRequest,
    ) -> RusotoFuture<DeleteReceiptRuleSetResponse, DeleteReceiptRuleSetError> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteReceiptRuleSet");
        params.put("Version", "2010-12-01");
        DeleteReceiptRuleSetRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteReceiptRuleSetError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DeleteReceiptRuleSetResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DeleteReceiptRuleSetResponseDeserializer::deserialize(
                        "DeleteReceiptRuleSetResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes an email template.</p> <p>You can execute this operation no more than once per second.</p>
    fn delete_template(
        &self,
        input: DeleteTemplateRequest,
    ) -> RusotoFuture<DeleteTemplateResponse, DeleteTemplateError> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteTemplate");
        params.put("Version", "2010-12-01");
        DeleteTemplateRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteTemplateError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DeleteTemplateResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DeleteTemplateResponseDeserializer::deserialize(
                        "DeleteTemplateResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Deprecated. Use the <code>DeleteIdentity</code> operation to delete email addresses and domains.</p>
    fn delete_verified_email_address(
        &self,
        input: DeleteVerifiedEmailAddressRequest,
    ) -> RusotoFuture<(), DeleteVerifiedEmailAddressError> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteVerifiedEmailAddress");
        params.put("Version", "2010-12-01");
        DeleteVerifiedEmailAddressRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteVerifiedEmailAddressError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(future::ok(::std::mem::drop(response)))
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns the metadata and receipt rules for the receipt rule set that is currently active.</p> <p>For information about setting up receipt rule sets, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-receipt-rule-set.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    fn describe_active_receipt_rule_set(
        &self,
        input: DescribeActiveReceiptRuleSetRequest,
    ) -> RusotoFuture<DescribeActiveReceiptRuleSetResponse, DescribeActiveReceiptRuleSetError> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeActiveReceiptRuleSet");
        params.put("Version", "2010-12-01");
        DescribeActiveReceiptRuleSetRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeActiveReceiptRuleSetError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DescribeActiveReceiptRuleSetResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(
                        DescribeActiveReceiptRuleSetResponseDeserializer::deserialize(
                            "DescribeActiveReceiptRuleSetResult",
                            &mut stack
                        )
                    );
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns the details of the specified configuration set. For information about using configuration sets, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/monitor-sending-activity.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    fn describe_configuration_set(
        &self,
        input: DescribeConfigurationSetRequest,
    ) -> RusotoFuture<DescribeConfigurationSetResponse, DescribeConfigurationSetError> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeConfigurationSet");
        params.put("Version", "2010-12-01");
        DescribeConfigurationSetRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeConfigurationSetError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DescribeConfigurationSetResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DescribeConfigurationSetResponseDeserializer::deserialize(
                        "DescribeConfigurationSetResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns the details of the specified receipt rule.</p> <p>For information about setting up receipt rules, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-receipt-rules.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    fn describe_receipt_rule(
        &self,
        input: DescribeReceiptRuleRequest,
    ) -> RusotoFuture<DescribeReceiptRuleResponse, DescribeReceiptRuleError> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeReceiptRule");
        params.put("Version", "2010-12-01");
        DescribeReceiptRuleRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeReceiptRuleError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DescribeReceiptRuleResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DescribeReceiptRuleResponseDeserializer::deserialize(
                        "DescribeReceiptRuleResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns the details of the specified receipt rule set.</p> <p>For information about managing receipt rule sets, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-managing-receipt-rule-sets.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    fn describe_receipt_rule_set(
        &self,
        input: DescribeReceiptRuleSetRequest,
    ) -> RusotoFuture<DescribeReceiptRuleSetResponse, DescribeReceiptRuleSetError> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeReceiptRuleSet");
        params.put("Version", "2010-12-01");
        DescribeReceiptRuleSetRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeReceiptRuleSetError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DescribeReceiptRuleSetResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DescribeReceiptRuleSetResponseDeserializer::deserialize(
                        "DescribeReceiptRuleSetResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns the email sending status of the Amazon SES account.</p> <p>You can execute this operation no more than once per second.</p>
    fn get_account_sending_enabled(
        &self,
    ) -> RusotoFuture<GetAccountSendingEnabledResponse, GetAccountSendingEnabledError> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "GetAccountSendingEnabled");
        params.put("Version", "2010-12-01");

        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetAccountSendingEnabledError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = GetAccountSendingEnabledResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(GetAccountSendingEnabledResponseDeserializer::deserialize(
                        "GetAccountSendingEnabledResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns the custom email verification template for the template name you specify.</p> <p>For more information about custom verification email templates, see <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/custom-verification-emails.html">Using Custom Verification Email Templates</a> in the <i>Amazon SES Developer Guide</i>.</p> <p>You can execute this operation no more than once per second.</p>
    fn get_custom_verification_email_template(
        &self,
        input: GetCustomVerificationEmailTemplateRequest,
    ) -> RusotoFuture<
        GetCustomVerificationEmailTemplateResponse,
        GetCustomVerificationEmailTemplateError,
    > {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "GetCustomVerificationEmailTemplate");
        params.put("Version", "2010-12-01");
        GetCustomVerificationEmailTemplateRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetCustomVerificationEmailTemplateError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = GetCustomVerificationEmailTemplateResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(
                        GetCustomVerificationEmailTemplateResponseDeserializer::deserialize(
                            "GetCustomVerificationEmailTemplateResult",
                            &mut stack
                        )
                    );
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns the current status of Easy DKIM signing for an entity. For domain name identities, this operation also returns the DKIM tokens that are required for Easy DKIM signing, and whether Amazon SES has successfully verified that these tokens have been published.</p> <p>This operation takes a list of identities as input and returns the following information for each:</p> <ul> <li> <p>Whether Easy DKIM signing is enabled or disabled.</p> </li> <li> <p>A set of DKIM tokens that represent the identity. If the identity is an email address, the tokens represent the domain of that address.</p> </li> <li> <p>Whether Amazon SES has successfully verified the DKIM tokens published in the domain's DNS. This information is only returned for domain name identities, not for email addresses.</p> </li> </ul> <p>This operation is throttled at one request per second and can only get DKIM attributes for up to 100 identities at a time.</p> <p>For more information about creating DNS records using DKIM tokens, go to the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/easy-dkim-dns-records.html">Amazon SES Developer Guide</a>.</p>
    fn get_identity_dkim_attributes(
        &self,
        input: GetIdentityDkimAttributesRequest,
    ) -> RusotoFuture<GetIdentityDkimAttributesResponse, GetIdentityDkimAttributesError> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "GetIdentityDkimAttributes");
        params.put("Version", "2010-12-01");
        GetIdentityDkimAttributesRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetIdentityDkimAttributesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = GetIdentityDkimAttributesResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(GetIdentityDkimAttributesResponseDeserializer::deserialize(
                        "GetIdentityDkimAttributesResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns the custom MAIL FROM attributes for a list of identities (email addresses : domains).</p> <p>This operation is throttled at one request per second and can only get custom MAIL FROM attributes for up to 100 identities at a time.</p>
    fn get_identity_mail_from_domain_attributes(
        &self,
        input: GetIdentityMailFromDomainAttributesRequest,
    ) -> RusotoFuture<
        GetIdentityMailFromDomainAttributesResponse,
        GetIdentityMailFromDomainAttributesError,
    > {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "GetIdentityMailFromDomainAttributes");
        params.put("Version", "2010-12-01");
        GetIdentityMailFromDomainAttributesRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetIdentityMailFromDomainAttributesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = GetIdentityMailFromDomainAttributesResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(
                        GetIdentityMailFromDomainAttributesResponseDeserializer::deserialize(
                            "GetIdentityMailFromDomainAttributesResult",
                            &mut stack
                        )
                    );
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Given a list of verified identities (email addresses and/or domains), returns a structure describing identity notification attributes.</p> <p>This operation is throttled at one request per second and can only get notification attributes for up to 100 identities at a time.</p> <p>For more information about using notifications with Amazon SES, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/notifications.html">Amazon SES Developer Guide</a>.</p>
    fn get_identity_notification_attributes(
        &self,
        input: GetIdentityNotificationAttributesRequest,
    ) -> RusotoFuture<
        GetIdentityNotificationAttributesResponse,
        GetIdentityNotificationAttributesError,
    > {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "GetIdentityNotificationAttributes");
        params.put("Version", "2010-12-01");
        GetIdentityNotificationAttributesRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetIdentityNotificationAttributesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = GetIdentityNotificationAttributesResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(
                        GetIdentityNotificationAttributesResponseDeserializer::deserialize(
                            "GetIdentityNotificationAttributesResult",
                            &mut stack
                        )
                    );
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns the requested sending authorization policies for the given identity (an email address or a domain). The policies are returned as a map of policy names to policy contents. You can retrieve a maximum of 20 policies at a time.</p> <note> <p>This API is for the identity owner only. If you have not verified the identity, this API will return an error.</p> </note> <p>Sending authorization is a feature that enables an identity owner to authorize other senders to use its identities. For information about using sending authorization, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/sending-authorization.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    fn get_identity_policies(
        &self,
        input: GetIdentityPoliciesRequest,
    ) -> RusotoFuture<GetIdentityPoliciesResponse, GetIdentityPoliciesError> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "GetIdentityPolicies");
        params.put("Version", "2010-12-01");
        GetIdentityPoliciesRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetIdentityPoliciesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = GetIdentityPoliciesResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(GetIdentityPoliciesResponseDeserializer::deserialize(
                        "GetIdentityPoliciesResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Given a list of identities (email addresses and/or domains), returns the verification status and (for domain identities) the verification token for each identity.</p> <p>The verification status of an email address is "Pending" until the email address owner clicks the link within the verification email that Amazon SES sent to that address. If the email address owner clicks the link within 24 hours, the verification status of the email address changes to "Success". If the link is not clicked within 24 hours, the verification status changes to "Failed." In that case, if you still want to verify the email address, you must restart the verification process from the beginning.</p> <p>For domain identities, the domain's verification status is "Pending" as Amazon SES searches for the required TXT record in the DNS settings of the domain. When Amazon SES detects the record, the domain's verification status changes to "Success". If Amazon SES is unable to detect the record within 72 hours, the domain's verification status changes to "Failed." In that case, if you still want to verify the domain, you must restart the verification process from the beginning.</p> <p>This operation is throttled at one request per second and can only get verification attributes for up to 100 identities at a time.</p>
    fn get_identity_verification_attributes(
        &self,
        input: GetIdentityVerificationAttributesRequest,
    ) -> RusotoFuture<
        GetIdentityVerificationAttributesResponse,
        GetIdentityVerificationAttributesError,
    > {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "GetIdentityVerificationAttributes");
        params.put("Version", "2010-12-01");
        GetIdentityVerificationAttributesRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetIdentityVerificationAttributesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = GetIdentityVerificationAttributesResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(
                        GetIdentityVerificationAttributesResponseDeserializer::deserialize(
                            "GetIdentityVerificationAttributesResult",
                            &mut stack
                        )
                    );
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Provides the sending limits for the Amazon SES account. </p> <p>You can execute this operation no more than once per second.</p>
    fn get_send_quota(&self) -> RusotoFuture<GetSendQuotaResponse, GetSendQuotaError> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "GetSendQuota");
        params.put("Version", "2010-12-01");

        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetSendQuotaError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = GetSendQuotaResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(GetSendQuotaResponseDeserializer::deserialize(
                        "GetSendQuotaResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Provides sending statistics for the Amazon SES account. The result is a list of data points, representing the last two weeks of sending activity. Each data point in the list contains statistics for a 15-minute period of time.</p> <p>You can execute this operation no more than once per second.</p>
    fn get_send_statistics(
        &self,
    ) -> RusotoFuture<GetSendStatisticsResponse, GetSendStatisticsError> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "GetSendStatistics");
        params.put("Version", "2010-12-01");

        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetSendStatisticsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = GetSendStatisticsResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(GetSendStatisticsResponseDeserializer::deserialize(
                        "GetSendStatisticsResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Displays the template object (which includes the Subject line, HTML part and text part) for the template you specify.</p> <p>You can execute this operation no more than once per second.</p>
    fn get_template(
        &self,
        input: GetTemplateRequest,
    ) -> RusotoFuture<GetTemplateResponse, GetTemplateError> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "GetTemplate");
        params.put("Version", "2010-12-01");
        GetTemplateRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetTemplateError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = GetTemplateResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(GetTemplateResponseDeserializer::deserialize(
                        "GetTemplateResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Provides a list of the configuration sets associated with your Amazon SES account. For information about using configuration sets, see <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/monitor-sending-activity.html">Monitoring Your Amazon SES Sending Activity</a> in the <i>Amazon SES Developer Guide.</i> </p> <p>You can execute this operation no more than once per second. This operation will return up to 1,000 configuration sets each time it is run. If your Amazon SES account has more than 1,000 configuration sets, this operation will also return a NextToken element. You can then execute the <code>ListConfigurationSets</code> operation again, passing the <code>NextToken</code> parameter and the value of the NextToken element to retrieve additional results.</p>
    fn list_configuration_sets(
        &self,
        input: ListConfigurationSetsRequest,
    ) -> RusotoFuture<ListConfigurationSetsResponse, ListConfigurationSetsError> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ListConfigurationSets");
        params.put("Version", "2010-12-01");
        ListConfigurationSetsRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListConfigurationSetsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ListConfigurationSetsResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(ListConfigurationSetsResponseDeserializer::deserialize(
                        "ListConfigurationSetsResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Lists the existing custom verification email templates for your account.</p> <p>For more information about custom verification email templates, see <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/custom-verification-emails.html">Using Custom Verification Email Templates</a> in the <i>Amazon SES Developer Guide</i>.</p> <p>You can execute this operation no more than once per second.</p>
    fn list_custom_verification_email_templates(
        &self,
        input: ListCustomVerificationEmailTemplatesRequest,
    ) -> RusotoFuture<
        ListCustomVerificationEmailTemplatesResponse,
        ListCustomVerificationEmailTemplatesError,
    > {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ListCustomVerificationEmailTemplates");
        params.put("Version", "2010-12-01");
        ListCustomVerificationEmailTemplatesRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListCustomVerificationEmailTemplatesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ListCustomVerificationEmailTemplatesResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(
                        ListCustomVerificationEmailTemplatesResponseDeserializer::deserialize(
                            "ListCustomVerificationEmailTemplatesResult",
                            &mut stack
                        )
                    );
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns a list containing all of the identities (email addresses and domains) for your AWS account, regardless of verification status.</p> <p>You can execute this operation no more than once per second.</p>
    fn list_identities(
        &self,
        input: ListIdentitiesRequest,
    ) -> RusotoFuture<ListIdentitiesResponse, ListIdentitiesError> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ListIdentities");
        params.put("Version", "2010-12-01");
        ListIdentitiesRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListIdentitiesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ListIdentitiesResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(ListIdentitiesResponseDeserializer::deserialize(
                        "ListIdentitiesResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns a list of sending authorization policies that are attached to the given identity (an email address or a domain). This API returns only a list. If you want the actual policy content, you can use <code>GetIdentityPolicies</code>.</p> <note> <p>This API is for the identity owner only. If you have not verified the identity, this API will return an error.</p> </note> <p>Sending authorization is a feature that enables an identity owner to authorize other senders to use its identities. For information about using sending authorization, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/sending-authorization.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    fn list_identity_policies(
        &self,
        input: ListIdentityPoliciesRequest,
    ) -> RusotoFuture<ListIdentityPoliciesResponse, ListIdentityPoliciesError> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ListIdentityPolicies");
        params.put("Version", "2010-12-01");
        ListIdentityPoliciesRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListIdentityPoliciesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ListIdentityPoliciesResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(ListIdentityPoliciesResponseDeserializer::deserialize(
                        "ListIdentityPoliciesResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Lists the IP address filters associated with your AWS account.</p> <p>For information about managing IP address filters, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-managing-ip-filters.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    fn list_receipt_filters(
        &self,
        input: ListReceiptFiltersRequest,
    ) -> RusotoFuture<ListReceiptFiltersResponse, ListReceiptFiltersError> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ListReceiptFilters");
        params.put("Version", "2010-12-01");
        ListReceiptFiltersRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListReceiptFiltersError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ListReceiptFiltersResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(ListReceiptFiltersResponseDeserializer::deserialize(
                        "ListReceiptFiltersResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Lists the receipt rule sets that exist under your AWS account. If there are additional receipt rule sets to be retrieved, you will receive a <code>NextToken</code> that you can provide to the next call to <code>ListReceiptRuleSets</code> to retrieve the additional entries.</p> <p>For information about managing receipt rule sets, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-managing-receipt-rule-sets.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    fn list_receipt_rule_sets(
        &self,
        input: ListReceiptRuleSetsRequest,
    ) -> RusotoFuture<ListReceiptRuleSetsResponse, ListReceiptRuleSetsError> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ListReceiptRuleSets");
        params.put("Version", "2010-12-01");
        ListReceiptRuleSetsRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListReceiptRuleSetsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ListReceiptRuleSetsResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(ListReceiptRuleSetsResponseDeserializer::deserialize(
                        "ListReceiptRuleSetsResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Lists the email templates present in your Amazon SES account.</p> <p>You can execute this operation no more than once per second.</p>
    fn list_templates(
        &self,
        input: ListTemplatesRequest,
    ) -> RusotoFuture<ListTemplatesResponse, ListTemplatesError> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ListTemplates");
        params.put("Version", "2010-12-01");
        ListTemplatesRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListTemplatesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ListTemplatesResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(ListTemplatesResponseDeserializer::deserialize(
                        "ListTemplatesResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Deprecated. Use the <code>ListIdentities</code> operation to list the email addresses and domains associated with your account.</p>
    fn list_verified_email_addresses(
        &self,
    ) -> RusotoFuture<ListVerifiedEmailAddressesResponse, ListVerifiedEmailAddressesError> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ListVerifiedEmailAddresses");
        params.put("Version", "2010-12-01");

        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListVerifiedEmailAddressesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ListVerifiedEmailAddressesResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(ListVerifiedEmailAddressesResponseDeserializer::deserialize(
                        "ListVerifiedEmailAddressesResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Adds or updates a sending authorization policy for the specified identity (an email address or a domain).</p> <note> <p>This API is for the identity owner only. If you have not verified the identity, this API will return an error.</p> </note> <p>Sending authorization is a feature that enables an identity owner to authorize other senders to use its identities. For information about using sending authorization, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/sending-authorization.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    fn put_identity_policy(
        &self,
        input: PutIdentityPolicyRequest,
    ) -> RusotoFuture<PutIdentityPolicyResponse, PutIdentityPolicyError> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "PutIdentityPolicy");
        params.put("Version", "2010-12-01");
        PutIdentityPolicyRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(PutIdentityPolicyError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = PutIdentityPolicyResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(PutIdentityPolicyResponseDeserializer::deserialize(
                        "PutIdentityPolicyResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Reorders the receipt rules within a receipt rule set.</p> <note> <p>All of the rules in the rule set must be represented in this request. That is, this API will return an error if the reorder request doesn't explicitly position all of the rules.</p> </note> <p>For information about managing receipt rule sets, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-managing-receipt-rule-sets.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    fn reorder_receipt_rule_set(
        &self,
        input: ReorderReceiptRuleSetRequest,
    ) -> RusotoFuture<ReorderReceiptRuleSetResponse, ReorderReceiptRuleSetError> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ReorderReceiptRuleSet");
        params.put("Version", "2010-12-01");
        ReorderReceiptRuleSetRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ReorderReceiptRuleSetError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ReorderReceiptRuleSetResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(ReorderReceiptRuleSetResponseDeserializer::deserialize(
                        "ReorderReceiptRuleSetResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Generates and sends a bounce message to the sender of an email you received through Amazon SES. You can only use this API on an email up to 24 hours after you receive it.</p> <note> <p>You cannot use this API to send generic bounces for mail that was not received by Amazon SES.</p> </note> <p>For information about receiving email through Amazon SES, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    fn send_bounce(
        &self,
        input: SendBounceRequest,
    ) -> RusotoFuture<SendBounceResponse, SendBounceError> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "SendBounce");
        params.put("Version", "2010-12-01");
        SendBounceRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(SendBounceError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = SendBounceResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(SendBounceResponseDeserializer::deserialize(
                        "SendBounceResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Composes an email message to multiple destinations. The message body is created using an email template.</p> <p>In order to send email using the <code>SendBulkTemplatedEmail</code> operation, your call to the API must meet the following requirements:</p> <ul> <li> <p>The call must refer to an existing email template. You can create email templates using the <a>CreateTemplate</a> operation.</p> </li> <li> <p>The message must be sent from a verified email address or domain.</p> </li> <li> <p>If your account is still in the Amazon SES sandbox, you may only send to verified addresses or domains, or to email addresses associated with the Amazon SES Mailbox Simulator. For more information, see <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/verify-addresses-and-domains.html">Verifying Email Addresses and Domains</a> in the <i>Amazon SES Developer Guide.</i> </p> </li> <li> <p>The total size of the message, including attachments, must be less than 10 MB.</p> </li> <li> <p>Each <code>Destination</code> parameter must include at least one recipient email address. The recipient address can be a To: address, a CC: address, or a BCC: address. If a recipient email address is invalid (that is, it is not in the format <i>UserName@[SubDomain.]Domain.TopLevelDomain</i>), the entire message will be rejected, even if the message contains other recipients that are valid.</p> </li> </ul></p>
    fn send_bulk_templated_email(
        &self,
        input: SendBulkTemplatedEmailRequest,
    ) -> RusotoFuture<SendBulkTemplatedEmailResponse, SendBulkTemplatedEmailError> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "SendBulkTemplatedEmail");
        params.put("Version", "2010-12-01");
        SendBulkTemplatedEmailRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(SendBulkTemplatedEmailError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = SendBulkTemplatedEmailResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(SendBulkTemplatedEmailResponseDeserializer::deserialize(
                        "SendBulkTemplatedEmailResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Adds an email address to the list of identities for your Amazon SES account and attempts to verify it. As a result of executing this operation, a customized verification email is sent to the specified address.</p> <p>To use this operation, you must first create a custom verification email template. For more information about creating and using custom verification email templates, see <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/custom-verification-emails.html">Using Custom Verification Email Templates</a> in the <i>Amazon SES Developer Guide</i>.</p> <p>You can execute this operation no more than once per second.</p>
    fn send_custom_verification_email(
        &self,
        input: SendCustomVerificationEmailRequest,
    ) -> RusotoFuture<SendCustomVerificationEmailResponse, SendCustomVerificationEmailError> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "SendCustomVerificationEmail");
        params.put("Version", "2010-12-01");
        SendCustomVerificationEmailRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(SendCustomVerificationEmailError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = SendCustomVerificationEmailResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(
                        SendCustomVerificationEmailResponseDeserializer::deserialize(
                            "SendCustomVerificationEmailResult",
                            &mut stack
                        )
                    );
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Composes an email message and immediately queues it for sending. In order to send email using the <code>SendEmail</code> operation, your message must meet the following requirements:</p> <ul> <li> <p>The message must be sent from a verified email address or domain. If you attempt to send email using a non-verified address or domain, the operation will result in an &quot;Email address not verified&quot; error. </p> </li> <li> <p>If your account is still in the Amazon SES sandbox, you may only send to verified addresses or domains, or to email addresses associated with the Amazon SES Mailbox Simulator. For more information, see <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/verify-addresses-and-domains.html">Verifying Email Addresses and Domains</a> in the <i>Amazon SES Developer Guide.</i> </p> </li> <li> <p>The total size of the message, including attachments, must be smaller than 10 MB.</p> </li> <li> <p>The message must include at least one recipient email address. The recipient address can be a To: address, a CC: address, or a BCC: address. If a recipient email address is invalid (that is, it is not in the format <i>UserName@[SubDomain.]Domain.TopLevelDomain</i>), the entire message will be rejected, even if the message contains other recipients that are valid.</p> </li> <li> <p>The message may not include more than 50 recipients, across the To:, CC: and BCC: fields. If you need to send an email message to a larger audience, you can divide your recipient list into groups of 50 or fewer, and then call the <code>SendEmail</code> operation several times to send the message to each group.</p> </li> </ul> <important> <p>For every message that you send, the total number of recipients (including each recipient in the To:, CC: and BCC: fields) is counted against the maximum number of emails you can send in a 24-hour period (your <i>sending quota</i>). For more information about sending quotas in Amazon SES, see <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/manage-sending-limits.html">Managing Your Amazon SES Sending Limits</a> in the <i>Amazon SES Developer Guide.</i> </p> </important></p>
    fn send_email(
        &self,
        input: SendEmailRequest,
    ) -> RusotoFuture<SendEmailResponse, SendEmailError> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "SendEmail");
        params.put("Version", "2010-12-01");
        SendEmailRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(SendEmailError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = SendEmailResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(SendEmailResponseDeserializer::deserialize(
                        "SendEmailResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Composes an email message and immediately queues it for sending. When calling this operation, you may specify the message headers as well as the content. The <code>SendRawEmail</code> operation is particularly useful for sending multipart MIME emails (such as those that contain both a plain-text and an HTML version). </p> <p>In order to send email using the <code>SendRawEmail</code> operation, your message must meet the following requirements:</p> <ul> <li> <p>The message must be sent from a verified email address or domain. If you attempt to send email using a non-verified address or domain, the operation will result in an &quot;Email address not verified&quot; error. </p> </li> <li> <p>If your account is still in the Amazon SES sandbox, you may only send to verified addresses or domains, or to email addresses associated with the Amazon SES Mailbox Simulator. For more information, see <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/verify-addresses-and-domains.html">Verifying Email Addresses and Domains</a> in the <i>Amazon SES Developer Guide.</i> </p> </li> <li> <p>The total size of the message, including attachments, must be smaller than 10 MB.</p> </li> <li> <p>The message must include at least one recipient email address. The recipient address can be a To: address, a CC: address, or a BCC: address. If a recipient email address is invalid (that is, it is not in the format <i>UserName@[SubDomain.]Domain.TopLevelDomain</i>), the entire message will be rejected, even if the message contains other recipients that are valid.</p> </li> <li> <p>The message may not include more than 50 recipients, across the To:, CC: and BCC: fields. If you need to send an email message to a larger audience, you can divide your recipient list into groups of 50 or fewer, and then call the <code>SendRawEmail</code> operation several times to send the message to each group.</p> </li> </ul> <important> <p>For every message that you send, the total number of recipients (including each recipient in the To:, CC: and BCC: fields) is counted against the maximum number of emails you can send in a 24-hour period (your <i>sending quota</i>). For more information about sending quotas in Amazon SES, see <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/manage-sending-limits.html">Managing Your Amazon SES Sending Limits</a> in the <i>Amazon SES Developer Guide.</i> </p> </important> <p>Additionally, keep the following considerations in mind when using the <code>SendRawEmail</code> operation:</p> <ul> <li> <p>Although you can customize the message headers when using the <code>SendRawEmail</code> operation, Amazon SES will automatically apply its own <code>Message-ID</code> and <code>Date</code> headers; if you passed these headers when creating the message, they will be overwritten by the values that Amazon SES provides.</p> </li> <li> <p>If you are using sending authorization to send on behalf of another user, <code>SendRawEmail</code> enables you to specify the cross-account identity for the email&#39;s Source, From, and Return-Path parameters in one of two ways: you can pass optional parameters <code>SourceArn</code>, <code>FromArn</code>, and/or <code>ReturnPathArn</code> to the API, or you can include the following X-headers in the header of your raw email:</p> <ul> <li> <p> <code>X-SES-SOURCE-ARN</code> </p> </li> <li> <p> <code>X-SES-FROM-ARN</code> </p> </li> <li> <p> <code>X-SES-RETURN-PATH-ARN</code> </p> </li> </ul> <important> <p>Do not include these X-headers in the DKIM signature; Amazon SES will remove them before sending the email.</p> </important> <p>For most common sending authorization scenarios, we recommend that you specify the <code>SourceIdentityArn</code> parameter and not the <code>FromIdentityArn</code> or <code>ReturnPathIdentityArn</code> parameters. If you only specify the <code>SourceIdentityArn</code> parameter, Amazon SES will set the From and Return Path addresses to the identity specified in <code>SourceIdentityArn</code>. For more information about sending authorization, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/sending-authorization.html">Using Sending Authorization with Amazon SES</a> in the <i>Amazon SES Developer Guide.</i> </p> </li> </ul></p>
    fn send_raw_email(
        &self,
        input: SendRawEmailRequest,
    ) -> RusotoFuture<SendRawEmailResponse, SendRawEmailError> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "SendRawEmail");
        params.put("Version", "2010-12-01");
        SendRawEmailRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(SendRawEmailError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = SendRawEmailResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(SendRawEmailResponseDeserializer::deserialize(
                        "SendRawEmailResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Composes an email message using an email template and immediately queues it for sending.</p> <p>In order to send email using the <code>SendTemplatedEmail</code> operation, your call to the API must meet the following requirements:</p> <ul> <li> <p>The call must refer to an existing email template. You can create email templates using the <a>CreateTemplate</a> operation.</p> </li> <li> <p>The message must be sent from a verified email address or domain.</p> </li> <li> <p>If your account is still in the Amazon SES sandbox, you may only send to verified addresses or domains, or to email addresses associated with the Amazon SES Mailbox Simulator. For more information, see <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/verify-addresses-and-domains.html">Verifying Email Addresses and Domains</a> in the <i>Amazon SES Developer Guide.</i> </p> </li> <li> <p>The total size of the message, including attachments, must be less than 10 MB.</p> </li> <li> <p>Calls to the <code>SendTemplatedEmail</code> operation may only include one <code>Destination</code> parameter. A destination is a set of recipients who will receive the same version of the email. The <code>Destination</code> parameter can include up to 50 recipients, across the To:, CC: and BCC: fields.</p> </li> <li> <p>The <code>Destination</code> parameter must include at least one recipient email address. The recipient address can be a To: address, a CC: address, or a BCC: address. If a recipient email address is invalid (that is, it is not in the format <i>UserName@[SubDomain.]Domain.TopLevelDomain</i>), the entire message will be rejected, even if the message contains other recipients that are valid.</p> </li> </ul></p>
    fn send_templated_email(
        &self,
        input: SendTemplatedEmailRequest,
    ) -> RusotoFuture<SendTemplatedEmailResponse, SendTemplatedEmailError> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "SendTemplatedEmail");
        params.put("Version", "2010-12-01");
        SendTemplatedEmailRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(SendTemplatedEmailError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = SendTemplatedEmailResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(SendTemplatedEmailResponseDeserializer::deserialize(
                        "SendTemplatedEmailResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Sets the specified receipt rule set as the active receipt rule set.</p> <note> <p>To disable your email-receiving through Amazon SES completely, you can call this API with RuleSetName set to null.</p> </note> <p>For information about managing receipt rule sets, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-managing-receipt-rule-sets.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    fn set_active_receipt_rule_set(
        &self,
        input: SetActiveReceiptRuleSetRequest,
    ) -> RusotoFuture<SetActiveReceiptRuleSetResponse, SetActiveReceiptRuleSetError> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "SetActiveReceiptRuleSet");
        params.put("Version", "2010-12-01");
        SetActiveReceiptRuleSetRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(SetActiveReceiptRuleSetError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = SetActiveReceiptRuleSetResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(SetActiveReceiptRuleSetResponseDeserializer::deserialize(
                        "SetActiveReceiptRuleSetResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Enables or disables Easy DKIM signing of email sent from an identity:</p> <ul> <li> <p>If Easy DKIM signing is enabled for a domain name identity (such as <code>example.com</code>), then Amazon SES will DKIM-sign all email sent by addresses under that domain name (for example, <code>user@example.com</code>).</p> </li> <li> <p>If Easy DKIM signing is enabled for an email address, then Amazon SES will DKIM-sign all email sent by that email address.</p> </li> </ul> <p>For email addresses (for example, <code>user@example.com</code>), you can only enable Easy DKIM signing if the corresponding domain (in this case, <code>example.com</code>) has been set up for Easy DKIM using the AWS Console or the <code>VerifyDomainDkim</code> operation.</p> <p>You can execute this operation no more than once per second.</p> <p>For more information about Easy DKIM signing, go to the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/easy-dkim.html">Amazon SES Developer Guide</a>.</p>
    fn set_identity_dkim_enabled(
        &self,
        input: SetIdentityDkimEnabledRequest,
    ) -> RusotoFuture<SetIdentityDkimEnabledResponse, SetIdentityDkimEnabledError> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "SetIdentityDkimEnabled");
        params.put("Version", "2010-12-01");
        SetIdentityDkimEnabledRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(SetIdentityDkimEnabledError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = SetIdentityDkimEnabledResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(SetIdentityDkimEnabledResponseDeserializer::deserialize(
                        "SetIdentityDkimEnabledResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Given an identity (an email address or a domain), enables or disables whether Amazon SES forwards bounce and complaint notifications as email. Feedback forwarding can only be disabled when Amazon Simple Notification Service (Amazon SNS) topics are specified for both bounces and complaints.</p> <note> <p>Feedback forwarding does not apply to delivery notifications. Delivery notifications are only available through Amazon SNS.</p> </note> <p>You can execute this operation no more than once per second.</p> <p>For more information about using notifications with Amazon SES, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/notifications.html">Amazon SES Developer Guide</a>.</p>
    fn set_identity_feedback_forwarding_enabled(
        &self,
        input: SetIdentityFeedbackForwardingEnabledRequest,
    ) -> RusotoFuture<
        SetIdentityFeedbackForwardingEnabledResponse,
        SetIdentityFeedbackForwardingEnabledError,
    > {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "SetIdentityFeedbackForwardingEnabled");
        params.put("Version", "2010-12-01");
        SetIdentityFeedbackForwardingEnabledRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(SetIdentityFeedbackForwardingEnabledError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = SetIdentityFeedbackForwardingEnabledResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(
                        SetIdentityFeedbackForwardingEnabledResponseDeserializer::deserialize(
                            "SetIdentityFeedbackForwardingEnabledResult",
                            &mut stack
                        )
                    );
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Given an identity (an email address or a domain), sets whether Amazon SES includes the original email headers in the Amazon Simple Notification Service (Amazon SNS) notifications of a specified type.</p> <p>You can execute this operation no more than once per second.</p> <p>For more information about using notifications with Amazon SES, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/notifications.html">Amazon SES Developer Guide</a>.</p>
    fn set_identity_headers_in_notifications_enabled(
        &self,
        input: SetIdentityHeadersInNotificationsEnabledRequest,
    ) -> RusotoFuture<
        SetIdentityHeadersInNotificationsEnabledResponse,
        SetIdentityHeadersInNotificationsEnabledError,
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
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(SetIdentityHeadersInNotificationsEnabledError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = SetIdentityHeadersInNotificationsEnabledResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(
                        SetIdentityHeadersInNotificationsEnabledResponseDeserializer::deserialize(
                            "SetIdentityHeadersInNotificationsEnabledResult",
                            &mut stack
                        )
                    );
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Enables or disables the custom MAIL FROM domain setup for a verified identity (an email address or a domain).</p> <important> <p>To send emails using the specified MAIL FROM domain, you must add an MX record to your MAIL FROM domain's DNS settings. If you want your emails to pass Sender Policy Framework (SPF) checks, you must also add or update an SPF record. For more information, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/mail-from-set.html">Amazon SES Developer Guide</a>.</p> </important> <p>You can execute this operation no more than once per second.</p>
    fn set_identity_mail_from_domain(
        &self,
        input: SetIdentityMailFromDomainRequest,
    ) -> RusotoFuture<SetIdentityMailFromDomainResponse, SetIdentityMailFromDomainError> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "SetIdentityMailFromDomain");
        params.put("Version", "2010-12-01");
        SetIdentityMailFromDomainRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(SetIdentityMailFromDomainError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = SetIdentityMailFromDomainResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(SetIdentityMailFromDomainResponseDeserializer::deserialize(
                        "SetIdentityMailFromDomainResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Given an identity (an email address or a domain), sets the Amazon Simple Notification Service (Amazon SNS) topic to which Amazon SES will publish bounce, complaint, and/or delivery notifications for emails sent with that identity as the <code>Source</code>.</p> <note> <p>Unless feedback forwarding is enabled, you must specify Amazon SNS topics for bounce and complaint notifications. For more information, see <code>SetIdentityFeedbackForwardingEnabled</code>.</p> </note> <p>You can execute this operation no more than once per second.</p> <p>For more information about feedback notification, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/notifications.html">Amazon SES Developer Guide</a>.</p>
    fn set_identity_notification_topic(
        &self,
        input: SetIdentityNotificationTopicRequest,
    ) -> RusotoFuture<SetIdentityNotificationTopicResponse, SetIdentityNotificationTopicError> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "SetIdentityNotificationTopic");
        params.put("Version", "2010-12-01");
        SetIdentityNotificationTopicRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(SetIdentityNotificationTopicError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = SetIdentityNotificationTopicResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(
                        SetIdentityNotificationTopicResponseDeserializer::deserialize(
                            "SetIdentityNotificationTopicResult",
                            &mut stack
                        )
                    );
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Sets the position of the specified receipt rule in the receipt rule set.</p> <p>For information about managing receipt rules, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-managing-receipt-rules.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    fn set_receipt_rule_position(
        &self,
        input: SetReceiptRulePositionRequest,
    ) -> RusotoFuture<SetReceiptRulePositionResponse, SetReceiptRulePositionError> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "SetReceiptRulePosition");
        params.put("Version", "2010-12-01");
        SetReceiptRulePositionRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(SetReceiptRulePositionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = SetReceiptRulePositionResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(SetReceiptRulePositionResponseDeserializer::deserialize(
                        "SetReceiptRulePositionResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates a preview of the MIME content of an email when provided with a template and a set of replacement data.</p> <p>You can execute this operation no more than once per second.</p>
    fn test_render_template(
        &self,
        input: TestRenderTemplateRequest,
    ) -> RusotoFuture<TestRenderTemplateResponse, TestRenderTemplateError> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "TestRenderTemplate");
        params.put("Version", "2010-12-01");
        TestRenderTemplateRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(TestRenderTemplateError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = TestRenderTemplateResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(TestRenderTemplateResponseDeserializer::deserialize(
                        "TestRenderTemplateResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Enables or disables email sending across your entire Amazon SES account. You can use this operation in conjunction with Amazon CloudWatch alarms to temporarily pause email sending across your Amazon SES account when reputation metrics (such as your bounce on complaint rate) reach certain thresholds.</p> <p>You can execute this operation no more than once per second.</p>
    fn update_account_sending_enabled(
        &self,
        input: UpdateAccountSendingEnabledRequest,
    ) -> RusotoFuture<(), UpdateAccountSendingEnabledError> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "UpdateAccountSendingEnabled");
        params.put("Version", "2010-12-01");
        UpdateAccountSendingEnabledRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateAccountSendingEnabledError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(future::ok(::std::mem::drop(response)))
        });

        RusotoFuture::new(future)
    }

    /// <p>Updates the event destination of a configuration set. Event destinations are associated with configuration sets, which enable you to publish email sending events to Amazon CloudWatch, Amazon Kinesis Firehose, or Amazon Simple Notification Service (Amazon SNS). For information about using configuration sets, see <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/monitor-sending-activity.html">Monitoring Your Amazon SES Sending Activity</a> in the <i>Amazon SES Developer Guide.</i> </p> <note> <p>When you create or update an event destination, you must provide one, and only one, destination. The destination can be Amazon CloudWatch, Amazon Kinesis Firehose, or Amazon Simple Notification Service (Amazon SNS).</p> </note> <p>You can execute this operation no more than once per second.</p>
    fn update_configuration_set_event_destination(
        &self,
        input: UpdateConfigurationSetEventDestinationRequest,
    ) -> RusotoFuture<
        UpdateConfigurationSetEventDestinationResponse,
        UpdateConfigurationSetEventDestinationError,
    > {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "UpdateConfigurationSetEventDestination");
        params.put("Version", "2010-12-01");
        UpdateConfigurationSetEventDestinationRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateConfigurationSetEventDestinationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = UpdateConfigurationSetEventDestinationResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(
                        UpdateConfigurationSetEventDestinationResponseDeserializer::deserialize(
                            "UpdateConfigurationSetEventDestinationResult",
                            &mut stack
                        )
                    );
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Enables or disables the publishing of reputation metrics for emails sent using a specific configuration set. Reputation metrics include bounce and complaint rates. These metrics are published to Amazon CloudWatch. By using Amazon CloudWatch, you can create alarms when bounce or complaint rates exceed a certain threshold.</p> <p>You can execute this operation no more than once per second.</p>
    fn update_configuration_set_reputation_metrics_enabled(
        &self,
        input: UpdateConfigurationSetReputationMetricsEnabledRequest,
    ) -> RusotoFuture<(), UpdateConfigurationSetReputationMetricsEnabledError> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "UpdateConfigurationSetReputationMetricsEnabled");
        params.put("Version", "2010-12-01");
        UpdateConfigurationSetReputationMetricsEnabledRequestSerializer::serialize(
            &mut params,
            "",
            &input,
        );
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(
                        UpdateConfigurationSetReputationMetricsEnabledError::from_body(
                            String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                        ),
                    )
                }));
            }

            future::Either::A(future::ok(::std::mem::drop(response)))
        });

        RusotoFuture::new(future)
    }

    /// <p>Enables or disables email sending for messages sent using a specific configuration set. You can use this operation in conjunction with Amazon CloudWatch alarms to temporarily pause email sending for a configuration set when the reputation metrics for that configuration set (such as your bounce on complaint rate) reach certain thresholds.</p> <p>You can execute this operation no more than once per second.</p>
    fn update_configuration_set_sending_enabled(
        &self,
        input: UpdateConfigurationSetSendingEnabledRequest,
    ) -> RusotoFuture<(), UpdateConfigurationSetSendingEnabledError> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "UpdateConfigurationSetSendingEnabled");
        params.put("Version", "2010-12-01");
        UpdateConfigurationSetSendingEnabledRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateConfigurationSetSendingEnabledError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(future::ok(::std::mem::drop(response)))
        });

        RusotoFuture::new(future)
    }

    /// <p>Modifies an association between a configuration set and a custom domain for open and click event tracking. </p> <p>By default, images and links used for tracking open and click events are hosted on domains operated by Amazon SES. You can configure a subdomain of your own to handle these events. For information about using configuration sets, see <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/configure-custom-open-click-domains.html">Configuring Custom Domains to Handle Open and Click Tracking</a> in the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/Welcome.html">Amazon SES Developer Guide</a>.</p>
    fn update_configuration_set_tracking_options(
        &self,
        input: UpdateConfigurationSetTrackingOptionsRequest,
    ) -> RusotoFuture<
        UpdateConfigurationSetTrackingOptionsResponse,
        UpdateConfigurationSetTrackingOptionsError,
    > {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "UpdateConfigurationSetTrackingOptions");
        params.put("Version", "2010-12-01");
        UpdateConfigurationSetTrackingOptionsRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateConfigurationSetTrackingOptionsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = UpdateConfigurationSetTrackingOptionsResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(
                        UpdateConfigurationSetTrackingOptionsResponseDeserializer::deserialize(
                            "UpdateConfigurationSetTrackingOptionsResult",
                            &mut stack
                        )
                    );
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Updates an existing custom verification email template.</p> <p>For more information about custom verification email templates, see <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/custom-verification-emails.html">Using Custom Verification Email Templates</a> in the <i>Amazon SES Developer Guide</i>.</p> <p>You can execute this operation no more than once per second.</p>
    fn update_custom_verification_email_template(
        &self,
        input: UpdateCustomVerificationEmailTemplateRequest,
    ) -> RusotoFuture<(), UpdateCustomVerificationEmailTemplateError> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "UpdateCustomVerificationEmailTemplate");
        params.put("Version", "2010-12-01");
        UpdateCustomVerificationEmailTemplateRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateCustomVerificationEmailTemplateError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(future::ok(::std::mem::drop(response)))
        });

        RusotoFuture::new(future)
    }

    /// <p>Updates a receipt rule.</p> <p>For information about managing receipt rules, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-managing-receipt-rules.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    fn update_receipt_rule(
        &self,
        input: UpdateReceiptRuleRequest,
    ) -> RusotoFuture<UpdateReceiptRuleResponse, UpdateReceiptRuleError> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "UpdateReceiptRule");
        params.put("Version", "2010-12-01");
        UpdateReceiptRuleRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateReceiptRuleError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = UpdateReceiptRuleResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(UpdateReceiptRuleResponseDeserializer::deserialize(
                        "UpdateReceiptRuleResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Updates an email template. Email templates enable you to send personalized email to one or more destinations in a single API operation. For more information, see the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/send-personalized-email-api.html">Amazon SES Developer Guide</a>.</p> <p>You can execute this operation no more than once per second.</p>
    fn update_template(
        &self,
        input: UpdateTemplateRequest,
    ) -> RusotoFuture<UpdateTemplateResponse, UpdateTemplateError> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "UpdateTemplate");
        params.put("Version", "2010-12-01");
        UpdateTemplateRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateTemplateError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = UpdateTemplateResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(UpdateTemplateResponseDeserializer::deserialize(
                        "UpdateTemplateResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns a set of DKIM tokens for a domain. DKIM <i>tokens</i> are character strings that represent your domain's identity. Using these tokens, you will need to create DNS CNAME records that point to DKIM public keys hosted by Amazon SES. Amazon Web Services will eventually detect that you have updated your DNS records; this detection process may take up to 72 hours. Upon successful detection, Amazon SES will be able to DKIM-sign email originating from that domain.</p> <p>You can execute this operation no more than once per second.</p> <p>To enable or disable Easy DKIM signing for a domain, use the <code>SetIdentityDkimEnabled</code> operation.</p> <p>For more information about creating DNS records using DKIM tokens, go to the <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/easy-dkim-dns-records.html">Amazon SES Developer Guide</a>.</p>
    fn verify_domain_dkim(
        &self,
        input: VerifyDomainDkimRequest,
    ) -> RusotoFuture<VerifyDomainDkimResponse, VerifyDomainDkimError> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "VerifyDomainDkim");
        params.put("Version", "2010-12-01");
        VerifyDomainDkimRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(VerifyDomainDkimError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = VerifyDomainDkimResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(VerifyDomainDkimResponseDeserializer::deserialize(
                        "VerifyDomainDkimResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Adds a domain to the list of identities for your Amazon SES account and attempts to verify it. For more information about verifying domains, see <a href="http://docs.aws.amazon.com/ses/latest/DeveloperGuide/verify-addresses-and-domains.html">Verifying Email Addresses and Domains</a> in the <i>Amazon SES Developer Guide.</i> </p> <p>You can execute this operation no more than once per second.</p>
    fn verify_domain_identity(
        &self,
        input: VerifyDomainIdentityRequest,
    ) -> RusotoFuture<VerifyDomainIdentityResponse, VerifyDomainIdentityError> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "VerifyDomainIdentity");
        params.put("Version", "2010-12-01");
        VerifyDomainIdentityRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(VerifyDomainIdentityError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = VerifyDomainIdentityResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(VerifyDomainIdentityResponseDeserializer::deserialize(
                        "VerifyDomainIdentityResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Deprecated. Use the <code>VerifyEmailIdentity</code> operation to verify a new email address.</p>
    fn verify_email_address(
        &self,
        input: VerifyEmailAddressRequest,
    ) -> RusotoFuture<(), VerifyEmailAddressError> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "VerifyEmailAddress");
        params.put("Version", "2010-12-01");
        VerifyEmailAddressRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(VerifyEmailAddressError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(future::ok(::std::mem::drop(response)))
        });

        RusotoFuture::new(future)
    }

    /// <p>Adds an email address to the list of identities for your Amazon SES account and attempts to verify it. As a result of executing this operation, a verification email is sent to the specified address.</p> <p>You can execute this operation no more than once per second.</p>
    fn verify_email_identity(
        &self,
        input: VerifyEmailIdentityRequest,
    ) -> RusotoFuture<VerifyEmailIdentityResponse, VerifyEmailIdentityError> {
        let mut request = SignedRequest::new("POST", "email", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "VerifyEmailIdentity");
        params.put("Version", "2010-12-01");
        VerifyEmailIdentityRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(VerifyEmailIdentityError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = VerifyEmailIdentityResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(VerifyEmailIdentityResponseDeserializer::deserialize(
                        "VerifyEmailIdentityResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }
}

#[cfg(test)]
mod protocol_tests {

    extern crate rusoto_mock;

    use self::rusoto_mock::*;
    use super::*;
    use rusoto_core::Region as rusoto_region;

    #[test]
    fn test_parse_error_ses_delete_identity() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/error",
            "ses-delete-identity.xml",
        );
        let mock = MockRequestDispatcher::with_status(400).with_body(&mock_response);
        let client = SesClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = DeleteIdentityRequest::default();
        let result = client.delete_identity(request).sync();
        assert!(!result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_ses_delete_identity() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "ses-delete-identity.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = SesClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = DeleteIdentityRequest::default();
        let result = client.delete_identity(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_ses_get_identity_dkim_attributes() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "ses-get-identity-dkim-attributes.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = SesClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = GetIdentityDkimAttributesRequest::default();
        let result = client.get_identity_dkim_attributes(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_ses_get_identity_notification_attributes() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "ses-get-identity-notification-attributes.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = SesClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = GetIdentityNotificationAttributesRequest::default();
        let result = client.get_identity_notification_attributes(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_ses_get_identity_verification_attributes() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "ses-get-identity-verification-attributes.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = SesClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = GetIdentityVerificationAttributesRequest::default();
        let result = client.get_identity_verification_attributes(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_ses_get_send_quota() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "ses-get-send-quota.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = SesClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);

        let result = client.get_send_quota().sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_ses_get_send_statistics() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "ses-get-send-statistics.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = SesClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);

        let result = client.get_send_statistics().sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_ses_list_identities() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "ses-list-identities.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = SesClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = ListIdentitiesRequest::default();
        let result = client.list_identities(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_ses_send_email() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "ses-send-email.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = SesClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = SendEmailRequest::default();
        let result = client.send_email(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_ses_send_raw_email() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "ses-send-raw-email.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = SesClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = SendRawEmailRequest::default();
        let result = client.send_raw_email(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_ses_set_identity_dkim_enabled() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "ses-set-identity-dkim-enabled.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = SesClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = SetIdentityDkimEnabledRequest::default();
        let result = client.set_identity_dkim_enabled(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_ses_verify_domain_dkim() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "ses-verify-domain-dkim.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = SesClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = VerifyDomainDkimRequest::default();
        let result = client.verify_domain_dkim(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_ses_verify_domain_identity() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "ses-verify-domain-identity.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = SesClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = VerifyDomainIdentityRequest::default();
        let result = client.verify_domain_identity(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }
}
