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
struct AccountDeserializer;
impl AccountDeserializer {
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

/// Serialize `ActionsList` contents to a `SignedRequest`.
struct ActionsListSerializer;
impl ActionsListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct AddPermissionInput {
    /// <p>The AWS account IDs of the users (principals) who will be given access to the specified actions. The users must have AWS accounts, but do not need to be signed up for this service.</p>
    pub aws_account_id: Vec<String>,
    /// <p>The action you want to allow for the specified principal(s).</p> <p>Valid values: any Amazon SNS action name.</p>
    pub action_name: Vec<String>,
    /// <p>A unique identifier for the new policy statement.</p>
    pub label: String,
    /// <p>The ARN of the topic whose access control policy you wish to modify.</p>
    pub topic_arn: String,
}

/// Serialize `AddPermissionInput` contents to a `SignedRequest`.
struct AddPermissionInputSerializer;
impl AddPermissionInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &AddPermissionInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        DelegatesListSerializer::serialize(
            params,
            &format!("{}{}", prefix, "AWSAccountId"),
            &obj.aws_account_id,
        );
        ActionsListSerializer::serialize(
            params,
            &format!("{}{}", prefix, "ActionName"),
            &obj.action_name,
        );
        params.put(
            &format!("{}{}", prefix, "Label"),
            &obj.label.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "TopicArn"),
            &obj.topic_arn.replace("+", "%2B"),
        );
    }
}

struct AttributeNameDeserializer;
impl AttributeNameDeserializer {
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
struct AttributeValueDeserializer;
impl AttributeValueDeserializer {
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
struct BooleanDeserializer;
impl BooleanDeserializer {
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
/// <p>The input for the <code>CheckIfPhoneNumberIsOptedOut</code> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CheckIfPhoneNumberIsOptedOutInput {
    /// <p>The phone number for which you want to check the opt out status.</p>
    pub phone_number: String,
}

/// Serialize `CheckIfPhoneNumberIsOptedOutInput` contents to a `SignedRequest`.
struct CheckIfPhoneNumberIsOptedOutInputSerializer;
impl CheckIfPhoneNumberIsOptedOutInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CheckIfPhoneNumberIsOptedOutInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "phoneNumber"),
            &obj.phone_number.replace("+", "%2B"),
        );
    }
}

/// <p>The response from the <code>CheckIfPhoneNumberIsOptedOut</code> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CheckIfPhoneNumberIsOptedOutResponse {
    /// <p><p>Indicates whether the phone number is opted out:</p> <ul> <li> <p> <code>true</code> – The phone number is opted out, meaning you cannot publish SMS messages to it.</p> </li> <li> <p> <code>false</code> – The phone number is opted in, meaning you can publish SMS messages to it.</p> </li> </ul></p>
    pub is_opted_out: Option<bool>,
}

struct CheckIfPhoneNumberIsOptedOutResponseDeserializer;
impl CheckIfPhoneNumberIsOptedOutResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CheckIfPhoneNumberIsOptedOutResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CheckIfPhoneNumberIsOptedOutResponse::default();

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
                    "isOptedOut" => {
                        obj.is_opted_out =
                            Some(try!(BooleanDeserializer::deserialize("isOptedOut", stack)));
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
/// <p>Input for ConfirmSubscription action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ConfirmSubscriptionInput {
    /// <p>Disallows unauthenticated unsubscribes of the subscription. If the value of this parameter is <code>true</code> and the request has an AWS signature, then only the topic owner and the subscription owner can unsubscribe the endpoint. The unsubscribe action requires AWS authentication. </p>
    pub authenticate_on_unsubscribe: Option<String>,
    /// <p>Short-lived token sent to an endpoint during the <code>Subscribe</code> action.</p>
    pub token: String,
    /// <p>The ARN of the topic for which you wish to confirm a subscription.</p>
    pub topic_arn: String,
}

/// Serialize `ConfirmSubscriptionInput` contents to a `SignedRequest`.
struct ConfirmSubscriptionInputSerializer;
impl ConfirmSubscriptionInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ConfirmSubscriptionInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.authenticate_on_unsubscribe {
            params.put(
                &format!("{}{}", prefix, "AuthenticateOnUnsubscribe"),
                &field_value.replace("+", "%2B"),
            );
        }
        params.put(
            &format!("{}{}", prefix, "Token"),
            &obj.token.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "TopicArn"),
            &obj.topic_arn.replace("+", "%2B"),
        );
    }
}

/// <p>Response for ConfirmSubscriptions action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ConfirmSubscriptionResponse {
    /// <p>The ARN of the created subscription.</p>
    pub subscription_arn: Option<String>,
}

struct ConfirmSubscriptionResponseDeserializer;
impl ConfirmSubscriptionResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ConfirmSubscriptionResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ConfirmSubscriptionResponse::default();

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
                    "SubscriptionArn" => {
                        obj.subscription_arn = Some(try!(
                            SubscriptionARNDeserializer::deserialize("SubscriptionArn", stack)
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
/// <p>Response from CreateEndpoint action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateEndpointResponse {
    /// <p>EndpointArn returned from CreateEndpoint action.</p>
    pub endpoint_arn: Option<String>,
}

struct CreateEndpointResponseDeserializer;
impl CreateEndpointResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateEndpointResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CreateEndpointResponse::default();

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
                    "EndpointArn" => {
                        obj.endpoint_arn =
                            Some(try!(StringDeserializer::deserialize("EndpointArn", stack)));
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
/// <p>Input for CreatePlatformApplication action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreatePlatformApplicationInput {
    /// <p>For a list of attributes, see <a href="http://docs.aws.amazon.com/sns/latest/api/API_SetPlatformApplicationAttributes.html">SetPlatformApplicationAttributes</a> </p>
    pub attributes: ::std::collections::HashMap<String, String>,
    /// <p>Application names must be made up of only uppercase and lowercase ASCII letters, numbers, underscores, hyphens, and periods, and must be between 1 and 256 characters long.</p>
    pub name: String,
    /// <p>The following platforms are supported: ADM (Amazon Device Messaging), APNS (Apple Push Notification Service), APNS_SANDBOX, and GCM (Google Cloud Messaging).</p>
    pub platform: String,
}

/// Serialize `CreatePlatformApplicationInput` contents to a `SignedRequest`.
struct CreatePlatformApplicationInputSerializer;
impl CreatePlatformApplicationInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CreatePlatformApplicationInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        MapStringToStringSerializer::serialize(
            params,
            &format!("{}{}", prefix, "Attributes"),
            &obj.attributes,
        );
        params.put(
            &format!("{}{}", prefix, "Name"),
            &obj.name.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "Platform"),
            &obj.platform.replace("+", "%2B"),
        );
    }
}

/// <p>Response from CreatePlatformApplication action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreatePlatformApplicationResponse {
    /// <p>PlatformApplicationArn is returned.</p>
    pub platform_application_arn: Option<String>,
}

struct CreatePlatformApplicationResponseDeserializer;
impl CreatePlatformApplicationResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreatePlatformApplicationResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CreatePlatformApplicationResponse::default();

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
                    "PlatformApplicationArn" => {
                        obj.platform_application_arn = Some(try!(StringDeserializer::deserialize(
                            "PlatformApplicationArn",
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
/// <p>Input for CreatePlatformEndpoint action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreatePlatformEndpointInput {
    /// <p>For a list of attributes, see <a href="http://docs.aws.amazon.com/sns/latest/api/API_SetEndpointAttributes.html">SetEndpointAttributes</a>.</p>
    pub attributes: Option<::std::collections::HashMap<String, String>>,
    /// <p>Arbitrary user data to associate with the endpoint. Amazon SNS does not use this data. The data must be in UTF-8 format and less than 2KB.</p>
    pub custom_user_data: Option<String>,
    /// <p>PlatformApplicationArn returned from CreatePlatformApplication is used to create a an endpoint.</p>
    pub platform_application_arn: String,
    /// <p>Unique identifier created by the notification service for an app on a device. The specific name for Token will vary, depending on which notification service is being used. For example, when using APNS as the notification service, you need the device token. Alternatively, when using GCM or ADM, the device token equivalent is called the registration ID.</p>
    pub token: String,
}

/// Serialize `CreatePlatformEndpointInput` contents to a `SignedRequest`.
struct CreatePlatformEndpointInputSerializer;
impl CreatePlatformEndpointInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CreatePlatformEndpointInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.attributes {
            MapStringToStringSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Attributes"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.custom_user_data {
            params.put(
                &format!("{}{}", prefix, "CustomUserData"),
                &field_value.replace("+", "%2B"),
            );
        }
        params.put(
            &format!("{}{}", prefix, "PlatformApplicationArn"),
            &obj.platform_application_arn.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "Token"),
            &obj.token.replace("+", "%2B"),
        );
    }
}

/// <p>Input for CreateTopic action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateTopicInput {
    /// <p>The name of the topic you want to create.</p> <p>Constraints: Topic names must be made up of only uppercase and lowercase ASCII letters, numbers, underscores, and hyphens, and must be between 1 and 256 characters long.</p>
    pub name: String,
}

/// Serialize `CreateTopicInput` contents to a `SignedRequest`.
struct CreateTopicInputSerializer;
impl CreateTopicInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CreateTopicInput) {
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

/// <p>Response from CreateTopic action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateTopicResponse {
    /// <p>The Amazon Resource Name (ARN) assigned to the created topic.</p>
    pub topic_arn: Option<String>,
}

struct CreateTopicResponseDeserializer;
impl CreateTopicResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateTopicResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CreateTopicResponse::default();

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
                    "TopicArn" => {
                        obj.topic_arn =
                            Some(try!(TopicARNDeserializer::deserialize("TopicArn", stack)));
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

/// Serialize `DelegatesList` contents to a `SignedRequest`.
struct DelegatesListSerializer;
impl DelegatesListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

/// <p>Input for DeleteEndpoint action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteEndpointInput {
    /// <p>EndpointArn of endpoint to delete.</p>
    pub endpoint_arn: String,
}

/// Serialize `DeleteEndpointInput` contents to a `SignedRequest`.
struct DeleteEndpointInputSerializer;
impl DeleteEndpointInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DeleteEndpointInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "EndpointArn"),
            &obj.endpoint_arn.replace("+", "%2B"),
        );
    }
}

/// <p>Input for DeletePlatformApplication action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeletePlatformApplicationInput {
    /// <p>PlatformApplicationArn of platform application object to delete.</p>
    pub platform_application_arn: String,
}

/// Serialize `DeletePlatformApplicationInput` contents to a `SignedRequest`.
struct DeletePlatformApplicationInputSerializer;
impl DeletePlatformApplicationInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DeletePlatformApplicationInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "PlatformApplicationArn"),
            &obj.platform_application_arn.replace("+", "%2B"),
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteTopicInput {
    /// <p>The ARN of the topic you want to delete.</p>
    pub topic_arn: String,
}

/// Serialize `DeleteTopicInput` contents to a `SignedRequest`.
struct DeleteTopicInputSerializer;
impl DeleteTopicInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DeleteTopicInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "TopicArn"),
            &obj.topic_arn.replace("+", "%2B"),
        );
    }
}

struct EndpointDeserializer;
impl EndpointDeserializer {
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
/// <p>Input for GetEndpointAttributes action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetEndpointAttributesInput {
    /// <p>EndpointArn for GetEndpointAttributes input.</p>
    pub endpoint_arn: String,
}

/// Serialize `GetEndpointAttributesInput` contents to a `SignedRequest`.
struct GetEndpointAttributesInputSerializer;
impl GetEndpointAttributesInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &GetEndpointAttributesInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "EndpointArn"),
            &obj.endpoint_arn.replace("+", "%2B"),
        );
    }
}

/// <p>Response from GetEndpointAttributes of the EndpointArn.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetEndpointAttributesResponse {
    /// <p><p>Attributes include the following:</p> <ul> <li> <p> <code>CustomUserData</code> -- arbitrary user data to associate with the endpoint. Amazon SNS does not use this data. The data must be in UTF-8 format and less than 2KB.</p> </li> <li> <p> <code>Enabled</code> -- flag that enables/disables delivery to the endpoint. Amazon SNS will set this to false when a notification service indicates to Amazon SNS that the endpoint is invalid. Users can set it back to true, typically after updating Token.</p> </li> <li> <p> <code>Token</code> -- device token, also referred to as a registration id, for an app and mobile device. This is returned from the notification service when an app and mobile device are registered with the notification service.</p> </li> </ul></p>
    pub attributes: Option<::std::collections::HashMap<String, String>>,
}

struct GetEndpointAttributesResponseDeserializer;
impl GetEndpointAttributesResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetEndpointAttributesResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = GetEndpointAttributesResponse::default();

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
                    "Attributes" => {
                        obj.attributes = Some(try!(MapStringToStringDeserializer::deserialize(
                            "Attributes",
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
/// <p>Input for GetPlatformApplicationAttributes action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetPlatformApplicationAttributesInput {
    /// <p>PlatformApplicationArn for GetPlatformApplicationAttributesInput.</p>
    pub platform_application_arn: String,
}

/// Serialize `GetPlatformApplicationAttributesInput` contents to a `SignedRequest`.
struct GetPlatformApplicationAttributesInputSerializer;
impl GetPlatformApplicationAttributesInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &GetPlatformApplicationAttributesInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "PlatformApplicationArn"),
            &obj.platform_application_arn.replace("+", "%2B"),
        );
    }
}

/// <p>Response for GetPlatformApplicationAttributes action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetPlatformApplicationAttributesResponse {
    /// <p><p>Attributes include the following:</p> <ul> <li> <p> <code>EventEndpointCreated</code> -- Topic ARN to which EndpointCreated event notifications should be sent.</p> </li> <li> <p> <code>EventEndpointDeleted</code> -- Topic ARN to which EndpointDeleted event notifications should be sent.</p> </li> <li> <p> <code>EventEndpointUpdated</code> -- Topic ARN to which EndpointUpdate event notifications should be sent.</p> </li> <li> <p> <code>EventDeliveryFailure</code> -- Topic ARN to which DeliveryFailure event notifications should be sent upon Direct Publish delivery failure (permanent) to one of the application&#39;s endpoints.</p> </li> </ul></p>
    pub attributes: Option<::std::collections::HashMap<String, String>>,
}

struct GetPlatformApplicationAttributesResponseDeserializer;
impl GetPlatformApplicationAttributesResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetPlatformApplicationAttributesResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = GetPlatformApplicationAttributesResponse::default();

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
                    "Attributes" => {
                        obj.attributes = Some(try!(MapStringToStringDeserializer::deserialize(
                            "Attributes",
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
/// <p>The input for the <code>GetSMSAttributes</code> request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetSMSAttributesInput {
    /// <p>A list of the individual attribute names, such as <code>MonthlySpendLimit</code>, for which you want values.</p> <p>For all attribute names, see <a href="http://docs.aws.amazon.com/sns/latest/api/API_SetSMSAttributes.html">SetSMSAttributes</a>.</p> <p>If you don't use this parameter, Amazon SNS returns all SMS attributes.</p>
    pub attributes: Option<Vec<String>>,
}

/// Serialize `GetSMSAttributesInput` contents to a `SignedRequest`.
struct GetSMSAttributesInputSerializer;
impl GetSMSAttributesInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &GetSMSAttributesInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.attributes {
            ListStringSerializer::serialize(
                params,
                &format!("{}{}", prefix, "attributes"),
                field_value,
            );
        }
    }
}

/// <p>The response from the <code>GetSMSAttributes</code> request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetSMSAttributesResponse {
    /// <p>The SMS attribute names and their values.</p>
    pub attributes: Option<::std::collections::HashMap<String, String>>,
}

struct GetSMSAttributesResponseDeserializer;
impl GetSMSAttributesResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetSMSAttributesResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = GetSMSAttributesResponse::default();

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
                    "attributes" => {
                        obj.attributes = Some(try!(MapStringToStringDeserializer::deserialize(
                            "attributes",
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
/// <p>Input for GetSubscriptionAttributes.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetSubscriptionAttributesInput {
    /// <p>The ARN of the subscription whose properties you want to get.</p>
    pub subscription_arn: String,
}

/// Serialize `GetSubscriptionAttributesInput` contents to a `SignedRequest`.
struct GetSubscriptionAttributesInputSerializer;
impl GetSubscriptionAttributesInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &GetSubscriptionAttributesInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "SubscriptionArn"),
            &obj.subscription_arn.replace("+", "%2B"),
        );
    }
}

/// <p>Response for GetSubscriptionAttributes action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetSubscriptionAttributesResponse {
    /// <p><p>A map of the subscription&#39;s attributes. Attributes in this map include the following:</p> <ul> <li> <p> <code>SubscriptionArn</code> -- the subscription&#39;s ARN</p> </li> <li> <p> <code>TopicArn</code> -- the topic ARN that the subscription is associated with</p> </li> <li> <p> <code>Owner</code> -- the AWS account ID of the subscription&#39;s owner</p> </li> <li> <p> <code>ConfirmationWasAuthenticated</code> -- true if the subscription confirmation request was authenticated</p> </li> <li> <p> <code>DeliveryPolicy</code> -- the JSON serialization of the subscription&#39;s delivery policy</p> </li> <li> <p> <code>EffectiveDeliveryPolicy</code> -- the JSON serialization of the effective delivery policy that takes into account the topic delivery policy and account system defaults</p> </li> </ul></p>
    pub attributes: Option<::std::collections::HashMap<String, String>>,
}

struct GetSubscriptionAttributesResponseDeserializer;
impl GetSubscriptionAttributesResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetSubscriptionAttributesResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = GetSubscriptionAttributesResponse::default();

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
                    "Attributes" => {
                        obj.attributes = Some(try!(
                            SubscriptionAttributesMapDeserializer::deserialize("Attributes", stack)
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
/// <p>Input for GetTopicAttributes action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetTopicAttributesInput {
    /// <p>The ARN of the topic whose properties you want to get.</p>
    pub topic_arn: String,
}

/// Serialize `GetTopicAttributesInput` contents to a `SignedRequest`.
struct GetTopicAttributesInputSerializer;
impl GetTopicAttributesInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &GetTopicAttributesInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "TopicArn"),
            &obj.topic_arn.replace("+", "%2B"),
        );
    }
}

/// <p>Response for GetTopicAttributes action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetTopicAttributesResponse {
    /// <p><p>A map of the topic&#39;s attributes. Attributes in this map include the following:</p> <ul> <li> <p> <code>TopicArn</code> -- the topic&#39;s ARN</p> </li> <li> <p> <code>Owner</code> -- the AWS account ID of the topic&#39;s owner</p> </li> <li> <p> <code>Policy</code> -- the JSON serialization of the topic&#39;s access control policy</p> </li> <li> <p> <code>DisplayName</code> -- the human-readable name used in the &quot;From&quot; field for notifications to email and email-json endpoints</p> </li> <li> <p> <code>SubscriptionsPending</code> -- the number of subscriptions pending confirmation on this topic</p> </li> <li> <p> <code>SubscriptionsConfirmed</code> -- the number of confirmed subscriptions on this topic</p> </li> <li> <p> <code>SubscriptionsDeleted</code> -- the number of deleted subscriptions on this topic</p> </li> <li> <p> <code>DeliveryPolicy</code> -- the JSON serialization of the topic&#39;s delivery policy</p> </li> <li> <p> <code>EffectiveDeliveryPolicy</code> -- the JSON serialization of the effective delivery policy that takes into account system defaults</p> </li> </ul></p>
    pub attributes: Option<::std::collections::HashMap<String, String>>,
}

struct GetTopicAttributesResponseDeserializer;
impl GetTopicAttributesResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetTopicAttributesResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = GetTopicAttributesResponse::default();

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
                    "Attributes" => {
                        obj.attributes = Some(try!(TopicAttributesMapDeserializer::deserialize(
                            "Attributes",
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
/// <p>Input for ListEndpointsByPlatformApplication action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListEndpointsByPlatformApplicationInput {
    /// <p>NextToken string is used when calling ListEndpointsByPlatformApplication action to retrieve additional records that are available after the first page results.</p>
    pub next_token: Option<String>,
    /// <p>PlatformApplicationArn for ListEndpointsByPlatformApplicationInput action.</p>
    pub platform_application_arn: String,
}

/// Serialize `ListEndpointsByPlatformApplicationInput` contents to a `SignedRequest`.
struct ListEndpointsByPlatformApplicationInputSerializer;
impl ListEndpointsByPlatformApplicationInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ListEndpointsByPlatformApplicationInput) {
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
        params.put(
            &format!("{}{}", prefix, "PlatformApplicationArn"),
            &obj.platform_application_arn.replace("+", "%2B"),
        );
    }
}

/// <p>Response for ListEndpointsByPlatformApplication action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListEndpointsByPlatformApplicationResponse {
    /// <p>Endpoints returned for ListEndpointsByPlatformApplication action.</p>
    pub endpoints: Option<Vec<String>>,
    /// <p>NextToken string is returned when calling ListEndpointsByPlatformApplication action if additional records are available after the first page results.</p>
    pub next_token: Option<String>,
}

