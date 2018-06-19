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
/// <p>Information about the <code>AccessLog</code> attribute.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct AccessLog {
    /// <p>The interval for publishing the access logs. You can specify an interval of either 5 minutes or 60 minutes.</p> <p>Default: 60 minutes</p>
    pub emit_interval: Option<i64>,
    /// <p>Specifies whether access logs are enabled for the load balancer.</p>
    pub enabled: bool,
    /// <p>The name of the Amazon S3 bucket where the access logs are stored.</p>
    pub s3_bucket_name: Option<String>,
    /// <p>The logical hierarchy you created for your Amazon S3 bucket, for example <code>my-bucket-prefix/prod</code>. If the prefix is not provided, the log is placed at the root level of the bucket.</p>
    pub s3_bucket_prefix: Option<String>,
}

struct AccessLogDeserializer;
impl AccessLogDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AccessLog, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = AccessLog::default();

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
                    "EmitInterval" => {
                        obj.emit_interval = Some(try!(AccessLogIntervalDeserializer::deserialize(
                            "EmitInterval",
                            stack
                        )));
                    }
                    "Enabled" => {
                        obj.enabled =
                            try!(AccessLogEnabledDeserializer::deserialize("Enabled", stack));
                    }
                    "S3BucketName" => {
                        obj.s3_bucket_name = Some(try!(S3BucketNameDeserializer::deserialize(
                            "S3BucketName",
                            stack
                        )));
                    }
                    "S3BucketPrefix" => {
                        obj.s3_bucket_prefix = Some(try!(
                            AccessLogPrefixDeserializer::deserialize("S3BucketPrefix", stack)
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

/// Serialize `AccessLog` contents to a `SignedRequest`.
struct AccessLogSerializer;
impl AccessLogSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &AccessLog) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.emit_interval {
            params.put(
                &format!("{}{}", prefix, "EmitInterval"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        params.put(
            &format!("{}{}", prefix, "Enabled"),
            &obj.enabled.to_string().replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.s3_bucket_name {
            params.put(
                &format!("{}{}", prefix, "S3BucketName"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.s3_bucket_prefix {
            params.put(
                &format!("{}{}", prefix, "S3BucketPrefix"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

struct AccessLogEnabledDeserializer;
impl AccessLogEnabledDeserializer {
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
struct AccessLogIntervalDeserializer;
impl AccessLogIntervalDeserializer {
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
struct AccessLogPrefixDeserializer;
impl AccessLogPrefixDeserializer {
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
struct AccessPointNameDeserializer;
impl AccessPointNameDeserializer {
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
struct AccessPointPortDeserializer;
impl AccessPointPortDeserializer {
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
/// <p>Contains the parameters for EnableAvailabilityZonesForLoadBalancer.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct AddAvailabilityZonesInput {
    /// <p>The Availability Zones. These must be in the same region as the load balancer.</p>
    pub availability_zones: Vec<String>,
    /// <p>The name of the load balancer.</p>
    pub load_balancer_name: String,
}

/// Serialize `AddAvailabilityZonesInput` contents to a `SignedRequest`.
struct AddAvailabilityZonesInputSerializer;
impl AddAvailabilityZonesInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &AddAvailabilityZonesInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        AvailabilityZonesSerializer::serialize(
            params,
            &format!("{}{}", prefix, "AvailabilityZones"),
            &obj.availability_zones,
        );
        params.put(
            &format!("{}{}", prefix, "LoadBalancerName"),
            &obj.load_balancer_name.replace("+", "%2B"),
        );
    }
}

/// <p>Contains the output of EnableAvailabilityZonesForLoadBalancer.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct AddAvailabilityZonesOutput {
    /// <p>The updated list of Availability Zones for the load balancer.</p>
    pub availability_zones: Option<Vec<String>>,
}

struct AddAvailabilityZonesOutputDeserializer;
impl AddAvailabilityZonesOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AddAvailabilityZonesOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = AddAvailabilityZonesOutput::default();

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
                    "AvailabilityZones" => {
                        obj.availability_zones = Some(try!(
                            AvailabilityZonesDeserializer::deserialize("AvailabilityZones", stack)
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
/// <p>Contains the parameters for AddTags.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct AddTagsInput {
    /// <p>The name of the load balancer. You can specify one load balancer only.</p>
    pub load_balancer_names: Vec<String>,
    /// <p>The tags.</p>
    pub tags: Vec<Tag>,
}

/// Serialize `AddTagsInput` contents to a `SignedRequest`.
struct AddTagsInputSerializer;
impl AddTagsInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &AddTagsInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        LoadBalancerNamesSerializer::serialize(
            params,
            &format!("{}{}", prefix, "LoadBalancerNames"),
            &obj.load_balancer_names,
        );
        TagListSerializer::serialize(params, &format!("{}{}", prefix, "Tags"), &obj.tags);
    }
}

/// <p>Contains the output of AddTags.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct AddTagsOutput {}

struct AddTagsOutputDeserializer;
impl AddTagsOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AddTagsOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let obj = AddTagsOutput::default();

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>This data type is reserved.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct AdditionalAttribute {
    /// <p>This parameter is reserved.</p>
    pub key: Option<String>,
    /// <p>This parameter is reserved.</p>
    pub value: Option<String>,
}

struct AdditionalAttributeDeserializer;
impl AdditionalAttributeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AdditionalAttribute, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = AdditionalAttribute::default();

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
                    "Key" => {
                        obj.key = Some(try!(AdditionalAttributeKeyDeserializer::deserialize(
                            "Key", stack
                        )));
                    }
                    "Value" => {
                        obj.value = Some(try!(AdditionalAttributeValueDeserializer::deserialize(
                            "Value", stack
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

/// Serialize `AdditionalAttribute` contents to a `SignedRequest`.
struct AdditionalAttributeSerializer;
impl AdditionalAttributeSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &AdditionalAttribute) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.key {
            params.put(
                &format!("{}{}", prefix, "Key"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.value {
            params.put(
                &format!("{}{}", prefix, "Value"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

struct AdditionalAttributeKeyDeserializer;
impl AdditionalAttributeKeyDeserializer {
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
struct AdditionalAttributeValueDeserializer;
impl AdditionalAttributeValueDeserializer {
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
struct AdditionalAttributesDeserializer;
impl AdditionalAttributesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<AdditionalAttribute>, XmlParseError> {
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
                        obj.push(try!(AdditionalAttributeDeserializer::deserialize(
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

/// Serialize `AdditionalAttributes` contents to a `SignedRequest`.
struct AdditionalAttributesSerializer;
impl AdditionalAttributesSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<AdditionalAttribute>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            AdditionalAttributeSerializer::serialize(params, &key, obj);
        }
    }
}

struct AppCookieStickinessPoliciesDeserializer;
impl AppCookieStickinessPoliciesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<AppCookieStickinessPolicy>, XmlParseError> {
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
                        obj.push(try!(AppCookieStickinessPolicyDeserializer::deserialize(
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
/// <p>Information about a policy for application-controlled session stickiness.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct AppCookieStickinessPolicy {
    /// <p>The name of the application cookie used for stickiness.</p>
    pub cookie_name: Option<String>,
    /// <p>The mnemonic name for the policy being created. The name must be unique within a set of policies for this load balancer.</p>
    pub policy_name: Option<String>,
}

struct AppCookieStickinessPolicyDeserializer;
impl AppCookieStickinessPolicyDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AppCookieStickinessPolicy, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = AppCookieStickinessPolicy::default();

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
                    "CookieName" => {
                        obj.cookie_name = Some(try!(CookieNameDeserializer::deserialize(
                            "CookieName",
                            stack
                        )));
                    }
                    "PolicyName" => {
                        obj.policy_name = Some(try!(PolicyNameDeserializer::deserialize(
                            "PolicyName",
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
/// <p>Contains the parameters for ApplySecurityGroupsToLoadBalancer.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ApplySecurityGroupsToLoadBalancerInput {
    /// <p>The name of the load balancer.</p>
    pub load_balancer_name: String,
    /// <p>The IDs of the security groups to associate with the load balancer. Note that you cannot specify the name of the security group.</p>
    pub security_groups: Vec<String>,
}

/// Serialize `ApplySecurityGroupsToLoadBalancerInput` contents to a `SignedRequest`.
struct ApplySecurityGroupsToLoadBalancerInputSerializer;
impl ApplySecurityGroupsToLoadBalancerInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ApplySecurityGroupsToLoadBalancerInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "LoadBalancerName"),
            &obj.load_balancer_name.replace("+", "%2B"),
        );
        SecurityGroupsSerializer::serialize(
            params,
            &format!("{}{}", prefix, "SecurityGroups"),
            &obj.security_groups,
        );
    }
}

/// <p>Contains the output of ApplySecurityGroupsToLoadBalancer.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ApplySecurityGroupsToLoadBalancerOutput {
    /// <p>The IDs of the security groups associated with the load balancer.</p>
    pub security_groups: Option<Vec<String>>,
}

struct ApplySecurityGroupsToLoadBalancerOutputDeserializer;
impl ApplySecurityGroupsToLoadBalancerOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ApplySecurityGroupsToLoadBalancerOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ApplySecurityGroupsToLoadBalancerOutput::default();

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
                    "SecurityGroups" => {
                        obj.security_groups = Some(try!(SecurityGroupsDeserializer::deserialize(
                            "SecurityGroups",
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
/// <p>Contains the parameters for AttachLoaBalancerToSubnets.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct AttachLoadBalancerToSubnetsInput {
    /// <p>The name of the load balancer.</p>
    pub load_balancer_name: String,
    /// <p>The IDs of the subnets to add. You can add only one subnet per Availability Zone.</p>
    pub subnets: Vec<String>,
}

/// Serialize `AttachLoadBalancerToSubnetsInput` contents to a `SignedRequest`.
struct AttachLoadBalancerToSubnetsInputSerializer;
impl AttachLoadBalancerToSubnetsInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &AttachLoadBalancerToSubnetsInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "LoadBalancerName"),
            &obj.load_balancer_name.replace("+", "%2B"),
        );
        SubnetsSerializer::serialize(params, &format!("{}{}", prefix, "Subnets"), &obj.subnets);
    }
}

/// <p>Contains the output of AttachLoadBalancerToSubnets.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct AttachLoadBalancerToSubnetsOutput {
    /// <p>The IDs of the subnets attached to the load balancer.</p>
    pub subnets: Option<Vec<String>>,
}

struct AttachLoadBalancerToSubnetsOutputDeserializer;
impl AttachLoadBalancerToSubnetsOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AttachLoadBalancerToSubnetsOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = AttachLoadBalancerToSubnetsOutput::default();

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
                    "Subnets" => {
                        obj.subnets =
                            Some(try!(SubnetsDeserializer::deserialize("Subnets", stack)));
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
struct AttributeTypeDeserializer;
impl AttributeTypeDeserializer {
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
struct AvailabilityZoneDeserializer;
impl AvailabilityZoneDeserializer {
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
struct AvailabilityZonesDeserializer;
impl AvailabilityZonesDeserializer {
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
                        obj.push(try!(AvailabilityZoneDeserializer::deserialize(
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

/// Serialize `AvailabilityZones` contents to a `SignedRequest`.
struct AvailabilityZonesSerializer;
impl AvailabilityZonesSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

/// <p>Information about the configuration of an EC2 instance.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct BackendServerDescription {
    /// <p>The port on which the EC2 instance is listening.</p>
    pub instance_port: Option<i64>,
    /// <p>The names of the policies enabled for the EC2 instance.</p>
    pub policy_names: Option<Vec<String>>,
}

struct BackendServerDescriptionDeserializer;
impl BackendServerDescriptionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<BackendServerDescription, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = BackendServerDescription::default();

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
                    "InstancePort" => {
                        obj.instance_port = Some(try!(InstancePortDeserializer::deserialize(
                            "InstancePort",
                            stack
                        )));
                    }
                    "PolicyNames" => {
                        obj.policy_names = Some(try!(PolicyNamesDeserializer::deserialize(
                            "PolicyNames",
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
struct BackendServerDescriptionsDeserializer;
impl BackendServerDescriptionsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<BackendServerDescription>, XmlParseError> {
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
                        obj.push(try!(BackendServerDescriptionDeserializer::deserialize(
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
struct CardinalityDeserializer;
impl CardinalityDeserializer {
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
/// <p>Contains the parameters for ConfigureHealthCheck.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ConfigureHealthCheckInput {
    /// <p>The configuration information.</p>
    pub health_check: HealthCheck,
    /// <p>The name of the load balancer.</p>
    pub load_balancer_name: String,
}

/// Serialize `ConfigureHealthCheckInput` contents to a `SignedRequest`.
struct ConfigureHealthCheckInputSerializer;
impl ConfigureHealthCheckInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ConfigureHealthCheckInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        HealthCheckSerializer::serialize(
            params,
            &format!("{}{}", prefix, "HealthCheck"),
            &obj.health_check,
        );
        params.put(
            &format!("{}{}", prefix, "LoadBalancerName"),
            &obj.load_balancer_name.replace("+", "%2B"),
        );
    }
}

/// <p>Contains the output of ConfigureHealthCheck.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ConfigureHealthCheckOutput {
    /// <p>The updated health check.</p>
    pub health_check: Option<HealthCheck>,
}

struct ConfigureHealthCheckOutputDeserializer;
impl ConfigureHealthCheckOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ConfigureHealthCheckOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ConfigureHealthCheckOutput::default();

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
                    "HealthCheck" => {
                        obj.health_check = Some(try!(HealthCheckDeserializer::deserialize(
                            "HealthCheck",
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
/// <p>Information about the <code>ConnectionDraining</code> attribute.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ConnectionDraining {
    /// <p>Specifies whether connection draining is enabled for the load balancer.</p>
    pub enabled: bool,
    /// <p>The maximum time, in seconds, to keep the existing connections open before deregistering the instances.</p>
    pub timeout: Option<i64>,
}

struct ConnectionDrainingDeserializer;
impl ConnectionDrainingDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ConnectionDraining, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ConnectionDraining::default();

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
                        obj.enabled = try!(ConnectionDrainingEnabledDeserializer::deserialize(
                            "Enabled", stack
                        ));
                    }
                    "Timeout" => {
                        obj.timeout = Some(try!(
                            ConnectionDrainingTimeoutDeserializer::deserialize("Timeout", stack)
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

/// Serialize `ConnectionDraining` contents to a `SignedRequest`.
struct ConnectionDrainingSerializer;
impl ConnectionDrainingSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ConnectionDraining) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "Enabled"),
            &obj.enabled.to_string().replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.timeout {
            params.put(
                &format!("{}{}", prefix, "Timeout"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
    }
}

struct ConnectionDrainingEnabledDeserializer;
impl ConnectionDrainingEnabledDeserializer {
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
struct ConnectionDrainingTimeoutDeserializer;
impl ConnectionDrainingTimeoutDeserializer {
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
/// <p>Information about the <code>ConnectionSettings</code> attribute.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ConnectionSettings {
    /// <p>The time, in seconds, that the connection is allowed to be idle (no data has been sent over the connection) before it is closed by the load balancer.</p>
    pub idle_timeout: i64,
}

struct ConnectionSettingsDeserializer;
impl ConnectionSettingsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ConnectionSettings, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ConnectionSettings::default();

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
                    "IdleTimeout" => {
                        obj.idle_timeout =
                            try!(IdleTimeoutDeserializer::deserialize("IdleTimeout", stack));
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

/// Serialize `ConnectionSettings` contents to a `SignedRequest`.
struct ConnectionSettingsSerializer;
impl ConnectionSettingsSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ConnectionSettings) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "IdleTimeout"),
            &obj.idle_timeout.to_string().replace("+", "%2B"),
        );
    }
}

struct CookieExpirationPeriodDeserializer;
impl CookieExpirationPeriodDeserializer {
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
struct CookieNameDeserializer;
impl CookieNameDeserializer {
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
/// <p>Contains the parameters for CreateLoadBalancer.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateAccessPointInput {
    /// <p>One or more Availability Zones from the same region as the load balancer.</p> <p>You must specify at least one Availability Zone.</p> <p>You can add more Availability Zones after you create the load balancer using <a>EnableAvailabilityZonesForLoadBalancer</a>.</p>
    pub availability_zones: Option<Vec<String>>,
    /// <p>The listeners.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/elasticloadbalancing/latest/classic/elb-listener-config.html">Listeners for Your Classic Load Balancer</a> in the <i>Classic Load Balancer Guide</i>.</p>
    pub listeners: Vec<Listener>,
    /// <p>The name of the load balancer.</p> <p>This name must be unique within your set of load balancers for the region, must have a maximum of 32 characters, must contain only alphanumeric characters or hyphens, and cannot begin or end with a hyphen.</p>
    pub load_balancer_name: String,
    /// <p>The type of a load balancer. Valid only for load balancers in a VPC.</p> <p>By default, Elastic Load Balancing creates an Internet-facing load balancer with a DNS name that resolves to public IP addresses. For more information about Internet-facing and Internal load balancers, see <a href="http://docs.aws.amazon.com/elasticloadbalancing/latest/userguide/how-elastic-load-balancing-works.html#load-balancer-scheme">Load Balancer Scheme</a> in the <i>Elastic Load Balancing User Guide</i>.</p> <p>Specify <code>internal</code> to create a load balancer with a DNS name that resolves to private IP addresses.</p>
    pub scheme: Option<String>,
    /// <p>The IDs of the security groups to assign to the load balancer.</p>
    pub security_groups: Option<Vec<String>>,
    /// <p>The IDs of the subnets in your VPC to attach to the load balancer. Specify one subnet per Availability Zone specified in <code>AvailabilityZones</code>.</p>
    pub subnets: Option<Vec<String>>,
    /// <p>A list of tags to assign to the load balancer.</p> <p>For more information about tagging your load balancer, see <a href="http://docs.aws.amazon.com/elasticloadbalancing/latest/classic/add-remove-tags.html">Tag Your Classic Load Balancer</a> in the <i>Classic Load Balancer Guide</i>.</p>
    pub tags: Option<Vec<Tag>>,
}

/// Serialize `CreateAccessPointInput` contents to a `SignedRequest`.
struct CreateAccessPointInputSerializer;
impl CreateAccessPointInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CreateAccessPointInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.availability_zones {
            AvailabilityZonesSerializer::serialize(
                params,
                &format!("{}{}", prefix, "AvailabilityZones"),
                field_value,
            );
        }
        ListenersSerializer::serialize(
            params,
            &format!("{}{}", prefix, "Listeners"),
            &obj.listeners,
        );
        params.put(
            &format!("{}{}", prefix, "LoadBalancerName"),
            &obj.load_balancer_name.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.scheme {
            params.put(
                &format!("{}{}", prefix, "Scheme"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.security_groups {
            SecurityGroupsSerializer::serialize(
                params,
                &format!("{}{}", prefix, "SecurityGroups"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.subnets {
            SubnetsSerializer::serialize(params, &format!("{}{}", prefix, "Subnets"), field_value);
        }
        if let Some(ref field_value) = obj.tags {
            TagListSerializer::serialize(params, &format!("{}{}", prefix, "Tags"), field_value);
        }
    }
}

/// <p>Contains the output for CreateLoadBalancer.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateAccessPointOutput {
    /// <p>The DNS name of the load balancer.</p>
    pub dns_name: Option<String>,
}

struct CreateAccessPointOutputDeserializer;
impl CreateAccessPointOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateAccessPointOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CreateAccessPointOutput::default();

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
                    "DNSName" => {
                        obj.dns_name =
                            Some(try!(DNSNameDeserializer::deserialize("DNSName", stack)));
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
/// <p>Contains the parameters for CreateAppCookieStickinessPolicy.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateAppCookieStickinessPolicyInput {
    /// <p>The name of the application cookie used for stickiness.</p>
    pub cookie_name: String,
    /// <p>The name of the load balancer.</p>
    pub load_balancer_name: String,
    /// <p>The name of the policy being created. Policy names must consist of alphanumeric characters and dashes (-). This name must be unique within the set of policies for this load balancer.</p>
    pub policy_name: String,
}

/// Serialize `CreateAppCookieStickinessPolicyInput` contents to a `SignedRequest`.
struct CreateAppCookieStickinessPolicyInputSerializer;
impl CreateAppCookieStickinessPolicyInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CreateAppCookieStickinessPolicyInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "CookieName"),
            &obj.cookie_name.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "LoadBalancerName"),
            &obj.load_balancer_name.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "PolicyName"),
            &obj.policy_name.replace("+", "%2B"),
        );
    }
}

/// <p>Contains the output for CreateAppCookieStickinessPolicy.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateAppCookieStickinessPolicyOutput {}

struct CreateAppCookieStickinessPolicyOutputDeserializer;
impl CreateAppCookieStickinessPolicyOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateAppCookieStickinessPolicyOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let obj = CreateAppCookieStickinessPolicyOutput::default();

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Contains the parameters for CreateLBCookieStickinessPolicy.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateLBCookieStickinessPolicyInput {
    /// <p>The time period, in seconds, after which the cookie should be considered stale. If you do not specify this parameter, the default value is 0, which indicates that the sticky session should last for the duration of the browser session.</p>
    pub cookie_expiration_period: Option<i64>,
    /// <p>The name of the load balancer.</p>
    pub load_balancer_name: String,
    /// <p>The name of the policy being created. Policy names must consist of alphanumeric characters and dashes (-). This name must be unique within the set of policies for this load balancer.</p>
    pub policy_name: String,
}

/// Serialize `CreateLBCookieStickinessPolicyInput` contents to a `SignedRequest`.
struct CreateLBCookieStickinessPolicyInputSerializer;
impl CreateLBCookieStickinessPolicyInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CreateLBCookieStickinessPolicyInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.cookie_expiration_period {
            params.put(
                &format!("{}{}", prefix, "CookieExpirationPeriod"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        params.put(
            &format!("{}{}", prefix, "LoadBalancerName"),
            &obj.load_balancer_name.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "PolicyName"),
            &obj.policy_name.replace("+", "%2B"),
        );
    }
}

/// <p>Contains the output for CreateLBCookieStickinessPolicy.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateLBCookieStickinessPolicyOutput {}

struct CreateLBCookieStickinessPolicyOutputDeserializer;
impl CreateLBCookieStickinessPolicyOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateLBCookieStickinessPolicyOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let obj = CreateLBCookieStickinessPolicyOutput::default();

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Contains the parameters for CreateLoadBalancerListeners.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateLoadBalancerListenerInput {
    /// <p>The listeners.</p>
    pub listeners: Vec<Listener>,
    /// <p>The name of the load balancer.</p>
    pub load_balancer_name: String,
}

/// Serialize `CreateLoadBalancerListenerInput` contents to a `SignedRequest`.
struct CreateLoadBalancerListenerInputSerializer;
impl CreateLoadBalancerListenerInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CreateLoadBalancerListenerInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        ListenersSerializer::serialize(
            params,
            &format!("{}{}", prefix, "Listeners"),
            &obj.listeners,
        );
        params.put(
            &format!("{}{}", prefix, "LoadBalancerName"),
            &obj.load_balancer_name.replace("+", "%2B"),
        );
    }
}

/// <p>Contains the parameters for CreateLoadBalancerListener.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateLoadBalancerListenerOutput {}

struct CreateLoadBalancerListenerOutputDeserializer;
impl CreateLoadBalancerListenerOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateLoadBalancerListenerOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let obj = CreateLoadBalancerListenerOutput::default();

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Contains the parameters for CreateLoadBalancerPolicy.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateLoadBalancerPolicyInput {
    /// <p>The name of the load balancer.</p>
    pub load_balancer_name: String,
    /// <p>The policy attributes.</p>
    pub policy_attributes: Option<Vec<PolicyAttribute>>,
    /// <p>The name of the load balancer policy to be created. This name must be unique within the set of policies for this load balancer.</p>
    pub policy_name: String,
    /// <p>The name of the base policy type. To get the list of policy types, use <a>DescribeLoadBalancerPolicyTypes</a>.</p>
    pub policy_type_name: String,
}

/// Serialize `CreateLoadBalancerPolicyInput` contents to a `SignedRequest`.
struct CreateLoadBalancerPolicyInputSerializer;
impl CreateLoadBalancerPolicyInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CreateLoadBalancerPolicyInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "LoadBalancerName"),
            &obj.load_balancer_name.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.policy_attributes {
            PolicyAttributesSerializer::serialize(
                params,
                &format!("{}{}", prefix, "PolicyAttributes"),
                field_value,
            );
        }
        params.put(
            &format!("{}{}", prefix, "PolicyName"),
            &obj.policy_name.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "PolicyTypeName"),
            &obj.policy_type_name.replace("+", "%2B"),
        );
    }
}

/// <p>Contains the output of CreateLoadBalancerPolicy.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateLoadBalancerPolicyOutput {}

struct CreateLoadBalancerPolicyOutputDeserializer;
impl CreateLoadBalancerPolicyOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateLoadBalancerPolicyOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let obj = CreateLoadBalancerPolicyOutput::default();

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct CreatedTimeDeserializer;
impl CreatedTimeDeserializer {
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
/// <p>Information about the <code>CrossZoneLoadBalancing</code> attribute.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CrossZoneLoadBalancing {
    /// <p>Specifies whether cross-zone load balancing is enabled for the load balancer.</p>
    pub enabled: bool,
}

struct CrossZoneLoadBalancingDeserializer;
impl CrossZoneLoadBalancingDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CrossZoneLoadBalancing, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CrossZoneLoadBalancing::default();

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
                        obj.enabled = try!(CrossZoneLoadBalancingEnabledDeserializer::deserialize(
                            "Enabled", stack
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

/// Serialize `CrossZoneLoadBalancing` contents to a `SignedRequest`.
struct CrossZoneLoadBalancingSerializer;
impl CrossZoneLoadBalancingSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CrossZoneLoadBalancing) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "Enabled"),
            &obj.enabled.to_string().replace("+", "%2B"),
        );
    }
}

struct CrossZoneLoadBalancingEnabledDeserializer;
impl CrossZoneLoadBalancingEnabledDeserializer {
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
struct DNSNameDeserializer;
impl DNSNameDeserializer {
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
struct DefaultValueDeserializer;
impl DefaultValueDeserializer {
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
/// <p>Contains the parameters for DeleteLoadBalancer.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteAccessPointInput {
    /// <p>The name of the load balancer.</p>
    pub load_balancer_name: String,
}

/// Serialize `DeleteAccessPointInput` contents to a `SignedRequest`.
struct DeleteAccessPointInputSerializer;
impl DeleteAccessPointInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DeleteAccessPointInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "LoadBalancerName"),
            &obj.load_balancer_name.replace("+", "%2B"),
        );
    }
}

/// <p>Contains the output of DeleteLoadBalancer.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteAccessPointOutput {}

struct DeleteAccessPointOutputDeserializer;
impl DeleteAccessPointOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteAccessPointOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let obj = DeleteAccessPointOutput::default();

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Contains the parameters for DeleteLoadBalancerListeners.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteLoadBalancerListenerInput {
    /// <p>The name of the load balancer.</p>
    pub load_balancer_name: String,
    /// <p>The client port numbers of the listeners.</p>
    pub load_balancer_ports: Vec<i64>,
}

/// Serialize `DeleteLoadBalancerListenerInput` contents to a `SignedRequest`.
struct DeleteLoadBalancerListenerInputSerializer;
impl DeleteLoadBalancerListenerInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DeleteLoadBalancerListenerInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "LoadBalancerName"),
            &obj.load_balancer_name.replace("+", "%2B"),
        );
        PortsSerializer::serialize(
            params,
            &format!("{}{}", prefix, "LoadBalancerPorts"),
            &obj.load_balancer_ports,
        );
    }
}

/// <p>Contains the output of DeleteLoadBalancerListeners.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteLoadBalancerListenerOutput {}

struct DeleteLoadBalancerListenerOutputDeserializer;
impl DeleteLoadBalancerListenerOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteLoadBalancerListenerOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let obj = DeleteLoadBalancerListenerOutput::default();

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Contains the parameters for DeleteLoadBalancerPolicy.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteLoadBalancerPolicyInput {
    /// <p>The name of the load balancer.</p>
    pub load_balancer_name: String,
    /// <p>The name of the policy.</p>
    pub policy_name: String,
}

/// Serialize `DeleteLoadBalancerPolicyInput` contents to a `SignedRequest`.
struct DeleteLoadBalancerPolicyInputSerializer;
impl DeleteLoadBalancerPolicyInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DeleteLoadBalancerPolicyInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "LoadBalancerName"),
            &obj.load_balancer_name.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "PolicyName"),
            &obj.policy_name.replace("+", "%2B"),
        );
    }
}

/// <p>Contains the output of DeleteLoadBalancerPolicy.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteLoadBalancerPolicyOutput {}

struct DeleteLoadBalancerPolicyOutputDeserializer;
impl DeleteLoadBalancerPolicyOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteLoadBalancerPolicyOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let obj = DeleteLoadBalancerPolicyOutput::default();

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Contains the parameters for DeregisterInstancesFromLoadBalancer.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeregisterEndPointsInput {
    /// <p>The IDs of the instances.</p>
    pub instances: Vec<Instance>,
    /// <p>The name of the load balancer.</p>
    pub load_balancer_name: String,
}

/// Serialize `DeregisterEndPointsInput` contents to a `SignedRequest`.
struct DeregisterEndPointsInputSerializer;
impl DeregisterEndPointsInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DeregisterEndPointsInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        InstancesSerializer::serialize(
            params,
            &format!("{}{}", prefix, "Instances"),
            &obj.instances,
        );
        params.put(
            &format!("{}{}", prefix, "LoadBalancerName"),
            &obj.load_balancer_name.replace("+", "%2B"),
        );
    }
}

/// <p>Contains the output of DeregisterInstancesFromLoadBalancer.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeregisterEndPointsOutput {
    /// <p>The remaining instances registered with the load balancer.</p>
    pub instances: Option<Vec<Instance>>,
}

struct DeregisterEndPointsOutputDeserializer;
impl DeregisterEndPointsOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeregisterEndPointsOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DeregisterEndPointsOutput::default();

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
                    "Instances" => {
                        obj.instances =
                            Some(try!(InstancesDeserializer::deserialize("Instances", stack)));
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
/// <p>Contains the parameters for DescribeLoadBalancers.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeAccessPointsInput {
    /// <p>The names of the load balancers.</p>
    pub load_balancer_names: Option<Vec<String>>,
    /// <p>The marker for the next set of results. (You received this marker from a previous call.)</p>
    pub marker: Option<String>,
    /// <p>The maximum number of results to return with this call (a number from 1 to 400). The default is 400.</p>
    pub page_size: Option<i64>,
}

/// Serialize `DescribeAccessPointsInput` contents to a `SignedRequest`.
struct DescribeAccessPointsInputSerializer;
impl DescribeAccessPointsInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeAccessPointsInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.load_balancer_names {
            LoadBalancerNamesSerializer::serialize(
                params,
                &format!("{}{}", prefix, "LoadBalancerNames"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.marker {
            params.put(
                &format!("{}{}", prefix, "Marker"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.page_size {
            params.put(
                &format!("{}{}", prefix, "PageSize"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
    }
}

/// <p>Contains the parameters for DescribeLoadBalancers.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeAccessPointsOutput {
    /// <p>Information about the load balancers.</p>
    pub load_balancer_descriptions: Option<Vec<LoadBalancerDescription>>,
    /// <p>The marker to use when requesting the next set of results. If there are no additional results, the string is empty.</p>
    pub next_marker: Option<String>,
}

struct DescribeAccessPointsOutputDeserializer;
impl DescribeAccessPointsOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeAccessPointsOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DescribeAccessPointsOutput::default();

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
                    "LoadBalancerDescriptions" => {
                        obj.load_balancer_descriptions =
                            Some(try!(LoadBalancerDescriptionsDeserializer::deserialize(
                                "LoadBalancerDescriptions",
                                stack
                            )));
                    }
                    "NextMarker" => {
                        obj.next_marker =
                            Some(try!(MarkerDeserializer::deserialize("NextMarker", stack)));
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
pub struct DescribeAccountLimitsInput {
    /// <p>The marker for the next set of results. (You received this marker from a previous call.)</p>
    pub marker: Option<String>,
    /// <p>The maximum number of results to return with this call.</p>
    pub page_size: Option<i64>,
}

/// Serialize `DescribeAccountLimitsInput` contents to a `SignedRequest`.
struct DescribeAccountLimitsInputSerializer;
impl DescribeAccountLimitsInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeAccountLimitsInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.marker {
            params.put(
                &format!("{}{}", prefix, "Marker"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.page_size {
            params.put(
                &format!("{}{}", prefix, "PageSize"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeAccountLimitsOutput {
    /// <p>Information about the limits.</p>
    pub limits: Option<Vec<Limit>>,
    /// <p>The marker to use when requesting the next set of results. If there are no additional results, the string is empty.</p>
    pub next_marker: Option<String>,
}

struct DescribeAccountLimitsOutputDeserializer;
impl DescribeAccountLimitsOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeAccountLimitsOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DescribeAccountLimitsOutput::default();

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
                    "Limits" => {
                        obj.limits = Some(try!(LimitsDeserializer::deserialize("Limits", stack)));
                    }
                    "NextMarker" => {
                        obj.next_marker =
                            Some(try!(MarkerDeserializer::deserialize("NextMarker", stack)));
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
/// <p>Contains the parameters for DescribeInstanceHealth.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeEndPointStateInput {
    /// <p>The IDs of the instances.</p>
    pub instances: Option<Vec<Instance>>,
    /// <p>The name of the load balancer.</p>
    pub load_balancer_name: String,
}

/// Serialize `DescribeEndPointStateInput` contents to a `SignedRequest`.
struct DescribeEndPointStateInputSerializer;
impl DescribeEndPointStateInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeEndPointStateInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.instances {
            InstancesSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Instances"),
                field_value,
            );
        }
        params.put(
            &format!("{}{}", prefix, "LoadBalancerName"),
            &obj.load_balancer_name.replace("+", "%2B"),
        );
    }
}

/// <p>Contains the output for DescribeInstanceHealth.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeEndPointStateOutput {
    /// <p>Information about the health of the instances.</p>
    pub instance_states: Option<Vec<InstanceState>>,
}

struct DescribeEndPointStateOutputDeserializer;
impl DescribeEndPointStateOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeEndPointStateOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DescribeEndPointStateOutput::default();

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
                    "InstanceStates" => {
                        obj.instance_states = Some(try!(InstanceStatesDeserializer::deserialize(
                            "InstanceStates",
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
/// <p>Contains the parameters for DescribeLoadBalancerAttributes.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeLoadBalancerAttributesInput {
    /// <p>The name of the load balancer.</p>
    pub load_balancer_name: String,
}

/// Serialize `DescribeLoadBalancerAttributesInput` contents to a `SignedRequest`.
struct DescribeLoadBalancerAttributesInputSerializer;
impl DescribeLoadBalancerAttributesInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeLoadBalancerAttributesInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "LoadBalancerName"),
            &obj.load_balancer_name.replace("+", "%2B"),
        );
    }
}

/// <p>Contains the output of DescribeLoadBalancerAttributes.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeLoadBalancerAttributesOutput {
    /// <p>Information about the load balancer attributes.</p>
    pub load_balancer_attributes: Option<LoadBalancerAttributes>,
}

struct DescribeLoadBalancerAttributesOutputDeserializer;
impl DescribeLoadBalancerAttributesOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeLoadBalancerAttributesOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DescribeLoadBalancerAttributesOutput::default();

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
                    "LoadBalancerAttributes" => {
                        obj.load_balancer_attributes =
                            Some(try!(LoadBalancerAttributesDeserializer::deserialize(
                                "LoadBalancerAttributes",
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
/// <p>Contains the parameters for DescribeLoadBalancerPolicies.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeLoadBalancerPoliciesInput {
    /// <p>The name of the load balancer.</p>
    pub load_balancer_name: Option<String>,
    /// <p>The names of the policies.</p>
    pub policy_names: Option<Vec<String>>,
}

/// Serialize `DescribeLoadBalancerPoliciesInput` contents to a `SignedRequest`.
struct DescribeLoadBalancerPoliciesInputSerializer;
impl DescribeLoadBalancerPoliciesInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeLoadBalancerPoliciesInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.load_balancer_name {
            params.put(
                &format!("{}{}", prefix, "LoadBalancerName"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.policy_names {
            PolicyNamesSerializer::serialize(
                params,
                &format!("{}{}", prefix, "PolicyNames"),
                field_value,
            );
        }
    }
}

/// <p>Contains the output of DescribeLoadBalancerPolicies.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeLoadBalancerPoliciesOutput {
    /// <p>Information about the policies.</p>
    pub policy_descriptions: Option<Vec<PolicyDescription>>,
}

struct DescribeLoadBalancerPoliciesOutputDeserializer;
impl DescribeLoadBalancerPoliciesOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeLoadBalancerPoliciesOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DescribeLoadBalancerPoliciesOutput::default();

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
                    "PolicyDescriptions" => {
                        obj.policy_descriptions =
                            Some(try!(PolicyDescriptionsDeserializer::deserialize(
                                "PolicyDescriptions",
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
/// <p>Contains the parameters for DescribeLoadBalancerPolicyTypes.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeLoadBalancerPolicyTypesInput {
    /// <p>The names of the policy types. If no names are specified, describes all policy types defined by Elastic Load Balancing.</p>
    pub policy_type_names: Option<Vec<String>>,
}

/// Serialize `DescribeLoadBalancerPolicyTypesInput` contents to a `SignedRequest`.
struct DescribeLoadBalancerPolicyTypesInputSerializer;
impl DescribeLoadBalancerPolicyTypesInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeLoadBalancerPolicyTypesInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.policy_type_names {
            PolicyTypeNamesSerializer::serialize(
                params,
                &format!("{}{}", prefix, "PolicyTypeNames"),
                field_value,
            );
        }
    }
}

/// <p>Contains the output of DescribeLoadBalancerPolicyTypes.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeLoadBalancerPolicyTypesOutput {
    /// <p>Information about the policy types.</p>
    pub policy_type_descriptions: Option<Vec<PolicyTypeDescription>>,
}

struct DescribeLoadBalancerPolicyTypesOutputDeserializer;
impl DescribeLoadBalancerPolicyTypesOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeLoadBalancerPolicyTypesOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DescribeLoadBalancerPolicyTypesOutput::default();

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
                    "PolicyTypeDescriptions" => {
                        obj.policy_type_descriptions =
                            Some(try!(PolicyTypeDescriptionsDeserializer::deserialize(
                                "PolicyTypeDescriptions",
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
/// <p>Contains the parameters for DescribeTags.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeTagsInput {
    /// <p>The names of the load balancers.</p>
    pub load_balancer_names: Vec<String>,
}

/// Serialize `DescribeTagsInput` contents to a `SignedRequest`.
struct DescribeTagsInputSerializer;
impl DescribeTagsInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeTagsInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        LoadBalancerNamesMax20Serializer::serialize(
            params,
            &format!("{}{}", prefix, "LoadBalancerNames"),
            &obj.load_balancer_names,
        );
    }
}

/// <p>Contains the output for DescribeTags.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeTagsOutput {
    /// <p>Information about the tags.</p>
    pub tag_descriptions: Option<Vec<TagDescription>>,
}

struct DescribeTagsOutputDeserializer;
impl DescribeTagsOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeTagsOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DescribeTagsOutput::default();

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
                    "TagDescriptions" => {
                        obj.tag_descriptions = Some(try!(
                            TagDescriptionsDeserializer::deserialize("TagDescriptions", stack)
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
struct DescriptionDeserializer;
impl DescriptionDeserializer {
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
/// <p>Contains the parameters for DetachLoadBalancerFromSubnets.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DetachLoadBalancerFromSubnetsInput {
    /// <p>The name of the load balancer.</p>
    pub load_balancer_name: String,
    /// <p>The IDs of the subnets.</p>
    pub subnets: Vec<String>,
}

/// Serialize `DetachLoadBalancerFromSubnetsInput` contents to a `SignedRequest`.
struct DetachLoadBalancerFromSubnetsInputSerializer;
impl DetachLoadBalancerFromSubnetsInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DetachLoadBalancerFromSubnetsInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "LoadBalancerName"),
            &obj.load_balancer_name.replace("+", "%2B"),
        );
        SubnetsSerializer::serialize(params, &format!("{}{}", prefix, "Subnets"), &obj.subnets);
    }
}

/// <p>Contains the output of DetachLoadBalancerFromSubnets.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DetachLoadBalancerFromSubnetsOutput {
    /// <p>The IDs of the remaining subnets for the load balancer.</p>
    pub subnets: Option<Vec<String>>,
}

struct DetachLoadBalancerFromSubnetsOutputDeserializer;
impl DetachLoadBalancerFromSubnetsOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DetachLoadBalancerFromSubnetsOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DetachLoadBalancerFromSubnetsOutput::default();

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
                    "Subnets" => {
                        obj.subnets =
                            Some(try!(SubnetsDeserializer::deserialize("Subnets", stack)));
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
/// <p>Information about a health check.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct HealthCheck {
    /// <p>The number of consecutive health checks successes required before moving the instance to the <code>Healthy</code> state.</p>
    pub healthy_threshold: i64,
    /// <p>The approximate interval, in seconds, between health checks of an individual instance.</p>
    pub interval: i64,
    /// <p>The instance being checked. The protocol is either TCP, HTTP, HTTPS, or SSL. The range of valid ports is one (1) through 65535.</p> <p>TCP is the default, specified as a TCP: port pair, for example "TCP:5000". In this case, a health check simply attempts to open a TCP connection to the instance on the specified port. Failure to connect within the configured timeout is considered unhealthy.</p> <p>SSL is also specified as SSL: port pair, for example, SSL:5000.</p> <p>For HTTP/HTTPS, you must include a ping path in the string. HTTP is specified as a HTTP:port;/;PathToPing; grouping, for example "HTTP:80/weather/us/wa/seattle". In this case, a HTTP GET request is issued to the instance on the given port and path. Any answer other than "200 OK" within the timeout period is considered unhealthy.</p> <p>The total length of the HTTP ping target must be 1024 16-bit Unicode characters or less.</p>
    pub target: String,
    /// <p>The amount of time, in seconds, during which no response means a failed health check.</p> <p>This value must be less than the <code>Interval</code> value.</p>
    pub timeout: i64,
    /// <p>The number of consecutive health check failures required before moving the instance to the <code>Unhealthy</code> state.</p>
    pub unhealthy_threshold: i64,
}

struct HealthCheckDeserializer;
impl HealthCheckDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<HealthCheck, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = HealthCheck::default();

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
                    "HealthyThreshold" => {
                        obj.healthy_threshold = try!(HealthyThresholdDeserializer::deserialize(
                            "HealthyThreshold",
                            stack
                        ));
                    }
                    "Interval" => {
                        obj.interval = try!(HealthCheckIntervalDeserializer::deserialize(
                            "Interval", stack
                        ));
                    }
                    "Target" => {
                        obj.target =
                            try!(HealthCheckTargetDeserializer::deserialize("Target", stack));
                    }
                    "Timeout" => {
                        obj.timeout = try!(HealthCheckTimeoutDeserializer::deserialize(
                            "Timeout", stack
                        ));
                    }
                    "UnhealthyThreshold" => {
                        obj.unhealthy_threshold =
                            try!(UnhealthyThresholdDeserializer::deserialize(
                                "UnhealthyThreshold",
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

/// Serialize `HealthCheck` contents to a `SignedRequest`.
struct HealthCheckSerializer;
impl HealthCheckSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &HealthCheck) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "HealthyThreshold"),
            &obj.healthy_threshold.to_string().replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "Interval"),
            &obj.interval.to_string().replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "Target"),
            &obj.target.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "Timeout"),
            &obj.timeout.to_string().replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "UnhealthyThreshold"),
            &obj.unhealthy_threshold.to_string().replace("+", "%2B"),
        );
    }
}

struct HealthCheckIntervalDeserializer;
impl HealthCheckIntervalDeserializer {
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
struct HealthCheckTargetDeserializer;
impl HealthCheckTargetDeserializer {
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
struct HealthCheckTimeoutDeserializer;
impl HealthCheckTimeoutDeserializer {
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
struct HealthyThresholdDeserializer;
impl HealthyThresholdDeserializer {
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
struct IdleTimeoutDeserializer;
impl IdleTimeoutDeserializer {
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
/// <p>The ID of an EC2 instance.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Instance {
    /// <p>The instance ID.</p>
    pub instance_id: Option<String>,
}

struct InstanceDeserializer;
impl InstanceDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Instance, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = Instance::default();

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
                    "InstanceId" => {
                        obj.instance_id = Some(try!(InstanceIdDeserializer::deserialize(
                            "InstanceId",
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

/// Serialize `Instance` contents to a `SignedRequest`.
struct InstanceSerializer;
impl InstanceSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Instance) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.instance_id {
            params.put(
                &format!("{}{}", prefix, "InstanceId"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

struct InstanceIdDeserializer;
impl InstanceIdDeserializer {
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
struct InstancePortDeserializer;
impl InstancePortDeserializer {
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
/// <p>Information about the state of an EC2 instance.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct InstanceState {
    /// <p><p>A description of the instance state. This string can contain one or more of the following messages.</p> <ul> <li> <p> <code>N/A</code> </p> </li> <li> <p> <code>A transient error occurred. Please try again later.</code> </p> </li> <li> <p> <code>Instance has failed at least the UnhealthyThreshold number of health checks consecutively.</code> </p> </li> <li> <p> <code>Instance has not passed the configured HealthyThreshold number of health checks consecutively.</code> </p> </li> <li> <p> <code>Instance registration is still in progress.</code> </p> </li> <li> <p> <code>Instance is in the EC2 Availability Zone for which LoadBalancer is not configured to route traffic to.</code> </p> </li> <li> <p> <code>Instance is not currently registered with the LoadBalancer.</code> </p> </li> <li> <p> <code>Instance deregistration currently in progress.</code> </p> </li> <li> <p> <code>Disable Availability Zone is currently in progress.</code> </p> </li> <li> <p> <code>Instance is in pending state.</code> </p> </li> <li> <p> <code>Instance is in stopped state.</code> </p> </li> <li> <p> <code>Instance is in terminated state.</code> </p> </li> </ul></p>
    pub description: Option<String>,
    /// <p>The ID of the instance.</p>
    pub instance_id: Option<String>,
    /// <p>Information about the cause of <code>OutOfService</code> instances. Specifically, whether the cause is Elastic Load Balancing or the instance.</p> <p>Valid values: <code>ELB</code> | <code>Instance</code> | <code>N/A</code> </p>
    pub reason_code: Option<String>,
    /// <p>The current state of the instance.</p> <p>Valid values: <code>InService</code> | <code>OutOfService</code> | <code>Unknown</code> </p>
    pub state: Option<String>,
}

struct InstanceStateDeserializer;
impl InstanceStateDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<InstanceState, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = InstanceState::default();

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
                    "Description" => {
                        obj.description = Some(try!(DescriptionDeserializer::deserialize(
                            "Description",
                            stack
                        )));
                    }
                    "InstanceId" => {
                        obj.instance_id = Some(try!(InstanceIdDeserializer::deserialize(
                            "InstanceId",
                            stack
                        )));
                    }
                    "ReasonCode" => {
                        obj.reason_code = Some(try!(ReasonCodeDeserializer::deserialize(
                            "ReasonCode",
                            stack
                        )));
                    }
                    "State" => {
                        obj.state = Some(try!(StateDeserializer::deserialize("State", stack)));
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
struct InstanceStatesDeserializer;
impl InstanceStatesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<InstanceState>, XmlParseError> {
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
                        obj.push(try!(InstanceStateDeserializer::deserialize(
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
struct InstancesDeserializer;
impl InstancesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Instance>, XmlParseError> {
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
                        obj.push(try!(InstanceDeserializer::deserialize("member", stack)));
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

/// Serialize `Instances` contents to a `SignedRequest`.
struct InstancesSerializer;
impl InstancesSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<Instance>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            InstanceSerializer::serialize(params, &key, obj);
        }
    }
}

struct LBCookieStickinessPoliciesDeserializer;
impl LBCookieStickinessPoliciesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<LBCookieStickinessPolicy>, XmlParseError> {
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
                        obj.push(try!(LBCookieStickinessPolicyDeserializer::deserialize(
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
/// <p>Information about a policy for duration-based session stickiness.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct LBCookieStickinessPolicy {
    /// <p>The time period, in seconds, after which the cookie should be considered stale. If this parameter is not specified, the stickiness session lasts for the duration of the browser session.</p>
    pub cookie_expiration_period: Option<i64>,
    /// <p>The name of the policy. This name must be unique within the set of policies for this load balancer.</p>
    pub policy_name: Option<String>,
}

struct LBCookieStickinessPolicyDeserializer;
impl LBCookieStickinessPolicyDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<LBCookieStickinessPolicy, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = LBCookieStickinessPolicy::default();

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
                    "CookieExpirationPeriod" => {
                        obj.cookie_expiration_period =
                            Some(try!(CookieExpirationPeriodDeserializer::deserialize(
                                "CookieExpirationPeriod",
                                stack
                            )));
                    }
                    "PolicyName" => {
                        obj.policy_name = Some(try!(PolicyNameDeserializer::deserialize(
                            "PolicyName",
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
/// <p>Information about an Elastic Load Balancing resource limit for your AWS account.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Limit {
    /// <p>The maximum value of the limit.</p>
    pub max: Option<String>,
    /// <p><p>The name of the limit. The possible values are:</p> <ul> <li> <p>classic-listeners</p> </li> <li> <p>classic-load-balancers</p> </li> </ul></p>
    pub name: Option<String>,
}

struct LimitDeserializer;
impl LimitDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Limit, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = Limit::default();

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
                    "Max" => {
                        obj.max = Some(try!(MaxDeserializer::deserialize("Max", stack)));
                    }
                    "Name" => {
                        obj.name = Some(try!(NameDeserializer::deserialize("Name", stack)));
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
struct LimitsDeserializer;
impl LimitsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Limit>, XmlParseError> {
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
                        obj.push(try!(LimitDeserializer::deserialize("member", stack)));
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
/// <p>Information about a listener.</p> <p>For information about the protocols and the ports supported by Elastic Load Balancing, see <a href="http://docs.aws.amazon.com/elasticloadbalancing/latest/classic/elb-listener-config.html">Listeners for Your Classic Load Balancer</a> in the <i>Classic Load Balancer Guide</i>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Listener {
    /// <p>The port on which the instance is listening.</p>
    pub instance_port: i64,
    /// <p>The protocol to use for routing traffic to instances: HTTP, HTTPS, TCP, or SSL.</p> <p>If the front-end protocol is HTTP, HTTPS, TCP, or SSL, <code>InstanceProtocol</code> must be at the same protocol.</p> <p>If there is another listener with the same <code>InstancePort</code> whose <code>InstanceProtocol</code> is secure, (HTTPS or SSL), the listener's <code>InstanceProtocol</code> must also be secure.</p> <p>If there is another listener with the same <code>InstancePort</code> whose <code>InstanceProtocol</code> is HTTP or TCP, the listener's <code>InstanceProtocol</code> must be HTTP or TCP.</p>
    pub instance_protocol: Option<String>,
    /// <p>The port on which the load balancer is listening. On EC2-VPC, you can specify any port from the range 1-65535. On EC2-Classic, you can specify any port from the following list: 25, 80, 443, 465, 587, 1024-65535.</p>
    pub load_balancer_port: i64,
    /// <p>The load balancer transport protocol to use for routing: HTTP, HTTPS, TCP, or SSL.</p>
    pub protocol: String,
    /// <p>The Amazon Resource Name (ARN) of the server certificate.</p>
    pub ssl_certificate_id: Option<String>,
}

struct ListenerDeserializer;
impl ListenerDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Listener, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = Listener::default();

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
                    "InstancePort" => {
                        obj.instance_port =
                            try!(InstancePortDeserializer::deserialize("InstancePort", stack));
                    }
                    "InstanceProtocol" => {
                        obj.instance_protocol = Some(try!(ProtocolDeserializer::deserialize(
                            "InstanceProtocol",
                            stack
                        )));
                    }
                    "LoadBalancerPort" => {
                        obj.load_balancer_port = try!(AccessPointPortDeserializer::deserialize(
                            "LoadBalancerPort",
                            stack
                        ));
                    }
                    "Protocol" => {
                        obj.protocol = try!(ProtocolDeserializer::deserialize("Protocol", stack));
                    }
                    "SSLCertificateId" => {
                        obj.ssl_certificate_id = Some(try!(
                            SSLCertificateIdDeserializer::deserialize("SSLCertificateId", stack)
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

/// Serialize `Listener` contents to a `SignedRequest`.
struct ListenerSerializer;
impl ListenerSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Listener) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "InstancePort"),
            &obj.instance_port.to_string().replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.instance_protocol {
            params.put(
                &format!("{}{}", prefix, "InstanceProtocol"),
                &field_value.replace("+", "%2B"),
            );
        }
        params.put(
            &format!("{}{}", prefix, "LoadBalancerPort"),
            &obj.load_balancer_port.to_string().replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "Protocol"),
            &obj.protocol.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.ssl_certificate_id {
            params.put(
                &format!("{}{}", prefix, "SSLCertificateId"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

/// <p>The policies enabled for a listener.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListenerDescription {
    /// <p>The listener.</p>
    pub listener: Option<Listener>,
    /// <p>The policies. If there are no policies enabled, the list is empty.</p>
    pub policy_names: Option<Vec<String>>,
}

struct ListenerDescriptionDeserializer;
impl ListenerDescriptionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListenerDescription, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ListenerDescription::default();

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
                    "Listener" => {
                        obj.listener =
                            Some(try!(ListenerDeserializer::deserialize("Listener", stack)));
                    }
                    "PolicyNames" => {
                        obj.policy_names = Some(try!(PolicyNamesDeserializer::deserialize(
                            "PolicyNames",
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
struct ListenerDescriptionsDeserializer;
impl ListenerDescriptionsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<ListenerDescription>, XmlParseError> {
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
                        obj.push(try!(ListenerDescriptionDeserializer::deserialize(
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

/// Serialize `Listeners` contents to a `SignedRequest`.
struct ListenersSerializer;
impl ListenersSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<Listener>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            ListenerSerializer::serialize(params, &key, obj);
        }
    }
}

/// <p>The attributes for a load balancer.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct LoadBalancerAttributes {
    /// <p>If enabled, the load balancer captures detailed information of all requests and delivers the information to the Amazon S3 bucket that you specify.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/elasticloadbalancing/latest/classic/enable-access-logs.html">Enable Access Logs</a> in the <i>Classic Load Balancer Guide</i>.</p>
    pub access_log: Option<AccessLog>,
    /// <p>This parameter is reserved.</p>
    pub additional_attributes: Option<Vec<AdditionalAttribute>>,
    /// <p>If enabled, the load balancer allows existing requests to complete before the load balancer shifts traffic away from a deregistered or unhealthy instance.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/elasticloadbalancing/latest/classic/config-conn-drain.html">Configure Connection Draining</a> in the <i>Classic Load Balancer Guide</i>.</p>
    pub connection_draining: Option<ConnectionDraining>,
    /// <p>If enabled, the load balancer allows the connections to remain idle (no data is sent over the connection) for the specified duration.</p> <p>By default, Elastic Load Balancing maintains a 60-second idle connection timeout for both front-end and back-end connections of your load balancer. For more information, see <a href="http://docs.aws.amazon.com/elasticloadbalancing/latest/classic/config-idle-timeout.html">Configure Idle Connection Timeout</a> in the <i>Classic Load Balancer Guide</i>.</p>
    pub connection_settings: Option<ConnectionSettings>,
    /// <p>If enabled, the load balancer routes the request traffic evenly across all instances regardless of the Availability Zones.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/elasticloadbalancing/latest/classic/enable-disable-crosszone-lb.html">Configure Cross-Zone Load Balancing</a> in the <i>Classic Load Balancer Guide</i>.</p>
    pub cross_zone_load_balancing: Option<CrossZoneLoadBalancing>,
}

struct LoadBalancerAttributesDeserializer;
impl LoadBalancerAttributesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<LoadBalancerAttributes, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = LoadBalancerAttributes::default();

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
                    "AccessLog" => {
                        obj.access_log =
                            Some(try!(AccessLogDeserializer::deserialize("AccessLog", stack)));
                    }
                    "AdditionalAttributes" => {
                        obj.additional_attributes =
                            Some(try!(AdditionalAttributesDeserializer::deserialize(
                                "AdditionalAttributes",
                                stack
                            )));
                    }
                    "ConnectionDraining" => {
                        obj.connection_draining =
                            Some(try!(ConnectionDrainingDeserializer::deserialize(
                                "ConnectionDraining",
                                stack
                            )));
                    }
                    "ConnectionSettings" => {
                        obj.connection_settings =
                            Some(try!(ConnectionSettingsDeserializer::deserialize(
                                "ConnectionSettings",
                                stack
                            )));
                    }
                    "CrossZoneLoadBalancing" => {
                        obj.cross_zone_load_balancing =
                            Some(try!(CrossZoneLoadBalancingDeserializer::deserialize(
                                "CrossZoneLoadBalancing",
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

/// Serialize `LoadBalancerAttributes` contents to a `SignedRequest`.
struct LoadBalancerAttributesSerializer;
impl LoadBalancerAttributesSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &LoadBalancerAttributes) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.access_log {
            AccessLogSerializer::serialize(
                params,
                &format!("{}{}", prefix, "AccessLog"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.additional_attributes {
            AdditionalAttributesSerializer::serialize(
                params,
                &format!("{}{}", prefix, "AdditionalAttributes"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.connection_draining {
            ConnectionDrainingSerializer::serialize(
                params,
                &format!("{}{}", prefix, "ConnectionDraining"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.connection_settings {
            ConnectionSettingsSerializer::serialize(
                params,
                &format!("{}{}", prefix, "ConnectionSettings"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.cross_zone_load_balancing {
            CrossZoneLoadBalancingSerializer::serialize(
                params,
                &format!("{}{}", prefix, "CrossZoneLoadBalancing"),
                field_value,
            );
        }
    }
}

/// <p>Information about a load balancer.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct LoadBalancerDescription {
    /// <p>The Availability Zones for the load balancer.</p>
    pub availability_zones: Option<Vec<String>>,
    /// <p>Information about your EC2 instances.</p>
    pub backend_server_descriptions: Option<Vec<BackendServerDescription>>,
    /// <p>The DNS name of the load balancer.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/elasticloadbalancing/latest/classic/using-domain-names-with-elb.html">Configure a Custom Domain Name</a> in the <i>Classic Load Balancer Guide</i>.</p>
    pub canonical_hosted_zone_name: Option<String>,
    /// <p>The ID of the Amazon Route 53 hosted zone for the load balancer.</p>
    pub canonical_hosted_zone_name_id: Option<String>,
    /// <p>The date and time the load balancer was created.</p>
    pub created_time: Option<String>,
    /// <p>The DNS name of the load balancer.</p>
    pub dns_name: Option<String>,
    /// <p>Information about the health checks conducted on the load balancer.</p>
    pub health_check: Option<HealthCheck>,
    /// <p>The IDs of the instances for the load balancer.</p>
    pub instances: Option<Vec<Instance>>,
    /// <p>The listeners for the load balancer.</p>
    pub listener_descriptions: Option<Vec<ListenerDescription>>,
    /// <p>The name of the load balancer.</p>
    pub load_balancer_name: Option<String>,
    /// <p>The policies defined for the load balancer.</p>
    pub policies: Option<Policies>,
    /// <p>The type of load balancer. Valid only for load balancers in a VPC.</p> <p>If <code>Scheme</code> is <code>internet-facing</code>, the load balancer has a public DNS name that resolves to a public IP address.</p> <p>If <code>Scheme</code> is <code>internal</code>, the load balancer has a public DNS name that resolves to a private IP address.</p>
    pub scheme: Option<String>,
    /// <p>The security groups for the load balancer. Valid only for load balancers in a VPC.</p>
    pub security_groups: Option<Vec<String>>,
    /// <p>The security group for the load balancer, which you can use as part of your inbound rules for your registered instances. To only allow traffic from load balancers, add a security group rule that specifies this source security group as the inbound source.</p>
    pub source_security_group: Option<SourceSecurityGroup>,
    /// <p>The IDs of the subnets for the load balancer.</p>
    pub subnets: Option<Vec<String>>,
    /// <p>The ID of the VPC for the load balancer.</p>
    pub vpc_id: Option<String>,
}

struct LoadBalancerDescriptionDeserializer;
impl LoadBalancerDescriptionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<LoadBalancerDescription, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = LoadBalancerDescription::default();

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
                    "AvailabilityZones" => {
                        obj.availability_zones = Some(try!(
                            AvailabilityZonesDeserializer::deserialize("AvailabilityZones", stack)
                        ));
                    }
                    "BackendServerDescriptions" => {
                        obj.backend_server_descriptions =
                            Some(try!(BackendServerDescriptionsDeserializer::deserialize(
                                "BackendServerDescriptions",
                                stack
                            )));
                    }
                    "CanonicalHostedZoneName" => {
                        obj.canonical_hosted_zone_name = Some(try!(
                            DNSNameDeserializer::deserialize("CanonicalHostedZoneName", stack)
                        ));
                    }
                    "CanonicalHostedZoneNameID" => {
                        obj.canonical_hosted_zone_name_id = Some(try!(
                            DNSNameDeserializer::deserialize("CanonicalHostedZoneNameID", stack)
                        ));
                    }
                    "CreatedTime" => {
                        obj.created_time = Some(try!(CreatedTimeDeserializer::deserialize(
                            "CreatedTime",
                            stack
                        )));
                    }
                    "DNSName" => {
                        obj.dns_name =
                            Some(try!(DNSNameDeserializer::deserialize("DNSName", stack)));
                    }
                    "HealthCheck" => {
                        obj.health_check = Some(try!(HealthCheckDeserializer::deserialize(
                            "HealthCheck",
                            stack
                        )));
                    }
                    "Instances" => {
                        obj.instances =
                            Some(try!(InstancesDeserializer::deserialize("Instances", stack)));
                    }
                    "ListenerDescriptions" => {
                        obj.listener_descriptions =
                            Some(try!(ListenerDescriptionsDeserializer::deserialize(
                                "ListenerDescriptions",
                                stack
                            )));
                    }
                    "LoadBalancerName" => {
                        obj.load_balancer_name = Some(try!(
                            AccessPointNameDeserializer::deserialize("LoadBalancerName", stack)
                        ));
                    }
                    "Policies" => {
                        obj.policies =
                            Some(try!(PoliciesDeserializer::deserialize("Policies", stack)));
                    }
                    "Scheme" => {
                        obj.scheme = Some(try!(LoadBalancerSchemeDeserializer::deserialize(
                            "Scheme", stack
                        )));
                    }
                    "SecurityGroups" => {
                        obj.security_groups = Some(try!(SecurityGroupsDeserializer::deserialize(
                            "SecurityGroups",
                            stack
                        )));
                    }
                    "SourceSecurityGroup" => {
                        obj.source_security_group =
                            Some(try!(SourceSecurityGroupDeserializer::deserialize(
                                "SourceSecurityGroup",
                                stack
                            )));
                    }
                    "Subnets" => {
                        obj.subnets =
                            Some(try!(SubnetsDeserializer::deserialize("Subnets", stack)));
                    }
                    "VPCId" => {
                        obj.vpc_id = Some(try!(VPCIdDeserializer::deserialize("VPCId", stack)));
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
struct LoadBalancerDescriptionsDeserializer;
impl LoadBalancerDescriptionsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<LoadBalancerDescription>, XmlParseError> {
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
                        obj.push(try!(LoadBalancerDescriptionDeserializer::deserialize(
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

/// Serialize `LoadBalancerNames` contents to a `SignedRequest`.
struct LoadBalancerNamesSerializer;
impl LoadBalancerNamesSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

/// Serialize `LoadBalancerNamesMax20` contents to a `SignedRequest`.
struct LoadBalancerNamesMax20Serializer;
impl LoadBalancerNamesMax20Serializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

struct LoadBalancerSchemeDeserializer;
impl LoadBalancerSchemeDeserializer {
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
struct MarkerDeserializer;
impl MarkerDeserializer {
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
struct MaxDeserializer;
impl MaxDeserializer {
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
/// <p>Contains the parameters for ModifyLoadBalancerAttributes.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ModifyLoadBalancerAttributesInput {
    /// <p>The attributes for the load balancer.</p>
    pub load_balancer_attributes: LoadBalancerAttributes,
    /// <p>The name of the load balancer.</p>
    pub load_balancer_name: String,
}

/// Serialize `ModifyLoadBalancerAttributesInput` contents to a `SignedRequest`.
struct ModifyLoadBalancerAttributesInputSerializer;
impl ModifyLoadBalancerAttributesInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ModifyLoadBalancerAttributesInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        LoadBalancerAttributesSerializer::serialize(
            params,
            &format!("{}{}", prefix, "LoadBalancerAttributes"),
            &obj.load_balancer_attributes,
        );
        params.put(
            &format!("{}{}", prefix, "LoadBalancerName"),
            &obj.load_balancer_name.replace("+", "%2B"),
        );
    }
}

/// <p>Contains the output of ModifyLoadBalancerAttributes.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ModifyLoadBalancerAttributesOutput {
    /// <p>Information about the load balancer attributes.</p>
    pub load_balancer_attributes: Option<LoadBalancerAttributes>,
    /// <p>The name of the load balancer.</p>
    pub load_balancer_name: Option<String>,
}

struct ModifyLoadBalancerAttributesOutputDeserializer;
impl ModifyLoadBalancerAttributesOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ModifyLoadBalancerAttributesOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ModifyLoadBalancerAttributesOutput::default();

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
                    "LoadBalancerAttributes" => {
                        obj.load_balancer_attributes =
                            Some(try!(LoadBalancerAttributesDeserializer::deserialize(
                                "LoadBalancerAttributes",
                                stack
                            )));
                    }
                    "LoadBalancerName" => {
                        obj.load_balancer_name = Some(try!(
                            AccessPointNameDeserializer::deserialize("LoadBalancerName", stack)
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
struct NameDeserializer;
impl NameDeserializer {
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
/// <p>The policies for a load balancer.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Policies {
    /// <p>The stickiness policies created using <a>CreateAppCookieStickinessPolicy</a>.</p>
    pub app_cookie_stickiness_policies: Option<Vec<AppCookieStickinessPolicy>>,
    /// <p>The stickiness policies created using <a>CreateLBCookieStickinessPolicy</a>.</p>
    pub lb_cookie_stickiness_policies: Option<Vec<LBCookieStickinessPolicy>>,
    /// <p>The policies other than the stickiness policies.</p>
    pub other_policies: Option<Vec<String>>,
}

struct PoliciesDeserializer;
impl PoliciesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Policies, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = Policies::default();

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
                    "AppCookieStickinessPolicies" => {
                        obj.app_cookie_stickiness_policies =
                            Some(try!(AppCookieStickinessPoliciesDeserializer::deserialize(
                                "AppCookieStickinessPolicies",
                                stack
                            )));
                    }
                    "LBCookieStickinessPolicies" => {
                        obj.lb_cookie_stickiness_policies =
                            Some(try!(LBCookieStickinessPoliciesDeserializer::deserialize(
                                "LBCookieStickinessPolicies",
                                stack
                            )));
                    }
                    "OtherPolicies" => {
                        obj.other_policies = Some(try!(PolicyNamesDeserializer::deserialize(
                            "OtherPolicies",
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
/// <p>Information about a policy attribute.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct PolicyAttribute {
    /// <p>The name of the attribute.</p>
    pub attribute_name: Option<String>,
    /// <p>The value of the attribute.</p>
    pub attribute_value: Option<String>,
}

/// Serialize `PolicyAttribute` contents to a `SignedRequest`.
struct PolicyAttributeSerializer;
impl PolicyAttributeSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &PolicyAttribute) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.attribute_name {
            params.put(
                &format!("{}{}", prefix, "AttributeName"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.attribute_value {
            params.put(
                &format!("{}{}", prefix, "AttributeValue"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

/// <p>Information about a policy attribute.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct PolicyAttributeDescription {
    /// <p>The name of the attribute.</p>
    pub attribute_name: Option<String>,
    /// <p>The value of the attribute.</p>
    pub attribute_value: Option<String>,
}

struct PolicyAttributeDescriptionDeserializer;
impl PolicyAttributeDescriptionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PolicyAttributeDescription, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = PolicyAttributeDescription::default();

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
                        "AttributeName" => {
                            obj.attribute_name = Some(try!(
                                AttributeNameDeserializer::deserialize("AttributeName", stack)
                            ));
                        }
                        "AttributeValue" => {
                            obj.attribute_value = Some(try!(
                                AttributeValueDeserializer::deserialize("AttributeValue", stack)
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
struct PolicyAttributeDescriptionsDeserializer;
impl PolicyAttributeDescriptionsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<PolicyAttributeDescription>, XmlParseError> {
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
                        obj.push(try!(PolicyAttributeDescriptionDeserializer::deserialize(
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
/// <p>Information about a policy attribute type.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct PolicyAttributeTypeDescription {
    /// <p>The name of the attribute.</p>
    pub attribute_name: Option<String>,
    /// <p>The type of the attribute. For example, <code>Boolean</code> or <code>Integer</code>.</p>
    pub attribute_type: Option<String>,
    /// <p><p>The cardinality of the attribute.</p> <p>Valid values:</p> <ul> <li> <p>ONE(1) : Single value required</p> </li> <li> <p>ZERO<em>OR</em>ONE(0..1) : Up to one value is allowed</p> </li> <li> <p>ZERO<em>OR</em>MORE(0..<em>) : Optional. Multiple values are allowed</p> </li> <li> <p>ONE<em>OR</em>MORE(1..</em>0) : Required. Multiple values are allowed</p> </li> </ul></p>
    pub cardinality: Option<String>,
    /// <p>The default value of the attribute, if applicable.</p>
    pub default_value: Option<String>,
    /// <p>A description of the attribute.</p>
    pub description: Option<String>,
}

struct PolicyAttributeTypeDescriptionDeserializer;
impl PolicyAttributeTypeDescriptionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PolicyAttributeTypeDescription, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = PolicyAttributeTypeDescription::default();

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
                        "AttributeName" => {
                            obj.attribute_name = Some(try!(
                                AttributeNameDeserializer::deserialize("AttributeName", stack)
                            ));
                        }
                        "AttributeType" => {
                            obj.attribute_type = Some(try!(
                                AttributeTypeDeserializer::deserialize("AttributeType", stack)
                            ));
                        }
                        "Cardinality" => {
                            obj.cardinality = Some(try!(CardinalityDeserializer::deserialize(
                                "Cardinality",
                                stack
                            )));
                        }
                        "DefaultValue" => {
                            obj.default_value = Some(try!(DefaultValueDeserializer::deserialize(
                                "DefaultValue",
                                stack
                            )));
                        }
                        "Description" => {
                            obj.description = Some(try!(DescriptionDeserializer::deserialize(
                                "Description",
                                stack
                            )));
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
struct PolicyAttributeTypeDescriptionsDeserializer;
impl PolicyAttributeTypeDescriptionsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<PolicyAttributeTypeDescription>, XmlParseError> {
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
                            PolicyAttributeTypeDescriptionDeserializer::deserialize(
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

/// Serialize `PolicyAttributes` contents to a `SignedRequest`.
struct PolicyAttributesSerializer;
impl PolicyAttributesSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<PolicyAttribute>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            PolicyAttributeSerializer::serialize(params, &key, obj);
        }
    }
}

/// <p>Information about a policy.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct PolicyDescription {
    /// <p>The policy attributes.</p>
    pub policy_attribute_descriptions: Option<Vec<PolicyAttributeDescription>>,
    /// <p>The name of the policy.</p>
    pub policy_name: Option<String>,
    /// <p>The name of the policy type.</p>
    pub policy_type_name: Option<String>,
}

struct PolicyDescriptionDeserializer;
impl PolicyDescriptionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PolicyDescription, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = PolicyDescription::default();

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
                    "PolicyAttributeDescriptions" => {
                        obj.policy_attribute_descriptions =
                            Some(try!(PolicyAttributeDescriptionsDeserializer::deserialize(
                                "PolicyAttributeDescriptions",
                                stack
                            )));
                    }
                    "PolicyName" => {
                        obj.policy_name = Some(try!(PolicyNameDeserializer::deserialize(
                            "PolicyName",
                            stack
                        )));
                    }
                    "PolicyTypeName" => {
                        obj.policy_type_name = Some(try!(PolicyTypeNameDeserializer::deserialize(
                            "PolicyTypeName",
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
struct PolicyDescriptionsDeserializer;
impl PolicyDescriptionsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<PolicyDescription>, XmlParseError> {
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
                        obj.push(try!(PolicyDescriptionDeserializer::deserialize(
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
struct PolicyNamesDeserializer;
impl PolicyNamesDeserializer {
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

/// Serialize `PolicyNames` contents to a `SignedRequest`.
struct PolicyNamesSerializer;
impl PolicyNamesSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

/// <p>Information about a policy type.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct PolicyTypeDescription {
    /// <p>A description of the policy type.</p>
    pub description: Option<String>,
    /// <p>The description of the policy attributes associated with the policies defined by Elastic Load Balancing.</p>
    pub policy_attribute_type_descriptions: Option<Vec<PolicyAttributeTypeDescription>>,
    /// <p>The name of the policy type.</p>
    pub policy_type_name: Option<String>,
}

struct PolicyTypeDescriptionDeserializer;
impl PolicyTypeDescriptionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PolicyTypeDescription, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = PolicyTypeDescription::default();

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
                    "Description" => {
                        obj.description = Some(try!(DescriptionDeserializer::deserialize(
                            "Description",
                            stack
                        )));
                    }
                    "PolicyAttributeTypeDescriptions" => {
                        obj.policy_attribute_type_descriptions = Some(try!(
                            PolicyAttributeTypeDescriptionsDeserializer::deserialize(
                                "PolicyAttributeTypeDescriptions",
                                stack
                            )
                        ));
                    }
                    "PolicyTypeName" => {
                        obj.policy_type_name = Some(try!(PolicyTypeNameDeserializer::deserialize(
                            "PolicyTypeName",
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
struct PolicyTypeDescriptionsDeserializer;
impl PolicyTypeDescriptionsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<PolicyTypeDescription>, XmlParseError> {
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
                        obj.push(try!(PolicyTypeDescriptionDeserializer::deserialize(
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
struct PolicyTypeNameDeserializer;
impl PolicyTypeNameDeserializer {
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

/// Serialize `PolicyTypeNames` contents to a `SignedRequest`.
struct PolicyTypeNamesSerializer;
impl PolicyTypeNamesSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

/// Serialize `Ports` contents to a `SignedRequest`.
struct PortsSerializer;
impl PortsSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<i64>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj.to_string());
        }
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
struct ReasonCodeDeserializer;
impl ReasonCodeDeserializer {
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
/// <p>Contains the parameters for RegisterInstancesWithLoadBalancer.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct RegisterEndPointsInput {
    /// <p>The IDs of the instances.</p>
    pub instances: Vec<Instance>,
    /// <p>The name of the load balancer.</p>
    pub load_balancer_name: String,
}

/// Serialize `RegisterEndPointsInput` contents to a `SignedRequest`.
struct RegisterEndPointsInputSerializer;
impl RegisterEndPointsInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &RegisterEndPointsInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        InstancesSerializer::serialize(
            params,
            &format!("{}{}", prefix, "Instances"),
            &obj.instances,
        );
        params.put(
            &format!("{}{}", prefix, "LoadBalancerName"),
            &obj.load_balancer_name.replace("+", "%2B"),
        );
    }
}

/// <p>Contains the output of RegisterInstancesWithLoadBalancer.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct RegisterEndPointsOutput {
    /// <p>The updated list of instances for the load balancer.</p>
    pub instances: Option<Vec<Instance>>,
}

struct RegisterEndPointsOutputDeserializer;
impl RegisterEndPointsOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<RegisterEndPointsOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = RegisterEndPointsOutput::default();

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
                    "Instances" => {
                        obj.instances =
                            Some(try!(InstancesDeserializer::deserialize("Instances", stack)));
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
/// <p>Contains the parameters for DisableAvailabilityZonesForLoadBalancer.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct RemoveAvailabilityZonesInput {
    /// <p>The Availability Zones.</p>
    pub availability_zones: Vec<String>,
    /// <p>The name of the load balancer.</p>
    pub load_balancer_name: String,
}

/// Serialize `RemoveAvailabilityZonesInput` contents to a `SignedRequest`.
struct RemoveAvailabilityZonesInputSerializer;
impl RemoveAvailabilityZonesInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &RemoveAvailabilityZonesInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        AvailabilityZonesSerializer::serialize(
            params,
            &format!("{}{}", prefix, "AvailabilityZones"),
            &obj.availability_zones,
        );
        params.put(
            &format!("{}{}", prefix, "LoadBalancerName"),
            &obj.load_balancer_name.replace("+", "%2B"),
        );
    }
}

/// <p>Contains the output for DisableAvailabilityZonesForLoadBalancer.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct RemoveAvailabilityZonesOutput {
    /// <p>The remaining Availability Zones for the load balancer.</p>
    pub availability_zones: Option<Vec<String>>,
}

struct RemoveAvailabilityZonesOutputDeserializer;
impl RemoveAvailabilityZonesOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<RemoveAvailabilityZonesOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = RemoveAvailabilityZonesOutput::default();

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
                    "AvailabilityZones" => {
                        obj.availability_zones = Some(try!(
                            AvailabilityZonesDeserializer::deserialize("AvailabilityZones", stack)
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
/// <p>Contains the parameters for RemoveTags.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct RemoveTagsInput {
    /// <p>The name of the load balancer. You can specify a maximum of one load balancer name.</p>
    pub load_balancer_names: Vec<String>,
    /// <p>The list of tag keys to remove.</p>
    pub tags: Vec<TagKeyOnly>,
}

/// Serialize `RemoveTagsInput` contents to a `SignedRequest`.
struct RemoveTagsInputSerializer;
impl RemoveTagsInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &RemoveTagsInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        LoadBalancerNamesSerializer::serialize(
            params,
            &format!("{}{}", prefix, "LoadBalancerNames"),
            &obj.load_balancer_names,
        );
        TagKeyListSerializer::serialize(params, &format!("{}{}", prefix, "Tags"), &obj.tags);
    }
}

/// <p>Contains the output of RemoveTags.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct RemoveTagsOutput {}

struct RemoveTagsOutputDeserializer;
impl RemoveTagsOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<RemoveTagsOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let obj = RemoveTagsOutput::default();

        try!(end_element(tag_name, stack));

        Ok(obj)
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
struct SSLCertificateIdDeserializer;
impl SSLCertificateIdDeserializer {
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
struct SecurityGroupIdDeserializer;
impl SecurityGroupIdDeserializer {
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
struct SecurityGroupNameDeserializer;
impl SecurityGroupNameDeserializer {
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
struct SecurityGroupOwnerAliasDeserializer;
impl SecurityGroupOwnerAliasDeserializer {
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
struct SecurityGroupsDeserializer;
impl SecurityGroupsDeserializer {
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
                        obj.push(try!(SecurityGroupIdDeserializer::deserialize(
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

/// Serialize `SecurityGroups` contents to a `SignedRequest`.
struct SecurityGroupsSerializer;
impl SecurityGroupsSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

/// <p>Contains the parameters for SetLoadBalancerListenerSSLCertificate.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SetLoadBalancerListenerSSLCertificateInput {
    /// <p>The name of the load balancer.</p>
    pub load_balancer_name: String,
    /// <p>The port that uses the specified SSL certificate.</p>
    pub load_balancer_port: i64,
    /// <p>The Amazon Resource Name (ARN) of the SSL certificate.</p>
    pub ssl_certificate_id: String,
}

/// Serialize `SetLoadBalancerListenerSSLCertificateInput` contents to a `SignedRequest`.
struct SetLoadBalancerListenerSSLCertificateInputSerializer;
impl SetLoadBalancerListenerSSLCertificateInputSerializer {
    fn serialize(
        params: &mut Params,
        name: &str,
        obj: &SetLoadBalancerListenerSSLCertificateInput,
    ) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "LoadBalancerName"),
            &obj.load_balancer_name.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "LoadBalancerPort"),
            &obj.load_balancer_port.to_string().replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "SSLCertificateId"),
            &obj.ssl_certificate_id.replace("+", "%2B"),
        );
    }
}

/// <p>Contains the output of SetLoadBalancerListenerSSLCertificate.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SetLoadBalancerListenerSSLCertificateOutput {}

struct SetLoadBalancerListenerSSLCertificateOutputDeserializer;
impl SetLoadBalancerListenerSSLCertificateOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<SetLoadBalancerListenerSSLCertificateOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let obj = SetLoadBalancerListenerSSLCertificateOutput::default();

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Contains the parameters for SetLoadBalancerPoliciesForBackendServer.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SetLoadBalancerPoliciesForBackendServerInput {
    /// <p>The port number associated with the EC2 instance.</p>
    pub instance_port: i64,
    /// <p>The name of the load balancer.</p>
    pub load_balancer_name: String,
    /// <p>The names of the policies. If the list is empty, then all current polices are removed from the EC2 instance.</p>
    pub policy_names: Vec<String>,
}

/// Serialize `SetLoadBalancerPoliciesForBackendServerInput` contents to a `SignedRequest`.
struct SetLoadBalancerPoliciesForBackendServerInputSerializer;
impl SetLoadBalancerPoliciesForBackendServerInputSerializer {
    fn serialize(
        params: &mut Params,
        name: &str,
        obj: &SetLoadBalancerPoliciesForBackendServerInput,
    ) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "InstancePort"),
            &obj.instance_port.to_string().replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "LoadBalancerName"),
            &obj.load_balancer_name.replace("+", "%2B"),
        );
        PolicyNamesSerializer::serialize(
            params,
            &format!("{}{}", prefix, "PolicyNames"),
            &obj.policy_names,
        );
    }
}

/// <p>Contains the output of SetLoadBalancerPoliciesForBackendServer.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SetLoadBalancerPoliciesForBackendServerOutput {}

struct SetLoadBalancerPoliciesForBackendServerOutputDeserializer;
impl SetLoadBalancerPoliciesForBackendServerOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<SetLoadBalancerPoliciesForBackendServerOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let obj = SetLoadBalancerPoliciesForBackendServerOutput::default();

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Contains the parameters for SetLoadBalancePoliciesOfListener.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SetLoadBalancerPoliciesOfListenerInput {
    /// <p>The name of the load balancer.</p>
    pub load_balancer_name: String,
    /// <p>The external port of the load balancer.</p>
    pub load_balancer_port: i64,
    /// <p>The names of the policies. This list must include all policies to be enabled. If you omit a policy that is currently enabled, it is disabled. If the list is empty, all current policies are disabled.</p>
    pub policy_names: Vec<String>,
}

/// Serialize `SetLoadBalancerPoliciesOfListenerInput` contents to a `SignedRequest`.
struct SetLoadBalancerPoliciesOfListenerInputSerializer;
impl SetLoadBalancerPoliciesOfListenerInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &SetLoadBalancerPoliciesOfListenerInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "LoadBalancerName"),
            &obj.load_balancer_name.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "LoadBalancerPort"),
            &obj.load_balancer_port.to_string().replace("+", "%2B"),
        );
        PolicyNamesSerializer::serialize(
            params,
            &format!("{}{}", prefix, "PolicyNames"),
            &obj.policy_names,
        );
    }
}

/// <p>Contains the output of SetLoadBalancePoliciesOfListener.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SetLoadBalancerPoliciesOfListenerOutput {}

struct SetLoadBalancerPoliciesOfListenerOutputDeserializer;
impl SetLoadBalancerPoliciesOfListenerOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<SetLoadBalancerPoliciesOfListenerOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let obj = SetLoadBalancerPoliciesOfListenerOutput::default();

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Information about a source security group.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SourceSecurityGroup {
    /// <p>The name of the security group.</p>
    pub group_name: Option<String>,
    /// <p>The owner of the security group.</p>
    pub owner_alias: Option<String>,
}

struct SourceSecurityGroupDeserializer;
impl SourceSecurityGroupDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<SourceSecurityGroup, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = SourceSecurityGroup::default();

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
                    "GroupName" => {
                        obj.group_name = Some(try!(SecurityGroupNameDeserializer::deserialize(
                            "GroupName",
                            stack
                        )));
                    }
                    "OwnerAlias" => {
                        obj.owner_alias = Some(try!(
                            SecurityGroupOwnerAliasDeserializer::deserialize("OwnerAlias", stack)
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
struct StateDeserializer;
impl StateDeserializer {
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
struct SubnetIdDeserializer;
impl SubnetIdDeserializer {
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
struct SubnetsDeserializer;
impl SubnetsDeserializer {
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
                        obj.push(try!(SubnetIdDeserializer::deserialize("member", stack)));
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

/// Serialize `Subnets` contents to a `SignedRequest`.
struct SubnetsSerializer;
impl SubnetsSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

/// <p>Information about a tag.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Tag {
    /// <p>The key of the tag.</p>
    pub key: String,
    /// <p>The value of the tag.</p>
    pub value: Option<String>,
}

struct TagDeserializer;
impl TagDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Tag, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = Tag::default();

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
                    "Key" => {
                        obj.key = try!(TagKeyDeserializer::deserialize("Key", stack));
                    }
                    "Value" => {
                        obj.value = Some(try!(TagValueDeserializer::deserialize("Value", stack)));
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

/// Serialize `Tag` contents to a `SignedRequest`.
struct TagSerializer;
impl TagSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Tag) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "Key"),
            &obj.key.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.value {
            params.put(
                &format!("{}{}", prefix, "Value"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

/// <p>The tags associated with a load balancer.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct TagDescription {
    /// <p>The name of the load balancer.</p>
    pub load_balancer_name: Option<String>,
    /// <p>The tags.</p>
    pub tags: Option<Vec<Tag>>,
}

struct TagDescriptionDeserializer;
impl TagDescriptionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<TagDescription, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = TagDescription::default();

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
                    "LoadBalancerName" => {
                        obj.load_balancer_name = Some(try!(
                            AccessPointNameDeserializer::deserialize("LoadBalancerName", stack)
                        ));
                    }
                    "Tags" => {
                        obj.tags = Some(try!(TagListDeserializer::deserialize("Tags", stack)));
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
struct TagDescriptionsDeserializer;
impl TagDescriptionsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<TagDescription>, XmlParseError> {
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
                        obj.push(try!(TagDescriptionDeserializer::deserialize(
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
struct TagKeyDeserializer;
impl TagKeyDeserializer {
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

/// Serialize `TagKeyList` contents to a `SignedRequest`.
struct TagKeyListSerializer;
impl TagKeyListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<TagKeyOnly>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            TagKeyOnlySerializer::serialize(params, &key, obj);
        }
    }
}

/// <p>The key of a tag.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct TagKeyOnly {
    /// <p>The name of the key.</p>
    pub key: Option<String>,
}

/// Serialize `TagKeyOnly` contents to a `SignedRequest`.
struct TagKeyOnlySerializer;
impl TagKeyOnlySerializer {
    fn serialize(params: &mut Params, name: &str, obj: &TagKeyOnly) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.key {
            params.put(
                &format!("{}{}", prefix, "Key"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

struct TagListDeserializer;
impl TagListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Tag>, XmlParseError> {
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
                        obj.push(try!(TagDeserializer::deserialize("member", stack)));
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

struct TagValueDeserializer;
impl TagValueDeserializer {
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
struct UnhealthyThresholdDeserializer;
impl UnhealthyThresholdDeserializer {
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
struct VPCIdDeserializer;
impl VPCIdDeserializer {
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
/// Errors returned by AddTags
#[derive(Debug, PartialEq)]
pub enum AddTagsError {
    /// <p>The specified load balancer does not exist.</p>
    AccessPointNotFound(String),
    /// <p>A tag key was specified more than once.</p>
    DuplicateTagKeys(String),
    /// <p>The quota for the number of tags that can be assigned to a load balancer has been reached.</p>
    TooManyTags(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl AddTagsError {
    pub fn from_body(body: &str) -> AddTagsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "LoadBalancerNotFound" => {
                    AddTagsError::AccessPointNotFound(String::from(parsed_error.message))
                }
                "DuplicateTagKeys" => {
                    AddTagsError::DuplicateTagKeys(String::from(parsed_error.message))
                }
                "TooManyTags" => AddTagsError::TooManyTags(String::from(parsed_error.message)),
                _ => AddTagsError::Unknown(String::from(body)),
            },
            Err(_) => AddTagsError::Unknown(body.to_string()),
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

impl From<XmlParseError> for AddTagsError {
    fn from(err: XmlParseError) -> AddTagsError {
        let XmlParseError(message) = err;
        AddTagsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for AddTagsError {
    fn from(err: CredentialsError) -> AddTagsError {
        AddTagsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AddTagsError {
    fn from(err: HttpDispatchError) -> AddTagsError {
        AddTagsError::HttpDispatch(err)
    }
}
impl From<io::Error> for AddTagsError {
    fn from(err: io::Error) -> AddTagsError {
        AddTagsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AddTagsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AddTagsError {
    fn description(&self) -> &str {
        match *self {
            AddTagsError::AccessPointNotFound(ref cause) => cause,
            AddTagsError::DuplicateTagKeys(ref cause) => cause,
            AddTagsError::TooManyTags(ref cause) => cause,
            AddTagsError::Validation(ref cause) => cause,
            AddTagsError::Credentials(ref err) => err.description(),
            AddTagsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            AddTagsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ApplySecurityGroupsToLoadBalancer
#[derive(Debug, PartialEq)]
pub enum ApplySecurityGroupsToLoadBalancerError {
    /// <p>The specified load balancer does not exist.</p>
    AccessPointNotFound(String),
    /// <p>The requested configuration change is not valid.</p>
    InvalidConfigurationRequest(String),
    /// <p>One or more of the specified security groups do not exist.</p>
    InvalidSecurityGroup(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ApplySecurityGroupsToLoadBalancerError {
    pub fn from_body(body: &str) -> ApplySecurityGroupsToLoadBalancerError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "LoadBalancerNotFound" => {
                    ApplySecurityGroupsToLoadBalancerError::AccessPointNotFound(String::from(
                        parsed_error.message,
                    ))
                }
                "InvalidConfigurationRequest" => {
                    ApplySecurityGroupsToLoadBalancerError::InvalidConfigurationRequest(
                        String::from(parsed_error.message),
                    )
                }
                "InvalidSecurityGroup" => {
                    ApplySecurityGroupsToLoadBalancerError::InvalidSecurityGroup(String::from(
                        parsed_error.message,
                    ))
                }
                _ => ApplySecurityGroupsToLoadBalancerError::Unknown(String::from(body)),
            },
            Err(_) => ApplySecurityGroupsToLoadBalancerError::Unknown(body.to_string()),
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

impl From<XmlParseError> for ApplySecurityGroupsToLoadBalancerError {
    fn from(err: XmlParseError) -> ApplySecurityGroupsToLoadBalancerError {
        let XmlParseError(message) = err;
        ApplySecurityGroupsToLoadBalancerError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ApplySecurityGroupsToLoadBalancerError {
    fn from(err: CredentialsError) -> ApplySecurityGroupsToLoadBalancerError {
        ApplySecurityGroupsToLoadBalancerError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ApplySecurityGroupsToLoadBalancerError {
    fn from(err: HttpDispatchError) -> ApplySecurityGroupsToLoadBalancerError {
        ApplySecurityGroupsToLoadBalancerError::HttpDispatch(err)
    }
}
impl From<io::Error> for ApplySecurityGroupsToLoadBalancerError {
    fn from(err: io::Error) -> ApplySecurityGroupsToLoadBalancerError {
        ApplySecurityGroupsToLoadBalancerError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ApplySecurityGroupsToLoadBalancerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ApplySecurityGroupsToLoadBalancerError {
    fn description(&self) -> &str {
        match *self {
            ApplySecurityGroupsToLoadBalancerError::AccessPointNotFound(ref cause) => cause,
            ApplySecurityGroupsToLoadBalancerError::InvalidConfigurationRequest(ref cause) => cause,
            ApplySecurityGroupsToLoadBalancerError::InvalidSecurityGroup(ref cause) => cause,
            ApplySecurityGroupsToLoadBalancerError::Validation(ref cause) => cause,
            ApplySecurityGroupsToLoadBalancerError::Credentials(ref err) => err.description(),
            ApplySecurityGroupsToLoadBalancerError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ApplySecurityGroupsToLoadBalancerError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by AttachLoadBalancerToSubnets
#[derive(Debug, PartialEq)]
pub enum AttachLoadBalancerToSubnetsError {
    /// <p>The specified load balancer does not exist.</p>
    AccessPointNotFound(String),
    /// <p>The requested configuration change is not valid.</p>
    InvalidConfigurationRequest(String),
    /// <p>The specified VPC has no associated Internet gateway.</p>
    InvalidSubnet(String),
    /// <p>One or more of the specified subnets do not exist.</p>
    SubnetNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl AttachLoadBalancerToSubnetsError {
    pub fn from_body(body: &str) -> AttachLoadBalancerToSubnetsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "LoadBalancerNotFound" => AttachLoadBalancerToSubnetsError::AccessPointNotFound(
                    String::from(parsed_error.message),
                ),
                "InvalidConfigurationRequest" => {
                    AttachLoadBalancerToSubnetsError::InvalidConfigurationRequest(String::from(
                        parsed_error.message,
                    ))
                }
                "InvalidSubnet" => AttachLoadBalancerToSubnetsError::InvalidSubnet(String::from(
                    parsed_error.message,
                )),
                "SubnetNotFound" => AttachLoadBalancerToSubnetsError::SubnetNotFound(String::from(
                    parsed_error.message,
                )),
                _ => AttachLoadBalancerToSubnetsError::Unknown(String::from(body)),
            },
            Err(_) => AttachLoadBalancerToSubnetsError::Unknown(body.to_string()),
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

impl From<XmlParseError> for AttachLoadBalancerToSubnetsError {
    fn from(err: XmlParseError) -> AttachLoadBalancerToSubnetsError {
        let XmlParseError(message) = err;
        AttachLoadBalancerToSubnetsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for AttachLoadBalancerToSubnetsError {
    fn from(err: CredentialsError) -> AttachLoadBalancerToSubnetsError {
        AttachLoadBalancerToSubnetsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AttachLoadBalancerToSubnetsError {
    fn from(err: HttpDispatchError) -> AttachLoadBalancerToSubnetsError {
        AttachLoadBalancerToSubnetsError::HttpDispatch(err)
    }
}
impl From<io::Error> for AttachLoadBalancerToSubnetsError {
    fn from(err: io::Error) -> AttachLoadBalancerToSubnetsError {
        AttachLoadBalancerToSubnetsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AttachLoadBalancerToSubnetsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AttachLoadBalancerToSubnetsError {
    fn description(&self) -> &str {
        match *self {
            AttachLoadBalancerToSubnetsError::AccessPointNotFound(ref cause) => cause,
            AttachLoadBalancerToSubnetsError::InvalidConfigurationRequest(ref cause) => cause,
            AttachLoadBalancerToSubnetsError::InvalidSubnet(ref cause) => cause,
            AttachLoadBalancerToSubnetsError::SubnetNotFound(ref cause) => cause,
            AttachLoadBalancerToSubnetsError::Validation(ref cause) => cause,
            AttachLoadBalancerToSubnetsError::Credentials(ref err) => err.description(),
            AttachLoadBalancerToSubnetsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AttachLoadBalancerToSubnetsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ConfigureHealthCheck
#[derive(Debug, PartialEq)]
pub enum ConfigureHealthCheckError {
    /// <p>The specified load balancer does not exist.</p>
    AccessPointNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ConfigureHealthCheckError {
    pub fn from_body(body: &str) -> ConfigureHealthCheckError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "LoadBalancerNotFound" => ConfigureHealthCheckError::AccessPointNotFound(
                    String::from(parsed_error.message),
                ),
                _ => ConfigureHealthCheckError::Unknown(String::from(body)),
            },
            Err(_) => ConfigureHealthCheckError::Unknown(body.to_string()),
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

impl From<XmlParseError> for ConfigureHealthCheckError {
    fn from(err: XmlParseError) -> ConfigureHealthCheckError {
        let XmlParseError(message) = err;
        ConfigureHealthCheckError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ConfigureHealthCheckError {
    fn from(err: CredentialsError) -> ConfigureHealthCheckError {
        ConfigureHealthCheckError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ConfigureHealthCheckError {
    fn from(err: HttpDispatchError) -> ConfigureHealthCheckError {
        ConfigureHealthCheckError::HttpDispatch(err)
    }
}
impl From<io::Error> for ConfigureHealthCheckError {
    fn from(err: io::Error) -> ConfigureHealthCheckError {
        ConfigureHealthCheckError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ConfigureHealthCheckError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ConfigureHealthCheckError {
    fn description(&self) -> &str {
        match *self {
            ConfigureHealthCheckError::AccessPointNotFound(ref cause) => cause,
            ConfigureHealthCheckError::Validation(ref cause) => cause,
            ConfigureHealthCheckError::Credentials(ref err) => err.description(),
            ConfigureHealthCheckError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ConfigureHealthCheckError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateAppCookieStickinessPolicy
#[derive(Debug, PartialEq)]
pub enum CreateAppCookieStickinessPolicyError {
    /// <p>The specified load balancer does not exist.</p>
    AccessPointNotFound(String),
    /// <p>A policy with the specified name already exists for this load balancer.</p>
    DuplicatePolicyName(String),
    /// <p>The requested configuration change is not valid.</p>
    InvalidConfigurationRequest(String),
    /// <p>The quota for the number of policies for this load balancer has been reached.</p>
    TooManyPolicies(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateAppCookieStickinessPolicyError {
    pub fn from_body(body: &str) -> CreateAppCookieStickinessPolicyError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "LoadBalancerNotFound" => {
                    CreateAppCookieStickinessPolicyError::AccessPointNotFound(String::from(
                        parsed_error.message,
                    ))
                }
                "DuplicatePolicyName" => CreateAppCookieStickinessPolicyError::DuplicatePolicyName(
                    String::from(parsed_error.message),
                ),
                "InvalidConfigurationRequest" => {
                    CreateAppCookieStickinessPolicyError::InvalidConfigurationRequest(String::from(
                        parsed_error.message,
                    ))
                }
                "TooManyPolicies" => CreateAppCookieStickinessPolicyError::TooManyPolicies(
                    String::from(parsed_error.message),
                ),
                _ => CreateAppCookieStickinessPolicyError::Unknown(String::from(body)),
            },
            Err(_) => CreateAppCookieStickinessPolicyError::Unknown(body.to_string()),
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

impl From<XmlParseError> for CreateAppCookieStickinessPolicyError {
    fn from(err: XmlParseError) -> CreateAppCookieStickinessPolicyError {
        let XmlParseError(message) = err;
        CreateAppCookieStickinessPolicyError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for CreateAppCookieStickinessPolicyError {
    fn from(err: CredentialsError) -> CreateAppCookieStickinessPolicyError {
        CreateAppCookieStickinessPolicyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateAppCookieStickinessPolicyError {
    fn from(err: HttpDispatchError) -> CreateAppCookieStickinessPolicyError {
        CreateAppCookieStickinessPolicyError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateAppCookieStickinessPolicyError {
    fn from(err: io::Error) -> CreateAppCookieStickinessPolicyError {
        CreateAppCookieStickinessPolicyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateAppCookieStickinessPolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateAppCookieStickinessPolicyError {
    fn description(&self) -> &str {
        match *self {
            CreateAppCookieStickinessPolicyError::AccessPointNotFound(ref cause) => cause,
            CreateAppCookieStickinessPolicyError::DuplicatePolicyName(ref cause) => cause,
            CreateAppCookieStickinessPolicyError::InvalidConfigurationRequest(ref cause) => cause,
            CreateAppCookieStickinessPolicyError::TooManyPolicies(ref cause) => cause,
            CreateAppCookieStickinessPolicyError::Validation(ref cause) => cause,
            CreateAppCookieStickinessPolicyError::Credentials(ref err) => err.description(),
            CreateAppCookieStickinessPolicyError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateAppCookieStickinessPolicyError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateLBCookieStickinessPolicy
#[derive(Debug, PartialEq)]
pub enum CreateLBCookieStickinessPolicyError {
    /// <p>The specified load balancer does not exist.</p>
    AccessPointNotFound(String),
    /// <p>A policy with the specified name already exists for this load balancer.</p>
    DuplicatePolicyName(String),
    /// <p>The requested configuration change is not valid.</p>
    InvalidConfigurationRequest(String),
    /// <p>The quota for the number of policies for this load balancer has been reached.</p>
    TooManyPolicies(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateLBCookieStickinessPolicyError {
    pub fn from_body(body: &str) -> CreateLBCookieStickinessPolicyError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "LoadBalancerNotFound" => CreateLBCookieStickinessPolicyError::AccessPointNotFound(
                    String::from(parsed_error.message),
                ),
                "DuplicatePolicyName" => CreateLBCookieStickinessPolicyError::DuplicatePolicyName(
                    String::from(parsed_error.message),
                ),
                "InvalidConfigurationRequest" => {
                    CreateLBCookieStickinessPolicyError::InvalidConfigurationRequest(String::from(
                        parsed_error.message,
                    ))
                }
                "TooManyPolicies" => CreateLBCookieStickinessPolicyError::TooManyPolicies(
                    String::from(parsed_error.message),
                ),
                _ => CreateLBCookieStickinessPolicyError::Unknown(String::from(body)),
            },
            Err(_) => CreateLBCookieStickinessPolicyError::Unknown(body.to_string()),
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

impl From<XmlParseError> for CreateLBCookieStickinessPolicyError {
    fn from(err: XmlParseError) -> CreateLBCookieStickinessPolicyError {
        let XmlParseError(message) = err;
        CreateLBCookieStickinessPolicyError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for CreateLBCookieStickinessPolicyError {
    fn from(err: CredentialsError) -> CreateLBCookieStickinessPolicyError {
        CreateLBCookieStickinessPolicyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateLBCookieStickinessPolicyError {
    fn from(err: HttpDispatchError) -> CreateLBCookieStickinessPolicyError {
        CreateLBCookieStickinessPolicyError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateLBCookieStickinessPolicyError {
    fn from(err: io::Error) -> CreateLBCookieStickinessPolicyError {
        CreateLBCookieStickinessPolicyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateLBCookieStickinessPolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateLBCookieStickinessPolicyError {
    fn description(&self) -> &str {
        match *self {
            CreateLBCookieStickinessPolicyError::AccessPointNotFound(ref cause) => cause,
            CreateLBCookieStickinessPolicyError::DuplicatePolicyName(ref cause) => cause,
            CreateLBCookieStickinessPolicyError::InvalidConfigurationRequest(ref cause) => cause,
            CreateLBCookieStickinessPolicyError::TooManyPolicies(ref cause) => cause,
            CreateLBCookieStickinessPolicyError::Validation(ref cause) => cause,
            CreateLBCookieStickinessPolicyError::Credentials(ref err) => err.description(),
            CreateLBCookieStickinessPolicyError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateLBCookieStickinessPolicyError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateLoadBalancer
#[derive(Debug, PartialEq)]
pub enum CreateLoadBalancerError {
    /// <p>The specified ARN does not refer to a valid SSL certificate in AWS Identity and Access Management (IAM) or AWS Certificate Manager (ACM). Note that if you recently uploaded the certificate to IAM, this error might indicate that the certificate is not fully available yet.</p>
    CertificateNotFound(String),
    /// <p>The specified load balancer name already exists for this account.</p>
    DuplicateAccessPointName(String),
    /// <p>A tag key was specified more than once.</p>
    DuplicateTagKeys(String),
    /// <p>The requested configuration change is not valid.</p>
    InvalidConfigurationRequest(String),
    /// <p>The specified value for the schema is not valid. You can only specify a scheme for load balancers in a VPC.</p>
    InvalidScheme(String),
    /// <p>One or more of the specified security groups do not exist.</p>
    InvalidSecurityGroup(String),
    /// <p>The specified VPC has no associated Internet gateway.</p>
    InvalidSubnet(String),
    /// <p>This operation is not allowed.</p>
    OperationNotPermitted(String),
    /// <p>One or more of the specified subnets do not exist.</p>
    SubnetNotFound(String),
    /// <p>The quota for the number of load balancers has been reached.</p>
    TooManyAccessPoints(String),
    /// <p>The quota for the number of tags that can be assigned to a load balancer has been reached.</p>
    TooManyTags(String),
    /// <p>The specified protocol or signature version is not supported.</p>
    UnsupportedProtocol(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateLoadBalancerError {
    pub fn from_body(body: &str) -> CreateLoadBalancerError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "CertificateNotFound" => {
                    CreateLoadBalancerError::CertificateNotFound(String::from(parsed_error.message))
                }
                "DuplicateLoadBalancerName" => CreateLoadBalancerError::DuplicateAccessPointName(
                    String::from(parsed_error.message),
                ),
                "DuplicateTagKeys" => {
                    CreateLoadBalancerError::DuplicateTagKeys(String::from(parsed_error.message))
                }
                "InvalidConfigurationRequest" => {
                    CreateLoadBalancerError::InvalidConfigurationRequest(String::from(
                        parsed_error.message,
                    ))
                }
                "InvalidScheme" => {
                    CreateLoadBalancerError::InvalidScheme(String::from(parsed_error.message))
                }
                "InvalidSecurityGroup" => CreateLoadBalancerError::InvalidSecurityGroup(
                    String::from(parsed_error.message),
                ),
                "InvalidSubnet" => {
                    CreateLoadBalancerError::InvalidSubnet(String::from(parsed_error.message))
                }
                "OperationNotPermitted" => CreateLoadBalancerError::OperationNotPermitted(
                    String::from(parsed_error.message),
                ),
                "SubnetNotFound" => {
                    CreateLoadBalancerError::SubnetNotFound(String::from(parsed_error.message))
                }
                "TooManyLoadBalancers" => {
                    CreateLoadBalancerError::TooManyAccessPoints(String::from(parsed_error.message))
                }
                "TooManyTags" => {
                    CreateLoadBalancerError::TooManyTags(String::from(parsed_error.message))
                }
                "UnsupportedProtocol" => {
                    CreateLoadBalancerError::UnsupportedProtocol(String::from(parsed_error.message))
                }
                _ => CreateLoadBalancerError::Unknown(String::from(body)),
            },
            Err(_) => CreateLoadBalancerError::Unknown(body.to_string()),
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

impl From<XmlParseError> for CreateLoadBalancerError {
    fn from(err: XmlParseError) -> CreateLoadBalancerError {
        let XmlParseError(message) = err;
        CreateLoadBalancerError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for CreateLoadBalancerError {
    fn from(err: CredentialsError) -> CreateLoadBalancerError {
        CreateLoadBalancerError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateLoadBalancerError {
    fn from(err: HttpDispatchError) -> CreateLoadBalancerError {
        CreateLoadBalancerError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateLoadBalancerError {
    fn from(err: io::Error) -> CreateLoadBalancerError {
        CreateLoadBalancerError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateLoadBalancerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateLoadBalancerError {
    fn description(&self) -> &str {
        match *self {
            CreateLoadBalancerError::CertificateNotFound(ref cause) => cause,
            CreateLoadBalancerError::DuplicateAccessPointName(ref cause) => cause,
            CreateLoadBalancerError::DuplicateTagKeys(ref cause) => cause,
            CreateLoadBalancerError::InvalidConfigurationRequest(ref cause) => cause,
            CreateLoadBalancerError::InvalidScheme(ref cause) => cause,
            CreateLoadBalancerError::InvalidSecurityGroup(ref cause) => cause,
            CreateLoadBalancerError::InvalidSubnet(ref cause) => cause,
            CreateLoadBalancerError::OperationNotPermitted(ref cause) => cause,
            CreateLoadBalancerError::SubnetNotFound(ref cause) => cause,
            CreateLoadBalancerError::TooManyAccessPoints(ref cause) => cause,
            CreateLoadBalancerError::TooManyTags(ref cause) => cause,
            CreateLoadBalancerError::UnsupportedProtocol(ref cause) => cause,
            CreateLoadBalancerError::Validation(ref cause) => cause,
            CreateLoadBalancerError::Credentials(ref err) => err.description(),
            CreateLoadBalancerError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateLoadBalancerError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateLoadBalancerListeners
#[derive(Debug, PartialEq)]
pub enum CreateLoadBalancerListenersError {
    /// <p>The specified load balancer does not exist.</p>
    AccessPointNotFound(String),
    /// <p>The specified ARN does not refer to a valid SSL certificate in AWS Identity and Access Management (IAM) or AWS Certificate Manager (ACM). Note that if you recently uploaded the certificate to IAM, this error might indicate that the certificate is not fully available yet.</p>
    CertificateNotFound(String),
    /// <p>A listener already exists for the specified load balancer name and port, but with a different instance port, protocol, or SSL certificate.</p>
    DuplicateListener(String),
    /// <p>The requested configuration change is not valid.</p>
    InvalidConfigurationRequest(String),
    /// <p>The specified protocol or signature version is not supported.</p>
    UnsupportedProtocol(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateLoadBalancerListenersError {
    pub fn from_body(body: &str) -> CreateLoadBalancerListenersError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "LoadBalancerNotFound" => CreateLoadBalancerListenersError::AccessPointNotFound(
                    String::from(parsed_error.message),
                ),
                "CertificateNotFound" => CreateLoadBalancerListenersError::CertificateNotFound(
                    String::from(parsed_error.message),
                ),
                "DuplicateListener" => CreateLoadBalancerListenersError::DuplicateListener(
                    String::from(parsed_error.message),
                ),
                "InvalidConfigurationRequest" => {
                    CreateLoadBalancerListenersError::InvalidConfigurationRequest(String::from(
                        parsed_error.message,
                    ))
                }
                "UnsupportedProtocol" => CreateLoadBalancerListenersError::UnsupportedProtocol(
                    String::from(parsed_error.message),
                ),
                _ => CreateLoadBalancerListenersError::Unknown(String::from(body)),
            },
            Err(_) => CreateLoadBalancerListenersError::Unknown(body.to_string()),
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

impl From<XmlParseError> for CreateLoadBalancerListenersError {
    fn from(err: XmlParseError) -> CreateLoadBalancerListenersError {
        let XmlParseError(message) = err;
        CreateLoadBalancerListenersError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for CreateLoadBalancerListenersError {
    fn from(err: CredentialsError) -> CreateLoadBalancerListenersError {
        CreateLoadBalancerListenersError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateLoadBalancerListenersError {
    fn from(err: HttpDispatchError) -> CreateLoadBalancerListenersError {
        CreateLoadBalancerListenersError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateLoadBalancerListenersError {
    fn from(err: io::Error) -> CreateLoadBalancerListenersError {
        CreateLoadBalancerListenersError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateLoadBalancerListenersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateLoadBalancerListenersError {
    fn description(&self) -> &str {
        match *self {
            CreateLoadBalancerListenersError::AccessPointNotFound(ref cause) => cause,
            CreateLoadBalancerListenersError::CertificateNotFound(ref cause) => cause,
            CreateLoadBalancerListenersError::DuplicateListener(ref cause) => cause,
            CreateLoadBalancerListenersError::InvalidConfigurationRequest(ref cause) => cause,
            CreateLoadBalancerListenersError::UnsupportedProtocol(ref cause) => cause,
            CreateLoadBalancerListenersError::Validation(ref cause) => cause,
            CreateLoadBalancerListenersError::Credentials(ref err) => err.description(),
            CreateLoadBalancerListenersError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateLoadBalancerListenersError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateLoadBalancerPolicy
#[derive(Debug, PartialEq)]
pub enum CreateLoadBalancerPolicyError {
    /// <p>The specified load balancer does not exist.</p>
    AccessPointNotFound(String),
    /// <p>A policy with the specified name already exists for this load balancer.</p>
    DuplicatePolicyName(String),
    /// <p>The requested configuration change is not valid.</p>
    InvalidConfigurationRequest(String),
    /// <p>One or more of the specified policy types do not exist.</p>
    PolicyTypeNotFound(String),
    /// <p>The quota for the number of policies for this load balancer has been reached.</p>
    TooManyPolicies(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateLoadBalancerPolicyError {
    pub fn from_body(body: &str) -> CreateLoadBalancerPolicyError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "LoadBalancerNotFound" => CreateLoadBalancerPolicyError::AccessPointNotFound(
                    String::from(parsed_error.message),
                ),
                "DuplicatePolicyName" => CreateLoadBalancerPolicyError::DuplicatePolicyName(
                    String::from(parsed_error.message),
                ),
                "InvalidConfigurationRequest" => {
                    CreateLoadBalancerPolicyError::InvalidConfigurationRequest(String::from(
                        parsed_error.message,
                    ))
                }
                "PolicyTypeNotFound" => CreateLoadBalancerPolicyError::PolicyTypeNotFound(
                    String::from(parsed_error.message),
                ),
                "TooManyPolicies" => CreateLoadBalancerPolicyError::TooManyPolicies(String::from(
                    parsed_error.message,
                )),
                _ => CreateLoadBalancerPolicyError::Unknown(String::from(body)),
            },
            Err(_) => CreateLoadBalancerPolicyError::Unknown(body.to_string()),
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

impl From<XmlParseError> for CreateLoadBalancerPolicyError {
    fn from(err: XmlParseError) -> CreateLoadBalancerPolicyError {
        let XmlParseError(message) = err;
        CreateLoadBalancerPolicyError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for CreateLoadBalancerPolicyError {
    fn from(err: CredentialsError) -> CreateLoadBalancerPolicyError {
        CreateLoadBalancerPolicyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateLoadBalancerPolicyError {
    fn from(err: HttpDispatchError) -> CreateLoadBalancerPolicyError {
        CreateLoadBalancerPolicyError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateLoadBalancerPolicyError {
    fn from(err: io::Error) -> CreateLoadBalancerPolicyError {
        CreateLoadBalancerPolicyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateLoadBalancerPolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateLoadBalancerPolicyError {
    fn description(&self) -> &str {
        match *self {
            CreateLoadBalancerPolicyError::AccessPointNotFound(ref cause) => cause,
            CreateLoadBalancerPolicyError::DuplicatePolicyName(ref cause) => cause,
            CreateLoadBalancerPolicyError::InvalidConfigurationRequest(ref cause) => cause,
            CreateLoadBalancerPolicyError::PolicyTypeNotFound(ref cause) => cause,
            CreateLoadBalancerPolicyError::TooManyPolicies(ref cause) => cause,
            CreateLoadBalancerPolicyError::Validation(ref cause) => cause,
            CreateLoadBalancerPolicyError::Credentials(ref err) => err.description(),
            CreateLoadBalancerPolicyError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateLoadBalancerPolicyError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteLoadBalancer
#[derive(Debug, PartialEq)]
pub enum DeleteLoadBalancerError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteLoadBalancerError {
    pub fn from_body(body: &str) -> DeleteLoadBalancerError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => DeleteLoadBalancerError::Unknown(String::from(body)),
            },
            Err(_) => DeleteLoadBalancerError::Unknown(body.to_string()),
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

impl From<XmlParseError> for DeleteLoadBalancerError {
    fn from(err: XmlParseError) -> DeleteLoadBalancerError {
        let XmlParseError(message) = err;
        DeleteLoadBalancerError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeleteLoadBalancerError {
    fn from(err: CredentialsError) -> DeleteLoadBalancerError {
        DeleteLoadBalancerError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteLoadBalancerError {
    fn from(err: HttpDispatchError) -> DeleteLoadBalancerError {
        DeleteLoadBalancerError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteLoadBalancerError {
    fn from(err: io::Error) -> DeleteLoadBalancerError {
        DeleteLoadBalancerError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteLoadBalancerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteLoadBalancerError {
    fn description(&self) -> &str {
        match *self {
            DeleteLoadBalancerError::Validation(ref cause) => cause,
            DeleteLoadBalancerError::Credentials(ref err) => err.description(),
            DeleteLoadBalancerError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteLoadBalancerError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteLoadBalancerListeners
#[derive(Debug, PartialEq)]
pub enum DeleteLoadBalancerListenersError {
    /// <p>The specified load balancer does not exist.</p>
    AccessPointNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteLoadBalancerListenersError {
    pub fn from_body(body: &str) -> DeleteLoadBalancerListenersError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "LoadBalancerNotFound" => DeleteLoadBalancerListenersError::AccessPointNotFound(
                    String::from(parsed_error.message),
                ),
                _ => DeleteLoadBalancerListenersError::Unknown(String::from(body)),
            },
            Err(_) => DeleteLoadBalancerListenersError::Unknown(body.to_string()),
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

impl From<XmlParseError> for DeleteLoadBalancerListenersError {
    fn from(err: XmlParseError) -> DeleteLoadBalancerListenersError {
        let XmlParseError(message) = err;
        DeleteLoadBalancerListenersError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeleteLoadBalancerListenersError {
    fn from(err: CredentialsError) -> DeleteLoadBalancerListenersError {
        DeleteLoadBalancerListenersError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteLoadBalancerListenersError {
    fn from(err: HttpDispatchError) -> DeleteLoadBalancerListenersError {
        DeleteLoadBalancerListenersError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteLoadBalancerListenersError {
    fn from(err: io::Error) -> DeleteLoadBalancerListenersError {
        DeleteLoadBalancerListenersError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteLoadBalancerListenersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteLoadBalancerListenersError {
    fn description(&self) -> &str {
        match *self {
            DeleteLoadBalancerListenersError::AccessPointNotFound(ref cause) => cause,
            DeleteLoadBalancerListenersError::Validation(ref cause) => cause,
            DeleteLoadBalancerListenersError::Credentials(ref err) => err.description(),
            DeleteLoadBalancerListenersError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteLoadBalancerListenersError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteLoadBalancerPolicy
#[derive(Debug, PartialEq)]
pub enum DeleteLoadBalancerPolicyError {
    /// <p>The specified load balancer does not exist.</p>
    AccessPointNotFound(String),
    /// <p>The requested configuration change is not valid.</p>
    InvalidConfigurationRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteLoadBalancerPolicyError {
    pub fn from_body(body: &str) -> DeleteLoadBalancerPolicyError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "LoadBalancerNotFound" => DeleteLoadBalancerPolicyError::AccessPointNotFound(
                    String::from(parsed_error.message),
                ),
                "InvalidConfigurationRequest" => {
                    DeleteLoadBalancerPolicyError::InvalidConfigurationRequest(String::from(
                        parsed_error.message,
                    ))
                }
                _ => DeleteLoadBalancerPolicyError::Unknown(String::from(body)),
            },
            Err(_) => DeleteLoadBalancerPolicyError::Unknown(body.to_string()),
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

impl From<XmlParseError> for DeleteLoadBalancerPolicyError {
    fn from(err: XmlParseError) -> DeleteLoadBalancerPolicyError {
        let XmlParseError(message) = err;
        DeleteLoadBalancerPolicyError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeleteLoadBalancerPolicyError {
    fn from(err: CredentialsError) -> DeleteLoadBalancerPolicyError {
        DeleteLoadBalancerPolicyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteLoadBalancerPolicyError {
    fn from(err: HttpDispatchError) -> DeleteLoadBalancerPolicyError {
        DeleteLoadBalancerPolicyError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteLoadBalancerPolicyError {
    fn from(err: io::Error) -> DeleteLoadBalancerPolicyError {
        DeleteLoadBalancerPolicyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteLoadBalancerPolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteLoadBalancerPolicyError {
    fn description(&self) -> &str {
        match *self {
            DeleteLoadBalancerPolicyError::AccessPointNotFound(ref cause) => cause,
            DeleteLoadBalancerPolicyError::InvalidConfigurationRequest(ref cause) => cause,
            DeleteLoadBalancerPolicyError::Validation(ref cause) => cause,
            DeleteLoadBalancerPolicyError::Credentials(ref err) => err.description(),
            DeleteLoadBalancerPolicyError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteLoadBalancerPolicyError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeregisterInstancesFromLoadBalancer
#[derive(Debug, PartialEq)]
pub enum DeregisterInstancesFromLoadBalancerError {
    /// <p>The specified load balancer does not exist.</p>
    AccessPointNotFound(String),
    /// <p>The specified endpoint is not valid.</p>
    InvalidEndPoint(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeregisterInstancesFromLoadBalancerError {
    pub fn from_body(body: &str) -> DeregisterInstancesFromLoadBalancerError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "LoadBalancerNotFound" => {
                    DeregisterInstancesFromLoadBalancerError::AccessPointNotFound(String::from(
                        parsed_error.message,
                    ))
                }
                "InvalidInstance" => DeregisterInstancesFromLoadBalancerError::InvalidEndPoint(
                    String::from(parsed_error.message),
                ),
                _ => DeregisterInstancesFromLoadBalancerError::Unknown(String::from(body)),
            },
            Err(_) => DeregisterInstancesFromLoadBalancerError::Unknown(body.to_string()),
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

impl From<XmlParseError> for DeregisterInstancesFromLoadBalancerError {
    fn from(err: XmlParseError) -> DeregisterInstancesFromLoadBalancerError {
        let XmlParseError(message) = err;
        DeregisterInstancesFromLoadBalancerError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeregisterInstancesFromLoadBalancerError {
    fn from(err: CredentialsError) -> DeregisterInstancesFromLoadBalancerError {
        DeregisterInstancesFromLoadBalancerError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeregisterInstancesFromLoadBalancerError {
    fn from(err: HttpDispatchError) -> DeregisterInstancesFromLoadBalancerError {
        DeregisterInstancesFromLoadBalancerError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeregisterInstancesFromLoadBalancerError {
    fn from(err: io::Error) -> DeregisterInstancesFromLoadBalancerError {
        DeregisterInstancesFromLoadBalancerError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeregisterInstancesFromLoadBalancerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeregisterInstancesFromLoadBalancerError {
    fn description(&self) -> &str {
        match *self {
            DeregisterInstancesFromLoadBalancerError::AccessPointNotFound(ref cause) => cause,
            DeregisterInstancesFromLoadBalancerError::InvalidEndPoint(ref cause) => cause,
            DeregisterInstancesFromLoadBalancerError::Validation(ref cause) => cause,
            DeregisterInstancesFromLoadBalancerError::Credentials(ref err) => err.description(),
            DeregisterInstancesFromLoadBalancerError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeregisterInstancesFromLoadBalancerError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeAccountLimits
#[derive(Debug, PartialEq)]
pub enum DescribeAccountLimitsError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeAccountLimitsError {
    pub fn from_body(body: &str) -> DescribeAccountLimitsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => DescribeAccountLimitsError::Unknown(String::from(body)),
            },
            Err(_) => DescribeAccountLimitsError::Unknown(body.to_string()),
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

impl From<XmlParseError> for DescribeAccountLimitsError {
    fn from(err: XmlParseError) -> DescribeAccountLimitsError {
        let XmlParseError(message) = err;
        DescribeAccountLimitsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DescribeAccountLimitsError {
    fn from(err: CredentialsError) -> DescribeAccountLimitsError {
        DescribeAccountLimitsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeAccountLimitsError {
    fn from(err: HttpDispatchError) -> DescribeAccountLimitsError {
        DescribeAccountLimitsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeAccountLimitsError {
    fn from(err: io::Error) -> DescribeAccountLimitsError {
        DescribeAccountLimitsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeAccountLimitsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeAccountLimitsError {
    fn description(&self) -> &str {
        match *self {
            DescribeAccountLimitsError::Validation(ref cause) => cause,
            DescribeAccountLimitsError::Credentials(ref err) => err.description(),
            DescribeAccountLimitsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeAccountLimitsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeInstanceHealth
#[derive(Debug, PartialEq)]
pub enum DescribeInstanceHealthError {
    /// <p>The specified load balancer does not exist.</p>
    AccessPointNotFound(String),
    /// <p>The specified endpoint is not valid.</p>
    InvalidEndPoint(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeInstanceHealthError {
    pub fn from_body(body: &str) -> DescribeInstanceHealthError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "LoadBalancerNotFound" => DescribeInstanceHealthError::AccessPointNotFound(
                    String::from(parsed_error.message),
                ),
                "InvalidInstance" => {
                    DescribeInstanceHealthError::InvalidEndPoint(String::from(parsed_error.message))
                }
                _ => DescribeInstanceHealthError::Unknown(String::from(body)),
            },
            Err(_) => DescribeInstanceHealthError::Unknown(body.to_string()),
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

impl From<XmlParseError> for DescribeInstanceHealthError {
    fn from(err: XmlParseError) -> DescribeInstanceHealthError {
        let XmlParseError(message) = err;
        DescribeInstanceHealthError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DescribeInstanceHealthError {
    fn from(err: CredentialsError) -> DescribeInstanceHealthError {
        DescribeInstanceHealthError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeInstanceHealthError {
    fn from(err: HttpDispatchError) -> DescribeInstanceHealthError {
        DescribeInstanceHealthError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeInstanceHealthError {
    fn from(err: io::Error) -> DescribeInstanceHealthError {
        DescribeInstanceHealthError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeInstanceHealthError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeInstanceHealthError {
    fn description(&self) -> &str {
        match *self {
            DescribeInstanceHealthError::AccessPointNotFound(ref cause) => cause,
            DescribeInstanceHealthError::InvalidEndPoint(ref cause) => cause,
            DescribeInstanceHealthError::Validation(ref cause) => cause,
            DescribeInstanceHealthError::Credentials(ref err) => err.description(),
            DescribeInstanceHealthError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeInstanceHealthError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeLoadBalancerAttributes
#[derive(Debug, PartialEq)]
pub enum DescribeLoadBalancerAttributesError {
    /// <p>The specified load balancer does not exist.</p>
    AccessPointNotFound(String),
    /// <p>The specified load balancer attribute does not exist.</p>
    LoadBalancerAttributeNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeLoadBalancerAttributesError {
    pub fn from_body(body: &str) -> DescribeLoadBalancerAttributesError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "LoadBalancerNotFound" => DescribeLoadBalancerAttributesError::AccessPointNotFound(
                    String::from(parsed_error.message),
                ),
                "LoadBalancerAttributeNotFound" => {
                    DescribeLoadBalancerAttributesError::LoadBalancerAttributeNotFound(
                        String::from(parsed_error.message),
                    )
                }
                _ => DescribeLoadBalancerAttributesError::Unknown(String::from(body)),
            },
            Err(_) => DescribeLoadBalancerAttributesError::Unknown(body.to_string()),
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

impl From<XmlParseError> for DescribeLoadBalancerAttributesError {
    fn from(err: XmlParseError) -> DescribeLoadBalancerAttributesError {
        let XmlParseError(message) = err;
        DescribeLoadBalancerAttributesError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DescribeLoadBalancerAttributesError {
    fn from(err: CredentialsError) -> DescribeLoadBalancerAttributesError {
        DescribeLoadBalancerAttributesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeLoadBalancerAttributesError {
    fn from(err: HttpDispatchError) -> DescribeLoadBalancerAttributesError {
        DescribeLoadBalancerAttributesError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeLoadBalancerAttributesError {
    fn from(err: io::Error) -> DescribeLoadBalancerAttributesError {
        DescribeLoadBalancerAttributesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeLoadBalancerAttributesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeLoadBalancerAttributesError {
    fn description(&self) -> &str {
        match *self {
            DescribeLoadBalancerAttributesError::AccessPointNotFound(ref cause) => cause,
            DescribeLoadBalancerAttributesError::LoadBalancerAttributeNotFound(ref cause) => cause,
            DescribeLoadBalancerAttributesError::Validation(ref cause) => cause,
            DescribeLoadBalancerAttributesError::Credentials(ref err) => err.description(),
            DescribeLoadBalancerAttributesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeLoadBalancerAttributesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeLoadBalancerPolicies
#[derive(Debug, PartialEq)]
pub enum DescribeLoadBalancerPoliciesError {
    /// <p>The specified load balancer does not exist.</p>
    AccessPointNotFound(String),
    /// <p>One or more of the specified policies do not exist.</p>
    PolicyNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeLoadBalancerPoliciesError {
    pub fn from_body(body: &str) -> DescribeLoadBalancerPoliciesError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "LoadBalancerNotFound" => DescribeLoadBalancerPoliciesError::AccessPointNotFound(
                    String::from(parsed_error.message),
                ),
                "PolicyNotFound" => DescribeLoadBalancerPoliciesError::PolicyNotFound(
                    String::from(parsed_error.message),
                ),
                _ => DescribeLoadBalancerPoliciesError::Unknown(String::from(body)),
            },
            Err(_) => DescribeLoadBalancerPoliciesError::Unknown(body.to_string()),
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

impl From<XmlParseError> for DescribeLoadBalancerPoliciesError {
    fn from(err: XmlParseError) -> DescribeLoadBalancerPoliciesError {
        let XmlParseError(message) = err;
        DescribeLoadBalancerPoliciesError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DescribeLoadBalancerPoliciesError {
    fn from(err: CredentialsError) -> DescribeLoadBalancerPoliciesError {
        DescribeLoadBalancerPoliciesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeLoadBalancerPoliciesError {
    fn from(err: HttpDispatchError) -> DescribeLoadBalancerPoliciesError {
        DescribeLoadBalancerPoliciesError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeLoadBalancerPoliciesError {
    fn from(err: io::Error) -> DescribeLoadBalancerPoliciesError {
        DescribeLoadBalancerPoliciesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeLoadBalancerPoliciesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeLoadBalancerPoliciesError {
    fn description(&self) -> &str {
        match *self {
            DescribeLoadBalancerPoliciesError::AccessPointNotFound(ref cause) => cause,
            DescribeLoadBalancerPoliciesError::PolicyNotFound(ref cause) => cause,
            DescribeLoadBalancerPoliciesError::Validation(ref cause) => cause,
            DescribeLoadBalancerPoliciesError::Credentials(ref err) => err.description(),
            DescribeLoadBalancerPoliciesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeLoadBalancerPoliciesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeLoadBalancerPolicyTypes
#[derive(Debug, PartialEq)]
pub enum DescribeLoadBalancerPolicyTypesError {
    /// <p>One or more of the specified policy types do not exist.</p>
    PolicyTypeNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeLoadBalancerPolicyTypesError {
    pub fn from_body(body: &str) -> DescribeLoadBalancerPolicyTypesError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "PolicyTypeNotFound" => DescribeLoadBalancerPolicyTypesError::PolicyTypeNotFound(
                    String::from(parsed_error.message),
                ),
                _ => DescribeLoadBalancerPolicyTypesError::Unknown(String::from(body)),
            },
            Err(_) => DescribeLoadBalancerPolicyTypesError::Unknown(body.to_string()),
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

impl From<XmlParseError> for DescribeLoadBalancerPolicyTypesError {
    fn from(err: XmlParseError) -> DescribeLoadBalancerPolicyTypesError {
        let XmlParseError(message) = err;
        DescribeLoadBalancerPolicyTypesError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DescribeLoadBalancerPolicyTypesError {
    fn from(err: CredentialsError) -> DescribeLoadBalancerPolicyTypesError {
        DescribeLoadBalancerPolicyTypesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeLoadBalancerPolicyTypesError {
    fn from(err: HttpDispatchError) -> DescribeLoadBalancerPolicyTypesError {
        DescribeLoadBalancerPolicyTypesError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeLoadBalancerPolicyTypesError {
    fn from(err: io::Error) -> DescribeLoadBalancerPolicyTypesError {
        DescribeLoadBalancerPolicyTypesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeLoadBalancerPolicyTypesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeLoadBalancerPolicyTypesError {
    fn description(&self) -> &str {
        match *self {
            DescribeLoadBalancerPolicyTypesError::PolicyTypeNotFound(ref cause) => cause,
            DescribeLoadBalancerPolicyTypesError::Validation(ref cause) => cause,
            DescribeLoadBalancerPolicyTypesError::Credentials(ref err) => err.description(),
            DescribeLoadBalancerPolicyTypesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeLoadBalancerPolicyTypesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeLoadBalancers
#[derive(Debug, PartialEq)]
pub enum DescribeLoadBalancersError {
    /// <p>The specified load balancer does not exist.</p>
    AccessPointNotFound(String),

    DependencyThrottle(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeLoadBalancersError {
    pub fn from_body(body: &str) -> DescribeLoadBalancersError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "LoadBalancerNotFound" => DescribeLoadBalancersError::AccessPointNotFound(
                    String::from(parsed_error.message),
                ),
                "DependencyThrottle" => DescribeLoadBalancersError::DependencyThrottle(
                    String::from(parsed_error.message),
                ),
                _ => DescribeLoadBalancersError::Unknown(String::from(body)),
            },
            Err(_) => DescribeLoadBalancersError::Unknown(body.to_string()),
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

impl From<XmlParseError> for DescribeLoadBalancersError {
    fn from(err: XmlParseError) -> DescribeLoadBalancersError {
        let XmlParseError(message) = err;
        DescribeLoadBalancersError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DescribeLoadBalancersError {
    fn from(err: CredentialsError) -> DescribeLoadBalancersError {
        DescribeLoadBalancersError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeLoadBalancersError {
    fn from(err: HttpDispatchError) -> DescribeLoadBalancersError {
        DescribeLoadBalancersError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeLoadBalancersError {
    fn from(err: io::Error) -> DescribeLoadBalancersError {
        DescribeLoadBalancersError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeLoadBalancersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeLoadBalancersError {
    fn description(&self) -> &str {
        match *self {
            DescribeLoadBalancersError::AccessPointNotFound(ref cause) => cause,
            DescribeLoadBalancersError::DependencyThrottle(ref cause) => cause,
            DescribeLoadBalancersError::Validation(ref cause) => cause,
            DescribeLoadBalancersError::Credentials(ref err) => err.description(),
            DescribeLoadBalancersError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeLoadBalancersError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeTags
#[derive(Debug, PartialEq)]
pub enum DescribeTagsError {
    /// <p>The specified load balancer does not exist.</p>
    AccessPointNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeTagsError {
    pub fn from_body(body: &str) -> DescribeTagsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "LoadBalancerNotFound" => {
                    DescribeTagsError::AccessPointNotFound(String::from(parsed_error.message))
                }
                _ => DescribeTagsError::Unknown(String::from(body)),
            },
            Err(_) => DescribeTagsError::Unknown(body.to_string()),
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

impl From<XmlParseError> for DescribeTagsError {
    fn from(err: XmlParseError) -> DescribeTagsError {
        let XmlParseError(message) = err;
        DescribeTagsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DescribeTagsError {
    fn from(err: CredentialsError) -> DescribeTagsError {
        DescribeTagsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeTagsError {
    fn from(err: HttpDispatchError) -> DescribeTagsError {
        DescribeTagsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeTagsError {
    fn from(err: io::Error) -> DescribeTagsError {
        DescribeTagsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeTagsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeTagsError {
    fn description(&self) -> &str {
        match *self {
            DescribeTagsError::AccessPointNotFound(ref cause) => cause,
            DescribeTagsError::Validation(ref cause) => cause,
            DescribeTagsError::Credentials(ref err) => err.description(),
            DescribeTagsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeTagsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DetachLoadBalancerFromSubnets
#[derive(Debug, PartialEq)]
pub enum DetachLoadBalancerFromSubnetsError {
    /// <p>The specified load balancer does not exist.</p>
    AccessPointNotFound(String),
    /// <p>The requested configuration change is not valid.</p>
    InvalidConfigurationRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DetachLoadBalancerFromSubnetsError {
    pub fn from_body(body: &str) -> DetachLoadBalancerFromSubnetsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "LoadBalancerNotFound" => DetachLoadBalancerFromSubnetsError::AccessPointNotFound(
                    String::from(parsed_error.message),
                ),
                "InvalidConfigurationRequest" => {
                    DetachLoadBalancerFromSubnetsError::InvalidConfigurationRequest(String::from(
                        parsed_error.message,
                    ))
                }
                _ => DetachLoadBalancerFromSubnetsError::Unknown(String::from(body)),
            },
            Err(_) => DetachLoadBalancerFromSubnetsError::Unknown(body.to_string()),
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

impl From<XmlParseError> for DetachLoadBalancerFromSubnetsError {
    fn from(err: XmlParseError) -> DetachLoadBalancerFromSubnetsError {
        let XmlParseError(message) = err;
        DetachLoadBalancerFromSubnetsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DetachLoadBalancerFromSubnetsError {
    fn from(err: CredentialsError) -> DetachLoadBalancerFromSubnetsError {
        DetachLoadBalancerFromSubnetsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DetachLoadBalancerFromSubnetsError {
    fn from(err: HttpDispatchError) -> DetachLoadBalancerFromSubnetsError {
        DetachLoadBalancerFromSubnetsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DetachLoadBalancerFromSubnetsError {
    fn from(err: io::Error) -> DetachLoadBalancerFromSubnetsError {
        DetachLoadBalancerFromSubnetsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DetachLoadBalancerFromSubnetsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DetachLoadBalancerFromSubnetsError {
    fn description(&self) -> &str {
        match *self {
            DetachLoadBalancerFromSubnetsError::AccessPointNotFound(ref cause) => cause,
            DetachLoadBalancerFromSubnetsError::InvalidConfigurationRequest(ref cause) => cause,
            DetachLoadBalancerFromSubnetsError::Validation(ref cause) => cause,
            DetachLoadBalancerFromSubnetsError::Credentials(ref err) => err.description(),
            DetachLoadBalancerFromSubnetsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DetachLoadBalancerFromSubnetsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DisableAvailabilityZonesForLoadBalancer
#[derive(Debug, PartialEq)]
pub enum DisableAvailabilityZonesForLoadBalancerError {
    /// <p>The specified load balancer does not exist.</p>
    AccessPointNotFound(String),
    /// <p>The requested configuration change is not valid.</p>
    InvalidConfigurationRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DisableAvailabilityZonesForLoadBalancerError {
    pub fn from_body(body: &str) -> DisableAvailabilityZonesForLoadBalancerError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "LoadBalancerNotFound" => {
                    DisableAvailabilityZonesForLoadBalancerError::AccessPointNotFound(String::from(
                        parsed_error.message,
                    ))
                }
                "InvalidConfigurationRequest" => {
                    DisableAvailabilityZonesForLoadBalancerError::InvalidConfigurationRequest(
                        String::from(parsed_error.message),
                    )
                }
                _ => DisableAvailabilityZonesForLoadBalancerError::Unknown(String::from(body)),
            },
            Err(_) => DisableAvailabilityZonesForLoadBalancerError::Unknown(body.to_string()),
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

impl From<XmlParseError> for DisableAvailabilityZonesForLoadBalancerError {
    fn from(err: XmlParseError) -> DisableAvailabilityZonesForLoadBalancerError {
        let XmlParseError(message) = err;
        DisableAvailabilityZonesForLoadBalancerError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DisableAvailabilityZonesForLoadBalancerError {
    fn from(err: CredentialsError) -> DisableAvailabilityZonesForLoadBalancerError {
        DisableAvailabilityZonesForLoadBalancerError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DisableAvailabilityZonesForLoadBalancerError {
    fn from(err: HttpDispatchError) -> DisableAvailabilityZonesForLoadBalancerError {
        DisableAvailabilityZonesForLoadBalancerError::HttpDispatch(err)
    }
}
impl From<io::Error> for DisableAvailabilityZonesForLoadBalancerError {
    fn from(err: io::Error) -> DisableAvailabilityZonesForLoadBalancerError {
        DisableAvailabilityZonesForLoadBalancerError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DisableAvailabilityZonesForLoadBalancerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DisableAvailabilityZonesForLoadBalancerError {
    fn description(&self) -> &str {
        match *self {
            DisableAvailabilityZonesForLoadBalancerError::AccessPointNotFound(ref cause) => cause,
            DisableAvailabilityZonesForLoadBalancerError::InvalidConfigurationRequest(
                ref cause,
            ) => cause,
            DisableAvailabilityZonesForLoadBalancerError::Validation(ref cause) => cause,
            DisableAvailabilityZonesForLoadBalancerError::Credentials(ref err) => err.description(),
            DisableAvailabilityZonesForLoadBalancerError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DisableAvailabilityZonesForLoadBalancerError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by EnableAvailabilityZonesForLoadBalancer
#[derive(Debug, PartialEq)]
pub enum EnableAvailabilityZonesForLoadBalancerError {
    /// <p>The specified load balancer does not exist.</p>
    AccessPointNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl EnableAvailabilityZonesForLoadBalancerError {
    pub fn from_body(body: &str) -> EnableAvailabilityZonesForLoadBalancerError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "LoadBalancerNotFound" => {
                    EnableAvailabilityZonesForLoadBalancerError::AccessPointNotFound(String::from(
                        parsed_error.message,
                    ))
                }
                _ => EnableAvailabilityZonesForLoadBalancerError::Unknown(String::from(body)),
            },
            Err(_) => EnableAvailabilityZonesForLoadBalancerError::Unknown(body.to_string()),
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

impl From<XmlParseError> for EnableAvailabilityZonesForLoadBalancerError {
    fn from(err: XmlParseError) -> EnableAvailabilityZonesForLoadBalancerError {
        let XmlParseError(message) = err;
        EnableAvailabilityZonesForLoadBalancerError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for EnableAvailabilityZonesForLoadBalancerError {
    fn from(err: CredentialsError) -> EnableAvailabilityZonesForLoadBalancerError {
        EnableAvailabilityZonesForLoadBalancerError::Credentials(err)
    }
}
impl From<HttpDispatchError> for EnableAvailabilityZonesForLoadBalancerError {
    fn from(err: HttpDispatchError) -> EnableAvailabilityZonesForLoadBalancerError {
        EnableAvailabilityZonesForLoadBalancerError::HttpDispatch(err)
    }
}
impl From<io::Error> for EnableAvailabilityZonesForLoadBalancerError {
    fn from(err: io::Error) -> EnableAvailabilityZonesForLoadBalancerError {
        EnableAvailabilityZonesForLoadBalancerError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for EnableAvailabilityZonesForLoadBalancerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for EnableAvailabilityZonesForLoadBalancerError {
    fn description(&self) -> &str {
        match *self {
            EnableAvailabilityZonesForLoadBalancerError::AccessPointNotFound(ref cause) => cause,
            EnableAvailabilityZonesForLoadBalancerError::Validation(ref cause) => cause,
            EnableAvailabilityZonesForLoadBalancerError::Credentials(ref err) => err.description(),
            EnableAvailabilityZonesForLoadBalancerError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            EnableAvailabilityZonesForLoadBalancerError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ModifyLoadBalancerAttributes
#[derive(Debug, PartialEq)]
pub enum ModifyLoadBalancerAttributesError {
    /// <p>The specified load balancer does not exist.</p>
    AccessPointNotFound(String),
    /// <p>The requested configuration change is not valid.</p>
    InvalidConfigurationRequest(String),
    /// <p>The specified load balancer attribute does not exist.</p>
    LoadBalancerAttributeNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ModifyLoadBalancerAttributesError {
    pub fn from_body(body: &str) -> ModifyLoadBalancerAttributesError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "LoadBalancerNotFound" => ModifyLoadBalancerAttributesError::AccessPointNotFound(
                    String::from(parsed_error.message),
                ),
                "InvalidConfigurationRequest" => {
                    ModifyLoadBalancerAttributesError::InvalidConfigurationRequest(String::from(
                        parsed_error.message,
                    ))
                }
                "LoadBalancerAttributeNotFound" => {
                    ModifyLoadBalancerAttributesError::LoadBalancerAttributeNotFound(String::from(
                        parsed_error.message,
                    ))
                }
                _ => ModifyLoadBalancerAttributesError::Unknown(String::from(body)),
            },
            Err(_) => ModifyLoadBalancerAttributesError::Unknown(body.to_string()),
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

impl From<XmlParseError> for ModifyLoadBalancerAttributesError {
    fn from(err: XmlParseError) -> ModifyLoadBalancerAttributesError {
        let XmlParseError(message) = err;
        ModifyLoadBalancerAttributesError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ModifyLoadBalancerAttributesError {
    fn from(err: CredentialsError) -> ModifyLoadBalancerAttributesError {
        ModifyLoadBalancerAttributesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ModifyLoadBalancerAttributesError {
    fn from(err: HttpDispatchError) -> ModifyLoadBalancerAttributesError {
        ModifyLoadBalancerAttributesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ModifyLoadBalancerAttributesError {
    fn from(err: io::Error) -> ModifyLoadBalancerAttributesError {
        ModifyLoadBalancerAttributesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ModifyLoadBalancerAttributesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ModifyLoadBalancerAttributesError {
    fn description(&self) -> &str {
        match *self {
            ModifyLoadBalancerAttributesError::AccessPointNotFound(ref cause) => cause,
            ModifyLoadBalancerAttributesError::InvalidConfigurationRequest(ref cause) => cause,
            ModifyLoadBalancerAttributesError::LoadBalancerAttributeNotFound(ref cause) => cause,
            ModifyLoadBalancerAttributesError::Validation(ref cause) => cause,
            ModifyLoadBalancerAttributesError::Credentials(ref err) => err.description(),
            ModifyLoadBalancerAttributesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ModifyLoadBalancerAttributesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RegisterInstancesWithLoadBalancer
#[derive(Debug, PartialEq)]
pub enum RegisterInstancesWithLoadBalancerError {
    /// <p>The specified load balancer does not exist.</p>
    AccessPointNotFound(String),
    /// <p>The specified endpoint is not valid.</p>
    InvalidEndPoint(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl RegisterInstancesWithLoadBalancerError {
    pub fn from_body(body: &str) -> RegisterInstancesWithLoadBalancerError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "LoadBalancerNotFound" => {
                    RegisterInstancesWithLoadBalancerError::AccessPointNotFound(String::from(
                        parsed_error.message,
                    ))
                }
                "InvalidInstance" => RegisterInstancesWithLoadBalancerError::InvalidEndPoint(
                    String::from(parsed_error.message),
                ),
                _ => RegisterInstancesWithLoadBalancerError::Unknown(String::from(body)),
            },
            Err(_) => RegisterInstancesWithLoadBalancerError::Unknown(body.to_string()),
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

impl From<XmlParseError> for RegisterInstancesWithLoadBalancerError {
    fn from(err: XmlParseError) -> RegisterInstancesWithLoadBalancerError {
        let XmlParseError(message) = err;
        RegisterInstancesWithLoadBalancerError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for RegisterInstancesWithLoadBalancerError {
    fn from(err: CredentialsError) -> RegisterInstancesWithLoadBalancerError {
        RegisterInstancesWithLoadBalancerError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RegisterInstancesWithLoadBalancerError {
    fn from(err: HttpDispatchError) -> RegisterInstancesWithLoadBalancerError {
        RegisterInstancesWithLoadBalancerError::HttpDispatch(err)
    }
}
impl From<io::Error> for RegisterInstancesWithLoadBalancerError {
    fn from(err: io::Error) -> RegisterInstancesWithLoadBalancerError {
        RegisterInstancesWithLoadBalancerError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RegisterInstancesWithLoadBalancerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RegisterInstancesWithLoadBalancerError {
    fn description(&self) -> &str {
        match *self {
            RegisterInstancesWithLoadBalancerError::AccessPointNotFound(ref cause) => cause,
            RegisterInstancesWithLoadBalancerError::InvalidEndPoint(ref cause) => cause,
            RegisterInstancesWithLoadBalancerError::Validation(ref cause) => cause,
            RegisterInstancesWithLoadBalancerError::Credentials(ref err) => err.description(),
            RegisterInstancesWithLoadBalancerError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            RegisterInstancesWithLoadBalancerError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RemoveTags
#[derive(Debug, PartialEq)]
pub enum RemoveTagsError {
    /// <p>The specified load balancer does not exist.</p>
    AccessPointNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl RemoveTagsError {
    pub fn from_body(body: &str) -> RemoveTagsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "LoadBalancerNotFound" => {
                    RemoveTagsError::AccessPointNotFound(String::from(parsed_error.message))
                }
                _ => RemoveTagsError::Unknown(String::from(body)),
            },
            Err(_) => RemoveTagsError::Unknown(body.to_string()),
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

impl From<XmlParseError> for RemoveTagsError {
    fn from(err: XmlParseError) -> RemoveTagsError {
        let XmlParseError(message) = err;
        RemoveTagsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for RemoveTagsError {
    fn from(err: CredentialsError) -> RemoveTagsError {
        RemoveTagsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RemoveTagsError {
    fn from(err: HttpDispatchError) -> RemoveTagsError {
        RemoveTagsError::HttpDispatch(err)
    }
}
impl From<io::Error> for RemoveTagsError {
    fn from(err: io::Error) -> RemoveTagsError {
        RemoveTagsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RemoveTagsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RemoveTagsError {
    fn description(&self) -> &str {
        match *self {
            RemoveTagsError::AccessPointNotFound(ref cause) => cause,
            RemoveTagsError::Validation(ref cause) => cause,
            RemoveTagsError::Credentials(ref err) => err.description(),
            RemoveTagsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            RemoveTagsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by SetLoadBalancerListenerSSLCertificate
#[derive(Debug, PartialEq)]
pub enum SetLoadBalancerListenerSSLCertificateError {
    /// <p>The specified load balancer does not exist.</p>
    AccessPointNotFound(String),
    /// <p>The specified ARN does not refer to a valid SSL certificate in AWS Identity and Access Management (IAM) or AWS Certificate Manager (ACM). Note that if you recently uploaded the certificate to IAM, this error might indicate that the certificate is not fully available yet.</p>
    CertificateNotFound(String),
    /// <p>The requested configuration change is not valid.</p>
    InvalidConfigurationRequest(String),
    /// <p>The load balancer does not have a listener configured at the specified port.</p>
    ListenerNotFound(String),
    /// <p>The specified protocol or signature version is not supported.</p>
    UnsupportedProtocol(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl SetLoadBalancerListenerSSLCertificateError {
    pub fn from_body(body: &str) -> SetLoadBalancerListenerSSLCertificateError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "LoadBalancerNotFound" => {
                    SetLoadBalancerListenerSSLCertificateError::AccessPointNotFound(String::from(
                        parsed_error.message,
                    ))
                }
                "CertificateNotFound" => {
                    SetLoadBalancerListenerSSLCertificateError::CertificateNotFound(String::from(
                        parsed_error.message,
                    ))
                }
                "InvalidConfigurationRequest" => {
                    SetLoadBalancerListenerSSLCertificateError::InvalidConfigurationRequest(
                        String::from(parsed_error.message),
                    )
                }
                "ListenerNotFound" => SetLoadBalancerListenerSSLCertificateError::ListenerNotFound(
                    String::from(parsed_error.message),
                ),
                "UnsupportedProtocol" => {
                    SetLoadBalancerListenerSSLCertificateError::UnsupportedProtocol(String::from(
                        parsed_error.message,
                    ))
                }
                _ => SetLoadBalancerListenerSSLCertificateError::Unknown(String::from(body)),
            },
            Err(_) => SetLoadBalancerListenerSSLCertificateError::Unknown(body.to_string()),
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

impl From<XmlParseError> for SetLoadBalancerListenerSSLCertificateError {
    fn from(err: XmlParseError) -> SetLoadBalancerListenerSSLCertificateError {
        let XmlParseError(message) = err;
        SetLoadBalancerListenerSSLCertificateError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for SetLoadBalancerListenerSSLCertificateError {
    fn from(err: CredentialsError) -> SetLoadBalancerListenerSSLCertificateError {
        SetLoadBalancerListenerSSLCertificateError::Credentials(err)
    }
}
impl From<HttpDispatchError> for SetLoadBalancerListenerSSLCertificateError {
    fn from(err: HttpDispatchError) -> SetLoadBalancerListenerSSLCertificateError {
        SetLoadBalancerListenerSSLCertificateError::HttpDispatch(err)
    }
}
impl From<io::Error> for SetLoadBalancerListenerSSLCertificateError {
    fn from(err: io::Error) -> SetLoadBalancerListenerSSLCertificateError {
        SetLoadBalancerListenerSSLCertificateError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for SetLoadBalancerListenerSSLCertificateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SetLoadBalancerListenerSSLCertificateError {
    fn description(&self) -> &str {
        match *self {
            SetLoadBalancerListenerSSLCertificateError::AccessPointNotFound(ref cause) => cause,
            SetLoadBalancerListenerSSLCertificateError::CertificateNotFound(ref cause) => cause,
            SetLoadBalancerListenerSSLCertificateError::InvalidConfigurationRequest(ref cause) => {
                cause
            }
            SetLoadBalancerListenerSSLCertificateError::ListenerNotFound(ref cause) => cause,
            SetLoadBalancerListenerSSLCertificateError::UnsupportedProtocol(ref cause) => cause,
            SetLoadBalancerListenerSSLCertificateError::Validation(ref cause) => cause,
            SetLoadBalancerListenerSSLCertificateError::Credentials(ref err) => err.description(),
            SetLoadBalancerListenerSSLCertificateError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            SetLoadBalancerListenerSSLCertificateError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by SetLoadBalancerPoliciesForBackendServer
#[derive(Debug, PartialEq)]
pub enum SetLoadBalancerPoliciesForBackendServerError {
    /// <p>The specified load balancer does not exist.</p>
    AccessPointNotFound(String),
    /// <p>The requested configuration change is not valid.</p>
    InvalidConfigurationRequest(String),
    /// <p>One or more of the specified policies do not exist.</p>
    PolicyNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl SetLoadBalancerPoliciesForBackendServerError {
    pub fn from_body(body: &str) -> SetLoadBalancerPoliciesForBackendServerError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "LoadBalancerNotFound" => {
                    SetLoadBalancerPoliciesForBackendServerError::AccessPointNotFound(String::from(
                        parsed_error.message,
                    ))
                }
                "InvalidConfigurationRequest" => {
                    SetLoadBalancerPoliciesForBackendServerError::InvalidConfigurationRequest(
                        String::from(parsed_error.message),
                    )
                }
                "PolicyNotFound" => SetLoadBalancerPoliciesForBackendServerError::PolicyNotFound(
                    String::from(parsed_error.message),
                ),
                _ => SetLoadBalancerPoliciesForBackendServerError::Unknown(String::from(body)),
            },
            Err(_) => SetLoadBalancerPoliciesForBackendServerError::Unknown(body.to_string()),
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

impl From<XmlParseError> for SetLoadBalancerPoliciesForBackendServerError {
    fn from(err: XmlParseError) -> SetLoadBalancerPoliciesForBackendServerError {
        let XmlParseError(message) = err;
        SetLoadBalancerPoliciesForBackendServerError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for SetLoadBalancerPoliciesForBackendServerError {
    fn from(err: CredentialsError) -> SetLoadBalancerPoliciesForBackendServerError {
        SetLoadBalancerPoliciesForBackendServerError::Credentials(err)
    }
}
impl From<HttpDispatchError> for SetLoadBalancerPoliciesForBackendServerError {
    fn from(err: HttpDispatchError) -> SetLoadBalancerPoliciesForBackendServerError {
        SetLoadBalancerPoliciesForBackendServerError::HttpDispatch(err)
    }
}
impl From<io::Error> for SetLoadBalancerPoliciesForBackendServerError {
    fn from(err: io::Error) -> SetLoadBalancerPoliciesForBackendServerError {
        SetLoadBalancerPoliciesForBackendServerError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for SetLoadBalancerPoliciesForBackendServerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SetLoadBalancerPoliciesForBackendServerError {
    fn description(&self) -> &str {
        match *self {
            SetLoadBalancerPoliciesForBackendServerError::AccessPointNotFound(ref cause) => cause,
            SetLoadBalancerPoliciesForBackendServerError::InvalidConfigurationRequest(
                ref cause,
            ) => cause,
            SetLoadBalancerPoliciesForBackendServerError::PolicyNotFound(ref cause) => cause,
            SetLoadBalancerPoliciesForBackendServerError::Validation(ref cause) => cause,
            SetLoadBalancerPoliciesForBackendServerError::Credentials(ref err) => err.description(),
            SetLoadBalancerPoliciesForBackendServerError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            SetLoadBalancerPoliciesForBackendServerError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by SetLoadBalancerPoliciesOfListener
#[derive(Debug, PartialEq)]
pub enum SetLoadBalancerPoliciesOfListenerError {
    /// <p>The specified load balancer does not exist.</p>
    AccessPointNotFound(String),
    /// <p>The requested configuration change is not valid.</p>
    InvalidConfigurationRequest(String),
    /// <p>The load balancer does not have a listener configured at the specified port.</p>
    ListenerNotFound(String),
    /// <p>One or more of the specified policies do not exist.</p>
    PolicyNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl SetLoadBalancerPoliciesOfListenerError {
    pub fn from_body(body: &str) -> SetLoadBalancerPoliciesOfListenerError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "LoadBalancerNotFound" => {
                    SetLoadBalancerPoliciesOfListenerError::AccessPointNotFound(String::from(
                        parsed_error.message,
                    ))
                }
                "InvalidConfigurationRequest" => {
                    SetLoadBalancerPoliciesOfListenerError::InvalidConfigurationRequest(
                        String::from(parsed_error.message),
                    )
                }
                "ListenerNotFound" => SetLoadBalancerPoliciesOfListenerError::ListenerNotFound(
                    String::from(parsed_error.message),
                ),
                "PolicyNotFound" => SetLoadBalancerPoliciesOfListenerError::PolicyNotFound(
                    String::from(parsed_error.message),
                ),
                _ => SetLoadBalancerPoliciesOfListenerError::Unknown(String::from(body)),
            },
            Err(_) => SetLoadBalancerPoliciesOfListenerError::Unknown(body.to_string()),
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

impl From<XmlParseError> for SetLoadBalancerPoliciesOfListenerError {
    fn from(err: XmlParseError) -> SetLoadBalancerPoliciesOfListenerError {
        let XmlParseError(message) = err;
        SetLoadBalancerPoliciesOfListenerError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for SetLoadBalancerPoliciesOfListenerError {
    fn from(err: CredentialsError) -> SetLoadBalancerPoliciesOfListenerError {
        SetLoadBalancerPoliciesOfListenerError::Credentials(err)
    }
}
impl From<HttpDispatchError> for SetLoadBalancerPoliciesOfListenerError {
    fn from(err: HttpDispatchError) -> SetLoadBalancerPoliciesOfListenerError {
        SetLoadBalancerPoliciesOfListenerError::HttpDispatch(err)
    }
}
impl From<io::Error> for SetLoadBalancerPoliciesOfListenerError {
    fn from(err: io::Error) -> SetLoadBalancerPoliciesOfListenerError {
        SetLoadBalancerPoliciesOfListenerError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for SetLoadBalancerPoliciesOfListenerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SetLoadBalancerPoliciesOfListenerError {
    fn description(&self) -> &str {
        match *self {
            SetLoadBalancerPoliciesOfListenerError::AccessPointNotFound(ref cause) => cause,
            SetLoadBalancerPoliciesOfListenerError::InvalidConfigurationRequest(ref cause) => cause,
            SetLoadBalancerPoliciesOfListenerError::ListenerNotFound(ref cause) => cause,
            SetLoadBalancerPoliciesOfListenerError::PolicyNotFound(ref cause) => cause,
            SetLoadBalancerPoliciesOfListenerError::Validation(ref cause) => cause,
            SetLoadBalancerPoliciesOfListenerError::Credentials(ref err) => err.description(),
            SetLoadBalancerPoliciesOfListenerError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            SetLoadBalancerPoliciesOfListenerError::Unknown(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the Elastic Load Balancing API. Elastic Load Balancing clients implement this trait.
pub trait Elb {
    /// <p>Adds the specified tags to the specified load balancer. Each load balancer can have a maximum of 10 tags.</p> <p>Each tag consists of a key and an optional value. If a tag with the same key is already associated with the load balancer, <code>AddTags</code> updates its value.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/elasticloadbalancing/latest/classic/add-remove-tags.html">Tag Your Classic Load Balancer</a> in the <i>Classic Load Balancer Guide</i>.</p>
    fn add_tags(&self, input: AddTagsInput) -> RusotoFuture<AddTagsOutput, AddTagsError>;

    /// <p>Associates one or more security groups with your load balancer in a virtual private cloud (VPC). The specified security groups override the previously associated security groups.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/elasticloadbalancing/latest/classic/elb-security-groups.html#elb-vpc-security-groups">Security Groups for Load Balancers in a VPC</a> in the <i>Classic Load Balancer Guide</i>.</p>
    fn apply_security_groups_to_load_balancer(
        &self,
        input: ApplySecurityGroupsToLoadBalancerInput,
    ) -> RusotoFuture<ApplySecurityGroupsToLoadBalancerOutput, ApplySecurityGroupsToLoadBalancerError>;

    /// <p>Adds one or more subnets to the set of configured subnets for the specified load balancer.</p> <p>The load balancer evenly distributes requests across all registered subnets. For more information, see <a href="http://docs.aws.amazon.com/elasticloadbalancing/latest/classic/elb-manage-subnets.html">Add or Remove Subnets for Your Load Balancer in a VPC</a> in the <i>Classic Load Balancer Guide</i>.</p>
    fn attach_load_balancer_to_subnets(
        &self,
        input: AttachLoadBalancerToSubnetsInput,
    ) -> RusotoFuture<AttachLoadBalancerToSubnetsOutput, AttachLoadBalancerToSubnetsError>;

    /// <p>Specifies the health check settings to use when evaluating the health state of your EC2 instances.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/elasticloadbalancing/latest/classic/elb-healthchecks.html">Configure Health Checks for Your Load Balancer</a> in the <i>Classic Load Balancer Guide</i>.</p>
    fn configure_health_check(
        &self,
        input: ConfigureHealthCheckInput,
    ) -> RusotoFuture<ConfigureHealthCheckOutput, ConfigureHealthCheckError>;

    /// <p>Generates a stickiness policy with sticky session lifetimes that follow that of an application-generated cookie. This policy can be associated only with HTTP/HTTPS listeners.</p> <p>This policy is similar to the policy created by <a>CreateLBCookieStickinessPolicy</a>, except that the lifetime of the special Elastic Load Balancing cookie, <code>AWSELB</code>, follows the lifetime of the application-generated cookie specified in the policy configuration. The load balancer only inserts a new stickiness cookie when the application response includes a new application cookie.</p> <p>If the application cookie is explicitly removed or expires, the session stops being sticky until a new application cookie is issued.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/elasticloadbalancing/latest/classic/elb-sticky-sessions.html#enable-sticky-sessions-application">Application-Controlled Session Stickiness</a> in the <i>Classic Load Balancer Guide</i>.</p>
    fn create_app_cookie_stickiness_policy(
        &self,
        input: CreateAppCookieStickinessPolicyInput,
    ) -> RusotoFuture<CreateAppCookieStickinessPolicyOutput, CreateAppCookieStickinessPolicyError>;

    /// <p>Generates a stickiness policy with sticky session lifetimes controlled by the lifetime of the browser (user-agent) or a specified expiration period. This policy can be associated only with HTTP/HTTPS listeners.</p> <p>When a load balancer implements this policy, the load balancer uses a special cookie to track the instance for each request. When the load balancer receives a request, it first checks to see if this cookie is present in the request. If so, the load balancer sends the request to the application server specified in the cookie. If not, the load balancer sends the request to a server that is chosen based on the existing load-balancing algorithm.</p> <p>A cookie is inserted into the response for binding subsequent requests from the same user to that server. The validity of the cookie is based on the cookie expiration time, which is specified in the policy configuration.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/elasticloadbalancing/latest/classic/elb-sticky-sessions.html#enable-sticky-sessions-duration">Duration-Based Session Stickiness</a> in the <i>Classic Load Balancer Guide</i>.</p>
    fn create_lb_cookie_stickiness_policy(
        &self,
        input: CreateLBCookieStickinessPolicyInput,
    ) -> RusotoFuture<CreateLBCookieStickinessPolicyOutput, CreateLBCookieStickinessPolicyError>;

    /// <p>Creates a Classic Load Balancer.</p> <p>You can add listeners, security groups, subnets, and tags when you create your load balancer, or you can add them later using <a>CreateLoadBalancerListeners</a>, <a>ApplySecurityGroupsToLoadBalancer</a>, <a>AttachLoadBalancerToSubnets</a>, and <a>AddTags</a>.</p> <p>To describe your current load balancers, see <a>DescribeLoadBalancers</a>. When you are finished with a load balancer, you can delete it using <a>DeleteLoadBalancer</a>.</p> <p>You can create up to 20 load balancers per region per account. You can request an increase for the number of load balancers for your account. For more information, see <a href="http://docs.aws.amazon.com/elasticloadbalancing/latest/classic/elb-limits.html">Limits for Your Classic Load Balancer</a> in the <i>Classic Load Balancer Guide</i>.</p>
    fn create_load_balancer(
        &self,
        input: CreateAccessPointInput,
    ) -> RusotoFuture<CreateAccessPointOutput, CreateLoadBalancerError>;

    /// <p>Creates one or more listeners for the specified load balancer. If a listener with the specified port does not already exist, it is created; otherwise, the properties of the new listener must match the properties of the existing listener.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/elasticloadbalancing/latest/classic/elb-listener-config.html">Listeners for Your Classic Load Balancer</a> in the <i>Classic Load Balancer Guide</i>.</p>
    fn create_load_balancer_listeners(
        &self,
        input: CreateLoadBalancerListenerInput,
    ) -> RusotoFuture<CreateLoadBalancerListenerOutput, CreateLoadBalancerListenersError>;

    /// <p>Creates a policy with the specified attributes for the specified load balancer.</p> <p>Policies are settings that are saved for your load balancer and that can be applied to the listener or the application server, depending on the policy type.</p>
    fn create_load_balancer_policy(
        &self,
        input: CreateLoadBalancerPolicyInput,
    ) -> RusotoFuture<CreateLoadBalancerPolicyOutput, CreateLoadBalancerPolicyError>;

    /// <p>Deletes the specified load balancer.</p> <p>If you are attempting to recreate a load balancer, you must reconfigure all settings. The DNS name associated with a deleted load balancer are no longer usable. The name and associated DNS record of the deleted load balancer no longer exist and traffic sent to any of its IP addresses is no longer delivered to your instances.</p> <p>If the load balancer does not exist or has already been deleted, the call to <code>DeleteLoadBalancer</code> still succeeds.</p>
    fn delete_load_balancer(
        &self,
        input: DeleteAccessPointInput,
    ) -> RusotoFuture<DeleteAccessPointOutput, DeleteLoadBalancerError>;

    /// <p>Deletes the specified listeners from the specified load balancer.</p>
    fn delete_load_balancer_listeners(
        &self,
        input: DeleteLoadBalancerListenerInput,
    ) -> RusotoFuture<DeleteLoadBalancerListenerOutput, DeleteLoadBalancerListenersError>;

    /// <p>Deletes the specified policy from the specified load balancer. This policy must not be enabled for any listeners.</p>
    fn delete_load_balancer_policy(
        &self,
        input: DeleteLoadBalancerPolicyInput,
    ) -> RusotoFuture<DeleteLoadBalancerPolicyOutput, DeleteLoadBalancerPolicyError>;

    /// <p>Deregisters the specified instances from the specified load balancer. After the instance is deregistered, it no longer receives traffic from the load balancer.</p> <p>You can use <a>DescribeLoadBalancers</a> to verify that the instance is deregistered from the load balancer.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/elasticloadbalancing/latest/classic/elb-deregister-register-instances.html">Register or De-Register EC2 Instances</a> in the <i>Classic Load Balancer Guide</i>.</p>
    fn deregister_instances_from_load_balancer(
        &self,
        input: DeregisterEndPointsInput,
    ) -> RusotoFuture<DeregisterEndPointsOutput, DeregisterInstancesFromLoadBalancerError>;

    /// <p>Describes the current Elastic Load Balancing resource limits for your AWS account.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/elasticloadbalancing/latest/classic/elb-limits.html">Limits for Your Classic Load Balancer</a> in the <i>Classic Load Balancer Guide</i>.</p>
    fn describe_account_limits(
        &self,
        input: DescribeAccountLimitsInput,
    ) -> RusotoFuture<DescribeAccountLimitsOutput, DescribeAccountLimitsError>;

    /// <p>Describes the state of the specified instances with respect to the specified load balancer. If no instances are specified, the call describes the state of all instances that are currently registered with the load balancer. If instances are specified, their state is returned even if they are no longer registered with the load balancer. The state of terminated instances is not returned.</p>
    fn describe_instance_health(
        &self,
        input: DescribeEndPointStateInput,
    ) -> RusotoFuture<DescribeEndPointStateOutput, DescribeInstanceHealthError>;

    /// <p>Describes the attributes for the specified load balancer.</p>
    fn describe_load_balancer_attributes(
        &self,
        input: DescribeLoadBalancerAttributesInput,
    ) -> RusotoFuture<DescribeLoadBalancerAttributesOutput, DescribeLoadBalancerAttributesError>;

    /// <p>Describes the specified policies.</p> <p>If you specify a load balancer name, the action returns the descriptions of all policies created for the load balancer. If you specify a policy name associated with your load balancer, the action returns the description of that policy. If you don't specify a load balancer name, the action returns descriptions of the specified sample policies, or descriptions of all sample policies. The names of the sample policies have the <code>ELBSample-</code> prefix.</p>
    fn describe_load_balancer_policies(
        &self,
        input: DescribeLoadBalancerPoliciesInput,
    ) -> RusotoFuture<DescribeLoadBalancerPoliciesOutput, DescribeLoadBalancerPoliciesError>;

    /// <p>Describes the specified load balancer policy types or all load balancer policy types.</p> <p>The description of each type indicates how it can be used. For example, some policies can be used only with layer 7 listeners, some policies can be used only with layer 4 listeners, and some policies can be used only with your EC2 instances.</p> <p>You can use <a>CreateLoadBalancerPolicy</a> to create a policy configuration for any of these policy types. Then, depending on the policy type, use either <a>SetLoadBalancerPoliciesOfListener</a> or <a>SetLoadBalancerPoliciesForBackendServer</a> to set the policy.</p>
    fn describe_load_balancer_policy_types(
        &self,
        input: DescribeLoadBalancerPolicyTypesInput,
    ) -> RusotoFuture<DescribeLoadBalancerPolicyTypesOutput, DescribeLoadBalancerPolicyTypesError>;

    /// <p>Describes the specified the load balancers. If no load balancers are specified, the call describes all of your load balancers.</p>
    fn describe_load_balancers(
        &self,
        input: DescribeAccessPointsInput,
    ) -> RusotoFuture<DescribeAccessPointsOutput, DescribeLoadBalancersError>;

    /// <p>Describes the tags associated with the specified load balancers.</p>
    fn describe_tags(
        &self,
        input: DescribeTagsInput,
    ) -> RusotoFuture<DescribeTagsOutput, DescribeTagsError>;

    /// <p>Removes the specified subnets from the set of configured subnets for the load balancer.</p> <p>After a subnet is removed, all EC2 instances registered with the load balancer in the removed subnet go into the <code>OutOfService</code> state. Then, the load balancer balances the traffic among the remaining routable subnets.</p>
    fn detach_load_balancer_from_subnets(
        &self,
        input: DetachLoadBalancerFromSubnetsInput,
    ) -> RusotoFuture<DetachLoadBalancerFromSubnetsOutput, DetachLoadBalancerFromSubnetsError>;

    /// <p>Removes the specified Availability Zones from the set of Availability Zones for the specified load balancer.</p> <p>There must be at least one Availability Zone registered with a load balancer at all times. After an Availability Zone is removed, all instances registered with the load balancer that are in the removed Availability Zone go into the <code>OutOfService</code> state. Then, the load balancer attempts to equally balance the traffic among its remaining Availability Zones.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/elasticloadbalancing/latest/classic/enable-disable-az.html">Add or Remove Availability Zones</a> in the <i>Classic Load Balancer Guide</i>.</p>
    fn disable_availability_zones_for_load_balancer(
        &self,
        input: RemoveAvailabilityZonesInput,
    ) -> RusotoFuture<RemoveAvailabilityZonesOutput, DisableAvailabilityZonesForLoadBalancerError>;

    /// <p>Adds the specified Availability Zones to the set of Availability Zones for the specified load balancer.</p> <p>The load balancer evenly distributes requests across all its registered Availability Zones that contain instances.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/elasticloadbalancing/latest/classic/enable-disable-az.html">Add or Remove Availability Zones</a> in the <i>Classic Load Balancer Guide</i>.</p>
    fn enable_availability_zones_for_load_balancer(
        &self,
        input: AddAvailabilityZonesInput,
    ) -> RusotoFuture<AddAvailabilityZonesOutput, EnableAvailabilityZonesForLoadBalancerError>;

    /// <p><p>Modifies the attributes of the specified load balancer.</p> <p>You can modify the load balancer attributes, such as <code>AccessLogs</code>, <code>ConnectionDraining</code>, and <code>CrossZoneLoadBalancing</code> by either enabling or disabling them. Or, you can modify the load balancer attribute <code>ConnectionSettings</code> by specifying an idle connection timeout value for your load balancer.</p> <p>For more information, see the following in the <i>Classic Load Balancer Guide</i>:</p> <ul> <li> <p> <a href="http://docs.aws.amazon.com/elasticloadbalancing/latest/classic/enable-disable-crosszone-lb.html">Cross-Zone Load Balancing</a> </p> </li> <li> <p> <a href="http://docs.aws.amazon.com/elasticloadbalancing/latest/classic/config-conn-drain.html">Connection Draining</a> </p> </li> <li> <p> <a href="http://docs.aws.amazon.com/elasticloadbalancing/latest/classic/access-log-collection.html">Access Logs</a> </p> </li> <li> <p> <a href="http://docs.aws.amazon.com/elasticloadbalancing/latest/classic/config-idle-timeout.html">Idle Connection Timeout</a> </p> </li> </ul></p>
    fn modify_load_balancer_attributes(
        &self,
        input: ModifyLoadBalancerAttributesInput,
    ) -> RusotoFuture<ModifyLoadBalancerAttributesOutput, ModifyLoadBalancerAttributesError>;

    /// <p>Adds the specified instances to the specified load balancer.</p> <p>The instance must be a running instance in the same network as the load balancer (EC2-Classic or the same VPC). If you have EC2-Classic instances and a load balancer in a VPC with ClassicLink enabled, you can link the EC2-Classic instances to that VPC and then register the linked EC2-Classic instances with the load balancer in the VPC.</p> <p>Note that <code>RegisterInstanceWithLoadBalancer</code> completes when the request has been registered. Instance registration takes a little time to complete. To check the state of the registered instances, use <a>DescribeLoadBalancers</a> or <a>DescribeInstanceHealth</a>.</p> <p>After the instance is registered, it starts receiving traffic and requests from the load balancer. Any instance that is not in one of the Availability Zones registered for the load balancer is moved to the <code>OutOfService</code> state. If an Availability Zone is added to the load balancer later, any instances registered with the load balancer move to the <code>InService</code> state.</p> <p>To deregister instances from a load balancer, use <a>DeregisterInstancesFromLoadBalancer</a>.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/elasticloadbalancing/latest/classic/elb-deregister-register-instances.html">Register or De-Register EC2 Instances</a> in the <i>Classic Load Balancer Guide</i>.</p>
    fn register_instances_with_load_balancer(
        &self,
        input: RegisterEndPointsInput,
    ) -> RusotoFuture<RegisterEndPointsOutput, RegisterInstancesWithLoadBalancerError>;

    /// <p>Removes one or more tags from the specified load balancer.</p>
    fn remove_tags(
        &self,
        input: RemoveTagsInput,
    ) -> RusotoFuture<RemoveTagsOutput, RemoveTagsError>;

    /// <p>Sets the certificate that terminates the specified listener's SSL connections. The specified certificate replaces any prior certificate that was used on the same load balancer and port.</p> <p>For more information about updating your SSL certificate, see <a href="http://docs.aws.amazon.com/elasticloadbalancing/latest/classic/elb-update-ssl-cert.html">Replace the SSL Certificate for Your Load Balancer</a> in the <i>Classic Load Balancer Guide</i>.</p>
    fn set_load_balancer_listener_ssl_certificate(
        &self,
        input: SetLoadBalancerListenerSSLCertificateInput,
    ) -> RusotoFuture<
        SetLoadBalancerListenerSSLCertificateOutput,
        SetLoadBalancerListenerSSLCertificateError,
    >;

    /// <p>Replaces the set of policies associated with the specified port on which the EC2 instance is listening with a new set of policies. At this time, only the back-end server authentication policy type can be applied to the instance ports; this policy type is composed of multiple public key policies.</p> <p>Each time you use <code>SetLoadBalancerPoliciesForBackendServer</code> to enable the policies, use the <code>PolicyNames</code> parameter to list the policies that you want to enable.</p> <p>You can use <a>DescribeLoadBalancers</a> or <a>DescribeLoadBalancerPolicies</a> to verify that the policy is associated with the EC2 instance.</p> <p>For more information about enabling back-end instance authentication, see <a href="http://docs.aws.amazon.com/elasticloadbalancing/latest/classic/elb-create-https-ssl-load-balancer.html#configure_backendauth_clt">Configure Back-end Instance Authentication</a> in the <i>Classic Load Balancer Guide</i>. For more information about Proxy Protocol, see <a href="http://docs.aws.amazon.com/elasticloadbalancing/latest/classic/enable-proxy-protocol.html">Configure Proxy Protocol Support</a> in the <i>Classic Load Balancer Guide</i>.</p>
    fn set_load_balancer_policies_for_backend_server(
        &self,
        input: SetLoadBalancerPoliciesForBackendServerInput,
    ) -> RusotoFuture<
        SetLoadBalancerPoliciesForBackendServerOutput,
        SetLoadBalancerPoliciesForBackendServerError,
    >;

    /// <p>Replaces the current set of policies for the specified load balancer port with the specified set of policies.</p> <p>To enable back-end server authentication, use <a>SetLoadBalancerPoliciesForBackendServer</a>.</p> <p>For more information about setting policies, see <a href="http://docs.aws.amazon.com/elasticloadbalancing/latest/classic/ssl-config-update.html">Update the SSL Negotiation Configuration</a>, <a href="http://docs.aws.amazon.com/elasticloadbalancing/latest/classic/elb-sticky-sessions.html#enable-sticky-sessions-duration">Duration-Based Session Stickiness</a>, and <a href="http://docs.aws.amazon.com/elasticloadbalancing/latest/classic/elb-sticky-sessions.html#enable-sticky-sessions-application">Application-Controlled Session Stickiness</a> in the <i>Classic Load Balancer Guide</i>.</p>
    fn set_load_balancer_policies_of_listener(
        &self,
        input: SetLoadBalancerPoliciesOfListenerInput,
    ) -> RusotoFuture<SetLoadBalancerPoliciesOfListenerOutput, SetLoadBalancerPoliciesOfListenerError>;
}
/// A client for the Elastic Load Balancing API.
pub struct ElbClient<P = CredentialsProvider, D = RequestDispatcher>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    inner: ClientInner<P, D>,
    region: region::Region,
}

impl ElbClient {
    /// Creates a simple client backed by an implicit event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    ///
    /// See the `rusoto_core::reactor` module for more details.
    pub fn simple(region: region::Region) -> ElbClient {
        ElbClient::new(
            RequestDispatcher::default(),
            CredentialsProvider::default(),
            region,
        )
    }
}

impl<P, D> ElbClient<P, D>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
        ElbClient {
            inner: ClientInner::new(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl<P, D> Elb for ElbClient<P, D>
where
    P: ProvideAwsCredentials + 'static,
    D: DispatchSignedRequest + 'static,
{
    /// <p>Adds the specified tags to the specified load balancer. Each load balancer can have a maximum of 10 tags.</p> <p>Each tag consists of a key and an optional value. If a tag with the same key is already associated with the load balancer, <code>AddTags</code> updates its value.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/elasticloadbalancing/latest/classic/add-remove-tags.html">Tag Your Classic Load Balancer</a> in the <i>Classic Load Balancer Guide</i>.</p>
    fn add_tags(&self, input: AddTagsInput) -> RusotoFuture<AddTagsOutput, AddTagsError> {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "AddTags");
        params.put("Version", "2012-06-01");
        AddTagsInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(AddTagsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = AddTagsOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(AddTagsOutputDeserializer::deserialize(
                        "AddTagsResult",
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

    /// <p>Associates one or more security groups with your load balancer in a virtual private cloud (VPC). The specified security groups override the previously associated security groups.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/elasticloadbalancing/latest/classic/elb-security-groups.html#elb-vpc-security-groups">Security Groups for Load Balancers in a VPC</a> in the <i>Classic Load Balancer Guide</i>.</p>
    fn apply_security_groups_to_load_balancer(
        &self,
        input: ApplySecurityGroupsToLoadBalancerInput,
    ) -> RusotoFuture<ApplySecurityGroupsToLoadBalancerOutput, ApplySecurityGroupsToLoadBalancerError>
    {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ApplySecurityGroupsToLoadBalancer");
        params.put("Version", "2012-06-01");
        ApplySecurityGroupsToLoadBalancerInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ApplySecurityGroupsToLoadBalancerError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ApplySecurityGroupsToLoadBalancerOutput::default();
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
                        ApplySecurityGroupsToLoadBalancerOutputDeserializer::deserialize(
                            "ApplySecurityGroupsToLoadBalancerResult",
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

    /// <p>Adds one or more subnets to the set of configured subnets for the specified load balancer.</p> <p>The load balancer evenly distributes requests across all registered subnets. For more information, see <a href="http://docs.aws.amazon.com/elasticloadbalancing/latest/classic/elb-manage-subnets.html">Add or Remove Subnets for Your Load Balancer in a VPC</a> in the <i>Classic Load Balancer Guide</i>.</p>
    fn attach_load_balancer_to_subnets(
        &self,
        input: AttachLoadBalancerToSubnetsInput,
    ) -> RusotoFuture<AttachLoadBalancerToSubnetsOutput, AttachLoadBalancerToSubnetsError> {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "AttachLoadBalancerToSubnets");
        params.put("Version", "2012-06-01");
        AttachLoadBalancerToSubnetsInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(AttachLoadBalancerToSubnetsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = AttachLoadBalancerToSubnetsOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(AttachLoadBalancerToSubnetsOutputDeserializer::deserialize(
                        "AttachLoadBalancerToSubnetsResult",
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

    /// <p>Specifies the health check settings to use when evaluating the health state of your EC2 instances.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/elasticloadbalancing/latest/classic/elb-healthchecks.html">Configure Health Checks for Your Load Balancer</a> in the <i>Classic Load Balancer Guide</i>.</p>
    fn configure_health_check(
        &self,
        input: ConfigureHealthCheckInput,
    ) -> RusotoFuture<ConfigureHealthCheckOutput, ConfigureHealthCheckError> {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ConfigureHealthCheck");
        params.put("Version", "2012-06-01");
        ConfigureHealthCheckInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ConfigureHealthCheckError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ConfigureHealthCheckOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(ConfigureHealthCheckOutputDeserializer::deserialize(
                        "ConfigureHealthCheckResult",
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

    /// <p>Generates a stickiness policy with sticky session lifetimes that follow that of an application-generated cookie. This policy can be associated only with HTTP/HTTPS listeners.</p> <p>This policy is similar to the policy created by <a>CreateLBCookieStickinessPolicy</a>, except that the lifetime of the special Elastic Load Balancing cookie, <code>AWSELB</code>, follows the lifetime of the application-generated cookie specified in the policy configuration. The load balancer only inserts a new stickiness cookie when the application response includes a new application cookie.</p> <p>If the application cookie is explicitly removed or expires, the session stops being sticky until a new application cookie is issued.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/elasticloadbalancing/latest/classic/elb-sticky-sessions.html#enable-sticky-sessions-application">Application-Controlled Session Stickiness</a> in the <i>Classic Load Balancer Guide</i>.</p>
    fn create_app_cookie_stickiness_policy(
        &self,
        input: CreateAppCookieStickinessPolicyInput,
    ) -> RusotoFuture<CreateAppCookieStickinessPolicyOutput, CreateAppCookieStickinessPolicyError>
    {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CreateAppCookieStickinessPolicy");
        params.put("Version", "2012-06-01");
        CreateAppCookieStickinessPolicyInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateAppCookieStickinessPolicyError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = CreateAppCookieStickinessPolicyOutput::default();
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
                        CreateAppCookieStickinessPolicyOutputDeserializer::deserialize(
                            "CreateAppCookieStickinessPolicyResult",
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

    /// <p>Generates a stickiness policy with sticky session lifetimes controlled by the lifetime of the browser (user-agent) or a specified expiration period. This policy can be associated only with HTTP/HTTPS listeners.</p> <p>When a load balancer implements this policy, the load balancer uses a special cookie to track the instance for each request. When the load balancer receives a request, it first checks to see if this cookie is present in the request. If so, the load balancer sends the request to the application server specified in the cookie. If not, the load balancer sends the request to a server that is chosen based on the existing load-balancing algorithm.</p> <p>A cookie is inserted into the response for binding subsequent requests from the same user to that server. The validity of the cookie is based on the cookie expiration time, which is specified in the policy configuration.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/elasticloadbalancing/latest/classic/elb-sticky-sessions.html#enable-sticky-sessions-duration">Duration-Based Session Stickiness</a> in the <i>Classic Load Balancer Guide</i>.</p>
    fn create_lb_cookie_stickiness_policy(
        &self,
        input: CreateLBCookieStickinessPolicyInput,
    ) -> RusotoFuture<CreateLBCookieStickinessPolicyOutput, CreateLBCookieStickinessPolicyError>
    {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CreateLBCookieStickinessPolicy");
        params.put("Version", "2012-06-01");
        CreateLBCookieStickinessPolicyInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateLBCookieStickinessPolicyError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = CreateLBCookieStickinessPolicyOutput::default();
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
                        CreateLBCookieStickinessPolicyOutputDeserializer::deserialize(
                            "CreateLBCookieStickinessPolicyResult",
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

    /// <p>Creates a Classic Load Balancer.</p> <p>You can add listeners, security groups, subnets, and tags when you create your load balancer, or you can add them later using <a>CreateLoadBalancerListeners</a>, <a>ApplySecurityGroupsToLoadBalancer</a>, <a>AttachLoadBalancerToSubnets</a>, and <a>AddTags</a>.</p> <p>To describe your current load balancers, see <a>DescribeLoadBalancers</a>. When you are finished with a load balancer, you can delete it using <a>DeleteLoadBalancer</a>.</p> <p>You can create up to 20 load balancers per region per account. You can request an increase for the number of load balancers for your account. For more information, see <a href="http://docs.aws.amazon.com/elasticloadbalancing/latest/classic/elb-limits.html">Limits for Your Classic Load Balancer</a> in the <i>Classic Load Balancer Guide</i>.</p>
    fn create_load_balancer(
        &self,
        input: CreateAccessPointInput,
    ) -> RusotoFuture<CreateAccessPointOutput, CreateLoadBalancerError> {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CreateLoadBalancer");
        params.put("Version", "2012-06-01");
        CreateAccessPointInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateLoadBalancerError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = CreateAccessPointOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(CreateAccessPointOutputDeserializer::deserialize(
                        "CreateLoadBalancerResult",
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

    /// <p>Creates one or more listeners for the specified load balancer. If a listener with the specified port does not already exist, it is created; otherwise, the properties of the new listener must match the properties of the existing listener.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/elasticloadbalancing/latest/classic/elb-listener-config.html">Listeners for Your Classic Load Balancer</a> in the <i>Classic Load Balancer Guide</i>.</p>
    fn create_load_balancer_listeners(
        &self,
        input: CreateLoadBalancerListenerInput,
    ) -> RusotoFuture<CreateLoadBalancerListenerOutput, CreateLoadBalancerListenersError> {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CreateLoadBalancerListeners");
        params.put("Version", "2012-06-01");
        CreateLoadBalancerListenerInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateLoadBalancerListenersError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = CreateLoadBalancerListenerOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(CreateLoadBalancerListenerOutputDeserializer::deserialize(
                        "CreateLoadBalancerListenersResult",
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

    /// <p>Creates a policy with the specified attributes for the specified load balancer.</p> <p>Policies are settings that are saved for your load balancer and that can be applied to the listener or the application server, depending on the policy type.</p>
    fn create_load_balancer_policy(
        &self,
        input: CreateLoadBalancerPolicyInput,
    ) -> RusotoFuture<CreateLoadBalancerPolicyOutput, CreateLoadBalancerPolicyError> {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CreateLoadBalancerPolicy");
        params.put("Version", "2012-06-01");
        CreateLoadBalancerPolicyInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateLoadBalancerPolicyError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = CreateLoadBalancerPolicyOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(CreateLoadBalancerPolicyOutputDeserializer::deserialize(
                        "CreateLoadBalancerPolicyResult",
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

    /// <p>Deletes the specified load balancer.</p> <p>If you are attempting to recreate a load balancer, you must reconfigure all settings. The DNS name associated with a deleted load balancer are no longer usable. The name and associated DNS record of the deleted load balancer no longer exist and traffic sent to any of its IP addresses is no longer delivered to your instances.</p> <p>If the load balancer does not exist or has already been deleted, the call to <code>DeleteLoadBalancer</code> still succeeds.</p>
    fn delete_load_balancer(
        &self,
        input: DeleteAccessPointInput,
    ) -> RusotoFuture<DeleteAccessPointOutput, DeleteLoadBalancerError> {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteLoadBalancer");
        params.put("Version", "2012-06-01");
        DeleteAccessPointInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteLoadBalancerError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DeleteAccessPointOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DeleteAccessPointOutputDeserializer::deserialize(
                        "DeleteLoadBalancerResult",
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

    /// <p>Deletes the specified listeners from the specified load balancer.</p>
    fn delete_load_balancer_listeners(
        &self,
        input: DeleteLoadBalancerListenerInput,
    ) -> RusotoFuture<DeleteLoadBalancerListenerOutput, DeleteLoadBalancerListenersError> {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteLoadBalancerListeners");
        params.put("Version", "2012-06-01");
        DeleteLoadBalancerListenerInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteLoadBalancerListenersError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DeleteLoadBalancerListenerOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DeleteLoadBalancerListenerOutputDeserializer::deserialize(
                        "DeleteLoadBalancerListenersResult",
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

    /// <p>Deletes the specified policy from the specified load balancer. This policy must not be enabled for any listeners.</p>
    fn delete_load_balancer_policy(
        &self,
        input: DeleteLoadBalancerPolicyInput,
    ) -> RusotoFuture<DeleteLoadBalancerPolicyOutput, DeleteLoadBalancerPolicyError> {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteLoadBalancerPolicy");
        params.put("Version", "2012-06-01");
        DeleteLoadBalancerPolicyInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteLoadBalancerPolicyError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DeleteLoadBalancerPolicyOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DeleteLoadBalancerPolicyOutputDeserializer::deserialize(
                        "DeleteLoadBalancerPolicyResult",
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

    /// <p>Deregisters the specified instances from the specified load balancer. After the instance is deregistered, it no longer receives traffic from the load balancer.</p> <p>You can use <a>DescribeLoadBalancers</a> to verify that the instance is deregistered from the load balancer.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/elasticloadbalancing/latest/classic/elb-deregister-register-instances.html">Register or De-Register EC2 Instances</a> in the <i>Classic Load Balancer Guide</i>.</p>
    fn deregister_instances_from_load_balancer(
        &self,
        input: DeregisterEndPointsInput,
    ) -> RusotoFuture<DeregisterEndPointsOutput, DeregisterInstancesFromLoadBalancerError> {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeregisterInstancesFromLoadBalancer");
        params.put("Version", "2012-06-01");
        DeregisterEndPointsInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeregisterInstancesFromLoadBalancerError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DeregisterEndPointsOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DeregisterEndPointsOutputDeserializer::deserialize(
                        "DeregisterInstancesFromLoadBalancerResult",
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

    /// <p>Describes the current Elastic Load Balancing resource limits for your AWS account.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/elasticloadbalancing/latest/classic/elb-limits.html">Limits for Your Classic Load Balancer</a> in the <i>Classic Load Balancer Guide</i>.</p>
    fn describe_account_limits(
        &self,
        input: DescribeAccountLimitsInput,
    ) -> RusotoFuture<DescribeAccountLimitsOutput, DescribeAccountLimitsError> {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeAccountLimits");
        params.put("Version", "2012-06-01");
        DescribeAccountLimitsInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeAccountLimitsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DescribeAccountLimitsOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DescribeAccountLimitsOutputDeserializer::deserialize(
                        "DescribeAccountLimitsResult",
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

    /// <p>Describes the state of the specified instances with respect to the specified load balancer. If no instances are specified, the call describes the state of all instances that are currently registered with the load balancer. If instances are specified, their state is returned even if they are no longer registered with the load balancer. The state of terminated instances is not returned.</p>
    fn describe_instance_health(
        &self,
        input: DescribeEndPointStateInput,
    ) -> RusotoFuture<DescribeEndPointStateOutput, DescribeInstanceHealthError> {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeInstanceHealth");
        params.put("Version", "2012-06-01");
        DescribeEndPointStateInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeInstanceHealthError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DescribeEndPointStateOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DescribeEndPointStateOutputDeserializer::deserialize(
                        "DescribeInstanceHealthResult",
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

    /// <p>Describes the attributes for the specified load balancer.</p>
    fn describe_load_balancer_attributes(
        &self,
        input: DescribeLoadBalancerAttributesInput,
    ) -> RusotoFuture<DescribeLoadBalancerAttributesOutput, DescribeLoadBalancerAttributesError>
    {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeLoadBalancerAttributes");
        params.put("Version", "2012-06-01");
        DescribeLoadBalancerAttributesInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeLoadBalancerAttributesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DescribeLoadBalancerAttributesOutput::default();
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
                        DescribeLoadBalancerAttributesOutputDeserializer::deserialize(
                            "DescribeLoadBalancerAttributesResult",
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

    /// <p>Describes the specified policies.</p> <p>If you specify a load balancer name, the action returns the descriptions of all policies created for the load balancer. If you specify a policy name associated with your load balancer, the action returns the description of that policy. If you don't specify a load balancer name, the action returns descriptions of the specified sample policies, or descriptions of all sample policies. The names of the sample policies have the <code>ELBSample-</code> prefix.</p>
    fn describe_load_balancer_policies(
        &self,
        input: DescribeLoadBalancerPoliciesInput,
    ) -> RusotoFuture<DescribeLoadBalancerPoliciesOutput, DescribeLoadBalancerPoliciesError> {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeLoadBalancerPolicies");
        params.put("Version", "2012-06-01");
        DescribeLoadBalancerPoliciesInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeLoadBalancerPoliciesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DescribeLoadBalancerPoliciesOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DescribeLoadBalancerPoliciesOutputDeserializer::deserialize(
                        "DescribeLoadBalancerPoliciesResult",
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

    /// <p>Describes the specified load balancer policy types or all load balancer policy types.</p> <p>The description of each type indicates how it can be used. For example, some policies can be used only with layer 7 listeners, some policies can be used only with layer 4 listeners, and some policies can be used only with your EC2 instances.</p> <p>You can use <a>CreateLoadBalancerPolicy</a> to create a policy configuration for any of these policy types. Then, depending on the policy type, use either <a>SetLoadBalancerPoliciesOfListener</a> or <a>SetLoadBalancerPoliciesForBackendServer</a> to set the policy.</p>
    fn describe_load_balancer_policy_types(
        &self,
        input: DescribeLoadBalancerPolicyTypesInput,
    ) -> RusotoFuture<DescribeLoadBalancerPolicyTypesOutput, DescribeLoadBalancerPolicyTypesError>
    {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeLoadBalancerPolicyTypes");
        params.put("Version", "2012-06-01");
        DescribeLoadBalancerPolicyTypesInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeLoadBalancerPolicyTypesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DescribeLoadBalancerPolicyTypesOutput::default();
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
                        DescribeLoadBalancerPolicyTypesOutputDeserializer::deserialize(
                            "DescribeLoadBalancerPolicyTypesResult",
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

    /// <p>Describes the specified the load balancers. If no load balancers are specified, the call describes all of your load balancers.</p>
    fn describe_load_balancers(
        &self,
        input: DescribeAccessPointsInput,
    ) -> RusotoFuture<DescribeAccessPointsOutput, DescribeLoadBalancersError> {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeLoadBalancers");
        params.put("Version", "2012-06-01");
        DescribeAccessPointsInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeLoadBalancersError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DescribeAccessPointsOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DescribeAccessPointsOutputDeserializer::deserialize(
                        "DescribeLoadBalancersResult",
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

    /// <p>Describes the tags associated with the specified load balancers.</p>
    fn describe_tags(
        &self,
        input: DescribeTagsInput,
    ) -> RusotoFuture<DescribeTagsOutput, DescribeTagsError> {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeTags");
        params.put("Version", "2012-06-01");
        DescribeTagsInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeTagsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DescribeTagsOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DescribeTagsOutputDeserializer::deserialize(
                        "DescribeTagsResult",
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

    /// <p>Removes the specified subnets from the set of configured subnets for the load balancer.</p> <p>After a subnet is removed, all EC2 instances registered with the load balancer in the removed subnet go into the <code>OutOfService</code> state. Then, the load balancer balances the traffic among the remaining routable subnets.</p>
    fn detach_load_balancer_from_subnets(
        &self,
        input: DetachLoadBalancerFromSubnetsInput,
    ) -> RusotoFuture<DetachLoadBalancerFromSubnetsOutput, DetachLoadBalancerFromSubnetsError> {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DetachLoadBalancerFromSubnets");
        params.put("Version", "2012-06-01");
        DetachLoadBalancerFromSubnetsInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DetachLoadBalancerFromSubnetsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DetachLoadBalancerFromSubnetsOutput::default();
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
                        DetachLoadBalancerFromSubnetsOutputDeserializer::deserialize(
                            "DetachLoadBalancerFromSubnetsResult",
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

    /// <p>Removes the specified Availability Zones from the set of Availability Zones for the specified load balancer.</p> <p>There must be at least one Availability Zone registered with a load balancer at all times. After an Availability Zone is removed, all instances registered with the load balancer that are in the removed Availability Zone go into the <code>OutOfService</code> state. Then, the load balancer attempts to equally balance the traffic among its remaining Availability Zones.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/elasticloadbalancing/latest/classic/enable-disable-az.html">Add or Remove Availability Zones</a> in the <i>Classic Load Balancer Guide</i>.</p>
    fn disable_availability_zones_for_load_balancer(
        &self,
        input: RemoveAvailabilityZonesInput,
    ) -> RusotoFuture<RemoveAvailabilityZonesOutput, DisableAvailabilityZonesForLoadBalancerError>
    {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DisableAvailabilityZonesForLoadBalancer");
        params.put("Version", "2012-06-01");
        RemoveAvailabilityZonesInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DisableAvailabilityZonesForLoadBalancerError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = RemoveAvailabilityZonesOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(RemoveAvailabilityZonesOutputDeserializer::deserialize(
                        "DisableAvailabilityZonesForLoadBalancerResult",
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

    /// <p>Adds the specified Availability Zones to the set of Availability Zones for the specified load balancer.</p> <p>The load balancer evenly distributes requests across all its registered Availability Zones that contain instances.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/elasticloadbalancing/latest/classic/enable-disable-az.html">Add or Remove Availability Zones</a> in the <i>Classic Load Balancer Guide</i>.</p>
    fn enable_availability_zones_for_load_balancer(
        &self,
        input: AddAvailabilityZonesInput,
    ) -> RusotoFuture<AddAvailabilityZonesOutput, EnableAvailabilityZonesForLoadBalancerError> {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "EnableAvailabilityZonesForLoadBalancer");
        params.put("Version", "2012-06-01");
        AddAvailabilityZonesInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(EnableAvailabilityZonesForLoadBalancerError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = AddAvailabilityZonesOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(AddAvailabilityZonesOutputDeserializer::deserialize(
                        "EnableAvailabilityZonesForLoadBalancerResult",
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

    /// <p><p>Modifies the attributes of the specified load balancer.</p> <p>You can modify the load balancer attributes, such as <code>AccessLogs</code>, <code>ConnectionDraining</code>, and <code>CrossZoneLoadBalancing</code> by either enabling or disabling them. Or, you can modify the load balancer attribute <code>ConnectionSettings</code> by specifying an idle connection timeout value for your load balancer.</p> <p>For more information, see the following in the <i>Classic Load Balancer Guide</i>:</p> <ul> <li> <p> <a href="http://docs.aws.amazon.com/elasticloadbalancing/latest/classic/enable-disable-crosszone-lb.html">Cross-Zone Load Balancing</a> </p> </li> <li> <p> <a href="http://docs.aws.amazon.com/elasticloadbalancing/latest/classic/config-conn-drain.html">Connection Draining</a> </p> </li> <li> <p> <a href="http://docs.aws.amazon.com/elasticloadbalancing/latest/classic/access-log-collection.html">Access Logs</a> </p> </li> <li> <p> <a href="http://docs.aws.amazon.com/elasticloadbalancing/latest/classic/config-idle-timeout.html">Idle Connection Timeout</a> </p> </li> </ul></p>
    fn modify_load_balancer_attributes(
        &self,
        input: ModifyLoadBalancerAttributesInput,
    ) -> RusotoFuture<ModifyLoadBalancerAttributesOutput, ModifyLoadBalancerAttributesError> {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ModifyLoadBalancerAttributes");
        params.put("Version", "2012-06-01");
        ModifyLoadBalancerAttributesInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ModifyLoadBalancerAttributesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ModifyLoadBalancerAttributesOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(ModifyLoadBalancerAttributesOutputDeserializer::deserialize(
                        "ModifyLoadBalancerAttributesResult",
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

    /// <p>Adds the specified instances to the specified load balancer.</p> <p>The instance must be a running instance in the same network as the load balancer (EC2-Classic or the same VPC). If you have EC2-Classic instances and a load balancer in a VPC with ClassicLink enabled, you can link the EC2-Classic instances to that VPC and then register the linked EC2-Classic instances with the load balancer in the VPC.</p> <p>Note that <code>RegisterInstanceWithLoadBalancer</code> completes when the request has been registered. Instance registration takes a little time to complete. To check the state of the registered instances, use <a>DescribeLoadBalancers</a> or <a>DescribeInstanceHealth</a>.</p> <p>After the instance is registered, it starts receiving traffic and requests from the load balancer. Any instance that is not in one of the Availability Zones registered for the load balancer is moved to the <code>OutOfService</code> state. If an Availability Zone is added to the load balancer later, any instances registered with the load balancer move to the <code>InService</code> state.</p> <p>To deregister instances from a load balancer, use <a>DeregisterInstancesFromLoadBalancer</a>.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/elasticloadbalancing/latest/classic/elb-deregister-register-instances.html">Register or De-Register EC2 Instances</a> in the <i>Classic Load Balancer Guide</i>.</p>
    fn register_instances_with_load_balancer(
        &self,
        input: RegisterEndPointsInput,
    ) -> RusotoFuture<RegisterEndPointsOutput, RegisterInstancesWithLoadBalancerError> {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "RegisterInstancesWithLoadBalancer");
        params.put("Version", "2012-06-01");
        RegisterEndPointsInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(RegisterInstancesWithLoadBalancerError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = RegisterEndPointsOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(RegisterEndPointsOutputDeserializer::deserialize(
                        "RegisterInstancesWithLoadBalancerResult",
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

    /// <p>Removes one or more tags from the specified load balancer.</p>
    fn remove_tags(
        &self,
        input: RemoveTagsInput,
    ) -> RusotoFuture<RemoveTagsOutput, RemoveTagsError> {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "RemoveTags");
        params.put("Version", "2012-06-01");
        RemoveTagsInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(RemoveTagsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = RemoveTagsOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(RemoveTagsOutputDeserializer::deserialize(
                        "RemoveTagsResult",
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

    /// <p>Sets the certificate that terminates the specified listener's SSL connections. The specified certificate replaces any prior certificate that was used on the same load balancer and port.</p> <p>For more information about updating your SSL certificate, see <a href="http://docs.aws.amazon.com/elasticloadbalancing/latest/classic/elb-update-ssl-cert.html">Replace the SSL Certificate for Your Load Balancer</a> in the <i>Classic Load Balancer Guide</i>.</p>
    fn set_load_balancer_listener_ssl_certificate(
        &self,
        input: SetLoadBalancerListenerSSLCertificateInput,
    ) -> RusotoFuture<
        SetLoadBalancerListenerSSLCertificateOutput,
        SetLoadBalancerListenerSSLCertificateError,
    > {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "SetLoadBalancerListenerSSLCertificate");
        params.put("Version", "2012-06-01");
        SetLoadBalancerListenerSSLCertificateInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(SetLoadBalancerListenerSSLCertificateError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = SetLoadBalancerListenerSSLCertificateOutput::default();
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
                        SetLoadBalancerListenerSSLCertificateOutputDeserializer::deserialize(
                            "SetLoadBalancerListenerSSLCertificateResult",
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

    /// <p>Replaces the set of policies associated with the specified port on which the EC2 instance is listening with a new set of policies. At this time, only the back-end server authentication policy type can be applied to the instance ports; this policy type is composed of multiple public key policies.</p> <p>Each time you use <code>SetLoadBalancerPoliciesForBackendServer</code> to enable the policies, use the <code>PolicyNames</code> parameter to list the policies that you want to enable.</p> <p>You can use <a>DescribeLoadBalancers</a> or <a>DescribeLoadBalancerPolicies</a> to verify that the policy is associated with the EC2 instance.</p> <p>For more information about enabling back-end instance authentication, see <a href="http://docs.aws.amazon.com/elasticloadbalancing/latest/classic/elb-create-https-ssl-load-balancer.html#configure_backendauth_clt">Configure Back-end Instance Authentication</a> in the <i>Classic Load Balancer Guide</i>. For more information about Proxy Protocol, see <a href="http://docs.aws.amazon.com/elasticloadbalancing/latest/classic/enable-proxy-protocol.html">Configure Proxy Protocol Support</a> in the <i>Classic Load Balancer Guide</i>.</p>
    fn set_load_balancer_policies_for_backend_server(
        &self,
        input: SetLoadBalancerPoliciesForBackendServerInput,
    ) -> RusotoFuture<
        SetLoadBalancerPoliciesForBackendServerOutput,
        SetLoadBalancerPoliciesForBackendServerError,
    > {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "SetLoadBalancerPoliciesForBackendServer");
        params.put("Version", "2012-06-01");
        SetLoadBalancerPoliciesForBackendServerInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(SetLoadBalancerPoliciesForBackendServerError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = SetLoadBalancerPoliciesForBackendServerOutput::default();
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
                        SetLoadBalancerPoliciesForBackendServerOutputDeserializer::deserialize(
                            "SetLoadBalancerPoliciesForBackendServerResult",
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

    /// <p>Replaces the current set of policies for the specified load balancer port with the specified set of policies.</p> <p>To enable back-end server authentication, use <a>SetLoadBalancerPoliciesForBackendServer</a>.</p> <p>For more information about setting policies, see <a href="http://docs.aws.amazon.com/elasticloadbalancing/latest/classic/ssl-config-update.html">Update the SSL Negotiation Configuration</a>, <a href="http://docs.aws.amazon.com/elasticloadbalancing/latest/classic/elb-sticky-sessions.html#enable-sticky-sessions-duration">Duration-Based Session Stickiness</a>, and <a href="http://docs.aws.amazon.com/elasticloadbalancing/latest/classic/elb-sticky-sessions.html#enable-sticky-sessions-application">Application-Controlled Session Stickiness</a> in the <i>Classic Load Balancer Guide</i>.</p>
    fn set_load_balancer_policies_of_listener(
        &self,
        input: SetLoadBalancerPoliciesOfListenerInput,
    ) -> RusotoFuture<SetLoadBalancerPoliciesOfListenerOutput, SetLoadBalancerPoliciesOfListenerError>
    {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "SetLoadBalancerPoliciesOfListener");
        params.put("Version", "2012-06-01");
        SetLoadBalancerPoliciesOfListenerInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(SetLoadBalancerPoliciesOfListenerError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = SetLoadBalancerPoliciesOfListenerOutput::default();
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
                        SetLoadBalancerPoliciesOfListenerOutputDeserializer::deserialize(
                            "SetLoadBalancerPoliciesOfListenerResult",
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
}

#[cfg(test)]
mod protocol_tests {

    extern crate rusoto_mock;

    use self::rusoto_mock::*;
    use super::*;
    use rusoto_core::Region as rusoto_region;

    #[test]
    fn test_parse_error_elb_describe_load_balancers() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/error",
            "elb-describe-load-balancers.xml",
        );
        let mock = MockRequestDispatcher::with_status(400).with_body(&mock_response);
        let client = ElbClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = DescribeAccessPointsInput::default();
        let result = client.describe_load_balancers(request).sync();
        assert!(!result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_elb_describe_load_balancer_policies() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "elb-describe-load-balancer-policies.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = ElbClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = DescribeLoadBalancerPoliciesInput::default();
        let result = client.describe_load_balancer_policies(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_elb_describe_load_balancer_policy_types() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "elb-describe-load-balancer-policy-types.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = ElbClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = DescribeLoadBalancerPolicyTypesInput::default();
        let result = client.describe_load_balancer_policy_types(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_elb_describe_load_balancers() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "elb-describe-load-balancers.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = ElbClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = DescribeAccessPointsInput::default();
        let result = client.describe_load_balancers(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }
}
