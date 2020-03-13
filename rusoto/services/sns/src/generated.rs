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

struct AccountDeserializer;
impl AccountDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AddPermissionInput {
    /// <p>The AWS account IDs of the users (principals) who will be given access to the specified actions. The users must have AWS accounts, but do not need to be signed up for this service.</p>
    pub aws_account_id: Vec<String>,
    /// <p>The action you want to allow for the specified principal(s).</p> <p>Valid values: Any Amazon SNS action name, for example <code>Publish</code>.</p>
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
        params.put(&format!("{}{}", prefix, "Label"), &obj.label);
        params.put(&format!("{}{}", prefix, "TopicArn"), &obj.topic_arn);
    }
}

struct AttributeNameDeserializer;
impl AttributeNameDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct AttributeValueDeserializer;
impl AttributeValueDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct BooleanDeserializer;
impl BooleanDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<bool, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = bool::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>The input for the <code>CheckIfPhoneNumberIsOptedOut</code> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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

        params.put(&format!("{}{}", prefix, "phoneNumber"), &obj.phone_number);
    }
}

/// <p>The response from the <code>CheckIfPhoneNumberIsOptedOut</code> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CheckIfPhoneNumberIsOptedOutResponse {
    /// <p><p>Indicates whether the phone number is opted out:</p> <ul> <li> <p> <code>true</code> – The phone number is opted out, meaning you cannot publish SMS messages to it.</p> </li> <li> <p> <code>false</code> – The phone number is opted in, meaning you can publish SMS messages to it.</p> </li> </ul></p>
    pub is_opted_out: Option<bool>,
}

struct CheckIfPhoneNumberIsOptedOutResponseDeserializer;
impl CheckIfPhoneNumberIsOptedOutResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CheckIfPhoneNumberIsOptedOutResponse, XmlParseError> {
        deserialize_elements::<_, CheckIfPhoneNumberIsOptedOutResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "isOptedOut" => {
                        obj.is_opted_out =
                            Some(BooleanDeserializer::deserialize("isOptedOut", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p>Input for ConfirmSubscription action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
                &field_value,
            );
        }
        params.put(&format!("{}{}", prefix, "Token"), &obj.token);
        params.put(&format!("{}{}", prefix, "TopicArn"), &obj.topic_arn);
    }
}

/// <p>Response for ConfirmSubscriptions action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ConfirmSubscriptionResponse {
    /// <p>The ARN of the created subscription.</p>
    pub subscription_arn: Option<String>,
}

struct ConfirmSubscriptionResponseDeserializer;
impl ConfirmSubscriptionResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ConfirmSubscriptionResponse, XmlParseError> {
        deserialize_elements::<_, ConfirmSubscriptionResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "SubscriptionArn" => {
                        obj.subscription_arn = Some(SubscriptionARNDeserializer::deserialize(
                            "SubscriptionArn",
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
/// <p>Response from CreateEndpoint action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CreateEndpointResponse {
    /// <p>EndpointArn returned from CreateEndpoint action.</p>
    pub endpoint_arn: Option<String>,
}

struct CreateEndpointResponseDeserializer;
impl CreateEndpointResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateEndpointResponse, XmlParseError> {
        deserialize_elements::<_, CreateEndpointResponse, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "EndpointArn" => {
                    obj.endpoint_arn = Some(StringDeserializer::deserialize("EndpointArn", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
/// <p>Input for CreatePlatformApplication action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreatePlatformApplicationInput {
    /// <p>For a list of attributes, see <a href="https://docs.aws.amazon.com/sns/latest/api/API_SetPlatformApplicationAttributes.html">SetPlatformApplicationAttributes</a> </p>
    pub attributes: ::std::collections::HashMap<String, String>,
    /// <p>Application names must be made up of only uppercase and lowercase ASCII letters, numbers, underscores, hyphens, and periods, and must be between 1 and 256 characters long.</p>
    pub name: String,
    /// <p>The following platforms are supported: ADM (Amazon Device Messaging), APNS (Apple Push Notification Service), APNS_SANDBOX, and FCM (Firebase Cloud Messaging).</p>
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
            &format!("{}{}.entry", prefix, "Attributes"),
            &obj.attributes,
        );
        params.put(&format!("{}{}", prefix, "Name"), &obj.name);
        params.put(&format!("{}{}", prefix, "Platform"), &obj.platform);
    }
}

/// <p>Response from CreatePlatformApplication action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CreatePlatformApplicationResponse {
    /// <p>PlatformApplicationArn is returned.</p>
    pub platform_application_arn: Option<String>,
}

struct CreatePlatformApplicationResponseDeserializer;
impl CreatePlatformApplicationResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreatePlatformApplicationResponse, XmlParseError> {
        deserialize_elements::<_, CreatePlatformApplicationResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "PlatformApplicationArn" => {
                        obj.platform_application_arn = Some(StringDeserializer::deserialize(
                            "PlatformApplicationArn",
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
/// <p>Input for CreatePlatformEndpoint action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreatePlatformEndpointInput {
    /// <p>For a list of attributes, see <a href="https://docs.aws.amazon.com/sns/latest/api/API_SetEndpointAttributes.html">SetEndpointAttributes</a>.</p>
    pub attributes: Option<::std::collections::HashMap<String, String>>,
    /// <p>Arbitrary user data to associate with the endpoint. Amazon SNS does not use this data. The data must be in UTF-8 format and less than 2KB.</p>
    pub custom_user_data: Option<String>,
    /// <p>PlatformApplicationArn returned from CreatePlatformApplication is used to create a an endpoint.</p>
    pub platform_application_arn: String,
    /// <p>Unique identifier created by the notification service for an app on a device. The specific name for Token will vary, depending on which notification service is being used. For example, when using APNS as the notification service, you need the device token. Alternatively, when using FCM or ADM, the device token equivalent is called the registration ID.</p>
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
                &format!("{}{}.entry", prefix, "Attributes"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.custom_user_data {
            params.put(&format!("{}{}", prefix, "CustomUserData"), &field_value);
        }
        params.put(
            &format!("{}{}", prefix, "PlatformApplicationArn"),
            &obj.platform_application_arn,
        );
        params.put(&format!("{}{}", prefix, "Token"), &obj.token);
    }
}

/// <p>Input for CreateTopic action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateTopicInput {
    /// <p><p>A map of attributes with their corresponding values.</p> <p>The following lists the names, descriptions, and values of the special request parameters that the <code>CreateTopic</code> action uses:</p> <ul> <li> <p> <code>DeliveryPolicy</code> – The policy that defines how Amazon SNS retries failed deliveries to HTTP/S endpoints.</p> </li> <li> <p> <code>DisplayName</code> – The display name to use for a topic with SMS subscriptions.</p> </li> <li> <p> <code>Policy</code> – The policy that defines who can access your topic. By default, only the topic owner can publish or subscribe to the topic.</p> </li> </ul> <p>The following attribute applies only to <a href="https://docs.aws.amazon.com/sns/latest/dg/sns-server-side-encryption.html">server-side-encryption</a>:</p> <ul> <li> <p> <code>KmsMasterKeyId</code> - The ID of an AWS-managed customer master key (CMK) for Amazon SNS or a custom CMK. For more information, see <a href="https://docs.aws.amazon.com/sns/latest/dg/sns-server-side-encryption.html#sse-key-terms">Key Terms</a>. For more examples, see <a href="https://docs.aws.amazon.com/kms/latest/APIReference/API_DescribeKey.html#API_DescribeKey_RequestParameters">KeyId</a> in the <i>AWS Key Management Service API Reference</i>. </p> </li> </ul></p>
    pub attributes: Option<::std::collections::HashMap<String, String>>,
    /// <p>The name of the topic you want to create.</p> <p>Constraints: Topic names must be made up of only uppercase and lowercase ASCII letters, numbers, underscores, and hyphens, and must be between 1 and 256 characters long.</p>
    pub name: String,
    /// <p><p>The list of tags to add to a new topic.</p> <note> <p>To be able to tag a topic on creation, you must have the <code>sns:CreateTopic</code> and <code>sns:TagResource</code> permissions.</p> </note></p>
    pub tags: Option<Vec<Tag>>,
}

/// Serialize `CreateTopicInput` contents to a `SignedRequest`.
struct CreateTopicInputSerializer;
impl CreateTopicInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CreateTopicInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.attributes {
            TopicAttributesMapSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Attributes"),
                field_value,
            );
        }
        params.put(&format!("{}{}", prefix, "Name"), &obj.name);
        if let Some(ref field_value) = obj.tags {
            TagListSerializer::serialize(params, &format!("{}{}", prefix, "Tags"), field_value);
        }
    }
}

/// <p>Response from CreateTopic action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CreateTopicResponse {
    /// <p>The Amazon Resource Name (ARN) assigned to the created topic.</p>
    pub topic_arn: Option<String>,
}

struct CreateTopicResponseDeserializer;
impl CreateTopicResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateTopicResponse, XmlParseError> {
        deserialize_elements::<_, CreateTopicResponse, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "TopicArn" => {
                    obj.topic_arn = Some(TopicARNDeserializer::deserialize("TopicArn", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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

        params.put(&format!("{}{}", prefix, "EndpointArn"), &obj.endpoint_arn);
    }
}

/// <p>Input for DeletePlatformApplication action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
            &obj.platform_application_arn,
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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

        params.put(&format!("{}{}", prefix, "TopicArn"), &obj.topic_arn);
    }
}

struct EndpointDeserializer;
impl EndpointDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Input for GetEndpointAttributes action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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

        params.put(&format!("{}{}", prefix, "EndpointArn"), &obj.endpoint_arn);
    }
}

/// <p>Response from GetEndpointAttributes of the EndpointArn.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct GetEndpointAttributesResponse {
    /// <p><p>Attributes include the following:</p> <ul> <li> <p> <code>CustomUserData</code> – arbitrary user data to associate with the endpoint. Amazon SNS does not use this data. The data must be in UTF-8 format and less than 2KB.</p> </li> <li> <p> <code>Enabled</code> – flag that enables/disables delivery to the endpoint. Amazon SNS will set this to false when a notification service indicates to Amazon SNS that the endpoint is invalid. Users can set it back to true, typically after updating Token.</p> </li> <li> <p> <code>Token</code> – device token, also referred to as a registration id, for an app and mobile device. This is returned from the notification service when an app and mobile device are registered with the notification service.</p> <note> <p>The device token for the iOS platform is returned in lowercase.</p> </note> </li> </ul></p>
    pub attributes: Option<::std::collections::HashMap<String, String>>,
}

struct GetEndpointAttributesResponseDeserializer;
impl GetEndpointAttributesResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetEndpointAttributesResponse, XmlParseError> {
        deserialize_elements::<_, GetEndpointAttributesResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Attributes" => {
                        obj.attributes = Some(MapStringToStringDeserializer::deserialize(
                            "Attributes",
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
/// <p>Input for GetPlatformApplicationAttributes action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
            &obj.platform_application_arn,
        );
    }
}

/// <p>Response for GetPlatformApplicationAttributes action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct GetPlatformApplicationAttributesResponse {
    /// <p><p>Attributes include the following:</p> <ul> <li> <p> <code>EventEndpointCreated</code> – Topic ARN to which EndpointCreated event notifications should be sent.</p> </li> <li> <p> <code>EventEndpointDeleted</code> – Topic ARN to which EndpointDeleted event notifications should be sent.</p> </li> <li> <p> <code>EventEndpointUpdated</code> – Topic ARN to which EndpointUpdate event notifications should be sent.</p> </li> <li> <p> <code>EventDeliveryFailure</code> – Topic ARN to which DeliveryFailure event notifications should be sent upon Direct Publish delivery failure (permanent) to one of the application&#39;s endpoints.</p> </li> </ul></p>
    pub attributes: Option<::std::collections::HashMap<String, String>>,
}

struct GetPlatformApplicationAttributesResponseDeserializer;
impl GetPlatformApplicationAttributesResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetPlatformApplicationAttributesResponse, XmlParseError> {
        deserialize_elements::<_, GetPlatformApplicationAttributesResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Attributes" => {
                        obj.attributes = Some(MapStringToStringDeserializer::deserialize(
                            "Attributes",
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
/// <p>The input for the <code>GetSMSAttributes</code> request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetSMSAttributesInput {
    /// <p>A list of the individual attribute names, such as <code>MonthlySpendLimit</code>, for which you want values.</p> <p>For all attribute names, see <a href="https://docs.aws.amazon.com/sns/latest/api/API_SetSMSAttributes.html">SetSMSAttributes</a>.</p> <p>If you don't use this parameter, Amazon SNS returns all SMS attributes.</p>
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
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct GetSMSAttributesResponse {
    /// <p>The SMS attribute names and their values.</p>
    pub attributes: Option<::std::collections::HashMap<String, String>>,
}

struct GetSMSAttributesResponseDeserializer;
impl GetSMSAttributesResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetSMSAttributesResponse, XmlParseError> {
        deserialize_elements::<_, GetSMSAttributesResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "attributes" => {
                        obj.attributes = Some(MapStringToStringDeserializer::deserialize(
                            "attributes",
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
/// <p>Input for GetSubscriptionAttributes.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
            &obj.subscription_arn,
        );
    }
}

/// <p>Response for GetSubscriptionAttributes action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct GetSubscriptionAttributesResponse {
    /// <p><p>A map of the subscription&#39;s attributes. Attributes in this map include the following:</p> <ul> <li> <p> <code>ConfirmationWasAuthenticated</code> – <code>true</code> if the subscription confirmation request was authenticated.</p> </li> <li> <p> <code>DeliveryPolicy</code> – The JSON serialization of the subscription&#39;s delivery policy.</p> </li> <li> <p> <code>EffectiveDeliveryPolicy</code> – The JSON serialization of the effective delivery policy that takes into account the topic delivery policy and account system defaults.</p> </li> <li> <p> <code>FilterPolicy</code> – The filter policy JSON that is assigned to the subscription.</p> </li> <li> <p> <code>Owner</code> – The AWS account ID of the subscription&#39;s owner.</p> </li> <li> <p> <code>PendingConfirmation</code> – <code>true</code> if the subscription hasn&#39;t been confirmed. To confirm a pending subscription, call the <code>ConfirmSubscription</code> action with a confirmation token.</p> </li> <li> <p> <code>RawMessageDelivery</code> – <code>true</code> if raw message delivery is enabled for the subscription. Raw messages are free of JSON formatting and can be sent to HTTP/S and Amazon SQS endpoints.</p> </li> <li> <p> <code>RedrivePolicy</code> – When specified, sends undeliverable messages to the specified Amazon SQS dead-letter queue. Messages that can&#39;t be delivered due to client errors (for example, when the subscribed endpoint is unreachable) or server errors (for example, when the service that powers the subscribed endpoint becomes unavailable) are held in the dead-letter queue for further analysis or reprocessing.</p> </li> <li> <p> <code>SubscriptionArn</code> – The subscription&#39;s ARN.</p> </li> <li> <p> <code>TopicArn</code> – The topic ARN that the subscription is associated with.</p> </li> </ul></p>
    pub attributes: Option<::std::collections::HashMap<String, String>>,
}

struct GetSubscriptionAttributesResponseDeserializer;
impl GetSubscriptionAttributesResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetSubscriptionAttributesResponse, XmlParseError> {
        deserialize_elements::<_, GetSubscriptionAttributesResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Attributes" => {
                        obj.attributes = Some(SubscriptionAttributesMapDeserializer::deserialize(
                            "Attributes",
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
/// <p>Input for GetTopicAttributes action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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

        params.put(&format!("{}{}", prefix, "TopicArn"), &obj.topic_arn);
    }
}

/// <p>Response for GetTopicAttributes action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct GetTopicAttributesResponse {
    /// <p><p>A map of the topic&#39;s attributes. Attributes in this map include the following:</p> <ul> <li> <p> <code>DeliveryPolicy</code> – The JSON serialization of the topic&#39;s delivery policy.</p> </li> <li> <p> <code>DisplayName</code> – The human-readable name used in the <code>From</code> field for notifications to <code>email</code> and <code>email-json</code> endpoints.</p> </li> <li> <p> <code>Owner</code> – The AWS account ID of the topic&#39;s owner.</p> </li> <li> <p> <code>Policy</code> – The JSON serialization of the topic&#39;s access control policy.</p> </li> <li> <p> <code>SubscriptionsConfirmed</code> – The number of confirmed subscriptions for the topic.</p> </li> <li> <p> <code>SubscriptionsDeleted</code> – The number of deleted subscriptions for the topic.</p> </li> <li> <p> <code>SubscriptionsPending</code> – The number of subscriptions pending confirmation for the topic.</p> </li> <li> <p> <code>TopicArn</code> – The topic&#39;s ARN.</p> </li> <li> <p> <code>EffectiveDeliveryPolicy</code> – Yhe JSON serialization of the effective delivery policy, taking system defaults into account.</p> </li> </ul> <p>The following attribute applies only to <a href="https://docs.aws.amazon.com/sns/latest/dg/sns-server-side-encryption.html">server-side-encryption</a>:</p> <ul> <li> <p> <code>KmsMasterKeyId</code> - The ID of an AWS-managed customer master key (CMK) for Amazon SNS or a custom CMK. For more information, see <a href="https://docs.aws.amazon.com/sns/latest/dg/sns-server-side-encryption.html#sse-key-terms">Key Terms</a>. For more examples, see <a href="https://docs.aws.amazon.com/kms/latest/APIReference/API_DescribeKey.html#API_DescribeKey_RequestParameters">KeyId</a> in the <i>AWS Key Management Service API Reference</i>.</p> </li> </ul></p>
    pub attributes: Option<::std::collections::HashMap<String, String>>,
}

struct GetTopicAttributesResponseDeserializer;
impl GetTopicAttributesResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetTopicAttributesResponse, XmlParseError> {
        deserialize_elements::<_, GetTopicAttributesResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Attributes" => {
                        obj.attributes = Some(TopicAttributesMapDeserializer::deserialize(
                            "Attributes",
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
/// <p>Input for ListEndpointsByPlatformApplication action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
            params.put(&format!("{}{}", prefix, "NextToken"), &field_value);
        }
        params.put(
            &format!("{}{}", prefix, "PlatformApplicationArn"),
            &obj.platform_application_arn,
        );
    }
}

/// <p>Response for ListEndpointsByPlatformApplication action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ListEndpointsByPlatformApplicationResponse {
    /// <p>Endpoints returned for ListEndpointsByPlatformApplication action.</p>
    pub endpoints: Option<Vec<String>>,
    /// <p>NextToken string is returned when calling ListEndpointsByPlatformApplication action if additional records are available after the first page results.</p>
    pub next_token: Option<String>,
}

struct ListEndpointsByPlatformApplicationResponseDeserializer;
impl ListEndpointsByPlatformApplicationResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListEndpointsByPlatformApplicationResponse, XmlParseError> {
        deserialize_elements::<_, ListEndpointsByPlatformApplicationResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Endpoints" => {
                        obj.endpoints.get_or_insert(vec![]).extend(
                            ListOfEndpointsDeserializer::deserialize("Endpoints", stack)?,
                        );
                    }
                    "NextToken" => {
                        obj.next_token = Some(StringDeserializer::deserialize("NextToken", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
struct ListOfEndpointsDeserializer;
impl ListOfEndpointsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(EndpointDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
struct ListOfPlatformApplicationsDeserializer;
impl ListOfPlatformApplicationsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<PlatformApplication>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(PlatformApplicationDeserializer::deserialize(
                    "member", stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
/// <p>The input for the <code>ListPhoneNumbersOptedOut</code> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
            params.put(&format!("{}{}", prefix, "nextToken"), &field_value);
        }
    }
}

/// <p>The response from the <code>ListPhoneNumbersOptedOut</code> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ListPhoneNumbersOptedOutResponse {
    /// <p>A <code>NextToken</code> string is returned when you call the <code>ListPhoneNumbersOptedOut</code> action if additional records are available after the first page of results.</p>
    pub next_token: Option<String>,
    /// <p>A list of phone numbers that are opted out of receiving SMS messages. The list is paginated, and each page can contain up to 100 phone numbers.</p>
    pub phone_numbers: Option<Vec<String>>,
}

struct ListPhoneNumbersOptedOutResponseDeserializer;
impl ListPhoneNumbersOptedOutResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListPhoneNumbersOptedOutResponse, XmlParseError> {
        deserialize_elements::<_, ListPhoneNumbersOptedOutResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "nextToken" => {
                        obj.next_token = Some(StringDeserializer::deserialize("nextToken", stack)?);
                    }
                    "phoneNumbers" => {
                        obj.phone_numbers.get_or_insert(vec![]).extend(
                            PhoneNumberListDeserializer::deserialize("phoneNumbers", stack)?,
                        );
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p>Input for ListPlatformApplications action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
            params.put(&format!("{}{}", prefix, "NextToken"), &field_value);
        }
    }
}

/// <p>Response for ListPlatformApplications action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ListPlatformApplicationsResponse {
    /// <p>NextToken string is returned when calling ListPlatformApplications action if additional records are available after the first page results.</p>
    pub next_token: Option<String>,
    /// <p>Platform applications returned when calling ListPlatformApplications action.</p>
    pub platform_applications: Option<Vec<PlatformApplication>>,
}

struct ListPlatformApplicationsResponseDeserializer;
impl ListPlatformApplicationsResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListPlatformApplicationsResponse, XmlParseError> {
        deserialize_elements::<_, ListPlatformApplicationsResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "NextToken" => {
                        obj.next_token = Some(StringDeserializer::deserialize("NextToken", stack)?);
                    }
                    "PlatformApplications" => {
                        obj.platform_applications.get_or_insert(vec![]).extend(
                            ListOfPlatformApplicationsDeserializer::deserialize(
                                "PlatformApplications",
                                stack,
                            )?,
                        );
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
            params.put(&format!("{}{}", prefix, "NextToken"), &field_value);
        }
        params.put(&format!("{}{}", prefix, "TopicArn"), &obj.topic_arn);
    }
}

/// <p>Response for ListSubscriptionsByTopic action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ListSubscriptionsByTopicResponse {
    /// <p>Token to pass along to the next <code>ListSubscriptionsByTopic</code> request. This element is returned if there are more subscriptions to retrieve.</p>
    pub next_token: Option<String>,
    /// <p>A list of subscriptions.</p>
    pub subscriptions: Option<Vec<Subscription>>,
}

