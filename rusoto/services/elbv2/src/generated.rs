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
/// <p>Information about an action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Action {
    /// <p>The Amazon Resource Name (ARN) of the target group.</p>
    pub target_group_arn: String,
    /// <p>The type of action.</p>
    pub type_: String,
}

struct ActionDeserializer;
impl ActionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Action, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = Action::default();

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
                    "TargetGroupArn" => {
                        obj.target_group_arn = try!(TargetGroupArnDeserializer::deserialize(
                            "TargetGroupArn",
                            stack
                        ));
                    }
                    "Type" => {
                        obj.type_ = try!(ActionTypeEnumDeserializer::deserialize("Type", stack));
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

/// Serialize `Action` contents to a `SignedRequest`.
struct ActionSerializer;
impl ActionSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Action) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "TargetGroupArn"),
            &obj.target_group_arn.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "Type"),
            &obj.type_.replace("+", "%2B"),
        );
    }
}

struct ActionTypeEnumDeserializer;
impl ActionTypeEnumDeserializer {
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
struct ActionsDeserializer;
impl ActionsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Action>, XmlParseError> {
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
                        obj.push(try!(ActionDeserializer::deserialize("member", stack)));
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

/// Serialize `Actions` contents to a `SignedRequest`.
struct ActionsSerializer;
impl ActionsSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<Action>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            ActionSerializer::serialize(params, &key, obj);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct AddListenerCertificatesInput {
    /// <p>The certificate to add. You can specify one certificate per call.</p>
    pub certificates: Vec<Certificate>,
    /// <p>The Amazon Resource Name (ARN) of the listener.</p>
    pub listener_arn: String,
}

/// Serialize `AddListenerCertificatesInput` contents to a `SignedRequest`.
struct AddListenerCertificatesInputSerializer;
impl AddListenerCertificatesInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &AddListenerCertificatesInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        CertificateListSerializer::serialize(
            params,
            &format!("{}{}", prefix, "Certificates"),
            &obj.certificates,
        );
        params.put(
            &format!("{}{}", prefix, "ListenerArn"),
            &obj.listener_arn.replace("+", "%2B"),
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct AddListenerCertificatesOutput {
    /// <p>Information about the certificates.</p>
    pub certificates: Option<Vec<Certificate>>,
}

struct AddListenerCertificatesOutputDeserializer;
impl AddListenerCertificatesOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AddListenerCertificatesOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = AddListenerCertificatesOutput::default();

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
                    "Certificates" => {
                        obj.certificates = Some(try!(CertificateListDeserializer::deserialize(
                            "Certificates",
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
pub struct AddTagsInput {
    /// <p>The Amazon Resource Name (ARN) of the resource.</p>
    pub resource_arns: Vec<String>,
    /// <p>The tags. Each resource can have a maximum of 10 tags.</p>
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

        ResourceArnsSerializer::serialize(
            params,
            &format!("{}{}", prefix, "ResourceArns"),
            &obj.resource_arns,
        );
        TagListSerializer::serialize(params, &format!("{}{}", prefix, "Tags"), &obj.tags);
    }
}

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
struct AllocationIdDeserializer;
impl AllocationIdDeserializer {
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
/// <p>Information about an Availability Zone.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct AvailabilityZone {
    /// <p>[Network Load Balancers] The static IP address.</p>
    pub load_balancer_addresses: Option<Vec<LoadBalancerAddress>>,
    /// <p>The ID of the subnet.</p>
    pub subnet_id: Option<String>,
    /// <p>The name of the Availability Zone.</p>
    pub zone_name: Option<String>,
}

struct AvailabilityZoneDeserializer;
impl AvailabilityZoneDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AvailabilityZone, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = AvailabilityZone::default();

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
                    "LoadBalancerAddresses" => {
                        obj.load_balancer_addresses =
                            Some(try!(LoadBalancerAddressesDeserializer::deserialize(
                                "LoadBalancerAddresses",
                                stack
                            )));
                    }
                    "SubnetId" => {
                        obj.subnet_id =
                            Some(try!(SubnetIdDeserializer::deserialize("SubnetId", stack)));
                    }
                    "ZoneName" => {
                        obj.zone_name =
                            Some(try!(ZoneNameDeserializer::deserialize("ZoneName", stack)));
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
struct AvailabilityZonesDeserializer;
impl AvailabilityZonesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<AvailabilityZone>, XmlParseError> {
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
struct CanonicalHostedZoneIdDeserializer;
impl CanonicalHostedZoneIdDeserializer {
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
/// <p>Information about an SSL server certificate.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Certificate {
    /// <p>The Amazon Resource Name (ARN) of the certificate.</p>
    pub certificate_arn: Option<String>,
    /// <p>Indicates whether the certificate is the default certificate.</p>
    pub is_default: Option<bool>,
}

struct CertificateDeserializer;
impl CertificateDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Certificate, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = Certificate::default();

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
                    "CertificateArn" => {
                        obj.certificate_arn = Some(try!(CertificateArnDeserializer::deserialize(
                            "CertificateArn",
                            stack
                        )));
                    }
                    "IsDefault" => {
                        obj.is_default =
                            Some(try!(DefaultDeserializer::deserialize("IsDefault", stack)));
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

/// Serialize `Certificate` contents to a `SignedRequest`.
struct CertificateSerializer;
impl CertificateSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Certificate) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.certificate_arn {
            params.put(
                &format!("{}{}", prefix, "CertificateArn"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.is_default {
            params.put(
                &format!("{}{}", prefix, "IsDefault"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
    }
}

struct CertificateArnDeserializer;
impl CertificateArnDeserializer {
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
struct CertificateListDeserializer;
impl CertificateListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Certificate>, XmlParseError> {
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
                        obj.push(try!(CertificateDeserializer::deserialize("member", stack)));
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

/// Serialize `CertificateList` contents to a `SignedRequest`.
struct CertificateListSerializer;
impl CertificateListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<Certificate>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            CertificateSerializer::serialize(params, &key, obj);
        }
    }
}

/// <p>Information about a cipher used in a policy.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Cipher {
    /// <p>The name of the cipher.</p>
    pub name: Option<String>,
    /// <p>The priority of the cipher.</p>
    pub priority: Option<i64>,
}

struct CipherDeserializer;
impl CipherDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Cipher, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = Cipher::default();

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
                        obj.name = Some(try!(CipherNameDeserializer::deserialize("Name", stack)));
                    }
                    "Priority" => {
                        obj.priority = Some(try!(CipherPriorityDeserializer::deserialize(
                            "Priority", stack
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
struct CipherNameDeserializer;
impl CipherNameDeserializer {
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
struct CipherPriorityDeserializer;
impl CipherPriorityDeserializer {
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
struct CiphersDeserializer;
impl CiphersDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Cipher>, XmlParseError> {
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
                        obj.push(try!(CipherDeserializer::deserialize("member", stack)));
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
struct ConditionFieldNameDeserializer;
impl ConditionFieldNameDeserializer {
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
pub struct CreateListenerInput {
    /// <p>[HTTPS listeners] The SSL server certificate. You must provide exactly one certificate.</p>
    pub certificates: Option<Vec<Certificate>>,
    /// <p>The default action for the listener. For Application Load Balancers, the protocol of the specified target group must be HTTP or HTTPS. For Network Load Balancers, the protocol of the specified target group must be TCP.</p>
    pub default_actions: Vec<Action>,
    /// <p>The Amazon Resource Name (ARN) of the load balancer.</p>
    pub load_balancer_arn: String,
    /// <p>The port on which the load balancer is listening.</p>
    pub port: i64,
    /// <p>The protocol for connections from clients to the load balancer. For Application Load Balancers, the supported protocols are HTTP and HTTPS. For Network Load Balancers, the supported protocol is TCP.</p>
    pub protocol: String,
    /// <p>[HTTPS listeners] The security policy that defines which ciphers and protocols are supported. The default is the current predefined security policy.</p>
    pub ssl_policy: Option<String>,
}

/// Serialize `CreateListenerInput` contents to a `SignedRequest`.
struct CreateListenerInputSerializer;
impl CreateListenerInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CreateListenerInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.certificates {
            CertificateListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Certificates"),
                field_value,
            );
        }
        ActionsSerializer::serialize(
            params,
            &format!("{}{}", prefix, "DefaultActions"),
            &obj.default_actions,
        );
        params.put(
            &format!("{}{}", prefix, "LoadBalancerArn"),
            &obj.load_balancer_arn.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "Port"),
            &obj.port.to_string().replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "Protocol"),
            &obj.protocol.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.ssl_policy {
            params.put(
                &format!("{}{}", prefix, "SslPolicy"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateListenerOutput {
    /// <p>Information about the listener.</p>
    pub listeners: Option<Vec<Listener>>,
}

struct CreateListenerOutputDeserializer;
impl CreateListenerOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateListenerOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CreateListenerOutput::default();

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
                    "Listeners" => {
                        obj.listeners =
                            Some(try!(ListenersDeserializer::deserialize("Listeners", stack)));
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
pub struct CreateLoadBalancerInput {
    /// <p>[Application Load Balancers] The type of IP addresses used by the subnets for your load balancer. The possible values are <code>ipv4</code> (for IPv4 addresses) and <code>dualstack</code> (for IPv4 and IPv6 addresses). Internal load balancers must use <code>ipv4</code>.</p>
    pub ip_address_type: Option<String>,
    /// <p>The name of the load balancer.</p> <p>This name must be unique per region per account, can have a maximum of 32 characters, must contain only alphanumeric characters or hyphens, and must not begin or end with a hyphen.</p>
    pub name: String,
    /// <p>The nodes of an Internet-facing load balancer have public IP addresses. The DNS name of an Internet-facing load balancer is publicly resolvable to the public IP addresses of the nodes. Therefore, Internet-facing load balancers can route requests from clients over the Internet.</p> <p>The nodes of an internal load balancer have only private IP addresses. The DNS name of an internal load balancer is publicly resolvable to the private IP addresses of the nodes. Therefore, internal load balancers can only route requests from clients with access to the VPC for the load balancer.</p> <p>The default is an Internet-facing load balancer.</p>
    pub scheme: Option<String>,
    /// <p>[Application Load Balancers] The IDs of the security groups for the load balancer.</p>
    pub security_groups: Option<Vec<String>>,
    /// <p>The IDs of the public subnets. You can specify only one subnet per Availability Zone. You must specify either subnets or subnet mappings.</p> <p>[Application Load Balancers] You must specify subnets from at least two Availability Zones. You cannot specify Elastic IP addresses for your subnets.</p> <p>[Network Load Balancers] You can specify subnets from one or more Availability Zones. You can specify one Elastic IP address per subnet.</p>
    pub subnet_mappings: Option<Vec<SubnetMapping>>,
    /// <p>The IDs of the public subnets. You can specify only one subnet per Availability Zone. You must specify either subnets or subnet mappings.</p> <p>[Application Load Balancers] You must specify subnets from at least two Availability Zones.</p> <p>[Network Load Balancers] You can specify subnets from one or more Availability Zones.</p>
    pub subnets: Option<Vec<String>>,
    /// <p>One or more tags to assign to the load balancer.</p>
    pub tags: Option<Vec<Tag>>,
    /// <p>The type of load balancer. The default is <code>application</code>.</p>
    pub type_: Option<String>,
}

/// Serialize `CreateLoadBalancerInput` contents to a `SignedRequest`.
struct CreateLoadBalancerInputSerializer;
impl CreateLoadBalancerInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CreateLoadBalancerInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.ip_address_type {
            params.put(
                &format!("{}{}", prefix, "IpAddressType"),
                &field_value.replace("+", "%2B"),
            );
        }
        params.put(
            &format!("{}{}", prefix, "Name"),
            &obj.name.replace("+", "%2B"),
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
        if let Some(ref field_value) = obj.subnet_mappings {
            SubnetMappingsSerializer::serialize(
                params,
                &format!("{}{}", prefix, "SubnetMappings"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.subnets {
            SubnetsSerializer::serialize(params, &format!("{}{}", prefix, "Subnets"), field_value);
        }
        if let Some(ref field_value) = obj.tags {
            TagListSerializer::serialize(params, &format!("{}{}", prefix, "Tags"), field_value);
        }
        if let Some(ref field_value) = obj.type_ {
            params.put(
                &format!("{}{}", prefix, "Type"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateLoadBalancerOutput {
    /// <p>Information about the load balancer.</p>
    pub load_balancers: Option<Vec<LoadBalancer>>,
}

struct CreateLoadBalancerOutputDeserializer;
impl CreateLoadBalancerOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateLoadBalancerOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CreateLoadBalancerOutput::default();

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
                    "LoadBalancers" => {
                        obj.load_balancers = Some(try!(LoadBalancersDeserializer::deserialize(
                            "LoadBalancers",
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
pub struct CreateRuleInput {
    /// <p>An action. Each action has the type <code>forward</code> and specifies a target group.</p>
    pub actions: Vec<Action>,
    /// <p><p>The conditions. Each condition specifies a field name and a single value.</p> <p>If the field name is <code>host-header</code>, you can specify a single host name (for example, my.example.com). A host name is case insensitive, can be up to 128 characters in length, and can contain any of the following characters. Note that you can include up to three wildcard characters.</p> <ul> <li> <p>A-Z, a-z, 0-9</p> </li> <li> <p>- .</p> </li> <li> <p>* (matches 0 or more characters)</p> </li> <li> <p>? (matches exactly 1 character)</p> </li> </ul> <p>If the field name is <code>path-pattern</code>, you can specify a single path pattern. A path pattern is case sensitive, can be up to 128 characters in length, and can contain any of the following characters. Note that you can include up to three wildcard characters.</p> <ul> <li> <p>A-Z, a-z, 0-9</p> </li> <li> <p>_ - . $ / ~ &quot; &#39; @ : +</p> </li> <li> <p>&amp; (using &amp;amp;)</p> </li> <li> <p>* (matches 0 or more characters)</p> </li> <li> <p>? (matches exactly 1 character)</p> </li> </ul></p>
    pub conditions: Vec<RuleCondition>,
    /// <p>The Amazon Resource Name (ARN) of the listener.</p>
    pub listener_arn: String,
    /// <p>The priority for the rule. A listener can't have multiple rules with the same priority.</p>
    pub priority: i64,
}

/// Serialize `CreateRuleInput` contents to a `SignedRequest`.
struct CreateRuleInputSerializer;
impl CreateRuleInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CreateRuleInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        ActionsSerializer::serialize(params, &format!("{}{}", prefix, "Actions"), &obj.actions);
        RuleConditionListSerializer::serialize(
            params,
            &format!("{}{}", prefix, "Conditions"),
            &obj.conditions,
        );
        params.put(
            &format!("{}{}", prefix, "ListenerArn"),
            &obj.listener_arn.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "Priority"),
            &obj.priority.to_string().replace("+", "%2B"),
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateRuleOutput {
    /// <p>Information about the rule.</p>
    pub rules: Option<Vec<Rule>>,
}

struct CreateRuleOutputDeserializer;
impl CreateRuleOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateRuleOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CreateRuleOutput::default();

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
                    "Rules" => {
                        obj.rules = Some(try!(RulesDeserializer::deserialize("Rules", stack)));
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
pub struct CreateTargetGroupInput {
    /// <p>The approximate amount of time, in seconds, between health checks of an individual target. For Application Load Balancers, the range is 5 to 300 seconds. For Network Load Balancers, the supported values are 10 or 30 seconds. The default is 30 seconds.</p>
    pub health_check_interval_seconds: Option<i64>,
    /// <p>[HTTP/HTTPS health checks] The ping path that is the destination on the targets for health checks. The default is /.</p>
    pub health_check_path: Option<String>,
    /// <p>The port the load balancer uses when performing health checks on targets. The default is <code>traffic-port</code>, which is the port on which each target receives traffic from the load balancer.</p>
    pub health_check_port: Option<String>,
    /// <p>The protocol the load balancer uses when performing health checks on targets. The TCP protocol is supported only if the protocol of the target group is TCP. For Application Load Balancers, the default is HTTP. For Network Load Balancers, the default is TCP.</p>
    pub health_check_protocol: Option<String>,
    /// <p>The amount of time, in seconds, during which no response from a target means a failed health check. For Application Load Balancers, the range is 2 to 60 seconds and the default is 5 seconds. For Network Load Balancers, this is 10 seconds for TCP and HTTPS health checks and 6 seconds for HTTP health checks.</p>
    pub health_check_timeout_seconds: Option<i64>,
    /// <p>The number of consecutive health checks successes required before considering an unhealthy target healthy. For Application Load Balancers, the default is 5. For Network Load Balancers, the default is 3.</p>
    pub healthy_threshold_count: Option<i64>,
    /// <p>[HTTP/HTTPS health checks] The HTTP codes to use when checking for a successful response from a target.</p>
    pub matcher: Option<Matcher>,
    /// <p>The name of the target group.</p> <p>This name must be unique per region per account, can have a maximum of 32 characters, must contain only alphanumeric characters or hyphens, and must not begin or end with a hyphen.</p>
    pub name: String,
    /// <p>The port on which the targets receive traffic. This port is used unless you specify a port override when registering the target.</p>
    pub port: i64,
    /// <p>The protocol to use for routing traffic to the targets. For Application Load Balancers, the supported protocols are HTTP and HTTPS. For Network Load Balancers, the supported protocol is TCP.</p>
    pub protocol: String,
    /// <p>The type of target that you must specify when registering targets with this target group. The possible values are <code>instance</code> (targets are specified by instance ID) or <code>ip</code> (targets are specified by IP address). The default is <code>instance</code>. Note that you can't specify targets for a target group using both instance IDs and IP addresses.</p> <p>If the target type is <code>ip</code>, specify IP addresses from the subnets of the virtual private cloud (VPC) for the target group, the RFC 1918 range (10.0.0.0/8, 172.16.0.0/12, and 192.168.0.0/16), and the RFC 6598 range (100.64.0.0/10). You can't specify publicly routable IP addresses.</p>
    pub target_type: Option<String>,
    /// <p>The number of consecutive health check failures required before considering a target unhealthy. For Application Load Balancers, the default is 2. For Network Load Balancers, this value must be the same as the healthy threshold count.</p>
    pub unhealthy_threshold_count: Option<i64>,
    /// <p>The identifier of the virtual private cloud (VPC).</p>
    pub vpc_id: String,
}

/// Serialize `CreateTargetGroupInput` contents to a `SignedRequest`.
struct CreateTargetGroupInputSerializer;
impl CreateTargetGroupInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CreateTargetGroupInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.health_check_interval_seconds {
            params.put(
                &format!("{}{}", prefix, "HealthCheckIntervalSeconds"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.health_check_path {
            params.put(
                &format!("{}{}", prefix, "HealthCheckPath"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.health_check_port {
            params.put(
                &format!("{}{}", prefix, "HealthCheckPort"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.health_check_protocol {
            params.put(
                &format!("{}{}", prefix, "HealthCheckProtocol"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.health_check_timeout_seconds {
            params.put(
                &format!("{}{}", prefix, "HealthCheckTimeoutSeconds"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.healthy_threshold_count {
            params.put(
                &format!("{}{}", prefix, "HealthyThresholdCount"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.matcher {
            MatcherSerializer::serialize(params, &format!("{}{}", prefix, "Matcher"), field_value);
        }
        params.put(
            &format!("{}{}", prefix, "Name"),
            &obj.name.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "Port"),
            &obj.port.to_string().replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "Protocol"),
            &obj.protocol.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.target_type {
            params.put(
                &format!("{}{}", prefix, "TargetType"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.unhealthy_threshold_count {
            params.put(
                &format!("{}{}", prefix, "UnhealthyThresholdCount"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        params.put(
            &format!("{}{}", prefix, "VpcId"),
            &obj.vpc_id.replace("+", "%2B"),
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateTargetGroupOutput {
    /// <p>Information about the target group.</p>
    pub target_groups: Option<Vec<TargetGroup>>,
}

struct CreateTargetGroupOutputDeserializer;
impl CreateTargetGroupOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateTargetGroupOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CreateTargetGroupOutput::default();

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
                    "TargetGroups" => {
                        obj.target_groups = Some(try!(TargetGroupsDeserializer::deserialize(
                            "TargetGroups",
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
struct DefaultDeserializer;
impl DefaultDeserializer {
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
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteListenerInput {
    /// <p>The Amazon Resource Name (ARN) of the listener.</p>
    pub listener_arn: String,
}

/// Serialize `DeleteListenerInput` contents to a `SignedRequest`.
struct DeleteListenerInputSerializer;
impl DeleteListenerInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DeleteListenerInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "ListenerArn"),
            &obj.listener_arn.replace("+", "%2B"),
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteListenerOutput {}

struct DeleteListenerOutputDeserializer;
impl DeleteListenerOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteListenerOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let obj = DeleteListenerOutput::default();

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteLoadBalancerInput {
    /// <p>The Amazon Resource Name (ARN) of the load balancer.</p>
    pub load_balancer_arn: String,
}

/// Serialize `DeleteLoadBalancerInput` contents to a `SignedRequest`.
struct DeleteLoadBalancerInputSerializer;
impl DeleteLoadBalancerInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DeleteLoadBalancerInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "LoadBalancerArn"),
            &obj.load_balancer_arn.replace("+", "%2B"),
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteLoadBalancerOutput {}

struct DeleteLoadBalancerOutputDeserializer;
impl DeleteLoadBalancerOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteLoadBalancerOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let obj = DeleteLoadBalancerOutput::default();

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteRuleInput {
    /// <p>The Amazon Resource Name (ARN) of the rule.</p>
    pub rule_arn: String,
}

/// Serialize `DeleteRuleInput` contents to a `SignedRequest`.
struct DeleteRuleInputSerializer;
impl DeleteRuleInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DeleteRuleInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "RuleArn"),
            &obj.rule_arn.replace("+", "%2B"),
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteRuleOutput {}

struct DeleteRuleOutputDeserializer;
impl DeleteRuleOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteRuleOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let obj = DeleteRuleOutput::default();

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteTargetGroupInput {
    /// <p>The Amazon Resource Name (ARN) of the target group.</p>
    pub target_group_arn: String,
}

/// Serialize `DeleteTargetGroupInput` contents to a `SignedRequest`.
struct DeleteTargetGroupInputSerializer;
impl DeleteTargetGroupInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DeleteTargetGroupInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "TargetGroupArn"),
            &obj.target_group_arn.replace("+", "%2B"),
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteTargetGroupOutput {}

struct DeleteTargetGroupOutputDeserializer;
impl DeleteTargetGroupOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteTargetGroupOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let obj = DeleteTargetGroupOutput::default();

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeregisterTargetsInput {
    /// <p>The Amazon Resource Name (ARN) of the target group.</p>
    pub target_group_arn: String,
    /// <p>The targets. If you specified a port override when you registered a target, you must specify both the target ID and the port when you deregister it.</p>
    pub targets: Vec<TargetDescription>,
}

/// Serialize `DeregisterTargetsInput` contents to a `SignedRequest`.
struct DeregisterTargetsInputSerializer;
impl DeregisterTargetsInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DeregisterTargetsInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "TargetGroupArn"),
            &obj.target_group_arn.replace("+", "%2B"),
        );
        TargetDescriptionsSerializer::serialize(
            params,
            &format!("{}{}", prefix, "Targets"),
            &obj.targets,
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeregisterTargetsOutput {}

struct DeregisterTargetsOutputDeserializer;
impl DeregisterTargetsOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeregisterTargetsOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let obj = DeregisterTargetsOutput::default();

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
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeListenerCertificatesInput {
    /// <p>The Amazon Resource Names (ARN) of the listener.</p>
    pub listener_arn: String,
    /// <p>The marker for the next set of results. (You received this marker from a previous call.)</p>
    pub marker: Option<String>,
    /// <p>The maximum number of results to return with this call.</p>
    pub page_size: Option<i64>,
}

/// Serialize `DescribeListenerCertificatesInput` contents to a `SignedRequest`.
struct DescribeListenerCertificatesInputSerializer;
impl DescribeListenerCertificatesInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeListenerCertificatesInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "ListenerArn"),
            &obj.listener_arn.replace("+", "%2B"),
        );
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
pub struct DescribeListenerCertificatesOutput {
    /// <p>Information about the certificates.</p>
    pub certificates: Option<Vec<Certificate>>,
    /// <p>The marker to use when requesting the next set of results. If there are no additional results, the string is empty.</p>
    pub next_marker: Option<String>,
}

struct DescribeListenerCertificatesOutputDeserializer;
impl DescribeListenerCertificatesOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeListenerCertificatesOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DescribeListenerCertificatesOutput::default();

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
                    "Certificates" => {
                        obj.certificates = Some(try!(CertificateListDeserializer::deserialize(
                            "Certificates",
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
pub struct DescribeListenersInput {
    /// <p>The Amazon Resource Names (ARN) of the listeners.</p>
    pub listener_arns: Option<Vec<String>>,
    /// <p>The Amazon Resource Name (ARN) of the load balancer.</p>
    pub load_balancer_arn: Option<String>,
    /// <p>The marker for the next set of results. (You received this marker from a previous call.)</p>
    pub marker: Option<String>,
    /// <p>The maximum number of results to return with this call.</p>
    pub page_size: Option<i64>,
}

/// Serialize `DescribeListenersInput` contents to a `SignedRequest`.
struct DescribeListenersInputSerializer;
impl DescribeListenersInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeListenersInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.listener_arns {
            ListenerArnsSerializer::serialize(
                params,
                &format!("{}{}", prefix, "ListenerArns"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.load_balancer_arn {
            params.put(
                &format!("{}{}", prefix, "LoadBalancerArn"),
                &field_value.replace("+", "%2B"),
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

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeListenersOutput {
    /// <p>Information about the listeners.</p>
    pub listeners: Option<Vec<Listener>>,
    /// <p>The marker to use when requesting the next set of results. If there are no additional results, the string is empty.</p>
    pub next_marker: Option<String>,
}

struct DescribeListenersOutputDeserializer;
impl DescribeListenersOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeListenersOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DescribeListenersOutput::default();

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
                    "Listeners" => {
                        obj.listeners =
                            Some(try!(ListenersDeserializer::deserialize("Listeners", stack)));
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
pub struct DescribeLoadBalancerAttributesInput {
    /// <p>The Amazon Resource Name (ARN) of the load balancer.</p>
    pub load_balancer_arn: String,
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
            &format!("{}{}", prefix, "LoadBalancerArn"),
            &obj.load_balancer_arn.replace("+", "%2B"),
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeLoadBalancerAttributesOutput {
    /// <p>Information about the load balancer attributes.</p>
    pub attributes: Option<Vec<LoadBalancerAttribute>>,
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
                    "Attributes" => {
                        obj.attributes = Some(try!(
                            LoadBalancerAttributesDeserializer::deserialize("Attributes", stack)
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
pub struct DescribeLoadBalancersInput {
    /// <p>The Amazon Resource Names (ARN) of the load balancers. You can specify up to 20 load balancers in a single call.</p>
    pub load_balancer_arns: Option<Vec<String>>,
    /// <p>The marker for the next set of results. (You received this marker from a previous call.)</p>
    pub marker: Option<String>,
    /// <p>The names of the load balancers.</p>
    pub names: Option<Vec<String>>,
    /// <p>The maximum number of results to return with this call.</p>
    pub page_size: Option<i64>,
}

/// Serialize `DescribeLoadBalancersInput` contents to a `SignedRequest`.
struct DescribeLoadBalancersInputSerializer;
impl DescribeLoadBalancersInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeLoadBalancersInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.load_balancer_arns {
            LoadBalancerArnsSerializer::serialize(
                params,
                &format!("{}{}", prefix, "LoadBalancerArns"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.marker {
            params.put(
                &format!("{}{}", prefix, "Marker"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.names {
            LoadBalancerNamesSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Names"),
                field_value,
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
pub struct DescribeLoadBalancersOutput {
    /// <p>Information about the load balancers.</p>
    pub load_balancers: Option<Vec<LoadBalancer>>,
    /// <p>The marker to use when requesting the next set of results. If there are no additional results, the string is empty.</p>
    pub next_marker: Option<String>,
}

struct DescribeLoadBalancersOutputDeserializer;
impl DescribeLoadBalancersOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeLoadBalancersOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DescribeLoadBalancersOutput::default();

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
                    "LoadBalancers" => {
                        obj.load_balancers = Some(try!(LoadBalancersDeserializer::deserialize(
                            "LoadBalancers",
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
pub struct DescribeRulesInput {
    /// <p>The Amazon Resource Name (ARN) of the listener.</p>
    pub listener_arn: Option<String>,
    /// <p>The marker for the next set of results. (You received this marker from a previous call.)</p>
    pub marker: Option<String>,
    /// <p>The maximum number of results to return with this call.</p>
    pub page_size: Option<i64>,
    /// <p>The Amazon Resource Names (ARN) of the rules.</p>
    pub rule_arns: Option<Vec<String>>,
}

/// Serialize `DescribeRulesInput` contents to a `SignedRequest`.
struct DescribeRulesInputSerializer;
impl DescribeRulesInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeRulesInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.listener_arn {
            params.put(
                &format!("{}{}", prefix, "ListenerArn"),
                &field_value.replace("+", "%2B"),
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
        if let Some(ref field_value) = obj.rule_arns {
            RuleArnsSerializer::serialize(
                params,
                &format!("{}{}", prefix, "RuleArns"),
                field_value,
            );
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeRulesOutput {
    /// <p>The marker to use when requesting the next set of results. If there are no additional results, the string is empty.</p>
    pub next_marker: Option<String>,
    /// <p>Information about the rules.</p>
    pub rules: Option<Vec<Rule>>,
}

struct DescribeRulesOutputDeserializer;
impl DescribeRulesOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeRulesOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DescribeRulesOutput::default();

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
                    "NextMarker" => {
                        obj.next_marker =
                            Some(try!(MarkerDeserializer::deserialize("NextMarker", stack)));
                    }
                    "Rules" => {
                        obj.rules = Some(try!(RulesDeserializer::deserialize("Rules", stack)));
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
pub struct DescribeSSLPoliciesInput {
    /// <p>The marker for the next set of results. (You received this marker from a previous call.)</p>
    pub marker: Option<String>,
    /// <p>The names of the policies.</p>
    pub names: Option<Vec<String>>,
    /// <p>The maximum number of results to return with this call.</p>
    pub page_size: Option<i64>,
}

/// Serialize `DescribeSSLPoliciesInput` contents to a `SignedRequest`.
struct DescribeSSLPoliciesInputSerializer;
impl DescribeSSLPoliciesInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeSSLPoliciesInput) {
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
        if let Some(ref field_value) = obj.names {
            SslPolicyNamesSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Names"),
                field_value,
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
pub struct DescribeSSLPoliciesOutput {
    /// <p>The marker to use when requesting the next set of results. If there are no additional results, the string is empty.</p>
    pub next_marker: Option<String>,
    /// <p>Information about the policies.</p>
    pub ssl_policies: Option<Vec<SslPolicy>>,
}

struct DescribeSSLPoliciesOutputDeserializer;
impl DescribeSSLPoliciesOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeSSLPoliciesOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DescribeSSLPoliciesOutput::default();

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
                    "NextMarker" => {
                        obj.next_marker =
                            Some(try!(MarkerDeserializer::deserialize("NextMarker", stack)));
                    }
                    "SslPolicies" => {
                        obj.ssl_policies = Some(try!(SslPoliciesDeserializer::deserialize(
                            "SslPolicies",
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
pub struct DescribeTagsInput {
    /// <p>The Amazon Resource Names (ARN) of the resources.</p>
    pub resource_arns: Vec<String>,
}

/// Serialize `DescribeTagsInput` contents to a `SignedRequest`.
struct DescribeTagsInputSerializer;
impl DescribeTagsInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeTagsInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        ResourceArnsSerializer::serialize(
            params,
            &format!("{}{}", prefix, "ResourceArns"),
            &obj.resource_arns,
        );
    }
}

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
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeTargetGroupAttributesInput {
    /// <p>The Amazon Resource Name (ARN) of the target group.</p>
    pub target_group_arn: String,
}

/// Serialize `DescribeTargetGroupAttributesInput` contents to a `SignedRequest`.
struct DescribeTargetGroupAttributesInputSerializer;
impl DescribeTargetGroupAttributesInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeTargetGroupAttributesInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "TargetGroupArn"),
            &obj.target_group_arn.replace("+", "%2B"),
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeTargetGroupAttributesOutput {
    /// <p>Information about the target group attributes</p>
    pub attributes: Option<Vec<TargetGroupAttribute>>,
}

struct DescribeTargetGroupAttributesOutputDeserializer;
impl DescribeTargetGroupAttributesOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeTargetGroupAttributesOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DescribeTargetGroupAttributesOutput::default();

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
                            TargetGroupAttributesDeserializer::deserialize("Attributes", stack)
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
pub struct DescribeTargetGroupsInput {
    /// <p>The Amazon Resource Name (ARN) of the load balancer.</p>
    pub load_balancer_arn: Option<String>,
    /// <p>The marker for the next set of results. (You received this marker from a previous call.)</p>
    pub marker: Option<String>,
    /// <p>The names of the target groups.</p>
    pub names: Option<Vec<String>>,
    /// <p>The maximum number of results to return with this call.</p>
    pub page_size: Option<i64>,
    /// <p>The Amazon Resource Names (ARN) of the target groups.</p>
    pub target_group_arns: Option<Vec<String>>,
}

/// Serialize `DescribeTargetGroupsInput` contents to a `SignedRequest`.
struct DescribeTargetGroupsInputSerializer;
impl DescribeTargetGroupsInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeTargetGroupsInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.load_balancer_arn {
            params.put(
                &format!("{}{}", prefix, "LoadBalancerArn"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.marker {
            params.put(
                &format!("{}{}", prefix, "Marker"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.names {
            TargetGroupNamesSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Names"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.page_size {
            params.put(
                &format!("{}{}", prefix, "PageSize"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.target_group_arns {
            TargetGroupArnsSerializer::serialize(
                params,
                &format!("{}{}", prefix, "TargetGroupArns"),
                field_value,
            );
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeTargetGroupsOutput {
    /// <p>The marker to use when requesting the next set of results. If there are no additional results, the string is empty.</p>
    pub next_marker: Option<String>,
    /// <p>Information about the target groups.</p>
    pub target_groups: Option<Vec<TargetGroup>>,
}

struct DescribeTargetGroupsOutputDeserializer;
impl DescribeTargetGroupsOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeTargetGroupsOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DescribeTargetGroupsOutput::default();

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
                    "NextMarker" => {
                        obj.next_marker =
                            Some(try!(MarkerDeserializer::deserialize("NextMarker", stack)));
                    }
                    "TargetGroups" => {
                        obj.target_groups = Some(try!(TargetGroupsDeserializer::deserialize(
                            "TargetGroups",
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
pub struct DescribeTargetHealthInput {
    /// <p>The Amazon Resource Name (ARN) of the target group.</p>
    pub target_group_arn: String,
    /// <p>The targets.</p>
    pub targets: Option<Vec<TargetDescription>>,
}

/// Serialize `DescribeTargetHealthInput` contents to a `SignedRequest`.
struct DescribeTargetHealthInputSerializer;
impl DescribeTargetHealthInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeTargetHealthInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "TargetGroupArn"),
            &obj.target_group_arn.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.targets {
            TargetDescriptionsSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Targets"),
                field_value,
            );
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeTargetHealthOutput {
    /// <p>Information about the health of the targets.</p>
    pub target_health_descriptions: Option<Vec<TargetHealthDescription>>,
}

struct DescribeTargetHealthOutputDeserializer;
impl DescribeTargetHealthOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeTargetHealthOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DescribeTargetHealthOutput::default();

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
                    "TargetHealthDescriptions" => {
                        obj.target_health_descriptions =
                            Some(try!(TargetHealthDescriptionsDeserializer::deserialize(
                                "TargetHealthDescriptions",
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
struct HealthCheckIntervalSecondsDeserializer;
impl HealthCheckIntervalSecondsDeserializer {
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
struct HealthCheckPortDeserializer;
impl HealthCheckPortDeserializer {
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
struct HealthCheckThresholdCountDeserializer;
impl HealthCheckThresholdCountDeserializer {
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
struct HealthCheckTimeoutSecondsDeserializer;
impl HealthCheckTimeoutSecondsDeserializer {
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
struct HttpCodeDeserializer;
impl HttpCodeDeserializer {
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
struct IpAddressDeserializer;
impl IpAddressDeserializer {
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
struct IpAddressTypeDeserializer;
impl IpAddressTypeDeserializer {
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
struct IsDefaultDeserializer;
impl IsDefaultDeserializer {
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
/// <p>Information about an Elastic Load Balancing resource limit for your AWS account.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Limit {
    /// <p>The maximum value of the limit.</p>
    pub max: Option<String>,
    /// <p><p>The name of the limit. The possible values are:</p> <ul> <li> <p>application-load-balancers</p> </li> <li> <p>listeners-per-application-load-balancer</p> </li> <li> <p>listeners-per-network-load-balancer</p> </li> <li> <p>network-load-balancers</p> </li> <li> <p>rules-per-application-load-balancer</p> </li> <li> <p>target-groups</p> </li> <li> <p>targets-per-application-load-balancer</p> </li> <li> <p>targets-per-availability-zone-per-network-load-balancer</p> </li> <li> <p>targets-per-network-load-balancer</p> </li> </ul></p>
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
struct ListOfStringDeserializer;
impl ListOfStringDeserializer {
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
                        obj.push(try!(StringValueDeserializer::deserialize("member", stack)));
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

/// Serialize `ListOfString` contents to a `SignedRequest`.
struct ListOfStringSerializer;
impl ListOfStringSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

/// <p>Information about a listener.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Listener {
    /// <p>The SSL server certificate. You must provide a certificate if the protocol is HTTPS.</p>
    pub certificates: Option<Vec<Certificate>>,
    /// <p>The default actions for the listener.</p>
    pub default_actions: Option<Vec<Action>>,
    /// <p>The Amazon Resource Name (ARN) of the listener.</p>
    pub listener_arn: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the load balancer.</p>
    pub load_balancer_arn: Option<String>,
    /// <p>The port on which the load balancer is listening.</p>
    pub port: Option<i64>,
    /// <p>The protocol for connections from clients to the load balancer.</p>
    pub protocol: Option<String>,
    /// <p>The security policy that defines which ciphers and protocols are supported. The default is the current predefined security policy.</p>
    pub ssl_policy: Option<String>,
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
                    "Certificates" => {
                        obj.certificates = Some(try!(CertificateListDeserializer::deserialize(
                            "Certificates",
                            stack
                        )));
                    }
                    "DefaultActions" => {
                        obj.default_actions = Some(try!(ActionsDeserializer::deserialize(
                            "DefaultActions",
                            stack
                        )));
                    }
                    "ListenerArn" => {
                        obj.listener_arn = Some(try!(ListenerArnDeserializer::deserialize(
                            "ListenerArn",
                            stack
                        )));
                    }
                    "LoadBalancerArn" => {
                        obj.load_balancer_arn = Some(try!(
                            LoadBalancerArnDeserializer::deserialize("LoadBalancerArn", stack)
                        ));
                    }
                    "Port" => {
                        obj.port = Some(try!(PortDeserializer::deserialize("Port", stack)));
                    }
                    "Protocol" => {
                        obj.protocol = Some(try!(ProtocolEnumDeserializer::deserialize(
                            "Protocol", stack
                        )));
                    }
                    "SslPolicy" => {
                        obj.ssl_policy = Some(try!(SslPolicyNameDeserializer::deserialize(
                            "SslPolicy",
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
struct ListenerArnDeserializer;
impl ListenerArnDeserializer {
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

/// Serialize `ListenerArns` contents to a `SignedRequest`.
struct ListenerArnsSerializer;
impl ListenerArnsSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

struct ListenersDeserializer;
impl ListenersDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Listener>, XmlParseError> {
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
                        obj.push(try!(ListenerDeserializer::deserialize("member", stack)));
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
/// <p>Information about a load balancer.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct LoadBalancer {
    /// <p>The Availability Zones for the load balancer.</p>
    pub availability_zones: Option<Vec<AvailabilityZone>>,
    /// <p>The ID of the Amazon Route 53 hosted zone associated with the load balancer.</p>
    pub canonical_hosted_zone_id: Option<String>,
    /// <p>The date and time the load balancer was created.</p>
    pub created_time: Option<String>,
    /// <p>The public DNS name of the load balancer.</p>
    pub dns_name: Option<String>,
    /// <p>The type of IP addresses used by the subnets for your load balancer. The possible values are <code>ipv4</code> (for IPv4 addresses) and <code>dualstack</code> (for IPv4 and IPv6 addresses).</p>
    pub ip_address_type: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the load balancer.</p>
    pub load_balancer_arn: Option<String>,
    /// <p>The name of the load balancer.</p>
    pub load_balancer_name: Option<String>,
    /// <p>The nodes of an Internet-facing load balancer have public IP addresses. The DNS name of an Internet-facing load balancer is publicly resolvable to the public IP addresses of the nodes. Therefore, Internet-facing load balancers can route requests from clients over the Internet.</p> <p>The nodes of an internal load balancer have only private IP addresses. The DNS name of an internal load balancer is publicly resolvable to the private IP addresses of the nodes. Therefore, internal load balancers can only route requests from clients with access to the VPC for the load balancer.</p>
    pub scheme: Option<String>,
    /// <p>The IDs of the security groups for the load balancer.</p>
    pub security_groups: Option<Vec<String>>,
    /// <p>The state of the load balancer.</p>
    pub state: Option<LoadBalancerState>,
    /// <p>The type of load balancer.</p>
    pub type_: Option<String>,
    /// <p>The ID of the VPC for the load balancer.</p>
    pub vpc_id: Option<String>,
}

struct LoadBalancerDeserializer;
impl LoadBalancerDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<LoadBalancer, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = LoadBalancer::default();

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
                    "CanonicalHostedZoneId" => {
                        obj.canonical_hosted_zone_id =
                            Some(try!(CanonicalHostedZoneIdDeserializer::deserialize(
                                "CanonicalHostedZoneId",
                                stack
                            )));
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
                    "IpAddressType" => {
                        obj.ip_address_type = Some(try!(IpAddressTypeDeserializer::deserialize(
                            "IpAddressType",
                            stack
                        )));
                    }
                    "LoadBalancerArn" => {
                        obj.load_balancer_arn = Some(try!(
                            LoadBalancerArnDeserializer::deserialize("LoadBalancerArn", stack)
                        ));
                    }
                    "LoadBalancerName" => {
                        obj.load_balancer_name = Some(try!(
                            LoadBalancerNameDeserializer::deserialize("LoadBalancerName", stack)
                        ));
                    }
                    "Scheme" => {
                        obj.scheme = Some(try!(LoadBalancerSchemeEnumDeserializer::deserialize(
                            "Scheme", stack
                        )));
                    }
                    "SecurityGroups" => {
                        obj.security_groups = Some(try!(SecurityGroupsDeserializer::deserialize(
                            "SecurityGroups",
                            stack
                        )));
                    }
                    "State" => {
                        obj.state = Some(try!(LoadBalancerStateDeserializer::deserialize(
                            "State", stack
                        )));
                    }
                    "Type" => {
                        obj.type_ = Some(try!(LoadBalancerTypeEnumDeserializer::deserialize(
                            "Type", stack
                        )));
                    }
                    "VpcId" => {
                        obj.vpc_id = Some(try!(VpcIdDeserializer::deserialize("VpcId", stack)));
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
/// <p>Information about a static IP address for a load balancer.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct LoadBalancerAddress {
    /// <p>[Network Load Balancers] The allocation ID of the Elastic IP address.</p>
    pub allocation_id: Option<String>,
    /// <p>The static IP address.</p>
    pub ip_address: Option<String>,
}

struct LoadBalancerAddressDeserializer;
impl LoadBalancerAddressDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<LoadBalancerAddress, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = LoadBalancerAddress::default();

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
                    "AllocationId" => {
                        obj.allocation_id = Some(try!(AllocationIdDeserializer::deserialize(
                            "AllocationId",
                            stack
                        )));
                    }
                    "IpAddress" => {
                        obj.ip_address =
                            Some(try!(IpAddressDeserializer::deserialize("IpAddress", stack)));
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
struct LoadBalancerAddressesDeserializer;
impl LoadBalancerAddressesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<LoadBalancerAddress>, XmlParseError> {
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
                        obj.push(try!(LoadBalancerAddressDeserializer::deserialize(
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
struct LoadBalancerArnDeserializer;
impl LoadBalancerArnDeserializer {
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
struct LoadBalancerArnsDeserializer;
impl LoadBalancerArnsDeserializer {
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
                        obj.push(try!(LoadBalancerArnDeserializer::deserialize(
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

/// Serialize `LoadBalancerArns` contents to a `SignedRequest`.
struct LoadBalancerArnsSerializer;
impl LoadBalancerArnsSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

/// <p>Information about a load balancer attribute.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct LoadBalancerAttribute {
    /// <p><p>The name of the attribute.</p> <ul> <li> <p> <code>access<em>logs.s3.enabled</code> - [Application Load Balancers] Indicates whether access logs stored in Amazon S3 are enabled. The value is <code>true</code> or <code>false</code>.</p> </li> <li> <p> <code>access</em>logs.s3.bucket</code> - [Application Load Balancers] The name of the S3 bucket for the access logs. This attribute is required if access logs in Amazon S3 are enabled. The bucket must exist in the same region as the load balancer and have a bucket policy that grants Elastic Load Balancing permission to write to the bucket.</p> </li> <li> <p> <code>access<em>logs.s3.prefix</code> - [Application Load Balancers] The prefix for the location in the S3 bucket. If you don&#39;t specify a prefix, the access logs are stored in the root of the bucket.</p> </li> <li> <p> <code>deletion</em>protection.enabled</code> - Indicates whether deletion protection is enabled. The value is <code>true</code> or <code>false</code>.</p> </li> <li> <p> <code>idle<em>timeout.timeout</em>seconds</code> - [Application Load Balancers] The idle timeout value, in seconds. The valid range is 1-4000. The default is 60 seconds.</p> </li> <li> <p> <code>load<em>balancing.cross</em>zone.enabled</code> - [Network Load Balancers] Indicates whether cross-zone load balancing is enabled. The value is <code>true</code> or <code>false</code>. The default is <code>false</code>.</p> </li> <li> <p> <code>routing.http2.enabled</code> - [Application Load Balancers] Indicates whether HTTP/2 is enabled. The value is <code>true</code> or <code>false</code>. The default is <code>true</code>.</p> </li> </ul></p>
    pub key: Option<String>,
    /// <p>The value of the attribute.</p>
    pub value: Option<String>,
}

struct LoadBalancerAttributeDeserializer;
impl LoadBalancerAttributeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<LoadBalancerAttribute, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = LoadBalancerAttribute::default();

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
                        obj.key = Some(try!(LoadBalancerAttributeKeyDeserializer::deserialize(
                            "Key", stack
                        )));
                    }
                    "Value" => {
                        obj.value = Some(try!(
                            LoadBalancerAttributeValueDeserializer::deserialize("Value", stack)
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

/// Serialize `LoadBalancerAttribute` contents to a `SignedRequest`.
struct LoadBalancerAttributeSerializer;
impl LoadBalancerAttributeSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &LoadBalancerAttribute) {
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

struct LoadBalancerAttributeKeyDeserializer;
impl LoadBalancerAttributeKeyDeserializer {
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
struct LoadBalancerAttributeValueDeserializer;
impl LoadBalancerAttributeValueDeserializer {
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
struct LoadBalancerAttributesDeserializer;
impl LoadBalancerAttributesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<LoadBalancerAttribute>, XmlParseError> {
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
                        obj.push(try!(LoadBalancerAttributeDeserializer::deserialize(
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

/// Serialize `LoadBalancerAttributes` contents to a `SignedRequest`.
struct LoadBalancerAttributesSerializer;
impl LoadBalancerAttributesSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<LoadBalancerAttribute>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            LoadBalancerAttributeSerializer::serialize(params, &key, obj);
        }
    }
}

struct LoadBalancerNameDeserializer;
impl LoadBalancerNameDeserializer {
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

struct LoadBalancerSchemeEnumDeserializer;
impl LoadBalancerSchemeEnumDeserializer {
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
/// <p>Information about the state of the load balancer.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct LoadBalancerState {
    /// <p>The state code. The initial state of the load balancer is <code>provisioning</code>. After the load balancer is fully set up and ready to route traffic, its state is <code>active</code>. If the load balancer could not be set up, its state is <code>failed</code>.</p>
    pub code: Option<String>,
    /// <p>A description of the state.</p>
    pub reason: Option<String>,
}

struct LoadBalancerStateDeserializer;
impl LoadBalancerStateDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<LoadBalancerState, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = LoadBalancerState::default();

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
                    "Code" => {
                        obj.code = Some(try!(LoadBalancerStateEnumDeserializer::deserialize(
                            "Code", stack
                        )));
                    }
                    "Reason" => {
                        obj.reason =
                            Some(try!(StateReasonDeserializer::deserialize("Reason", stack)));
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
struct LoadBalancerStateEnumDeserializer;
impl LoadBalancerStateEnumDeserializer {
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
struct LoadBalancerTypeEnumDeserializer;
impl LoadBalancerTypeEnumDeserializer {
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
struct LoadBalancersDeserializer;
impl LoadBalancersDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<LoadBalancer>, XmlParseError> {
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
                        obj.push(try!(LoadBalancerDeserializer::deserialize("member", stack)));
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
/// <p>Information to use when checking for a successful response from a target.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Matcher {
    /// <p>The HTTP codes.</p> <p>For Application Load Balancers, you can specify values between 200 and 499, and the default value is 200. You can specify multiple values (for example, "200,202") or a range of values (for example, "200-299").</p> <p>For Network Load Balancers, this is 200 to 399.</p>
    pub http_code: String,
}

struct MatcherDeserializer;
impl MatcherDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Matcher, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = Matcher::default();

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
                    "HttpCode" => {
                        obj.http_code = try!(HttpCodeDeserializer::deserialize("HttpCode", stack));
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

/// Serialize `Matcher` contents to a `SignedRequest`.
struct MatcherSerializer;
impl MatcherSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Matcher) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "HttpCode"),
            &obj.http_code.replace("+", "%2B"),
        );
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
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ModifyListenerInput {
    /// <p>The default SSL server certificate.</p>
    pub certificates: Option<Vec<Certificate>>,
    /// <p>The default action. For Application Load Balancers, the protocol of the specified target group must be HTTP or HTTPS. For Network Load Balancers, the protocol of the specified target group must be TCP.</p>
    pub default_actions: Option<Vec<Action>>,
    /// <p>The Amazon Resource Name (ARN) of the listener.</p>
    pub listener_arn: String,
    /// <p>The port for connections from clients to the load balancer.</p>
    pub port: Option<i64>,
    /// <p>The protocol for connections from clients to the load balancer. Application Load Balancers support HTTP and HTTPS and Network Load Balancers support TCP.</p>
    pub protocol: Option<String>,
    /// <p>The security policy that defines which protocols and ciphers are supported. For more information, see <a href="http://docs.aws.amazon.com/elasticloadbalancing/latest/application/create-https-listener.html#describe-ssl-policies">Security Policies</a> in the <i>Application Load Balancers Guide</i>.</p>
    pub ssl_policy: Option<String>,
}

/// Serialize `ModifyListenerInput` contents to a `SignedRequest`.
struct ModifyListenerInputSerializer;
impl ModifyListenerInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ModifyListenerInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.certificates {
            CertificateListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Certificates"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.default_actions {
            ActionsSerializer::serialize(
                params,
                &format!("{}{}", prefix, "DefaultActions"),
                field_value,
            );
        }
        params.put(
            &format!("{}{}", prefix, "ListenerArn"),
            &obj.listener_arn.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.port {
            params.put(
                &format!("{}{}", prefix, "Port"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.protocol {
            params.put(
                &format!("{}{}", prefix, "Protocol"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.ssl_policy {
            params.put(
                &format!("{}{}", prefix, "SslPolicy"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct ModifyListenerOutput {
    /// <p>Information about the modified listeners.</p>
    pub listeners: Option<Vec<Listener>>,
}

struct ModifyListenerOutputDeserializer;
impl ModifyListenerOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ModifyListenerOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ModifyListenerOutput::default();

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
                    "Listeners" => {
                        obj.listeners =
                            Some(try!(ListenersDeserializer::deserialize("Listeners", stack)));
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
pub struct ModifyLoadBalancerAttributesInput {
    /// <p>The load balancer attributes.</p>
    pub attributes: Vec<LoadBalancerAttribute>,
    /// <p>The Amazon Resource Name (ARN) of the load balancer.</p>
    pub load_balancer_arn: String,
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
            &format!("{}{}", prefix, "Attributes"),
            &obj.attributes,
        );
        params.put(
            &format!("{}{}", prefix, "LoadBalancerArn"),
            &obj.load_balancer_arn.replace("+", "%2B"),
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct ModifyLoadBalancerAttributesOutput {
    /// <p>Information about the load balancer attributes.</p>
    pub attributes: Option<Vec<LoadBalancerAttribute>>,
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
                    "Attributes" => {
                        obj.attributes = Some(try!(
                            LoadBalancerAttributesDeserializer::deserialize("Attributes", stack)
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
pub struct ModifyRuleInput {
    /// <p>The actions. The target group must use the HTTP or HTTPS protocol.</p>
    pub actions: Option<Vec<Action>>,
    /// <p>The conditions.</p>
    pub conditions: Option<Vec<RuleCondition>>,
    /// <p>The Amazon Resource Name (ARN) of the rule.</p>
    pub rule_arn: String,
}

/// Serialize `ModifyRuleInput` contents to a `SignedRequest`.
struct ModifyRuleInputSerializer;
impl ModifyRuleInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ModifyRuleInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.actions {
            ActionsSerializer::serialize(params, &format!("{}{}", prefix, "Actions"), field_value);
        }
        if let Some(ref field_value) = obj.conditions {
            RuleConditionListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Conditions"),
                field_value,
            );
        }
        params.put(
            &format!("{}{}", prefix, "RuleArn"),
            &obj.rule_arn.replace("+", "%2B"),
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct ModifyRuleOutput {
    /// <p>Information about the rule.</p>
    pub rules: Option<Vec<Rule>>,
}

struct ModifyRuleOutputDeserializer;
impl ModifyRuleOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ModifyRuleOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ModifyRuleOutput::default();

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
                    "Rules" => {
                        obj.rules = Some(try!(RulesDeserializer::deserialize("Rules", stack)));
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
pub struct ModifyTargetGroupAttributesInput {
    /// <p>The attributes.</p>
    pub attributes: Vec<TargetGroupAttribute>,
    /// <p>The Amazon Resource Name (ARN) of the target group.</p>
    pub target_group_arn: String,
}

/// Serialize `ModifyTargetGroupAttributesInput` contents to a `SignedRequest`.
struct ModifyTargetGroupAttributesInputSerializer;
impl ModifyTargetGroupAttributesInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ModifyTargetGroupAttributesInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        TargetGroupAttributesSerializer::serialize(
            params,
            &format!("{}{}", prefix, "Attributes"),
            &obj.attributes,
        );
        params.put(
            &format!("{}{}", prefix, "TargetGroupArn"),
            &obj.target_group_arn.replace("+", "%2B"),
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct ModifyTargetGroupAttributesOutput {
    /// <p>Information about the attributes.</p>
    pub attributes: Option<Vec<TargetGroupAttribute>>,
}

struct ModifyTargetGroupAttributesOutputDeserializer;
impl ModifyTargetGroupAttributesOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ModifyTargetGroupAttributesOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ModifyTargetGroupAttributesOutput::default();

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
                            TargetGroupAttributesDeserializer::deserialize("Attributes", stack)
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
pub struct ModifyTargetGroupInput {
    /// <p>The approximate amount of time, in seconds, between health checks of an individual target. For Application Load Balancers, the range is 5 to 300 seconds. For Network Load Balancers, the supported values are 10 or 30 seconds.</p>
    pub health_check_interval_seconds: Option<i64>,
    /// <p>[HTTP/HTTPS health checks] The ping path that is the destination for the health check request.</p>
    pub health_check_path: Option<String>,
    /// <p>The port the load balancer uses when performing health checks on targets.</p>
    pub health_check_port: Option<String>,
    /// <p>The protocol the load balancer uses when performing health checks on targets. The TCP protocol is supported only if the protocol of the target group is TCP.</p>
    pub health_check_protocol: Option<String>,
    /// <p>[HTTP/HTTPS health checks] The amount of time, in seconds, during which no response means a failed health check.</p>
    pub health_check_timeout_seconds: Option<i64>,
    /// <p>The number of consecutive health checks successes required before considering an unhealthy target healthy.</p>
    pub healthy_threshold_count: Option<i64>,
    /// <p>[HTTP/HTTPS health checks] The HTTP codes to use when checking for a successful response from a target.</p>
    pub matcher: Option<Matcher>,
    /// <p>The Amazon Resource Name (ARN) of the target group.</p>
    pub target_group_arn: String,
    /// <p>The number of consecutive health check failures required before considering the target unhealthy. For Network Load Balancers, this value must be the same as the healthy threshold count.</p>
    pub unhealthy_threshold_count: Option<i64>,
}

/// Serialize `ModifyTargetGroupInput` contents to a `SignedRequest`.
struct ModifyTargetGroupInputSerializer;
impl ModifyTargetGroupInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ModifyTargetGroupInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.health_check_interval_seconds {
            params.put(
                &format!("{}{}", prefix, "HealthCheckIntervalSeconds"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.health_check_path {
            params.put(
                &format!("{}{}", prefix, "HealthCheckPath"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.health_check_port {
            params.put(
                &format!("{}{}", prefix, "HealthCheckPort"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.health_check_protocol {
            params.put(
                &format!("{}{}", prefix, "HealthCheckProtocol"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.health_check_timeout_seconds {
            params.put(
                &format!("{}{}", prefix, "HealthCheckTimeoutSeconds"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.healthy_threshold_count {
            params.put(
                &format!("{}{}", prefix, "HealthyThresholdCount"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.matcher {
            MatcherSerializer::serialize(params, &format!("{}{}", prefix, "Matcher"), field_value);
        }
        params.put(
            &format!("{}{}", prefix, "TargetGroupArn"),
            &obj.target_group_arn.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.unhealthy_threshold_count {
            params.put(
                &format!("{}{}", prefix, "UnhealthyThresholdCount"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct ModifyTargetGroupOutput {
    /// <p>Information about the target group.</p>
    pub target_groups: Option<Vec<TargetGroup>>,
}

struct ModifyTargetGroupOutputDeserializer;
impl ModifyTargetGroupOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ModifyTargetGroupOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ModifyTargetGroupOutput::default();

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
                    "TargetGroups" => {
                        obj.target_groups = Some(try!(TargetGroupsDeserializer::deserialize(
                            "TargetGroups",
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
struct PathDeserializer;
impl PathDeserializer {
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
struct PortDeserializer;
impl PortDeserializer {
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
struct ProtocolEnumDeserializer;
impl ProtocolEnumDeserializer {
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
pub struct RegisterTargetsInput {
    /// <p>The Amazon Resource Name (ARN) of the target group.</p>
    pub target_group_arn: String,
    /// <p>The targets.</p>
    pub targets: Vec<TargetDescription>,
}

/// Serialize `RegisterTargetsInput` contents to a `SignedRequest`.
struct RegisterTargetsInputSerializer;
impl RegisterTargetsInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &RegisterTargetsInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "TargetGroupArn"),
            &obj.target_group_arn.replace("+", "%2B"),
        );
        TargetDescriptionsSerializer::serialize(
            params,
            &format!("{}{}", prefix, "Targets"),
            &obj.targets,
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct RegisterTargetsOutput {}

struct RegisterTargetsOutputDeserializer;
impl RegisterTargetsOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<RegisterTargetsOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let obj = RegisterTargetsOutput::default();

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct RemoveListenerCertificatesInput {
    /// <p>The certificate to remove. You can specify one certificate per call.</p>
    pub certificates: Vec<Certificate>,
    /// <p>The Amazon Resource Name (ARN) of the listener.</p>
    pub listener_arn: String,
}

/// Serialize `RemoveListenerCertificatesInput` contents to a `SignedRequest`.
struct RemoveListenerCertificatesInputSerializer;
impl RemoveListenerCertificatesInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &RemoveListenerCertificatesInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        CertificateListSerializer::serialize(
            params,
            &format!("{}{}", prefix, "Certificates"),
            &obj.certificates,
        );
        params.put(
            &format!("{}{}", prefix, "ListenerArn"),
            &obj.listener_arn.replace("+", "%2B"),
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct RemoveListenerCertificatesOutput {}

struct RemoveListenerCertificatesOutputDeserializer;
impl RemoveListenerCertificatesOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<RemoveListenerCertificatesOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let obj = RemoveListenerCertificatesOutput::default();

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct RemoveTagsInput {
    /// <p>The Amazon Resource Name (ARN) of the resource.</p>
    pub resource_arns: Vec<String>,
    /// <p>The tag keys for the tags to remove.</p>
    pub tag_keys: Vec<String>,
}

/// Serialize `RemoveTagsInput` contents to a `SignedRequest`.
struct RemoveTagsInputSerializer;
impl RemoveTagsInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &RemoveTagsInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        ResourceArnsSerializer::serialize(
            params,
            &format!("{}{}", prefix, "ResourceArns"),
            &obj.resource_arns,
        );
        TagKeysSerializer::serialize(params, &format!("{}{}", prefix, "TagKeys"), &obj.tag_keys);
    }
}

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
struct ResourceArnDeserializer;
impl ResourceArnDeserializer {
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

/// Serialize `ResourceArns` contents to a `SignedRequest`.
struct ResourceArnsSerializer;
impl ResourceArnsSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

/// <p>Information about a rule.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Rule {
    /// <p>The actions.</p>
    pub actions: Option<Vec<Action>>,
    /// <p>The conditions.</p>
    pub conditions: Option<Vec<RuleCondition>>,
    /// <p>Indicates whether this is the default rule.</p>
    pub is_default: Option<bool>,
    /// <p>The priority.</p>
    pub priority: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the rule.</p>
    pub rule_arn: Option<String>,
}

struct RuleDeserializer;
impl RuleDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Rule, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = Rule::default();

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
                        obj.actions =
                            Some(try!(ActionsDeserializer::deserialize("Actions", stack)));
                    }
                    "Conditions" => {
                        obj.conditions = Some(try!(RuleConditionListDeserializer::deserialize(
                            "Conditions",
                            stack
                        )));
                    }
                    "IsDefault" => {
                        obj.is_default =
                            Some(try!(IsDefaultDeserializer::deserialize("IsDefault", stack)));
                    }
                    "Priority" => {
                        obj.priority =
                            Some(try!(StringDeserializer::deserialize("Priority", stack)));
                    }
                    "RuleArn" => {
                        obj.rule_arn =
                            Some(try!(RuleArnDeserializer::deserialize("RuleArn", stack)));
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
struct RuleArnDeserializer;
impl RuleArnDeserializer {
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

/// Serialize `RuleArns` contents to a `SignedRequest`.
struct RuleArnsSerializer;
impl RuleArnsSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

/// <p>Information about a condition for a rule.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct RuleCondition {
    /// <p>The name of the field. The possible values are <code>host-header</code> and <code>path-pattern</code>.</p>
    pub field: Option<String>,
    /// <p><p>The condition value.</p> <p>If the field name is <code>host-header</code>, you can specify a single host name (for example, my.example.com). A host name is case insensitive, can be up to 128 characters in length, and can contain any of the following characters. Note that you can include up to three wildcard characters.</p> <ul> <li> <p>A-Z, a-z, 0-9</p> </li> <li> <p>- .</p> </li> <li> <p>* (matches 0 or more characters)</p> </li> <li> <p>? (matches exactly 1 character)</p> </li> </ul> <p>If the field name is <code>path-pattern</code>, you can specify a single path pattern (for example, /img/<em>). A path pattern is case sensitive, can be up to 128 characters in length, and can contain any of the following characters. Note that you can include up to three wildcard characters.</p> <ul> <li> <p>A-Z, a-z, 0-9</p> </li> <li> <p>_ - . $ / ~ &quot; &#39; @ : +</p> </li> <li> <p>&amp; (using &amp;amp;)</p> </li> <li> <p></em> (matches 0 or more characters)</p> </li> <li> <p>? (matches exactly 1 character)</p> </li> </ul></p>
    pub values: Option<Vec<String>>,
}

struct RuleConditionDeserializer;
impl RuleConditionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<RuleCondition, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = RuleCondition::default();

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
                    "Field" => {
                        obj.field = Some(try!(ConditionFieldNameDeserializer::deserialize(
                            "Field", stack
                        )));
                    }
                    "Values" => {
                        obj.values =
                            Some(try!(ListOfStringDeserializer::deserialize("Values", stack)));
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

/// Serialize `RuleCondition` contents to a `SignedRequest`.
struct RuleConditionSerializer;
impl RuleConditionSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &RuleCondition) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.field {
            params.put(
                &format!("{}{}", prefix, "Field"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.values {
            ListOfStringSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Values"),
                field_value,
            );
        }
    }
}

struct RuleConditionListDeserializer;
impl RuleConditionListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<RuleCondition>, XmlParseError> {
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
                        obj.push(try!(RuleConditionDeserializer::deserialize(
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

/// Serialize `RuleConditionList` contents to a `SignedRequest`.
struct RuleConditionListSerializer;
impl RuleConditionListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<RuleCondition>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            RuleConditionSerializer::serialize(params, &key, obj);
        }
    }
}

/// Serialize `RulePriorityList` contents to a `SignedRequest`.
struct RulePriorityListSerializer;
impl RulePriorityListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<RulePriorityPair>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            RulePriorityPairSerializer::serialize(params, &key, obj);
        }
    }
}

/// <p>Information about the priorities for the rules for a listener.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct RulePriorityPair {
    /// <p>The rule priority.</p>
    pub priority: Option<i64>,
    /// <p>The Amazon Resource Name (ARN) of the rule.</p>
    pub rule_arn: Option<String>,
}

/// Serialize `RulePriorityPair` contents to a `SignedRequest`.
struct RulePriorityPairSerializer;
impl RulePriorityPairSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &RulePriorityPair) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.priority {
            params.put(
                &format!("{}{}", prefix, "Priority"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.rule_arn {
            params.put(
                &format!("{}{}", prefix, "RuleArn"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

struct RulesDeserializer;
impl RulesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Rule>, XmlParseError> {
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
                        obj.push(try!(RuleDeserializer::deserialize("member", stack)));
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

#[derive(Default, Debug, Clone, PartialEq)]
pub struct SetIpAddressTypeInput {
    /// <p>The IP address type. The possible values are <code>ipv4</code> (for IPv4 addresses) and <code>dualstack</code> (for IPv4 and IPv6 addresses). Internal load balancers must use <code>ipv4</code>.</p>
    pub ip_address_type: String,
    /// <p>The Amazon Resource Name (ARN) of the load balancer.</p>
    pub load_balancer_arn: String,
}

/// Serialize `SetIpAddressTypeInput` contents to a `SignedRequest`.
struct SetIpAddressTypeInputSerializer;
impl SetIpAddressTypeInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &SetIpAddressTypeInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "IpAddressType"),
            &obj.ip_address_type.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "LoadBalancerArn"),
            &obj.load_balancer_arn.replace("+", "%2B"),
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct SetIpAddressTypeOutput {
    /// <p>The IP address type.</p>
    pub ip_address_type: Option<String>,
}

struct SetIpAddressTypeOutputDeserializer;
impl SetIpAddressTypeOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<SetIpAddressTypeOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = SetIpAddressTypeOutput::default();

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
                    "IpAddressType" => {
                        obj.ip_address_type = Some(try!(IpAddressTypeDeserializer::deserialize(
                            "IpAddressType",
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
pub struct SetRulePrioritiesInput {
    /// <p>The rule priorities.</p>
    pub rule_priorities: Vec<RulePriorityPair>,
}

/// Serialize `SetRulePrioritiesInput` contents to a `SignedRequest`.
struct SetRulePrioritiesInputSerializer;
impl SetRulePrioritiesInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &SetRulePrioritiesInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        RulePriorityListSerializer::serialize(
            params,
            &format!("{}{}", prefix, "RulePriorities"),
            &obj.rule_priorities,
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct SetRulePrioritiesOutput {
    /// <p>Information about the rules.</p>
    pub rules: Option<Vec<Rule>>,
}

struct SetRulePrioritiesOutputDeserializer;
impl SetRulePrioritiesOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<SetRulePrioritiesOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = SetRulePrioritiesOutput::default();

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
                    "Rules" => {
                        obj.rules = Some(try!(RulesDeserializer::deserialize("Rules", stack)));
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
pub struct SetSecurityGroupsInput {
    /// <p>The Amazon Resource Name (ARN) of the load balancer.</p>
    pub load_balancer_arn: String,
    /// <p>The IDs of the security groups.</p>
    pub security_groups: Vec<String>,
}

/// Serialize `SetSecurityGroupsInput` contents to a `SignedRequest`.
struct SetSecurityGroupsInputSerializer;
impl SetSecurityGroupsInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &SetSecurityGroupsInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "LoadBalancerArn"),
            &obj.load_balancer_arn.replace("+", "%2B"),
        );
        SecurityGroupsSerializer::serialize(
            params,
            &format!("{}{}", prefix, "SecurityGroups"),
            &obj.security_groups,
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct SetSecurityGroupsOutput {
    /// <p>The IDs of the security groups associated with the load balancer.</p>
    pub security_group_ids: Option<Vec<String>>,
}

struct SetSecurityGroupsOutputDeserializer;
impl SetSecurityGroupsOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<SetSecurityGroupsOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = SetSecurityGroupsOutput::default();

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
                    "SecurityGroupIds" => {
                        obj.security_group_ids = Some(try!(
                            SecurityGroupsDeserializer::deserialize("SecurityGroupIds", stack)
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
pub struct SetSubnetsInput {
    /// <p>The Amazon Resource Name (ARN) of the load balancer.</p>
    pub load_balancer_arn: String,
    /// <p>The IDs of the public subnets. You must specify subnets from at least two Availability Zones. You can specify only one subnet per Availability Zone. You must specify either subnets or subnet mappings.</p> <p>You cannot specify Elastic IP addresses for your subnets.</p>
    pub subnet_mappings: Option<Vec<SubnetMapping>>,
    /// <p>The IDs of the public subnets. You must specify subnets from at least two Availability Zones. You can specify only one subnet per Availability Zone. You must specify either subnets or subnet mappings.</p>
    pub subnets: Vec<String>,
}

/// Serialize `SetSubnetsInput` contents to a `SignedRequest`.
struct SetSubnetsInputSerializer;
impl SetSubnetsInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &SetSubnetsInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "LoadBalancerArn"),
            &obj.load_balancer_arn.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.subnet_mappings {
            SubnetMappingsSerializer::serialize(
                params,
                &format!("{}{}", prefix, "SubnetMappings"),
                field_value,
            );
        }
        SubnetsSerializer::serialize(params, &format!("{}{}", prefix, "Subnets"), &obj.subnets);
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct SetSubnetsOutput {
    /// <p>Information about the subnet and Availability Zone.</p>
    pub availability_zones: Option<Vec<AvailabilityZone>>,
}

struct SetSubnetsOutputDeserializer;
impl SetSubnetsOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<SetSubnetsOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = SetSubnetsOutput::default();

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
struct SslPoliciesDeserializer;
impl SslPoliciesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<SslPolicy>, XmlParseError> {
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
                        obj.push(try!(SslPolicyDeserializer::deserialize("member", stack)));
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
/// <p>Information about a policy used for SSL negotiation.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SslPolicy {
    /// <p>The ciphers.</p>
    pub ciphers: Option<Vec<Cipher>>,
    /// <p>The name of the policy.</p>
    pub name: Option<String>,
    /// <p>The protocols.</p>
    pub ssl_protocols: Option<Vec<String>>,
}

struct SslPolicyDeserializer;
impl SslPolicyDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<SslPolicy, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = SslPolicy::default();

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
                    "Ciphers" => {
                        obj.ciphers =
                            Some(try!(CiphersDeserializer::deserialize("Ciphers", stack)));
                    }
                    "Name" => {
                        obj.name =
                            Some(try!(SslPolicyNameDeserializer::deserialize("Name", stack)));
                    }
                    "SslProtocols" => {
                        obj.ssl_protocols = Some(try!(SslProtocolsDeserializer::deserialize(
                            "SslProtocols",
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
struct SslPolicyNameDeserializer;
impl SslPolicyNameDeserializer {
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

/// Serialize `SslPolicyNames` contents to a `SignedRequest`.
struct SslPolicyNamesSerializer;
impl SslPolicyNamesSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

struct SslProtocolDeserializer;
impl SslProtocolDeserializer {
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
struct SslProtocolsDeserializer;
impl SslProtocolsDeserializer {
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
                        obj.push(try!(SslProtocolDeserializer::deserialize("member", stack)));
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
struct StateReasonDeserializer;
impl StateReasonDeserializer {
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
struct StringValueDeserializer;
impl StringValueDeserializer {
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
/// <p>Information about a subnet mapping.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SubnetMapping {
    /// <p>[Network Load Balancers] The allocation ID of the Elastic IP address.</p>
    pub allocation_id: Option<String>,
    /// <p>The ID of the subnet.</p>
    pub subnet_id: Option<String>,
}

/// Serialize `SubnetMapping` contents to a `SignedRequest`.
struct SubnetMappingSerializer;
impl SubnetMappingSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &SubnetMapping) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.allocation_id {
            params.put(
                &format!("{}{}", prefix, "AllocationId"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.subnet_id {
            params.put(
                &format!("{}{}", prefix, "SubnetId"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

/// Serialize `SubnetMappings` contents to a `SignedRequest`.
struct SubnetMappingsSerializer;
impl SubnetMappingsSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<SubnetMapping>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            SubnetMappingSerializer::serialize(params, &key, obj);
        }
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

/// <p>The tags associated with a resource.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct TagDescription {
    /// <p>The Amazon Resource Name (ARN) of the resource.</p>
    pub resource_arn: Option<String>,
    /// <p>Information about the tags.</p>
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
                    "ResourceArn" => {
                        obj.resource_arn = Some(try!(ResourceArnDeserializer::deserialize(
                            "ResourceArn",
                            stack
                        )));
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

/// Serialize `TagKeys` contents to a `SignedRequest`.
struct TagKeysSerializer;
impl TagKeysSerializer {
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
/// <p>Information about a target.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct TargetDescription {
    /// <p>An Availability Zone or <code>all</code>. This determines whether the target receives traffic from the load balancer nodes in the specified Availability Zone or from all enabled Availability Zones for the load balancer.</p> <p>This parameter is not supported if the target type of the target group is <code>instance</code>. If the IP address is in a subnet of the VPC for the target group, the Availability Zone is automatically detected and this parameter is optional. If the IP address is outside the VPC, this parameter is required.</p> <p>With an Application Load Balancer, if the IP address is outside the VPC for the target group, the only supported value is <code>all</code>.</p>
    pub availability_zone: Option<String>,
    /// <p>The ID of the target. If the target type of the target group is <code>instance</code>, specify an instance ID. If the target type is <code>ip</code>, specify an IP address.</p>
    pub id: String,
    /// <p>The port on which the target is listening.</p>
    pub port: Option<i64>,
}

struct TargetDescriptionDeserializer;
impl TargetDescriptionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<TargetDescription, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = TargetDescription::default();

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
                    "AvailabilityZone" => {
                        obj.availability_zone = Some(try!(ZoneNameDeserializer::deserialize(
                            "AvailabilityZone",
                            stack
                        )));
                    }
                    "Id" => {
                        obj.id = try!(TargetIdDeserializer::deserialize("Id", stack));
                    }
                    "Port" => {
                        obj.port = Some(try!(PortDeserializer::deserialize("Port", stack)));
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

/// Serialize `TargetDescription` contents to a `SignedRequest`.
struct TargetDescriptionSerializer;
impl TargetDescriptionSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &TargetDescription) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.availability_zone {
            params.put(
                &format!("{}{}", prefix, "AvailabilityZone"),
                &field_value.replace("+", "%2B"),
            );
        }
        params.put(&format!("{}{}", prefix, "Id"), &obj.id.replace("+", "%2B"));
        if let Some(ref field_value) = obj.port {
            params.put(
                &format!("{}{}", prefix, "Port"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
    }
}

/// Serialize `TargetDescriptions` contents to a `SignedRequest`.
struct TargetDescriptionsSerializer;
impl TargetDescriptionsSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<TargetDescription>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            TargetDescriptionSerializer::serialize(params, &key, obj);
        }
    }
}

/// <p>Information about a target group.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct TargetGroup {
    /// <p>The approximate amount of time, in seconds, between health checks of an individual target.</p>
    pub health_check_interval_seconds: Option<i64>,
    /// <p>The destination for the health check request.</p>
    pub health_check_path: Option<String>,
    /// <p>The port to use to connect with the target.</p>
    pub health_check_port: Option<String>,
    /// <p>The protocol to use to connect with the target.</p>
    pub health_check_protocol: Option<String>,
    /// <p>The amount of time, in seconds, during which no response means a failed health check.</p>
    pub health_check_timeout_seconds: Option<i64>,
    /// <p>The number of consecutive health checks successes required before considering an unhealthy target healthy.</p>
    pub healthy_threshold_count: Option<i64>,
    /// <p>The Amazon Resource Names (ARN) of the load balancers that route traffic to this target group.</p>
    pub load_balancer_arns: Option<Vec<String>>,
    /// <p>The HTTP codes to use when checking for a successful response from a target.</p>
    pub matcher: Option<Matcher>,
    /// <p>The port on which the targets are listening.</p>
    pub port: Option<i64>,
    /// <p>The protocol to use for routing traffic to the targets.</p>
    pub protocol: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the target group.</p>
    pub target_group_arn: Option<String>,
    /// <p>The name of the target group.</p>
    pub target_group_name: Option<String>,
    /// <p>The type of target that you must specify when registering targets with this target group. The possible values are <code>instance</code> (targets are specified by instance ID) or <code>ip</code> (targets are specified by IP address).</p>
    pub target_type: Option<String>,
    /// <p>The number of consecutive health check failures required before considering the target unhealthy.</p>
    pub unhealthy_threshold_count: Option<i64>,
    /// <p>The ID of the VPC for the targets.</p>
    pub vpc_id: Option<String>,
}

struct TargetGroupDeserializer;
impl TargetGroupDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<TargetGroup, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = TargetGroup::default();

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
                    "HealthCheckIntervalSeconds" => {
                        obj.health_check_interval_seconds =
                            Some(try!(HealthCheckIntervalSecondsDeserializer::deserialize(
                                "HealthCheckIntervalSeconds",
                                stack
                            )));
                    }
                    "HealthCheckPath" => {
                        obj.health_check_path = Some(try!(PathDeserializer::deserialize(
                            "HealthCheckPath",
                            stack
                        )));
                    }
                    "HealthCheckPort" => {
                        obj.health_check_port = Some(try!(
                            HealthCheckPortDeserializer::deserialize("HealthCheckPort", stack)
                        ));
                    }
                    "HealthCheckProtocol" => {
                        obj.health_check_protocol = Some(try!(
                            ProtocolEnumDeserializer::deserialize("HealthCheckProtocol", stack)
                        ));
                    }
                    "HealthCheckTimeoutSeconds" => {
                        obj.health_check_timeout_seconds =
                            Some(try!(HealthCheckTimeoutSecondsDeserializer::deserialize(
                                "HealthCheckTimeoutSeconds",
                                stack
                            )));
                    }
                    "HealthyThresholdCount" => {
                        obj.healthy_threshold_count =
                            Some(try!(HealthCheckThresholdCountDeserializer::deserialize(
                                "HealthyThresholdCount",
                                stack
                            )));
                    }
                    "LoadBalancerArns" => {
                        obj.load_balancer_arns = Some(try!(
                            LoadBalancerArnsDeserializer::deserialize("LoadBalancerArns", stack)
                        ));
                    }
                    "Matcher" => {
                        obj.matcher =
                            Some(try!(MatcherDeserializer::deserialize("Matcher", stack)));
                    }
                    "Port" => {
                        obj.port = Some(try!(PortDeserializer::deserialize("Port", stack)));
                    }
                    "Protocol" => {
                        obj.protocol = Some(try!(ProtocolEnumDeserializer::deserialize(
                            "Protocol", stack
                        )));
                    }
                    "TargetGroupArn" => {
                        obj.target_group_arn = Some(try!(TargetGroupArnDeserializer::deserialize(
                            "TargetGroupArn",
                            stack
                        )));
                    }
                    "TargetGroupName" => {
                        obj.target_group_name = Some(try!(
                            TargetGroupNameDeserializer::deserialize("TargetGroupName", stack)
                        ));
                    }
                    "TargetType" => {
                        obj.target_type = Some(try!(TargetTypeEnumDeserializer::deserialize(
                            "TargetType",
                            stack
                        )));
                    }
                    "UnhealthyThresholdCount" => {
                        obj.unhealthy_threshold_count =
                            Some(try!(HealthCheckThresholdCountDeserializer::deserialize(
                                "UnhealthyThresholdCount",
                                stack
                            )));
                    }
                    "VpcId" => {
                        obj.vpc_id = Some(try!(VpcIdDeserializer::deserialize("VpcId", stack)));
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
struct TargetGroupArnDeserializer;
impl TargetGroupArnDeserializer {
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

/// Serialize `TargetGroupArns` contents to a `SignedRequest`.
struct TargetGroupArnsSerializer;
impl TargetGroupArnsSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

/// <p>Information about a target group attribute.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct TargetGroupAttribute {
    /// <p><p>The name of the attribute.</p> <ul> <li> <p> <code>deregistration<em>delay.timeout</em>seconds</code> - The amount time for Elastic Load Balancing to wait before changing the state of a deregistering target from <code>draining</code> to <code>unused</code>. The range is 0-3600 seconds. The default value is 300 seconds.</p> </li> <li> <p> <code>proxy<em>protocol</em>v2.enabled</code> - [Network Load Balancers] Indicates whether Proxy Protocol version 2 is enabled.</p> </li> <li> <p> <code>stickiness.enabled</code> - [Application Load Balancers] Indicates whether sticky sessions are enabled. The value is <code>true</code> or <code>false</code>.</p> </li> <li> <p> <code>stickiness.type</code> - [Application Load Balancers] The type of sticky sessions. The possible value is <code>lb<em>cookie</code>.</p> </li> <li> <p> <code>stickiness.lb</em>cookie.duration_seconds</code> - [Application Load Balancers] The time period, in seconds, during which requests from a client should be routed to the same target. After this time period expires, the load balancer-generated cookie is considered stale. The range is 1 second to 1 week (604800 seconds). The default value is 1 day (86400 seconds).</p> </li> </ul></p>
    pub key: Option<String>,
    /// <p>The value of the attribute.</p>
    pub value: Option<String>,
}

struct TargetGroupAttributeDeserializer;
impl TargetGroupAttributeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<TargetGroupAttribute, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = TargetGroupAttribute::default();

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
                        obj.key = Some(try!(TargetGroupAttributeKeyDeserializer::deserialize(
                            "Key", stack
                        )));
                    }
                    "Value" => {
                        obj.value = Some(try!(TargetGroupAttributeValueDeserializer::deserialize(
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

/// Serialize `TargetGroupAttribute` contents to a `SignedRequest`.
struct TargetGroupAttributeSerializer;
impl TargetGroupAttributeSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &TargetGroupAttribute) {
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

struct TargetGroupAttributeKeyDeserializer;
impl TargetGroupAttributeKeyDeserializer {
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
struct TargetGroupAttributeValueDeserializer;
impl TargetGroupAttributeValueDeserializer {
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
struct TargetGroupAttributesDeserializer;
impl TargetGroupAttributesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<TargetGroupAttribute>, XmlParseError> {
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
                        obj.push(try!(TargetGroupAttributeDeserializer::deserialize(
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

/// Serialize `TargetGroupAttributes` contents to a `SignedRequest`.
struct TargetGroupAttributesSerializer;
impl TargetGroupAttributesSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<TargetGroupAttribute>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            TargetGroupAttributeSerializer::serialize(params, &key, obj);
        }
    }
}

struct TargetGroupNameDeserializer;
impl TargetGroupNameDeserializer {
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

/// Serialize `TargetGroupNames` contents to a `SignedRequest`.
struct TargetGroupNamesSerializer;
impl TargetGroupNamesSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

struct TargetGroupsDeserializer;
impl TargetGroupsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<TargetGroup>, XmlParseError> {
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
                        obj.push(try!(TargetGroupDeserializer::deserialize("member", stack)));
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
/// <p>Information about the current health of a target.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct TargetHealth {
    /// <p>A description of the target health that provides additional details. If the state is <code>healthy</code>, a description is not provided.</p>
    pub description: Option<String>,
    /// <p><p>The reason code. If the target state is <code>healthy</code>, a reason code is not provided.</p> <p>If the target state is <code>initial</code>, the reason code can be one of the following values:</p> <ul> <li> <p> <code>Elb.RegistrationInProgress</code> - The target is in the process of being registered with the load balancer.</p> </li> <li> <p> <code>Elb.InitialHealthChecking</code> - The load balancer is still sending the target the minimum number of health checks required to determine its health status.</p> </li> </ul> <p>If the target state is <code>unhealthy</code>, the reason code can be one of the following values:</p> <ul> <li> <p> <code>Target.ResponseCodeMismatch</code> - The health checks did not return an expected HTTP code.</p> </li> <li> <p> <code>Target.Timeout</code> - The health check requests timed out.</p> </li> <li> <p> <code>Target.FailedHealthChecks</code> - The health checks failed because the connection to the target timed out, the target response was malformed, or the target failed the health check for an unknown reason.</p> </li> <li> <p> <code>Elb.InternalError</code> - The health checks failed due to an internal error.</p> </li> </ul> <p>If the target state is <code>unused</code>, the reason code can be one of the following values:</p> <ul> <li> <p> <code>Target.NotRegistered</code> - The target is not registered with the target group.</p> </li> <li> <p> <code>Target.NotInUse</code> - The target group is not used by any load balancer or the target is in an Availability Zone that is not enabled for its load balancer.</p> </li> <li> <p> <code>Target.IpUnusable</code> - The target IP address is reserved for use by a load balancer.</p> </li> <li> <p> <code>Target.InvalidState</code> - The target is in the stopped or terminated state.</p> </li> </ul> <p>If the target state is <code>draining</code>, the reason code can be the following value:</p> <ul> <li> <p> <code>Target.DeregistrationInProgress</code> - The target is in the process of being deregistered and the deregistration delay period has not expired.</p> </li> </ul></p>
    pub reason: Option<String>,
    /// <p>The state of the target.</p>
    pub state: Option<String>,
}

struct TargetHealthDeserializer;
impl TargetHealthDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<TargetHealth, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = TargetHealth::default();

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
                    "Reason" => {
                        obj.reason = Some(try!(TargetHealthReasonEnumDeserializer::deserialize(
                            "Reason", stack
                        )));
                    }
                    "State" => {
                        obj.state = Some(try!(TargetHealthStateEnumDeserializer::deserialize(
                            "State", stack
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
/// <p>Information about the health of a target.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct TargetHealthDescription {
    /// <p>The port to use to connect with the target.</p>
    pub health_check_port: Option<String>,
    /// <p>The description of the target.</p>
    pub target: Option<TargetDescription>,
    /// <p>The health information for the target.</p>
    pub target_health: Option<TargetHealth>,
}

struct TargetHealthDescriptionDeserializer;
impl TargetHealthDescriptionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<TargetHealthDescription, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = TargetHealthDescription::default();

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
                    "HealthCheckPort" => {
                        obj.health_check_port = Some(try!(
                            HealthCheckPortDeserializer::deserialize("HealthCheckPort", stack)
                        ));
                    }
                    "Target" => {
                        obj.target = Some(try!(TargetDescriptionDeserializer::deserialize(
                            "Target", stack
                        )));
                    }
                    "TargetHealth" => {
                        obj.target_health = Some(try!(TargetHealthDeserializer::deserialize(
                            "TargetHealth",
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
struct TargetHealthDescriptionsDeserializer;
impl TargetHealthDescriptionsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<TargetHealthDescription>, XmlParseError> {
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
                        obj.push(try!(TargetHealthDescriptionDeserializer::deserialize(
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
struct TargetHealthReasonEnumDeserializer;
impl TargetHealthReasonEnumDeserializer {
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
struct TargetHealthStateEnumDeserializer;
impl TargetHealthStateEnumDeserializer {
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
struct TargetIdDeserializer;
impl TargetIdDeserializer {
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
struct TargetTypeEnumDeserializer;
impl TargetTypeEnumDeserializer {
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
struct VpcIdDeserializer;
impl VpcIdDeserializer {
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
struct ZoneNameDeserializer;
impl ZoneNameDeserializer {
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
/// Errors returned by AddListenerCertificates
#[derive(Debug, PartialEq)]
pub enum AddListenerCertificatesError {
    /// <p>The specified certificate does not exist.</p>
    CertificateNotFound(String),
    /// <p>The specified listener does not exist.</p>
    ListenerNotFound(String),
    /// <p>You've reached the limit on the number of certificates per load balancer.</p>
    TooManyCertificates(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl AddListenerCertificatesError {
    pub fn from_body(body: &str) -> AddListenerCertificatesError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "CertificateNotFound" => AddListenerCertificatesError::CertificateNotFound(
                    String::from(parsed_error.message),
                ),
                "ListenerNotFound" => AddListenerCertificatesError::ListenerNotFound(String::from(
                    parsed_error.message,
                )),
                "TooManyCertificates" => AddListenerCertificatesError::TooManyCertificates(
                    String::from(parsed_error.message),
                ),
                _ => AddListenerCertificatesError::Unknown(String::from(body)),
            },
            Err(_) => AddListenerCertificatesError::Unknown(body.to_string()),
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

impl From<XmlParseError> for AddListenerCertificatesError {
    fn from(err: XmlParseError) -> AddListenerCertificatesError {
        let XmlParseError(message) = err;
        AddListenerCertificatesError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for AddListenerCertificatesError {
    fn from(err: CredentialsError) -> AddListenerCertificatesError {
        AddListenerCertificatesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AddListenerCertificatesError {
    fn from(err: HttpDispatchError) -> AddListenerCertificatesError {
        AddListenerCertificatesError::HttpDispatch(err)
    }
}
impl From<io::Error> for AddListenerCertificatesError {
    fn from(err: io::Error) -> AddListenerCertificatesError {
        AddListenerCertificatesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AddListenerCertificatesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AddListenerCertificatesError {
    fn description(&self) -> &str {
        match *self {
            AddListenerCertificatesError::CertificateNotFound(ref cause) => cause,
            AddListenerCertificatesError::ListenerNotFound(ref cause) => cause,
            AddListenerCertificatesError::TooManyCertificates(ref cause) => cause,
            AddListenerCertificatesError::Validation(ref cause) => cause,
            AddListenerCertificatesError::Credentials(ref err) => err.description(),
            AddListenerCertificatesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AddListenerCertificatesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by AddTags
#[derive(Debug, PartialEq)]
pub enum AddTagsError {
    /// <p>A tag key was specified more than once.</p>
    DuplicateTagKeys(String),
    /// <p>The specified load balancer does not exist.</p>
    LoadBalancerNotFound(String),
    /// <p>The specified target group does not exist.</p>
    TargetGroupNotFound(String),
    /// <p>You've reached the limit on the number of tags per load balancer.</p>
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
                "DuplicateTagKeys" => {
                    AddTagsError::DuplicateTagKeys(String::from(parsed_error.message))
                }
                "LoadBalancerNotFound" => {
                    AddTagsError::LoadBalancerNotFound(String::from(parsed_error.message))
                }
                "TargetGroupNotFound" => {
                    AddTagsError::TargetGroupNotFound(String::from(parsed_error.message))
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
            AddTagsError::DuplicateTagKeys(ref cause) => cause,
            AddTagsError::LoadBalancerNotFound(ref cause) => cause,
            AddTagsError::TargetGroupNotFound(ref cause) => cause,
            AddTagsError::TooManyTags(ref cause) => cause,
            AddTagsError::Validation(ref cause) => cause,
            AddTagsError::Credentials(ref err) => err.description(),
            AddTagsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            AddTagsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateListener
#[derive(Debug, PartialEq)]
pub enum CreateListenerError {
    /// <p>The specified certificate does not exist.</p>
    CertificateNotFound(String),
    /// <p>A listener with the specified port already exists.</p>
    DuplicateListener(String),
    /// <p>The specified configuration is not valid with this protocol.</p>
    IncompatibleProtocols(String),
    /// <p>The requested configuration is not valid.</p>
    InvalidConfigurationRequest(String),
    /// <p>The specified load balancer does not exist.</p>
    LoadBalancerNotFound(String),
    /// <p>The specified SSL policy does not exist.</p>
    SSLPolicyNotFound(String),
    /// <p>You've reached the limit on the number of load balancers per target group.</p>
    TargetGroupAssociationLimit(String),
    /// <p>The specified target group does not exist.</p>
    TargetGroupNotFound(String),
    /// <p>You've reached the limit on the number of certificates per load balancer.</p>
    TooManyCertificates(String),
    /// <p>You've reached the limit on the number of listeners per load balancer.</p>
    TooManyListeners(String),
    /// <p>You've reached the limit on the number of times a target can be registered with a load balancer.</p>
    TooManyRegistrationsForTargetId(String),
    /// <p>You've reached the limit on the number of targets.</p>
    TooManyTargets(String),
    /// <p>The specified protocol is not supported.</p>
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

impl CreateListenerError {
    pub fn from_body(body: &str) -> CreateListenerError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "CertificateNotFound" => {
                    CreateListenerError::CertificateNotFound(String::from(parsed_error.message))
                }
                "DuplicateListener" => {
                    CreateListenerError::DuplicateListener(String::from(parsed_error.message))
                }
                "IncompatibleProtocols" => {
                    CreateListenerError::IncompatibleProtocols(String::from(parsed_error.message))
                }
                "InvalidConfigurationRequest" => CreateListenerError::InvalidConfigurationRequest(
                    String::from(parsed_error.message),
                ),
                "LoadBalancerNotFound" => {
                    CreateListenerError::LoadBalancerNotFound(String::from(parsed_error.message))
                }
                "SSLPolicyNotFound" => {
                    CreateListenerError::SSLPolicyNotFound(String::from(parsed_error.message))
                }
                "TargetGroupAssociationLimit" => CreateListenerError::TargetGroupAssociationLimit(
                    String::from(parsed_error.message),
                ),
                "TargetGroupNotFound" => {
                    CreateListenerError::TargetGroupNotFound(String::from(parsed_error.message))
                }
                "TooManyCertificates" => {
                    CreateListenerError::TooManyCertificates(String::from(parsed_error.message))
                }
                "TooManyListeners" => {
                    CreateListenerError::TooManyListeners(String::from(parsed_error.message))
                }
                "TooManyRegistrationsForTargetId" => {
                    CreateListenerError::TooManyRegistrationsForTargetId(String::from(
                        parsed_error.message,
                    ))
                }
                "TooManyTargets" => {
                    CreateListenerError::TooManyTargets(String::from(parsed_error.message))
                }
                "UnsupportedProtocol" => {
                    CreateListenerError::UnsupportedProtocol(String::from(parsed_error.message))
                }
                _ => CreateListenerError::Unknown(String::from(body)),
            },
            Err(_) => CreateListenerError::Unknown(body.to_string()),
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

impl From<XmlParseError> for CreateListenerError {
    fn from(err: XmlParseError) -> CreateListenerError {
        let XmlParseError(message) = err;
        CreateListenerError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for CreateListenerError {
    fn from(err: CredentialsError) -> CreateListenerError {
        CreateListenerError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateListenerError {
    fn from(err: HttpDispatchError) -> CreateListenerError {
        CreateListenerError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateListenerError {
    fn from(err: io::Error) -> CreateListenerError {
        CreateListenerError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateListenerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateListenerError {
    fn description(&self) -> &str {
        match *self {
            CreateListenerError::CertificateNotFound(ref cause) => cause,
            CreateListenerError::DuplicateListener(ref cause) => cause,
            CreateListenerError::IncompatibleProtocols(ref cause) => cause,
            CreateListenerError::InvalidConfigurationRequest(ref cause) => cause,
            CreateListenerError::LoadBalancerNotFound(ref cause) => cause,
            CreateListenerError::SSLPolicyNotFound(ref cause) => cause,
            CreateListenerError::TargetGroupAssociationLimit(ref cause) => cause,
            CreateListenerError::TargetGroupNotFound(ref cause) => cause,
            CreateListenerError::TooManyCertificates(ref cause) => cause,
            CreateListenerError::TooManyListeners(ref cause) => cause,
            CreateListenerError::TooManyRegistrationsForTargetId(ref cause) => cause,
            CreateListenerError::TooManyTargets(ref cause) => cause,
            CreateListenerError::UnsupportedProtocol(ref cause) => cause,
            CreateListenerError::Validation(ref cause) => cause,
            CreateListenerError::Credentials(ref err) => err.description(),
            CreateListenerError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateListenerError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateLoadBalancer
#[derive(Debug, PartialEq)]
pub enum CreateLoadBalancerError {
    /// <p>The specified allocation ID does not exist.</p>
    AllocationIdNotFound(String),
    /// <p>The specified Availability Zone is not supported.</p>
    AvailabilityZoneNotSupported(String),
    /// <p>A load balancer with the specified name already exists.</p>
    DuplicateLoadBalancerName(String),
    /// <p>A tag key was specified more than once.</p>
    DuplicateTagKeys(String),
    /// <p>The requested configuration is not valid.</p>
    InvalidConfigurationRequest(String),
    /// <p>The requested scheme is not valid.</p>
    InvalidScheme(String),
    /// <p>The specified security group does not exist.</p>
    InvalidSecurityGroup(String),
    /// <p>The specified subnet is out of available addresses.</p>
    InvalidSubnet(String),
    /// <p>This operation is not allowed.</p>
    OperationNotPermitted(String),
    /// <p>A specified resource is in use.</p>
    ResourceInUse(String),
    /// <p>The specified subnet does not exist.</p>
    SubnetNotFound(String),
    /// <p>You've reached the limit on the number of load balancers for your AWS account.</p>
    TooManyLoadBalancers(String),
    /// <p>You've reached the limit on the number of tags per load balancer.</p>
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

impl CreateLoadBalancerError {
    pub fn from_body(body: &str) -> CreateLoadBalancerError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "AllocationIdNotFound" => CreateLoadBalancerError::AllocationIdNotFound(
                    String::from(parsed_error.message),
                ),
                "AvailabilityZoneNotSupported" => {
                    CreateLoadBalancerError::AvailabilityZoneNotSupported(String::from(
                        parsed_error.message,
                    ))
                }
                "DuplicateLoadBalancerName" => CreateLoadBalancerError::DuplicateLoadBalancerName(
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
                "ResourceInUse" => {
                    CreateLoadBalancerError::ResourceInUse(String::from(parsed_error.message))
                }
                "SubnetNotFound" => {
                    CreateLoadBalancerError::SubnetNotFound(String::from(parsed_error.message))
                }
                "TooManyLoadBalancers" => CreateLoadBalancerError::TooManyLoadBalancers(
                    String::from(parsed_error.message),
                ),
                "TooManyTags" => {
                    CreateLoadBalancerError::TooManyTags(String::from(parsed_error.message))
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
            CreateLoadBalancerError::AllocationIdNotFound(ref cause) => cause,
            CreateLoadBalancerError::AvailabilityZoneNotSupported(ref cause) => cause,
            CreateLoadBalancerError::DuplicateLoadBalancerName(ref cause) => cause,
            CreateLoadBalancerError::DuplicateTagKeys(ref cause) => cause,
            CreateLoadBalancerError::InvalidConfigurationRequest(ref cause) => cause,
            CreateLoadBalancerError::InvalidScheme(ref cause) => cause,
            CreateLoadBalancerError::InvalidSecurityGroup(ref cause) => cause,
            CreateLoadBalancerError::InvalidSubnet(ref cause) => cause,
            CreateLoadBalancerError::OperationNotPermitted(ref cause) => cause,
            CreateLoadBalancerError::ResourceInUse(ref cause) => cause,
            CreateLoadBalancerError::SubnetNotFound(ref cause) => cause,
            CreateLoadBalancerError::TooManyLoadBalancers(ref cause) => cause,
            CreateLoadBalancerError::TooManyTags(ref cause) => cause,
            CreateLoadBalancerError::Validation(ref cause) => cause,
            CreateLoadBalancerError::Credentials(ref err) => err.description(),
            CreateLoadBalancerError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateLoadBalancerError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateRule
#[derive(Debug, PartialEq)]
pub enum CreateRuleError {
    /// <p>The specified configuration is not valid with this protocol.</p>
    IncompatibleProtocols(String),
    /// <p>The requested configuration is not valid.</p>
    InvalidConfigurationRequest(String),
    /// <p>The specified listener does not exist.</p>
    ListenerNotFound(String),
    /// <p>The specified priority is in use.</p>
    PriorityInUse(String),
    /// <p>You've reached the limit on the number of load balancers per target group.</p>
    TargetGroupAssociationLimit(String),
    /// <p>The specified target group does not exist.</p>
    TargetGroupNotFound(String),
    /// <p>You've reached the limit on the number of times a target can be registered with a load balancer.</p>
    TooManyRegistrationsForTargetId(String),
    /// <p>You've reached the limit on the number of rules per load balancer.</p>
    TooManyRules(String),
    /// <p>You've reached the limit on the number of target groups for your AWS account.</p>
    TooManyTargetGroups(String),
    /// <p>You've reached the limit on the number of targets.</p>
    TooManyTargets(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateRuleError {
    pub fn from_body(body: &str) -> CreateRuleError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "IncompatibleProtocols" => {
                    CreateRuleError::IncompatibleProtocols(String::from(parsed_error.message))
                }
                "InvalidConfigurationRequest" => {
                    CreateRuleError::InvalidConfigurationRequest(String::from(parsed_error.message))
                }
                "ListenerNotFound" => {
                    CreateRuleError::ListenerNotFound(String::from(parsed_error.message))
                }
                "PriorityInUse" => {
                    CreateRuleError::PriorityInUse(String::from(parsed_error.message))
                }
                "TargetGroupAssociationLimit" => {
                    CreateRuleError::TargetGroupAssociationLimit(String::from(parsed_error.message))
                }
                "TargetGroupNotFound" => {
                    CreateRuleError::TargetGroupNotFound(String::from(parsed_error.message))
                }
                "TooManyRegistrationsForTargetId" => {
                    CreateRuleError::TooManyRegistrationsForTargetId(String::from(
                        parsed_error.message,
                    ))
                }
                "TooManyRules" => CreateRuleError::TooManyRules(String::from(parsed_error.message)),
                "TooManyTargetGroups" => {
                    CreateRuleError::TooManyTargetGroups(String::from(parsed_error.message))
                }
                "TooManyTargets" => {
                    CreateRuleError::TooManyTargets(String::from(parsed_error.message))
                }
                _ => CreateRuleError::Unknown(String::from(body)),
            },
            Err(_) => CreateRuleError::Unknown(body.to_string()),
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

impl From<XmlParseError> for CreateRuleError {
    fn from(err: XmlParseError) -> CreateRuleError {
        let XmlParseError(message) = err;
        CreateRuleError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for CreateRuleError {
    fn from(err: CredentialsError) -> CreateRuleError {
        CreateRuleError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateRuleError {
    fn from(err: HttpDispatchError) -> CreateRuleError {
        CreateRuleError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateRuleError {
    fn from(err: io::Error) -> CreateRuleError {
        CreateRuleError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateRuleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateRuleError {
    fn description(&self) -> &str {
        match *self {
            CreateRuleError::IncompatibleProtocols(ref cause) => cause,
            CreateRuleError::InvalidConfigurationRequest(ref cause) => cause,
            CreateRuleError::ListenerNotFound(ref cause) => cause,
            CreateRuleError::PriorityInUse(ref cause) => cause,
            CreateRuleError::TargetGroupAssociationLimit(ref cause) => cause,
            CreateRuleError::TargetGroupNotFound(ref cause) => cause,
            CreateRuleError::TooManyRegistrationsForTargetId(ref cause) => cause,
            CreateRuleError::TooManyRules(ref cause) => cause,
            CreateRuleError::TooManyTargetGroups(ref cause) => cause,
            CreateRuleError::TooManyTargets(ref cause) => cause,
            CreateRuleError::Validation(ref cause) => cause,
            CreateRuleError::Credentials(ref err) => err.description(),
            CreateRuleError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateRuleError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateTargetGroup
#[derive(Debug, PartialEq)]
pub enum CreateTargetGroupError {
    /// <p>A target group with the specified name already exists.</p>
    DuplicateTargetGroupName(String),
    /// <p>The requested configuration is not valid.</p>
    InvalidConfigurationRequest(String),
    /// <p>You've reached the limit on the number of target groups for your AWS account.</p>
    TooManyTargetGroups(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateTargetGroupError {
    pub fn from_body(body: &str) -> CreateTargetGroupError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "DuplicateTargetGroupName" => CreateTargetGroupError::DuplicateTargetGroupName(
                    String::from(parsed_error.message),
                ),
                "InvalidConfigurationRequest" => {
                    CreateTargetGroupError::InvalidConfigurationRequest(String::from(
                        parsed_error.message,
                    ))
                }
                "TooManyTargetGroups" => {
                    CreateTargetGroupError::TooManyTargetGroups(String::from(parsed_error.message))
                }
                _ => CreateTargetGroupError::Unknown(String::from(body)),
            },
            Err(_) => CreateTargetGroupError::Unknown(body.to_string()),
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

impl From<XmlParseError> for CreateTargetGroupError {
    fn from(err: XmlParseError) -> CreateTargetGroupError {
        let XmlParseError(message) = err;
        CreateTargetGroupError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for CreateTargetGroupError {
    fn from(err: CredentialsError) -> CreateTargetGroupError {
        CreateTargetGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateTargetGroupError {
    fn from(err: HttpDispatchError) -> CreateTargetGroupError {
        CreateTargetGroupError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateTargetGroupError {
    fn from(err: io::Error) -> CreateTargetGroupError {
        CreateTargetGroupError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateTargetGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateTargetGroupError {
    fn description(&self) -> &str {
        match *self {
            CreateTargetGroupError::DuplicateTargetGroupName(ref cause) => cause,
            CreateTargetGroupError::InvalidConfigurationRequest(ref cause) => cause,
            CreateTargetGroupError::TooManyTargetGroups(ref cause) => cause,
            CreateTargetGroupError::Validation(ref cause) => cause,
            CreateTargetGroupError::Credentials(ref err) => err.description(),
            CreateTargetGroupError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateTargetGroupError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteListener
#[derive(Debug, PartialEq)]
pub enum DeleteListenerError {
    /// <p>The specified listener does not exist.</p>
    ListenerNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteListenerError {
    pub fn from_body(body: &str) -> DeleteListenerError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ListenerNotFound" => {
                    DeleteListenerError::ListenerNotFound(String::from(parsed_error.message))
                }
                _ => DeleteListenerError::Unknown(String::from(body)),
            },
            Err(_) => DeleteListenerError::Unknown(body.to_string()),
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

impl From<XmlParseError> for DeleteListenerError {
    fn from(err: XmlParseError) -> DeleteListenerError {
        let XmlParseError(message) = err;
        DeleteListenerError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeleteListenerError {
    fn from(err: CredentialsError) -> DeleteListenerError {
        DeleteListenerError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteListenerError {
    fn from(err: HttpDispatchError) -> DeleteListenerError {
        DeleteListenerError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteListenerError {
    fn from(err: io::Error) -> DeleteListenerError {
        DeleteListenerError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteListenerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteListenerError {
    fn description(&self) -> &str {
        match *self {
            DeleteListenerError::ListenerNotFound(ref cause) => cause,
            DeleteListenerError::Validation(ref cause) => cause,
            DeleteListenerError::Credentials(ref err) => err.description(),
            DeleteListenerError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteListenerError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteLoadBalancer
#[derive(Debug, PartialEq)]
pub enum DeleteLoadBalancerError {
    /// <p>The specified load balancer does not exist.</p>
    LoadBalancerNotFound(String),
    /// <p>This operation is not allowed.</p>
    OperationNotPermitted(String),
    /// <p>A specified resource is in use.</p>
    ResourceInUse(String),
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
                "LoadBalancerNotFound" => DeleteLoadBalancerError::LoadBalancerNotFound(
                    String::from(parsed_error.message),
                ),
                "OperationNotPermitted" => DeleteLoadBalancerError::OperationNotPermitted(
                    String::from(parsed_error.message),
                ),
                "ResourceInUse" => {
                    DeleteLoadBalancerError::ResourceInUse(String::from(parsed_error.message))
                }
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
            DeleteLoadBalancerError::LoadBalancerNotFound(ref cause) => cause,
            DeleteLoadBalancerError::OperationNotPermitted(ref cause) => cause,
            DeleteLoadBalancerError::ResourceInUse(ref cause) => cause,
            DeleteLoadBalancerError::Validation(ref cause) => cause,
            DeleteLoadBalancerError::Credentials(ref err) => err.description(),
            DeleteLoadBalancerError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteLoadBalancerError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteRule
#[derive(Debug, PartialEq)]
pub enum DeleteRuleError {
    /// <p>This operation is not allowed.</p>
    OperationNotPermitted(String),
    /// <p>The specified rule does not exist.</p>
    RuleNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteRuleError {
    pub fn from_body(body: &str) -> DeleteRuleError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "OperationNotPermitted" => {
                    DeleteRuleError::OperationNotPermitted(String::from(parsed_error.message))
                }
                "RuleNotFound" => DeleteRuleError::RuleNotFound(String::from(parsed_error.message)),
                _ => DeleteRuleError::Unknown(String::from(body)),
            },
            Err(_) => DeleteRuleError::Unknown(body.to_string()),
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

impl From<XmlParseError> for DeleteRuleError {
    fn from(err: XmlParseError) -> DeleteRuleError {
        let XmlParseError(message) = err;
        DeleteRuleError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeleteRuleError {
    fn from(err: CredentialsError) -> DeleteRuleError {
        DeleteRuleError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteRuleError {
    fn from(err: HttpDispatchError) -> DeleteRuleError {
        DeleteRuleError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteRuleError {
    fn from(err: io::Error) -> DeleteRuleError {
        DeleteRuleError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteRuleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteRuleError {
    fn description(&self) -> &str {
        match *self {
            DeleteRuleError::OperationNotPermitted(ref cause) => cause,
            DeleteRuleError::RuleNotFound(ref cause) => cause,
            DeleteRuleError::Validation(ref cause) => cause,
            DeleteRuleError::Credentials(ref err) => err.description(),
            DeleteRuleError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteRuleError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteTargetGroup
#[derive(Debug, PartialEq)]
pub enum DeleteTargetGroupError {
    /// <p>A specified resource is in use.</p>
    ResourceInUse(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteTargetGroupError {
    pub fn from_body(body: &str) -> DeleteTargetGroupError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ResourceInUse" => {
                    DeleteTargetGroupError::ResourceInUse(String::from(parsed_error.message))
                }
                _ => DeleteTargetGroupError::Unknown(String::from(body)),
            },
            Err(_) => DeleteTargetGroupError::Unknown(body.to_string()),
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

impl From<XmlParseError> for DeleteTargetGroupError {
    fn from(err: XmlParseError) -> DeleteTargetGroupError {
        let XmlParseError(message) = err;
        DeleteTargetGroupError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeleteTargetGroupError {
    fn from(err: CredentialsError) -> DeleteTargetGroupError {
        DeleteTargetGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteTargetGroupError {
    fn from(err: HttpDispatchError) -> DeleteTargetGroupError {
        DeleteTargetGroupError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteTargetGroupError {
    fn from(err: io::Error) -> DeleteTargetGroupError {
        DeleteTargetGroupError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteTargetGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteTargetGroupError {
    fn description(&self) -> &str {
        match *self {
            DeleteTargetGroupError::ResourceInUse(ref cause) => cause,
            DeleteTargetGroupError::Validation(ref cause) => cause,
            DeleteTargetGroupError::Credentials(ref err) => err.description(),
            DeleteTargetGroupError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteTargetGroupError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeregisterTargets
#[derive(Debug, PartialEq)]
pub enum DeregisterTargetsError {
    /// <p>The specified target does not exist, is not in the same VPC as the target group, or has an unsupported instance type.</p>
    InvalidTarget(String),
    /// <p>The specified target group does not exist.</p>
    TargetGroupNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeregisterTargetsError {
    pub fn from_body(body: &str) -> DeregisterTargetsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "InvalidTarget" => {
                    DeregisterTargetsError::InvalidTarget(String::from(parsed_error.message))
                }
                "TargetGroupNotFound" => {
                    DeregisterTargetsError::TargetGroupNotFound(String::from(parsed_error.message))
                }
                _ => DeregisterTargetsError::Unknown(String::from(body)),
            },
            Err(_) => DeregisterTargetsError::Unknown(body.to_string()),
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

impl From<XmlParseError> for DeregisterTargetsError {
    fn from(err: XmlParseError) -> DeregisterTargetsError {
        let XmlParseError(message) = err;
        DeregisterTargetsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeregisterTargetsError {
    fn from(err: CredentialsError) -> DeregisterTargetsError {
        DeregisterTargetsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeregisterTargetsError {
    fn from(err: HttpDispatchError) -> DeregisterTargetsError {
        DeregisterTargetsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeregisterTargetsError {
    fn from(err: io::Error) -> DeregisterTargetsError {
        DeregisterTargetsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeregisterTargetsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeregisterTargetsError {
    fn description(&self) -> &str {
        match *self {
            DeregisterTargetsError::InvalidTarget(ref cause) => cause,
            DeregisterTargetsError::TargetGroupNotFound(ref cause) => cause,
            DeregisterTargetsError::Validation(ref cause) => cause,
            DeregisterTargetsError::Credentials(ref err) => err.description(),
            DeregisterTargetsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeregisterTargetsError::Unknown(ref cause) => cause,
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
/// Errors returned by DescribeListenerCertificates
#[derive(Debug, PartialEq)]
pub enum DescribeListenerCertificatesError {
    /// <p>The specified listener does not exist.</p>
    ListenerNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeListenerCertificatesError {
    pub fn from_body(body: &str) -> DescribeListenerCertificatesError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ListenerNotFound" => DescribeListenerCertificatesError::ListenerNotFound(
                    String::from(parsed_error.message),
                ),
                _ => DescribeListenerCertificatesError::Unknown(String::from(body)),
            },
            Err(_) => DescribeListenerCertificatesError::Unknown(body.to_string()),
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

impl From<XmlParseError> for DescribeListenerCertificatesError {
    fn from(err: XmlParseError) -> DescribeListenerCertificatesError {
        let XmlParseError(message) = err;
        DescribeListenerCertificatesError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DescribeListenerCertificatesError {
    fn from(err: CredentialsError) -> DescribeListenerCertificatesError {
        DescribeListenerCertificatesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeListenerCertificatesError {
    fn from(err: HttpDispatchError) -> DescribeListenerCertificatesError {
        DescribeListenerCertificatesError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeListenerCertificatesError {
    fn from(err: io::Error) -> DescribeListenerCertificatesError {
        DescribeListenerCertificatesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeListenerCertificatesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeListenerCertificatesError {
    fn description(&self) -> &str {
        match *self {
            DescribeListenerCertificatesError::ListenerNotFound(ref cause) => cause,
            DescribeListenerCertificatesError::Validation(ref cause) => cause,
            DescribeListenerCertificatesError::Credentials(ref err) => err.description(),
            DescribeListenerCertificatesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeListenerCertificatesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeListeners
#[derive(Debug, PartialEq)]
pub enum DescribeListenersError {
    /// <p>The specified listener does not exist.</p>
    ListenerNotFound(String),
    /// <p>The specified load balancer does not exist.</p>
    LoadBalancerNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeListenersError {
    pub fn from_body(body: &str) -> DescribeListenersError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ListenerNotFound" => {
                    DescribeListenersError::ListenerNotFound(String::from(parsed_error.message))
                }
                "LoadBalancerNotFound" => {
                    DescribeListenersError::LoadBalancerNotFound(String::from(parsed_error.message))
                }
                _ => DescribeListenersError::Unknown(String::from(body)),
            },
            Err(_) => DescribeListenersError::Unknown(body.to_string()),
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

impl From<XmlParseError> for DescribeListenersError {
    fn from(err: XmlParseError) -> DescribeListenersError {
        let XmlParseError(message) = err;
        DescribeListenersError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DescribeListenersError {
    fn from(err: CredentialsError) -> DescribeListenersError {
        DescribeListenersError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeListenersError {
    fn from(err: HttpDispatchError) -> DescribeListenersError {
        DescribeListenersError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeListenersError {
    fn from(err: io::Error) -> DescribeListenersError {
        DescribeListenersError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeListenersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeListenersError {
    fn description(&self) -> &str {
        match *self {
            DescribeListenersError::ListenerNotFound(ref cause) => cause,
            DescribeListenersError::LoadBalancerNotFound(ref cause) => cause,
            DescribeListenersError::Validation(ref cause) => cause,
            DescribeListenersError::Credentials(ref err) => err.description(),
            DescribeListenersError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeListenersError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeLoadBalancerAttributes
#[derive(Debug, PartialEq)]
pub enum DescribeLoadBalancerAttributesError {
    /// <p>The specified load balancer does not exist.</p>
    LoadBalancerNotFound(String),
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
                "LoadBalancerNotFound" => {
                    DescribeLoadBalancerAttributesError::LoadBalancerNotFound(String::from(
                        parsed_error.message,
                    ))
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
            DescribeLoadBalancerAttributesError::LoadBalancerNotFound(ref cause) => cause,
            DescribeLoadBalancerAttributesError::Validation(ref cause) => cause,
            DescribeLoadBalancerAttributesError::Credentials(ref err) => err.description(),
            DescribeLoadBalancerAttributesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeLoadBalancerAttributesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeLoadBalancers
#[derive(Debug, PartialEq)]
pub enum DescribeLoadBalancersError {
    /// <p>The specified load balancer does not exist.</p>
    LoadBalancerNotFound(String),
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
                "LoadBalancerNotFound" => DescribeLoadBalancersError::LoadBalancerNotFound(
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
            DescribeLoadBalancersError::LoadBalancerNotFound(ref cause) => cause,
            DescribeLoadBalancersError::Validation(ref cause) => cause,
            DescribeLoadBalancersError::Credentials(ref err) => err.description(),
            DescribeLoadBalancersError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeLoadBalancersError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeRules
#[derive(Debug, PartialEq)]
pub enum DescribeRulesError {
    /// <p>The specified listener does not exist.</p>
    ListenerNotFound(String),
    /// <p>The specified rule does not exist.</p>
    RuleNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeRulesError {
    pub fn from_body(body: &str) -> DescribeRulesError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ListenerNotFound" => {
                    DescribeRulesError::ListenerNotFound(String::from(parsed_error.message))
                }
                "RuleNotFound" => {
                    DescribeRulesError::RuleNotFound(String::from(parsed_error.message))
                }
                _ => DescribeRulesError::Unknown(String::from(body)),
            },
            Err(_) => DescribeRulesError::Unknown(body.to_string()),
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

impl From<XmlParseError> for DescribeRulesError {
    fn from(err: XmlParseError) -> DescribeRulesError {
        let XmlParseError(message) = err;
        DescribeRulesError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DescribeRulesError {
    fn from(err: CredentialsError) -> DescribeRulesError {
        DescribeRulesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeRulesError {
    fn from(err: HttpDispatchError) -> DescribeRulesError {
        DescribeRulesError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeRulesError {
    fn from(err: io::Error) -> DescribeRulesError {
        DescribeRulesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeRulesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeRulesError {
    fn description(&self) -> &str {
        match *self {
            DescribeRulesError::ListenerNotFound(ref cause) => cause,
            DescribeRulesError::RuleNotFound(ref cause) => cause,
            DescribeRulesError::Validation(ref cause) => cause,
            DescribeRulesError::Credentials(ref err) => err.description(),
            DescribeRulesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeRulesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeSSLPolicies
#[derive(Debug, PartialEq)]
pub enum DescribeSSLPoliciesError {
    /// <p>The specified SSL policy does not exist.</p>
    SSLPolicyNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeSSLPoliciesError {
    pub fn from_body(body: &str) -> DescribeSSLPoliciesError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "SSLPolicyNotFound" => {
                    DescribeSSLPoliciesError::SSLPolicyNotFound(String::from(parsed_error.message))
                }
                _ => DescribeSSLPoliciesError::Unknown(String::from(body)),
            },
            Err(_) => DescribeSSLPoliciesError::Unknown(body.to_string()),
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

impl From<XmlParseError> for DescribeSSLPoliciesError {
    fn from(err: XmlParseError) -> DescribeSSLPoliciesError {
        let XmlParseError(message) = err;
        DescribeSSLPoliciesError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DescribeSSLPoliciesError {
    fn from(err: CredentialsError) -> DescribeSSLPoliciesError {
        DescribeSSLPoliciesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeSSLPoliciesError {
    fn from(err: HttpDispatchError) -> DescribeSSLPoliciesError {
        DescribeSSLPoliciesError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeSSLPoliciesError {
    fn from(err: io::Error) -> DescribeSSLPoliciesError {
        DescribeSSLPoliciesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeSSLPoliciesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeSSLPoliciesError {
    fn description(&self) -> &str {
        match *self {
            DescribeSSLPoliciesError::SSLPolicyNotFound(ref cause) => cause,
            DescribeSSLPoliciesError::Validation(ref cause) => cause,
            DescribeSSLPoliciesError::Credentials(ref err) => err.description(),
            DescribeSSLPoliciesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeSSLPoliciesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeTags
#[derive(Debug, PartialEq)]
pub enum DescribeTagsError {
    /// <p>The specified listener does not exist.</p>
    ListenerNotFound(String),
    /// <p>The specified load balancer does not exist.</p>
    LoadBalancerNotFound(String),
    /// <p>The specified rule does not exist.</p>
    RuleNotFound(String),
    /// <p>The specified target group does not exist.</p>
    TargetGroupNotFound(String),
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
                "ListenerNotFound" => {
                    DescribeTagsError::ListenerNotFound(String::from(parsed_error.message))
                }
                "LoadBalancerNotFound" => {
                    DescribeTagsError::LoadBalancerNotFound(String::from(parsed_error.message))
                }
                "RuleNotFound" => {
                    DescribeTagsError::RuleNotFound(String::from(parsed_error.message))
                }
                "TargetGroupNotFound" => {
                    DescribeTagsError::TargetGroupNotFound(String::from(parsed_error.message))
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
            DescribeTagsError::ListenerNotFound(ref cause) => cause,
            DescribeTagsError::LoadBalancerNotFound(ref cause) => cause,
            DescribeTagsError::RuleNotFound(ref cause) => cause,
            DescribeTagsError::TargetGroupNotFound(ref cause) => cause,
            DescribeTagsError::Validation(ref cause) => cause,
            DescribeTagsError::Credentials(ref err) => err.description(),
            DescribeTagsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeTagsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeTargetGroupAttributes
#[derive(Debug, PartialEq)]
pub enum DescribeTargetGroupAttributesError {
    /// <p>The specified target group does not exist.</p>
    TargetGroupNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeTargetGroupAttributesError {
    pub fn from_body(body: &str) -> DescribeTargetGroupAttributesError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "TargetGroupNotFound" => DescribeTargetGroupAttributesError::TargetGroupNotFound(
                    String::from(parsed_error.message),
                ),
                _ => DescribeTargetGroupAttributesError::Unknown(String::from(body)),
            },
            Err(_) => DescribeTargetGroupAttributesError::Unknown(body.to_string()),
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

impl From<XmlParseError> for DescribeTargetGroupAttributesError {
    fn from(err: XmlParseError) -> DescribeTargetGroupAttributesError {
        let XmlParseError(message) = err;
        DescribeTargetGroupAttributesError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DescribeTargetGroupAttributesError {
    fn from(err: CredentialsError) -> DescribeTargetGroupAttributesError {
        DescribeTargetGroupAttributesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeTargetGroupAttributesError {
    fn from(err: HttpDispatchError) -> DescribeTargetGroupAttributesError {
        DescribeTargetGroupAttributesError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeTargetGroupAttributesError {
    fn from(err: io::Error) -> DescribeTargetGroupAttributesError {
        DescribeTargetGroupAttributesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeTargetGroupAttributesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeTargetGroupAttributesError {
    fn description(&self) -> &str {
        match *self {
            DescribeTargetGroupAttributesError::TargetGroupNotFound(ref cause) => cause,
            DescribeTargetGroupAttributesError::Validation(ref cause) => cause,
            DescribeTargetGroupAttributesError::Credentials(ref err) => err.description(),
            DescribeTargetGroupAttributesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeTargetGroupAttributesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeTargetGroups
#[derive(Debug, PartialEq)]
pub enum DescribeTargetGroupsError {
    /// <p>The specified load balancer does not exist.</p>
    LoadBalancerNotFound(String),
    /// <p>The specified target group does not exist.</p>
    TargetGroupNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeTargetGroupsError {
    pub fn from_body(body: &str) -> DescribeTargetGroupsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "LoadBalancerNotFound" => DescribeTargetGroupsError::LoadBalancerNotFound(
                    String::from(parsed_error.message),
                ),
                "TargetGroupNotFound" => DescribeTargetGroupsError::TargetGroupNotFound(
                    String::from(parsed_error.message),
                ),
                _ => DescribeTargetGroupsError::Unknown(String::from(body)),
            },
            Err(_) => DescribeTargetGroupsError::Unknown(body.to_string()),
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

impl From<XmlParseError> for DescribeTargetGroupsError {
    fn from(err: XmlParseError) -> DescribeTargetGroupsError {
        let XmlParseError(message) = err;
        DescribeTargetGroupsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DescribeTargetGroupsError {
    fn from(err: CredentialsError) -> DescribeTargetGroupsError {
        DescribeTargetGroupsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeTargetGroupsError {
    fn from(err: HttpDispatchError) -> DescribeTargetGroupsError {
        DescribeTargetGroupsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeTargetGroupsError {
    fn from(err: io::Error) -> DescribeTargetGroupsError {
        DescribeTargetGroupsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeTargetGroupsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeTargetGroupsError {
    fn description(&self) -> &str {
        match *self {
            DescribeTargetGroupsError::LoadBalancerNotFound(ref cause) => cause,
            DescribeTargetGroupsError::TargetGroupNotFound(ref cause) => cause,
            DescribeTargetGroupsError::Validation(ref cause) => cause,
            DescribeTargetGroupsError::Credentials(ref err) => err.description(),
            DescribeTargetGroupsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeTargetGroupsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeTargetHealth
#[derive(Debug, PartialEq)]
pub enum DescribeTargetHealthError {
    /// <p>The health of the specified targets could not be retrieved due to an internal error.</p>
    HealthUnavailable(String),
    /// <p>The specified target does not exist, is not in the same VPC as the target group, or has an unsupported instance type.</p>
    InvalidTarget(String),
    /// <p>The specified target group does not exist.</p>
    TargetGroupNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeTargetHealthError {
    pub fn from_body(body: &str) -> DescribeTargetHealthError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "HealthUnavailable" => {
                    DescribeTargetHealthError::HealthUnavailable(String::from(parsed_error.message))
                }
                "InvalidTarget" => {
                    DescribeTargetHealthError::InvalidTarget(String::from(parsed_error.message))
                }
                "TargetGroupNotFound" => DescribeTargetHealthError::TargetGroupNotFound(
                    String::from(parsed_error.message),
                ),
                _ => DescribeTargetHealthError::Unknown(String::from(body)),
            },
            Err(_) => DescribeTargetHealthError::Unknown(body.to_string()),
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

impl From<XmlParseError> for DescribeTargetHealthError {
    fn from(err: XmlParseError) -> DescribeTargetHealthError {
        let XmlParseError(message) = err;
        DescribeTargetHealthError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DescribeTargetHealthError {
    fn from(err: CredentialsError) -> DescribeTargetHealthError {
        DescribeTargetHealthError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeTargetHealthError {
    fn from(err: HttpDispatchError) -> DescribeTargetHealthError {
        DescribeTargetHealthError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeTargetHealthError {
    fn from(err: io::Error) -> DescribeTargetHealthError {
        DescribeTargetHealthError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeTargetHealthError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeTargetHealthError {
    fn description(&self) -> &str {
        match *self {
            DescribeTargetHealthError::HealthUnavailable(ref cause) => cause,
            DescribeTargetHealthError::InvalidTarget(ref cause) => cause,
            DescribeTargetHealthError::TargetGroupNotFound(ref cause) => cause,
            DescribeTargetHealthError::Validation(ref cause) => cause,
            DescribeTargetHealthError::Credentials(ref err) => err.description(),
            DescribeTargetHealthError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeTargetHealthError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ModifyListener
#[derive(Debug, PartialEq)]
pub enum ModifyListenerError {
    /// <p>The specified certificate does not exist.</p>
    CertificateNotFound(String),
    /// <p>A listener with the specified port already exists.</p>
    DuplicateListener(String),
    /// <p>The specified configuration is not valid with this protocol.</p>
    IncompatibleProtocols(String),
    /// <p>The requested configuration is not valid.</p>
    InvalidConfigurationRequest(String),
    /// <p>The specified listener does not exist.</p>
    ListenerNotFound(String),
    /// <p>The specified SSL policy does not exist.</p>
    SSLPolicyNotFound(String),
    /// <p>You've reached the limit on the number of load balancers per target group.</p>
    TargetGroupAssociationLimit(String),
    /// <p>The specified target group does not exist.</p>
    TargetGroupNotFound(String),
    /// <p>You've reached the limit on the number of certificates per load balancer.</p>
    TooManyCertificates(String),
    /// <p>You've reached the limit on the number of listeners per load balancer.</p>
    TooManyListeners(String),
    /// <p>You've reached the limit on the number of times a target can be registered with a load balancer.</p>
    TooManyRegistrationsForTargetId(String),
    /// <p>You've reached the limit on the number of targets.</p>
    TooManyTargets(String),
    /// <p>The specified protocol is not supported.</p>
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

impl ModifyListenerError {
    pub fn from_body(body: &str) -> ModifyListenerError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "CertificateNotFound" => {
                    ModifyListenerError::CertificateNotFound(String::from(parsed_error.message))
                }
                "DuplicateListener" => {
                    ModifyListenerError::DuplicateListener(String::from(parsed_error.message))
                }
                "IncompatibleProtocols" => {
                    ModifyListenerError::IncompatibleProtocols(String::from(parsed_error.message))
                }
                "InvalidConfigurationRequest" => ModifyListenerError::InvalidConfigurationRequest(
                    String::from(parsed_error.message),
                ),
                "ListenerNotFound" => {
                    ModifyListenerError::ListenerNotFound(String::from(parsed_error.message))
                }
                "SSLPolicyNotFound" => {
                    ModifyListenerError::SSLPolicyNotFound(String::from(parsed_error.message))
                }
                "TargetGroupAssociationLimit" => ModifyListenerError::TargetGroupAssociationLimit(
                    String::from(parsed_error.message),
                ),
                "TargetGroupNotFound" => {
                    ModifyListenerError::TargetGroupNotFound(String::from(parsed_error.message))
                }
                "TooManyCertificates" => {
                    ModifyListenerError::TooManyCertificates(String::from(parsed_error.message))
                }
                "TooManyListeners" => {
                    ModifyListenerError::TooManyListeners(String::from(parsed_error.message))
                }
                "TooManyRegistrationsForTargetId" => {
                    ModifyListenerError::TooManyRegistrationsForTargetId(String::from(
                        parsed_error.message,
                    ))
                }
                "TooManyTargets" => {
                    ModifyListenerError::TooManyTargets(String::from(parsed_error.message))
                }
                "UnsupportedProtocol" => {
                    ModifyListenerError::UnsupportedProtocol(String::from(parsed_error.message))
                }
                _ => ModifyListenerError::Unknown(String::from(body)),
            },
            Err(_) => ModifyListenerError::Unknown(body.to_string()),
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

impl From<XmlParseError> for ModifyListenerError {
    fn from(err: XmlParseError) -> ModifyListenerError {
        let XmlParseError(message) = err;
        ModifyListenerError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ModifyListenerError {
    fn from(err: CredentialsError) -> ModifyListenerError {
        ModifyListenerError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ModifyListenerError {
    fn from(err: HttpDispatchError) -> ModifyListenerError {
        ModifyListenerError::HttpDispatch(err)
    }
}
impl From<io::Error> for ModifyListenerError {
    fn from(err: io::Error) -> ModifyListenerError {
        ModifyListenerError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ModifyListenerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ModifyListenerError {
    fn description(&self) -> &str {
        match *self {
            ModifyListenerError::CertificateNotFound(ref cause) => cause,
            ModifyListenerError::DuplicateListener(ref cause) => cause,
            ModifyListenerError::IncompatibleProtocols(ref cause) => cause,
            ModifyListenerError::InvalidConfigurationRequest(ref cause) => cause,
            ModifyListenerError::ListenerNotFound(ref cause) => cause,
            ModifyListenerError::SSLPolicyNotFound(ref cause) => cause,
            ModifyListenerError::TargetGroupAssociationLimit(ref cause) => cause,
            ModifyListenerError::TargetGroupNotFound(ref cause) => cause,
            ModifyListenerError::TooManyCertificates(ref cause) => cause,
            ModifyListenerError::TooManyListeners(ref cause) => cause,
            ModifyListenerError::TooManyRegistrationsForTargetId(ref cause) => cause,
            ModifyListenerError::TooManyTargets(ref cause) => cause,
            ModifyListenerError::UnsupportedProtocol(ref cause) => cause,
            ModifyListenerError::Validation(ref cause) => cause,
            ModifyListenerError::Credentials(ref err) => err.description(),
            ModifyListenerError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ModifyListenerError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ModifyLoadBalancerAttributes
#[derive(Debug, PartialEq)]
pub enum ModifyLoadBalancerAttributesError {
    /// <p>The requested configuration is not valid.</p>
    InvalidConfigurationRequest(String),
    /// <p>The specified load balancer does not exist.</p>
    LoadBalancerNotFound(String),
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
                "InvalidConfigurationRequest" => {
                    ModifyLoadBalancerAttributesError::InvalidConfigurationRequest(String::from(
                        parsed_error.message,
                    ))
                }
                "LoadBalancerNotFound" => ModifyLoadBalancerAttributesError::LoadBalancerNotFound(
                    String::from(parsed_error.message),
                ),
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
            ModifyLoadBalancerAttributesError::InvalidConfigurationRequest(ref cause) => cause,
            ModifyLoadBalancerAttributesError::LoadBalancerNotFound(ref cause) => cause,
            ModifyLoadBalancerAttributesError::Validation(ref cause) => cause,
            ModifyLoadBalancerAttributesError::Credentials(ref err) => err.description(),
            ModifyLoadBalancerAttributesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ModifyLoadBalancerAttributesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ModifyRule
#[derive(Debug, PartialEq)]
pub enum ModifyRuleError {
    /// <p>The specified configuration is not valid with this protocol.</p>
    IncompatibleProtocols(String),
    /// <p>This operation is not allowed.</p>
    OperationNotPermitted(String),
    /// <p>The specified rule does not exist.</p>
    RuleNotFound(String),
    /// <p>You've reached the limit on the number of load balancers per target group.</p>
    TargetGroupAssociationLimit(String),
    /// <p>The specified target group does not exist.</p>
    TargetGroupNotFound(String),
    /// <p>You've reached the limit on the number of times a target can be registered with a load balancer.</p>
    TooManyRegistrationsForTargetId(String),
    /// <p>You've reached the limit on the number of targets.</p>
    TooManyTargets(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ModifyRuleError {
    pub fn from_body(body: &str) -> ModifyRuleError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "IncompatibleProtocols" => {
                    ModifyRuleError::IncompatibleProtocols(String::from(parsed_error.message))
                }
                "OperationNotPermitted" => {
                    ModifyRuleError::OperationNotPermitted(String::from(parsed_error.message))
                }
                "RuleNotFound" => ModifyRuleError::RuleNotFound(String::from(parsed_error.message)),
                "TargetGroupAssociationLimit" => {
                    ModifyRuleError::TargetGroupAssociationLimit(String::from(parsed_error.message))
                }
                "TargetGroupNotFound" => {
                    ModifyRuleError::TargetGroupNotFound(String::from(parsed_error.message))
                }
                "TooManyRegistrationsForTargetId" => {
                    ModifyRuleError::TooManyRegistrationsForTargetId(String::from(
                        parsed_error.message,
                    ))
                }
                "TooManyTargets" => {
                    ModifyRuleError::TooManyTargets(String::from(parsed_error.message))
                }
                _ => ModifyRuleError::Unknown(String::from(body)),
            },
            Err(_) => ModifyRuleError::Unknown(body.to_string()),
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

impl From<XmlParseError> for ModifyRuleError {
    fn from(err: XmlParseError) -> ModifyRuleError {
        let XmlParseError(message) = err;
        ModifyRuleError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ModifyRuleError {
    fn from(err: CredentialsError) -> ModifyRuleError {
        ModifyRuleError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ModifyRuleError {
    fn from(err: HttpDispatchError) -> ModifyRuleError {
        ModifyRuleError::HttpDispatch(err)
    }
}
impl From<io::Error> for ModifyRuleError {
    fn from(err: io::Error) -> ModifyRuleError {
        ModifyRuleError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ModifyRuleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ModifyRuleError {
    fn description(&self) -> &str {
        match *self {
            ModifyRuleError::IncompatibleProtocols(ref cause) => cause,
            ModifyRuleError::OperationNotPermitted(ref cause) => cause,
            ModifyRuleError::RuleNotFound(ref cause) => cause,
            ModifyRuleError::TargetGroupAssociationLimit(ref cause) => cause,
            ModifyRuleError::TargetGroupNotFound(ref cause) => cause,
            ModifyRuleError::TooManyRegistrationsForTargetId(ref cause) => cause,
            ModifyRuleError::TooManyTargets(ref cause) => cause,
            ModifyRuleError::Validation(ref cause) => cause,
            ModifyRuleError::Credentials(ref err) => err.description(),
            ModifyRuleError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ModifyRuleError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ModifyTargetGroup
#[derive(Debug, PartialEq)]
pub enum ModifyTargetGroupError {
    /// <p>The requested configuration is not valid.</p>
    InvalidConfigurationRequest(String),
    /// <p>The specified target group does not exist.</p>
    TargetGroupNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ModifyTargetGroupError {
    pub fn from_body(body: &str) -> ModifyTargetGroupError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "InvalidConfigurationRequest" => {
                    ModifyTargetGroupError::InvalidConfigurationRequest(String::from(
                        parsed_error.message,
                    ))
                }
                "TargetGroupNotFound" => {
                    ModifyTargetGroupError::TargetGroupNotFound(String::from(parsed_error.message))
                }
                _ => ModifyTargetGroupError::Unknown(String::from(body)),
            },
            Err(_) => ModifyTargetGroupError::Unknown(body.to_string()),
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

impl From<XmlParseError> for ModifyTargetGroupError {
    fn from(err: XmlParseError) -> ModifyTargetGroupError {
        let XmlParseError(message) = err;
        ModifyTargetGroupError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ModifyTargetGroupError {
    fn from(err: CredentialsError) -> ModifyTargetGroupError {
        ModifyTargetGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ModifyTargetGroupError {
    fn from(err: HttpDispatchError) -> ModifyTargetGroupError {
        ModifyTargetGroupError::HttpDispatch(err)
    }
}
impl From<io::Error> for ModifyTargetGroupError {
    fn from(err: io::Error) -> ModifyTargetGroupError {
        ModifyTargetGroupError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ModifyTargetGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ModifyTargetGroupError {
    fn description(&self) -> &str {
        match *self {
            ModifyTargetGroupError::InvalidConfigurationRequest(ref cause) => cause,
            ModifyTargetGroupError::TargetGroupNotFound(ref cause) => cause,
            ModifyTargetGroupError::Validation(ref cause) => cause,
            ModifyTargetGroupError::Credentials(ref err) => err.description(),
            ModifyTargetGroupError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ModifyTargetGroupError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ModifyTargetGroupAttributes
#[derive(Debug, PartialEq)]
pub enum ModifyTargetGroupAttributesError {
    /// <p>The requested configuration is not valid.</p>
    InvalidConfigurationRequest(String),
    /// <p>The specified target group does not exist.</p>
    TargetGroupNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ModifyTargetGroupAttributesError {
    pub fn from_body(body: &str) -> ModifyTargetGroupAttributesError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "InvalidConfigurationRequest" => {
                    ModifyTargetGroupAttributesError::InvalidConfigurationRequest(String::from(
                        parsed_error.message,
                    ))
                }
                "TargetGroupNotFound" => ModifyTargetGroupAttributesError::TargetGroupNotFound(
                    String::from(parsed_error.message),
                ),
                _ => ModifyTargetGroupAttributesError::Unknown(String::from(body)),
            },
            Err(_) => ModifyTargetGroupAttributesError::Unknown(body.to_string()),
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

impl From<XmlParseError> for ModifyTargetGroupAttributesError {
    fn from(err: XmlParseError) -> ModifyTargetGroupAttributesError {
        let XmlParseError(message) = err;
        ModifyTargetGroupAttributesError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ModifyTargetGroupAttributesError {
    fn from(err: CredentialsError) -> ModifyTargetGroupAttributesError {
        ModifyTargetGroupAttributesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ModifyTargetGroupAttributesError {
    fn from(err: HttpDispatchError) -> ModifyTargetGroupAttributesError {
        ModifyTargetGroupAttributesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ModifyTargetGroupAttributesError {
    fn from(err: io::Error) -> ModifyTargetGroupAttributesError {
        ModifyTargetGroupAttributesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ModifyTargetGroupAttributesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ModifyTargetGroupAttributesError {
    fn description(&self) -> &str {
        match *self {
            ModifyTargetGroupAttributesError::InvalidConfigurationRequest(ref cause) => cause,
            ModifyTargetGroupAttributesError::TargetGroupNotFound(ref cause) => cause,
            ModifyTargetGroupAttributesError::Validation(ref cause) => cause,
            ModifyTargetGroupAttributesError::Credentials(ref err) => err.description(),
            ModifyTargetGroupAttributesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ModifyTargetGroupAttributesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RegisterTargets
#[derive(Debug, PartialEq)]
pub enum RegisterTargetsError {
    /// <p>The specified target does not exist, is not in the same VPC as the target group, or has an unsupported instance type.</p>
    InvalidTarget(String),
    /// <p>The specified target group does not exist.</p>
    TargetGroupNotFound(String),
    /// <p>You've reached the limit on the number of times a target can be registered with a load balancer.</p>
    TooManyRegistrationsForTargetId(String),
    /// <p>You've reached the limit on the number of targets.</p>
    TooManyTargets(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl RegisterTargetsError {
    pub fn from_body(body: &str) -> RegisterTargetsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "InvalidTarget" => {
                    RegisterTargetsError::InvalidTarget(String::from(parsed_error.message))
                }
                "TargetGroupNotFound" => {
                    RegisterTargetsError::TargetGroupNotFound(String::from(parsed_error.message))
                }
                "TooManyRegistrationsForTargetId" => {
                    RegisterTargetsError::TooManyRegistrationsForTargetId(String::from(
                        parsed_error.message,
                    ))
                }
                "TooManyTargets" => {
                    RegisterTargetsError::TooManyTargets(String::from(parsed_error.message))
                }
                _ => RegisterTargetsError::Unknown(String::from(body)),
            },
            Err(_) => RegisterTargetsError::Unknown(body.to_string()),
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

impl From<XmlParseError> for RegisterTargetsError {
    fn from(err: XmlParseError) -> RegisterTargetsError {
        let XmlParseError(message) = err;
        RegisterTargetsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for RegisterTargetsError {
    fn from(err: CredentialsError) -> RegisterTargetsError {
        RegisterTargetsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RegisterTargetsError {
    fn from(err: HttpDispatchError) -> RegisterTargetsError {
        RegisterTargetsError::HttpDispatch(err)
    }
}
impl From<io::Error> for RegisterTargetsError {
    fn from(err: io::Error) -> RegisterTargetsError {
        RegisterTargetsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RegisterTargetsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RegisterTargetsError {
    fn description(&self) -> &str {
        match *self {
            RegisterTargetsError::InvalidTarget(ref cause) => cause,
            RegisterTargetsError::TargetGroupNotFound(ref cause) => cause,
            RegisterTargetsError::TooManyRegistrationsForTargetId(ref cause) => cause,
            RegisterTargetsError::TooManyTargets(ref cause) => cause,
            RegisterTargetsError::Validation(ref cause) => cause,
            RegisterTargetsError::Credentials(ref err) => err.description(),
            RegisterTargetsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            RegisterTargetsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RemoveListenerCertificates
#[derive(Debug, PartialEq)]
pub enum RemoveListenerCertificatesError {
    /// <p>The specified listener does not exist.</p>
    ListenerNotFound(String),
    /// <p>This operation is not allowed.</p>
    OperationNotPermitted(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl RemoveListenerCertificatesError {
    pub fn from_body(body: &str) -> RemoveListenerCertificatesError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ListenerNotFound" => RemoveListenerCertificatesError::ListenerNotFound(
                    String::from(parsed_error.message),
                ),
                "OperationNotPermitted" => RemoveListenerCertificatesError::OperationNotPermitted(
                    String::from(parsed_error.message),
                ),
                _ => RemoveListenerCertificatesError::Unknown(String::from(body)),
            },
            Err(_) => RemoveListenerCertificatesError::Unknown(body.to_string()),
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

impl From<XmlParseError> for RemoveListenerCertificatesError {
    fn from(err: XmlParseError) -> RemoveListenerCertificatesError {
        let XmlParseError(message) = err;
        RemoveListenerCertificatesError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for RemoveListenerCertificatesError {
    fn from(err: CredentialsError) -> RemoveListenerCertificatesError {
        RemoveListenerCertificatesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RemoveListenerCertificatesError {
    fn from(err: HttpDispatchError) -> RemoveListenerCertificatesError {
        RemoveListenerCertificatesError::HttpDispatch(err)
    }
}
impl From<io::Error> for RemoveListenerCertificatesError {
    fn from(err: io::Error) -> RemoveListenerCertificatesError {
        RemoveListenerCertificatesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RemoveListenerCertificatesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RemoveListenerCertificatesError {
    fn description(&self) -> &str {
        match *self {
            RemoveListenerCertificatesError::ListenerNotFound(ref cause) => cause,
            RemoveListenerCertificatesError::OperationNotPermitted(ref cause) => cause,
            RemoveListenerCertificatesError::Validation(ref cause) => cause,
            RemoveListenerCertificatesError::Credentials(ref err) => err.description(),
            RemoveListenerCertificatesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            RemoveListenerCertificatesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RemoveTags
#[derive(Debug, PartialEq)]
pub enum RemoveTagsError {
    /// <p>The specified listener does not exist.</p>
    ListenerNotFound(String),
    /// <p>The specified load balancer does not exist.</p>
    LoadBalancerNotFound(String),
    /// <p>The specified rule does not exist.</p>
    RuleNotFound(String),
    /// <p>The specified target group does not exist.</p>
    TargetGroupNotFound(String),
    /// <p>You've reached the limit on the number of tags per load balancer.</p>
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

impl RemoveTagsError {
    pub fn from_body(body: &str) -> RemoveTagsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ListenerNotFound" => {
                    RemoveTagsError::ListenerNotFound(String::from(parsed_error.message))
                }
                "LoadBalancerNotFound" => {
                    RemoveTagsError::LoadBalancerNotFound(String::from(parsed_error.message))
                }
                "RuleNotFound" => RemoveTagsError::RuleNotFound(String::from(parsed_error.message)),
                "TargetGroupNotFound" => {
                    RemoveTagsError::TargetGroupNotFound(String::from(parsed_error.message))
                }
                "TooManyTags" => RemoveTagsError::TooManyTags(String::from(parsed_error.message)),
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
            RemoveTagsError::ListenerNotFound(ref cause) => cause,
            RemoveTagsError::LoadBalancerNotFound(ref cause) => cause,
            RemoveTagsError::RuleNotFound(ref cause) => cause,
            RemoveTagsError::TargetGroupNotFound(ref cause) => cause,
            RemoveTagsError::TooManyTags(ref cause) => cause,
            RemoveTagsError::Validation(ref cause) => cause,
            RemoveTagsError::Credentials(ref err) => err.description(),
            RemoveTagsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            RemoveTagsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by SetIpAddressType
#[derive(Debug, PartialEq)]
pub enum SetIpAddressTypeError {
    /// <p>The requested configuration is not valid.</p>
    InvalidConfigurationRequest(String),
    /// <p>The specified subnet is out of available addresses.</p>
    InvalidSubnet(String),
    /// <p>The specified load balancer does not exist.</p>
    LoadBalancerNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl SetIpAddressTypeError {
    pub fn from_body(body: &str) -> SetIpAddressTypeError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "InvalidConfigurationRequest" => {
                    SetIpAddressTypeError::InvalidConfigurationRequest(String::from(
                        parsed_error.message,
                    ))
                }
                "InvalidSubnet" => {
                    SetIpAddressTypeError::InvalidSubnet(String::from(parsed_error.message))
                }
                "LoadBalancerNotFound" => {
                    SetIpAddressTypeError::LoadBalancerNotFound(String::from(parsed_error.message))
                }
                _ => SetIpAddressTypeError::Unknown(String::from(body)),
            },
            Err(_) => SetIpAddressTypeError::Unknown(body.to_string()),
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

impl From<XmlParseError> for SetIpAddressTypeError {
    fn from(err: XmlParseError) -> SetIpAddressTypeError {
        let XmlParseError(message) = err;
        SetIpAddressTypeError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for SetIpAddressTypeError {
    fn from(err: CredentialsError) -> SetIpAddressTypeError {
        SetIpAddressTypeError::Credentials(err)
    }
}
impl From<HttpDispatchError> for SetIpAddressTypeError {
    fn from(err: HttpDispatchError) -> SetIpAddressTypeError {
        SetIpAddressTypeError::HttpDispatch(err)
    }
}
impl From<io::Error> for SetIpAddressTypeError {
    fn from(err: io::Error) -> SetIpAddressTypeError {
        SetIpAddressTypeError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for SetIpAddressTypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SetIpAddressTypeError {
    fn description(&self) -> &str {
        match *self {
            SetIpAddressTypeError::InvalidConfigurationRequest(ref cause) => cause,
            SetIpAddressTypeError::InvalidSubnet(ref cause) => cause,
            SetIpAddressTypeError::LoadBalancerNotFound(ref cause) => cause,
            SetIpAddressTypeError::Validation(ref cause) => cause,
            SetIpAddressTypeError::Credentials(ref err) => err.description(),
            SetIpAddressTypeError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            SetIpAddressTypeError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by SetRulePriorities
#[derive(Debug, PartialEq)]
pub enum SetRulePrioritiesError {
    /// <p>This operation is not allowed.</p>
    OperationNotPermitted(String),
    /// <p>The specified priority is in use.</p>
    PriorityInUse(String),
    /// <p>The specified rule does not exist.</p>
    RuleNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl SetRulePrioritiesError {
    pub fn from_body(body: &str) -> SetRulePrioritiesError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "OperationNotPermitted" => SetRulePrioritiesError::OperationNotPermitted(
                    String::from(parsed_error.message),
                ),
                "PriorityInUse" => {
                    SetRulePrioritiesError::PriorityInUse(String::from(parsed_error.message))
                }
                "RuleNotFound" => {
                    SetRulePrioritiesError::RuleNotFound(String::from(parsed_error.message))
                }
                _ => SetRulePrioritiesError::Unknown(String::from(body)),
            },
            Err(_) => SetRulePrioritiesError::Unknown(body.to_string()),
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

impl From<XmlParseError> for SetRulePrioritiesError {
    fn from(err: XmlParseError) -> SetRulePrioritiesError {
        let XmlParseError(message) = err;
        SetRulePrioritiesError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for SetRulePrioritiesError {
    fn from(err: CredentialsError) -> SetRulePrioritiesError {
        SetRulePrioritiesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for SetRulePrioritiesError {
    fn from(err: HttpDispatchError) -> SetRulePrioritiesError {
        SetRulePrioritiesError::HttpDispatch(err)
    }
}
impl From<io::Error> for SetRulePrioritiesError {
    fn from(err: io::Error) -> SetRulePrioritiesError {
        SetRulePrioritiesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for SetRulePrioritiesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SetRulePrioritiesError {
    fn description(&self) -> &str {
        match *self {
            SetRulePrioritiesError::OperationNotPermitted(ref cause) => cause,
            SetRulePrioritiesError::PriorityInUse(ref cause) => cause,
            SetRulePrioritiesError::RuleNotFound(ref cause) => cause,
            SetRulePrioritiesError::Validation(ref cause) => cause,
            SetRulePrioritiesError::Credentials(ref err) => err.description(),
            SetRulePrioritiesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            SetRulePrioritiesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by SetSecurityGroups
#[derive(Debug, PartialEq)]
pub enum SetSecurityGroupsError {
    /// <p>The requested configuration is not valid.</p>
    InvalidConfigurationRequest(String),
    /// <p>The specified security group does not exist.</p>
    InvalidSecurityGroup(String),
    /// <p>The specified load balancer does not exist.</p>
    LoadBalancerNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl SetSecurityGroupsError {
    pub fn from_body(body: &str) -> SetSecurityGroupsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "InvalidConfigurationRequest" => {
                    SetSecurityGroupsError::InvalidConfigurationRequest(String::from(
                        parsed_error.message,
                    ))
                }
                "InvalidSecurityGroup" => {
                    SetSecurityGroupsError::InvalidSecurityGroup(String::from(parsed_error.message))
                }
                "LoadBalancerNotFound" => {
                    SetSecurityGroupsError::LoadBalancerNotFound(String::from(parsed_error.message))
                }
                _ => SetSecurityGroupsError::Unknown(String::from(body)),
            },
            Err(_) => SetSecurityGroupsError::Unknown(body.to_string()),
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

impl From<XmlParseError> for SetSecurityGroupsError {
    fn from(err: XmlParseError) -> SetSecurityGroupsError {
        let XmlParseError(message) = err;
        SetSecurityGroupsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for SetSecurityGroupsError {
    fn from(err: CredentialsError) -> SetSecurityGroupsError {
        SetSecurityGroupsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for SetSecurityGroupsError {
    fn from(err: HttpDispatchError) -> SetSecurityGroupsError {
        SetSecurityGroupsError::HttpDispatch(err)
    }
}
impl From<io::Error> for SetSecurityGroupsError {
    fn from(err: io::Error) -> SetSecurityGroupsError {
        SetSecurityGroupsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for SetSecurityGroupsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SetSecurityGroupsError {
    fn description(&self) -> &str {
        match *self {
            SetSecurityGroupsError::InvalidConfigurationRequest(ref cause) => cause,
            SetSecurityGroupsError::InvalidSecurityGroup(ref cause) => cause,
            SetSecurityGroupsError::LoadBalancerNotFound(ref cause) => cause,
            SetSecurityGroupsError::Validation(ref cause) => cause,
            SetSecurityGroupsError::Credentials(ref err) => err.description(),
            SetSecurityGroupsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            SetSecurityGroupsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by SetSubnets
#[derive(Debug, PartialEq)]
pub enum SetSubnetsError {
    /// <p>The specified allocation ID does not exist.</p>
    AllocationIdNotFound(String),
    /// <p>The specified Availability Zone is not supported.</p>
    AvailabilityZoneNotSupported(String),
    /// <p>The requested configuration is not valid.</p>
    InvalidConfigurationRequest(String),
    /// <p>The specified subnet is out of available addresses.</p>
    InvalidSubnet(String),
    /// <p>The specified load balancer does not exist.</p>
    LoadBalancerNotFound(String),
    /// <p>The specified subnet does not exist.</p>
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

impl SetSubnetsError {
    pub fn from_body(body: &str) -> SetSubnetsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "AllocationIdNotFound" => {
                    SetSubnetsError::AllocationIdNotFound(String::from(parsed_error.message))
                }
                "AvailabilityZoneNotSupported" => SetSubnetsError::AvailabilityZoneNotSupported(
                    String::from(parsed_error.message),
                ),
                "InvalidConfigurationRequest" => {
                    SetSubnetsError::InvalidConfigurationRequest(String::from(parsed_error.message))
                }
                "InvalidSubnet" => {
                    SetSubnetsError::InvalidSubnet(String::from(parsed_error.message))
                }
                "LoadBalancerNotFound" => {
                    SetSubnetsError::LoadBalancerNotFound(String::from(parsed_error.message))
                }
                "SubnetNotFound" => {
                    SetSubnetsError::SubnetNotFound(String::from(parsed_error.message))
                }
                _ => SetSubnetsError::Unknown(String::from(body)),
            },
            Err(_) => SetSubnetsError::Unknown(body.to_string()),
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

impl From<XmlParseError> for SetSubnetsError {
    fn from(err: XmlParseError) -> SetSubnetsError {
        let XmlParseError(message) = err;
        SetSubnetsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for SetSubnetsError {
    fn from(err: CredentialsError) -> SetSubnetsError {
        SetSubnetsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for SetSubnetsError {
    fn from(err: HttpDispatchError) -> SetSubnetsError {
        SetSubnetsError::HttpDispatch(err)
    }
}
impl From<io::Error> for SetSubnetsError {
    fn from(err: io::Error) -> SetSubnetsError {
        SetSubnetsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for SetSubnetsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SetSubnetsError {
    fn description(&self) -> &str {
        match *self {
            SetSubnetsError::AllocationIdNotFound(ref cause) => cause,
            SetSubnetsError::AvailabilityZoneNotSupported(ref cause) => cause,
            SetSubnetsError::InvalidConfigurationRequest(ref cause) => cause,
            SetSubnetsError::InvalidSubnet(ref cause) => cause,
            SetSubnetsError::LoadBalancerNotFound(ref cause) => cause,
            SetSubnetsError::SubnetNotFound(ref cause) => cause,
            SetSubnetsError::Validation(ref cause) => cause,
            SetSubnetsError::Credentials(ref err) => err.description(),
            SetSubnetsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            SetSubnetsError::Unknown(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the Elastic Load Balancing v2 API. Elastic Load Balancing v2 clients implement this trait.
pub trait Elb {
    /// <p>Adds the specified certificate to the specified secure listener.</p> <p>If the certificate was already added, the call is successful but the certificate is not added again.</p> <p>To list the certificates for your listener, use <a>DescribeListenerCertificates</a>. To remove certificates from your listener, use <a>RemoveListenerCertificates</a>.</p>
    fn add_listener_certificates(
        &self,
        input: AddListenerCertificatesInput,
    ) -> RusotoFuture<AddListenerCertificatesOutput, AddListenerCertificatesError>;

    /// <p>Adds the specified tags to the specified Elastic Load Balancing resource. You can tag your Application Load Balancers, Network Load Balancers, and your target groups.</p> <p>Each tag consists of a key and an optional value. If a resource already has a tag with the same key, <code>AddTags</code> updates its value.</p> <p>To list the current tags for your resources, use <a>DescribeTags</a>. To remove tags from your resources, use <a>RemoveTags</a>.</p>
    fn add_tags(&self, input: AddTagsInput) -> RusotoFuture<AddTagsOutput, AddTagsError>;

    /// <p>Creates a listener for the specified Application Load Balancer or Network Load Balancer.</p> <p>To update a listener, use <a>ModifyListener</a>. When you are finished with a listener, you can delete it using <a>DeleteListener</a>. If you are finished with both the listener and the load balancer, you can delete them both using <a>DeleteLoadBalancer</a>.</p> <p>This operation is idempotent, which means that it completes at most one time. If you attempt to create multiple listeners with the same settings, each call succeeds.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/elasticloadbalancing/latest/application/load-balancer-listeners.html">Listeners for Your Application Load Balancers</a> in the <i>Application Load Balancers Guide</i> and <a href="http://docs.aws.amazon.com/elasticloadbalancing/latest/network/load-balancer-listeners.html">Listeners for Your Network Load Balancers</a> in the <i>Network Load Balancers Guide</i>.</p>
    fn create_listener(
        &self,
        input: CreateListenerInput,
    ) -> RusotoFuture<CreateListenerOutput, CreateListenerError>;

    /// <p>Creates an Application Load Balancer or a Network Load Balancer.</p> <p>When you create a load balancer, you can specify security groups, public subnets, IP address type, and tags. Otherwise, you could do so later using <a>SetSecurityGroups</a>, <a>SetSubnets</a>, <a>SetIpAddressType</a>, and <a>AddTags</a>.</p> <p>To create listeners for your load balancer, use <a>CreateListener</a>. To describe your current load balancers, see <a>DescribeLoadBalancers</a>. When you are finished with a load balancer, you can delete it using <a>DeleteLoadBalancer</a>.</p> <p>For limit information, see <a href="http://docs.aws.amazon.com/elasticloadbalancing/latest/application/load-balancer-limits.html">Limits for Your Application Load Balancer</a> in the <i>Application Load Balancers Guide</i> and <a href="http://docs.aws.amazon.com/elasticloadbalancing/latest/network/load-balancer-limits.html">Limits for Your Network Load Balancer</a> in the <i>Network Load Balancers Guide</i>.</p> <p>This operation is idempotent, which means that it completes at most one time. If you attempt to create multiple load balancers with the same settings, each call succeeds.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/elasticloadbalancing/latest/application/application-load-balancers.html">Application Load Balancers</a> in the <i>Application Load Balancers Guide</i> and <a href="http://docs.aws.amazon.com/elasticloadbalancing/latest/network/network-load-balancers.html">Network Load Balancers</a> in the <i>Network Load Balancers Guide</i>.</p>
    fn create_load_balancer(
        &self,
        input: CreateLoadBalancerInput,
    ) -> RusotoFuture<CreateLoadBalancerOutput, CreateLoadBalancerError>;

    /// <p>Creates a rule for the specified listener. The listener must be associated with an Application Load Balancer.</p> <p>Rules are evaluated in priority order, from the lowest value to the highest value. When the condition for a rule is met, the specified action is taken. If no conditions are met, the action for the default rule is taken. For more information, see <a href="http://docs.aws.amazon.com/elasticloadbalancing/latest/application/load-balancer-listeners.html#listener-rules">Listener Rules</a> in the <i>Application Load Balancers Guide</i>.</p> <p>To view your current rules, use <a>DescribeRules</a>. To update a rule, use <a>ModifyRule</a>. To set the priorities of your rules, use <a>SetRulePriorities</a>. To delete a rule, use <a>DeleteRule</a>.</p>
    fn create_rule(
        &self,
        input: CreateRuleInput,
    ) -> RusotoFuture<CreateRuleOutput, CreateRuleError>;

    /// <p>Creates a target group.</p> <p>To register targets with the target group, use <a>RegisterTargets</a>. To update the health check settings for the target group, use <a>ModifyTargetGroup</a>. To monitor the health of targets in the target group, use <a>DescribeTargetHealth</a>.</p> <p>To route traffic to the targets in a target group, specify the target group in an action using <a>CreateListener</a> or <a>CreateRule</a>.</p> <p>To delete a target group, use <a>DeleteTargetGroup</a>.</p> <p>This operation is idempotent, which means that it completes at most one time. If you attempt to create multiple target groups with the same settings, each call succeeds.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/elasticloadbalancing/latest/application/load-balancer-target-groups.html">Target Groups for Your Application Load Balancers</a> in the <i>Application Load Balancers Guide</i> or <a href="http://docs.aws.amazon.com/elasticloadbalancing/latest/network/load-balancer-target-groups.html">Target Groups for Your Network Load Balancers</a> in the <i>Network Load Balancers Guide</i>.</p>
    fn create_target_group(
        &self,
        input: CreateTargetGroupInput,
    ) -> RusotoFuture<CreateTargetGroupOutput, CreateTargetGroupError>;

    /// <p>Deletes the specified listener.</p> <p>Alternatively, your listener is deleted when you delete the load balancer it is attached to using <a>DeleteLoadBalancer</a>.</p>
    fn delete_listener(
        &self,
        input: DeleteListenerInput,
    ) -> RusotoFuture<DeleteListenerOutput, DeleteListenerError>;

    /// <p>Deletes the specified Application Load Balancer or Network Load Balancer and its attached listeners.</p> <p>You can't delete a load balancer if deletion protection is enabled. If the load balancer does not exist or has already been deleted, the call succeeds.</p> <p>Deleting a load balancer does not affect its registered targets. For example, your EC2 instances continue to run and are still registered to their target groups. If you no longer need these EC2 instances, you can stop or terminate them.</p>
    fn delete_load_balancer(
        &self,
        input: DeleteLoadBalancerInput,
    ) -> RusotoFuture<DeleteLoadBalancerOutput, DeleteLoadBalancerError>;

    /// <p>Deletes the specified rule.</p>
    fn delete_rule(
        &self,
        input: DeleteRuleInput,
    ) -> RusotoFuture<DeleteRuleOutput, DeleteRuleError>;

    /// <p>Deletes the specified target group.</p> <p>You can delete a target group if it is not referenced by any actions. Deleting a target group also deletes any associated health checks.</p>
    fn delete_target_group(
        &self,
        input: DeleteTargetGroupInput,
    ) -> RusotoFuture<DeleteTargetGroupOutput, DeleteTargetGroupError>;

    /// <p>Deregisters the specified targets from the specified target group. After the targets are deregistered, they no longer receive traffic from the load balancer.</p>
    fn deregister_targets(
        &self,
        input: DeregisterTargetsInput,
    ) -> RusotoFuture<DeregisterTargetsOutput, DeregisterTargetsError>;

    /// <p>Describes the current Elastic Load Balancing resource limits for your AWS account.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/elasticloadbalancing/latest/application/load-balancer-limits.html">Limits for Your Application Load Balancers</a> in the <i>Application Load Balancer Guide</i> or <a href="http://docs.aws.amazon.com/elasticloadbalancing/latest/network/load-balancer-limits.html">Limits for Your Network Load Balancers</a> in the <i>Network Load Balancers Guide</i>.</p>
    fn describe_account_limits(
        &self,
        input: DescribeAccountLimitsInput,
    ) -> RusotoFuture<DescribeAccountLimitsOutput, DescribeAccountLimitsError>;

    /// <p>Describes the certificates for the specified secure listener.</p>
    fn describe_listener_certificates(
        &self,
        input: DescribeListenerCertificatesInput,
    ) -> RusotoFuture<DescribeListenerCertificatesOutput, DescribeListenerCertificatesError>;

    /// <p>Describes the specified listeners or the listeners for the specified Application Load Balancer or Network Load Balancer. You must specify either a load balancer or one or more listeners.</p>
    fn describe_listeners(
        &self,
        input: DescribeListenersInput,
    ) -> RusotoFuture<DescribeListenersOutput, DescribeListenersError>;

    /// <p>Describes the attributes for the specified Application Load Balancer or Network Load Balancer.</p>
    fn describe_load_balancer_attributes(
        &self,
        input: DescribeLoadBalancerAttributesInput,
    ) -> RusotoFuture<DescribeLoadBalancerAttributesOutput, DescribeLoadBalancerAttributesError>;

    /// <p>Describes the specified load balancers or all of your load balancers.</p> <p>To describe the listeners for a load balancer, use <a>DescribeListeners</a>. To describe the attributes for a load balancer, use <a>DescribeLoadBalancerAttributes</a>.</p>
    fn describe_load_balancers(
        &self,
        input: DescribeLoadBalancersInput,
    ) -> RusotoFuture<DescribeLoadBalancersOutput, DescribeLoadBalancersError>;

    /// <p>Describes the specified rules or the rules for the specified listener. You must specify either a listener or one or more rules.</p>
    fn describe_rules(
        &self,
        input: DescribeRulesInput,
    ) -> RusotoFuture<DescribeRulesOutput, DescribeRulesError>;

    /// <p>Describes the specified policies or all policies used for SSL negotiation.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/elasticloadbalancing/latest/application/create-https-listener.html#describe-ssl-policies">Security Policies</a> in the <i>Application Load Balancers Guide</i>.</p>
    fn describe_ssl_policies(
        &self,
        input: DescribeSSLPoliciesInput,
    ) -> RusotoFuture<DescribeSSLPoliciesOutput, DescribeSSLPoliciesError>;

    /// <p>Describes the tags for the specified resources. You can describe the tags for one or more Application Load Balancers, Network Load Balancers, and target groups.</p>
    fn describe_tags(
        &self,
        input: DescribeTagsInput,
    ) -> RusotoFuture<DescribeTagsOutput, DescribeTagsError>;

    /// <p>Describes the attributes for the specified target group.</p>
    fn describe_target_group_attributes(
        &self,
        input: DescribeTargetGroupAttributesInput,
    ) -> RusotoFuture<DescribeTargetGroupAttributesOutput, DescribeTargetGroupAttributesError>;

    /// <p>Describes the specified target groups or all of your target groups. By default, all target groups are described. Alternatively, you can specify one of the following to filter the results: the ARN of the load balancer, the names of one or more target groups, or the ARNs of one or more target groups.</p> <p>To describe the targets for a target group, use <a>DescribeTargetHealth</a>. To describe the attributes of a target group, use <a>DescribeTargetGroupAttributes</a>.</p>
    fn describe_target_groups(
        &self,
        input: DescribeTargetGroupsInput,
    ) -> RusotoFuture<DescribeTargetGroupsOutput, DescribeTargetGroupsError>;

    /// <p>Describes the health of the specified targets or all of your targets.</p>
    fn describe_target_health(
        &self,
        input: DescribeTargetHealthInput,
    ) -> RusotoFuture<DescribeTargetHealthOutput, DescribeTargetHealthError>;

    /// <p>Modifies the specified properties of the specified listener.</p> <p>Any properties that you do not specify retain their current values. However, changing the protocol from HTTPS to HTTP removes the security policy and SSL certificate properties. If you change the protocol from HTTP to HTTPS, you must add the security policy and server certificate.</p>
    fn modify_listener(
        &self,
        input: ModifyListenerInput,
    ) -> RusotoFuture<ModifyListenerOutput, ModifyListenerError>;

    /// <p>Modifies the specified attributes of the specified Application Load Balancer or Network Load Balancer.</p> <p>If any of the specified attributes can't be modified as requested, the call fails. Any existing attributes that you do not modify retain their current values.</p>
    fn modify_load_balancer_attributes(
        &self,
        input: ModifyLoadBalancerAttributesInput,
    ) -> RusotoFuture<ModifyLoadBalancerAttributesOutput, ModifyLoadBalancerAttributesError>;

    /// <p>Modifies the specified rule.</p> <p>Any existing properties that you do not modify retain their current values.</p> <p>To modify the default action, use <a>ModifyListener</a>.</p>
    fn modify_rule(
        &self,
        input: ModifyRuleInput,
    ) -> RusotoFuture<ModifyRuleOutput, ModifyRuleError>;

    /// <p>Modifies the health checks used when evaluating the health state of the targets in the specified target group.</p> <p>To monitor the health of the targets, use <a>DescribeTargetHealth</a>.</p>
    fn modify_target_group(
        &self,
        input: ModifyTargetGroupInput,
    ) -> RusotoFuture<ModifyTargetGroupOutput, ModifyTargetGroupError>;

    /// <p>Modifies the specified attributes of the specified target group.</p>
    fn modify_target_group_attributes(
        &self,
        input: ModifyTargetGroupAttributesInput,
    ) -> RusotoFuture<ModifyTargetGroupAttributesOutput, ModifyTargetGroupAttributesError>;

    /// <p>Registers the specified targets with the specified target group.</p> <p>You can register targets by instance ID or by IP address. If the target is an EC2 instance, it must be in the <code>running</code> state when you register it.</p> <p>By default, the load balancer routes requests to registered targets using the protocol and port for the target group. Alternatively, you can override the port for a target when you register it. You can register each EC2 instance or IP address with the same target group multiple times using different ports.</p> <p>With a Network Load Balancer, you cannot register instances by instance ID if they have the following instance types: C1, CC1, CC2, CG1, CG2, CR1, CS1, G1, G2, HI1, HS1, M1, M2, M3, and T1. You can register instances of these types by IP address.</p> <p>To remove a target from a target group, use <a>DeregisterTargets</a>.</p>
    fn register_targets(
        &self,
        input: RegisterTargetsInput,
    ) -> RusotoFuture<RegisterTargetsOutput, RegisterTargetsError>;

    /// <p>Removes the specified certificate from the specified secure listener.</p> <p>You can't remove the default certificate for a listener. To replace the default certificate, call <a>ModifyListener</a>.</p> <p>To list the certificates for your listener, use <a>DescribeListenerCertificates</a>.</p>
    fn remove_listener_certificates(
        &self,
        input: RemoveListenerCertificatesInput,
    ) -> RusotoFuture<RemoveListenerCertificatesOutput, RemoveListenerCertificatesError>;

    /// <p>Removes the specified tags from the specified Elastic Load Balancing resource.</p> <p>To list the current tags for your resources, use <a>DescribeTags</a>.</p>
    fn remove_tags(
        &self,
        input: RemoveTagsInput,
    ) -> RusotoFuture<RemoveTagsOutput, RemoveTagsError>;

    /// <p>Sets the type of IP addresses used by the subnets of the specified Application Load Balancer or Network Load Balancer.</p> <p>Note that Network Load Balancers must use <code>ipv4</code>.</p>
    fn set_ip_address_type(
        &self,
        input: SetIpAddressTypeInput,
    ) -> RusotoFuture<SetIpAddressTypeOutput, SetIpAddressTypeError>;

    /// <p>Sets the priorities of the specified rules.</p> <p>You can reorder the rules as long as there are no priority conflicts in the new order. Any existing rules that you do not specify retain their current priority.</p>
    fn set_rule_priorities(
        &self,
        input: SetRulePrioritiesInput,
    ) -> RusotoFuture<SetRulePrioritiesOutput, SetRulePrioritiesError>;

    /// <p>Associates the specified security groups with the specified Application Load Balancer. The specified security groups override the previously associated security groups.</p> <p>Note that you can't specify a security group for a Network Load Balancer.</p>
    fn set_security_groups(
        &self,
        input: SetSecurityGroupsInput,
    ) -> RusotoFuture<SetSecurityGroupsOutput, SetSecurityGroupsError>;

    /// <p>Enables the Availability Zone for the specified public subnets for the specified Application Load Balancer. The specified subnets replace the previously enabled subnets.</p> <p>Note that you can't change the subnets for a Network Load Balancer.</p>
    fn set_subnets(
        &self,
        input: SetSubnetsInput,
    ) -> RusotoFuture<SetSubnetsOutput, SetSubnetsError>;
}
/// A client for the Elastic Load Balancing v2 API.
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
    /// <p>Adds the specified certificate to the specified secure listener.</p> <p>If the certificate was already added, the call is successful but the certificate is not added again.</p> <p>To list the certificates for your listener, use <a>DescribeListenerCertificates</a>. To remove certificates from your listener, use <a>RemoveListenerCertificates</a>.</p>
    fn add_listener_certificates(
        &self,
        input: AddListenerCertificatesInput,
    ) -> RusotoFuture<AddListenerCertificatesOutput, AddListenerCertificatesError> {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "AddListenerCertificates");
        params.put("Version", "2015-12-01");
        AddListenerCertificatesInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(AddListenerCertificatesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = AddListenerCertificatesOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(AddListenerCertificatesOutputDeserializer::deserialize(
                        "AddListenerCertificatesResult",
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

    /// <p>Adds the specified tags to the specified Elastic Load Balancing resource. You can tag your Application Load Balancers, Network Load Balancers, and your target groups.</p> <p>Each tag consists of a key and an optional value. If a resource already has a tag with the same key, <code>AddTags</code> updates its value.</p> <p>To list the current tags for your resources, use <a>DescribeTags</a>. To remove tags from your resources, use <a>RemoveTags</a>.</p>
    fn add_tags(&self, input: AddTagsInput) -> RusotoFuture<AddTagsOutput, AddTagsError> {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "AddTags");
        params.put("Version", "2015-12-01");
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

    /// <p>Creates a listener for the specified Application Load Balancer or Network Load Balancer.</p> <p>To update a listener, use <a>ModifyListener</a>. When you are finished with a listener, you can delete it using <a>DeleteListener</a>. If you are finished with both the listener and the load balancer, you can delete them both using <a>DeleteLoadBalancer</a>.</p> <p>This operation is idempotent, which means that it completes at most one time. If you attempt to create multiple listeners with the same settings, each call succeeds.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/elasticloadbalancing/latest/application/load-balancer-listeners.html">Listeners for Your Application Load Balancers</a> in the <i>Application Load Balancers Guide</i> and <a href="http://docs.aws.amazon.com/elasticloadbalancing/latest/network/load-balancer-listeners.html">Listeners for Your Network Load Balancers</a> in the <i>Network Load Balancers Guide</i>.</p>
    fn create_listener(
        &self,
        input: CreateListenerInput,
    ) -> RusotoFuture<CreateListenerOutput, CreateListenerError> {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CreateListener");
        params.put("Version", "2015-12-01");
        CreateListenerInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateListenerError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = CreateListenerOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(CreateListenerOutputDeserializer::deserialize(
                        "CreateListenerResult",
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

    /// <p>Creates an Application Load Balancer or a Network Load Balancer.</p> <p>When you create a load balancer, you can specify security groups, public subnets, IP address type, and tags. Otherwise, you could do so later using <a>SetSecurityGroups</a>, <a>SetSubnets</a>, <a>SetIpAddressType</a>, and <a>AddTags</a>.</p> <p>To create listeners for your load balancer, use <a>CreateListener</a>. To describe your current load balancers, see <a>DescribeLoadBalancers</a>. When you are finished with a load balancer, you can delete it using <a>DeleteLoadBalancer</a>.</p> <p>For limit information, see <a href="http://docs.aws.amazon.com/elasticloadbalancing/latest/application/load-balancer-limits.html">Limits for Your Application Load Balancer</a> in the <i>Application Load Balancers Guide</i> and <a href="http://docs.aws.amazon.com/elasticloadbalancing/latest/network/load-balancer-limits.html">Limits for Your Network Load Balancer</a> in the <i>Network Load Balancers Guide</i>.</p> <p>This operation is idempotent, which means that it completes at most one time. If you attempt to create multiple load balancers with the same settings, each call succeeds.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/elasticloadbalancing/latest/application/application-load-balancers.html">Application Load Balancers</a> in the <i>Application Load Balancers Guide</i> and <a href="http://docs.aws.amazon.com/elasticloadbalancing/latest/network/network-load-balancers.html">Network Load Balancers</a> in the <i>Network Load Balancers Guide</i>.</p>
    fn create_load_balancer(
        &self,
        input: CreateLoadBalancerInput,
    ) -> RusotoFuture<CreateLoadBalancerOutput, CreateLoadBalancerError> {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CreateLoadBalancer");
        params.put("Version", "2015-12-01");
        CreateLoadBalancerInputSerializer::serialize(&mut params, "", &input);
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
                    result = CreateLoadBalancerOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(CreateLoadBalancerOutputDeserializer::deserialize(
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

    /// <p>Creates a rule for the specified listener. The listener must be associated with an Application Load Balancer.</p> <p>Rules are evaluated in priority order, from the lowest value to the highest value. When the condition for a rule is met, the specified action is taken. If no conditions are met, the action for the default rule is taken. For more information, see <a href="http://docs.aws.amazon.com/elasticloadbalancing/latest/application/load-balancer-listeners.html#listener-rules">Listener Rules</a> in the <i>Application Load Balancers Guide</i>.</p> <p>To view your current rules, use <a>DescribeRules</a>. To update a rule, use <a>ModifyRule</a>. To set the priorities of your rules, use <a>SetRulePriorities</a>. To delete a rule, use <a>DeleteRule</a>.</p>
    fn create_rule(
        &self,
        input: CreateRuleInput,
    ) -> RusotoFuture<CreateRuleOutput, CreateRuleError> {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CreateRule");
        params.put("Version", "2015-12-01");
        CreateRuleInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateRuleError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = CreateRuleOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(CreateRuleOutputDeserializer::deserialize(
                        "CreateRuleResult",
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

    /// <p>Creates a target group.</p> <p>To register targets with the target group, use <a>RegisterTargets</a>. To update the health check settings for the target group, use <a>ModifyTargetGroup</a>. To monitor the health of targets in the target group, use <a>DescribeTargetHealth</a>.</p> <p>To route traffic to the targets in a target group, specify the target group in an action using <a>CreateListener</a> or <a>CreateRule</a>.</p> <p>To delete a target group, use <a>DeleteTargetGroup</a>.</p> <p>This operation is idempotent, which means that it completes at most one time. If you attempt to create multiple target groups with the same settings, each call succeeds.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/elasticloadbalancing/latest/application/load-balancer-target-groups.html">Target Groups for Your Application Load Balancers</a> in the <i>Application Load Balancers Guide</i> or <a href="http://docs.aws.amazon.com/elasticloadbalancing/latest/network/load-balancer-target-groups.html">Target Groups for Your Network Load Balancers</a> in the <i>Network Load Balancers Guide</i>.</p>
    fn create_target_group(
        &self,
        input: CreateTargetGroupInput,
    ) -> RusotoFuture<CreateTargetGroupOutput, CreateTargetGroupError> {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CreateTargetGroup");
        params.put("Version", "2015-12-01");
        CreateTargetGroupInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateTargetGroupError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = CreateTargetGroupOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(CreateTargetGroupOutputDeserializer::deserialize(
                        "CreateTargetGroupResult",
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

    /// <p>Deletes the specified listener.</p> <p>Alternatively, your listener is deleted when you delete the load balancer it is attached to using <a>DeleteLoadBalancer</a>.</p>
    fn delete_listener(
        &self,
        input: DeleteListenerInput,
    ) -> RusotoFuture<DeleteListenerOutput, DeleteListenerError> {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteListener");
        params.put("Version", "2015-12-01");
        DeleteListenerInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteListenerError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DeleteListenerOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DeleteListenerOutputDeserializer::deserialize(
                        "DeleteListenerResult",
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

    /// <p>Deletes the specified Application Load Balancer or Network Load Balancer and its attached listeners.</p> <p>You can't delete a load balancer if deletion protection is enabled. If the load balancer does not exist or has already been deleted, the call succeeds.</p> <p>Deleting a load balancer does not affect its registered targets. For example, your EC2 instances continue to run and are still registered to their target groups. If you no longer need these EC2 instances, you can stop or terminate them.</p>
    fn delete_load_balancer(
        &self,
        input: DeleteLoadBalancerInput,
    ) -> RusotoFuture<DeleteLoadBalancerOutput, DeleteLoadBalancerError> {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteLoadBalancer");
        params.put("Version", "2015-12-01");
        DeleteLoadBalancerInputSerializer::serialize(&mut params, "", &input);
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
                    result = DeleteLoadBalancerOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DeleteLoadBalancerOutputDeserializer::deserialize(
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

    /// <p>Deletes the specified rule.</p>
    fn delete_rule(
        &self,
        input: DeleteRuleInput,
    ) -> RusotoFuture<DeleteRuleOutput, DeleteRuleError> {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteRule");
        params.put("Version", "2015-12-01");
        DeleteRuleInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteRuleError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DeleteRuleOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DeleteRuleOutputDeserializer::deserialize(
                        "DeleteRuleResult",
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

    /// <p>Deletes the specified target group.</p> <p>You can delete a target group if it is not referenced by any actions. Deleting a target group also deletes any associated health checks.</p>
    fn delete_target_group(
        &self,
        input: DeleteTargetGroupInput,
    ) -> RusotoFuture<DeleteTargetGroupOutput, DeleteTargetGroupError> {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteTargetGroup");
        params.put("Version", "2015-12-01");
        DeleteTargetGroupInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteTargetGroupError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DeleteTargetGroupOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DeleteTargetGroupOutputDeserializer::deserialize(
                        "DeleteTargetGroupResult",
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

    /// <p>Deregisters the specified targets from the specified target group. After the targets are deregistered, they no longer receive traffic from the load balancer.</p>
    fn deregister_targets(
        &self,
        input: DeregisterTargetsInput,
    ) -> RusotoFuture<DeregisterTargetsOutput, DeregisterTargetsError> {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeregisterTargets");
        params.put("Version", "2015-12-01");
        DeregisterTargetsInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeregisterTargetsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DeregisterTargetsOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DeregisterTargetsOutputDeserializer::deserialize(
                        "DeregisterTargetsResult",
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

    /// <p>Describes the current Elastic Load Balancing resource limits for your AWS account.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/elasticloadbalancing/latest/application/load-balancer-limits.html">Limits for Your Application Load Balancers</a> in the <i>Application Load Balancer Guide</i> or <a href="http://docs.aws.amazon.com/elasticloadbalancing/latest/network/load-balancer-limits.html">Limits for Your Network Load Balancers</a> in the <i>Network Load Balancers Guide</i>.</p>
    fn describe_account_limits(
        &self,
        input: DescribeAccountLimitsInput,
    ) -> RusotoFuture<DescribeAccountLimitsOutput, DescribeAccountLimitsError> {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeAccountLimits");
        params.put("Version", "2015-12-01");
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

    /// <p>Describes the certificates for the specified secure listener.</p>
    fn describe_listener_certificates(
        &self,
        input: DescribeListenerCertificatesInput,
    ) -> RusotoFuture<DescribeListenerCertificatesOutput, DescribeListenerCertificatesError> {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeListenerCertificates");
        params.put("Version", "2015-12-01");
        DescribeListenerCertificatesInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeListenerCertificatesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DescribeListenerCertificatesOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DescribeListenerCertificatesOutputDeserializer::deserialize(
                        "DescribeListenerCertificatesResult",
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

    /// <p>Describes the specified listeners or the listeners for the specified Application Load Balancer or Network Load Balancer. You must specify either a load balancer or one or more listeners.</p>
    fn describe_listeners(
        &self,
        input: DescribeListenersInput,
    ) -> RusotoFuture<DescribeListenersOutput, DescribeListenersError> {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeListeners");
        params.put("Version", "2015-12-01");
        DescribeListenersInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeListenersError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DescribeListenersOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DescribeListenersOutputDeserializer::deserialize(
                        "DescribeListenersResult",
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

    /// <p>Describes the attributes for the specified Application Load Balancer or Network Load Balancer.</p>
    fn describe_load_balancer_attributes(
        &self,
        input: DescribeLoadBalancerAttributesInput,
    ) -> RusotoFuture<DescribeLoadBalancerAttributesOutput, DescribeLoadBalancerAttributesError>
    {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeLoadBalancerAttributes");
        params.put("Version", "2015-12-01");
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

    /// <p>Describes the specified load balancers or all of your load balancers.</p> <p>To describe the listeners for a load balancer, use <a>DescribeListeners</a>. To describe the attributes for a load balancer, use <a>DescribeLoadBalancerAttributes</a>.</p>
    fn describe_load_balancers(
        &self,
        input: DescribeLoadBalancersInput,
    ) -> RusotoFuture<DescribeLoadBalancersOutput, DescribeLoadBalancersError> {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeLoadBalancers");
        params.put("Version", "2015-12-01");
        DescribeLoadBalancersInputSerializer::serialize(&mut params, "", &input);
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
                    result = DescribeLoadBalancersOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DescribeLoadBalancersOutputDeserializer::deserialize(
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

    /// <p>Describes the specified rules or the rules for the specified listener. You must specify either a listener or one or more rules.</p>
    fn describe_rules(
        &self,
        input: DescribeRulesInput,
    ) -> RusotoFuture<DescribeRulesOutput, DescribeRulesError> {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeRules");
        params.put("Version", "2015-12-01");
        DescribeRulesInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeRulesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DescribeRulesOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DescribeRulesOutputDeserializer::deserialize(
                        "DescribeRulesResult",
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

    /// <p>Describes the specified policies or all policies used for SSL negotiation.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/elasticloadbalancing/latest/application/create-https-listener.html#describe-ssl-policies">Security Policies</a> in the <i>Application Load Balancers Guide</i>.</p>
    fn describe_ssl_policies(
        &self,
        input: DescribeSSLPoliciesInput,
    ) -> RusotoFuture<DescribeSSLPoliciesOutput, DescribeSSLPoliciesError> {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeSSLPolicies");
        params.put("Version", "2015-12-01");
        DescribeSSLPoliciesInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeSSLPoliciesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DescribeSSLPoliciesOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DescribeSSLPoliciesOutputDeserializer::deserialize(
                        "DescribeSSLPoliciesResult",
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

    /// <p>Describes the tags for the specified resources. You can describe the tags for one or more Application Load Balancers, Network Load Balancers, and target groups.</p>
    fn describe_tags(
        &self,
        input: DescribeTagsInput,
    ) -> RusotoFuture<DescribeTagsOutput, DescribeTagsError> {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeTags");
        params.put("Version", "2015-12-01");
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

    /// <p>Describes the attributes for the specified target group.</p>
    fn describe_target_group_attributes(
        &self,
        input: DescribeTargetGroupAttributesInput,
    ) -> RusotoFuture<DescribeTargetGroupAttributesOutput, DescribeTargetGroupAttributesError> {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeTargetGroupAttributes");
        params.put("Version", "2015-12-01");
        DescribeTargetGroupAttributesInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeTargetGroupAttributesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DescribeTargetGroupAttributesOutput::default();
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
                        DescribeTargetGroupAttributesOutputDeserializer::deserialize(
                            "DescribeTargetGroupAttributesResult",
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

    /// <p>Describes the specified target groups or all of your target groups. By default, all target groups are described. Alternatively, you can specify one of the following to filter the results: the ARN of the load balancer, the names of one or more target groups, or the ARNs of one or more target groups.</p> <p>To describe the targets for a target group, use <a>DescribeTargetHealth</a>. To describe the attributes of a target group, use <a>DescribeTargetGroupAttributes</a>.</p>
    fn describe_target_groups(
        &self,
        input: DescribeTargetGroupsInput,
    ) -> RusotoFuture<DescribeTargetGroupsOutput, DescribeTargetGroupsError> {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeTargetGroups");
        params.put("Version", "2015-12-01");
        DescribeTargetGroupsInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeTargetGroupsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DescribeTargetGroupsOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DescribeTargetGroupsOutputDeserializer::deserialize(
                        "DescribeTargetGroupsResult",
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

    /// <p>Describes the health of the specified targets or all of your targets.</p>
    fn describe_target_health(
        &self,
        input: DescribeTargetHealthInput,
    ) -> RusotoFuture<DescribeTargetHealthOutput, DescribeTargetHealthError> {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeTargetHealth");
        params.put("Version", "2015-12-01");
        DescribeTargetHealthInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeTargetHealthError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DescribeTargetHealthOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DescribeTargetHealthOutputDeserializer::deserialize(
                        "DescribeTargetHealthResult",
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

    /// <p>Modifies the specified properties of the specified listener.</p> <p>Any properties that you do not specify retain their current values. However, changing the protocol from HTTPS to HTTP removes the security policy and SSL certificate properties. If you change the protocol from HTTP to HTTPS, you must add the security policy and server certificate.</p>
    fn modify_listener(
        &self,
        input: ModifyListenerInput,
    ) -> RusotoFuture<ModifyListenerOutput, ModifyListenerError> {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ModifyListener");
        params.put("Version", "2015-12-01");
        ModifyListenerInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ModifyListenerError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ModifyListenerOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(ModifyListenerOutputDeserializer::deserialize(
                        "ModifyListenerResult",
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

    /// <p>Modifies the specified attributes of the specified Application Load Balancer or Network Load Balancer.</p> <p>If any of the specified attributes can't be modified as requested, the call fails. Any existing attributes that you do not modify retain their current values.</p>
    fn modify_load_balancer_attributes(
        &self,
        input: ModifyLoadBalancerAttributesInput,
    ) -> RusotoFuture<ModifyLoadBalancerAttributesOutput, ModifyLoadBalancerAttributesError> {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ModifyLoadBalancerAttributes");
        params.put("Version", "2015-12-01");
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

    /// <p>Modifies the specified rule.</p> <p>Any existing properties that you do not modify retain their current values.</p> <p>To modify the default action, use <a>ModifyListener</a>.</p>
    fn modify_rule(
        &self,
        input: ModifyRuleInput,
    ) -> RusotoFuture<ModifyRuleOutput, ModifyRuleError> {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ModifyRule");
        params.put("Version", "2015-12-01");
        ModifyRuleInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ModifyRuleError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ModifyRuleOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(ModifyRuleOutputDeserializer::deserialize(
                        "ModifyRuleResult",
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

    /// <p>Modifies the health checks used when evaluating the health state of the targets in the specified target group.</p> <p>To monitor the health of the targets, use <a>DescribeTargetHealth</a>.</p>
    fn modify_target_group(
        &self,
        input: ModifyTargetGroupInput,
    ) -> RusotoFuture<ModifyTargetGroupOutput, ModifyTargetGroupError> {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ModifyTargetGroup");
        params.put("Version", "2015-12-01");
        ModifyTargetGroupInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ModifyTargetGroupError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ModifyTargetGroupOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(ModifyTargetGroupOutputDeserializer::deserialize(
                        "ModifyTargetGroupResult",
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

    /// <p>Modifies the specified attributes of the specified target group.</p>
    fn modify_target_group_attributes(
        &self,
        input: ModifyTargetGroupAttributesInput,
    ) -> RusotoFuture<ModifyTargetGroupAttributesOutput, ModifyTargetGroupAttributesError> {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ModifyTargetGroupAttributes");
        params.put("Version", "2015-12-01");
        ModifyTargetGroupAttributesInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ModifyTargetGroupAttributesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ModifyTargetGroupAttributesOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(ModifyTargetGroupAttributesOutputDeserializer::deserialize(
                        "ModifyTargetGroupAttributesResult",
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

    /// <p>Registers the specified targets with the specified target group.</p> <p>You can register targets by instance ID or by IP address. If the target is an EC2 instance, it must be in the <code>running</code> state when you register it.</p> <p>By default, the load balancer routes requests to registered targets using the protocol and port for the target group. Alternatively, you can override the port for a target when you register it. You can register each EC2 instance or IP address with the same target group multiple times using different ports.</p> <p>With a Network Load Balancer, you cannot register instances by instance ID if they have the following instance types: C1, CC1, CC2, CG1, CG2, CR1, CS1, G1, G2, HI1, HS1, M1, M2, M3, and T1. You can register instances of these types by IP address.</p> <p>To remove a target from a target group, use <a>DeregisterTargets</a>.</p>
    fn register_targets(
        &self,
        input: RegisterTargetsInput,
    ) -> RusotoFuture<RegisterTargetsOutput, RegisterTargetsError> {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "RegisterTargets");
        params.put("Version", "2015-12-01");
        RegisterTargetsInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(RegisterTargetsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = RegisterTargetsOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(RegisterTargetsOutputDeserializer::deserialize(
                        "RegisterTargetsResult",
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

    /// <p>Removes the specified certificate from the specified secure listener.</p> <p>You can't remove the default certificate for a listener. To replace the default certificate, call <a>ModifyListener</a>.</p> <p>To list the certificates for your listener, use <a>DescribeListenerCertificates</a>.</p>
    fn remove_listener_certificates(
        &self,
        input: RemoveListenerCertificatesInput,
    ) -> RusotoFuture<RemoveListenerCertificatesOutput, RemoveListenerCertificatesError> {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "RemoveListenerCertificates");
        params.put("Version", "2015-12-01");
        RemoveListenerCertificatesInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(RemoveListenerCertificatesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = RemoveListenerCertificatesOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(RemoveListenerCertificatesOutputDeserializer::deserialize(
                        "RemoveListenerCertificatesResult",
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

    /// <p>Removes the specified tags from the specified Elastic Load Balancing resource.</p> <p>To list the current tags for your resources, use <a>DescribeTags</a>.</p>
    fn remove_tags(
        &self,
        input: RemoveTagsInput,
    ) -> RusotoFuture<RemoveTagsOutput, RemoveTagsError> {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "RemoveTags");
        params.put("Version", "2015-12-01");
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

    /// <p>Sets the type of IP addresses used by the subnets of the specified Application Load Balancer or Network Load Balancer.</p> <p>Note that Network Load Balancers must use <code>ipv4</code>.</p>
    fn set_ip_address_type(
        &self,
        input: SetIpAddressTypeInput,
    ) -> RusotoFuture<SetIpAddressTypeOutput, SetIpAddressTypeError> {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "SetIpAddressType");
        params.put("Version", "2015-12-01");
        SetIpAddressTypeInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(SetIpAddressTypeError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = SetIpAddressTypeOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(SetIpAddressTypeOutputDeserializer::deserialize(
                        "SetIpAddressTypeResult",
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

    /// <p>Sets the priorities of the specified rules.</p> <p>You can reorder the rules as long as there are no priority conflicts in the new order. Any existing rules that you do not specify retain their current priority.</p>
    fn set_rule_priorities(
        &self,
        input: SetRulePrioritiesInput,
    ) -> RusotoFuture<SetRulePrioritiesOutput, SetRulePrioritiesError> {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "SetRulePriorities");
        params.put("Version", "2015-12-01");
        SetRulePrioritiesInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(SetRulePrioritiesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = SetRulePrioritiesOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(SetRulePrioritiesOutputDeserializer::deserialize(
                        "SetRulePrioritiesResult",
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

    /// <p>Associates the specified security groups with the specified Application Load Balancer. The specified security groups override the previously associated security groups.</p> <p>Note that you can't specify a security group for a Network Load Balancer.</p>
    fn set_security_groups(
        &self,
        input: SetSecurityGroupsInput,
    ) -> RusotoFuture<SetSecurityGroupsOutput, SetSecurityGroupsError> {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "SetSecurityGroups");
        params.put("Version", "2015-12-01");
        SetSecurityGroupsInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(SetSecurityGroupsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = SetSecurityGroupsOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(SetSecurityGroupsOutputDeserializer::deserialize(
                        "SetSecurityGroupsResult",
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

    /// <p>Enables the Availability Zone for the specified public subnets for the specified Application Load Balancer. The specified subnets replace the previously enabled subnets.</p> <p>Note that you can't change the subnets for a Network Load Balancer.</p>
    fn set_subnets(
        &self,
        input: SetSubnetsInput,
    ) -> RusotoFuture<SetSubnetsOutput, SetSubnetsError> {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "SetSubnets");
        params.put("Version", "2015-12-01");
        SetSubnetsInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(SetSubnetsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = SetSubnetsOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(SetSubnetsOutputDeserializer::deserialize(
                        "SetSubnetsResult",
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
    fn test_parse_error_elb_describe_load_balancers() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/error",
            "elb-describe-load-balancers.xml",
        );
        let mock = MockRequestDispatcher::with_status(400).with_body(&mock_response);
        let client = ElbClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = DescribeLoadBalancersInput::default();
        let result = client.describe_load_balancers(request).sync();
        assert!(!result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_elb_describe_load_balancers() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "elb-describe-load-balancers.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = ElbClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = DescribeLoadBalancersInput::default();
        let result = client.describe_load_balancers(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }
}