struct ListEndpointsByPlatformApplicationResponseDeserializer;
impl ListEndpointsByPlatformApplicationResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListEndpointsByPlatformApplicationResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ListEndpointsByPlatformApplicationResponse::default();

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
                    "Endpoints" => {
                        obj.endpoints = Some(try!(ListOfEndpointsDeserializer::deserialize(
                            "Endpoints",
                            stack
                        )));
                    }
                    "NextToken" => {
                        obj.next_token =
                            Some(try!(StringDeserializer::deserialize("NextToken", stack)));
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
struct ListOfEndpointsDeserializer;
impl ListOfEndpointsDeserializer {
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
                        obj.push(try!(EndpointDeserializer::deserialize("member", stack)));
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
struct ListOfPlatformApplicationsDeserializer;
impl ListOfPlatformApplicationsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<PlatformApplication>, XmlParseError> {
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
                        obj.push(try!(PlatformApplicationDeserializer::deserialize(
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
/// <p>The input for the <code>ListPhoneNumbersOptedOut</code> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListPhoneNumbersOptedOutInput {
    /// <p>A <code>NextToken</code> string is used when you call the <code>ListPhoneNumbersOptedOut</code> action to retrieve additional records that are available after the first page of results.</p>
    pub next_token: Option<String>,
}

/// Serialize `ListPhoneNumbersOptedOutInput` contents to a `SignedRequest`.
struct ListPhoneNumbersOptedOutInputSerializer;
impl ListPhoneNumbersOptedOutInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ListPhoneNumbersOptedOutInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.next_token {
            params.put(
                &format!("{}{}", prefix, "nextToken"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

/// <p>The response from the <code>ListPhoneNumbersOptedOut</code> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListPhoneNumbersOptedOutResponse {
    /// <p>A <code>NextToken</code> string is returned when you call the <code>ListPhoneNumbersOptedOut</code> action if additional records are available after the first page of results.</p>
    pub next_token: Option<String>,
    /// <p>A list of phone numbers that are opted out of receiving SMS messages. The list is paginated, and each page can contain up to 100 phone numbers.</p>
    pub phone_numbers: Option<Vec<String>>,
}

struct ListPhoneNumbersOptedOutResponseDeserializer;
impl ListPhoneNumbersOptedOutResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListPhoneNumbersOptedOutResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ListPhoneNumbersOptedOutResponse::default();

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
                    "nextToken" => {
                        obj.next_token =
                            Some(try!(StringDeserializer::deserialize("nextToken", stack)));
                    }
                    "phoneNumbers" => {
                        obj.phone_numbers = Some(try!(PhoneNumberListDeserializer::deserialize(
                            "phoneNumbers",
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
/// <p>Input for ListPlatformApplications action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListPlatformApplicationsInput {
    /// <p>NextToken string is used when calling ListPlatformApplications action to retrieve additional records that are available after the first page results.</p>
    pub next_token: Option<String>,
}

/// Serialize `ListPlatformApplicationsInput` contents to a `SignedRequest`.
struct ListPlatformApplicationsInputSerializer;
impl ListPlatformApplicationsInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ListPlatformApplicationsInput) {
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

/// <p>Response for ListPlatformApplications action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListPlatformApplicationsResponse {
    /// <p>NextToken string is returned when calling ListPlatformApplications action if additional records are available after the first page results.</p>
    pub next_token: Option<String>,
    /// <p>Platform applications returned when calling ListPlatformApplications action.</p>
    pub platform_applications: Option<Vec<PlatformApplication>>,
}

struct ListPlatformApplicationsResponseDeserializer;
impl ListPlatformApplicationsResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListPlatformApplicationsResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ListPlatformApplicationsResponse::default();

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
                            Some(try!(StringDeserializer::deserialize("NextToken", stack)));
                    }
                    "PlatformApplications" => {
                        obj.platform_applications =
                            Some(try!(ListOfPlatformApplicationsDeserializer::deserialize(
                                "PlatformApplications",
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

/// Serialize `ListString` contents to a `SignedRequest`.
struct ListStringSerializer;
impl ListStringSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

/// <p>Input for ListSubscriptionsByTopic action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListSubscriptionsByTopicInput {
    /// <p>Token returned by the previous <code>ListSubscriptionsByTopic</code> request.</p>
    pub next_token: Option<String>,
    /// <p>The ARN of the topic for which you wish to find subscriptions.</p>
    pub topic_arn: String,
}

/// Serialize `ListSubscriptionsByTopicInput` contents to a `SignedRequest`.
struct ListSubscriptionsByTopicInputSerializer;
impl ListSubscriptionsByTopicInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ListSubscriptionsByTopicInput) {
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
        params.put(
            &format!("{}{}", prefix, "TopicArn"),
            &obj.topic_arn.replace("+", "%2B"),
        );
    }
}

/// <p>Response for ListSubscriptionsByTopic action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListSubscriptionsByTopicResponse {
    /// <p>Token to pass along to the next <code>ListSubscriptionsByTopic</code> request. This element is returned if there are more subscriptions to retrieve.</p>
    pub next_token: Option<String>,
    /// <p>A list of subscriptions.</p>
    pub subscriptions: Option<Vec<Subscription>>,
}

struct ListSubscriptionsByTopicResponseDeserializer;
impl ListSubscriptionsByTopicResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListSubscriptionsByTopicResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ListSubscriptionsByTopicResponse::default();

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
                    "Subscriptions" => {
                        obj.subscriptions = Some(try!(SubscriptionsListDeserializer::deserialize(
                            "Subscriptions",
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
/// <p>Input for ListSubscriptions action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListSubscriptionsInput {
    /// <p>Token returned by the previous <code>ListSubscriptions</code> request.</p>
    pub next_token: Option<String>,
}

/// Serialize `ListSubscriptionsInput` contents to a `SignedRequest`.
struct ListSubscriptionsInputSerializer;
impl ListSubscriptionsInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ListSubscriptionsInput) {
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

/// <p>Response for ListSubscriptions action</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListSubscriptionsResponse {
    /// <p>Token to pass along to the next <code>ListSubscriptions</code> request. This element is returned if there are more subscriptions to retrieve.</p>
    pub next_token: Option<String>,
    /// <p>A list of subscriptions.</p>
    pub subscriptions: Option<Vec<Subscription>>,
}

struct ListSubscriptionsResponseDeserializer;
impl ListSubscriptionsResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListSubscriptionsResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ListSubscriptionsResponse::default();

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
                    "Subscriptions" => {
                        obj.subscriptions = Some(try!(SubscriptionsListDeserializer::deserialize(
                            "Subscriptions",
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
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListTopicsInput {
    /// <p>Token returned by the previous <code>ListTopics</code> request.</p>
    pub next_token: Option<String>,
}

/// Serialize `ListTopicsInput` contents to a `SignedRequest`.
struct ListTopicsInputSerializer;
impl ListTopicsInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ListTopicsInput) {
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

/// <p>Response for ListTopics action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListTopicsResponse {
    /// <p>Token to pass along to the next <code>ListTopics</code> request. This element is returned if there are additional topics to retrieve.</p>
    pub next_token: Option<String>,
    /// <p>A list of topic ARNs.</p>
    pub topics: Option<Vec<Topic>>,
}

struct ListTopicsResponseDeserializer;
impl ListTopicsResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListTopicsResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ListTopicsResponse::default();

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
                    "Topics" => {
                        obj.topics =
                            Some(try!(TopicsListDeserializer::deserialize("Topics", stack)));
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
struct MapStringToStringDeserializer;
impl MapStringToStringDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<::std::collections::HashMap<String, String>, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ::std::collections::HashMap::new();

        while try!(peek_at_name(stack)) == "entry" {
            try!(start_element("entry", stack));
            let key = try!(StringDeserializer::deserialize("key", stack));
            let value = try!(StringDeserializer::deserialize("value", stack));
            obj.insert(key, value);
            try!(end_element("entry", stack));
        }

        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Serialize `MapStringToString` contents to a `SignedRequest`.
struct MapStringToStringSerializer;
impl MapStringToStringSerializer {
    fn serialize(
        params: &mut Params,
        name: &str,
        obj: &::std::collections::HashMap<String, String>,
    ) {
        for (index, (key, value)) in obj.iter().enumerate() {
            let prefix = format!("{}.{}", name, index + 1);
            params.put(&format!("{}.{}", prefix, "key"), &key);
            params.put(&key, &value);
        }
    }
}

/// Serialize `MessageAttributeMap` contents to a `SignedRequest`.
struct MessageAttributeMapSerializer;
impl MessageAttributeMapSerializer {
    fn serialize(
        params: &mut Params,
        name: &str,
        obj: &::std::collections::HashMap<String, MessageAttributeValue>,
    ) {
        for (index, (key, value)) in obj.iter().enumerate() {
            let prefix = format!("{}.{}", name, index + 1);
            params.put(&format!("{}.{}", prefix, "Name"), &key);
            MessageAttributeValueSerializer::serialize(
                params,
                &format!("{}.{}", prefix, "Value"),
                value,
            );
        }
    }
}

/// <p>The user-specified message attribute value. For string data types, the value attribute has the same restrictions on the content as the message body. For more information, see <a href="http://docs.aws.amazon.com/sns/latest/api/API_Publish.html">Publish</a>.</p> <p>Name, type, and value must not be empty or null. In addition, the message body should not be empty or null. All parts of the message attribute, including name, type, and value, are included in the message size restriction, which is currently 256 KB (262,144 bytes). For more information, see <a href="http://docs.aws.amazon.com/sns/latest/dg/SNSMessageAttributes.html">Using Amazon SNS Message Attributes</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct MessageAttributeValue {
    /// <p>Binary type attributes can store any binary data, for example, compressed data, encrypted data, or images.</p>
    pub binary_value: Option<Vec<u8>>,
    /// <p>Amazon SNS supports the following logical data types: String, Number, and Binary. For more information, see <a href="http://docs.aws.amazon.com/sns/latest/dg/SNSMessageAttributes.html#SNSMessageAttributes.DataTypes">Message Attribute Data Types</a>.</p>
    pub data_type: String,
    /// <p>Strings are Unicode with UTF8 binary encoding. For a list of code values, see <a href="http://en.wikipedia.org/wiki/ASCII#ASCII_printable_characters">http://en.wikipedia.org/wiki/ASCII#ASCII_printable_characters</a>.</p>
    pub string_value: Option<String>,
}

/// Serialize `MessageAttributeValue` contents to a `SignedRequest`.
struct MessageAttributeValueSerializer;
impl MessageAttributeValueSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &MessageAttributeValue) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.binary_value {
            params.put(
                &format!("{}{}", prefix, "BinaryValue"),
                ::std::str::from_utf8(&field_value)
                    .unwrap()
                    .replace("+", "%2B"),
            );
        }
        params.put(
            &format!("{}{}", prefix, "DataType"),
            &obj.data_type.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.string_value {
            params.put(
                &format!("{}{}", prefix, "StringValue"),
                &field_value.replace("+", "%2B"),
            );
        }
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
/// <p>Input for the OptInPhoneNumber action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct OptInPhoneNumberInput {
    /// <p>The phone number to opt in.</p>
    pub phone_number: String,
}

/// Serialize `OptInPhoneNumberInput` contents to a `SignedRequest`.
struct OptInPhoneNumberInputSerializer;
impl OptInPhoneNumberInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &OptInPhoneNumberInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "phoneNumber"),
            &obj.phone_number.replace("+", "%2B"),
        );
    }
}

/// <p>The response for the OptInPhoneNumber action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct OptInPhoneNumberResponse {}

struct OptInPhoneNumberResponseDeserializer;
impl OptInPhoneNumberResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<OptInPhoneNumberResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let obj = OptInPhoneNumberResponse::default();

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct PhoneNumberDeserializer;
impl PhoneNumberDeserializer {
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
struct PhoneNumberListDeserializer;
impl PhoneNumberListDeserializer {
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
                        obj.push(try!(PhoneNumberDeserializer::deserialize("member", stack)));
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
/// <p>Platform application object.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct PlatformApplication {
    /// <p>Attributes for platform application object.</p>
    pub attributes: Option<::std::collections::HashMap<String, String>>,
    /// <p>PlatformApplicationArn for platform application object.</p>
    pub platform_application_arn: Option<String>,
}

struct PlatformApplicationDeserializer;
impl PlatformApplicationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PlatformApplication, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = PlatformApplication::default();

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
                        "Attributes" => {
                            obj.attributes = Some(try!(
                                MapStringToStringDeserializer::deserialize("Attributes", stack)
                            ));
                        }
                        "PlatformApplicationArn" => {
                            obj.platform_application_arn = Some(try!(
                                StringDeserializer::deserialize("PlatformApplicationArn", stack)
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
struct ProtocolDeserializer;
impl ProtocolDeserializer {
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
/// <p>Input for Publish action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct PublishInput {
    /// <p><p>The message you want to send to the topic.</p> <p>If you want to send the same message to all transport protocols, include the text of the message as a String value.</p> <p>If you want to send different messages for each transport protocol, set the value of the <code>MessageStructure</code> parameter to <code>json</code> and use a JSON object for the <code>Message</code> parameter. </p> <p>Constraints: Messages must be UTF-8 encoded strings at most 256 KB in size (262144 bytes, not 262144 characters).</p> <p>JSON-specific constraints:</p> <ul> <li> <p>Keys in the JSON object that correspond to supported transport protocols must have simple JSON string values.</p> </li> <li> <p>The values will be parsed (unescaped) before they are used in outgoing messages.</p> </li> <li> <p>Outbound notifications are JSON encoded (meaning that the characters will be reescaped for sending).</p> </li> <li> <p>Values have a minimum length of 0 (the empty string, &quot;&quot;, is allowed).</p> </li> <li> <p>Values have a maximum length bounded by the overall message size (so, including multiple protocols may limit message sizes).</p> </li> <li> <p>Non-string values will cause the key to be ignored.</p> </li> <li> <p>Keys that do not correspond to supported transport protocols are ignored.</p> </li> <li> <p>Duplicate keys are not allowed.</p> </li> <li> <p>Failure to parse or validate any key or value in the message will cause the <code>Publish</code> call to return an error (no partial delivery).</p> </li> </ul></p>
    pub message: String,
    /// <p>Message attributes for Publish action.</p>
    pub message_attributes: Option<::std::collections::HashMap<String, MessageAttributeValue>>,
    /// <p>Set <code>MessageStructure</code> to <code>json</code> if you want to send a different message for each protocol. For example, using one publish action, you can send a short message to your SMS subscribers and a longer message to your email subscribers. If you set <code>MessageStructure</code> to <code>json</code>, the value of the <code>Message</code> parameter must: </p> <ul> <li> <p>be a syntactically valid JSON object; and</p> </li> <li> <p>contain at least a top-level JSON key of "default" with a value that is a string.</p> </li> </ul> <p>You can define other top-level keys that define the message you want to send to a specific transport protocol (e.g., "http").</p> <p>For information about sending different messages for each protocol using the AWS Management Console, go to <a href="http://docs.aws.amazon.com/sns/latest/gsg/Publish.html#sns-message-formatting-by-protocol">Create Different Messages for Each Protocol</a> in the <i>Amazon Simple Notification Service Getting Started Guide</i>. </p> <p>Valid value: <code>json</code> </p>
    pub message_structure: Option<String>,
    /// <p>The phone number to which you want to deliver an SMS message. Use E.164 format.</p> <p>If you don't specify a value for the <code>PhoneNumber</code> parameter, you must specify a value for the <code>TargetArn</code> or <code>TopicArn</code> parameters.</p>
    pub phone_number: Option<String>,
    /// <p>Optional parameter to be used as the "Subject" line when the message is delivered to email endpoints. This field will also be included, if present, in the standard JSON messages delivered to other endpoints.</p> <p>Constraints: Subjects must be ASCII text that begins with a letter, number, or punctuation mark; must not include line breaks or control characters; and must be less than 100 characters long.</p>
    pub subject: Option<String>,
    /// <p>Either TopicArn or EndpointArn, but not both.</p> <p>If you don't specify a value for the <code>TargetArn</code> parameter, you must specify a value for the <code>PhoneNumber</code> or <code>TopicArn</code> parameters.</p>
    pub target_arn: Option<String>,
    /// <p>The topic you want to publish to.</p> <p>If you don't specify a value for the <code>TopicArn</code> parameter, you must specify a value for the <code>PhoneNumber</code> or <code>TargetArn</code> parameters.</p>
    pub topic_arn: Option<String>,
}

/// Serialize `PublishInput` contents to a `SignedRequest`.
struct PublishInputSerializer;
impl PublishInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &PublishInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "Message"),
            &obj.message.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.message_attributes {
            MessageAttributeMapSerializer::serialize(
                params,
                &format!("{}{}", prefix, "MessageAttributes"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.message_structure {
            params.put(
                &format!("{}{}", prefix, "MessageStructure"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.phone_number {
            params.put(
                &format!("{}{}", prefix, "PhoneNumber"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.subject {
            params.put(
                &format!("{}{}", prefix, "Subject"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.target_arn {
            params.put(
                &format!("{}{}", prefix, "TargetArn"),
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

/// <p>Response for Publish action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct PublishResponse {
    /// <p>Unique identifier assigned to the published message.</p> <p>Length Constraint: Maximum 100 characters</p>
    pub message_id: Option<String>,
}

struct PublishResponseDeserializer;
impl PublishResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PublishResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = PublishResponse::default();

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
/// <p>Input for RemovePermission action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct RemovePermissionInput {
    /// <p>The unique label of the statement you want to remove.</p>
    pub label: String,
    /// <p>The ARN of the topic whose access control policy you wish to modify.</p>
    pub topic_arn: String,
}

/// Serialize `RemovePermissionInput` contents to a `SignedRequest`.
struct RemovePermissionInputSerializer;
impl RemovePermissionInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &RemovePermissionInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "Label"),
            &obj.label.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "TopicArn"),
            &obj.topic_arn.replace("+", "%2B"),
        );
    }
}

/// <p>Input for SetEndpointAttributes action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SetEndpointAttributesInput {
    /// <p><p>A map of the endpoint attributes. Attributes in this map include the following:</p> <ul> <li> <p> <code>CustomUserData</code> -- arbitrary user data to associate with the endpoint. Amazon SNS does not use this data. The data must be in UTF-8 format and less than 2KB.</p> </li> <li> <p> <code>Enabled</code> -- flag that enables/disables delivery to the endpoint. Amazon SNS will set this to false when a notification service indicates to Amazon SNS that the endpoint is invalid. Users can set it back to true, typically after updating Token.</p> </li> <li> <p> <code>Token</code> -- device token, also referred to as a registration id, for an app and mobile device. This is returned from the notification service when an app and mobile device are registered with the notification service.</p> </li> </ul></p>
    pub attributes: ::std::collections::HashMap<String, String>,
    /// <p>EndpointArn used for SetEndpointAttributes action.</p>
    pub endpoint_arn: String,
}

/// Serialize `SetEndpointAttributesInput` contents to a `SignedRequest`.
struct SetEndpointAttributesInputSerializer;
impl SetEndpointAttributesInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &SetEndpointAttributesInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        MapStringToStringSerializer::serialize(
            params,
            &format!("{}{}", prefix, "Attributes"),
            &obj.attributes,
        );
        params.put(
            &format!("{}{}", prefix, "EndpointArn"),
            &obj.endpoint_arn.replace("+", "%2B"),
        );
    }
}

/// <p>Input for SetPlatformApplicationAttributes action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SetPlatformApplicationAttributesInput {
    /// <p><p>A map of the platform application attributes. Attributes in this map include the following:</p> <ul> <li> <p> <code>PlatformCredential</code> -- The credential received from the notification service. For APNS/APNS<em>SANDBOX, PlatformCredential is private key. For GCM, PlatformCredential is &quot;API key&quot;. For ADM, PlatformCredential is &quot;client secret&quot;.</p> </li> <li> <p> <code>PlatformPrincipal</code> -- The principal received from the notification service. For APNS/APNS</em>SANDBOX, PlatformPrincipal is SSL certificate. For GCM, PlatformPrincipal is not applicable. For ADM, PlatformPrincipal is &quot;client id&quot;.</p> </li> <li> <p> <code>EventEndpointCreated</code> -- Topic ARN to which EndpointCreated event notifications should be sent.</p> </li> <li> <p> <code>EventEndpointDeleted</code> -- Topic ARN to which EndpointDeleted event notifications should be sent.</p> </li> <li> <p> <code>EventEndpointUpdated</code> -- Topic ARN to which EndpointUpdate event notifications should be sent.</p> </li> <li> <p> <code>EventDeliveryFailure</code> -- Topic ARN to which DeliveryFailure event notifications should be sent upon Direct Publish delivery failure (permanent) to one of the application&#39;s endpoints.</p> </li> <li> <p> <code>SuccessFeedbackRoleArn</code> -- IAM role ARN used to give Amazon SNS write access to use CloudWatch Logs on your behalf.</p> </li> <li> <p> <code>FailureFeedbackRoleArn</code> -- IAM role ARN used to give Amazon SNS write access to use CloudWatch Logs on your behalf.</p> </li> <li> <p> <code>SuccessFeedbackSampleRate</code> -- Sample rate percentage (0-100) of successfully delivered messages.</p> </li> </ul></p>
    pub attributes: ::std::collections::HashMap<String, String>,
    /// <p>PlatformApplicationArn for SetPlatformApplicationAttributes action.</p>
    pub platform_application_arn: String,
}

/// Serialize `SetPlatformApplicationAttributesInput` contents to a `SignedRequest`.
struct SetPlatformApplicationAttributesInputSerializer;
impl SetPlatformApplicationAttributesInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &SetPlatformApplicationAttributesInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        MapStringToStringSerializer::serialize(
            params,
            &format!("{}{}", prefix, "Attributes"),
            &obj.attributes,
        );
        params.put(
            &format!("{}{}", prefix, "PlatformApplicationArn"),
            &obj.platform_application_arn.replace("+", "%2B"),
        );
    }
}

/// <p>The input for the SetSMSAttributes action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SetSMSAttributesInput {
    /// <p>The default settings for sending SMS messages from your account. You can set values for the following attribute names:</p> <p> <code>MonthlySpendLimit</code> – The maximum amount in USD that you are willing to spend each month to send SMS messages. When Amazon SNS determines that sending an SMS message would incur a cost that exceeds this limit, it stops sending SMS messages within minutes.</p> <important> <p>Amazon SNS stops sending SMS messages within minutes of the limit being crossed. During that interval, if you continue to send SMS messages, you will incur costs that exceed your limit.</p> </important> <p>By default, the spend limit is set to the maximum allowed by Amazon SNS. If you want to exceed the maximum, contact <a href="https://aws.amazon.com/premiumsupport/">AWS Support</a> or your AWS sales representative for a service limit increase.</p> <p> <code>DeliveryStatusIAMRole</code> – The ARN of the IAM role that allows Amazon SNS to write logs about SMS deliveries in CloudWatch Logs. For each SMS message that you send, Amazon SNS writes a log that includes the message price, the success or failure status, the reason for failure (if the message failed), the message dwell time, and other information.</p> <p> <code>DeliveryStatusSuccessSamplingRate</code> – The percentage of successful SMS deliveries for which Amazon SNS will write logs in CloudWatch Logs. The value can be an integer from 0 - 100. For example, to write logs only for failed deliveries, set this value to <code>0</code>. To write logs for 10% of your successful deliveries, set it to <code>10</code>.</p> <p> <code>DefaultSenderID</code> – A string, such as your business brand, that is displayed as the sender on the receiving device. Support for sender IDs varies by country. The sender ID can be 1 - 11 alphanumeric characters, and it must contain at least one letter.</p> <p> <code>DefaultSMSType</code> – The type of SMS message that you will send by default. You can assign the following values:</p> <ul> <li> <p> <code>Promotional</code> – (Default) Noncritical messages, such as marketing messages. Amazon SNS optimizes the message delivery to incur the lowest cost.</p> </li> <li> <p> <code>Transactional</code> – Critical messages that support customer transactions, such as one-time passcodes for multi-factor authentication. Amazon SNS optimizes the message delivery to achieve the highest reliability.</p> </li> </ul> <p> <code>UsageReportS3Bucket</code> – The name of the Amazon S3 bucket to receive daily SMS usage reports from Amazon SNS. Each day, Amazon SNS will deliver a usage report as a CSV file to the bucket. The report includes the following information for each SMS message that was successfully delivered by your account:</p> <ul> <li> <p>Time that the message was published (in UTC)</p> </li> <li> <p>Message ID</p> </li> <li> <p>Destination phone number</p> </li> <li> <p>Message type</p> </li> <li> <p>Delivery status</p> </li> <li> <p>Message price (in USD)</p> </li> <li> <p>Part number (a message is split into multiple parts if it is too long for a single message)</p> </li> <li> <p>Total number of parts</p> </li> </ul> <p>To receive the report, the bucket must have a policy that allows the Amazon SNS service principle to perform the <code>s3:PutObject</code> and <code>s3:GetBucketLocation</code> actions.</p> <p>For an example bucket policy and usage report, see <a href="http://docs.aws.amazon.com/sns/latest/dg/sms_stats.html">Monitoring SMS Activity</a> in the <i>Amazon SNS Developer Guide</i>.</p>
    pub attributes: ::std::collections::HashMap<String, String>,
}

/// Serialize `SetSMSAttributesInput` contents to a `SignedRequest`.
struct SetSMSAttributesInputSerializer;
impl SetSMSAttributesInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &SetSMSAttributesInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        MapStringToStringSerializer::serialize(
            params,
            &format!("{}{}", prefix, "attributes"),
            &obj.attributes,
        );
    }
}

/// <p>The response for the SetSMSAttributes action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SetSMSAttributesResponse {}

struct SetSMSAttributesResponseDeserializer;
impl SetSMSAttributesResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<SetSMSAttributesResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let obj = SetSMSAttributesResponse::default();

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Input for SetSubscriptionAttributes action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SetSubscriptionAttributesInput {
    /// <p>The name of the attribute you want to set. Only a subset of the subscriptions attributes are mutable.</p> <p>Valid values: <code>DeliveryPolicy</code> | <code>RawMessageDelivery</code> </p>
    pub attribute_name: String,
    /// <p>The new value for the attribute in JSON format.</p>
    pub attribute_value: Option<String>,
    /// <p>The ARN of the subscription to modify.</p>
    pub subscription_arn: String,
}

/// Serialize `SetSubscriptionAttributesInput` contents to a `SignedRequest`.
struct SetSubscriptionAttributesInputSerializer;
impl SetSubscriptionAttributesInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &SetSubscriptionAttributesInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "AttributeName"),
            &obj.attribute_name.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.attribute_value {
            params.put(
                &format!("{}{}", prefix, "AttributeValue"),
                &field_value.replace("+", "%2B"),
            );
        }
        params.put(
            &format!("{}{}", prefix, "SubscriptionArn"),
            &obj.subscription_arn.replace("+", "%2B"),
        );
    }
}

/// <p>Input for SetTopicAttributes action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SetTopicAttributesInput {
    /// <p>The name of the attribute you want to set. Only a subset of the topic's attributes are mutable.</p> <p>Valid values: <code>Policy</code> | <code>DisplayName</code> | <code>DeliveryPolicy</code> </p>
    pub attribute_name: String,
    /// <p>The new value for the attribute.</p>
    pub attribute_value: Option<String>,
    /// <p>The ARN of the topic to modify.</p>
    pub topic_arn: String,
}

/// Serialize `SetTopicAttributesInput` contents to a `SignedRequest`.
struct SetTopicAttributesInputSerializer;
impl SetTopicAttributesInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &SetTopicAttributesInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "AttributeName"),
            &obj.attribute_name.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.attribute_value {
            params.put(
                &format!("{}{}", prefix, "AttributeValue"),
                &field_value.replace("+", "%2B"),
            );
        }
        params.put(
            &format!("{}{}", prefix, "TopicArn"),
            &obj.topic_arn.replace("+", "%2B"),
        );
    }
}

struct StringDeserializer;
impl StringDeserializer {
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
/// <p>Input for Subscribe action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SubscribeInput {
    /// <p><p>The endpoint that you want to receive notifications. Endpoints vary by protocol:</p> <ul> <li> <p>For the <code>http</code> protocol, the endpoint is an URL beginning with &quot;http://&quot;</p> </li> <li> <p>For the <code>https</code> protocol, the endpoint is a URL beginning with &quot;https://&quot;</p> </li> <li> <p>For the <code>email</code> protocol, the endpoint is an email address</p> </li> <li> <p>For the <code>email-json</code> protocol, the endpoint is an email address</p> </li> <li> <p>For the <code>sms</code> protocol, the endpoint is a phone number of an SMS-enabled device</p> </li> <li> <p>For the <code>sqs</code> protocol, the endpoint is the ARN of an Amazon SQS queue</p> </li> <li> <p>For the <code>application</code> protocol, the endpoint is the EndpointArn of a mobile app and device.</p> </li> <li> <p>For the <code>lambda</code> protocol, the endpoint is the ARN of an AWS Lambda function.</p> </li> </ul></p>
    pub endpoint: Option<String>,
    /// <p><p>The protocol you want to use. Supported protocols include:</p> <ul> <li> <p> <code>http</code> -- delivery of JSON-encoded message via HTTP POST</p> </li> <li> <p> <code>https</code> -- delivery of JSON-encoded message via HTTPS POST</p> </li> <li> <p> <code>email</code> -- delivery of message via SMTP</p> </li> <li> <p> <code>email-json</code> -- delivery of JSON-encoded message via SMTP</p> </li> <li> <p> <code>sms</code> -- delivery of message via SMS</p> </li> <li> <p> <code>sqs</code> -- delivery of JSON-encoded message to an Amazon SQS queue</p> </li> <li> <p> <code>application</code> -- delivery of JSON-encoded message to an EndpointArn for a mobile app and device.</p> </li> <li> <p> <code>lambda</code> -- delivery of JSON-encoded message to an AWS Lambda function.</p> </li> </ul></p>
    pub protocol: String,
    /// <p>The ARN of the topic you want to subscribe to.</p>
    pub topic_arn: String,
}

/// Serialize `SubscribeInput` contents to a `SignedRequest`.
struct SubscribeInputSerializer;
impl SubscribeInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &SubscribeInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.endpoint {
            params.put(
                &format!("{}{}", prefix, "Endpoint"),
                &field_value.replace("+", "%2B"),
            );
        }
        params.put(
            &format!("{}{}", prefix, "Protocol"),
            &obj.protocol.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "TopicArn"),
            &obj.topic_arn.replace("+", "%2B"),
        );
    }
}