struct ListSubscriptionsByTopicResponseDeserializer;
impl ListSubscriptionsByTopicResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListSubscriptionsByTopicResponse, XmlParseError> {
        deserialize_elements::<_, ListSubscriptionsByTopicResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "NextToken" => {
                        obj.next_token =
                            Some(NextTokenDeserializer::deserialize("NextToken", stack)?);
                    }
                    "Subscriptions" => {
                        obj.subscriptions.get_or_insert(vec![]).extend(
                            SubscriptionsListDeserializer::deserialize("Subscriptions", stack)?,
                        );
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p>Input for ListSubscriptions action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
            params.put(&format!("{}{}", prefix, "NextToken"), &field_value);
        }
    }
}

/// <p>Response for ListSubscriptions action</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ListSubscriptionsResponse {
    /// <p>Token to pass along to the next <code>ListSubscriptions</code> request. This element is returned if there are more subscriptions to retrieve.</p>
    pub next_token: Option<String>,
    /// <p>A list of subscriptions.</p>
    pub subscriptions: Option<Vec<Subscription>>,
}

struct ListSubscriptionsResponseDeserializer;
impl ListSubscriptionsResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListSubscriptionsResponse, XmlParseError> {
        deserialize_elements::<_, ListSubscriptionsResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "NextToken" => {
                        obj.next_token =
                            Some(NextTokenDeserializer::deserialize("NextToken", stack)?);
                    }
                    "Subscriptions" => {
                        obj.subscriptions.get_or_insert(vec![]).extend(
                            SubscriptionsListDeserializer::deserialize("Subscriptions", stack)?,
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
pub struct ListTagsForResourceRequest {
    /// <p>The ARN of the topic for which to list tags.</p>
    pub resource_arn: String,
}

/// Serialize `ListTagsForResourceRequest` contents to a `SignedRequest`.
struct ListTagsForResourceRequestSerializer;
impl ListTagsForResourceRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ListTagsForResourceRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "ResourceArn"), &obj.resource_arn);
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ListTagsForResourceResponse {
    /// <p>The tags associated with the specified topic.</p>
    pub tags: Option<Vec<Tag>>,
}

struct ListTagsForResourceResponseDeserializer;
impl ListTagsForResourceResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListTagsForResourceResponse, XmlParseError> {
        deserialize_elements::<_, ListTagsForResourceResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Tags" => {
                        obj.tags
                            .get_or_insert(vec![])
                            .extend(TagListDeserializer::deserialize("Tags", stack)?);
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
            params.put(&format!("{}{}", prefix, "NextToken"), &field_value);
        }
    }
}

/// <p>Response for ListTopics action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ListTopicsResponse {
    /// <p>Token to pass along to the next <code>ListTopics</code> request. This element is returned if there are additional topics to retrieve.</p>
    pub next_token: Option<String>,
    /// <p>A list of topic ARNs.</p>
    pub topics: Option<Vec<Topic>>,
}

struct ListTopicsResponseDeserializer;
impl ListTopicsResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListTopicsResponse, XmlParseError> {
        deserialize_elements::<_, ListTopicsResponse, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "NextToken" => {
                    obj.next_token = Some(NextTokenDeserializer::deserialize("NextToken", stack)?);
                }
                "Topics" => {
                    obj.topics
                        .get_or_insert(vec![])
                        .extend(TopicsListDeserializer::deserialize("Topics", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct MapStringToStringDeserializer;
impl MapStringToStringDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<::std::collections::HashMap<String, String>, XmlParseError> {
        start_element(tag_name, stack)?;

        let mut obj = ::std::collections::HashMap::new();

        while peek_at_name(stack)? == "entry" {
            start_element("entry", stack)?;
            let key = StringDeserializer::deserialize("key", stack)?;
            let value = StringDeserializer::deserialize("value", stack)?;
            obj.insert(key, value);
            end_element("entry", stack)?;
        }

        end_element(tag_name, stack)?;
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
            params.put(&format!("{}.{}", prefix, "value"), &value);
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
            let prefix = format!("{}.entry.{}", name, index + 1);
            params.put(&format!("{}.{}", prefix, "Name"), &key);
            MessageAttributeValueSerializer::serialize(
                params,
                &format!("{}.{}", prefix, "Value"),
                value,
            );
        }
    }
}

/// <p>The user-specified message attribute value. For string data types, the value attribute has the same restrictions on the content as the message body. For more information, see <a href="https://docs.aws.amazon.com/sns/latest/api/API_Publish.html">Publish</a>.</p> <p>Name, type, and value must not be empty or null. In addition, the message body should not be empty or null. All parts of the message attribute, including name, type, and value, are included in the message size restriction, which is currently 256 KB (262,144 bytes). For more information, see <a href="https://docs.aws.amazon.com/sns/latest/dg/SNSMessageAttributes.html">Using Amazon SNS Message Attributes</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct MessageAttributeValue {
    /// <p>Binary type attributes can store any binary data, for example, compressed data, encrypted data, or images.</p>
    pub binary_value: Option<bytes::Bytes>,
    /// <p>Amazon SNS supports the following logical data types: String, String.Array, Number, and Binary. For more information, see <a href="https://docs.aws.amazon.com/sns/latest/dg/SNSMessageAttributes.html#SNSMessageAttributes.DataTypes">Message Attribute Data Types</a>.</p>
    pub data_type: String,
    /// <p>Strings are Unicode with UTF8 binary encoding. For a list of code values, see <a href="https://en.wikipedia.org/wiki/ASCII#ASCII_printable_characters">ASCII Printable Characters</a>.</p>
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
                ::std::str::from_utf8(&field_value).unwrap(),
            );
        }
        params.put(&format!("{}{}", prefix, "DataType"), &obj.data_type);
        if let Some(ref field_value) = obj.string_value {
            params.put(&format!("{}{}", prefix, "StringValue"), &field_value);
        }
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
/// <p>Input for the OptInPhoneNumber action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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

        params.put(&format!("{}{}", prefix, "phoneNumber"), &obj.phone_number);
    }
}

/// <p>The response for the OptInPhoneNumber action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct OptInPhoneNumberResponse {}

struct OptInPhoneNumberResponseDeserializer;
impl OptInPhoneNumberResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<OptInPhoneNumberResponse, XmlParseError> {
        start_element(tag_name, stack)?;

        let obj = OptInPhoneNumberResponse::default();

        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct PhoneNumberDeserializer;
impl PhoneNumberDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct PhoneNumberListDeserializer;
impl PhoneNumberListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(PhoneNumberDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
/// <p>Platform application object.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct PlatformApplication {
    /// <p>Attributes for platform application object.</p>
    pub attributes: Option<::std::collections::HashMap<String, String>>,
    /// <p>PlatformApplicationArn for platform application object.</p>
    pub platform_application_arn: Option<String>,
}

struct PlatformApplicationDeserializer;
impl PlatformApplicationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PlatformApplication, XmlParseError> {
        deserialize_elements::<_, PlatformApplication, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Attributes" => {
                    obj.attributes = Some(MapStringToStringDeserializer::deserialize(
                        "Attributes",
                        stack,
                    )?);
                }
                "PlatformApplicationArn" => {
                    obj.platform_application_arn = Some(StringDeserializer::deserialize(
                        "PlatformApplicationArn",
                        stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct ProtocolDeserializer;
impl ProtocolDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Input for Publish action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PublishInput {
    /// <p><p>The message you want to send.</p> <p>If you are publishing to a topic and you want to send the same message to all transport protocols, include the text of the message as a String value. If you want to send different messages for each transport protocol, set the value of the <code>MessageStructure</code> parameter to <code>json</code> and use a JSON object for the <code>Message</code> parameter. </p> <p/> <p>Constraints:</p> <ul> <li> <p>With the exception of SMS, messages must be UTF-8 encoded strings and at most 256 KB in size (262,144 bytes, not 262,144 characters).</p> </li> <li> <p>For SMS, each message can contain up to 140 characters. This character limit depends on the encoding schema. For example, an SMS message can contain 160 GSM characters, 140 ASCII characters, or 70 UCS-2 characters.</p> <p>If you publish a message that exceeds this size limit, Amazon SNS sends the message as multiple messages, each fitting within the size limit. Messages aren&#39;t truncated mid-word but are cut off at whole-word boundaries.</p> <p>The total size limit for a single SMS <code>Publish</code> action is 1,600 characters.</p> </li> </ul> <p>JSON-specific constraints:</p> <ul> <li> <p>Keys in the JSON object that correspond to supported transport protocols must have simple JSON string values.</p> </li> <li> <p>The values will be parsed (unescaped) before they are used in outgoing messages.</p> </li> <li> <p>Outbound notifications are JSON encoded (meaning that the characters will be reescaped for sending).</p> </li> <li> <p>Values have a minimum length of 0 (the empty string, &quot;&quot;, is allowed).</p> </li> <li> <p>Values have a maximum length bounded by the overall message size (so, including multiple protocols may limit message sizes).</p> </li> <li> <p>Non-string values will cause the key to be ignored.</p> </li> <li> <p>Keys that do not correspond to supported transport protocols are ignored.</p> </li> <li> <p>Duplicate keys are not allowed.</p> </li> <li> <p>Failure to parse or validate any key or value in the message will cause the <code>Publish</code> call to return an error (no partial delivery).</p> </li> </ul></p>
    pub message: String,
    /// <p>Message attributes for Publish action.</p>
    pub message_attributes: Option<::std::collections::HashMap<String, MessageAttributeValue>>,
    /// <p>Set <code>MessageStructure</code> to <code>json</code> if you want to send a different message for each protocol. For example, using one publish action, you can send a short message to your SMS subscribers and a longer message to your email subscribers. If you set <code>MessageStructure</code> to <code>json</code>, the value of the <code>Message</code> parameter must: </p> <ul> <li> <p>be a syntactically valid JSON object; and</p> </li> <li> <p>contain at least a top-level JSON key of "default" with a value that is a string.</p> </li> </ul> <p>You can define other top-level keys that define the message you want to send to a specific transport protocol (e.g., "http").</p> <p>Valid value: <code>json</code> </p>
    pub message_structure: Option<String>,
    /// <p>The phone number to which you want to deliver an SMS message. Use E.164 format.</p> <p>If you don't specify a value for the <code>PhoneNumber</code> parameter, you must specify a value for the <code>TargetArn</code> or <code>TopicArn</code> parameters.</p>
    pub phone_number: Option<String>,
    /// <p>Optional parameter to be used as the "Subject" line when the message is delivered to email endpoints. This field will also be included, if present, in the standard JSON messages delivered to other endpoints.</p> <p>Constraints: Subjects must be ASCII text that begins with a letter, number, or punctuation mark; must not include line breaks or control characters; and must be less than 100 characters long.</p>
    pub subject: Option<String>,
    /// <p>If you don't specify a value for the <code>TargetArn</code> parameter, you must specify a value for the <code>PhoneNumber</code> or <code>TopicArn</code> parameters.</p>
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

        params.put(&format!("{}{}", prefix, "Message"), &obj.message);
        if let Some(ref field_value) = obj.message_attributes {
            MessageAttributeMapSerializer::serialize(
                params,
                &format!("{}{}", prefix, "MessageAttributes"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.message_structure {
            params.put(&format!("{}{}", prefix, "MessageStructure"), &field_value);
        }
        if let Some(ref field_value) = obj.phone_number {
            params.put(&format!("{}{}", prefix, "PhoneNumber"), &field_value);
        }
        if let Some(ref field_value) = obj.subject {
            params.put(&format!("{}{}", prefix, "Subject"), &field_value);
        }
        if let Some(ref field_value) = obj.target_arn {
            params.put(&format!("{}{}", prefix, "TargetArn"), &field_value);
        }
        if let Some(ref field_value) = obj.topic_arn {
            params.put(&format!("{}{}", prefix, "TopicArn"), &field_value);
        }
    }
}

/// <p>Response for Publish action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct PublishResponse {
    /// <p>Unique identifier assigned to the published message.</p> <p>Length Constraint: Maximum 100 characters</p>
    pub message_id: Option<String>,
}

struct PublishResponseDeserializer;
impl PublishResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PublishResponse, XmlParseError> {
        deserialize_elements::<_, PublishResponse, _>(tag_name, stack, |name, stack, obj| {
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
/// <p>Input for RemovePermission action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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

        params.put(&format!("{}{}", prefix, "Label"), &obj.label);
        params.put(&format!("{}{}", prefix, "TopicArn"), &obj.topic_arn);
    }
}

/// <p>Input for SetEndpointAttributes action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SetEndpointAttributesInput {
    /// <p><p>A map of the endpoint attributes. Attributes in this map include the following:</p> <ul> <li> <p> <code>CustomUserData</code> – arbitrary user data to associate with the endpoint. Amazon SNS does not use this data. The data must be in UTF-8 format and less than 2KB.</p> </li> <li> <p> <code>Enabled</code> – flag that enables/disables delivery to the endpoint. Amazon SNS will set this to false when a notification service indicates to Amazon SNS that the endpoint is invalid. Users can set it back to true, typically after updating Token.</p> </li> <li> <p> <code>Token</code> – device token, also referred to as a registration id, for an app and mobile device. This is returned from the notification service when an app and mobile device are registered with the notification service.</p> </li> </ul></p>
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
            &format!("{}{}.entry", prefix, "Attributes"),
            &obj.attributes,
        );
        params.put(&format!("{}{}", prefix, "EndpointArn"), &obj.endpoint_arn);
    }
}

/// <p>Input for SetPlatformApplicationAttributes action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SetPlatformApplicationAttributesInput {
    /// <p><p>A map of the platform application attributes. Attributes in this map include the following:</p> <ul> <li> <p> <code>PlatformCredential</code> – The credential received from the notification service. For APNS/APNS<em>SANDBOX, PlatformCredential is private key. For FCM, PlatformCredential is &quot;API key&quot;. For ADM, PlatformCredential is &quot;client secret&quot;.</p> </li> <li> <p> <code>PlatformPrincipal</code> – The principal received from the notification service. For APNS/APNS</em>SANDBOX, PlatformPrincipal is SSL certificate. For FCM, PlatformPrincipal is not applicable. For ADM, PlatformPrincipal is &quot;client id&quot;.</p> </li> <li> <p> <code>EventEndpointCreated</code> – Topic ARN to which EndpointCreated event notifications should be sent.</p> </li> <li> <p> <code>EventEndpointDeleted</code> – Topic ARN to which EndpointDeleted event notifications should be sent.</p> </li> <li> <p> <code>EventEndpointUpdated</code> – Topic ARN to which EndpointUpdate event notifications should be sent.</p> </li> <li> <p> <code>EventDeliveryFailure</code> – Topic ARN to which DeliveryFailure event notifications should be sent upon Direct Publish delivery failure (permanent) to one of the application&#39;s endpoints.</p> </li> <li> <p> <code>SuccessFeedbackRoleArn</code> – IAM role ARN used to give Amazon SNS write access to use CloudWatch Logs on your behalf.</p> </li> <li> <p> <code>FailureFeedbackRoleArn</code> – IAM role ARN used to give Amazon SNS write access to use CloudWatch Logs on your behalf.</p> </li> <li> <p> <code>SuccessFeedbackSampleRate</code> – Sample rate percentage (0-100) of successfully delivered messages.</p> </li> </ul></p>
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
            &format!("{}{}.entry", prefix, "Attributes"),
            &obj.attributes,
        );
        params.put(
            &format!("{}{}", prefix, "PlatformApplicationArn"),
            &obj.platform_application_arn,
        );
    }
}