/// <p>Response for Subscribe action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SubscribeResponse {
    /// <p>The ARN of the subscription, if the service was able to create a subscription immediately (without requiring endpoint owner confirmation).</p>
    pub subscription_arn: Option<String>,
}

struct SubscribeResponseDeserializer;
impl SubscribeResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<SubscribeResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = SubscribeResponse::default();

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
                    "SubscriptionArn" => {
                        obj.subscription_arn = Some(try!(
                            SubscriptionARNDeserializer::deserialize("SubscriptionArn", stack)
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
/// <p>A wrapper type for the attributes of an Amazon SNS subscription.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Subscription {
    /// <p>The subscription's endpoint (format depends on the protocol).</p>
    pub endpoint: Option<String>,
    /// <p>The subscription's owner.</p>
    pub owner: Option<String>,
    /// <p>The subscription's protocol.</p>
    pub protocol: Option<String>,
    /// <p>The subscription's ARN.</p>
    pub subscription_arn: Option<String>,
    /// <p>The ARN of the subscription's topic.</p>
    pub topic_arn: Option<String>,
}

struct SubscriptionDeserializer;
impl SubscriptionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Subscription, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = Subscription::default();

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
                    "Endpoint" => {
                        obj.endpoint =
                            Some(try!(EndpointDeserializer::deserialize("Endpoint", stack)));
                    }
                    "Owner" => {
                        obj.owner = Some(try!(AccountDeserializer::deserialize("Owner", stack)));
                    }
                    "Protocol" => {
                        obj.protocol =
                            Some(try!(ProtocolDeserializer::deserialize("Protocol", stack)));
                    }
                    "SubscriptionArn" => {
                        obj.subscription_arn = Some(try!(
                            SubscriptionARNDeserializer::deserialize("SubscriptionArn", stack)
                        ));
                    }
                    "TopicArn" => {
                        obj.topic_arn =
                            Some(try!(TopicARNDeserializer::deserialize("TopicArn", stack)));
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
struct SubscriptionARNDeserializer;
impl SubscriptionARNDeserializer {
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
struct SubscriptionAttributesMapDeserializer;
impl SubscriptionAttributesMapDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<::std::collections::HashMap<String, String>, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ::std::collections::HashMap::new();

        while try!(peek_at_name(stack)) == "entry" {
            try!(start_element("entry", stack));
            let key = try!(AttributeNameDeserializer::deserialize("key", stack));
            let value = try!(AttributeValueDeserializer::deserialize("value", stack));
            obj.insert(key, value);
            try!(end_element("entry", stack));
        }

        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
struct SubscriptionsListDeserializer;
impl SubscriptionsListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Subscription>, XmlParseError> {
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
                        obj.push(try!(SubscriptionDeserializer::deserialize("member", stack)));
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
/// <p>A wrapper type for the topic's Amazon Resource Name (ARN). To retrieve a topic's attributes, use <code>GetTopicAttributes</code>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Topic {
    /// <p>The topic's ARN.</p>
    pub topic_arn: Option<String>,
}

struct TopicDeserializer;
impl TopicDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Topic, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = Topic::default();

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
                    "TopicArn" => {
                        obj.topic_arn =
                            Some(try!(TopicARNDeserializer::deserialize("TopicArn", stack)));
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
struct TopicARNDeserializer;
impl TopicARNDeserializer {
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
struct TopicAttributesMapDeserializer;
impl TopicAttributesMapDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<::std::collections::HashMap<String, String>, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ::std::collections::HashMap::new();

        while try!(peek_at_name(stack)) == "entry" {
            try!(start_element("entry", stack));
            let key = try!(AttributeNameDeserializer::deserialize("key", stack));
            let value = try!(AttributeValueDeserializer::deserialize("value", stack));
            obj.insert(key, value);
            try!(end_element("entry", stack));
        }

        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
struct TopicsListDeserializer;
impl TopicsListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Topic>, XmlParseError> {
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
                        obj.push(try!(TopicDeserializer::deserialize("member", stack)));
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
/// <p>Input for Unsubscribe action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct UnsubscribeInput {
    /// <p>The ARN of the subscription to be deleted.</p>
    pub subscription_arn: String,
}

/// Serialize `UnsubscribeInput` contents to a `SignedRequest`.
struct UnsubscribeInputSerializer;
impl UnsubscribeInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &UnsubscribeInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "SubscriptionArn"),
            &obj.subscription_arn.replace("+", "%2B"),
        );
    }
}

/// Errors returned by AddPermission
#[derive(Debug, PartialEq)]
pub enum AddPermissionError {
    /// <p>Indicates that the user has been denied access to the requested resource.</p>
    AuthorizationError(String),
    /// <p>Indicates an internal service error.</p>
    InternalError(String),
    /// <p>Indicates that a request parameter does not comply with the associated constraints.</p>
    InvalidParameter(String),
    /// <p>Indicates that the requested resource does not exist.</p>
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

impl AddPermissionError {
    pub fn from_body(body: &str) -> AddPermissionError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "AuthorizationError" => {
                    AddPermissionError::AuthorizationError(String::from(parsed_error.message))
                }
                "InternalError" => {
                    AddPermissionError::InternalError(String::from(parsed_error.message))
                }
                "InvalidParameter" => {
                    AddPermissionError::InvalidParameter(String::from(parsed_error.message))
                }
                "NotFound" => AddPermissionError::NotFound(String::from(parsed_error.message)),
                _ => AddPermissionError::Unknown(String::from(body)),
            },
            Err(_) => AddPermissionError::Unknown(body.to_string()),
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

impl From<XmlParseError> for AddPermissionError {
    fn from(err: XmlParseError) -> AddPermissionError {
        let XmlParseError(message) = err;
        AddPermissionError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for AddPermissionError {
    fn from(err: CredentialsError) -> AddPermissionError {
        AddPermissionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AddPermissionError {
    fn from(err: HttpDispatchError) -> AddPermissionError {
        AddPermissionError::HttpDispatch(err)
    }
}
impl From<io::Error> for AddPermissionError {
    fn from(err: io::Error) -> AddPermissionError {
        AddPermissionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AddPermissionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AddPermissionError {
    fn description(&self) -> &str {
        match *self {
            AddPermissionError::AuthorizationError(ref cause) => cause,
            AddPermissionError::InternalError(ref cause) => cause,
            AddPermissionError::InvalidParameter(ref cause) => cause,
            AddPermissionError::NotFound(ref cause) => cause,
            AddPermissionError::Validation(ref cause) => cause,
            AddPermissionError::Credentials(ref err) => err.description(),
            AddPermissionError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            AddPermissionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CheckIfPhoneNumberIsOptedOut
#[derive(Debug, PartialEq)]
pub enum CheckIfPhoneNumberIsOptedOutError {
    /// <p>Indicates that the user has been denied access to the requested resource.</p>
    AuthorizationError(String),
    /// <p>Indicates an internal service error.</p>
    InternalError(String),
    /// <p>Indicates that a request parameter does not comply with the associated constraints.</p>
    InvalidParameter(String),
    /// <p>Indicates that the rate at which requests have been submitted for this action exceeds the limit for your account.</p>
    Throttled(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CheckIfPhoneNumberIsOptedOutError {
    pub fn from_body(body: &str) -> CheckIfPhoneNumberIsOptedOutError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "AuthorizationError" => CheckIfPhoneNumberIsOptedOutError::AuthorizationError(
                    String::from(parsed_error.message),
                ),
                "InternalError" => CheckIfPhoneNumberIsOptedOutError::InternalError(String::from(
                    parsed_error.message,
                )),
                "InvalidParameter" => CheckIfPhoneNumberIsOptedOutError::InvalidParameter(
                    String::from(parsed_error.message),
                ),
                "Throttled" => {
                    CheckIfPhoneNumberIsOptedOutError::Throttled(String::from(parsed_error.message))
                }
                _ => CheckIfPhoneNumberIsOptedOutError::Unknown(String::from(body)),
            },
            Err(_) => CheckIfPhoneNumberIsOptedOutError::Unknown(body.to_string()),
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

impl From<XmlParseError> for CheckIfPhoneNumberIsOptedOutError {
    fn from(err: XmlParseError) -> CheckIfPhoneNumberIsOptedOutError {
        let XmlParseError(message) = err;
        CheckIfPhoneNumberIsOptedOutError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for CheckIfPhoneNumberIsOptedOutError {
    fn from(err: CredentialsError) -> CheckIfPhoneNumberIsOptedOutError {
        CheckIfPhoneNumberIsOptedOutError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CheckIfPhoneNumberIsOptedOutError {
    fn from(err: HttpDispatchError) -> CheckIfPhoneNumberIsOptedOutError {
        CheckIfPhoneNumberIsOptedOutError::HttpDispatch(err)
    }
}
impl From<io::Error> for CheckIfPhoneNumberIsOptedOutError {
    fn from(err: io::Error) -> CheckIfPhoneNumberIsOptedOutError {
        CheckIfPhoneNumberIsOptedOutError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CheckIfPhoneNumberIsOptedOutError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CheckIfPhoneNumberIsOptedOutError {
    fn description(&self) -> &str {
        match *self {
            CheckIfPhoneNumberIsOptedOutError::AuthorizationError(ref cause) => cause,
            CheckIfPhoneNumberIsOptedOutError::InternalError(ref cause) => cause,
            CheckIfPhoneNumberIsOptedOutError::InvalidParameter(ref cause) => cause,
            CheckIfPhoneNumberIsOptedOutError::Throttled(ref cause) => cause,
            CheckIfPhoneNumberIsOptedOutError::Validation(ref cause) => cause,
            CheckIfPhoneNumberIsOptedOutError::Credentials(ref err) => err.description(),
            CheckIfPhoneNumberIsOptedOutError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CheckIfPhoneNumberIsOptedOutError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ConfirmSubscription
#[derive(Debug, PartialEq)]
pub enum ConfirmSubscriptionError {
    /// <p>Indicates that the user has been denied access to the requested resource.</p>
    AuthorizationError(String),
    /// <p>Indicates an internal service error.</p>
    InternalError(String),
    /// <p>Indicates that a request parameter does not comply with the associated constraints.</p>
    InvalidParameter(String),
    /// <p>Indicates that the requested resource does not exist.</p>
    NotFound(String),
    /// <p>Indicates that the customer already owns the maximum allowed number of subscriptions.</p>
    SubscriptionLimitExceeded(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ConfirmSubscriptionError {
    pub fn from_body(body: &str) -> ConfirmSubscriptionError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "AuthorizationError" => {
                    ConfirmSubscriptionError::AuthorizationError(String::from(parsed_error.message))
                }
                "InternalError" => {
                    ConfirmSubscriptionError::InternalError(String::from(parsed_error.message))
                }
                "InvalidParameter" => {
                    ConfirmSubscriptionError::InvalidParameter(String::from(parsed_error.message))
                }
                "NotFound" => {
                    ConfirmSubscriptionError::NotFound(String::from(parsed_error.message))
                }
                "SubscriptionLimitExceeded" => ConfirmSubscriptionError::SubscriptionLimitExceeded(
                    String::from(parsed_error.message),
                ),
                _ => ConfirmSubscriptionError::Unknown(String::from(body)),
            },
            Err(_) => ConfirmSubscriptionError::Unknown(body.to_string()),
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

impl From<XmlParseError> for ConfirmSubscriptionError {
    fn from(err: XmlParseError) -> ConfirmSubscriptionError {
        let XmlParseError(message) = err;
        ConfirmSubscriptionError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ConfirmSubscriptionError {
    fn from(err: CredentialsError) -> ConfirmSubscriptionError {
        ConfirmSubscriptionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ConfirmSubscriptionError {
    fn from(err: HttpDispatchError) -> ConfirmSubscriptionError {
        ConfirmSubscriptionError::HttpDispatch(err)
    }
}
impl From<io::Error> for ConfirmSubscriptionError {
    fn from(err: io::Error) -> ConfirmSubscriptionError {
        ConfirmSubscriptionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ConfirmSubscriptionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ConfirmSubscriptionError {
    fn description(&self) -> &str {
        match *self {
            ConfirmSubscriptionError::AuthorizationError(ref cause) => cause,
            ConfirmSubscriptionError::InternalError(ref cause) => cause,
            ConfirmSubscriptionError::InvalidParameter(ref cause) => cause,
            ConfirmSubscriptionError::NotFound(ref cause) => cause,
            ConfirmSubscriptionError::SubscriptionLimitExceeded(ref cause) => cause,
            ConfirmSubscriptionError::Validation(ref cause) => cause,
            ConfirmSubscriptionError::Credentials(ref err) => err.description(),
            ConfirmSubscriptionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ConfirmSubscriptionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreatePlatformApplication
#[derive(Debug, PartialEq)]
pub enum CreatePlatformApplicationError {
    /// <p>Indicates that the user has been denied access to the requested resource.</p>
    AuthorizationError(String),
    /// <p>Indicates an internal service error.</p>
    InternalError(String),
    /// <p>Indicates that a request parameter does not comply with the associated constraints.</p>
    InvalidParameter(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreatePlatformApplicationError {
    pub fn from_body(body: &str) -> CreatePlatformApplicationError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "AuthorizationError" => CreatePlatformApplicationError::AuthorizationError(
                    String::from(parsed_error.message),
                ),
                "InternalError" => CreatePlatformApplicationError::InternalError(String::from(
                    parsed_error.message,
                )),
                "InvalidParameter" => CreatePlatformApplicationError::InvalidParameter(
                    String::from(parsed_error.message),
                ),
                _ => CreatePlatformApplicationError::Unknown(String::from(body)),
            },
            Err(_) => CreatePlatformApplicationError::Unknown(body.to_string()),
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

impl From<XmlParseError> for CreatePlatformApplicationError {
    fn from(err: XmlParseError) -> CreatePlatformApplicationError {
        let XmlParseError(message) = err;
        CreatePlatformApplicationError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for CreatePlatformApplicationError {
    fn from(err: CredentialsError) -> CreatePlatformApplicationError {
        CreatePlatformApplicationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreatePlatformApplicationError {
    fn from(err: HttpDispatchError) -> CreatePlatformApplicationError {
        CreatePlatformApplicationError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreatePlatformApplicationError {
    fn from(err: io::Error) -> CreatePlatformApplicationError {
        CreatePlatformApplicationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreatePlatformApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreatePlatformApplicationError {
    fn description(&self) -> &str {
        match *self {
            CreatePlatformApplicationError::AuthorizationError(ref cause) => cause,
            CreatePlatformApplicationError::InternalError(ref cause) => cause,
            CreatePlatformApplicationError::InvalidParameter(ref cause) => cause,
            CreatePlatformApplicationError::Validation(ref cause) => cause,
            CreatePlatformApplicationError::Credentials(ref err) => err.description(),
            CreatePlatformApplicationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreatePlatformApplicationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreatePlatformEndpoint
#[derive(Debug, PartialEq)]
pub enum CreatePlatformEndpointError {
    /// <p>Indicates that the user has been denied access to the requested resource.</p>
    AuthorizationError(String),
    /// <p>Indicates an internal service error.</p>
    InternalError(String),
    /// <p>Indicates that a request parameter does not comply with the associated constraints.</p>
    InvalidParameter(String),
    /// <p>Indicates that the requested resource does not exist.</p>
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

impl CreatePlatformEndpointError {
    pub fn from_body(body: &str) -> CreatePlatformEndpointError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "AuthorizationError" => CreatePlatformEndpointError::AuthorizationError(
                    String::from(parsed_error.message),
                ),
                "InternalError" => {
                    CreatePlatformEndpointError::InternalError(String::from(parsed_error.message))
                }
                "InvalidParameter" => CreatePlatformEndpointError::InvalidParameter(String::from(
                    parsed_error.message,
                )),
                "NotFound" => {
                    CreatePlatformEndpointError::NotFound(String::from(parsed_error.message))
                }
                _ => CreatePlatformEndpointError::Unknown(String::from(body)),
            },
            Err(_) => CreatePlatformEndpointError::Unknown(body.to_string()),
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

impl From<XmlParseError> for CreatePlatformEndpointError {
    fn from(err: XmlParseError) -> CreatePlatformEndpointError {
        let XmlParseError(message) = err;
        CreatePlatformEndpointError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for CreatePlatformEndpointError {
    fn from(err: CredentialsError) -> CreatePlatformEndpointError {
        CreatePlatformEndpointError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreatePlatformEndpointError {
    fn from(err: HttpDispatchError) -> CreatePlatformEndpointError {
        CreatePlatformEndpointError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreatePlatformEndpointError {
    fn from(err: io::Error) -> CreatePlatformEndpointError {
        CreatePlatformEndpointError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreatePlatformEndpointError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreatePlatformEndpointError {
    fn description(&self) -> &str {
        match *self {
            CreatePlatformEndpointError::AuthorizationError(ref cause) => cause,
            CreatePlatformEndpointError::InternalError(ref cause) => cause,
            CreatePlatformEndpointError::InvalidParameter(ref cause) => cause,
            CreatePlatformEndpointError::NotFound(ref cause) => cause,
            CreatePlatformEndpointError::Validation(ref cause) => cause,
            CreatePlatformEndpointError::Credentials(ref err) => err.description(),
            CreatePlatformEndpointError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreatePlatformEndpointError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateTopic
#[derive(Debug, PartialEq)]
pub enum CreateTopicError {
    /// <p>Indicates that the user has been denied access to the requested resource.</p>
    AuthorizationError(String),
    /// <p>Indicates an internal service error.</p>
    InternalError(String),
    /// <p>Indicates that a request parameter does not comply with the associated constraints.</p>
    InvalidParameter(String),
    /// <p>Indicates that the customer already owns the maximum allowed number of topics.</p>
    TopicLimitExceeded(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateTopicError {
    pub fn from_body(body: &str) -> CreateTopicError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "AuthorizationError" => {
                    CreateTopicError::AuthorizationError(String::from(parsed_error.message))
                }
                "InternalError" => {
                    CreateTopicError::InternalError(String::from(parsed_error.message))
                }
                "InvalidParameter" => {
                    CreateTopicError::InvalidParameter(String::from(parsed_error.message))
                }
                "TopicLimitExceeded" => {
                    CreateTopicError::TopicLimitExceeded(String::from(parsed_error.message))
                }
                _ => CreateTopicError::Unknown(String::from(body)),
            },
            Err(_) => CreateTopicError::Unknown(body.to_string()),
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

impl From<XmlParseError> for CreateTopicError {
    fn from(err: XmlParseError) -> CreateTopicError {
        let XmlParseError(message) = err;
        CreateTopicError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for CreateTopicError {
    fn from(err: CredentialsError) -> CreateTopicError {
        CreateTopicError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateTopicError {
    fn from(err: HttpDispatchError) -> CreateTopicError {
        CreateTopicError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateTopicError {
    fn from(err: io::Error) -> CreateTopicError {
        CreateTopicError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateTopicError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateTopicError {
    fn description(&self) -> &str {
        match *self {
            CreateTopicError::AuthorizationError(ref cause) => cause,
            CreateTopicError::InternalError(ref cause) => cause,
            CreateTopicError::InvalidParameter(ref cause) => cause,
            CreateTopicError::TopicLimitExceeded(ref cause) => cause,
            CreateTopicError::Validation(ref cause) => cause,
            CreateTopicError::Credentials(ref err) => err.description(),
            CreateTopicError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateTopicError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteEndpoint
#[derive(Debug, PartialEq)]
pub enum DeleteEndpointError {
    /// <p>Indicates that the user has been denied access to the requested resource.</p>
    AuthorizationError(String),
    /// <p>Indicates an internal service error.</p>
    InternalError(String),
    /// <p>Indicates that a request parameter does not comply with the associated constraints.</p>
    InvalidParameter(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteEndpointError {
    pub fn from_body(body: &str) -> DeleteEndpointError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "AuthorizationError" => {
                    DeleteEndpointError::AuthorizationError(String::from(parsed_error.message))
                }
                "InternalError" => {
                    DeleteEndpointError::InternalError(String::from(parsed_error.message))
                }
                "InvalidParameter" => {
                    DeleteEndpointError::InvalidParameter(String::from(parsed_error.message))
                }
                _ => DeleteEndpointError::Unknown(String::from(body)),
            },
            Err(_) => DeleteEndpointError::Unknown(body.to_string()),
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

impl From<XmlParseError> for DeleteEndpointError {
    fn from(err: XmlParseError) -> DeleteEndpointError {
        let XmlParseError(message) = err;
        DeleteEndpointError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeleteEndpointError {
    fn from(err: CredentialsError) -> DeleteEndpointError {
        DeleteEndpointError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteEndpointError {
    fn from(err: HttpDispatchError) -> DeleteEndpointError {
        DeleteEndpointError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteEndpointError {
    fn from(err: io::Error) -> DeleteEndpointError {
        DeleteEndpointError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteEndpointError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteEndpointError {
    fn description(&self) -> &str {
        match *self {
            DeleteEndpointError::AuthorizationError(ref cause) => cause,
            DeleteEndpointError::InternalError(ref cause) => cause,
            DeleteEndpointError::InvalidParameter(ref cause) => cause,
            DeleteEndpointError::Validation(ref cause) => cause,
            DeleteEndpointError::Credentials(ref err) => err.description(),
            DeleteEndpointError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteEndpointError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeletePlatformApplication
#[derive(Debug, PartialEq)]
pub enum DeletePlatformApplicationError {
    /// <p>Indicates that the user has been denied access to the requested resource.</p>
    AuthorizationError(String),
    /// <p>Indicates an internal service error.</p>
    InternalError(String),
    /// <p>Indicates that a request parameter does not comply with the associated constraints.</p>
    InvalidParameter(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeletePlatformApplicationError {
    pub fn from_body(body: &str) -> DeletePlatformApplicationError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "AuthorizationError" => DeletePlatformApplicationError::AuthorizationError(
                    String::from(parsed_error.message),
                ),
                "InternalError" => DeletePlatformApplicationError::InternalError(String::from(
                    parsed_error.message,
                )),
                "InvalidParameter" => DeletePlatformApplicationError::InvalidParameter(
                    String::from(parsed_error.message),
                ),
                _ => DeletePlatformApplicationError::Unknown(String::from(body)),
            },
            Err(_) => DeletePlatformApplicationError::Unknown(body.to_string()),
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

impl From<XmlParseError> for DeletePlatformApplicationError {
    fn from(err: XmlParseError) -> DeletePlatformApplicationError {
        let XmlParseError(message) = err;
        DeletePlatformApplicationError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeletePlatformApplicationError {
    fn from(err: CredentialsError) -> DeletePlatformApplicationError {
        DeletePlatformApplicationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeletePlatformApplicationError {
    fn from(err: HttpDispatchError) -> DeletePlatformApplicationError {
        DeletePlatformApplicationError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeletePlatformApplicationError {
    fn from(err: io::Error) -> DeletePlatformApplicationError {
        DeletePlatformApplicationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeletePlatformApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeletePlatformApplicationError {
    fn description(&self) -> &str {
        match *self {
            DeletePlatformApplicationError::AuthorizationError(ref cause) => cause,
            DeletePlatformApplicationError::InternalError(ref cause) => cause,
            DeletePlatformApplicationError::InvalidParameter(ref cause) => cause,
            DeletePlatformApplicationError::Validation(ref cause) => cause,
            DeletePlatformApplicationError::Credentials(ref err) => err.description(),
            DeletePlatformApplicationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeletePlatformApplicationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteTopic
#[derive(Debug, PartialEq)]
pub enum DeleteTopicError {
    /// <p>Indicates that the user has been denied access to the requested resource.</p>
    AuthorizationError(String),
    /// <p>Indicates an internal service error.</p>
    InternalError(String),
    /// <p>Indicates that a request parameter does not comply with the associated constraints.</p>
    InvalidParameter(String),
    /// <p>Indicates that the requested resource does not exist.</p>
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

impl DeleteTopicError {
    pub fn from_body(body: &str) -> DeleteTopicError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "AuthorizationError" => {
                    DeleteTopicError::AuthorizationError(String::from(parsed_error.message))
                }
                "InternalError" => {
                    DeleteTopicError::InternalError(String::from(parsed_error.message))
                }
                "InvalidParameter" => {
                    DeleteTopicError::InvalidParameter(String::from(parsed_error.message))
                }
                "NotFound" => DeleteTopicError::NotFound(String::from(parsed_error.message)),
                _ => DeleteTopicError::Unknown(String::from(body)),
            },
            Err(_) => DeleteTopicError::Unknown(body.to_string()),
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

impl From<XmlParseError> for DeleteTopicError {
    fn from(err: XmlParseError) -> DeleteTopicError {
        let XmlParseError(message) = err;
        DeleteTopicError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeleteTopicError {
    fn from(err: CredentialsError) -> DeleteTopicError {
        DeleteTopicError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteTopicError {
    fn from(err: HttpDispatchError) -> DeleteTopicError {
        DeleteTopicError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteTopicError {
    fn from(err: io::Error) -> DeleteTopicError {
        DeleteTopicError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteTopicError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteTopicError {
    fn description(&self) -> &str {
        match *self {
            DeleteTopicError::AuthorizationError(ref cause) => cause,
            DeleteTopicError::InternalError(ref cause) => cause,
            DeleteTopicError::InvalidParameter(ref cause) => cause,
            DeleteTopicError::NotFound(ref cause) => cause,
            DeleteTopicError::Validation(ref cause) => cause,
            DeleteTopicError::Credentials(ref err) => err.description(),
            DeleteTopicError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteTopicError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetEndpointAttributes
#[derive(Debug, PartialEq)]
pub enum GetEndpointAttributesError {
    /// <p>Indicates that the user has been denied access to the requested resource.</p>
    AuthorizationError(String),
    /// <p>Indicates an internal service error.</p>
    InternalError(String),
    /// <p>Indicates that a request parameter does not comply with the associated constraints.</p>
    InvalidParameter(String),
    /// <p>Indicates that the requested resource does not exist.</p>
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

impl GetEndpointAttributesError {
    pub fn from_body(body: &str) -> GetEndpointAttributesError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "AuthorizationError" => GetEndpointAttributesError::AuthorizationError(
                    String::from(parsed_error.message),
                ),
                "InternalError" => {
                    GetEndpointAttributesError::InternalError(String::from(parsed_error.message))
                }
                "InvalidParameter" => {
                    GetEndpointAttributesError::InvalidParameter(String::from(parsed_error.message))
                }
                "NotFound" => {
                    GetEndpointAttributesError::NotFound(String::from(parsed_error.message))
                }
                _ => GetEndpointAttributesError::Unknown(String::from(body)),
            },
            Err(_) => GetEndpointAttributesError::Unknown(body.to_string()),
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

impl From<XmlParseError> for GetEndpointAttributesError {
    fn from(err: XmlParseError) -> GetEndpointAttributesError {
        let XmlParseError(message) = err;
        GetEndpointAttributesError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for GetEndpointAttributesError {
    fn from(err: CredentialsError) -> GetEndpointAttributesError {
        GetEndpointAttributesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetEndpointAttributesError {
    fn from(err: HttpDispatchError) -> GetEndpointAttributesError {
        GetEndpointAttributesError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetEndpointAttributesError {
    fn from(err: io::Error) -> GetEndpointAttributesError {
        GetEndpointAttributesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetEndpointAttributesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetEndpointAttributesError {
    fn description(&self) -> &str {
        match *self {
            GetEndpointAttributesError::AuthorizationError(ref cause) => cause,
            GetEndpointAttributesError::InternalError(ref cause) => cause,
            GetEndpointAttributesError::InvalidParameter(ref cause) => cause,
            GetEndpointAttributesError::NotFound(ref cause) => cause,
            GetEndpointAttributesError::Validation(ref cause) => cause,
            GetEndpointAttributesError::Credentials(ref err) => err.description(),
            GetEndpointAttributesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetEndpointAttributesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetPlatformApplicationAttributes
#[derive(Debug, PartialEq)]
pub enum GetPlatformApplicationAttributesError {
    /// <p>Indicates that the user has been denied access to the requested resource.</p>
    AuthorizationError(String),
    /// <p>Indicates an internal service error.</p>
    InternalError(String),
    /// <p>Indicates that a request parameter does not comply with the associated constraints.</p>
    InvalidParameter(String),
    /// <p>Indicates that the requested resource does not exist.</p>
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

impl GetPlatformApplicationAttributesError {
    pub fn from_body(body: &str) -> GetPlatformApplicationAttributesError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "AuthorizationError" => GetPlatformApplicationAttributesError::AuthorizationError(
                    String::from(parsed_error.message),
                ),
                "InternalError" => GetPlatformApplicationAttributesError::InternalError(
                    String::from(parsed_error.message),
                ),
                "InvalidParameter" => GetPlatformApplicationAttributesError::InvalidParameter(
                    String::from(parsed_error.message),
                ),
                "NotFound" => GetPlatformApplicationAttributesError::NotFound(String::from(
                    parsed_error.message,
                )),
                _ => GetPlatformApplicationAttributesError::Unknown(String::from(body)),
            },
            Err(_) => GetPlatformApplicationAttributesError::Unknown(body.to_string()),
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

impl From<XmlParseError> for GetPlatformApplicationAttributesError {
    fn from(err: XmlParseError) -> GetPlatformApplicationAttributesError {
        let XmlParseError(message) = err;
        GetPlatformApplicationAttributesError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for GetPlatformApplicationAttributesError {
    fn from(err: CredentialsError) -> GetPlatformApplicationAttributesError {
        GetPlatformApplicationAttributesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetPlatformApplicationAttributesError {
    fn from(err: HttpDispatchError) -> GetPlatformApplicationAttributesError {
        GetPlatformApplicationAttributesError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetPlatformApplicationAttributesError {
    fn from(err: io::Error) -> GetPlatformApplicationAttributesError {
        GetPlatformApplicationAttributesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetPlatformApplicationAttributesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetPlatformApplicationAttributesError {
    fn description(&self) -> &str {
        match *self {
            GetPlatformApplicationAttributesError::AuthorizationError(ref cause) => cause,
            GetPlatformApplicationAttributesError::InternalError(ref cause) => cause,
            GetPlatformApplicationAttributesError::InvalidParameter(ref cause) => cause,
            GetPlatformApplicationAttributesError::NotFound(ref cause) => cause,
            GetPlatformApplicationAttributesError::Validation(ref cause) => cause,
            GetPlatformApplicationAttributesError::Credentials(ref err) => err.description(),
            GetPlatformApplicationAttributesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetPlatformApplicationAttributesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetSMSAttributes
#[derive(Debug, PartialEq)]
pub enum GetSMSAttributesError {
    /// <p>Indicates that the user has been denied access to the requested resource.</p>
    AuthorizationError(String),
    /// <p>Indicates an internal service error.</p>
    InternalError(String),
    /// <p>Indicates that a request parameter does not comply with the associated constraints.</p>
    InvalidParameter(String),
    /// <p>Indicates that the rate at which requests have been submitted for this action exceeds the limit for your account.</p>
    Throttled(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetSMSAttributesError {
    pub fn from_body(body: &str) -> GetSMSAttributesError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "AuthorizationError" => {
                    GetSMSAttributesError::AuthorizationError(String::from(parsed_error.message))
                }
                "InternalError" => {
                    GetSMSAttributesError::InternalError(String::from(parsed_error.message))
                }
                "InvalidParameter" => {
                    GetSMSAttributesError::InvalidParameter(String::from(parsed_error.message))
                }
                "Throttled" => GetSMSAttributesError::Throttled(String::from(parsed_error.message)),
                _ => GetSMSAttributesError::Unknown(String::from(body)),
            },
            Err(_) => GetSMSAttributesError::Unknown(body.to_string()),
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

impl From<XmlParseError> for GetSMSAttributesError {
    fn from(err: XmlParseError) -> GetSMSAttributesError {
        let XmlParseError(message) = err;
        GetSMSAttributesError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for GetSMSAttributesError {
    fn from(err: CredentialsError) -> GetSMSAttributesError {
        GetSMSAttributesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetSMSAttributesError {
    fn from(err: HttpDispatchError) -> GetSMSAttributesError {
        GetSMSAttributesError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetSMSAttributesError {
    fn from(err: io::Error) -> GetSMSAttributesError {
        GetSMSAttributesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetSMSAttributesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetSMSAttributesError {
    fn description(&self) -> &str {
        match *self {
            GetSMSAttributesError::AuthorizationError(ref cause) => cause,
            GetSMSAttributesError::InternalError(ref cause) => cause,
            GetSMSAttributesError::InvalidParameter(ref cause) => cause,
            GetSMSAttributesError::Throttled(ref cause) => cause,
            GetSMSAttributesError::Validation(ref cause) => cause,
            GetSMSAttributesError::Credentials(ref err) => err.description(),
            GetSMSAttributesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetSMSAttributesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetSubscriptionAttributes
#[derive(Debug, PartialEq)]
pub enum GetSubscriptionAttributesError {
    /// <p>Indicates that the user has been denied access to the requested resource.</p>
    AuthorizationError(String),
    /// <p>Indicates an internal service error.</p>
    InternalError(String),
    /// <p>Indicates that a request parameter does not comply with the associated constraints.</p>
    InvalidParameter(String),
    /// <p>Indicates that the requested resource does not exist.</p>
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

impl GetSubscriptionAttributesError {
    pub fn from_body(body: &str) -> GetSubscriptionAttributesError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "AuthorizationError" => GetSubscriptionAttributesError::AuthorizationError(
                    String::from(parsed_error.message),
                ),
                "InternalError" => GetSubscriptionAttributesError::InternalError(String::from(
                    parsed_error.message,
                )),
                "InvalidParameter" => GetSubscriptionAttributesError::InvalidParameter(
                    String::from(parsed_error.message),
                ),
                "NotFound" => {
                    GetSubscriptionAttributesError::NotFound(String::from(parsed_error.message))
                }
                _ => GetSubscriptionAttributesError::Unknown(String::from(body)),
            },
            Err(_) => GetSubscriptionAttributesError::Unknown(body.to_string()),
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

impl From<XmlParseError> for GetSubscriptionAttributesError {
    fn from(err: XmlParseError) -> GetSubscriptionAttributesError {
        let XmlParseError(message) = err;
        GetSubscriptionAttributesError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for GetSubscriptionAttributesError {
    fn from(err: CredentialsError) -> GetSubscriptionAttributesError {
        GetSubscriptionAttributesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetSubscriptionAttributesError {
    fn from(err: HttpDispatchError) -> GetSubscriptionAttributesError {
        GetSubscriptionAttributesError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetSubscriptionAttributesError {
    fn from(err: io::Error) -> GetSubscriptionAttributesError {
        GetSubscriptionAttributesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetSubscriptionAttributesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetSubscriptionAttributesError {
    fn description(&self) -> &str {
        match *self {
            GetSubscriptionAttributesError::AuthorizationError(ref cause) => cause,
            GetSubscriptionAttributesError::InternalError(ref cause) => cause,
            GetSubscriptionAttributesError::InvalidParameter(ref cause) => cause,
            GetSubscriptionAttributesError::NotFound(ref cause) => cause,
            GetSubscriptionAttributesError::Validation(ref cause) => cause,
            GetSubscriptionAttributesError::Credentials(ref err) => err.description(),
            GetSubscriptionAttributesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetSubscriptionAttributesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetTopicAttributes
#[derive(Debug, PartialEq)]
pub enum GetTopicAttributesError {
    /// <p>Indicates that the user has been denied access to the requested resource.</p>
    AuthorizationError(String),
    /// <p>Indicates an internal service error.</p>
    InternalError(String),
    /// <p>Indicates that a request parameter does not comply with the associated constraints.</p>
    InvalidParameter(String),
    /// <p>Indicates that the requested resource does not exist.</p>
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

impl GetTopicAttributesError {
    pub fn from_body(body: &str) -> GetTopicAttributesError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "AuthorizationError" => {
                    GetTopicAttributesError::AuthorizationError(String::from(parsed_error.message))
                }
                "InternalError" => {
                    GetTopicAttributesError::InternalError(String::from(parsed_error.message))
                }
                "InvalidParameter" => {
                    GetTopicAttributesError::InvalidParameter(String::from(parsed_error.message))
                }
                "NotFound" => GetTopicAttributesError::NotFound(String::from(parsed_error.message)),
                _ => GetTopicAttributesError::Unknown(String::from(body)),
            },
            Err(_) => GetTopicAttributesError::Unknown(body.to_string()),
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

impl From<XmlParseError> for GetTopicAttributesError {
    fn from(err: XmlParseError) -> GetTopicAttributesError {
        let XmlParseError(message) = err;
        GetTopicAttributesError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for GetTopicAttributesError {
    fn from(err: CredentialsError) -> GetTopicAttributesError {
        GetTopicAttributesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetTopicAttributesError {
    fn from(err: HttpDispatchError) -> GetTopicAttributesError {
        GetTopicAttributesError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetTopicAttributesError {
    fn from(err: io::Error) -> GetTopicAttributesError {
        GetTopicAttributesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetTopicAttributesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetTopicAttributesError {
    fn description(&self) -> &str {
        match *self {
            GetTopicAttributesError::AuthorizationError(ref cause) => cause,
            GetTopicAttributesError::InternalError(ref cause) => cause,
            GetTopicAttributesError::InvalidParameter(ref cause) => cause,
            GetTopicAttributesError::NotFound(ref cause) => cause,
            GetTopicAttributesError::Validation(ref cause) => cause,
            GetTopicAttributesError::Credentials(ref err) => err.description(),
            GetTopicAttributesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetTopicAttributesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListEndpointsByPlatformApplication
#[derive(Debug, PartialEq)]
pub enum ListEndpointsByPlatformApplicationError {
    /// <p>Indicates that the user has been denied access to the requested resource.</p>
    AuthorizationError(String),
    /// <p>Indicates an internal service error.</p>
    InternalError(String),
    /// <p>Indicates that a request parameter does not comply with the associated constraints.</p>
    InvalidParameter(String),
    /// <p>Indicates that the requested resource does not exist.</p>
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

impl ListEndpointsByPlatformApplicationError {
    pub fn from_body(body: &str) -> ListEndpointsByPlatformApplicationError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "AuthorizationError" => {
                    ListEndpointsByPlatformApplicationError::AuthorizationError(String::from(
                        parsed_error.message,
                    ))
                }
                "InternalError" => ListEndpointsByPlatformApplicationError::InternalError(
                    String::from(parsed_error.message),
                ),
                "InvalidParameter" => ListEndpointsByPlatformApplicationError::InvalidParameter(
                    String::from(parsed_error.message),
                ),
                "NotFound" => ListEndpointsByPlatformApplicationError::NotFound(String::from(
                    parsed_error.message,
                )),
                _ => ListEndpointsByPlatformApplicationError::Unknown(String::from(body)),
            },
            Err(_) => ListEndpointsByPlatformApplicationError::Unknown(body.to_string()),
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

impl From<XmlParseError> for ListEndpointsByPlatformApplicationError {
    fn from(err: XmlParseError) -> ListEndpointsByPlatformApplicationError {
        let XmlParseError(message) = err;
        ListEndpointsByPlatformApplicationError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ListEndpointsByPlatformApplicationError {
    fn from(err: CredentialsError) -> ListEndpointsByPlatformApplicationError {
        ListEndpointsByPlatformApplicationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListEndpointsByPlatformApplicationError {
    fn from(err: HttpDispatchError) -> ListEndpointsByPlatformApplicationError {
        ListEndpointsByPlatformApplicationError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListEndpointsByPlatformApplicationError {
    fn from(err: io::Error) -> ListEndpointsByPlatformApplicationError {
        ListEndpointsByPlatformApplicationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListEndpointsByPlatformApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListEndpointsByPlatformApplicationError {
    fn description(&self) -> &str {
        match *self {
            ListEndpointsByPlatformApplicationError::AuthorizationError(ref cause) => cause,
            ListEndpointsByPlatformApplicationError::InternalError(ref cause) => cause,
            ListEndpointsByPlatformApplicationError::InvalidParameter(ref cause) => cause,
            ListEndpointsByPlatformApplicationError::NotFound(ref cause) => cause,
            ListEndpointsByPlatformApplicationError::Validation(ref cause) => cause,
            ListEndpointsByPlatformApplicationError::Credentials(ref err) => err.description(),
            ListEndpointsByPlatformApplicationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListEndpointsByPlatformApplicationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListPhoneNumbersOptedOut
#[derive(Debug, PartialEq)]
pub enum ListPhoneNumbersOptedOutError {
    /// <p>Indicates that the user has been denied access to the requested resource.</p>
    AuthorizationError(String),
    /// <p>Indicates an internal service error.</p>
    InternalError(String),
    /// <p>Indicates that a request parameter does not comply with the associated constraints.</p>
    InvalidParameter(String),
    /// <p>Indicates that the rate at which requests have been submitted for this action exceeds the limit for your account.</p>
    Throttled(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListPhoneNumbersOptedOutError {
    pub fn from_body(body: &str) -> ListPhoneNumbersOptedOutError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "AuthorizationError" => ListPhoneNumbersOptedOutError::AuthorizationError(
                    String::from(parsed_error.message),
                ),
                "InternalError" => {
                    ListPhoneNumbersOptedOutError::InternalError(String::from(parsed_error.message))
                }
                "InvalidParameter" => ListPhoneNumbersOptedOutError::InvalidParameter(
                    String::from(parsed_error.message),
                ),
                "Throttled" => {
                    ListPhoneNumbersOptedOutError::Throttled(String::from(parsed_error.message))
                }
                _ => ListPhoneNumbersOptedOutError::Unknown(String::from(body)),
            },
            Err(_) => ListPhoneNumbersOptedOutError::Unknown(body.to_string()),
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

impl From<XmlParseError> for ListPhoneNumbersOptedOutError {
    fn from(err: XmlParseError) -> ListPhoneNumbersOptedOutError {
        let XmlParseError(message) = err;
        ListPhoneNumbersOptedOutError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ListPhoneNumbersOptedOutError {
    fn from(err: CredentialsError) -> ListPhoneNumbersOptedOutError {
        ListPhoneNumbersOptedOutError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListPhoneNumbersOptedOutError {
    fn from(err: HttpDispatchError) -> ListPhoneNumbersOptedOutError {
        ListPhoneNumbersOptedOutError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListPhoneNumbersOptedOutError {
    fn from(err: io::Error) -> ListPhoneNumbersOptedOutError {
        ListPhoneNumbersOptedOutError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListPhoneNumbersOptedOutError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListPhoneNumbersOptedOutError {
    fn description(&self) -> &str {
        match *self {
            ListPhoneNumbersOptedOutError::AuthorizationError(ref cause) => cause,
            ListPhoneNumbersOptedOutError::InternalError(ref cause) => cause,
            ListPhoneNumbersOptedOutError::InvalidParameter(ref cause) => cause,
            ListPhoneNumbersOptedOutError::Throttled(ref cause) => cause,
            ListPhoneNumbersOptedOutError::Validation(ref cause) => cause,
            ListPhoneNumbersOptedOutError::Credentials(ref err) => err.description(),
            ListPhoneNumbersOptedOutError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListPhoneNumbersOptedOutError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListPlatformApplications
#[derive(Debug, PartialEq)]
pub enum ListPlatformApplicationsError {
    /// <p>Indicates that the user has been denied access to the requested resource.</p>
    AuthorizationError(String),
    /// <p>Indicates an internal service error.</p>
    InternalError(String),
    /// <p>Indicates that a request parameter does not comply with the associated constraints.</p>
    InvalidParameter(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListPlatformApplicationsError {
    pub fn from_body(body: &str) -> ListPlatformApplicationsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "AuthorizationError" => ListPlatformApplicationsError::AuthorizationError(
                    String::from(parsed_error.message),
                ),
                "InternalError" => {
                    ListPlatformApplicationsError::InternalError(String::from(parsed_error.message))
                }
                "InvalidParameter" => ListPlatformApplicationsError::InvalidParameter(
                    String::from(parsed_error.message),
                ),
                _ => ListPlatformApplicationsError::Unknown(String::from(body)),
            },
            Err(_) => ListPlatformApplicationsError::Unknown(body.to_string()),
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

impl From<XmlParseError> for ListPlatformApplicationsError {
    fn from(err: XmlParseError) -> ListPlatformApplicationsError {
        let XmlParseError(message) = err;
        ListPlatformApplicationsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ListPlatformApplicationsError {
    fn from(err: CredentialsError) -> ListPlatformApplicationsError {
        ListPlatformApplicationsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListPlatformApplicationsError {
    fn from(err: HttpDispatchError) -> ListPlatformApplicationsError {
        ListPlatformApplicationsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListPlatformApplicationsError {
    fn from(err: io::Error) -> ListPlatformApplicationsError {
        ListPlatformApplicationsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListPlatformApplicationsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListPlatformApplicationsError {
    fn description(&self) -> &str {
        match *self {
            ListPlatformApplicationsError::AuthorizationError(ref cause) => cause,
            ListPlatformApplicationsError::InternalError(ref cause) => cause,
            ListPlatformApplicationsError::InvalidParameter(ref cause) => cause,
            ListPlatformApplicationsError::Validation(ref cause) => cause,
            ListPlatformApplicationsError::Credentials(ref err) => err.description(),
            ListPlatformApplicationsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListPlatformApplicationsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListSubscriptions
#[derive(Debug, PartialEq)]
pub enum ListSubscriptionsError {
    /// <p>Indicates that the user has been denied access to the requested resource.</p>
    AuthorizationError(String),
    /// <p>Indicates an internal service error.</p>
    InternalError(String),
    /// <p>Indicates that a request parameter does not comply with the associated constraints.</p>
    InvalidParameter(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListSubscriptionsError {
    pub fn from_body(body: &str) -> ListSubscriptionsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "AuthorizationError" => {
                    ListSubscriptionsError::AuthorizationError(String::from(parsed_error.message))
                }
                "InternalError" => {
                    ListSubscriptionsError::InternalError(String::from(parsed_error.message))
                }
                "InvalidParameter" => {
                    ListSubscriptionsError::InvalidParameter(String::from(parsed_error.message))
                }
                _ => ListSubscriptionsError::Unknown(String::from(body)),
            },
            Err(_) => ListSubscriptionsError::Unknown(body.to_string()),
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

impl From<XmlParseError> for ListSubscriptionsError {
    fn from(err: XmlParseError) -> ListSubscriptionsError {
        let XmlParseError(message) = err;
        ListSubscriptionsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ListSubscriptionsError {
    fn from(err: CredentialsError) -> ListSubscriptionsError {
        ListSubscriptionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListSubscriptionsError {
    fn from(err: HttpDispatchError) -> ListSubscriptionsError {
        ListSubscriptionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListSubscriptionsError {
    fn from(err: io::Error) -> ListSubscriptionsError {
        ListSubscriptionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListSubscriptionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListSubscriptionsError {
    fn description(&self) -> &str {
        match *self {
            ListSubscriptionsError::AuthorizationError(ref cause) => cause,
            ListSubscriptionsError::InternalError(ref cause) => cause,
            ListSubscriptionsError::InvalidParameter(ref cause) => cause,
            ListSubscriptionsError::Validation(ref cause) => cause,
            ListSubscriptionsError::Credentials(ref err) => err.description(),
            ListSubscriptionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListSubscriptionsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListSubscriptionsByTopic
#[derive(Debug, PartialEq)]
pub enum ListSubscriptionsByTopicError {
    /// <p>Indicates that the user has been denied access to the requested resource.</p>
    AuthorizationError(String),
    /// <p>Indicates an internal service error.</p>
    InternalError(String),
    /// <p>Indicates that a request parameter does not comply with the associated constraints.</p>
    InvalidParameter(String),
    /// <p>Indicates that the requested resource does not exist.</p>
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

impl ListSubscriptionsByTopicError {
    pub fn from_body(body: &str) -> ListSubscriptionsByTopicError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "AuthorizationError" => ListSubscriptionsByTopicError::AuthorizationError(
                    String::from(parsed_error.message),
                ),
                "InternalError" => {
                    ListSubscriptionsByTopicError::InternalError(String::from(parsed_error.message))
                }
                "InvalidParameter" => ListSubscriptionsByTopicError::InvalidParameter(
                    String::from(parsed_error.message),
                ),
                "NotFound" => {
                    ListSubscriptionsByTopicError::NotFound(String::from(parsed_error.message))
                }
                _ => ListSubscriptionsByTopicError::Unknown(String::from(body)),
            },
            Err(_) => ListSubscriptionsByTopicError::Unknown(body.to_string()),
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

impl From<XmlParseError> for ListSubscriptionsByTopicError {
    fn from(err: XmlParseError) -> ListSubscriptionsByTopicError {
        let XmlParseError(message) = err;
        ListSubscriptionsByTopicError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ListSubscriptionsByTopicError {
    fn from(err: CredentialsError) -> ListSubscriptionsByTopicError {
        ListSubscriptionsByTopicError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListSubscriptionsByTopicError {
    fn from(err: HttpDispatchError) -> ListSubscriptionsByTopicError {
        ListSubscriptionsByTopicError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListSubscriptionsByTopicError {
    fn from(err: io::Error) -> ListSubscriptionsByTopicError {
        ListSubscriptionsByTopicError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListSubscriptionsByTopicError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListSubscriptionsByTopicError {
    fn description(&self) -> &str {
        match *self {
            ListSubscriptionsByTopicError::AuthorizationError(ref cause) => cause,
            ListSubscriptionsByTopicError::InternalError(ref cause) => cause,
            ListSubscriptionsByTopicError::InvalidParameter(ref cause) => cause,
            ListSubscriptionsByTopicError::NotFound(ref cause) => cause,
            ListSubscriptionsByTopicError::Validation(ref cause) => cause,
            ListSubscriptionsByTopicError::Credentials(ref err) => err.description(),
            ListSubscriptionsByTopicError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListSubscriptionsByTopicError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListTopics
#[derive(Debug, PartialEq)]
pub enum ListTopicsError {
    /// <p>Indicates that the user has been denied access to the requested resource.</p>
    AuthorizationError(String),
    /// <p>Indicates an internal service error.</p>
    InternalError(String),
    /// <p>Indicates that a request parameter does not comply with the associated constraints.</p>
    InvalidParameter(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListTopicsError {
    pub fn from_body(body: &str) -> ListTopicsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "AuthorizationError" => {
                    ListTopicsError::AuthorizationError(String::from(parsed_error.message))
                }
                "InternalError" => {
                    ListTopicsError::InternalError(String::from(parsed_error.message))
                }
                "InvalidParameter" => {
                    ListTopicsError::InvalidParameter(String::from(parsed_error.message))
                }
                _ => ListTopicsError::Unknown(String::from(body)),
            },
            Err(_) => ListTopicsError::Unknown(body.to_string()),
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

impl From<XmlParseError> for ListTopicsError {
    fn from(err: XmlParseError) -> ListTopicsError {
        let XmlParseError(message) = err;
        ListTopicsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ListTopicsError {
    fn from(err: CredentialsError) -> ListTopicsError {
        ListTopicsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListTopicsError {
    fn from(err: HttpDispatchError) -> ListTopicsError {
        ListTopicsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListTopicsError {
    fn from(err: io::Error) -> ListTopicsError {
        ListTopicsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListTopicsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListTopicsError {
    fn description(&self) -> &str {
        match *self {
            ListTopicsError::AuthorizationError(ref cause) => cause,
            ListTopicsError::InternalError(ref cause) => cause,
            ListTopicsError::InvalidParameter(ref cause) => cause,
            ListTopicsError::Validation(ref cause) => cause,
            ListTopicsError::Credentials(ref err) => err.description(),
            ListTopicsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListTopicsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by OptInPhoneNumber
#[derive(Debug, PartialEq)]
pub enum OptInPhoneNumberError {
    /// <p>Indicates that the user has been denied access to the requested resource.</p>
    AuthorizationError(String),
    /// <p>Indicates an internal service error.</p>
    InternalError(String),
    /// <p>Indicates that a request parameter does not comply with the associated constraints.</p>
    InvalidParameter(String),
    /// <p>Indicates that the rate at which requests have been submitted for this action exceeds the limit for your account.</p>
    Throttled(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl OptInPhoneNumberError {
    pub fn from_body(body: &str) -> OptInPhoneNumberError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "AuthorizationError" => {
                    OptInPhoneNumberError::AuthorizationError(String::from(parsed_error.message))
                }
                "InternalError" => {
                    OptInPhoneNumberError::InternalError(String::from(parsed_error.message))
                }
                "InvalidParameter" => {
                    OptInPhoneNumberError::InvalidParameter(String::from(parsed_error.message))
                }
                "Throttled" => OptInPhoneNumberError::Throttled(String::from(parsed_error.message)),
                _ => OptInPhoneNumberError::Unknown(String::from(body)),
            },
            Err(_) => OptInPhoneNumberError::Unknown(body.to_string()),
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

impl From<XmlParseError> for OptInPhoneNumberError {
    fn from(err: XmlParseError) -> OptInPhoneNumberError {
        let XmlParseError(message) = err;
        OptInPhoneNumberError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for OptInPhoneNumberError {
    fn from(err: CredentialsError) -> OptInPhoneNumberError {
        OptInPhoneNumberError::Credentials(err)
    }
}
impl From<HttpDispatchError> for OptInPhoneNumberError {
    fn from(err: HttpDispatchError) -> OptInPhoneNumberError {
        OptInPhoneNumberError::HttpDispatch(err)
    }
}
impl From<io::Error> for OptInPhoneNumberError {
    fn from(err: io::Error) -> OptInPhoneNumberError {
        OptInPhoneNumberError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for OptInPhoneNumberError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for OptInPhoneNumberError {
    fn description(&self) -> &str {
        match *self {
            OptInPhoneNumberError::AuthorizationError(ref cause) => cause,
            OptInPhoneNumberError::InternalError(ref cause) => cause,
            OptInPhoneNumberError::InvalidParameter(ref cause) => cause,
            OptInPhoneNumberError::Throttled(ref cause) => cause,
            OptInPhoneNumberError::Validation(ref cause) => cause,
            OptInPhoneNumberError::Credentials(ref err) => err.description(),
            OptInPhoneNumberError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            OptInPhoneNumberError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by Publish
#[derive(Debug, PartialEq)]
pub enum PublishError {
    /// <p>Indicates that the user has been denied access to the requested resource.</p>
    AuthorizationError(String),
    /// <p>Exception error indicating endpoint disabled.</p>
    EndpointDisabled(String),
    /// <p>Indicates an internal service error.</p>
    InternalError(String),
    /// <p>Indicates that a request parameter does not comply with the associated constraints.</p>
    InvalidParameter(String),
    /// <p>Indicates that a request parameter does not comply with the associated constraints.</p>
    InvalidParameterValue(String),
    /// <p>Indicates that the requested resource does not exist.</p>
    NotFound(String),
    /// <p>Exception error indicating platform application disabled.</p>
    PlatformApplicationDisabled(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl PublishError {
    pub fn from_body(body: &str) -> PublishError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "AuthorizationError" => {
                    PublishError::AuthorizationError(String::from(parsed_error.message))
                }
                "EndpointDisabled" => {
                    PublishError::EndpointDisabled(String::from(parsed_error.message))
                }
                "InternalError" => PublishError::InternalError(String::from(parsed_error.message)),
                "InvalidParameter" => {
                    PublishError::InvalidParameter(String::from(parsed_error.message))
                }
                "ParameterValueInvalid" => {
                    PublishError::InvalidParameterValue(String::from(parsed_error.message))
                }
                "NotFound" => PublishError::NotFound(String::from(parsed_error.message)),
                "PlatformApplicationDisabled" => {
                    PublishError::PlatformApplicationDisabled(String::from(parsed_error.message))
                }
                _ => PublishError::Unknown(String::from(body)),
            },
            Err(_) => PublishError::Unknown(body.to_string()),
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

impl From<XmlParseError> for PublishError {
    fn from(err: XmlParseError) -> PublishError {
        let XmlParseError(message) = err;
        PublishError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for PublishError {
    fn from(err: CredentialsError) -> PublishError {
        PublishError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PublishError {
    fn from(err: HttpDispatchError) -> PublishError {
        PublishError::HttpDispatch(err)
    }
}
impl From<io::Error> for PublishError {
    fn from(err: io::Error) -> PublishError {
        PublishError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PublishError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PublishError {
    fn description(&self) -> &str {
        match *self {
            PublishError::AuthorizationError(ref cause) => cause,
            PublishError::EndpointDisabled(ref cause) => cause,
            PublishError::InternalError(ref cause) => cause,
            PublishError::InvalidParameter(ref cause) => cause,
            PublishError::InvalidParameterValue(ref cause) => cause,
            PublishError::NotFound(ref cause) => cause,
            PublishError::PlatformApplicationDisabled(ref cause) => cause,
            PublishError::Validation(ref cause) => cause,
            PublishError::Credentials(ref err) => err.description(),
            PublishError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            PublishError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RemovePermission
#[derive(Debug, PartialEq)]
pub enum RemovePermissionError {
    /// <p>Indicates that the user has been denied access to the requested resource.</p>
    AuthorizationError(String),
    /// <p>Indicates an internal service error.</p>
    InternalError(String),
    /// <p>Indicates that a request parameter does not comply with the associated constraints.</p>
    InvalidParameter(String),
    /// <p>Indicates that the requested resource does not exist.</p>
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

impl RemovePermissionError {
    pub fn from_body(body: &str) -> RemovePermissionError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "AuthorizationError" => {
                    RemovePermissionError::AuthorizationError(String::from(parsed_error.message))
                }
                "InternalError" => {
                    RemovePermissionError::InternalError(String::from(parsed_error.message))
                }
                "InvalidParameter" => {
                    RemovePermissionError::InvalidParameter(String::from(parsed_error.message))
                }
                "NotFound" => RemovePermissionError::NotFound(String::from(parsed_error.message)),
                _ => RemovePermissionError::Unknown(String::from(body)),
            },
            Err(_) => RemovePermissionError::Unknown(body.to_string()),
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

impl From<XmlParseError> for RemovePermissionError {
    fn from(err: XmlParseError) -> RemovePermissionError {
        let XmlParseError(message) = err;
        RemovePermissionError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for RemovePermissionError {
    fn from(err: CredentialsError) -> RemovePermissionError {
        RemovePermissionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RemovePermissionError {
    fn from(err: HttpDispatchError) -> RemovePermissionError {
        RemovePermissionError::HttpDispatch(err)
    }
}
impl From<io::Error> for RemovePermissionError {
    fn from(err: io::Error) -> RemovePermissionError {
        RemovePermissionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RemovePermissionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RemovePermissionError {
    fn description(&self) -> &str {
        match *self {
            RemovePermissionError::AuthorizationError(ref cause) => cause,
            RemovePermissionError::InternalError(ref cause) => cause,
            RemovePermissionError::InvalidParameter(ref cause) => cause,
            RemovePermissionError::NotFound(ref cause) => cause,
            RemovePermissionError::Validation(ref cause) => cause,
            RemovePermissionError::Credentials(ref err) => err.description(),
            RemovePermissionError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            RemovePermissionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by SetEndpointAttributes
#[derive(Debug, PartialEq)]
pub enum SetEndpointAttributesError {
    /// <p>Indicates that the user has been denied access to the requested resource.</p>
    AuthorizationError(String),
    /// <p>Indicates an internal service error.</p>
    InternalError(String),
    /// <p>Indicates that a request parameter does not comply with the associated constraints.</p>
    InvalidParameter(String),
    /// <p>Indicates that the requested resource does not exist.</p>
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

impl SetEndpointAttributesError {
    pub fn from_body(body: &str) -> SetEndpointAttributesError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "AuthorizationError" => SetEndpointAttributesError::AuthorizationError(
                    String::from(parsed_error.message),
                ),
                "InternalError" => {
                    SetEndpointAttributesError::InternalError(String::from(parsed_error.message))
                }
                "InvalidParameter" => {
                    SetEndpointAttributesError::InvalidParameter(String::from(parsed_error.message))
                }
                "NotFound" => {
                    SetEndpointAttributesError::NotFound(String::from(parsed_error.message))
                }
                _ => SetEndpointAttributesError::Unknown(String::from(body)),
            },
            Err(_) => SetEndpointAttributesError::Unknown(body.to_string()),
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

impl From<XmlParseError> for SetEndpointAttributesError {
    fn from(err: XmlParseError) -> SetEndpointAttributesError {
        let XmlParseError(message) = err;
        SetEndpointAttributesError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for SetEndpointAttributesError {
    fn from(err: CredentialsError) -> SetEndpointAttributesError {
        SetEndpointAttributesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for SetEndpointAttributesError {
    fn from(err: HttpDispatchError) -> SetEndpointAttributesError {
        SetEndpointAttributesError::HttpDispatch(err)
    }
}
impl From<io::Error> for SetEndpointAttributesError {
    fn from(err: io::Error) -> SetEndpointAttributesError {
        SetEndpointAttributesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for SetEndpointAttributesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SetEndpointAttributesError {
    fn description(&self) -> &str {
        match *self {
            SetEndpointAttributesError::AuthorizationError(ref cause) => cause,
            SetEndpointAttributesError::InternalError(ref cause) => cause,
            SetEndpointAttributesError::InvalidParameter(ref cause) => cause,
            SetEndpointAttributesError::NotFound(ref cause) => cause,
            SetEndpointAttributesError::Validation(ref cause) => cause,
            SetEndpointAttributesError::Credentials(ref err) => err.description(),
            SetEndpointAttributesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            SetEndpointAttributesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by SetPlatformApplicationAttributes
#[derive(Debug, PartialEq)]
pub enum SetPlatformApplicationAttributesError {
    /// <p>Indicates that the user has been denied access to the requested resource.</p>
    AuthorizationError(String),
    /// <p>Indicates an internal service error.</p>
    InternalError(String),
    /// <p>Indicates that a request parameter does not comply with the associated constraints.</p>
    InvalidParameter(String),
    /// <p>Indicates that the requested resource does not exist.</p>
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

impl SetPlatformApplicationAttributesError {
    pub fn from_body(body: &str) -> SetPlatformApplicationAttributesError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "AuthorizationError" => SetPlatformApplicationAttributesError::AuthorizationError(
                    String::from(parsed_error.message),
                ),
                "InternalError" => SetPlatformApplicationAttributesError::InternalError(
                    String::from(parsed_error.message),
                ),
                "InvalidParameter" => SetPlatformApplicationAttributesError::InvalidParameter(
                    String::from(parsed_error.message),
                ),
                "NotFound" => SetPlatformApplicationAttributesError::NotFound(String::from(
                    parsed_error.message,
                )),
                _ => SetPlatformApplicationAttributesError::Unknown(String::from(body)),
            },
            Err(_) => SetPlatformApplicationAttributesError::Unknown(body.to_string()),
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

impl From<XmlParseError> for SetPlatformApplicationAttributesError {
    fn from(err: XmlParseError) -> SetPlatformApplicationAttributesError {
        let XmlParseError(message) = err;
        SetPlatformApplicationAttributesError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for SetPlatformApplicationAttributesError {
    fn from(err: CredentialsError) -> SetPlatformApplicationAttributesError {
        SetPlatformApplicationAttributesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for SetPlatformApplicationAttributesError {
    fn from(err: HttpDispatchError) -> SetPlatformApplicationAttributesError {
        SetPlatformApplicationAttributesError::HttpDispatch(err)
    }
}
impl From<io::Error> for SetPlatformApplicationAttributesError {
    fn from(err: io::Error) -> SetPlatformApplicationAttributesError {
        SetPlatformApplicationAttributesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for SetPlatformApplicationAttributesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SetPlatformApplicationAttributesError {
    fn description(&self) -> &str {
        match *self {
            SetPlatformApplicationAttributesError::AuthorizationError(ref cause) => cause,
            SetPlatformApplicationAttributesError::InternalError(ref cause) => cause,
            SetPlatformApplicationAttributesError::InvalidParameter(ref cause) => cause,
            SetPlatformApplicationAttributesError::NotFound(ref cause) => cause,
            SetPlatformApplicationAttributesError::Validation(ref cause) => cause,
            SetPlatformApplicationAttributesError::Credentials(ref err) => err.description(),
            SetPlatformApplicationAttributesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            SetPlatformApplicationAttributesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by SetSMSAttributes
#[derive(Debug, PartialEq)]
pub enum SetSMSAttributesError {
    /// <p>Indicates that the user has been denied access to the requested resource.</p>
    AuthorizationError(String),
    /// <p>Indicates an internal service error.</p>
    InternalError(String),
    /// <p>Indicates that a request parameter does not comply with the associated constraints.</p>
    InvalidParameter(String),
    /// <p>Indicates that the rate at which requests have been submitted for this action exceeds the limit for your account.</p>
    Throttled(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl SetSMSAttributesError {
    pub fn from_body(body: &str) -> SetSMSAttributesError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "AuthorizationError" => {
                    SetSMSAttributesError::AuthorizationError(String::from(parsed_error.message))
                }
                "InternalError" => {
                    SetSMSAttributesError::InternalError(String::from(parsed_error.message))
                }
                "InvalidParameter" => {
                    SetSMSAttributesError::InvalidParameter(String::from(parsed_error.message))
                }
                "Throttled" => SetSMSAttributesError::Throttled(String::from(parsed_error.message)),
                _ => SetSMSAttributesError::Unknown(String::from(body)),
            },
            Err(_) => SetSMSAttributesError::Unknown(body.to_string()),
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

impl From<XmlParseError> for SetSMSAttributesError {
    fn from(err: XmlParseError) -> SetSMSAttributesError {
        let XmlParseError(message) = err;
        SetSMSAttributesError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for SetSMSAttributesError {
    fn from(err: CredentialsError) -> SetSMSAttributesError {
        SetSMSAttributesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for SetSMSAttributesError {
    fn from(err: HttpDispatchError) -> SetSMSAttributesError {
        SetSMSAttributesError::HttpDispatch(err)
    }
}
impl From<io::Error> for SetSMSAttributesError {
    fn from(err: io::Error) -> SetSMSAttributesError {
        SetSMSAttributesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for SetSMSAttributesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SetSMSAttributesError {
    fn description(&self) -> &str {
        match *self {
            SetSMSAttributesError::AuthorizationError(ref cause) => cause,
            SetSMSAttributesError::InternalError(ref cause) => cause,
            SetSMSAttributesError::InvalidParameter(ref cause) => cause,
            SetSMSAttributesError::Throttled(ref cause) => cause,
            SetSMSAttributesError::Validation(ref cause) => cause,
            SetSMSAttributesError::Credentials(ref err) => err.description(),
            SetSMSAttributesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            SetSMSAttributesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by SetSubscriptionAttributes
#[derive(Debug, PartialEq)]
pub enum SetSubscriptionAttributesError {
    /// <p>Indicates that the user has been denied access to the requested resource.</p>
    AuthorizationError(String),
    /// <p>Indicates an internal service error.</p>
    InternalError(String),
    /// <p>Indicates that a request parameter does not comply with the associated constraints.</p>
    InvalidParameter(String),
    /// <p>Indicates that the requested resource does not exist.</p>
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

impl SetSubscriptionAttributesError {
    pub fn from_body(body: &str) -> SetSubscriptionAttributesError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "AuthorizationError" => SetSubscriptionAttributesError::AuthorizationError(
                    String::from(parsed_error.message),
                ),
                "InternalError" => SetSubscriptionAttributesError::InternalError(String::from(
                    parsed_error.message,
                )),
                "InvalidParameter" => SetSubscriptionAttributesError::InvalidParameter(
                    String::from(parsed_error.message),
                ),
                "NotFound" => {
                    SetSubscriptionAttributesError::NotFound(String::from(parsed_error.message))
                }
                _ => SetSubscriptionAttributesError::Unknown(String::from(body)),
            },
            Err(_) => SetSubscriptionAttributesError::Unknown(body.to_string()),
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

impl From<XmlParseError> for SetSubscriptionAttributesError {
    fn from(err: XmlParseError) -> SetSubscriptionAttributesError {
        let XmlParseError(message) = err;
        SetSubscriptionAttributesError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for SetSubscriptionAttributesError {
    fn from(err: CredentialsError) -> SetSubscriptionAttributesError {
        SetSubscriptionAttributesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for SetSubscriptionAttributesError {
    fn from(err: HttpDispatchError) -> SetSubscriptionAttributesError {
        SetSubscriptionAttributesError::HttpDispatch(err)
    }
}
impl From<io::Error> for SetSubscriptionAttributesError {
    fn from(err: io::Error) -> SetSubscriptionAttributesError {
        SetSubscriptionAttributesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for SetSubscriptionAttributesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SetSubscriptionAttributesError {
    fn description(&self) -> &str {
        match *self {
            SetSubscriptionAttributesError::AuthorizationError(ref cause) => cause,
            SetSubscriptionAttributesError::InternalError(ref cause) => cause,
            SetSubscriptionAttributesError::InvalidParameter(ref cause) => cause,
            SetSubscriptionAttributesError::NotFound(ref cause) => cause,
            SetSubscriptionAttributesError::Validation(ref cause) => cause,
            SetSubscriptionAttributesError::Credentials(ref err) => err.description(),
            SetSubscriptionAttributesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            SetSubscriptionAttributesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by SetTopicAttributes
#[derive(Debug, PartialEq)]
pub enum SetTopicAttributesError {
    /// <p>Indicates that the user has been denied access to the requested resource.</p>
    AuthorizationError(String),
    /// <p>Indicates an internal service error.</p>
    InternalError(String),
    /// <p>Indicates that a request parameter does not comply with the associated constraints.</p>
    InvalidParameter(String),
    /// <p>Indicates that the requested resource does not exist.</p>
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

impl SetTopicAttributesError {
    pub fn from_body(body: &str) -> SetTopicAttributesError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "AuthorizationError" => {
                    SetTopicAttributesError::AuthorizationError(String::from(parsed_error.message))
                }
                "InternalError" => {
                    SetTopicAttributesError::InternalError(String::from(parsed_error.message))
                }
                "InvalidParameter" => {
                    SetTopicAttributesError::InvalidParameter(String::from(parsed_error.message))
                }
                "NotFound" => SetTopicAttributesError::NotFound(String::from(parsed_error.message)),
                _ => SetTopicAttributesError::Unknown(String::from(body)),
            },
            Err(_) => SetTopicAttributesError::Unknown(body.to_string()),
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

impl From<XmlParseError> for SetTopicAttributesError {
    fn from(err: XmlParseError) -> SetTopicAttributesError {
        let XmlParseError(message) = err;
        SetTopicAttributesError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for SetTopicAttributesError {
    fn from(err: CredentialsError) -> SetTopicAttributesError {
        SetTopicAttributesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for SetTopicAttributesError {
    fn from(err: HttpDispatchError) -> SetTopicAttributesError {
        SetTopicAttributesError::HttpDispatch(err)
    }
}
impl From<io::Error> for SetTopicAttributesError {
    fn from(err: io::Error) -> SetTopicAttributesError {
        SetTopicAttributesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for SetTopicAttributesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SetTopicAttributesError {
    fn description(&self) -> &str {
        match *self {
            SetTopicAttributesError::AuthorizationError(ref cause) => cause,
            SetTopicAttributesError::InternalError(ref cause) => cause,
            SetTopicAttributesError::InvalidParameter(ref cause) => cause,
            SetTopicAttributesError::NotFound(ref cause) => cause,
            SetTopicAttributesError::Validation(ref cause) => cause,
            SetTopicAttributesError::Credentials(ref err) => err.description(),
            SetTopicAttributesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            SetTopicAttributesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by Subscribe
#[derive(Debug, PartialEq)]
pub enum SubscribeError {
    /// <p>Indicates that the user has been denied access to the requested resource.</p>
    AuthorizationError(String),
    /// <p>Indicates an internal service error.</p>
    InternalError(String),
    /// <p>Indicates that a request parameter does not comply with the associated constraints.</p>
    InvalidParameter(String),
    /// <p>Indicates that the requested resource does not exist.</p>
    NotFound(String),
    /// <p>Indicates that the customer already owns the maximum allowed number of subscriptions.</p>
    SubscriptionLimitExceeded(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl SubscribeError {
    pub fn from_body(body: &str) -> SubscribeError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "AuthorizationError" => {
                    SubscribeError::AuthorizationError(String::from(parsed_error.message))
                }
                "InternalError" => {
                    SubscribeError::InternalError(String::from(parsed_error.message))
                }
                "InvalidParameter" => {
                    SubscribeError::InvalidParameter(String::from(parsed_error.message))
                }
                "NotFound" => SubscribeError::NotFound(String::from(parsed_error.message)),
                "SubscriptionLimitExceeded" => {
                    SubscribeError::SubscriptionLimitExceeded(String::from(parsed_error.message))
                }
                _ => SubscribeError::Unknown(String::from(body)),
            },
            Err(_) => SubscribeError::Unknown(body.to_string()),
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

impl From<XmlParseError> for SubscribeError {
    fn from(err: XmlParseError) -> SubscribeError {
        let XmlParseError(message) = err;
        SubscribeError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for SubscribeError {
    fn from(err: CredentialsError) -> SubscribeError {
        SubscribeError::Credentials(err)
    }
}
impl From<HttpDispatchError> for SubscribeError {
    fn from(err: HttpDispatchError) -> SubscribeError {
        SubscribeError::HttpDispatch(err)
    }
}
impl From<io::Error> for SubscribeError {
    fn from(err: io::Error) -> SubscribeError {
        SubscribeError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for SubscribeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SubscribeError {
    fn description(&self) -> &str {
        match *self {
            SubscribeError::AuthorizationError(ref cause) => cause,
            SubscribeError::InternalError(ref cause) => cause,
            SubscribeError::InvalidParameter(ref cause) => cause,
            SubscribeError::NotFound(ref cause) => cause,
            SubscribeError::SubscriptionLimitExceeded(ref cause) => cause,
            SubscribeError::Validation(ref cause) => cause,
            SubscribeError::Credentials(ref err) => err.description(),
            SubscribeError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            SubscribeError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by Unsubscribe
#[derive(Debug, PartialEq)]
pub enum UnsubscribeError {
    /// <p>Indicates that the user has been denied access to the requested resource.</p>
    AuthorizationError(String),
    /// <p>Indicates an internal service error.</p>
    InternalError(String),
    /// <p>Indicates that a request parameter does not comply with the associated constraints.</p>
    InvalidParameter(String),
    /// <p>Indicates that the requested resource does not exist.</p>
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

impl UnsubscribeError {
    pub fn from_body(body: &str) -> UnsubscribeError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "AuthorizationError" => {
                    UnsubscribeError::AuthorizationError(String::from(parsed_error.message))
                }
                "InternalError" => {
                    UnsubscribeError::InternalError(String::from(parsed_error.message))
                }
                "InvalidParameter" => {
                    UnsubscribeError::InvalidParameter(String::from(parsed_error.message))
                }
                "NotFound" => UnsubscribeError::NotFound(String::from(parsed_error.message)),
                _ => UnsubscribeError::Unknown(String::from(body)),
            },
            Err(_) => UnsubscribeError::Unknown(body.to_string()),
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

impl From<XmlParseError> for UnsubscribeError {
    fn from(err: XmlParseError) -> UnsubscribeError {
        let XmlParseError(message) = err;
        UnsubscribeError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for UnsubscribeError {
    fn from(err: CredentialsError) -> UnsubscribeError {
        UnsubscribeError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UnsubscribeError {
    fn from(err: HttpDispatchError) -> UnsubscribeError {
        UnsubscribeError::HttpDispatch(err)
    }
}
impl From<io::Error> for UnsubscribeError {
    fn from(err: io::Error) -> UnsubscribeError {
        UnsubscribeError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UnsubscribeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UnsubscribeError {
    fn description(&self) -> &str {
        match *self {
            UnsubscribeError::AuthorizationError(ref cause) => cause,
            UnsubscribeError::InternalError(ref cause) => cause,
            UnsubscribeError::InvalidParameter(ref cause) => cause,
            UnsubscribeError::NotFound(ref cause) => cause,
            UnsubscribeError::Validation(ref cause) => cause,
            UnsubscribeError::Credentials(ref err) => err.description(),
            UnsubscribeError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UnsubscribeError::Unknown(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the Amazon SNS API. Amazon SNS clients implement this trait.
pub trait Sns {
    /// <p>Adds a statement to a topic's access control policy, granting access for the specified AWS accounts to the specified actions.</p>
    fn add_permission(&self, input: AddPermissionInput) -> RusotoFuture<(), AddPermissionError>;

    /// <p>Accepts a phone number and indicates whether the phone holder has opted out of receiving SMS messages from your account. You cannot send SMS messages to a number that is opted out.</p> <p>To resume sending messages, you can opt in the number by using the <code>OptInPhoneNumber</code> action.</p>
    fn check_if_phone_number_is_opted_out(
        &self,
        input: CheckIfPhoneNumberIsOptedOutInput,
    ) -> RusotoFuture<CheckIfPhoneNumberIsOptedOutResponse, CheckIfPhoneNumberIsOptedOutError>;

    /// <p>Verifies an endpoint owner's intent to receive messages by validating the token sent to the endpoint by an earlier <code>Subscribe</code> action. If the token is valid, the action creates a new subscription and returns its Amazon Resource Name (ARN). This call requires an AWS signature only when the <code>AuthenticateOnUnsubscribe</code> flag is set to "true".</p>
    fn confirm_subscription(
        &self,
        input: ConfirmSubscriptionInput,
    ) -> RusotoFuture<ConfirmSubscriptionResponse, ConfirmSubscriptionError>;

    /// <p>Creates a platform application object for one of the supported push notification services, such as APNS and GCM, to which devices and mobile apps may register. You must specify PlatformPrincipal and PlatformCredential attributes when using the <code>CreatePlatformApplication</code> action. The PlatformPrincipal is received from the notification service. For APNS/APNS_SANDBOX, PlatformPrincipal is "SSL certificate". For GCM, PlatformPrincipal is not applicable. For ADM, PlatformPrincipal is "client id". The PlatformCredential is also received from the notification service. For WNS, PlatformPrincipal is "Package Security Identifier". For MPNS, PlatformPrincipal is "TLS certificate". For Baidu, PlatformPrincipal is "API key".</p> <p>For APNS/APNS_SANDBOX, PlatformCredential is "private key". For GCM, PlatformCredential is "API key". For ADM, PlatformCredential is "client secret". For WNS, PlatformCredential is "secret key". For MPNS, PlatformCredential is "private key". For Baidu, PlatformCredential is "secret key". The PlatformApplicationArn that is returned when using <code>CreatePlatformApplication</code> is then used as an attribute for the <code>CreatePlatformEndpoint</code> action. For more information, see <a href="http://docs.aws.amazon.com/sns/latest/dg/SNSMobilePush.html">Using Amazon SNS Mobile Push Notifications</a>. For more information about obtaining the PlatformPrincipal and PlatformCredential for each of the supported push notification services, see <a href="http://docs.aws.amazon.com/sns/latest/dg/mobile-push-apns.html">Getting Started with Apple Push Notification Service</a>, <a href="http://docs.aws.amazon.com/sns/latest/dg/mobile-push-adm.html">Getting Started with Amazon Device Messaging</a>, <a href="http://docs.aws.amazon.com/sns/latest/dg/mobile-push-baidu.html">Getting Started with Baidu Cloud Push</a>, <a href="http://docs.aws.amazon.com/sns/latest/dg/mobile-push-gcm.html">Getting Started with Google Cloud Messaging for Android</a>, <a href="http://docs.aws.amazon.com/sns/latest/dg/mobile-push-mpns.html">Getting Started with MPNS</a>, or <a href="http://docs.aws.amazon.com/sns/latest/dg/mobile-push-wns.html">Getting Started with WNS</a>. </p>
    fn create_platform_application(
        &self,
        input: CreatePlatformApplicationInput,
    ) -> RusotoFuture<CreatePlatformApplicationResponse, CreatePlatformApplicationError>;

    /// <p>Creates an endpoint for a device and mobile app on one of the supported push notification services, such as GCM and APNS. <code>CreatePlatformEndpoint</code> requires the PlatformApplicationArn that is returned from <code>CreatePlatformApplication</code>. The EndpointArn that is returned when using <code>CreatePlatformEndpoint</code> can then be used by the <code>Publish</code> action to send a message to a mobile app or by the <code>Subscribe</code> action for subscription to a topic. The <code>CreatePlatformEndpoint</code> action is idempotent, so if the requester already owns an endpoint with the same device token and attributes, that endpoint's ARN is returned without creating a new endpoint. For more information, see <a href="http://docs.aws.amazon.com/sns/latest/dg/SNSMobilePush.html">Using Amazon SNS Mobile Push Notifications</a>. </p> <p>When using <code>CreatePlatformEndpoint</code> with Baidu, two attributes must be provided: ChannelId and UserId. The token field must also contain the ChannelId. For more information, see <a href="http://docs.aws.amazon.com/sns/latest/dg/SNSMobilePushBaiduEndpoint.html">Creating an Amazon SNS Endpoint for Baidu</a>. </p>
    fn create_platform_endpoint(
        &self,
        input: CreatePlatformEndpointInput,
    ) -> RusotoFuture<CreateEndpointResponse, CreatePlatformEndpointError>;

    /// <p>Creates a topic to which notifications can be published. Users can create at most 100,000 topics. For more information, see <a href="http://aws.amazon.com/sns/">http://aws.amazon.com/sns</a>. This action is idempotent, so if the requester already owns a topic with the specified name, that topic's ARN is returned without creating a new topic.</p>
    fn create_topic(
        &self,
        input: CreateTopicInput,
    ) -> RusotoFuture<CreateTopicResponse, CreateTopicError>;

    /// <p>Deletes the endpoint for a device and mobile app from Amazon SNS. This action is idempotent. For more information, see <a href="http://docs.aws.amazon.com/sns/latest/dg/SNSMobilePush.html">Using Amazon SNS Mobile Push Notifications</a>. </p> <p>When you delete an endpoint that is also subscribed to a topic, then you must also unsubscribe the endpoint from the topic.</p>
    fn delete_endpoint(&self, input: DeleteEndpointInput) -> RusotoFuture<(), DeleteEndpointError>;

    /// <p>Deletes a platform application object for one of the supported push notification services, such as APNS and GCM. For more information, see <a href="http://docs.aws.amazon.com/sns/latest/dg/SNSMobilePush.html">Using Amazon SNS Mobile Push Notifications</a>. </p>
    fn delete_platform_application(
        &self,
        input: DeletePlatformApplicationInput,
    ) -> RusotoFuture<(), DeletePlatformApplicationError>;

    /// <p>Deletes a topic and all its subscriptions. Deleting a topic might prevent some messages previously sent to the topic from being delivered to subscribers. This action is idempotent, so deleting a topic that does not exist does not result in an error.</p>
    fn delete_topic(&self, input: DeleteTopicInput) -> RusotoFuture<(), DeleteTopicError>;

    /// <p>Retrieves the endpoint attributes for a device on one of the supported push notification services, such as GCM and APNS. For more information, see <a href="http://docs.aws.amazon.com/sns/latest/dg/SNSMobilePush.html">Using Amazon SNS Mobile Push Notifications</a>. </p>
    fn get_endpoint_attributes(
        &self,
        input: GetEndpointAttributesInput,
    ) -> RusotoFuture<GetEndpointAttributesResponse, GetEndpointAttributesError>;

    /// <p>Retrieves the attributes of the platform application object for the supported push notification services, such as APNS and GCM. For more information, see <a href="http://docs.aws.amazon.com/sns/latest/dg/SNSMobilePush.html">Using Amazon SNS Mobile Push Notifications</a>. </p>
    fn get_platform_application_attributes(
        &self,
        input: GetPlatformApplicationAttributesInput,
    ) -> RusotoFuture<GetPlatformApplicationAttributesResponse, GetPlatformApplicationAttributesError>;

    /// <p>Returns the settings for sending SMS messages from your account.</p> <p>These settings are set with the <code>SetSMSAttributes</code> action.</p>
    fn get_sms_attributes(
        &self,
        input: GetSMSAttributesInput,
    ) -> RusotoFuture<GetSMSAttributesResponse, GetSMSAttributesError>;

    /// <p>Returns all of the properties of a subscription.</p>
    fn get_subscription_attributes(
        &self,
        input: GetSubscriptionAttributesInput,
    ) -> RusotoFuture<GetSubscriptionAttributesResponse, GetSubscriptionAttributesError>;

    /// <p>Returns all of the properties of a topic. Topic properties returned might differ based on the authorization of the user.</p>
    fn get_topic_attributes(
        &self,
        input: GetTopicAttributesInput,
    ) -> RusotoFuture<GetTopicAttributesResponse, GetTopicAttributesError>;

    /// <p>Lists the endpoints and endpoint attributes for devices in a supported push notification service, such as GCM and APNS. The results for <code>ListEndpointsByPlatformApplication</code> are paginated and return a limited list of endpoints, up to 100. If additional records are available after the first page results, then a NextToken string will be returned. To receive the next page, you call <code>ListEndpointsByPlatformApplication</code> again using the NextToken string received from the previous call. When there are no more records to return, NextToken will be null. For more information, see <a href="http://docs.aws.amazon.com/sns/latest/dg/SNSMobilePush.html">Using Amazon SNS Mobile Push Notifications</a>. </p>
    fn list_endpoints_by_platform_application(
        &self,
        input: ListEndpointsByPlatformApplicationInput,
    ) -> RusotoFuture<
        ListEndpointsByPlatformApplicationResponse,
        ListEndpointsByPlatformApplicationError,
    >;

    /// <p>Returns a list of phone numbers that are opted out, meaning you cannot send SMS messages to them.</p> <p>The results for <code>ListPhoneNumbersOptedOut</code> are paginated, and each page returns up to 100 phone numbers. If additional phone numbers are available after the first page of results, then a <code>NextToken</code> string will be returned. To receive the next page, you call <code>ListPhoneNumbersOptedOut</code> again using the <code>NextToken</code> string received from the previous call. When there are no more records to return, <code>NextToken</code> will be null.</p>
    fn list_phone_numbers_opted_out(
        &self,
        input: ListPhoneNumbersOptedOutInput,
    ) -> RusotoFuture<ListPhoneNumbersOptedOutResponse, ListPhoneNumbersOptedOutError>;

    /// <p>Lists the platform application objects for the supported push notification services, such as APNS and GCM. The results for <code>ListPlatformApplications</code> are paginated and return a limited list of applications, up to 100. If additional records are available after the first page results, then a NextToken string will be returned. To receive the next page, you call <code>ListPlatformApplications</code> using the NextToken string received from the previous call. When there are no more records to return, NextToken will be null. For more information, see <a href="http://docs.aws.amazon.com/sns/latest/dg/SNSMobilePush.html">Using Amazon SNS Mobile Push Notifications</a>. </p>
    fn list_platform_applications(
        &self,
        input: ListPlatformApplicationsInput,
    ) -> RusotoFuture<ListPlatformApplicationsResponse, ListPlatformApplicationsError>;

    /// <p>Returns a list of the requester's subscriptions. Each call returns a limited list of subscriptions, up to 100. If there are more subscriptions, a <code>NextToken</code> is also returned. Use the <code>NextToken</code> parameter in a new <code>ListSubscriptions</code> call to get further results.</p>
    fn list_subscriptions(
        &self,
        input: ListSubscriptionsInput,
    ) -> RusotoFuture<ListSubscriptionsResponse, ListSubscriptionsError>;

    /// <p>Returns a list of the subscriptions to a specific topic. Each call returns a limited list of subscriptions, up to 100. If there are more subscriptions, a <code>NextToken</code> is also returned. Use the <code>NextToken</code> parameter in a new <code>ListSubscriptionsByTopic</code> call to get further results.</p>
    fn list_subscriptions_by_topic(
        &self,
        input: ListSubscriptionsByTopicInput,
    ) -> RusotoFuture<ListSubscriptionsByTopicResponse, ListSubscriptionsByTopicError>;

    /// <p>Returns a list of the requester's topics. Each call returns a limited list of topics, up to 100. If there are more topics, a <code>NextToken</code> is also returned. Use the <code>NextToken</code> parameter in a new <code>ListTopics</code> call to get further results.</p>
    fn list_topics(
        &self,
        input: ListTopicsInput,
    ) -> RusotoFuture<ListTopicsResponse, ListTopicsError>;

    /// <p>Use this request to opt in a phone number that is opted out, which enables you to resume sending SMS messages to the number.</p> <p>You can opt in a phone number only once every 30 days.</p>
    fn opt_in_phone_number(
        &self,
        input: OptInPhoneNumberInput,
    ) -> RusotoFuture<OptInPhoneNumberResponse, OptInPhoneNumberError>;

    /// <p>Sends a message to all of a topic's subscribed endpoints. When a <code>messageId</code> is returned, the message has been saved and Amazon SNS will attempt to deliver it to the topic's subscribers shortly. The format of the outgoing message to each subscribed endpoint depends on the notification protocol.</p> <p>To use the <code>Publish</code> action for sending a message to a mobile endpoint, such as an app on a Kindle device or mobile phone, you must specify the EndpointArn for the TargetArn parameter. The EndpointArn is returned when making a call with the <code>CreatePlatformEndpoint</code> action. </p> <p>For more information about formatting messages, see <a href="http://docs.aws.amazon.com/sns/latest/dg/mobile-push-send-custommessage.html">Send Custom Platform-Specific Payloads in Messages to Mobile Devices</a>. </p>
    fn publish(&self, input: PublishInput) -> RusotoFuture<PublishResponse, PublishError>;

    /// <p>Removes a statement from a topic's access control policy.</p>
    fn remove_permission(
        &self,
        input: RemovePermissionInput,
    ) -> RusotoFuture<(), RemovePermissionError>;

    /// <p>Sets the attributes for an endpoint for a device on one of the supported push notification services, such as GCM and APNS. For more information, see <a href="http://docs.aws.amazon.com/sns/latest/dg/SNSMobilePush.html">Using Amazon SNS Mobile Push Notifications</a>. </p>
    fn set_endpoint_attributes(
        &self,
        input: SetEndpointAttributesInput,
    ) -> RusotoFuture<(), SetEndpointAttributesError>;

    /// <p>Sets the attributes of the platform application object for the supported push notification services, such as APNS and GCM. For more information, see <a href="http://docs.aws.amazon.com/sns/latest/dg/SNSMobilePush.html">Using Amazon SNS Mobile Push Notifications</a>. For information on configuring attributes for message delivery status, see <a href="http://docs.aws.amazon.com/sns/latest/dg/sns-msg-status.html">Using Amazon SNS Application Attributes for Message Delivery Status</a>. </p>
    fn set_platform_application_attributes(
        &self,
        input: SetPlatformApplicationAttributesInput,
    ) -> RusotoFuture<(), SetPlatformApplicationAttributesError>;

    /// <p>Use this request to set the default settings for sending SMS messages and receiving daily SMS usage reports.</p> <p>You can override some of these settings for a single message when you use the <code>Publish</code> action with the <code>MessageAttributes.entry.N</code> parameter. For more information, see <a href="http://docs.aws.amazon.com/sns/latest/dg/sms_publish-to-phone.html">Sending an SMS Message</a> in the <i>Amazon SNS Developer Guide</i>.</p>
    fn set_sms_attributes(
        &self,
        input: SetSMSAttributesInput,
    ) -> RusotoFuture<SetSMSAttributesResponse, SetSMSAttributesError>;

    /// <p>Allows a subscription owner to set an attribute of the topic to a new value.</p>
    fn set_subscription_attributes(
        &self,
        input: SetSubscriptionAttributesInput,
    ) -> RusotoFuture<(), SetSubscriptionAttributesError>;

    /// <p>Allows a topic owner to set an attribute of the topic to a new value.</p>
    fn set_topic_attributes(
        &self,
        input: SetTopicAttributesInput,
    ) -> RusotoFuture<(), SetTopicAttributesError>;

    /// <p>Prepares to subscribe an endpoint by sending the endpoint a confirmation message. To actually create a subscription, the endpoint owner must call the <code>ConfirmSubscription</code> action with the token from the confirmation message. Confirmation tokens are valid for three days.</p>
    fn subscribe(&self, input: SubscribeInput) -> RusotoFuture<SubscribeResponse, SubscribeError>;

    /// <p>Deletes a subscription. If the subscription requires authentication for deletion, only the owner of the subscription or the topic's owner can unsubscribe, and an AWS signature is required. If the <code>Unsubscribe</code> call does not require authentication and the requester is not the subscription owner, a final cancellation message is delivered to the endpoint, so that the endpoint owner can easily resubscribe to the topic if the <code>Unsubscribe</code> request was unintended.</p>
    fn unsubscribe(&self, input: UnsubscribeInput) -> RusotoFuture<(), UnsubscribeError>;
}
/// A client for the Amazon SNS API.
pub struct SnsClient<P = CredentialsProvider, D = RequestDispatcher>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    inner: ClientInner<P, D>,
    region: region::Region,
}

impl SnsClient {
    /// Creates a simple client backed by an implicit event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    ///
    /// See the `rusoto_core::reactor` module for more details.
    pub fn simple(region: region::Region) -> SnsClient {
        SnsClient::new(
            RequestDispatcher::default(),
            CredentialsProvider::default(),
            region,
        )
    }
}

impl<P, D> SnsClient<P, D>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
        SnsClient {
            inner: ClientInner::new(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl<P, D> Sns for SnsClient<P, D>
where
    P: ProvideAwsCredentials + 'static,
    D: DispatchSignedRequest + 'static,
{
    /// <p>Adds a statement to a topic's access control policy, granting access for the specified AWS accounts to the specified actions.</p>
    fn add_permission(&self, input: AddPermissionInput) -> RusotoFuture<(), AddPermissionError> {
        let mut request = SignedRequest::new("POST", "sns", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "AddPermission");
        params.put("Version", "2010-03-31");
        AddPermissionInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(AddPermissionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(future::ok(::std::mem::drop(response)))
        });

        RusotoFuture::new(future)
    }

    /// <p>Accepts a phone number and indicates whether the phone holder has opted out of receiving SMS messages from your account. You cannot send SMS messages to a number that is opted out.</p> <p>To resume sending messages, you can opt in the number by using the <code>OptInPhoneNumber</code> action.</p>
    fn check_if_phone_number_is_opted_out(
        &self,
        input: CheckIfPhoneNumberIsOptedOutInput,
    ) -> RusotoFuture<CheckIfPhoneNumberIsOptedOutResponse, CheckIfPhoneNumberIsOptedOutError> {
        let mut request = SignedRequest::new("POST", "sns", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CheckIfPhoneNumberIsOptedOut");
        params.put("Version", "2010-03-31");
        CheckIfPhoneNumberIsOptedOutInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CheckIfPhoneNumberIsOptedOutError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = CheckIfPhoneNumberIsOptedOutResponse::default();
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
                        CheckIfPhoneNumberIsOptedOutResponseDeserializer::deserialize(
                            "CheckIfPhoneNumberIsOptedOutResult",
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

    /// <p>Verifies an endpoint owner's intent to receive messages by validating the token sent to the endpoint by an earlier <code>Subscribe</code> action. If the token is valid, the action creates a new subscription and returns its Amazon Resource Name (ARN). This call requires an AWS signature only when the <code>AuthenticateOnUnsubscribe</code> flag is set to "true".</p>
    fn confirm_subscription(
        &self,
        input: ConfirmSubscriptionInput,
    ) -> RusotoFuture<ConfirmSubscriptionResponse, ConfirmSubscriptionError> {
        let mut request = SignedRequest::new("POST", "sns", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ConfirmSubscription");
        params.put("Version", "2010-03-31");
        ConfirmSubscriptionInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ConfirmSubscriptionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ConfirmSubscriptionResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(ConfirmSubscriptionResponseDeserializer::deserialize(
                        "ConfirmSubscriptionResult",
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

    /// <p>Creates a platform application object for one of the supported push notification services, such as APNS and GCM, to which devices and mobile apps may register. You must specify PlatformPrincipal and PlatformCredential attributes when using the <code>CreatePlatformApplication</code> action. The PlatformPrincipal is received from the notification service. For APNS/APNS_SANDBOX, PlatformPrincipal is "SSL certificate". For GCM, PlatformPrincipal is not applicable. For ADM, PlatformPrincipal is "client id". The PlatformCredential is also received from the notification service. For WNS, PlatformPrincipal is "Package Security Identifier". For MPNS, PlatformPrincipal is "TLS certificate". For Baidu, PlatformPrincipal is "API key".</p> <p>For APNS/APNS_SANDBOX, PlatformCredential is "private key". For GCM, PlatformCredential is "API key". For ADM, PlatformCredential is "client secret". For WNS, PlatformCredential is "secret key". For MPNS, PlatformCredential is "private key". For Baidu, PlatformCredential is "secret key". The PlatformApplicationArn that is returned when using <code>CreatePlatformApplication</code> is then used as an attribute for the <code>CreatePlatformEndpoint</code> action. For more information, see <a href="http://docs.aws.amazon.com/sns/latest/dg/SNSMobilePush.html">Using Amazon SNS Mobile Push Notifications</a>. For more information about obtaining the PlatformPrincipal and PlatformCredential for each of the supported push notification services, see <a href="http://docs.aws.amazon.com/sns/latest/dg/mobile-push-apns.html">Getting Started with Apple Push Notification Service</a>, <a href="http://docs.aws.amazon.com/sns/latest/dg/mobile-push-adm.html">Getting Started with Amazon Device Messaging</a>, <a href="http://docs.aws.amazon.com/sns/latest/dg/mobile-push-baidu.html">Getting Started with Baidu Cloud Push</a>, <a href="http://docs.aws.amazon.com/sns/latest/dg/mobile-push-gcm.html">Getting Started with Google Cloud Messaging for Android</a>, <a href="http://docs.aws.amazon.com/sns/latest/dg/mobile-push-mpns.html">Getting Started with MPNS</a>, or <a href="http://docs.aws.amazon.com/sns/latest/dg/mobile-push-wns.html">Getting Started with WNS</a>. </p>
    fn create_platform_application(
        &self,
        input: CreatePlatformApplicationInput,
    ) -> RusotoFuture<CreatePlatformApplicationResponse, CreatePlatformApplicationError> {
        let mut request = SignedRequest::new("POST", "sns", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CreatePlatformApplication");
        params.put("Version", "2010-03-31");
        CreatePlatformApplicationInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreatePlatformApplicationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = CreatePlatformApplicationResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(CreatePlatformApplicationResponseDeserializer::deserialize(
                        "CreatePlatformApplicationResult",
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

    /// <p>Creates an endpoint for a device and mobile app on one of the supported push notification services, such as GCM and APNS. <code>CreatePlatformEndpoint</code> requires the PlatformApplicationArn that is returned from <code>CreatePlatformApplication</code>. The EndpointArn that is returned when using <code>CreatePlatformEndpoint</code> can then be used by the <code>Publish</code> action to send a message to a mobile app or by the <code>Subscribe</code> action for subscription to a topic. The <code>CreatePlatformEndpoint</code> action is idempotent, so if the requester already owns an endpoint with the same device token and attributes, that endpoint's ARN is returned without creating a new endpoint. For more information, see <a href="http://docs.aws.amazon.com/sns/latest/dg/SNSMobilePush.html">Using Amazon SNS Mobile Push Notifications</a>. </p> <p>When using <code>CreatePlatformEndpoint</code> with Baidu, two attributes must be provided: ChannelId and UserId. The token field must also contain the ChannelId. For more information, see <a href="http://docs.aws.amazon.com/sns/latest/dg/SNSMobilePushBaiduEndpoint.html">Creating an Amazon SNS Endpoint for Baidu</a>. </p>
    fn create_platform_endpoint(
        &self,
        input: CreatePlatformEndpointInput,
    ) -> RusotoFuture<CreateEndpointResponse, CreatePlatformEndpointError> {
        let mut request = SignedRequest::new("POST", "sns", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CreatePlatformEndpoint");
        params.put("Version", "2010-03-31");
        CreatePlatformEndpointInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreatePlatformEndpointError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = CreateEndpointResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(CreateEndpointResponseDeserializer::deserialize(
                        "CreatePlatformEndpointResult",
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

    /// <p>Creates a topic to which notifications can be published. Users can create at most 100,000 topics. For more information, see <a href="http://aws.amazon.com/sns/">http://aws.amazon.com/sns</a>. This action is idempotent, so if the requester already owns a topic with the specified name, that topic's ARN is returned without creating a new topic.</p>
    fn create_topic(
        &self,
        input: CreateTopicInput,
    ) -> RusotoFuture<CreateTopicResponse, CreateTopicError> {
        let mut request = SignedRequest::new("POST", "sns", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CreateTopic");
        params.put("Version", "2010-03-31");
        CreateTopicInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateTopicError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = CreateTopicResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(CreateTopicResponseDeserializer::deserialize(
                        "CreateTopicResult",
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

    /// <p>Deletes the endpoint for a device and mobile app from Amazon SNS. This action is idempotent. For more information, see <a href="http://docs.aws.amazon.com/sns/latest/dg/SNSMobilePush.html">Using Amazon SNS Mobile Push Notifications</a>. </p> <p>When you delete an endpoint that is also subscribed to a topic, then you must also unsubscribe the endpoint from the topic.</p>
    fn delete_endpoint(&self, input: DeleteEndpointInput) -> RusotoFuture<(), DeleteEndpointError> {
        let mut request = SignedRequest::new("POST", "sns", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteEndpoint");
        params.put("Version", "2010-03-31");
        DeleteEndpointInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteEndpointError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(future::ok(::std::mem::drop(response)))
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes a platform application object for one of the supported push notification services, such as APNS and GCM. For more information, see <a href="http://docs.aws.amazon.com/sns/latest/dg/SNSMobilePush.html">Using Amazon SNS Mobile Push Notifications</a>. </p>
    fn delete_platform_application(
        &self,
        input: DeletePlatformApplicationInput,
    ) -> RusotoFuture<(), DeletePlatformApplicationError> {
        let mut request = SignedRequest::new("POST", "sns", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeletePlatformApplication");
        params.put("Version", "2010-03-31");
        DeletePlatformApplicationInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeletePlatformApplicationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(future::ok(::std::mem::drop(response)))
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes a topic and all its subscriptions. Deleting a topic might prevent some messages previously sent to the topic from being delivered to subscribers. This action is idempotent, so deleting a topic that does not exist does not result in an error.</p>
    fn delete_topic(&self, input: DeleteTopicInput) -> RusotoFuture<(), DeleteTopicError> {
        let mut request = SignedRequest::new("POST", "sns", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteTopic");
        params.put("Version", "2010-03-31");
        DeleteTopicInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteTopicError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(future::ok(::std::mem::drop(response)))
        });

        RusotoFuture::new(future)
    }

    /// <p>Retrieves the endpoint attributes for a device on one of the supported push notification services, such as GCM and APNS. For more information, see <a href="http://docs.aws.amazon.com/sns/latest/dg/SNSMobilePush.html">Using Amazon SNS Mobile Push Notifications</a>. </p>
    fn get_endpoint_attributes(
        &self,
        input: GetEndpointAttributesInput,
    ) -> RusotoFuture<GetEndpointAttributesResponse, GetEndpointAttributesError> {
        let mut request = SignedRequest::new("POST", "sns", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "GetEndpointAttributes");
        params.put("Version", "2010-03-31");
        GetEndpointAttributesInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetEndpointAttributesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = GetEndpointAttributesResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(GetEndpointAttributesResponseDeserializer::deserialize(
                        "GetEndpointAttributesResult",
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

    /// <p>Retrieves the attributes of the platform application object for the supported push notification services, such as APNS and GCM. For more information, see <a href="http://docs.aws.amazon.com/sns/latest/dg/SNSMobilePush.html">Using Amazon SNS Mobile Push Notifications</a>. </p>
    fn get_platform_application_attributes(
        &self,
        input: GetPlatformApplicationAttributesInput,
    ) -> RusotoFuture<GetPlatformApplicationAttributesResponse, GetPlatformApplicationAttributesError>
    {
        let mut request = SignedRequest::new("POST", "sns", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "GetPlatformApplicationAttributes");
        params.put("Version", "2010-03-31");
        GetPlatformApplicationAttributesInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetPlatformApplicationAttributesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = GetPlatformApplicationAttributesResponse::default();
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
                        GetPlatformApplicationAttributesResponseDeserializer::deserialize(
                            "GetPlatformApplicationAttributesResult",
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

    /// <p>Returns the settings for sending SMS messages from your account.</p> <p>These settings are set with the <code>SetSMSAttributes</code> action.</p>
    fn get_sms_attributes(
        &self,
        input: GetSMSAttributesInput,
    ) -> RusotoFuture<GetSMSAttributesResponse, GetSMSAttributesError> {
        let mut request = SignedRequest::new("POST", "sns", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "GetSMSAttributes");
        params.put("Version", "2010-03-31");
        GetSMSAttributesInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetSMSAttributesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = GetSMSAttributesResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(GetSMSAttributesResponseDeserializer::deserialize(
                        "GetSMSAttributesResult",
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

    /// <p>Returns all of the properties of a subscription.</p>
    fn get_subscription_attributes(
        &self,
        input: GetSubscriptionAttributesInput,
    ) -> RusotoFuture<GetSubscriptionAttributesResponse, GetSubscriptionAttributesError> {
        let mut request = SignedRequest::new("POST", "sns", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "GetSubscriptionAttributes");
        params.put("Version", "2010-03-31");
        GetSubscriptionAttributesInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetSubscriptionAttributesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = GetSubscriptionAttributesResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(GetSubscriptionAttributesResponseDeserializer::deserialize(
                        "GetSubscriptionAttributesResult",
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

    /// <p>Returns all of the properties of a topic. Topic properties returned might differ based on the authorization of the user.</p>
    fn get_topic_attributes(
        &self,
        input: GetTopicAttributesInput,
    ) -> RusotoFuture<GetTopicAttributesResponse, GetTopicAttributesError> {
        let mut request = SignedRequest::new("POST", "sns", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "GetTopicAttributes");
        params.put("Version", "2010-03-31");
        GetTopicAttributesInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetTopicAttributesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = GetTopicAttributesResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(GetTopicAttributesResponseDeserializer::deserialize(
                        "GetTopicAttributesResult",
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

    /// <p>Lists the endpoints and endpoint attributes for devices in a supported push notification service, such as GCM and APNS. The results for <code>ListEndpointsByPlatformApplication</code> are paginated and return a limited list of endpoints, up to 100. If additional records are available after the first page results, then a NextToken string will be returned. To receive the next page, you call <code>ListEndpointsByPlatformApplication</code> again using the NextToken string received from the previous call. When there are no more records to return, NextToken will be null. For more information, see <a href="http://docs.aws.amazon.com/sns/latest/dg/SNSMobilePush.html">Using Amazon SNS Mobile Push Notifications</a>. </p>
    fn list_endpoints_by_platform_application(
        &self,
        input: ListEndpointsByPlatformApplicationInput,
    ) -> RusotoFuture<
        ListEndpointsByPlatformApplicationResponse,
        ListEndpointsByPlatformApplicationError,
    > {
        let mut request = SignedRequest::new("POST", "sns", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ListEndpointsByPlatformApplication");
        params.put("Version", "2010-03-31");
        ListEndpointsByPlatformApplicationInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListEndpointsByPlatformApplicationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ListEndpointsByPlatformApplicationResponse::default();
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
                        ListEndpointsByPlatformApplicationResponseDeserializer::deserialize(
                            "ListEndpointsByPlatformApplicationResult",
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

    /// <p>Returns a list of phone numbers that are opted out, meaning you cannot send SMS messages to them.</p> <p>The results for <code>ListPhoneNumbersOptedOut</code> are paginated, and each page returns up to 100 phone numbers. If additional phone numbers are available after the first page of results, then a <code>NextToken</code> string will be returned. To receive the next page, you call <code>ListPhoneNumbersOptedOut</code> again using the <code>NextToken</code> string received from the previous call. When there are no more records to return, <code>NextToken</code> will be null.</p>
    fn list_phone_numbers_opted_out(
        &self,
        input: ListPhoneNumbersOptedOutInput,
    ) -> RusotoFuture<ListPhoneNumbersOptedOutResponse, ListPhoneNumbersOptedOutError> {
        let mut request = SignedRequest::new("POST", "sns", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ListPhoneNumbersOptedOut");
        params.put("Version", "2010-03-31");
        ListPhoneNumbersOptedOutInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListPhoneNumbersOptedOutError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ListPhoneNumbersOptedOutResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(ListPhoneNumbersOptedOutResponseDeserializer::deserialize(
                        "ListPhoneNumbersOptedOutResult",
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

    /// <p>Lists the platform application objects for the supported push notification services, such as APNS and GCM. The results for <code>ListPlatformApplications</code> are paginated and return a limited list of applications, up to 100. If additional records are available after the first page results, then a NextToken string will be returned. To receive the next page, you call <code>ListPlatformApplications</code> using the NextToken string received from the previous call. When there are no more records to return, NextToken will be null. For more information, see <a href="http://docs.aws.amazon.com/sns/latest/dg/SNSMobilePush.html">Using Amazon SNS Mobile Push Notifications</a>. </p>
    fn list_platform_applications(
        &self,
        input: ListPlatformApplicationsInput,
    ) -> RusotoFuture<ListPlatformApplicationsResponse, ListPlatformApplicationsError> {
        let mut request = SignedRequest::new("POST", "sns", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ListPlatformApplications");
        params.put("Version", "2010-03-31");
        ListPlatformApplicationsInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListPlatformApplicationsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ListPlatformApplicationsResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(ListPlatformApplicationsResponseDeserializer::deserialize(
                        "ListPlatformApplicationsResult",
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

    /// <p>Returns a list of the requester's subscriptions. Each call returns a limited list of subscriptions, up to 100. If there are more subscriptions, a <code>NextToken</code> is also returned. Use the <code>NextToken</code> parameter in a new <code>ListSubscriptions</code> call to get further results.</p>
    fn list_subscriptions(
        &self,
        input: ListSubscriptionsInput,
    ) -> RusotoFuture<ListSubscriptionsResponse, ListSubscriptionsError> {
        let mut request = SignedRequest::new("POST", "sns", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ListSubscriptions");
        params.put("Version", "2010-03-31");
        ListSubscriptionsInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListSubscriptionsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ListSubscriptionsResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(ListSubscriptionsResponseDeserializer::deserialize(
                        "ListSubscriptionsResult",
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

    /// <p>Returns a list of the subscriptions to a specific topic. Each call returns a limited list of subscriptions, up to 100. If there are more subscriptions, a <code>NextToken</code> is also returned. Use the <code>NextToken</code> parameter in a new <code>ListSubscriptionsByTopic</code> call to get further results.</p>
    fn list_subscriptions_by_topic(
        &self,
        input: ListSubscriptionsByTopicInput,
    ) -> RusotoFuture<ListSubscriptionsByTopicResponse, ListSubscriptionsByTopicError> {
        let mut request = SignedRequest::new("POST", "sns", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ListSubscriptionsByTopic");
        params.put("Version", "2010-03-31");
        ListSubscriptionsByTopicInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListSubscriptionsByTopicError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ListSubscriptionsByTopicResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(ListSubscriptionsByTopicResponseDeserializer::deserialize(
                        "ListSubscriptionsByTopicResult",
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

    /// <p>Returns a list of the requester's topics. Each call returns a limited list of topics, up to 100. If there are more topics, a <code>NextToken</code> is also returned. Use the <code>NextToken</code> parameter in a new <code>ListTopics</code> call to get further results.</p>
    fn list_topics(
        &self,
        input: ListTopicsInput,
    ) -> RusotoFuture<ListTopicsResponse, ListTopicsError> {
        let mut request = SignedRequest::new("POST", "sns", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ListTopics");
        params.put("Version", "2010-03-31");
        ListTopicsInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListTopicsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ListTopicsResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(ListTopicsResponseDeserializer::deserialize(
                        "ListTopicsResult",
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

    /// <p>Use this request to opt in a phone number that is opted out, which enables you to resume sending SMS messages to the number.</p> <p>You can opt in a phone number only once every 30 days.</p>
    fn opt_in_phone_number(
        &self,
        input: OptInPhoneNumberInput,
    ) -> RusotoFuture<OptInPhoneNumberResponse, OptInPhoneNumberError> {
        let mut request = SignedRequest::new("POST", "sns", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "OptInPhoneNumber");
        params.put("Version", "2010-03-31");
        OptInPhoneNumberInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(OptInPhoneNumberError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = OptInPhoneNumberResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(OptInPhoneNumberResponseDeserializer::deserialize(
                        "OptInPhoneNumberResult",
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

    /// <p>Sends a message to all of a topic's subscribed endpoints. When a <code>messageId</code> is returned, the message has been saved and Amazon SNS will attempt to deliver it to the topic's subscribers shortly. The format of the outgoing message to each subscribed endpoint depends on the notification protocol.</p> <p>To use the <code>Publish</code> action for sending a message to a mobile endpoint, such as an app on a Kindle device or mobile phone, you must specify the EndpointArn for the TargetArn parameter. The EndpointArn is returned when making a call with the <code>CreatePlatformEndpoint</code> action. </p> <p>For more information about formatting messages, see <a href="http://docs.aws.amazon.com/sns/latest/dg/mobile-push-send-custommessage.html">Send Custom Platform-Specific Payloads in Messages to Mobile Devices</a>. </p>
    fn publish(&self, input: PublishInput) -> RusotoFuture<PublishResponse, PublishError> {
        let mut request = SignedRequest::new("POST", "sns", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "Publish");
        params.put("Version", "2010-03-31");
        PublishInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(PublishError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = PublishResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(PublishResponseDeserializer::deserialize(
                        "PublishResult",
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

    /// <p>Removes a statement from a topic's access control policy.</p>
    fn remove_permission(
        &self,
        input: RemovePermissionInput,
    ) -> RusotoFuture<(), RemovePermissionError> {
        let mut request = SignedRequest::new("POST", "sns", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "RemovePermission");
        params.put("Version", "2010-03-31");
        RemovePermissionInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(RemovePermissionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(future::ok(::std::mem::drop(response)))
        });

        RusotoFuture::new(future)
    }

    /// <p>Sets the attributes for an endpoint for a device on one of the supported push notification services, such as GCM and APNS. For more information, see <a href="http://docs.aws.amazon.com/sns/latest/dg/SNSMobilePush.html">Using Amazon SNS Mobile Push Notifications</a>. </p>
    fn set_endpoint_attributes(
        &self,
        input: SetEndpointAttributesInput,
    ) -> RusotoFuture<(), SetEndpointAttributesError> {
        let mut request = SignedRequest::new("POST", "sns", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "SetEndpointAttributes");
        params.put("Version", "2010-03-31");
        SetEndpointAttributesInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(SetEndpointAttributesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(future::ok(::std::mem::drop(response)))
        });

        RusotoFuture::new(future)
    }

    /// <p>Sets the attributes of the platform application object for the supported push notification services, such as APNS and GCM. For more information, see <a href="http://docs.aws.amazon.com/sns/latest/dg/SNSMobilePush.html">Using Amazon SNS Mobile Push Notifications</a>. For information on configuring attributes for message delivery status, see <a href="http://docs.aws.amazon.com/sns/latest/dg/sns-msg-status.html">Using Amazon SNS Application Attributes for Message Delivery Status</a>. </p>
    fn set_platform_application_attributes(
        &self,
        input: SetPlatformApplicationAttributesInput,
    ) -> RusotoFuture<(), SetPlatformApplicationAttributesError> {
        let mut request = SignedRequest::new("POST", "sns", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "SetPlatformApplicationAttributes");
        params.put("Version", "2010-03-31");
        SetPlatformApplicationAttributesInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(SetPlatformApplicationAttributesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(future::ok(::std::mem::drop(response)))
        });

        RusotoFuture::new(future)
    }

    /// <p>Use this request to set the default settings for sending SMS messages and receiving daily SMS usage reports.</p> <p>You can override some of these settings for a single message when you use the <code>Publish</code> action with the <code>MessageAttributes.entry.N</code> parameter. For more information, see <a href="http://docs.aws.amazon.com/sns/latest/dg/sms_publish-to-phone.html">Sending an SMS Message</a> in the <i>Amazon SNS Developer Guide</i>.</p>
    fn set_sms_attributes(
        &self,
        input: SetSMSAttributesInput,
    ) -> RusotoFuture<SetSMSAttributesResponse, SetSMSAttributesError> {
        let mut request = SignedRequest::new("POST", "sns", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "SetSMSAttributes");
        params.put("Version", "2010-03-31");
        SetSMSAttributesInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(SetSMSAttributesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = SetSMSAttributesResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(SetSMSAttributesResponseDeserializer::deserialize(
                        "SetSMSAttributesResult",
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

    /// <p>Allows a subscription owner to set an attribute of the topic to a new value.</p>
    fn set_subscription_attributes(
        &self,
        input: SetSubscriptionAttributesInput,
    ) -> RusotoFuture<(), SetSubscriptionAttributesError> {
        let mut request = SignedRequest::new("POST", "sns", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "SetSubscriptionAttributes");
        params.put("Version", "2010-03-31");
        SetSubscriptionAttributesInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(SetSubscriptionAttributesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(future::ok(::std::mem::drop(response)))
        });

        RusotoFuture::new(future)
    }

    /// <p>Allows a topic owner to set an attribute of the topic to a new value.</p>
    fn set_topic_attributes(
        &self,
        input: SetTopicAttributesInput,
    ) -> RusotoFuture<(), SetTopicAttributesError> {
        let mut request = SignedRequest::new("POST", "sns", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "SetTopicAttributes");
        params.put("Version", "2010-03-31");
        SetTopicAttributesInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(SetTopicAttributesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(future::ok(::std::mem::drop(response)))
        });

        RusotoFuture::new(future)
    }

    /// <p>Prepares to subscribe an endpoint by sending the endpoint a confirmation message. To actually create a subscription, the endpoint owner must call the <code>ConfirmSubscription</code> action with the token from the confirmation message. Confirmation tokens are valid for three days.</p>
    fn subscribe(&self, input: SubscribeInput) -> RusotoFuture<SubscribeResponse, SubscribeError> {
        let mut request = SignedRequest::new("POST", "sns", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "Subscribe");
        params.put("Version", "2010-03-31");
        SubscribeInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(SubscribeError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = SubscribeResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(SubscribeResponseDeserializer::deserialize(
                        "SubscribeResult",
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

    /// <p>Deletes a subscription. If the subscription requires authentication for deletion, only the owner of the subscription or the topic's owner can unsubscribe, and an AWS signature is required. If the <code>Unsubscribe</code> call does not require authentication and the requester is not the subscription owner, a final cancellation message is delivered to the endpoint, so that the endpoint owner can easily resubscribe to the topic if the <code>Unsubscribe</code> request was unintended.</p>
    fn unsubscribe(&self, input: UnsubscribeInput) -> RusotoFuture<(), UnsubscribeError> {
        let mut request = SignedRequest::new("POST", "sns", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "Unsubscribe");
        params.put("Version", "2010-03-31");
        UnsubscribeInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UnsubscribeError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(future::ok(::std::mem::drop(response)))
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
    fn test_parse_error_sns_delete_topic() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/error",
            "sns-delete-topic.xml",
        );
        let mock = MockRequestDispatcher::with_status(400).with_body(&mock_response);
        let client = SnsClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = DeleteTopicInput::default();
        let result = client.delete_topic(request).sync();
        assert!(!result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_sns_add_permission() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "sns-add-permission.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = SnsClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = AddPermissionInput::default();
        let result = client.add_permission(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_sns_confirm_subscription() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "sns-confirm-subscription.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = SnsClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = ConfirmSubscriptionInput::default();
        let result = client.confirm_subscription(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_sns_create_topic() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "sns-create-topic.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = SnsClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = CreateTopicInput::default();
        let result = client.create_topic(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_sns_get_subscription_attributes() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "sns-get-subscription-attributes.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = SnsClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = GetSubscriptionAttributesInput::default();
        let result = client.get_subscription_attributes(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_sns_get_topic_attributes() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "sns-get-topic-attributes.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = SnsClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = GetTopicAttributesInput::default();
        let result = client.get_topic_attributes(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_sns_list_subscriptions_by_topic() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "sns-list-subscriptions-by-topic.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = SnsClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = ListSubscriptionsByTopicInput::default();
        let result = client.list_subscriptions_by_topic(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_sns_list_subscriptions() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "sns-list-subscriptions.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = SnsClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = ListSubscriptionsInput::default();
        let result = client.list_subscriptions(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_sns_list_topics() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "sns-list-topics.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = SnsClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = ListTopicsInput::default();
        let result = client.list_topics(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_sns_publish() {
        let mock_response =
            MockResponseReader::read_response("test_resources/generated/valid", "sns-publish.xml");
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = SnsClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = PublishInput::default();
        let result = client.publish(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_sns_subscribe() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "sns-subscribe.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = SnsClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = SubscribeInput::default();
        let result = client.subscribe(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }
}