/// <p>The input for the SetSMSAttributes action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SetSMSAttributesInput {
    /// <p>The default settings for sending SMS messages from your account. You can set values for the following attribute names:</p> <p> <code>MonthlySpendLimit</code> – The maximum amount in USD that you are willing to spend each month to send SMS messages. When Amazon SNS determines that sending an SMS message would incur a cost that exceeds this limit, it stops sending SMS messages within minutes.</p> <important> <p>Amazon SNS stops sending SMS messages within minutes of the limit being crossed. During that interval, if you continue to send SMS messages, you will incur costs that exceed your limit.</p> </important> <p>By default, the spend limit is set to the maximum allowed by Amazon SNS. If you want to raise the limit, submit an <a href="https://console.aws.amazon.com/support/home#/case/create?issueType=service-limit-increase&amp;limitType=service-code-sns">SNS Limit Increase case</a>. For <b>New limit value</b>, enter your desired monthly spend limit. In the <b>Use Case Description</b> field, explain that you are requesting an SMS monthly spend limit increase.</p> <p> <code>DeliveryStatusIAMRole</code> – The ARN of the IAM role that allows Amazon SNS to write logs about SMS deliveries in CloudWatch Logs. For each SMS message that you send, Amazon SNS writes a log that includes the message price, the success or failure status, the reason for failure (if the message failed), the message dwell time, and other information.</p> <p> <code>DeliveryStatusSuccessSamplingRate</code> – The percentage of successful SMS deliveries for which Amazon SNS will write logs in CloudWatch Logs. The value can be an integer from 0 - 100. For example, to write logs only for failed deliveries, set this value to <code>0</code>. To write logs for 10% of your successful deliveries, set it to <code>10</code>.</p> <p> <code>DefaultSenderID</code> – A string, such as your business brand, that is displayed as the sender on the receiving device. Support for sender IDs varies by country. The sender ID can be 1 - 11 alphanumeric characters, and it must contain at least one letter.</p> <p> <code>DefaultSMSType</code> – The type of SMS message that you will send by default. You can assign the following values:</p> <ul> <li> <p> <code>Promotional</code> – (Default) Noncritical messages, such as marketing messages. Amazon SNS optimizes the message delivery to incur the lowest cost.</p> </li> <li> <p> <code>Transactional</code> – Critical messages that support customer transactions, such as one-time passcodes for multi-factor authentication. Amazon SNS optimizes the message delivery to achieve the highest reliability.</p> </li> </ul> <p> <code>UsageReportS3Bucket</code> – The name of the Amazon S3 bucket to receive daily SMS usage reports from Amazon SNS. Each day, Amazon SNS will deliver a usage report as a CSV file to the bucket. The report includes the following information for each SMS message that was successfully delivered by your account:</p> <ul> <li> <p>Time that the message was published (in UTC)</p> </li> <li> <p>Message ID</p> </li> <li> <p>Destination phone number</p> </li> <li> <p>Message type</p> </li> <li> <p>Delivery status</p> </li> <li> <p>Message price (in USD)</p> </li> <li> <p>Part number (a message is split into multiple parts if it is too long for a single message)</p> </li> <li> <p>Total number of parts</p> </li> </ul> <p>To receive the report, the bucket must have a policy that allows the Amazon SNS service principle to perform the <code>s3:PutObject</code> and <code>s3:GetBucketLocation</code> actions.</p> <p>For an example bucket policy and usage report, see <a href="https://docs.aws.amazon.com/sns/latest/dg/sms_stats.html">Monitoring SMS Activity</a> in the <i>Amazon SNS Developer Guide</i>.</p>
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
            &format!("{}{}.entry", prefix, "attributes"),
            &obj.attributes,
        );
    }
}

/// <p>The response for the SetSMSAttributes action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct SetSMSAttributesResponse {}

struct SetSMSAttributesResponseDeserializer;
impl SetSMSAttributesResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<SetSMSAttributesResponse, XmlParseError> {
        start_element(tag_name, stack)?;

        let obj = SetSMSAttributesResponse::default();

        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Input for SetSubscriptionAttributes action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SetSubscriptionAttributesInput {
    /// <p><p>A map of attributes with their corresponding values.</p> <p>The following lists the names, descriptions, and values of the special request parameters that the <code>SetTopicAttributes</code> action uses:</p> <ul> <li> <p> <code>DeliveryPolicy</code> – The policy that defines how Amazon SNS retries failed deliveries to HTTP/S endpoints.</p> </li> <li> <p> <code>FilterPolicy</code> – The simple JSON object that lets your subscriber receive only a subset of messages, rather than receiving every message published to the topic.</p> </li> <li> <p> <code>RawMessageDelivery</code> – When set to <code>true</code>, enables raw message delivery to Amazon SQS or HTTP/S endpoints. This eliminates the need for the endpoints to process JSON formatting, which is otherwise created for Amazon SNS metadata.</p> </li> <li> <p> <code>RedrivePolicy</code> – When specified, sends undeliverable messages to the specified Amazon SQS dead-letter queue. Messages that can&#39;t be delivered due to client errors (for example, when the subscribed endpoint is unreachable) or server errors (for example, when the service that powers the subscribed endpoint becomes unavailable) are held in the dead-letter queue for further analysis or reprocessing.</p> </li> </ul></p>
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
            &obj.attribute_name,
        );
        if let Some(ref field_value) = obj.attribute_value {
            params.put(&format!("{}{}", prefix, "AttributeValue"), &field_value);
        }
        params.put(
            &format!("{}{}", prefix, "SubscriptionArn"),
            &obj.subscription_arn,
        );
    }
}

/// <p>Input for SetTopicAttributes action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SetTopicAttributesInput {
    /// <p><p>A map of attributes with their corresponding values.</p> <p>The following lists the names, descriptions, and values of the special request parameters that the <code>SetTopicAttributes</code> action uses:</p> <ul> <li> <p> <code>DeliveryPolicy</code> – The policy that defines how Amazon SNS retries failed deliveries to HTTP/S endpoints.</p> </li> <li> <p> <code>DisplayName</code> – The display name to use for a topic with SMS subscriptions.</p> </li> <li> <p> <code>Policy</code> – The policy that defines who can access your topic. By default, only the topic owner can publish or subscribe to the topic.</p> </li> </ul> <p>The following attribute applies only to <a href="https://docs.aws.amazon.com/sns/latest/dg/sns-server-side-encryption.html">server-side-encryption</a>:</p> <ul> <li> <p> <code>KmsMasterKeyId</code> - The ID of an AWS-managed customer master key (CMK) for Amazon SNS or a custom CMK. For more information, see <a href="https://docs.aws.amazon.com/sns/latest/dg/sns-server-side-encryption.html#sse-key-terms">Key Terms</a>. For more examples, see <a href="https://docs.aws.amazon.com/kms/latest/APIReference/API_DescribeKey.html#API_DescribeKey_RequestParameters">KeyId</a> in the <i>AWS Key Management Service API Reference</i>. </p> </li> </ul></p>
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
            &obj.attribute_name,
        );
        if let Some(ref field_value) = obj.attribute_value {
            params.put(&format!("{}{}", prefix, "AttributeValue"), &field_value);
        }
        params.put(&format!("{}{}", prefix, "TopicArn"), &obj.topic_arn);
    }
}

struct StringDeserializer;
impl StringDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Input for Subscribe action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SubscribeInput {
    /// <p><p>A map of attributes with their corresponding values.</p> <p>The following lists the names, descriptions, and values of the special request parameters that the <code>SetTopicAttributes</code> action uses:</p> <ul> <li> <p> <code>DeliveryPolicy</code> – The policy that defines how Amazon SNS retries failed deliveries to HTTP/S endpoints.</p> </li> <li> <p> <code>FilterPolicy</code> – The simple JSON object that lets your subscriber receive only a subset of messages, rather than receiving every message published to the topic.</p> </li> <li> <p> <code>RawMessageDelivery</code> – When set to <code>true</code>, enables raw message delivery to Amazon SQS or HTTP/S endpoints. This eliminates the need for the endpoints to process JSON formatting, which is otherwise created for Amazon SNS metadata.</p> </li> <li> <p> <code>RedrivePolicy</code> – When specified, sends undeliverable messages to the specified Amazon SQS dead-letter queue. Messages that can&#39;t be delivered due to client errors (for example, when the subscribed endpoint is unreachable) or server errors (for example, when the service that powers the subscribed endpoint becomes unavailable) are held in the dead-letter queue for further analysis or reprocessing.</p> </li> </ul></p>
    pub attributes: Option<::std::collections::HashMap<String, String>>,
    /// <p><p>The endpoint that you want to receive notifications. Endpoints vary by protocol:</p> <ul> <li> <p>For the <code>http</code> protocol, the endpoint is an URL beginning with <code>http://</code> </p> </li> <li> <p>For the <code>https</code> protocol, the endpoint is a URL beginning with <code>https://</code> </p> </li> <li> <p>For the <code>email</code> protocol, the endpoint is an email address</p> </li> <li> <p>For the <code>email-json</code> protocol, the endpoint is an email address</p> </li> <li> <p>For the <code>sms</code> protocol, the endpoint is a phone number of an SMS-enabled device</p> </li> <li> <p>For the <code>sqs</code> protocol, the endpoint is the ARN of an Amazon SQS queue</p> </li> <li> <p>For the <code>application</code> protocol, the endpoint is the EndpointArn of a mobile app and device.</p> </li> <li> <p>For the <code>lambda</code> protocol, the endpoint is the ARN of an Amazon Lambda function.</p> </li> </ul></p>
    pub endpoint: Option<String>,
    /// <p><p>The protocol you want to use. Supported protocols include:</p> <ul> <li> <p> <code>http</code> – delivery of JSON-encoded message via HTTP POST</p> </li> <li> <p> <code>https</code> – delivery of JSON-encoded message via HTTPS POST</p> </li> <li> <p> <code>email</code> – delivery of message via SMTP</p> </li> <li> <p> <code>email-json</code> – delivery of JSON-encoded message via SMTP</p> </li> <li> <p> <code>sms</code> – delivery of message via SMS</p> </li> <li> <p> <code>sqs</code> – delivery of JSON-encoded message to an Amazon SQS queue</p> </li> <li> <p> <code>application</code> – delivery of JSON-encoded message to an EndpointArn for a mobile app and device.</p> </li> <li> <p> <code>lambda</code> – delivery of JSON-encoded message to an Amazon Lambda function.</p> </li> </ul></p>
    pub protocol: String,
    /// <p>Sets whether the response from the <code>Subscribe</code> request includes the subscription ARN, even if the subscription is not yet confirmed.</p> <ul> <li> <p>If you have the subscription ARN returned, the response includes the ARN in all cases, even if the subscription is not yet confirmed.</p> </li> <li> <p>If you don't have the subscription ARN returned, in addition to the ARN for confirmed subscriptions, the response also includes the <code>pending subscription</code> ARN value for subscriptions that aren't yet confirmed. A subscription becomes confirmed when the subscriber calls the <code>ConfirmSubscription</code> action with a confirmation token.</p> </li> </ul> <p>If you set this parameter to <code>true</code>, .</p> <p>The default value is <code>false</code>.</p>
    pub return_subscription_arn: Option<bool>,
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

        if let Some(ref field_value) = obj.attributes {
            SubscriptionAttributesMapSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Attributes"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.endpoint {
            params.put(&format!("{}{}", prefix, "Endpoint"), &field_value);
        }
        params.put(&format!("{}{}", prefix, "Protocol"), &obj.protocol);
        if let Some(ref field_value) = obj.return_subscription_arn {
            params.put(
                &format!("{}{}", prefix, "ReturnSubscriptionArn"),
                &field_value,
            );
        }
        params.put(&format!("{}{}", prefix, "TopicArn"), &obj.topic_arn);
    }
}

/// <p>Response for Subscribe action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct SubscribeResponse {
    /// <p>The ARN of the subscription if it is confirmed, or the string "pending confirmation" if the subscription requires confirmation. However, if the API request parameter <code>ReturnSubscriptionArn</code> is true, then the value is always the subscription ARN, even if the subscription requires confirmation.</p>
    pub subscription_arn: Option<String>,
}

struct SubscribeResponseDeserializer;
impl SubscribeResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<SubscribeResponse, XmlParseError> {
        deserialize_elements::<_, SubscribeResponse, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "SubscriptionArn" => {
                    obj.subscription_arn = Some(SubscriptionARNDeserializer::deserialize(
                        "SubscriptionArn",
                        stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
/// <p>A wrapper type for the attributes of an Amazon SNS subscription.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
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
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Subscription, XmlParseError> {
        deserialize_elements::<_, Subscription, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Endpoint" => {
                    obj.endpoint = Some(EndpointDeserializer::deserialize("Endpoint", stack)?);
                }
                "Owner" => {
                    obj.owner = Some(AccountDeserializer::deserialize("Owner", stack)?);
                }
                "Protocol" => {
                    obj.protocol = Some(ProtocolDeserializer::deserialize("Protocol", stack)?);
                }
                "SubscriptionArn" => {
                    obj.subscription_arn = Some(SubscriptionARNDeserializer::deserialize(
                        "SubscriptionArn",
                        stack,
                    )?);
                }
                "TopicArn" => {
                    obj.topic_arn = Some(TopicARNDeserializer::deserialize("TopicArn", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct SubscriptionARNDeserializer;
impl SubscriptionARNDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct SubscriptionAttributesMapDeserializer;
impl SubscriptionAttributesMapDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<::std::collections::HashMap<String, String>, XmlParseError> {
        start_element(tag_name, stack)?;

        let mut obj = ::std::collections::HashMap::new();

        while peek_at_name(stack)? == "entry" {
            start_element("entry", stack)?;
            let key = AttributeNameDeserializer::deserialize("key", stack)?;
            let value = AttributeValueDeserializer::deserialize("value", stack)?;
            obj.insert(key, value);
            end_element("entry", stack)?;
        }

        end_element(tag_name, stack)?;
        Ok(obj)
    }
}

/// Serialize `SubscriptionAttributesMap` contents to a `SignedRequest`.
struct SubscriptionAttributesMapSerializer;
impl SubscriptionAttributesMapSerializer {
    fn serialize(
        params: &mut Params,
        name: &str,
        obj: &::std::collections::HashMap<String, String>,
    ) {
        for (index, (key, value)) in obj.iter().enumerate() {
            let prefix = format!("{}.entry.{}", name, index + 1);
            params.put(&format!("{}.{}", prefix, "key"), &key);
            params.put(&format!("{}.{}", prefix, "value"), &value);
        }
    }
}

struct SubscriptionsListDeserializer;
impl SubscriptionsListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Subscription>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(SubscriptionDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
/// <p>The list of tags to be added to the specified topic.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Tag {
    /// <p>The required key portion of the tag.</p>
    pub key: String,
    /// <p>The optional value portion of the tag.</p>
    pub value: String,
}

struct TagDeserializer;
impl TagDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Tag, XmlParseError> {
        deserialize_elements::<_, Tag, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Key" => {
                    obj.key = TagKeyDeserializer::deserialize("Key", stack)?;
                }
                "Value" => {
                    obj.value = TagValueDeserializer::deserialize("Value", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

/// Serialize `Tag` contents to a `SignedRequest`.
struct TagSerializer;
impl TagSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Tag) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "Key"), &obj.key);
        params.put(&format!("{}{}", prefix, "Value"), &obj.value);
    }
}

struct TagKeyDeserializer;
impl TagKeyDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

/// Serialize `TagKeyList` contents to a `SignedRequest`.
struct TagKeyListSerializer;
impl TagKeyListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

struct TagListDeserializer;
impl TagListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Tag>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(TagDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}

/// Serialize `TagList` contents to a `SignedRequest`.
struct TagListSerializer;
impl TagListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<Tag>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            TagSerializer::serialize(params, &key, obj);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceRequest {
    /// <p>The ARN of the topic to which to add tags.</p>
    pub resource_arn: String,
    /// <p>The tags to be added to the specified topic. A tag consists of a required key and an optional value.</p>
    pub tags: Vec<Tag>,
}

/// Serialize `TagResourceRequest` contents to a `SignedRequest`.
struct TagResourceRequestSerializer;
impl TagResourceRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &TagResourceRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "ResourceArn"), &obj.resource_arn);
        TagListSerializer::serialize(params, &format!("{}{}", prefix, "Tags"), &obj.tags);
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct TagResourceResponse {}

struct TagResourceResponseDeserializer;
impl TagResourceResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<TagResourceResponse, XmlParseError> {
        start_element(tag_name, stack)?;

        let obj = TagResourceResponse::default();

        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct TagValueDeserializer;
impl TagValueDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>A wrapper type for the topic's Amazon Resource Name (ARN). To retrieve a topic's attributes, use <code>GetTopicAttributes</code>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct Topic {
    /// <p>The topic's ARN.</p>
    pub topic_arn: Option<String>,
}

struct TopicDeserializer;
impl TopicDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Topic, XmlParseError> {
        deserialize_elements::<_, Topic, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "TopicArn" => {
                    obj.topic_arn = Some(TopicARNDeserializer::deserialize("TopicArn", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct TopicARNDeserializer;
impl TopicARNDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct TopicAttributesMapDeserializer;
impl TopicAttributesMapDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<::std::collections::HashMap<String, String>, XmlParseError> {
        start_element(tag_name, stack)?;

        let mut obj = ::std::collections::HashMap::new();

        while peek_at_name(stack)? == "entry" {
            start_element("entry", stack)?;
            let key = AttributeNameDeserializer::deserialize("key", stack)?;
            let value = AttributeValueDeserializer::deserialize("value", stack)?;
            obj.insert(key, value);
            end_element("entry", stack)?;
        }

        end_element(tag_name, stack)?;
        Ok(obj)
    }
}

/// Serialize `TopicAttributesMap` contents to a `SignedRequest`.
struct TopicAttributesMapSerializer;
impl TopicAttributesMapSerializer {
    fn serialize(
        params: &mut Params,
        name: &str,
        obj: &::std::collections::HashMap<String, String>,
    ) {
        for (index, (key, value)) in obj.iter().enumerate() {
            let prefix = format!("{}.entry.{}", name, index + 1);
            params.put(&format!("{}.{}", prefix, "key"), &key);
            params.put(&format!("{}.{}", prefix, "value"), &value);
        }
    }
}

struct TopicsListDeserializer;
impl TopicsListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Topic>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(TopicDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
/// <p>Input for Unsubscribe action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
            &obj.subscription_arn,
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceRequest {
    /// <p>The ARN of the topic from which to remove tags.</p>
    pub resource_arn: String,
    /// <p>The list of tag keys to remove from the specified topic.</p>
    pub tag_keys: Vec<String>,
}

/// Serialize `UntagResourceRequest` contents to a `SignedRequest`.
struct UntagResourceRequestSerializer;
impl UntagResourceRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &UntagResourceRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "ResourceArn"), &obj.resource_arn);
        TagKeyListSerializer::serialize(params, &format!("{}{}", prefix, "TagKeys"), &obj.tag_keys);
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct UntagResourceResponse {}

struct UntagResourceResponseDeserializer;
impl UntagResourceResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<UntagResourceResponse, XmlParseError> {
        start_element(tag_name, stack)?;

        let obj = UntagResourceResponse::default();

        end_element(tag_name, stack)?;

        Ok(obj)
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
}

impl AddPermissionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AddPermissionError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AuthorizationError" => {
                        return RusotoError::Service(AddPermissionError::AuthorizationError(
                            parsed_error.message,
                        ))
                    }
                    "InternalError" => {
                        return RusotoError::Service(AddPermissionError::InternalError(
                            parsed_error.message,
                        ))
                    }
                    "InvalidParameter" => {
                        return RusotoError::Service(AddPermissionError::InvalidParameter(
                            parsed_error.message,
                        ))
                    }
                    "NotFound" => {
                        return RusotoError::Service(AddPermissionError::NotFound(
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
impl fmt::Display for AddPermissionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AddPermissionError::AuthorizationError(ref cause) => write!(f, "{}", cause),
            AddPermissionError::InternalError(ref cause) => write!(f, "{}", cause),
            AddPermissionError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            AddPermissionError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AddPermissionError {}
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
}

impl CheckIfPhoneNumberIsOptedOutError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CheckIfPhoneNumberIsOptedOutError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AuthorizationError" => {
                        return RusotoError::Service(
                            CheckIfPhoneNumberIsOptedOutError::AuthorizationError(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InternalError" => {
                        return RusotoError::Service(
                            CheckIfPhoneNumberIsOptedOutError::InternalError(parsed_error.message),
                        )
                    }
                    "InvalidParameter" => {
                        return RusotoError::Service(
                            CheckIfPhoneNumberIsOptedOutError::InvalidParameter(
                                parsed_error.message,
                            ),
                        )
                    }
                    "Throttled" => {
                        return RusotoError::Service(CheckIfPhoneNumberIsOptedOutError::Throttled(
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
impl fmt::Display for CheckIfPhoneNumberIsOptedOutError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CheckIfPhoneNumberIsOptedOutError::AuthorizationError(ref cause) => {
                write!(f, "{}", cause)
            }
            CheckIfPhoneNumberIsOptedOutError::InternalError(ref cause) => write!(f, "{}", cause),
            CheckIfPhoneNumberIsOptedOutError::InvalidParameter(ref cause) => {
                write!(f, "{}", cause)
            }
            CheckIfPhoneNumberIsOptedOutError::Throttled(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CheckIfPhoneNumberIsOptedOutError {}
/// Errors returned by ConfirmSubscription
#[derive(Debug, PartialEq)]
pub enum ConfirmSubscriptionError {
    /// <p>Indicates that the user has been denied access to the requested resource.</p>
    AuthorizationError(String),
    /// <p>Indicates that the number of filter polices in your AWS account exceeds the limit. To add more filter polices, submit an SNS Limit Increase case in the AWS Support Center.</p>
    FilterPolicyLimitExceeded(String),
    /// <p>Indicates an internal service error.</p>
    InternalError(String),
    /// <p>Indicates that a request parameter does not comply with the associated constraints.</p>
    InvalidParameter(String),
    /// <p>Indicates that the requested resource does not exist.</p>
    NotFound(String),
    /// <p>Indicates that the customer already owns the maximum allowed number of subscriptions.</p>
    SubscriptionLimitExceeded(String),
}

impl ConfirmSubscriptionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ConfirmSubscriptionError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AuthorizationError" => {
                        return RusotoError::Service(ConfirmSubscriptionError::AuthorizationError(
                            parsed_error.message,
                        ))
                    }
                    "FilterPolicyLimitExceeded" => {
                        return RusotoError::Service(
                            ConfirmSubscriptionError::FilterPolicyLimitExceeded(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InternalError" => {
                        return RusotoError::Service(ConfirmSubscriptionError::InternalError(
                            parsed_error.message,
                        ))
                    }
                    "InvalidParameter" => {
                        return RusotoError::Service(ConfirmSubscriptionError::InvalidParameter(
                            parsed_error.message,
                        ))
                    }
                    "NotFound" => {
                        return RusotoError::Service(ConfirmSubscriptionError::NotFound(
                            parsed_error.message,
                        ))
                    }
                    "SubscriptionLimitExceeded" => {
                        return RusotoError::Service(
                            ConfirmSubscriptionError::SubscriptionLimitExceeded(
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
impl fmt::Display for ConfirmSubscriptionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ConfirmSubscriptionError::AuthorizationError(ref cause) => write!(f, "{}", cause),
            ConfirmSubscriptionError::FilterPolicyLimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            ConfirmSubscriptionError::InternalError(ref cause) => write!(f, "{}", cause),
            ConfirmSubscriptionError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ConfirmSubscriptionError::NotFound(ref cause) => write!(f, "{}", cause),
            ConfirmSubscriptionError::SubscriptionLimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ConfirmSubscriptionError {}
/// Errors returned by CreatePlatformApplication
#[derive(Debug, PartialEq)]
pub enum CreatePlatformApplicationError {
    /// <p>Indicates that the user has been denied access to the requested resource.</p>
    AuthorizationError(String),
    /// <p>Indicates an internal service error.</p>
    InternalError(String),
    /// <p>Indicates that a request parameter does not comply with the associated constraints.</p>
    InvalidParameter(String),
}

impl CreatePlatformApplicationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreatePlatformApplicationError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AuthorizationError" => {
                        return RusotoError::Service(
                            CreatePlatformApplicationError::AuthorizationError(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InternalError" => {
                        return RusotoError::Service(CreatePlatformApplicationError::InternalError(
                            parsed_error.message,
                        ))
                    }
                    "InvalidParameter" => {
                        return RusotoError::Service(
                            CreatePlatformApplicationError::InvalidParameter(parsed_error.message),
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
impl fmt::Display for CreatePlatformApplicationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreatePlatformApplicationError::AuthorizationError(ref cause) => write!(f, "{}", cause),
            CreatePlatformApplicationError::InternalError(ref cause) => write!(f, "{}", cause),
            CreatePlatformApplicationError::InvalidParameter(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreatePlatformApplicationError {}
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
}

impl CreatePlatformEndpointError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreatePlatformEndpointError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AuthorizationError" => {
                        return RusotoError::Service(
                            CreatePlatformEndpointError::AuthorizationError(parsed_error.message),
                        )
                    }
                    "InternalError" => {
                        return RusotoError::Service(CreatePlatformEndpointError::InternalError(
                            parsed_error.message,
                        ))
                    }
                    "InvalidParameter" => {
                        return RusotoError::Service(CreatePlatformEndpointError::InvalidParameter(
                            parsed_error.message,
                        ))
                    }
                    "NotFound" => {
                        return RusotoError::Service(CreatePlatformEndpointError::NotFound(
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
impl fmt::Display for CreatePlatformEndpointError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreatePlatformEndpointError::AuthorizationError(ref cause) => write!(f, "{}", cause),
            CreatePlatformEndpointError::InternalError(ref cause) => write!(f, "{}", cause),
            CreatePlatformEndpointError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            CreatePlatformEndpointError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreatePlatformEndpointError {}
/// Errors returned by CreateTopic
#[derive(Debug, PartialEq)]
pub enum CreateTopicError {
    /// <p>Indicates that the user has been denied access to the requested resource.</p>
    AuthorizationError(String),
    /// <p>Can't perform multiple operations on a tag simultaneously. Perform the operations sequentially.</p>
    ConcurrentAccess(String),
    /// <p>Indicates an internal service error.</p>
    InternalError(String),
    /// <p>Indicates that a request parameter does not comply with the associated constraints.</p>
    InvalidParameter(String),
    /// <p>The credential signature isn't valid. You must use an HTTPS endpoint and sign your request using Signature Version 4.</p>
    InvalidSecurity(String),
    /// <p>A tag has been added to a resource with the same ARN as a deleted resource. Wait a short while and then retry the operation.</p>
    StaleTag(String),
    /// <p>Can't add more than 50 tags to a topic.</p>
    TagLimitExceeded(String),
    /// <p>The request doesn't comply with the IAM tag policy. Correct your request and then retry it.</p>
    TagPolicy(String),
    /// <p>Indicates that the customer already owns the maximum allowed number of topics.</p>
    TopicLimitExceeded(String),
}

impl CreateTopicError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateTopicError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AuthorizationError" => {
                        return RusotoError::Service(CreateTopicError::AuthorizationError(
                            parsed_error.message,
                        ))
                    }
                    "ConcurrentAccess" => {
                        return RusotoError::Service(CreateTopicError::ConcurrentAccess(
                            parsed_error.message,
                        ))
                    }
                    "InternalError" => {
                        return RusotoError::Service(CreateTopicError::InternalError(
                            parsed_error.message,
                        ))
                    }
                    "InvalidParameter" => {
                        return RusotoError::Service(CreateTopicError::InvalidParameter(
                            parsed_error.message,
                        ))
                    }
                    "InvalidSecurity" => {
                        return RusotoError::Service(CreateTopicError::InvalidSecurity(
                            parsed_error.message,
                        ))
                    }
                    "StaleTag" => {
                        return RusotoError::Service(CreateTopicError::StaleTag(
                            parsed_error.message,
                        ))
                    }
                    "TagLimitExceeded" => {
                        return RusotoError::Service(CreateTopicError::TagLimitExceeded(
                            parsed_error.message,
                        ))
                    }
                    "TagPolicy" => {
                        return RusotoError::Service(CreateTopicError::TagPolicy(
                            parsed_error.message,
                        ))
                    }
                    "TopicLimitExceeded" => {
                        return RusotoError::Service(CreateTopicError::TopicLimitExceeded(
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
impl fmt::Display for CreateTopicError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateTopicError::AuthorizationError(ref cause) => write!(f, "{}", cause),
            CreateTopicError::ConcurrentAccess(ref cause) => write!(f, "{}", cause),
            CreateTopicError::InternalError(ref cause) => write!(f, "{}", cause),
            CreateTopicError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            CreateTopicError::InvalidSecurity(ref cause) => write!(f, "{}", cause),
            CreateTopicError::StaleTag(ref cause) => write!(f, "{}", cause),
            CreateTopicError::TagLimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateTopicError::TagPolicy(ref cause) => write!(f, "{}", cause),
            CreateTopicError::TopicLimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateTopicError {}
/// Errors returned by DeleteEndpoint
#[derive(Debug, PartialEq)]
pub enum DeleteEndpointError {
    /// <p>Indicates that the user has been denied access to the requested resource.</p>
    AuthorizationError(String),
    /// <p>Indicates an internal service error.</p>
    InternalError(String),
    /// <p>Indicates that a request parameter does not comply with the associated constraints.</p>
    InvalidParameter(String),
}

impl DeleteEndpointError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteEndpointError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AuthorizationError" => {
                        return RusotoError::Service(DeleteEndpointError::AuthorizationError(
                            parsed_error.message,
                        ))
                    }
                    "InternalError" => {
                        return RusotoError::Service(DeleteEndpointError::InternalError(
                            parsed_error.message,
                        ))
                    }
                    "InvalidParameter" => {
                        return RusotoError::Service(DeleteEndpointError::InvalidParameter(
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
impl fmt::Display for DeleteEndpointError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteEndpointError::AuthorizationError(ref cause) => write!(f, "{}", cause),
            DeleteEndpointError::InternalError(ref cause) => write!(f, "{}", cause),
            DeleteEndpointError::InvalidParameter(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteEndpointError {}
/// Errors returned by DeletePlatformApplication
#[derive(Debug, PartialEq)]
pub enum DeletePlatformApplicationError {
    /// <p>Indicates that the user has been denied access to the requested resource.</p>
    AuthorizationError(String),
    /// <p>Indicates an internal service error.</p>
    InternalError(String),
    /// <p>Indicates that a request parameter does not comply with the associated constraints.</p>
    InvalidParameter(String),
}

impl DeletePlatformApplicationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeletePlatformApplicationError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AuthorizationError" => {
                        return RusotoError::Service(
                            DeletePlatformApplicationError::AuthorizationError(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InternalError" => {
                        return RusotoError::Service(DeletePlatformApplicationError::InternalError(
                            parsed_error.message,
                        ))
                    }
                    "InvalidParameter" => {
                        return RusotoError::Service(
                            DeletePlatformApplicationError::InvalidParameter(parsed_error.message),
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
impl fmt::Display for DeletePlatformApplicationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeletePlatformApplicationError::AuthorizationError(ref cause) => write!(f, "{}", cause),
            DeletePlatformApplicationError::InternalError(ref cause) => write!(f, "{}", cause),
            DeletePlatformApplicationError::InvalidParameter(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeletePlatformApplicationError {}
/// Errors returned by DeleteTopic
#[derive(Debug, PartialEq)]
pub enum DeleteTopicError {
    /// <p>Indicates that the user has been denied access to the requested resource.</p>
    AuthorizationError(String),
    /// <p>Can't perform multiple operations on a tag simultaneously. Perform the operations sequentially.</p>
    ConcurrentAccess(String),
    /// <p>Indicates an internal service error.</p>
    InternalError(String),
    /// <p>Indicates that a request parameter does not comply with the associated constraints.</p>
    InvalidParameter(String),
    /// <p>Indicates that the requested resource does not exist.</p>
    NotFound(String),
    /// <p>A tag has been added to a resource with the same ARN as a deleted resource. Wait a short while and then retry the operation.</p>
    StaleTag(String),
    /// <p>The request doesn't comply with the IAM tag policy. Correct your request and then retry it.</p>
    TagPolicy(String),
}

impl DeleteTopicError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteTopicError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AuthorizationError" => {
                        return RusotoError::Service(DeleteTopicError::AuthorizationError(
                            parsed_error.message,
                        ))
                    }
                    "ConcurrentAccess" => {
                        return RusotoError::Service(DeleteTopicError::ConcurrentAccess(
                            parsed_error.message,
                        ))
                    }
                    "InternalError" => {
                        return RusotoError::Service(DeleteTopicError::InternalError(
                            parsed_error.message,
                        ))
                    }
                    "InvalidParameter" => {
                        return RusotoError::Service(DeleteTopicError::InvalidParameter(
                            parsed_error.message,
                        ))
                    }
                    "NotFound" => {
                        return RusotoError::Service(DeleteTopicError::NotFound(
                            parsed_error.message,
                        ))
                    }
                    "StaleTag" => {
                        return RusotoError::Service(DeleteTopicError::StaleTag(
                            parsed_error.message,
                        ))
                    }
                    "TagPolicy" => {
                        return RusotoError::Service(DeleteTopicError::TagPolicy(
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
impl fmt::Display for DeleteTopicError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteTopicError::AuthorizationError(ref cause) => write!(f, "{}", cause),
            DeleteTopicError::ConcurrentAccess(ref cause) => write!(f, "{}", cause),
            DeleteTopicError::InternalError(ref cause) => write!(f, "{}", cause),
            DeleteTopicError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DeleteTopicError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteTopicError::StaleTag(ref cause) => write!(f, "{}", cause),
            DeleteTopicError::TagPolicy(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteTopicError {}
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
}

impl GetEndpointAttributesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetEndpointAttributesError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AuthorizationError" => {
                        return RusotoError::Service(
                            GetEndpointAttributesError::AuthorizationError(parsed_error.message),
                        )
                    }
                    "InternalError" => {
                        return RusotoError::Service(GetEndpointAttributesError::InternalError(
                            parsed_error.message,
                        ))
                    }
                    "InvalidParameter" => {
                        return RusotoError::Service(GetEndpointAttributesError::InvalidParameter(
                            parsed_error.message,
                        ))
                    }
                    "NotFound" => {
                        return RusotoError::Service(GetEndpointAttributesError::NotFound(
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
impl fmt::Display for GetEndpointAttributesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetEndpointAttributesError::AuthorizationError(ref cause) => write!(f, "{}", cause),
            GetEndpointAttributesError::InternalError(ref cause) => write!(f, "{}", cause),
            GetEndpointAttributesError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            GetEndpointAttributesError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetEndpointAttributesError {}
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
}

impl GetPlatformApplicationAttributesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetPlatformApplicationAttributesError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AuthorizationError" => {
                        return RusotoError::Service(
                            GetPlatformApplicationAttributesError::AuthorizationError(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InternalError" => {
                        return RusotoError::Service(
                            GetPlatformApplicationAttributesError::InternalError(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidParameter" => {
                        return RusotoError::Service(
                            GetPlatformApplicationAttributesError::InvalidParameter(
                                parsed_error.message,
                            ),
                        )
                    }
                    "NotFound" => {
                        return RusotoError::Service(
                            GetPlatformApplicationAttributesError::NotFound(parsed_error.message),
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
impl fmt::Display for GetPlatformApplicationAttributesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetPlatformApplicationAttributesError::AuthorizationError(ref cause) => {
                write!(f, "{}", cause)
            }
            GetPlatformApplicationAttributesError::InternalError(ref cause) => {
                write!(f, "{}", cause)
            }
            GetPlatformApplicationAttributesError::InvalidParameter(ref cause) => {
                write!(f, "{}", cause)
            }
            GetPlatformApplicationAttributesError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetPlatformApplicationAttributesError {}
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
}

impl GetSMSAttributesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetSMSAttributesError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AuthorizationError" => {
                        return RusotoError::Service(GetSMSAttributesError::AuthorizationError(
                            parsed_error.message,
                        ))
                    }
                    "InternalError" => {
                        return RusotoError::Service(GetSMSAttributesError::InternalError(
                            parsed_error.message,
                        ))
                    }
                    "InvalidParameter" => {
                        return RusotoError::Service(GetSMSAttributesError::InvalidParameter(
                            parsed_error.message,
                        ))
                    }
                    "Throttled" => {
                        return RusotoError::Service(GetSMSAttributesError::Throttled(
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
impl fmt::Display for GetSMSAttributesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetSMSAttributesError::AuthorizationError(ref cause) => write!(f, "{}", cause),
            GetSMSAttributesError::InternalError(ref cause) => write!(f, "{}", cause),
            GetSMSAttributesError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            GetSMSAttributesError::Throttled(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetSMSAttributesError {}
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
}

impl GetSubscriptionAttributesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetSubscriptionAttributesError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AuthorizationError" => {
                        return RusotoError::Service(
                            GetSubscriptionAttributesError::AuthorizationError(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InternalError" => {
                        return RusotoError::Service(GetSubscriptionAttributesError::InternalError(
                            parsed_error.message,
                        ))
                    }
                    "InvalidParameter" => {
                        return RusotoError::Service(
                            GetSubscriptionAttributesError::InvalidParameter(parsed_error.message),
                        )
                    }
                    "NotFound" => {
                        return RusotoError::Service(GetSubscriptionAttributesError::NotFound(
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
impl fmt::Display for GetSubscriptionAttributesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetSubscriptionAttributesError::AuthorizationError(ref cause) => write!(f, "{}", cause),
            GetSubscriptionAttributesError::InternalError(ref cause) => write!(f, "{}", cause),
            GetSubscriptionAttributesError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            GetSubscriptionAttributesError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetSubscriptionAttributesError {}
/// Errors returned by GetTopicAttributes
#[derive(Debug, PartialEq)]
pub enum GetTopicAttributesError {
    /// <p>Indicates that the user has been denied access to the requested resource.</p>
    AuthorizationError(String),
    /// <p>Indicates an internal service error.</p>
    InternalError(String),
    /// <p>Indicates that a request parameter does not comply with the associated constraints.</p>
    InvalidParameter(String),
    /// <p>The credential signature isn't valid. You must use an HTTPS endpoint and sign your request using Signature Version 4.</p>
    InvalidSecurity(String),
    /// <p>Indicates that the requested resource does not exist.</p>
    NotFound(String),
}

impl GetTopicAttributesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetTopicAttributesError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AuthorizationError" => {
                        return RusotoError::Service(GetTopicAttributesError::AuthorizationError(
                            parsed_error.message,
                        ))
                    }
                    "InternalError" => {
                        return RusotoError::Service(GetTopicAttributesError::InternalError(
                            parsed_error.message,
                        ))
                    }
                    "InvalidParameter" => {
                        return RusotoError::Service(GetTopicAttributesError::InvalidParameter(
                            parsed_error.message,
                        ))
                    }
                    "InvalidSecurity" => {
                        return RusotoError::Service(GetTopicAttributesError::InvalidSecurity(
                            parsed_error.message,
                        ))
                    }
                    "NotFound" => {
                        return RusotoError::Service(GetTopicAttributesError::NotFound(
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
impl fmt::Display for GetTopicAttributesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetTopicAttributesError::AuthorizationError(ref cause) => write!(f, "{}", cause),
            GetTopicAttributesError::InternalError(ref cause) => write!(f, "{}", cause),
            GetTopicAttributesError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            GetTopicAttributesError::InvalidSecurity(ref cause) => write!(f, "{}", cause),
            GetTopicAttributesError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetTopicAttributesError {}
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
}

impl ListEndpointsByPlatformApplicationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListEndpointsByPlatformApplicationError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AuthorizationError" => {
                        return RusotoError::Service(
                            ListEndpointsByPlatformApplicationError::AuthorizationError(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InternalError" => {
                        return RusotoError::Service(
                            ListEndpointsByPlatformApplicationError::InternalError(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidParameter" => {
                        return RusotoError::Service(
                            ListEndpointsByPlatformApplicationError::InvalidParameter(
                                parsed_error.message,
                            ),
                        )
                    }
                    "NotFound" => {
                        return RusotoError::Service(
                            ListEndpointsByPlatformApplicationError::NotFound(parsed_error.message),
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
impl fmt::Display for ListEndpointsByPlatformApplicationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListEndpointsByPlatformApplicationError::AuthorizationError(ref cause) => {
                write!(f, "{}", cause)
            }
            ListEndpointsByPlatformApplicationError::InternalError(ref cause) => {
                write!(f, "{}", cause)
            }
            ListEndpointsByPlatformApplicationError::InvalidParameter(ref cause) => {
                write!(f, "{}", cause)
            }
            ListEndpointsByPlatformApplicationError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListEndpointsByPlatformApplicationError {}
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
}

impl ListPhoneNumbersOptedOutError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListPhoneNumbersOptedOutError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AuthorizationError" => {
                        return RusotoError::Service(
                            ListPhoneNumbersOptedOutError::AuthorizationError(parsed_error.message),
                        )
                    }
                    "InternalError" => {
                        return RusotoError::Service(ListPhoneNumbersOptedOutError::InternalError(
                            parsed_error.message,
                        ))
                    }
                    "InvalidParameter" => {
                        return RusotoError::Service(
                            ListPhoneNumbersOptedOutError::InvalidParameter(parsed_error.message),
                        )
                    }
                    "Throttled" => {
                        return RusotoError::Service(ListPhoneNumbersOptedOutError::Throttled(
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
impl fmt::Display for ListPhoneNumbersOptedOutError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListPhoneNumbersOptedOutError::AuthorizationError(ref cause) => write!(f, "{}", cause),
            ListPhoneNumbersOptedOutError::InternalError(ref cause) => write!(f, "{}", cause),
            ListPhoneNumbersOptedOutError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListPhoneNumbersOptedOutError::Throttled(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListPhoneNumbersOptedOutError {}
/// Errors returned by ListPlatformApplications
#[derive(Debug, PartialEq)]
pub enum ListPlatformApplicationsError {
    /// <p>Indicates that the user has been denied access to the requested resource.</p>
    AuthorizationError(String),
    /// <p>Indicates an internal service error.</p>
    InternalError(String),
    /// <p>Indicates that a request parameter does not comply with the associated constraints.</p>
    InvalidParameter(String),
}

impl ListPlatformApplicationsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListPlatformApplicationsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AuthorizationError" => {
                        return RusotoError::Service(
                            ListPlatformApplicationsError::AuthorizationError(parsed_error.message),
                        )
                    }
                    "InternalError" => {
                        return RusotoError::Service(ListPlatformApplicationsError::InternalError(
                            parsed_error.message,
                        ))
                    }
                    "InvalidParameter" => {
                        return RusotoError::Service(
                            ListPlatformApplicationsError::InvalidParameter(parsed_error.message),
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
impl fmt::Display for ListPlatformApplicationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListPlatformApplicationsError::AuthorizationError(ref cause) => write!(f, "{}", cause),
            ListPlatformApplicationsError::InternalError(ref cause) => write!(f, "{}", cause),
            ListPlatformApplicationsError::InvalidParameter(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListPlatformApplicationsError {}
/// Errors returned by ListSubscriptions
#[derive(Debug, PartialEq)]
pub enum ListSubscriptionsError {
    /// <p>Indicates that the user has been denied access to the requested resource.</p>
    AuthorizationError(String),
    /// <p>Indicates an internal service error.</p>
    InternalError(String),
    /// <p>Indicates that a request parameter does not comply with the associated constraints.</p>
    InvalidParameter(String),
}

impl ListSubscriptionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListSubscriptionsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AuthorizationError" => {
                        return RusotoError::Service(ListSubscriptionsError::AuthorizationError(
                            parsed_error.message,
                        ))
                    }
                    "InternalError" => {
                        return RusotoError::Service(ListSubscriptionsError::InternalError(
                            parsed_error.message,
                        ))
                    }
                    "InvalidParameter" => {
                        return RusotoError::Service(ListSubscriptionsError::InvalidParameter(
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
impl fmt::Display for ListSubscriptionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListSubscriptionsError::AuthorizationError(ref cause) => write!(f, "{}", cause),
            ListSubscriptionsError::InternalError(ref cause) => write!(f, "{}", cause),
            ListSubscriptionsError::InvalidParameter(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListSubscriptionsError {}
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
}

impl ListSubscriptionsByTopicError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListSubscriptionsByTopicError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AuthorizationError" => {
                        return RusotoError::Service(
                            ListSubscriptionsByTopicError::AuthorizationError(parsed_error.message),
                        )
                    }
                    "InternalError" => {
                        return RusotoError::Service(ListSubscriptionsByTopicError::InternalError(
                            parsed_error.message,
                        ))
                    }
                    "InvalidParameter" => {
                        return RusotoError::Service(
                            ListSubscriptionsByTopicError::InvalidParameter(parsed_error.message),
                        )
                    }
                    "NotFound" => {
                        return RusotoError::Service(ListSubscriptionsByTopicError::NotFound(
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
impl fmt::Display for ListSubscriptionsByTopicError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListSubscriptionsByTopicError::AuthorizationError(ref cause) => write!(f, "{}", cause),
            ListSubscriptionsByTopicError::InternalError(ref cause) => write!(f, "{}", cause),
            ListSubscriptionsByTopicError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListSubscriptionsByTopicError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListSubscriptionsByTopicError {}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>Indicates that the user has been denied access to the requested resource.</p>
    AuthorizationError(String),
    /// <p>Can't perform multiple operations on a tag simultaneously. Perform the operations sequentially.</p>
    ConcurrentAccess(String),
    /// <p>Indicates that a request parameter does not comply with the associated constraints.</p>
    InvalidParameter(String),
    /// <p>Can't tag resource. Verify that the topic exists.</p>
    ResourceNotFound(String),
    /// <p>The request doesn't comply with the IAM tag policy. Correct your request and then retry it.</p>
    TagPolicy(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AuthorizationError" => {
                        return RusotoError::Service(ListTagsForResourceError::AuthorizationError(
                            parsed_error.message,
                        ))
                    }
                    "ConcurrentAccess" => {
                        return RusotoError::Service(ListTagsForResourceError::ConcurrentAccess(
                            parsed_error.message,
                        ))
                    }
                    "InvalidParameter" => {
                        return RusotoError::Service(ListTagsForResourceError::InvalidParameter(
                            parsed_error.message,
                        ))
                    }
                    "ResourceNotFound" => {
                        return RusotoError::Service(ListTagsForResourceError::ResourceNotFound(
                            parsed_error.message,
                        ))
                    }
                    "TagPolicy" => {
                        return RusotoError::Service(ListTagsForResourceError::TagPolicy(
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
impl fmt::Display for ListTagsForResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListTagsForResourceError::AuthorizationError(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::ConcurrentAccess(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::TagPolicy(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTagsForResourceError {}
/// Errors returned by ListTopics
#[derive(Debug, PartialEq)]
pub enum ListTopicsError {
    /// <p>Indicates that the user has been denied access to the requested resource.</p>
    AuthorizationError(String),
    /// <p>Indicates an internal service error.</p>
    InternalError(String),
    /// <p>Indicates that a request parameter does not comply with the associated constraints.</p>
    InvalidParameter(String),
}

impl ListTopicsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTopicsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AuthorizationError" => {
                        return RusotoError::Service(ListTopicsError::AuthorizationError(
                            parsed_error.message,
                        ))
                    }
                    "InternalError" => {
                        return RusotoError::Service(ListTopicsError::InternalError(
                            parsed_error.message,
                        ))
                    }
                    "InvalidParameter" => {
                        return RusotoError::Service(ListTopicsError::InvalidParameter(
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
impl fmt::Display for ListTopicsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListTopicsError::AuthorizationError(ref cause) => write!(f, "{}", cause),
            ListTopicsError::InternalError(ref cause) => write!(f, "{}", cause),
            ListTopicsError::InvalidParameter(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTopicsError {}
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
}

impl OptInPhoneNumberError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<OptInPhoneNumberError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AuthorizationError" => {
                        return RusotoError::Service(OptInPhoneNumberError::AuthorizationError(
                            parsed_error.message,
                        ))
                    }
                    "InternalError" => {
                        return RusotoError::Service(OptInPhoneNumberError::InternalError(
                            parsed_error.message,
                        ))
                    }
                    "InvalidParameter" => {
                        return RusotoError::Service(OptInPhoneNumberError::InvalidParameter(
                            parsed_error.message,
                        ))
                    }
                    "Throttled" => {
                        return RusotoError::Service(OptInPhoneNumberError::Throttled(
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
impl fmt::Display for OptInPhoneNumberError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            OptInPhoneNumberError::AuthorizationError(ref cause) => write!(f, "{}", cause),
            OptInPhoneNumberError::InternalError(ref cause) => write!(f, "{}", cause),
            OptInPhoneNumberError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            OptInPhoneNumberError::Throttled(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for OptInPhoneNumberError {}
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
    /// <p>The credential signature isn't valid. You must use an HTTPS endpoint and sign your request using Signature Version 4.</p>
    InvalidSecurity(String),
    /// <p>The ciphertext references a key that doesn't exist or that you don't have access to.</p>
    KMSAccessDenied(String),
    /// <p>The request was rejected because the specified customer master key (CMK) isn't enabled.</p>
    KMSDisabled(String),
    /// <p>The request was rejected because the state of the specified resource isn't valid for this request. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified entity or resource can't be found.</p>
    KMSNotFound(String),
    /// <p>The AWS access key ID needs a subscription for the service.</p>
    KMSOptInRequired(String),
    /// <p>The request was denied due to request throttling. For more information about throttling, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/limits.html#requests-per-second">Limits</a> in the <i>AWS Key Management Service Developer Guide.</i> </p>
    KMSThrottling(String),
    /// <p>Indicates that the requested resource does not exist.</p>
    NotFound(String),
    /// <p>Exception error indicating platform application disabled.</p>
    PlatformApplicationDisabled(String),
}

impl PublishError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PublishError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AuthorizationError" => {
                        return RusotoError::Service(PublishError::AuthorizationError(
                            parsed_error.message,
                        ))
                    }
                    "EndpointDisabled" => {
                        return RusotoError::Service(PublishError::EndpointDisabled(
                            parsed_error.message,
                        ))
                    }
                    "InternalError" => {
                        return RusotoError::Service(PublishError::InternalError(
                            parsed_error.message,
                        ))
                    }
                    "InvalidParameter" => {
                        return RusotoError::Service(PublishError::InvalidParameter(
                            parsed_error.message,
                        ))
                    }
                    "ParameterValueInvalid" => {
                        return RusotoError::Service(PublishError::InvalidParameterValue(
                            parsed_error.message,
                        ))
                    }
                    "InvalidSecurity" => {
                        return RusotoError::Service(PublishError::InvalidSecurity(
                            parsed_error.message,
                        ))
                    }
                    "KMSAccessDenied" => {
                        return RusotoError::Service(PublishError::KMSAccessDenied(
                            parsed_error.message,
                        ))
                    }
                    "KMSDisabled" => {
                        return RusotoError::Service(PublishError::KMSDisabled(
                            parsed_error.message,
                        ))
                    }
                    "KMSInvalidState" => {
                        return RusotoError::Service(PublishError::KMSInvalidState(
                            parsed_error.message,
                        ))
                    }
                    "KMSNotFound" => {
                        return RusotoError::Service(PublishError::KMSNotFound(
                            parsed_error.message,
                        ))
                    }
                    "KMSOptInRequired" => {
                        return RusotoError::Service(PublishError::KMSOptInRequired(
                            parsed_error.message,
                        ))
                    }
                    "KMSThrottling" => {
                        return RusotoError::Service(PublishError::KMSThrottling(
                            parsed_error.message,
                        ))
                    }
                    "NotFound" => {
                        return RusotoError::Service(PublishError::NotFound(parsed_error.message))
                    }
                    "PlatformApplicationDisabled" => {
                        return RusotoError::Service(PublishError::PlatformApplicationDisabled(
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
impl fmt::Display for PublishError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PublishError::AuthorizationError(ref cause) => write!(f, "{}", cause),
            PublishError::EndpointDisabled(ref cause) => write!(f, "{}", cause),
            PublishError::InternalError(ref cause) => write!(f, "{}", cause),
            PublishError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            PublishError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            PublishError::InvalidSecurity(ref cause) => write!(f, "{}", cause),
            PublishError::KMSAccessDenied(ref cause) => write!(f, "{}", cause),
            PublishError::KMSDisabled(ref cause) => write!(f, "{}", cause),
            PublishError::KMSInvalidState(ref cause) => write!(f, "{}", cause),
            PublishError::KMSNotFound(ref cause) => write!(f, "{}", cause),
            PublishError::KMSOptInRequired(ref cause) => write!(f, "{}", cause),
            PublishError::KMSThrottling(ref cause) => write!(f, "{}", cause),
            PublishError::NotFound(ref cause) => write!(f, "{}", cause),
            PublishError::PlatformApplicationDisabled(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PublishError {}
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
}

impl RemovePermissionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RemovePermissionError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AuthorizationError" => {
                        return RusotoError::Service(RemovePermissionError::AuthorizationError(
                            parsed_error.message,
                        ))
                    }
                    "InternalError" => {
                        return RusotoError::Service(RemovePermissionError::InternalError(
                            parsed_error.message,
                        ))
                    }
                    "InvalidParameter" => {
                        return RusotoError::Service(RemovePermissionError::InvalidParameter(
                            parsed_error.message,
                        ))
                    }
                    "NotFound" => {
                        return RusotoError::Service(RemovePermissionError::NotFound(
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
impl fmt::Display for RemovePermissionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RemovePermissionError::AuthorizationError(ref cause) => write!(f, "{}", cause),
            RemovePermissionError::InternalError(ref cause) => write!(f, "{}", cause),
            RemovePermissionError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            RemovePermissionError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RemovePermissionError {}
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
}

impl SetEndpointAttributesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SetEndpointAttributesError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AuthorizationError" => {
                        return RusotoError::Service(
                            SetEndpointAttributesError::AuthorizationError(parsed_error.message),
                        )
                    }
                    "InternalError" => {
                        return RusotoError::Service(SetEndpointAttributesError::InternalError(
                            parsed_error.message,
                        ))
                    }
                    "InvalidParameter" => {
                        return RusotoError::Service(SetEndpointAttributesError::InvalidParameter(
                            parsed_error.message,
                        ))
                    }
                    "NotFound" => {
                        return RusotoError::Service(SetEndpointAttributesError::NotFound(
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
impl fmt::Display for SetEndpointAttributesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SetEndpointAttributesError::AuthorizationError(ref cause) => write!(f, "{}", cause),
            SetEndpointAttributesError::InternalError(ref cause) => write!(f, "{}", cause),
            SetEndpointAttributesError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            SetEndpointAttributesError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for SetEndpointAttributesError {}
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
}

impl SetPlatformApplicationAttributesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<SetPlatformApplicationAttributesError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AuthorizationError" => {
                        return RusotoError::Service(
                            SetPlatformApplicationAttributesError::AuthorizationError(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InternalError" => {
                        return RusotoError::Service(
                            SetPlatformApplicationAttributesError::InternalError(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidParameter" => {
                        return RusotoError::Service(
                            SetPlatformApplicationAttributesError::InvalidParameter(
                                parsed_error.message,
                            ),
                        )
                    }
                    "NotFound" => {
                        return RusotoError::Service(
                            SetPlatformApplicationAttributesError::NotFound(parsed_error.message),
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
impl fmt::Display for SetPlatformApplicationAttributesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SetPlatformApplicationAttributesError::AuthorizationError(ref cause) => {
                write!(f, "{}", cause)
            }
            SetPlatformApplicationAttributesError::InternalError(ref cause) => {
                write!(f, "{}", cause)
            }
            SetPlatformApplicationAttributesError::InvalidParameter(ref cause) => {
                write!(f, "{}", cause)
            }
            SetPlatformApplicationAttributesError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for SetPlatformApplicationAttributesError {}
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
}

impl SetSMSAttributesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SetSMSAttributesError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AuthorizationError" => {
                        return RusotoError::Service(SetSMSAttributesError::AuthorizationError(
                            parsed_error.message,
                        ))
                    }
                    "InternalError" => {
                        return RusotoError::Service(SetSMSAttributesError::InternalError(
                            parsed_error.message,
                        ))
                    }
                    "InvalidParameter" => {
                        return RusotoError::Service(SetSMSAttributesError::InvalidParameter(
                            parsed_error.message,
                        ))
                    }
                    "Throttled" => {
                        return RusotoError::Service(SetSMSAttributesError::Throttled(
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
impl fmt::Display for SetSMSAttributesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SetSMSAttributesError::AuthorizationError(ref cause) => write!(f, "{}", cause),
            SetSMSAttributesError::InternalError(ref cause) => write!(f, "{}", cause),
            SetSMSAttributesError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            SetSMSAttributesError::Throttled(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for SetSMSAttributesError {}
/// Errors returned by SetSubscriptionAttributes
#[derive(Debug, PartialEq)]
pub enum SetSubscriptionAttributesError {
    /// <p>Indicates that the user has been denied access to the requested resource.</p>
    AuthorizationError(String),
    /// <p>Indicates that the number of filter polices in your AWS account exceeds the limit. To add more filter polices, submit an SNS Limit Increase case in the AWS Support Center.</p>
    FilterPolicyLimitExceeded(String),
    /// <p>Indicates an internal service error.</p>
    InternalError(String),
    /// <p>Indicates that a request parameter does not comply with the associated constraints.</p>
    InvalidParameter(String),
    /// <p>Indicates that the requested resource does not exist.</p>
    NotFound(String),
}

impl SetSubscriptionAttributesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SetSubscriptionAttributesError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AuthorizationError" => {
                        return RusotoError::Service(
                            SetSubscriptionAttributesError::AuthorizationError(
                                parsed_error.message,
                            ),
                        )
                    }
                    "FilterPolicyLimitExceeded" => {
                        return RusotoError::Service(
                            SetSubscriptionAttributesError::FilterPolicyLimitExceeded(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InternalError" => {
                        return RusotoError::Service(SetSubscriptionAttributesError::InternalError(
                            parsed_error.message,
                        ))
                    }
                    "InvalidParameter" => {
                        return RusotoError::Service(
                            SetSubscriptionAttributesError::InvalidParameter(parsed_error.message),
                        )
                    }
                    "NotFound" => {
                        return RusotoError::Service(SetSubscriptionAttributesError::NotFound(
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
impl fmt::Display for SetSubscriptionAttributesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SetSubscriptionAttributesError::AuthorizationError(ref cause) => write!(f, "{}", cause),
            SetSubscriptionAttributesError::FilterPolicyLimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            SetSubscriptionAttributesError::InternalError(ref cause) => write!(f, "{}", cause),
            SetSubscriptionAttributesError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            SetSubscriptionAttributesError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for SetSubscriptionAttributesError {}
/// Errors returned by SetTopicAttributes
#[derive(Debug, PartialEq)]
pub enum SetTopicAttributesError {
    /// <p>Indicates that the user has been denied access to the requested resource.</p>
    AuthorizationError(String),
    /// <p>Indicates an internal service error.</p>
    InternalError(String),
    /// <p>Indicates that a request parameter does not comply with the associated constraints.</p>
    InvalidParameter(String),
    /// <p>The credential signature isn't valid. You must use an HTTPS endpoint and sign your request using Signature Version 4.</p>
    InvalidSecurity(String),
    /// <p>Indicates that the requested resource does not exist.</p>
    NotFound(String),
}

impl SetTopicAttributesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SetTopicAttributesError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AuthorizationError" => {
                        return RusotoError::Service(SetTopicAttributesError::AuthorizationError(
                            parsed_error.message,
                        ))
                    }
                    "InternalError" => {
                        return RusotoError::Service(SetTopicAttributesError::InternalError(
                            parsed_error.message,
                        ))
                    }
                    "InvalidParameter" => {
                        return RusotoError::Service(SetTopicAttributesError::InvalidParameter(
                            parsed_error.message,
                        ))
                    }
                    "InvalidSecurity" => {
                        return RusotoError::Service(SetTopicAttributesError::InvalidSecurity(
                            parsed_error.message,
                        ))
                    }
                    "NotFound" => {
                        return RusotoError::Service(SetTopicAttributesError::NotFound(
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
impl fmt::Display for SetTopicAttributesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SetTopicAttributesError::AuthorizationError(ref cause) => write!(f, "{}", cause),
            SetTopicAttributesError::InternalError(ref cause) => write!(f, "{}", cause),
            SetTopicAttributesError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            SetTopicAttributesError::InvalidSecurity(ref cause) => write!(f, "{}", cause),
            SetTopicAttributesError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for SetTopicAttributesError {}
/// Errors returned by Subscribe
#[derive(Debug, PartialEq)]
pub enum SubscribeError {
    /// <p>Indicates that the user has been denied access to the requested resource.</p>
    AuthorizationError(String),
    /// <p>Indicates that the number of filter polices in your AWS account exceeds the limit. To add more filter polices, submit an SNS Limit Increase case in the AWS Support Center.</p>
    FilterPolicyLimitExceeded(String),
    /// <p>Indicates an internal service error.</p>
    InternalError(String),
    /// <p>Indicates that a request parameter does not comply with the associated constraints.</p>
    InvalidParameter(String),
    /// <p>The credential signature isn't valid. You must use an HTTPS endpoint and sign your request using Signature Version 4.</p>
    InvalidSecurity(String),
    /// <p>Indicates that the requested resource does not exist.</p>
    NotFound(String),
    /// <p>Indicates that the customer already owns the maximum allowed number of subscriptions.</p>
    SubscriptionLimitExceeded(String),
}

impl SubscribeError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SubscribeError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AuthorizationError" => {
                        return RusotoError::Service(SubscribeError::AuthorizationError(
                            parsed_error.message,
                        ))
                    }
                    "FilterPolicyLimitExceeded" => {
                        return RusotoError::Service(SubscribeError::FilterPolicyLimitExceeded(
                            parsed_error.message,
                        ))
                    }
                    "InternalError" => {
                        return RusotoError::Service(SubscribeError::InternalError(
                            parsed_error.message,
                        ))
                    }
                    "InvalidParameter" => {
                        return RusotoError::Service(SubscribeError::InvalidParameter(
                            parsed_error.message,
                        ))
                    }
                    "InvalidSecurity" => {
                        return RusotoError::Service(SubscribeError::InvalidSecurity(
                            parsed_error.message,
                        ))
                    }
                    "NotFound" => {
                        return RusotoError::Service(SubscribeError::NotFound(parsed_error.message))
                    }
                    "SubscriptionLimitExceeded" => {
                        return RusotoError::Service(SubscribeError::SubscriptionLimitExceeded(
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
impl fmt::Display for SubscribeError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SubscribeError::AuthorizationError(ref cause) => write!(f, "{}", cause),
            SubscribeError::FilterPolicyLimitExceeded(ref cause) => write!(f, "{}", cause),
            SubscribeError::InternalError(ref cause) => write!(f, "{}", cause),
            SubscribeError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            SubscribeError::InvalidSecurity(ref cause) => write!(f, "{}", cause),
            SubscribeError::NotFound(ref cause) => write!(f, "{}", cause),
            SubscribeError::SubscriptionLimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for SubscribeError {}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>Indicates that the user has been denied access to the requested resource.</p>
    AuthorizationError(String),
    /// <p>Can't perform multiple operations on a tag simultaneously. Perform the operations sequentially.</p>
    ConcurrentAccess(String),
    /// <p>Indicates that a request parameter does not comply with the associated constraints.</p>
    InvalidParameter(String),
    /// <p>Can't tag resource. Verify that the topic exists.</p>
    ResourceNotFound(String),
    /// <p>A tag has been added to a resource with the same ARN as a deleted resource. Wait a short while and then retry the operation.</p>
    StaleTag(String),
    /// <p>Can't add more than 50 tags to a topic.</p>
    TagLimitExceeded(String),
    /// <p>The request doesn't comply with the IAM tag policy. Correct your request and then retry it.</p>
    TagPolicy(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AuthorizationError" => {
                        return RusotoError::Service(TagResourceError::AuthorizationError(
                            parsed_error.message,
                        ))
                    }
                    "ConcurrentAccess" => {
                        return RusotoError::Service(TagResourceError::ConcurrentAccess(
                            parsed_error.message,
                        ))
                    }
                    "InvalidParameter" => {
                        return RusotoError::Service(TagResourceError::InvalidParameter(
                            parsed_error.message,
                        ))
                    }
                    "ResourceNotFound" => {
                        return RusotoError::Service(TagResourceError::ResourceNotFound(
                            parsed_error.message,
                        ))
                    }
                    "StaleTag" => {
                        return RusotoError::Service(TagResourceError::StaleTag(
                            parsed_error.message,
                        ))
                    }
                    "TagLimitExceeded" => {
                        return RusotoError::Service(TagResourceError::TagLimitExceeded(
                            parsed_error.message,
                        ))
                    }
                    "TagPolicy" => {
                        return RusotoError::Service(TagResourceError::TagPolicy(
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
impl fmt::Display for TagResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TagResourceError::AuthorizationError(ref cause) => write!(f, "{}", cause),
            TagResourceError::ConcurrentAccess(ref cause) => write!(f, "{}", cause),
            TagResourceError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            TagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            TagResourceError::StaleTag(ref cause) => write!(f, "{}", cause),
            TagResourceError::TagLimitExceeded(ref cause) => write!(f, "{}", cause),
            TagResourceError::TagPolicy(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TagResourceError {}
/// Errors returned by Unsubscribe
#[derive(Debug, PartialEq)]
pub enum UnsubscribeError {
    /// <p>Indicates that the user has been denied access to the requested resource.</p>
    AuthorizationError(String),
    /// <p>Indicates an internal service error.</p>
    InternalError(String),
    /// <p>Indicates that a request parameter does not comply with the associated constraints.</p>
    InvalidParameter(String),
    /// <p>The credential signature isn't valid. You must use an HTTPS endpoint and sign your request using Signature Version 4.</p>
    InvalidSecurity(String),
    /// <p>Indicates that the requested resource does not exist.</p>
    NotFound(String),
}

impl UnsubscribeError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UnsubscribeError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AuthorizationError" => {
                        return RusotoError::Service(UnsubscribeError::AuthorizationError(
                            parsed_error.message,
                        ))
                    }
                    "InternalError" => {
                        return RusotoError::Service(UnsubscribeError::InternalError(
                            parsed_error.message,
                        ))
                    }
                    "InvalidParameter" => {
                        return RusotoError::Service(UnsubscribeError::InvalidParameter(
                            parsed_error.message,
                        ))
                    }
                    "InvalidSecurity" => {
                        return RusotoError::Service(UnsubscribeError::InvalidSecurity(
                            parsed_error.message,
                        ))
                    }
                    "NotFound" => {
                        return RusotoError::Service(UnsubscribeError::NotFound(
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
impl fmt::Display for UnsubscribeError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UnsubscribeError::AuthorizationError(ref cause) => write!(f, "{}", cause),
            UnsubscribeError::InternalError(ref cause) => write!(f, "{}", cause),
            UnsubscribeError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            UnsubscribeError::InvalidSecurity(ref cause) => write!(f, "{}", cause),
            UnsubscribeError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UnsubscribeError {}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>Indicates that the user has been denied access to the requested resource.</p>
    AuthorizationError(String),
    /// <p>Can't perform multiple operations on a tag simultaneously. Perform the operations sequentially.</p>
    ConcurrentAccess(String),
    /// <p>Indicates that a request parameter does not comply with the associated constraints.</p>
    InvalidParameter(String),
    /// <p>Can't tag resource. Verify that the topic exists.</p>
    ResourceNotFound(String),
    /// <p>A tag has been added to a resource with the same ARN as a deleted resource. Wait a short while and then retry the operation.</p>
    StaleTag(String),
    /// <p>Can't add more than 50 tags to a topic.</p>
    TagLimitExceeded(String),
    /// <p>The request doesn't comply with the IAM tag policy. Correct your request and then retry it.</p>
    TagPolicy(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AuthorizationError" => {
                        return RusotoError::Service(UntagResourceError::AuthorizationError(
                            parsed_error.message,
                        ))
                    }
                    "ConcurrentAccess" => {
                        return RusotoError::Service(UntagResourceError::ConcurrentAccess(
                            parsed_error.message,
                        ))
                    }
                    "InvalidParameter" => {
                        return RusotoError::Service(UntagResourceError::InvalidParameter(
                            parsed_error.message,
                        ))
                    }
                    "ResourceNotFound" => {
                        return RusotoError::Service(UntagResourceError::ResourceNotFound(
                            parsed_error.message,
                        ))
                    }
                    "StaleTag" => {
                        return RusotoError::Service(UntagResourceError::StaleTag(
                            parsed_error.message,
                        ))
                    }
                    "TagLimitExceeded" => {
                        return RusotoError::Service(UntagResourceError::TagLimitExceeded(
                            parsed_error.message,
                        ))
                    }
                    "TagPolicy" => {
                        return RusotoError::Service(UntagResourceError::TagPolicy(
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
impl fmt::Display for UntagResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UntagResourceError::AuthorizationError(ref cause) => write!(f, "{}", cause),
            UntagResourceError::ConcurrentAccess(ref cause) => write!(f, "{}", cause),
            UntagResourceError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            UntagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UntagResourceError::StaleTag(ref cause) => write!(f, "{}", cause),
            UntagResourceError::TagLimitExceeded(ref cause) => write!(f, "{}", cause),
            UntagResourceError::TagPolicy(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UntagResourceError {}
/// Trait representing the capabilities of the Amazon SNS API. Amazon SNS clients implement this trait.
#[async_trait]
pub trait Sns {
    /// <p>Adds a statement to a topic's access control policy, granting access for the specified AWS accounts to the specified actions.</p>
    async fn add_permission(
        &self,
        input: AddPermissionInput,
    ) -> Result<(), RusotoError<AddPermissionError>>;

    /// <p>Accepts a phone number and indicates whether the phone holder has opted out of receiving SMS messages from your account. You cannot send SMS messages to a number that is opted out.</p> <p>To resume sending messages, you can opt in the number by using the <code>OptInPhoneNumber</code> action.</p>
    async fn check_if_phone_number_is_opted_out(
        &self,
        input: CheckIfPhoneNumberIsOptedOutInput,
    ) -> Result<CheckIfPhoneNumberIsOptedOutResponse, RusotoError<CheckIfPhoneNumberIsOptedOutError>>;

    /// <p>Verifies an endpoint owner's intent to receive messages by validating the token sent to the endpoint by an earlier <code>Subscribe</code> action. If the token is valid, the action creates a new subscription and returns its Amazon Resource Name (ARN). This call requires an AWS signature only when the <code>AuthenticateOnUnsubscribe</code> flag is set to "true".</p>
    async fn confirm_subscription(
        &self,
        input: ConfirmSubscriptionInput,
    ) -> Result<ConfirmSubscriptionResponse, RusotoError<ConfirmSubscriptionError>>;

    /// <p>Creates a platform application object for one of the supported push notification services, such as APNS and FCM, to which devices and mobile apps may register. You must specify PlatformPrincipal and PlatformCredential attributes when using the <code>CreatePlatformApplication</code> action. The PlatformPrincipal is received from the notification service. For APNS/APNS_SANDBOX, PlatformPrincipal is "SSL certificate". For FCM, PlatformPrincipal is not applicable. For ADM, PlatformPrincipal is "client id". The PlatformCredential is also received from the notification service. For WNS, PlatformPrincipal is "Package Security Identifier". For MPNS, PlatformPrincipal is "TLS certificate". For Baidu, PlatformPrincipal is "API key".</p> <p>For APNS/APNS_SANDBOX, PlatformCredential is "private key". For FCM, PlatformCredential is "API key". For ADM, PlatformCredential is "client secret". For WNS, PlatformCredential is "secret key". For MPNS, PlatformCredential is "private key". For Baidu, PlatformCredential is "secret key". The PlatformApplicationArn that is returned when using <code>CreatePlatformApplication</code> is then used as an attribute for the <code>CreatePlatformEndpoint</code> action.</p>
    async fn create_platform_application(
        &self,
        input: CreatePlatformApplicationInput,
    ) -> Result<CreatePlatformApplicationResponse, RusotoError<CreatePlatformApplicationError>>;

    /// <p>Creates an endpoint for a device and mobile app on one of the supported push notification services, such as FCM and APNS. <code>CreatePlatformEndpoint</code> requires the PlatformApplicationArn that is returned from <code>CreatePlatformApplication</code>. The EndpointArn that is returned when using <code>CreatePlatformEndpoint</code> can then be used by the <code>Publish</code> action to send a message to a mobile app or by the <code>Subscribe</code> action for subscription to a topic. The <code>CreatePlatformEndpoint</code> action is idempotent, so if the requester already owns an endpoint with the same device token and attributes, that endpoint's ARN is returned without creating a new endpoint. For more information, see <a href="https://docs.aws.amazon.com/sns/latest/dg/SNSMobilePush.html">Using Amazon SNS Mobile Push Notifications</a>. </p> <p>When using <code>CreatePlatformEndpoint</code> with Baidu, two attributes must be provided: ChannelId and UserId. The token field must also contain the ChannelId. For more information, see <a href="https://docs.aws.amazon.com/sns/latest/dg/SNSMobilePushBaiduEndpoint.html">Creating an Amazon SNS Endpoint for Baidu</a>. </p>
    async fn create_platform_endpoint(
        &self,
        input: CreatePlatformEndpointInput,
    ) -> Result<CreateEndpointResponse, RusotoError<CreatePlatformEndpointError>>;

    /// <p>Creates a topic to which notifications can be published. Users can create at most 100,000 topics. For more information, see <a href="http://aws.amazon.com/sns/">https://aws.amazon.com/sns</a>. This action is idempotent, so if the requester already owns a topic with the specified name, that topic's ARN is returned without creating a new topic.</p>
    async fn create_topic(
        &self,
        input: CreateTopicInput,
    ) -> Result<CreateTopicResponse, RusotoError<CreateTopicError>>;

    /// <p>Deletes the endpoint for a device and mobile app from Amazon SNS. This action is idempotent. For more information, see <a href="https://docs.aws.amazon.com/sns/latest/dg/SNSMobilePush.html">Using Amazon SNS Mobile Push Notifications</a>. </p> <p>When you delete an endpoint that is also subscribed to a topic, then you must also unsubscribe the endpoint from the topic.</p>
    async fn delete_endpoint(
        &self,
        input: DeleteEndpointInput,
    ) -> Result<(), RusotoError<DeleteEndpointError>>;

    /// <p>Deletes a platform application object for one of the supported push notification services, such as APNS and FCM. For more information, see <a href="https://docs.aws.amazon.com/sns/latest/dg/SNSMobilePush.html">Using Amazon SNS Mobile Push Notifications</a>. </p>
    async fn delete_platform_application(
        &self,
        input: DeletePlatformApplicationInput,
    ) -> Result<(), RusotoError<DeletePlatformApplicationError>>;

    /// <p>Deletes a topic and all its subscriptions. Deleting a topic might prevent some messages previously sent to the topic from being delivered to subscribers. This action is idempotent, so deleting a topic that does not exist does not result in an error.</p>
    async fn delete_topic(
        &self,
        input: DeleteTopicInput,
    ) -> Result<(), RusotoError<DeleteTopicError>>;

    /// <p>Retrieves the endpoint attributes for a device on one of the supported push notification services, such as FCM and APNS. For more information, see <a href="https://docs.aws.amazon.com/sns/latest/dg/SNSMobilePush.html">Using Amazon SNS Mobile Push Notifications</a>. </p>
    async fn get_endpoint_attributes(
        &self,
        input: GetEndpointAttributesInput,
    ) -> Result<GetEndpointAttributesResponse, RusotoError<GetEndpointAttributesError>>;

    /// <p>Retrieves the attributes of the platform application object for the supported push notification services, such as APNS and FCM. For more information, see <a href="https://docs.aws.amazon.com/sns/latest/dg/SNSMobilePush.html">Using Amazon SNS Mobile Push Notifications</a>. </p>
    async fn get_platform_application_attributes(
        &self,
        input: GetPlatformApplicationAttributesInput,
    ) -> Result<
        GetPlatformApplicationAttributesResponse,
        RusotoError<GetPlatformApplicationAttributesError>,
    >;

    /// <p>Returns the settings for sending SMS messages from your account.</p> <p>These settings are set with the <code>SetSMSAttributes</code> action.</p>
    async fn get_sms_attributes(
        &self,
        input: GetSMSAttributesInput,
    ) -> Result<GetSMSAttributesResponse, RusotoError<GetSMSAttributesError>>;

    /// <p>Returns all of the properties of a subscription.</p>
    async fn get_subscription_attributes(
        &self,
        input: GetSubscriptionAttributesInput,
    ) -> Result<GetSubscriptionAttributesResponse, RusotoError<GetSubscriptionAttributesError>>;

    /// <p>Returns all of the properties of a topic. Topic properties returned might differ based on the authorization of the user.</p>
    async fn get_topic_attributes(
        &self,
        input: GetTopicAttributesInput,
    ) -> Result<GetTopicAttributesResponse, RusotoError<GetTopicAttributesError>>;

    /// <p>Lists the endpoints and endpoint attributes for devices in a supported push notification service, such as FCM and APNS. The results for <code>ListEndpointsByPlatformApplication</code> are paginated and return a limited list of endpoints, up to 100. If additional records are available after the first page results, then a NextToken string will be returned. To receive the next page, you call <code>ListEndpointsByPlatformApplication</code> again using the NextToken string received from the previous call. When there are no more records to return, NextToken will be null. For more information, see <a href="https://docs.aws.amazon.com/sns/latest/dg/SNSMobilePush.html">Using Amazon SNS Mobile Push Notifications</a>. </p> <p>This action is throttled at 30 transactions per second (TPS).</p>
    async fn list_endpoints_by_platform_application(
        &self,
        input: ListEndpointsByPlatformApplicationInput,
    ) -> Result<
        ListEndpointsByPlatformApplicationResponse,
        RusotoError<ListEndpointsByPlatformApplicationError>,
    >;

    /// <p>Returns a list of phone numbers that are opted out, meaning you cannot send SMS messages to them.</p> <p>The results for <code>ListPhoneNumbersOptedOut</code> are paginated, and each page returns up to 100 phone numbers. If additional phone numbers are available after the first page of results, then a <code>NextToken</code> string will be returned. To receive the next page, you call <code>ListPhoneNumbersOptedOut</code> again using the <code>NextToken</code> string received from the previous call. When there are no more records to return, <code>NextToken</code> will be null.</p>
    async fn list_phone_numbers_opted_out(
        &self,
        input: ListPhoneNumbersOptedOutInput,
    ) -> Result<ListPhoneNumbersOptedOutResponse, RusotoError<ListPhoneNumbersOptedOutError>>;

    /// <p>Lists the platform application objects for the supported push notification services, such as APNS and FCM. The results for <code>ListPlatformApplications</code> are paginated and return a limited list of applications, up to 100. If additional records are available after the first page results, then a NextToken string will be returned. To receive the next page, you call <code>ListPlatformApplications</code> using the NextToken string received from the previous call. When there are no more records to return, NextToken will be null. For more information, see <a href="https://docs.aws.amazon.com/sns/latest/dg/SNSMobilePush.html">Using Amazon SNS Mobile Push Notifications</a>. </p> <p>This action is throttled at 15 transactions per second (TPS).</p>
    async fn list_platform_applications(
        &self,
        input: ListPlatformApplicationsInput,
    ) -> Result<ListPlatformApplicationsResponse, RusotoError<ListPlatformApplicationsError>>;

    /// <p>Returns a list of the requester's subscriptions. Each call returns a limited list of subscriptions, up to 100. If there are more subscriptions, a <code>NextToken</code> is also returned. Use the <code>NextToken</code> parameter in a new <code>ListSubscriptions</code> call to get further results.</p> <p>This action is throttled at 30 transactions per second (TPS).</p>
    async fn list_subscriptions(
        &self,
        input: ListSubscriptionsInput,
    ) -> Result<ListSubscriptionsResponse, RusotoError<ListSubscriptionsError>>;

    /// <p>Returns a list of the subscriptions to a specific topic. Each call returns a limited list of subscriptions, up to 100. If there are more subscriptions, a <code>NextToken</code> is also returned. Use the <code>NextToken</code> parameter in a new <code>ListSubscriptionsByTopic</code> call to get further results.</p> <p>This action is throttled at 30 transactions per second (TPS).</p>
    async fn list_subscriptions_by_topic(
        &self,
        input: ListSubscriptionsByTopicInput,
    ) -> Result<ListSubscriptionsByTopicResponse, RusotoError<ListSubscriptionsByTopicError>>;

    /// <p>List all tags added to the specified Amazon SNS topic. For an overview, see <a href="https://docs.aws.amazon.com/sns/latest/dg/sns-tags.html">Amazon SNS Tags</a> in the <i>Amazon Simple Notification Service Developer Guide</i>.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>>;

    /// <p>Returns a list of the requester's topics. Each call returns a limited list of topics, up to 100. If there are more topics, a <code>NextToken</code> is also returned. Use the <code>NextToken</code> parameter in a new <code>ListTopics</code> call to get further results.</p> <p>This action is throttled at 30 transactions per second (TPS).</p>
    async fn list_topics(
        &self,
        input: ListTopicsInput,
    ) -> Result<ListTopicsResponse, RusotoError<ListTopicsError>>;

    /// <p>Use this request to opt in a phone number that is opted out, which enables you to resume sending SMS messages to the number.</p> <p>You can opt in a phone number only once every 30 days.</p>
    async fn opt_in_phone_number(
        &self,
        input: OptInPhoneNumberInput,
    ) -> Result<OptInPhoneNumberResponse, RusotoError<OptInPhoneNumberError>>;

    /// <p>Sends a message to an Amazon SNS topic or sends a text message (SMS message) directly to a phone number. </p> <p>If you send a message to a topic, Amazon SNS delivers the message to each endpoint that is subscribed to the topic. The format of the message depends on the notification protocol for each subscribed endpoint.</p> <p>When a <code>messageId</code> is returned, the message has been saved and Amazon SNS will attempt to deliver it shortly.</p> <p>To use the <code>Publish</code> action for sending a message to a mobile endpoint, such as an app on a Kindle device or mobile phone, you must specify the EndpointArn for the TargetArn parameter. The EndpointArn is returned when making a call with the <code>CreatePlatformEndpoint</code> action. </p> <p>For more information about formatting messages, see <a href="https://docs.aws.amazon.com/sns/latest/dg/mobile-push-send-custommessage.html">Send Custom Platform-Specific Payloads in Messages to Mobile Devices</a>. </p>
    async fn publish(
        &self,
        input: PublishInput,
    ) -> Result<PublishResponse, RusotoError<PublishError>>;

    /// <p>Removes a statement from a topic's access control policy.</p>
    async fn remove_permission(
        &self,
        input: RemovePermissionInput,
    ) -> Result<(), RusotoError<RemovePermissionError>>;

    /// <p>Sets the attributes for an endpoint for a device on one of the supported push notification services, such as FCM and APNS. For more information, see <a href="https://docs.aws.amazon.com/sns/latest/dg/SNSMobilePush.html">Using Amazon SNS Mobile Push Notifications</a>. </p>
    async fn set_endpoint_attributes(
        &self,
        input: SetEndpointAttributesInput,
    ) -> Result<(), RusotoError<SetEndpointAttributesError>>;

    /// <p>Sets the attributes of the platform application object for the supported push notification services, such as APNS and FCM. For more information, see <a href="https://docs.aws.amazon.com/sns/latest/dg/SNSMobilePush.html">Using Amazon SNS Mobile Push Notifications</a>. For information on configuring attributes for message delivery status, see <a href="https://docs.aws.amazon.com/sns/latest/dg/sns-msg-status.html">Using Amazon SNS Application Attributes for Message Delivery Status</a>. </p>
    async fn set_platform_application_attributes(
        &self,
        input: SetPlatformApplicationAttributesInput,
    ) -> Result<(), RusotoError<SetPlatformApplicationAttributesError>>;

    /// <p>Use this request to set the default settings for sending SMS messages and receiving daily SMS usage reports.</p> <p>You can override some of these settings for a single message when you use the <code>Publish</code> action with the <code>MessageAttributes.entry.N</code> parameter. For more information, see <a href="https://docs.aws.amazon.com/sns/latest/dg/sms_publish-to-phone.html">Sending an SMS Message</a> in the <i>Amazon SNS Developer Guide</i>.</p>
    async fn set_sms_attributes(
        &self,
        input: SetSMSAttributesInput,
    ) -> Result<SetSMSAttributesResponse, RusotoError<SetSMSAttributesError>>;

    /// <p>Allows a subscription owner to set an attribute of the subscription to a new value.</p>
    async fn set_subscription_attributes(
        &self,
        input: SetSubscriptionAttributesInput,
    ) -> Result<(), RusotoError<SetSubscriptionAttributesError>>;

    /// <p>Allows a topic owner to set an attribute of the topic to a new value.</p>
    async fn set_topic_attributes(
        &self,
        input: SetTopicAttributesInput,
    ) -> Result<(), RusotoError<SetTopicAttributesError>>;

    /// <p>Prepares to subscribe an endpoint by sending the endpoint a confirmation message. To actually create a subscription, the endpoint owner must call the <code>ConfirmSubscription</code> action with the token from the confirmation message. Confirmation tokens are valid for three days.</p> <p>This action is throttled at 100 transactions per second (TPS).</p>
    async fn subscribe(
        &self,
        input: SubscribeInput,
    ) -> Result<SubscribeResponse, RusotoError<SubscribeError>>;

    /// <p><p>Add tags to the specified Amazon SNS topic. For an overview, see <a href="https://docs.aws.amazon.com/sns/latest/dg/sns-tags.html">Amazon SNS Tags</a> in the <i>Amazon SNS Developer Guide</i>.</p> <p>When you use topic tags, keep the following guidelines in mind:</p> <ul> <li> <p>Adding more than 50 tags to a topic isn&#39;t recommended.</p> </li> <li> <p>Tags don&#39;t have any semantic meaning. Amazon SNS interprets tags as character strings.</p> </li> <li> <p>Tags are case-sensitive.</p> </li> <li> <p>A new tag with a key identical to that of an existing tag overwrites the existing tag.</p> </li> <li> <p>Tagging actions are limited to 10 TPS per AWS account, per AWS region. If your application requires a higher throughput, file a <a href="https://console.aws.amazon.com/support/home#/case/create?issueType=technical">technical support request</a>.</p> </li> </ul></p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>>;

    /// <p>Deletes a subscription. If the subscription requires authentication for deletion, only the owner of the subscription or the topic's owner can unsubscribe, and an AWS signature is required. If the <code>Unsubscribe</code> call does not require authentication and the requester is not the subscription owner, a final cancellation message is delivered to the endpoint, so that the endpoint owner can easily resubscribe to the topic if the <code>Unsubscribe</code> request was unintended.</p> <p>This action is throttled at 100 transactions per second (TPS).</p>
    async fn unsubscribe(
        &self,
        input: UnsubscribeInput,
    ) -> Result<(), RusotoError<UnsubscribeError>>;

    /// <p>Remove tags from the specified Amazon SNS topic. For an overview, see <a href="https://docs.aws.amazon.com/sns/latest/dg/sns-tags.html">Amazon SNS Tags</a> in the <i>Amazon SNS Developer Guide</i>.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>>;
}
/// A client for the Amazon SNS API.
#[derive(Clone)]
pub struct SnsClient {
    client: Client,
    region: region::Region,
}

impl SnsClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> SnsClient {
        SnsClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> SnsClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        SnsClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> SnsClient {
        SnsClient { client, region }
    }
}

#[async_trait]
impl Sns for SnsClient {
    /// <p>Adds a statement to a topic's access control policy, granting access for the specified AWS accounts to the specified actions.</p>
    async fn add_permission(
        &self,
        input: AddPermissionInput,
    ) -> Result<(), RusotoError<AddPermissionError>> {
        let mut request = SignedRequest::new("POST", "sns", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "AddPermission");
        params.put("Version", "2010-03-31");
        AddPermissionInputSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(AddPermissionError::from_response(response));
        }

        std::mem::drop(response);
        Ok(())
    }

    /// <p>Accepts a phone number and indicates whether the phone holder has opted out of receiving SMS messages from your account. You cannot send SMS messages to a number that is opted out.</p> <p>To resume sending messages, you can opt in the number by using the <code>OptInPhoneNumber</code> action.</p>
    async fn check_if_phone_number_is_opted_out(
        &self,
        input: CheckIfPhoneNumberIsOptedOutInput,
    ) -> Result<CheckIfPhoneNumberIsOptedOutResponse, RusotoError<CheckIfPhoneNumberIsOptedOutError>>
    {
        let mut request = SignedRequest::new("POST", "sns", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CheckIfPhoneNumberIsOptedOut");
        params.put("Version", "2010-03-31");
        CheckIfPhoneNumberIsOptedOutInputSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(CheckIfPhoneNumberIsOptedOutError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = CheckIfPhoneNumberIsOptedOutResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = CheckIfPhoneNumberIsOptedOutResponseDeserializer::deserialize(
                "CheckIfPhoneNumberIsOptedOutResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Verifies an endpoint owner's intent to receive messages by validating the token sent to the endpoint by an earlier <code>Subscribe</code> action. If the token is valid, the action creates a new subscription and returns its Amazon Resource Name (ARN). This call requires an AWS signature only when the <code>AuthenticateOnUnsubscribe</code> flag is set to "true".</p>
    async fn confirm_subscription(
        &self,
        input: ConfirmSubscriptionInput,
    ) -> Result<ConfirmSubscriptionResponse, RusotoError<ConfirmSubscriptionError>> {
        let mut request = SignedRequest::new("POST", "sns", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ConfirmSubscription");
        params.put("Version", "2010-03-31");
        ConfirmSubscriptionInputSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(ConfirmSubscriptionError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = ConfirmSubscriptionResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = ConfirmSubscriptionResponseDeserializer::deserialize(
                "ConfirmSubscriptionResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Creates a platform application object for one of the supported push notification services, such as APNS and FCM, to which devices and mobile apps may register. You must specify PlatformPrincipal and PlatformCredential attributes when using the <code>CreatePlatformApplication</code> action. The PlatformPrincipal is received from the notification service. For APNS/APNS_SANDBOX, PlatformPrincipal is "SSL certificate". For FCM, PlatformPrincipal is not applicable. For ADM, PlatformPrincipal is "client id". The PlatformCredential is also received from the notification service. For WNS, PlatformPrincipal is "Package Security Identifier". For MPNS, PlatformPrincipal is "TLS certificate". For Baidu, PlatformPrincipal is "API key".</p> <p>For APNS/APNS_SANDBOX, PlatformCredential is "private key". For FCM, PlatformCredential is "API key". For ADM, PlatformCredential is "client secret". For WNS, PlatformCredential is "secret key". For MPNS, PlatformCredential is "private key". For Baidu, PlatformCredential is "secret key". The PlatformApplicationArn that is returned when using <code>CreatePlatformApplication</code> is then used as an attribute for the <code>CreatePlatformEndpoint</code> action.</p>
    async fn create_platform_application(
        &self,
        input: CreatePlatformApplicationInput,
    ) -> Result<CreatePlatformApplicationResponse, RusotoError<CreatePlatformApplicationError>>
    {
        let mut request = SignedRequest::new("POST", "sns", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CreatePlatformApplication");
        params.put("Version", "2010-03-31");
        CreatePlatformApplicationInputSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(CreatePlatformApplicationError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = CreatePlatformApplicationResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = CreatePlatformApplicationResponseDeserializer::deserialize(
                "CreatePlatformApplicationResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Creates an endpoint for a device and mobile app on one of the supported push notification services, such as FCM and APNS. <code>CreatePlatformEndpoint</code> requires the PlatformApplicationArn that is returned from <code>CreatePlatformApplication</code>. The EndpointArn that is returned when using <code>CreatePlatformEndpoint</code> can then be used by the <code>Publish</code> action to send a message to a mobile app or by the <code>Subscribe</code> action for subscription to a topic. The <code>CreatePlatformEndpoint</code> action is idempotent, so if the requester already owns an endpoint with the same device token and attributes, that endpoint's ARN is returned without creating a new endpoint. For more information, see <a href="https://docs.aws.amazon.com/sns/latest/dg/SNSMobilePush.html">Using Amazon SNS Mobile Push Notifications</a>. </p> <p>When using <code>CreatePlatformEndpoint</code> with Baidu, two attributes must be provided: ChannelId and UserId. The token field must also contain the ChannelId. For more information, see <a href="https://docs.aws.amazon.com/sns/latest/dg/SNSMobilePushBaiduEndpoint.html">Creating an Amazon SNS Endpoint for Baidu</a>. </p>
    async fn create_platform_endpoint(
        &self,
        input: CreatePlatformEndpointInput,
    ) -> Result<CreateEndpointResponse, RusotoError<CreatePlatformEndpointError>> {
        let mut request = SignedRequest::new("POST", "sns", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CreatePlatformEndpoint");
        params.put("Version", "2010-03-31");
        CreatePlatformEndpointInputSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(CreatePlatformEndpointError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = CreateEndpointResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = CreateEndpointResponseDeserializer::deserialize(
                "CreatePlatformEndpointResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Creates a topic to which notifications can be published. Users can create at most 100,000 topics. For more information, see <a href="http://aws.amazon.com/sns/">https://aws.amazon.com/sns</a>. This action is idempotent, so if the requester already owns a topic with the specified name, that topic's ARN is returned without creating a new topic.</p>
    async fn create_topic(
        &self,
        input: CreateTopicInput,
    ) -> Result<CreateTopicResponse, RusotoError<CreateTopicError>> {
        let mut request = SignedRequest::new("POST", "sns", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CreateTopic");
        params.put("Version", "2010-03-31");
        CreateTopicInputSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(CreateTopicError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = CreateTopicResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = CreateTopicResponseDeserializer::deserialize("CreateTopicResult", &mut stack)?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Deletes the endpoint for a device and mobile app from Amazon SNS. This action is idempotent. For more information, see <a href="https://docs.aws.amazon.com/sns/latest/dg/SNSMobilePush.html">Using Amazon SNS Mobile Push Notifications</a>. </p> <p>When you delete an endpoint that is also subscribed to a topic, then you must also unsubscribe the endpoint from the topic.</p>
    async fn delete_endpoint(
        &self,
        input: DeleteEndpointInput,
    ) -> Result<(), RusotoError<DeleteEndpointError>> {
        let mut request = SignedRequest::new("POST", "sns", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteEndpoint");
        params.put("Version", "2010-03-31");
        DeleteEndpointInputSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DeleteEndpointError::from_response(response));
        }

        std::mem::drop(response);
        Ok(())
    }

    /// <p>Deletes a platform application object for one of the supported push notification services, such as APNS and FCM. For more information, see <a href="https://docs.aws.amazon.com/sns/latest/dg/SNSMobilePush.html">Using Amazon SNS Mobile Push Notifications</a>. </p>
    async fn delete_platform_application(
        &self,
        input: DeletePlatformApplicationInput,
    ) -> Result<(), RusotoError<DeletePlatformApplicationError>> {
        let mut request = SignedRequest::new("POST", "sns", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeletePlatformApplication");
        params.put("Version", "2010-03-31");
        DeletePlatformApplicationInputSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DeletePlatformApplicationError::from_response(response));
        }

        std::mem::drop(response);
        Ok(())
    }

    /// <p>Deletes a topic and all its subscriptions. Deleting a topic might prevent some messages previously sent to the topic from being delivered to subscribers. This action is idempotent, so deleting a topic that does not exist does not result in an error.</p>
    async fn delete_topic(
        &self,
        input: DeleteTopicInput,
    ) -> Result<(), RusotoError<DeleteTopicError>> {
        let mut request = SignedRequest::new("POST", "sns", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteTopic");
        params.put("Version", "2010-03-31");
        DeleteTopicInputSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DeleteTopicError::from_response(response));
        }

        std::mem::drop(response);
        Ok(())
    }

    /// <p>Retrieves the endpoint attributes for a device on one of the supported push notification services, such as FCM and APNS. For more information, see <a href="https://docs.aws.amazon.com/sns/latest/dg/SNSMobilePush.html">Using Amazon SNS Mobile Push Notifications</a>. </p>
    async fn get_endpoint_attributes(
        &self,
        input: GetEndpointAttributesInput,
    ) -> Result<GetEndpointAttributesResponse, RusotoError<GetEndpointAttributesError>> {
        let mut request = SignedRequest::new("POST", "sns", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "GetEndpointAttributes");
        params.put("Version", "2010-03-31");
        GetEndpointAttributesInputSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(GetEndpointAttributesError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = GetEndpointAttributesResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = GetEndpointAttributesResponseDeserializer::deserialize(
                "GetEndpointAttributesResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Retrieves the attributes of the platform application object for the supported push notification services, such as APNS and FCM. For more information, see <a href="https://docs.aws.amazon.com/sns/latest/dg/SNSMobilePush.html">Using Amazon SNS Mobile Push Notifications</a>. </p>
    async fn get_platform_application_attributes(
        &self,
        input: GetPlatformApplicationAttributesInput,
    ) -> Result<
        GetPlatformApplicationAttributesResponse,
        RusotoError<GetPlatformApplicationAttributesError>,
    > {
        let mut request = SignedRequest::new("POST", "sns", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "GetPlatformApplicationAttributes");
        params.put("Version", "2010-03-31");
        GetPlatformApplicationAttributesInputSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(GetPlatformApplicationAttributesError::from_response(
                response,
            ));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = GetPlatformApplicationAttributesResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = GetPlatformApplicationAttributesResponseDeserializer::deserialize(
                "GetPlatformApplicationAttributesResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Returns the settings for sending SMS messages from your account.</p> <p>These settings are set with the <code>SetSMSAttributes</code> action.</p>
    async fn get_sms_attributes(
        &self,
        input: GetSMSAttributesInput,
    ) -> Result<GetSMSAttributesResponse, RusotoError<GetSMSAttributesError>> {
        let mut request = SignedRequest::new("POST", "sns", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "GetSMSAttributes");
        params.put("Version", "2010-03-31");
        GetSMSAttributesInputSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(GetSMSAttributesError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = GetSMSAttributesResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = GetSMSAttributesResponseDeserializer::deserialize(
                "GetSMSAttributesResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Returns all of the properties of a subscription.</p>
    async fn get_subscription_attributes(
        &self,
        input: GetSubscriptionAttributesInput,
    ) -> Result<GetSubscriptionAttributesResponse, RusotoError<GetSubscriptionAttributesError>>
    {
        let mut request = SignedRequest::new("POST", "sns", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "GetSubscriptionAttributes");
        params.put("Version", "2010-03-31");
        GetSubscriptionAttributesInputSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(GetSubscriptionAttributesError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = GetSubscriptionAttributesResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = GetSubscriptionAttributesResponseDeserializer::deserialize(
                "GetSubscriptionAttributesResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Returns all of the properties of a topic. Topic properties returned might differ based on the authorization of the user.</p>
    async fn get_topic_attributes(
        &self,
        input: GetTopicAttributesInput,
    ) -> Result<GetTopicAttributesResponse, RusotoError<GetTopicAttributesError>> {
        let mut request = SignedRequest::new("POST", "sns", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "GetTopicAttributes");
        params.put("Version", "2010-03-31");
        GetTopicAttributesInputSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(GetTopicAttributesError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = GetTopicAttributesResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = GetTopicAttributesResponseDeserializer::deserialize(
                "GetTopicAttributesResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Lists the endpoints and endpoint attributes for devices in a supported push notification service, such as FCM and APNS. The results for <code>ListEndpointsByPlatformApplication</code> are paginated and return a limited list of endpoints, up to 100. If additional records are available after the first page results, then a NextToken string will be returned. To receive the next page, you call <code>ListEndpointsByPlatformApplication</code> again using the NextToken string received from the previous call. When there are no more records to return, NextToken will be null. For more information, see <a href="https://docs.aws.amazon.com/sns/latest/dg/SNSMobilePush.html">Using Amazon SNS Mobile Push Notifications</a>. </p> <p>This action is throttled at 30 transactions per second (TPS).</p>
    async fn list_endpoints_by_platform_application(
        &self,
        input: ListEndpointsByPlatformApplicationInput,
    ) -> Result<
        ListEndpointsByPlatformApplicationResponse,
        RusotoError<ListEndpointsByPlatformApplicationError>,
    > {
        let mut request = SignedRequest::new("POST", "sns", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ListEndpointsByPlatformApplication");
        params.put("Version", "2010-03-31");
        ListEndpointsByPlatformApplicationInputSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(ListEndpointsByPlatformApplicationError::from_response(
                response,
            ));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = ListEndpointsByPlatformApplicationResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = ListEndpointsByPlatformApplicationResponseDeserializer::deserialize(
                "ListEndpointsByPlatformApplicationResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Returns a list of phone numbers that are opted out, meaning you cannot send SMS messages to them.</p> <p>The results for <code>ListPhoneNumbersOptedOut</code> are paginated, and each page returns up to 100 phone numbers. If additional phone numbers are available after the first page of results, then a <code>NextToken</code> string will be returned. To receive the next page, you call <code>ListPhoneNumbersOptedOut</code> again using the <code>NextToken</code> string received from the previous call. When there are no more records to return, <code>NextToken</code> will be null.</p>
    async fn list_phone_numbers_opted_out(
        &self,
        input: ListPhoneNumbersOptedOutInput,
    ) -> Result<ListPhoneNumbersOptedOutResponse, RusotoError<ListPhoneNumbersOptedOutError>> {
        let mut request = SignedRequest::new("POST", "sns", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ListPhoneNumbersOptedOut");
        params.put("Version", "2010-03-31");
        ListPhoneNumbersOptedOutInputSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(ListPhoneNumbersOptedOutError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = ListPhoneNumbersOptedOutResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = ListPhoneNumbersOptedOutResponseDeserializer::deserialize(
                "ListPhoneNumbersOptedOutResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Lists the platform application objects for the supported push notification services, such as APNS and FCM. The results for <code>ListPlatformApplications</code> are paginated and return a limited list of applications, up to 100. If additional records are available after the first page results, then a NextToken string will be returned. To receive the next page, you call <code>ListPlatformApplications</code> using the NextToken string received from the previous call. When there are no more records to return, NextToken will be null. For more information, see <a href="https://docs.aws.amazon.com/sns/latest/dg/SNSMobilePush.html">Using Amazon SNS Mobile Push Notifications</a>. </p> <p>This action is throttled at 15 transactions per second (TPS).</p>
    async fn list_platform_applications(
        &self,
        input: ListPlatformApplicationsInput,
    ) -> Result<ListPlatformApplicationsResponse, RusotoError<ListPlatformApplicationsError>> {
        let mut request = SignedRequest::new("POST", "sns", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ListPlatformApplications");
        params.put("Version", "2010-03-31");
        ListPlatformApplicationsInputSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(ListPlatformApplicationsError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = ListPlatformApplicationsResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = ListPlatformApplicationsResponseDeserializer::deserialize(
                "ListPlatformApplicationsResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Returns a list of the requester's subscriptions. Each call returns a limited list of subscriptions, up to 100. If there are more subscriptions, a <code>NextToken</code> is also returned. Use the <code>NextToken</code> parameter in a new <code>ListSubscriptions</code> call to get further results.</p> <p>This action is throttled at 30 transactions per second (TPS).</p>
    async fn list_subscriptions(
        &self,
        input: ListSubscriptionsInput,
    ) -> Result<ListSubscriptionsResponse, RusotoError<ListSubscriptionsError>> {
        let mut request = SignedRequest::new("POST", "sns", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ListSubscriptions");
        params.put("Version", "2010-03-31");
        ListSubscriptionsInputSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(ListSubscriptionsError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = ListSubscriptionsResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = ListSubscriptionsResponseDeserializer::deserialize(
                "ListSubscriptionsResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Returns a list of the subscriptions to a specific topic. Each call returns a limited list of subscriptions, up to 100. If there are more subscriptions, a <code>NextToken</code> is also returned. Use the <code>NextToken</code> parameter in a new <code>ListSubscriptionsByTopic</code> call to get further results.</p> <p>This action is throttled at 30 transactions per second (TPS).</p>
    async fn list_subscriptions_by_topic(
        &self,
        input: ListSubscriptionsByTopicInput,
    ) -> Result<ListSubscriptionsByTopicResponse, RusotoError<ListSubscriptionsByTopicError>> {
        let mut request = SignedRequest::new("POST", "sns", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ListSubscriptionsByTopic");
        params.put("Version", "2010-03-31");
        ListSubscriptionsByTopicInputSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(ListSubscriptionsByTopicError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = ListSubscriptionsByTopicResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = ListSubscriptionsByTopicResponseDeserializer::deserialize(
                "ListSubscriptionsByTopicResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>List all tags added to the specified Amazon SNS topic. For an overview, see <a href="https://docs.aws.amazon.com/sns/latest/dg/sns-tags.html">Amazon SNS Tags</a> in the <i>Amazon Simple Notification Service Developer Guide</i>.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>> {
        let mut request = SignedRequest::new("POST", "sns", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ListTagsForResource");
        params.put("Version", "2010-03-31");
        ListTagsForResourceRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(ListTagsForResourceError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = ListTagsForResourceResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = ListTagsForResourceResponseDeserializer::deserialize(
                "ListTagsForResourceResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Returns a list of the requester's topics. Each call returns a limited list of topics, up to 100. If there are more topics, a <code>NextToken</code> is also returned. Use the <code>NextToken</code> parameter in a new <code>ListTopics</code> call to get further results.</p> <p>This action is throttled at 30 transactions per second (TPS).</p>
    async fn list_topics(
        &self,
        input: ListTopicsInput,
    ) -> Result<ListTopicsResponse, RusotoError<ListTopicsError>> {
        let mut request = SignedRequest::new("POST", "sns", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ListTopics");
        params.put("Version", "2010-03-31");
        ListTopicsInputSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(ListTopicsError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = ListTopicsResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = ListTopicsResponseDeserializer::deserialize("ListTopicsResult", &mut stack)?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Use this request to opt in a phone number that is opted out, which enables you to resume sending SMS messages to the number.</p> <p>You can opt in a phone number only once every 30 days.</p>
    async fn opt_in_phone_number(
        &self,
        input: OptInPhoneNumberInput,
    ) -> Result<OptInPhoneNumberResponse, RusotoError<OptInPhoneNumberError>> {
        let mut request = SignedRequest::new("POST", "sns", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "OptInPhoneNumber");
        params.put("Version", "2010-03-31");
        OptInPhoneNumberInputSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(OptInPhoneNumberError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = OptInPhoneNumberResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = OptInPhoneNumberResponseDeserializer::deserialize(
                "OptInPhoneNumberResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Sends a message to an Amazon SNS topic or sends a text message (SMS message) directly to a phone number. </p> <p>If you send a message to a topic, Amazon SNS delivers the message to each endpoint that is subscribed to the topic. The format of the message depends on the notification protocol for each subscribed endpoint.</p> <p>When a <code>messageId</code> is returned, the message has been saved and Amazon SNS will attempt to deliver it shortly.</p> <p>To use the <code>Publish</code> action for sending a message to a mobile endpoint, such as an app on a Kindle device or mobile phone, you must specify the EndpointArn for the TargetArn parameter. The EndpointArn is returned when making a call with the <code>CreatePlatformEndpoint</code> action. </p> <p>For more information about formatting messages, see <a href="https://docs.aws.amazon.com/sns/latest/dg/mobile-push-send-custommessage.html">Send Custom Platform-Specific Payloads in Messages to Mobile Devices</a>. </p>
    async fn publish(
        &self,
        input: PublishInput,
    ) -> Result<PublishResponse, RusotoError<PublishError>> {
        let mut request = SignedRequest::new("POST", "sns", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "Publish");
        params.put("Version", "2010-03-31");
        PublishInputSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(PublishError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = PublishResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = PublishResponseDeserializer::deserialize("PublishResult", &mut stack)?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Removes a statement from a topic's access control policy.</p>
    async fn remove_permission(
        &self,
        input: RemovePermissionInput,
    ) -> Result<(), RusotoError<RemovePermissionError>> {
        let mut request = SignedRequest::new("POST", "sns", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "RemovePermission");
        params.put("Version", "2010-03-31");
        RemovePermissionInputSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(RemovePermissionError::from_response(response));
        }

        std::mem::drop(response);
        Ok(())
    }

    /// <p>Sets the attributes for an endpoint for a device on one of the supported push notification services, such as FCM and APNS. For more information, see <a href="https://docs.aws.amazon.com/sns/latest/dg/SNSMobilePush.html">Using Amazon SNS Mobile Push Notifications</a>. </p>
    async fn set_endpoint_attributes(
        &self,
        input: SetEndpointAttributesInput,
    ) -> Result<(), RusotoError<SetEndpointAttributesError>> {
        let mut request = SignedRequest::new("POST", "sns", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "SetEndpointAttributes");
        params.put("Version", "2010-03-31");
        SetEndpointAttributesInputSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(SetEndpointAttributesError::from_response(response));
        }

        std::mem::drop(response);
        Ok(())
    }

    /// <p>Sets the attributes of the platform application object for the supported push notification services, such as APNS and FCM. For more information, see <a href="https://docs.aws.amazon.com/sns/latest/dg/SNSMobilePush.html">Using Amazon SNS Mobile Push Notifications</a>. For information on configuring attributes for message delivery status, see <a href="https://docs.aws.amazon.com/sns/latest/dg/sns-msg-status.html">Using Amazon SNS Application Attributes for Message Delivery Status</a>. </p>
    async fn set_platform_application_attributes(
        &self,
        input: SetPlatformApplicationAttributesInput,
    ) -> Result<(), RusotoError<SetPlatformApplicationAttributesError>> {
        let mut request = SignedRequest::new("POST", "sns", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "SetPlatformApplicationAttributes");
        params.put("Version", "2010-03-31");
        SetPlatformApplicationAttributesInputSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(SetPlatformApplicationAttributesError::from_response(
                response,
            ));
        }

        std::mem::drop(response);
        Ok(())
    }

    /// <p>Use this request to set the default settings for sending SMS messages and receiving daily SMS usage reports.</p> <p>You can override some of these settings for a single message when you use the <code>Publish</code> action with the <code>MessageAttributes.entry.N</code> parameter. For more information, see <a href="https://docs.aws.amazon.com/sns/latest/dg/sms_publish-to-phone.html">Sending an SMS Message</a> in the <i>Amazon SNS Developer Guide</i>.</p>
    async fn set_sms_attributes(
        &self,
        input: SetSMSAttributesInput,
    ) -> Result<SetSMSAttributesResponse, RusotoError<SetSMSAttributesError>> {
        let mut request = SignedRequest::new("POST", "sns", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "SetSMSAttributes");
        params.put("Version", "2010-03-31");
        SetSMSAttributesInputSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(SetSMSAttributesError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = SetSMSAttributesResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = SetSMSAttributesResponseDeserializer::deserialize(
                "SetSMSAttributesResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Allows a subscription owner to set an attribute of the subscription to a new value.</p>
    async fn set_subscription_attributes(
        &self,
        input: SetSubscriptionAttributesInput,
    ) -> Result<(), RusotoError<SetSubscriptionAttributesError>> {
        let mut request = SignedRequest::new("POST", "sns", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "SetSubscriptionAttributes");
        params.put("Version", "2010-03-31");
        SetSubscriptionAttributesInputSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(SetSubscriptionAttributesError::from_response(response));
        }

        std::mem::drop(response);
        Ok(())
    }

    /// <p>Allows a topic owner to set an attribute of the topic to a new value.</p>
    async fn set_topic_attributes(
        &self,
        input: SetTopicAttributesInput,
    ) -> Result<(), RusotoError<SetTopicAttributesError>> {
        let mut request = SignedRequest::new("POST", "sns", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "SetTopicAttributes");
        params.put("Version", "2010-03-31");
        SetTopicAttributesInputSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(SetTopicAttributesError::from_response(response));
        }

        std::mem::drop(response);
        Ok(())
    }

    /// <p>Prepares to subscribe an endpoint by sending the endpoint a confirmation message. To actually create a subscription, the endpoint owner must call the <code>ConfirmSubscription</code> action with the token from the confirmation message. Confirmation tokens are valid for three days.</p> <p>This action is throttled at 100 transactions per second (TPS).</p>
    async fn subscribe(
        &self,
        input: SubscribeInput,
    ) -> Result<SubscribeResponse, RusotoError<SubscribeError>> {
        let mut request = SignedRequest::new("POST", "sns", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "Subscribe");
        params.put("Version", "2010-03-31");
        SubscribeInputSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(SubscribeError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = SubscribeResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = SubscribeResponseDeserializer::deserialize("SubscribeResult", &mut stack)?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p><p>Add tags to the specified Amazon SNS topic. For an overview, see <a href="https://docs.aws.amazon.com/sns/latest/dg/sns-tags.html">Amazon SNS Tags</a> in the <i>Amazon SNS Developer Guide</i>.</p> <p>When you use topic tags, keep the following guidelines in mind:</p> <ul> <li> <p>Adding more than 50 tags to a topic isn&#39;t recommended.</p> </li> <li> <p>Tags don&#39;t have any semantic meaning. Amazon SNS interprets tags as character strings.</p> </li> <li> <p>Tags are case-sensitive.</p> </li> <li> <p>A new tag with a key identical to that of an existing tag overwrites the existing tag.</p> </li> <li> <p>Tagging actions are limited to 10 TPS per AWS account, per AWS region. If your application requires a higher throughput, file a <a href="https://console.aws.amazon.com/support/home#/case/create?issueType=technical">technical support request</a>.</p> </li> </ul></p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>> {
        let mut request = SignedRequest::new("POST", "sns", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "TagResource");
        params.put("Version", "2010-03-31");
        TagResourceRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(TagResourceError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = TagResourceResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = TagResourceResponseDeserializer::deserialize("TagResourceResult", &mut stack)?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Deletes a subscription. If the subscription requires authentication for deletion, only the owner of the subscription or the topic's owner can unsubscribe, and an AWS signature is required. If the <code>Unsubscribe</code> call does not require authentication and the requester is not the subscription owner, a final cancellation message is delivered to the endpoint, so that the endpoint owner can easily resubscribe to the topic if the <code>Unsubscribe</code> request was unintended.</p> <p>This action is throttled at 100 transactions per second (TPS).</p>
    async fn unsubscribe(
        &self,
        input: UnsubscribeInput,
    ) -> Result<(), RusotoError<UnsubscribeError>> {
        let mut request = SignedRequest::new("POST", "sns", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "Unsubscribe");
        params.put("Version", "2010-03-31");
        UnsubscribeInputSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(UnsubscribeError::from_response(response));
        }

        std::mem::drop(response);
        Ok(())
    }

    /// <p>Remove tags from the specified Amazon SNS topic. For an overview, see <a href="https://docs.aws.amazon.com/sns/latest/dg/sns-tags.html">Amazon SNS Tags</a> in the <i>Amazon SNS Developer Guide</i>.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>> {
        let mut request = SignedRequest::new("POST", "sns", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "UntagResource");
        params.put("Version", "2010-03-31");
        UntagResourceRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(UntagResourceError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = UntagResourceResponse::default();
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
                UntagResourceResponseDeserializer::deserialize("UntagResourceResult", &mut stack)?;
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
    async fn test_parse_error_sns_delete_topic() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/error",
            "sns-delete-topic.xml",
        );
        let mock = MockRequestDispatcher::with_status(400).with_body(&mock_response);
        let client = SnsClient::new_with(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = DeleteTopicInput::default();
        let result = client.delete_topic(request).await;
        assert!(!result.is_ok(), "parse error: {:?}", result);
    }

    #[tokio::test]
    async fn test_parse_valid_sns_add_permission() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "sns-add-permission.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = SnsClient::new_with(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = AddPermissionInput::default();
        let result = client.add_permission(request).await;
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[tokio::test]
    async fn test_parse_valid_sns_confirm_subscription() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "sns-confirm-subscription.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = SnsClient::new_with(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = ConfirmSubscriptionInput::default();
        let result = client.confirm_subscription(request).await;
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[tokio::test]
    async fn test_parse_valid_sns_create_topic() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "sns-create-topic.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = SnsClient::new_with(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = CreateTopicInput::default();
        let result = client.create_topic(request).await;
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[tokio::test]
    async fn test_parse_valid_sns_get_subscription_attributes() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "sns-get-subscription-attributes.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = SnsClient::new_with(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = GetSubscriptionAttributesInput::default();
        let result = client.get_subscription_attributes(request).await;
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[tokio::test]
    async fn test_parse_valid_sns_get_topic_attributes() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "sns-get-topic-attributes.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = SnsClient::new_with(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = GetTopicAttributesInput::default();
        let result = client.get_topic_attributes(request).await;
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[tokio::test]
    async fn test_parse_valid_sns_list_subscriptions_by_topic() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "sns-list-subscriptions-by-topic.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = SnsClient::new_with(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = ListSubscriptionsByTopicInput::default();
        let result = client.list_subscriptions_by_topic(request).await;
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[tokio::test]
    async fn test_parse_valid_sns_list_subscriptions() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "sns-list-subscriptions.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = SnsClient::new_with(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = ListSubscriptionsInput::default();
        let result = client.list_subscriptions(request).await;
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[tokio::test]
    async fn test_parse_valid_sns_list_topics() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "sns-list-topics.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = SnsClient::new_with(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = ListTopicsInput::default();
        let result = client.list_topics(request).await;
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[tokio::test]
    async fn test_parse_valid_sns_publish() {
        let mock_response =
            MockResponseReader::read_response("test_resources/generated/valid", "sns-publish.xml");
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = SnsClient::new_with(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = PublishInput::default();
        let result = client.publish(request).await;
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[tokio::test]
    async fn test_parse_valid_sns_subscribe() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "sns-subscribe.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = SnsClient::new_with(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = SubscribeInput::default();
        let result = client.subscribe(request).await;
        assert!(result.is_ok(), "parse error: {:?}", result);
    }
}
