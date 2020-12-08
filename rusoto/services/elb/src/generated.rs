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
    self as xml_util, deserialize_elements, find_start_element, skip_tree,
};
use rusoto_core::proto::xml::util::{Next, Peek, XmlParseError, XmlResponse};
use rusoto_core::request::HttpResponse;
use rusoto_core::signature::SignedRequest;
#[cfg(feature = "deserialize_structs")]
use serde::Deserialize;
#[cfg(feature = "serialize_structs")]
use serde::Serialize;
use serde_urlencoded;
use std::str::FromStr;
use xml::EventReader;

impl ElbClient {
    fn new_params(&self, operation_name: &str) -> Params {
        let mut params = Params::new();

        params.put("Action", operation_name);
        params.put("Version", "2012-06-01");

        params
    }

    async fn sign_and_dispatch(
        &self,
        request: SignedRequest,
    ) -> Result<HttpResponse, RusotoError<std::convert::Infallible>> {
        let mut response = self.client.sign_and_dispatch(request).await?;
        if !response.status.is_success() {
            let response = response.buffer().await?;
            return Err(RusotoError::Unknown(response));
        }

        Ok(response)
    }
}

/// <p>Information about the <code>AccessLog</code> attribute.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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

#[allow(dead_code)]
struct AccessLogDeserializer;
impl AccessLogDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AccessLog, XmlParseError> {
        deserialize_elements::<_, AccessLog, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "EmitInterval" => {
                    obj.emit_interval = Some(AccessLogIntervalDeserializer::deserialize(
                        "EmitInterval",
                        stack,
                    )?);
                }
                "Enabled" => {
                    obj.enabled = AccessLogEnabledDeserializer::deserialize("Enabled", stack)?;
                }
                "S3BucketName" => {
                    obj.s3_bucket_name = Some(S3BucketNameDeserializer::deserialize(
                        "S3BucketName",
                        stack,
                    )?);
                }
                "S3BucketPrefix" => {
                    obj.s3_bucket_prefix = Some(AccessLogPrefixDeserializer::deserialize(
                        "S3BucketPrefix",
                        stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
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
            params.put(&format!("{}{}", prefix, "EmitInterval"), &field_value);
        }
        params.put(&format!("{}{}", prefix, "Enabled"), &obj.enabled);
        if let Some(ref field_value) = obj.s3_bucket_name {
            params.put(&format!("{}{}", prefix, "S3BucketName"), &field_value);
        }
        if let Some(ref field_value) = obj.s3_bucket_prefix {
            params.put(&format!("{}{}", prefix, "S3BucketPrefix"), &field_value);
        }
    }
}

#[allow(dead_code)]
struct AccessLogEnabledDeserializer;
impl AccessLogEnabledDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<bool, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, |s| Ok(bool::from_str(&s).unwrap()))
    }
}
#[allow(dead_code)]
struct AccessLogIntervalDeserializer;
impl AccessLogIntervalDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, |s| Ok(i64::from_str(&s).unwrap()))
    }
}
#[allow(dead_code)]
struct AccessLogPrefixDeserializer;
impl AccessLogPrefixDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}
#[allow(dead_code)]
struct AccessPointNameDeserializer;
impl AccessPointNameDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}
#[allow(dead_code)]
struct AccessPointPortDeserializer;
impl AccessPointPortDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, |s| Ok(i64::from_str(&s).unwrap()))
    }
}
/// <p>Contains the parameters for EnableAvailabilityZonesForLoadBalancer.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
            &obj.load_balancer_name,
        );
    }
}

/// <p>Contains the output of EnableAvailabilityZonesForLoadBalancer.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct AddAvailabilityZonesOutput {
    /// <p>The updated list of Availability Zones for the load balancer.</p>
    pub availability_zones: Option<Vec<String>>,
}

#[allow(dead_code)]
struct AddAvailabilityZonesOutputDeserializer;
impl AddAvailabilityZonesOutputDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AddAvailabilityZonesOutput, XmlParseError> {
        deserialize_elements::<_, AddAvailabilityZonesOutput, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "AvailabilityZones" => {
                        obj.availability_zones.get_or_insert(vec![]).extend(
                            AvailabilityZonesDeserializer::deserialize("AvailabilityZones", stack)?,
                        );
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p>Contains the parameters for AddTags.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct AddTagsOutput {}

#[allow(dead_code)]
struct AddTagsOutputDeserializer;
impl AddTagsOutputDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AddTagsOutput, XmlParseError> {
        xml_util::start_element(tag_name, stack)?;

        let obj = AddTagsOutput::default();

        xml_util::end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Information about additional load balancer attributes.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AdditionalAttribute {
    /// <p><p>The name of the attribute.</p> <p>The following attribute is supported.</p> <ul> <li> <p> <code>elb.http.desyncmitigationmode</code> - Determines how the load balancer handles requests that might pose a security risk to your application. The possible values are <code>monitor</code>, <code>defensive</code>, and <code>strictest</code>. The default is <code>defensive</code>.</p> </li> </ul></p>
    pub key: Option<String>,
    /// <p>This value of the attribute.</p>
    pub value: Option<String>,
}

#[allow(dead_code)]
struct AdditionalAttributeDeserializer;
impl AdditionalAttributeDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AdditionalAttribute, XmlParseError> {
        deserialize_elements::<_, AdditionalAttribute, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Key" => {
                    obj.key = Some(AdditionalAttributeKeyDeserializer::deserialize(
                        "Key", stack,
                    )?);
                }
                "Value" => {
                    obj.value = Some(AdditionalAttributeValueDeserializer::deserialize(
                        "Value", stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
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
            params.put(&format!("{}{}", prefix, "Key"), &field_value);
        }
        if let Some(ref field_value) = obj.value {
            params.put(&format!("{}{}", prefix, "Value"), &field_value);
        }
    }
}

#[allow(dead_code)]
struct AdditionalAttributeKeyDeserializer;
impl AdditionalAttributeKeyDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}
#[allow(dead_code)]
struct AdditionalAttributeValueDeserializer;
impl AdditionalAttributeValueDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}
#[allow(dead_code)]
struct AdditionalAttributesDeserializer;
impl AdditionalAttributesDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<AdditionalAttribute>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(AdditionalAttributeDeserializer::deserialize(
                    "member", stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
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

#[allow(dead_code)]
struct AppCookieStickinessPoliciesDeserializer;
impl AppCookieStickinessPoliciesDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<AppCookieStickinessPolicy>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(AppCookieStickinessPolicyDeserializer::deserialize(
                    "member", stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
/// <p>Information about a policy for application-controlled session stickiness.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct AppCookieStickinessPolicy {
    /// <p>The name of the application cookie used for stickiness.</p>
    pub cookie_name: Option<String>,
    /// <p>The mnemonic name for the policy being created. The name must be unique within a set of policies for this load balancer.</p>
    pub policy_name: Option<String>,
}

#[allow(dead_code)]
struct AppCookieStickinessPolicyDeserializer;
impl AppCookieStickinessPolicyDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AppCookieStickinessPolicy, XmlParseError> {
        deserialize_elements::<_, AppCookieStickinessPolicy, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "CookieName" => {
                        obj.cookie_name =
                            Some(CookieNameDeserializer::deserialize("CookieName", stack)?);
                    }
                    "PolicyName" => {
                        obj.policy_name =
                            Some(PolicyNameDeserializer::deserialize("PolicyName", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p>Contains the parameters for ApplySecurityGroupsToLoadBalancer.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
            &obj.load_balancer_name,
        );
        SecurityGroupsSerializer::serialize(
            params,
            &format!("{}{}", prefix, "SecurityGroups"),
            &obj.security_groups,
        );
    }
}

/// <p>Contains the output of ApplySecurityGroupsToLoadBalancer.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ApplySecurityGroupsToLoadBalancerOutput {
    /// <p>The IDs of the security groups associated with the load balancer.</p>
    pub security_groups: Option<Vec<String>>,
}

#[allow(dead_code)]
struct ApplySecurityGroupsToLoadBalancerOutputDeserializer;
impl ApplySecurityGroupsToLoadBalancerOutputDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ApplySecurityGroupsToLoadBalancerOutput, XmlParseError> {
        deserialize_elements::<_, ApplySecurityGroupsToLoadBalancerOutput, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "SecurityGroups" => {
                        obj.security_groups.get_or_insert(vec![]).extend(
                            SecurityGroupsDeserializer::deserialize("SecurityGroups", stack)?,
                        );
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p>Contains the parameters for AttachLoaBalancerToSubnets.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
            &obj.load_balancer_name,
        );
        SubnetsSerializer::serialize(params, &format!("{}{}", prefix, "Subnets"), &obj.subnets);
    }
}

/// <p>Contains the output of AttachLoadBalancerToSubnets.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct AttachLoadBalancerToSubnetsOutput {
    /// <p>The IDs of the subnets attached to the load balancer.</p>
    pub subnets: Option<Vec<String>>,
}

#[allow(dead_code)]
struct AttachLoadBalancerToSubnetsOutputDeserializer;
impl AttachLoadBalancerToSubnetsOutputDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AttachLoadBalancerToSubnetsOutput, XmlParseError> {
        deserialize_elements::<_, AttachLoadBalancerToSubnetsOutput, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Subnets" => {
                        obj.subnets
                            .get_or_insert(vec![])
                            .extend(SubnetsDeserializer::deserialize("Subnets", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
#[allow(dead_code)]
struct AttributeNameDeserializer;
impl AttributeNameDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}
#[allow(dead_code)]
struct AttributeTypeDeserializer;
impl AttributeTypeDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}
#[allow(dead_code)]
struct AttributeValueDeserializer;
impl AttributeValueDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}
#[allow(dead_code)]
struct AvailabilityZoneDeserializer;
impl AvailabilityZoneDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}
#[allow(dead_code)]
struct AvailabilityZonesDeserializer;
impl AvailabilityZonesDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(AvailabilityZoneDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
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
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct BackendServerDescription {
    /// <p>The port on which the EC2 instance is listening.</p>
    pub instance_port: Option<i64>,
    /// <p>The names of the policies enabled for the EC2 instance.</p>
    pub policy_names: Option<Vec<String>>,
}

#[allow(dead_code)]
struct BackendServerDescriptionDeserializer;
impl BackendServerDescriptionDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<BackendServerDescription, XmlParseError> {
        deserialize_elements::<_, BackendServerDescription, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "InstancePort" => {
                        obj.instance_port = Some(InstancePortDeserializer::deserialize(
                            "InstancePort",
                            stack,
                        )?);
                    }
                    "PolicyNames" => {
                        obj.policy_names
                            .get_or_insert(vec![])
                            .extend(PolicyNamesDeserializer::deserialize("PolicyNames", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
#[allow(dead_code)]
struct BackendServerDescriptionsDeserializer;
impl BackendServerDescriptionsDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<BackendServerDescription>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(BackendServerDescriptionDeserializer::deserialize(
                    "member", stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
#[allow(dead_code)]
struct CardinalityDeserializer;
impl CardinalityDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}
/// <p>Contains the parameters for ConfigureHealthCheck.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
            &obj.load_balancer_name,
        );
    }
}

/// <p>Contains the output of ConfigureHealthCheck.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ConfigureHealthCheckOutput {
    /// <p>The updated health check.</p>
    pub health_check: Option<HealthCheck>,
}

#[allow(dead_code)]
struct ConfigureHealthCheckOutputDeserializer;
impl ConfigureHealthCheckOutputDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ConfigureHealthCheckOutput, XmlParseError> {
        deserialize_elements::<_, ConfigureHealthCheckOutput, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "HealthCheck" => {
                        obj.health_check =
                            Some(HealthCheckDeserializer::deserialize("HealthCheck", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p>Information about the <code>ConnectionDraining</code> attribute.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ConnectionDraining {
    /// <p>Specifies whether connection draining is enabled for the load balancer.</p>
    pub enabled: bool,
    /// <p>The maximum time, in seconds, to keep the existing connections open before deregistering the instances.</p>
    pub timeout: Option<i64>,
}

#[allow(dead_code)]
struct ConnectionDrainingDeserializer;
impl ConnectionDrainingDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ConnectionDraining, XmlParseError> {
        deserialize_elements::<_, ConnectionDraining, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Enabled" => {
                    obj.enabled =
                        ConnectionDrainingEnabledDeserializer::deserialize("Enabled", stack)?;
                }
                "Timeout" => {
                    obj.timeout = Some(ConnectionDrainingTimeoutDeserializer::deserialize(
                        "Timeout", stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
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

        params.put(&format!("{}{}", prefix, "Enabled"), &obj.enabled);
        if let Some(ref field_value) = obj.timeout {
            params.put(&format!("{}{}", prefix, "Timeout"), &field_value);
        }
    }
}

#[allow(dead_code)]
struct ConnectionDrainingEnabledDeserializer;
impl ConnectionDrainingEnabledDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<bool, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, |s| Ok(bool::from_str(&s).unwrap()))
    }
}
#[allow(dead_code)]
struct ConnectionDrainingTimeoutDeserializer;
impl ConnectionDrainingTimeoutDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, |s| Ok(i64::from_str(&s).unwrap()))
    }
}
/// <p>Information about the <code>ConnectionSettings</code> attribute.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ConnectionSettings {
    /// <p>The time, in seconds, that the connection is allowed to be idle (no data has been sent over the connection) before it is closed by the load balancer.</p>
    pub idle_timeout: i64,
}

#[allow(dead_code)]
struct ConnectionSettingsDeserializer;
impl ConnectionSettingsDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ConnectionSettings, XmlParseError> {
        deserialize_elements::<_, ConnectionSettings, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "IdleTimeout" => {
                    obj.idle_timeout = IdleTimeoutDeserializer::deserialize("IdleTimeout", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
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

        params.put(&format!("{}{}", prefix, "IdleTimeout"), &obj.idle_timeout);
    }
}

#[allow(dead_code)]
struct CookieExpirationPeriodDeserializer;
impl CookieExpirationPeriodDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, |s| Ok(i64::from_str(&s).unwrap()))
    }
}
#[allow(dead_code)]
struct CookieNameDeserializer;
impl CookieNameDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}
/// <p>Contains the parameters for CreateLoadBalancer.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateAccessPointInput {
    /// <p>One or more Availability Zones from the same region as the load balancer.</p> <p>You must specify at least one Availability Zone.</p> <p>You can add more Availability Zones after you create the load balancer using <a>EnableAvailabilityZonesForLoadBalancer</a>.</p>
    pub availability_zones: Option<Vec<String>>,
    /// <p>The listeners.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/classic/elb-listener-config.html">Listeners for Your Classic Load Balancer</a> in the <i>Classic Load Balancers Guide</i>.</p>
    pub listeners: Vec<Listener>,
    /// <p>The name of the load balancer.</p> <p>This name must be unique within your set of load balancers for the region, must have a maximum of 32 characters, must contain only alphanumeric characters or hyphens, and cannot begin or end with a hyphen.</p>
    pub load_balancer_name: String,
    /// <p>The type of a load balancer. Valid only for load balancers in a VPC.</p> <p>By default, Elastic Load Balancing creates an Internet-facing load balancer with a DNS name that resolves to public IP addresses. For more information about Internet-facing and Internal load balancers, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/userguide/how-elastic-load-balancing-works.html#load-balancer-scheme">Load Balancer Scheme</a> in the <i>Elastic Load Balancing User Guide</i>.</p> <p>Specify <code>internal</code> to create a load balancer with a DNS name that resolves to private IP addresses.</p>
    pub scheme: Option<String>,
    /// <p>The IDs of the security groups to assign to the load balancer.</p>
    pub security_groups: Option<Vec<String>>,
    /// <p>The IDs of the subnets in your VPC to attach to the load balancer. Specify one subnet per Availability Zone specified in <code>AvailabilityZones</code>.</p>
    pub subnets: Option<Vec<String>>,
    /// <p>A list of tags to assign to the load balancer.</p> <p>For more information about tagging your load balancer, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/classic/add-remove-tags.html">Tag Your Classic Load Balancer</a> in the <i>Classic Load Balancers Guide</i>.</p>
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
            &obj.load_balancer_name,
        );
        if let Some(ref field_value) = obj.scheme {
            params.put(&format!("{}{}", prefix, "Scheme"), &field_value);
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
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CreateAccessPointOutput {
    /// <p>The DNS name of the load balancer.</p>
    pub dns_name: Option<String>,
}

#[allow(dead_code)]
struct CreateAccessPointOutputDeserializer;
impl CreateAccessPointOutputDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateAccessPointOutput, XmlParseError> {
        deserialize_elements::<_, CreateAccessPointOutput, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "DNSName" => {
                        obj.dns_name = Some(DNSNameDeserializer::deserialize("DNSName", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p>Contains the parameters for CreateAppCookieStickinessPolicy.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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

        params.put(&format!("{}{}", prefix, "CookieName"), &obj.cookie_name);
        params.put(
            &format!("{}{}", prefix, "LoadBalancerName"),
            &obj.load_balancer_name,
        );
        params.put(&format!("{}{}", prefix, "PolicyName"), &obj.policy_name);
    }
}

/// <p>Contains the output for CreateAppCookieStickinessPolicy.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CreateAppCookieStickinessPolicyOutput {}

#[allow(dead_code)]
struct CreateAppCookieStickinessPolicyOutputDeserializer;
impl CreateAppCookieStickinessPolicyOutputDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateAppCookieStickinessPolicyOutput, XmlParseError> {
        xml_util::start_element(tag_name, stack)?;

        let obj = CreateAppCookieStickinessPolicyOutput::default();

        xml_util::end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Contains the parameters for CreateLBCookieStickinessPolicy.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
                &field_value,
            );
        }
        params.put(
            &format!("{}{}", prefix, "LoadBalancerName"),
            &obj.load_balancer_name,
        );
        params.put(&format!("{}{}", prefix, "PolicyName"), &obj.policy_name);
    }
}

/// <p>Contains the output for CreateLBCookieStickinessPolicy.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CreateLBCookieStickinessPolicyOutput {}

#[allow(dead_code)]
struct CreateLBCookieStickinessPolicyOutputDeserializer;
impl CreateLBCookieStickinessPolicyOutputDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateLBCookieStickinessPolicyOutput, XmlParseError> {
        xml_util::start_element(tag_name, stack)?;

        let obj = CreateLBCookieStickinessPolicyOutput::default();

        xml_util::end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Contains the parameters for CreateLoadBalancerListeners.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
            &obj.load_balancer_name,
        );
    }
}

/// <p>Contains the parameters for CreateLoadBalancerListener.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CreateLoadBalancerListenerOutput {}

#[allow(dead_code)]
struct CreateLoadBalancerListenerOutputDeserializer;
impl CreateLoadBalancerListenerOutputDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateLoadBalancerListenerOutput, XmlParseError> {
        xml_util::start_element(tag_name, stack)?;

        let obj = CreateLoadBalancerListenerOutput::default();

        xml_util::end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Contains the parameters for CreateLoadBalancerPolicy.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
            &obj.load_balancer_name,
        );
        if let Some(ref field_value) = obj.policy_attributes {
            PolicyAttributesSerializer::serialize(
                params,
                &format!("{}{}", prefix, "PolicyAttributes"),
                field_value,
            );
        }
        params.put(&format!("{}{}", prefix, "PolicyName"), &obj.policy_name);
        params.put(
            &format!("{}{}", prefix, "PolicyTypeName"),
            &obj.policy_type_name,
        );
    }
}

/// <p>Contains the output of CreateLoadBalancerPolicy.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CreateLoadBalancerPolicyOutput {}

#[allow(dead_code)]
struct CreateLoadBalancerPolicyOutputDeserializer;
impl CreateLoadBalancerPolicyOutputDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateLoadBalancerPolicyOutput, XmlParseError> {
        xml_util::start_element(tag_name, stack)?;

        let obj = CreateLoadBalancerPolicyOutput::default();

        xml_util::end_element(tag_name, stack)?;

        Ok(obj)
    }
}
#[allow(dead_code)]
struct CreatedTimeDeserializer;
impl CreatedTimeDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}
/// <p>Information about the <code>CrossZoneLoadBalancing</code> attribute.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CrossZoneLoadBalancing {
    /// <p>Specifies whether cross-zone load balancing is enabled for the load balancer.</p>
    pub enabled: bool,
}

#[allow(dead_code)]
struct CrossZoneLoadBalancingDeserializer;
impl CrossZoneLoadBalancingDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CrossZoneLoadBalancing, XmlParseError> {
        deserialize_elements::<_, CrossZoneLoadBalancing, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Enabled" => {
                    obj.enabled =
                        CrossZoneLoadBalancingEnabledDeserializer::deserialize("Enabled", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
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

        params.put(&format!("{}{}", prefix, "Enabled"), &obj.enabled);
    }
}

#[allow(dead_code)]
struct CrossZoneLoadBalancingEnabledDeserializer;
impl CrossZoneLoadBalancingEnabledDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<bool, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, |s| Ok(bool::from_str(&s).unwrap()))
    }
}
#[allow(dead_code)]
struct DNSNameDeserializer;
impl DNSNameDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}
#[allow(dead_code)]
struct DefaultValueDeserializer;
impl DefaultValueDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}
/// <p>Contains the parameters for DeleteLoadBalancer.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
            &obj.load_balancer_name,
        );
    }
}

/// <p>Contains the output of DeleteLoadBalancer.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DeleteAccessPointOutput {}

#[allow(dead_code)]
struct DeleteAccessPointOutputDeserializer;
impl DeleteAccessPointOutputDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteAccessPointOutput, XmlParseError> {
        xml_util::start_element(tag_name, stack)?;

        let obj = DeleteAccessPointOutput::default();

        xml_util::end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Contains the parameters for DeleteLoadBalancerListeners.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
            &obj.load_balancer_name,
        );
        PortsSerializer::serialize(
            params,
            &format!("{}{}", prefix, "LoadBalancerPorts"),
            &obj.load_balancer_ports,
        );
    }
}

/// <p>Contains the output of DeleteLoadBalancerListeners.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DeleteLoadBalancerListenerOutput {}

#[allow(dead_code)]
struct DeleteLoadBalancerListenerOutputDeserializer;
impl DeleteLoadBalancerListenerOutputDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteLoadBalancerListenerOutput, XmlParseError> {
        xml_util::start_element(tag_name, stack)?;

        let obj = DeleteLoadBalancerListenerOutput::default();

        xml_util::end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Contains the parameters for DeleteLoadBalancerPolicy.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
            &obj.load_balancer_name,
        );
        params.put(&format!("{}{}", prefix, "PolicyName"), &obj.policy_name);
    }
}

/// <p>Contains the output of DeleteLoadBalancerPolicy.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DeleteLoadBalancerPolicyOutput {}

#[allow(dead_code)]
struct DeleteLoadBalancerPolicyOutputDeserializer;
impl DeleteLoadBalancerPolicyOutputDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteLoadBalancerPolicyOutput, XmlParseError> {
        xml_util::start_element(tag_name, stack)?;

        let obj = DeleteLoadBalancerPolicyOutput::default();

        xml_util::end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Contains the parameters for DeregisterInstancesFromLoadBalancer.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
            &obj.load_balancer_name,
        );
    }
}

/// <p>Contains the output of DeregisterInstancesFromLoadBalancer.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DeregisterEndPointsOutput {
    /// <p>The remaining instances registered with the load balancer.</p>
    pub instances: Option<Vec<Instance>>,
}

#[allow(dead_code)]
struct DeregisterEndPointsOutputDeserializer;
impl DeregisterEndPointsOutputDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeregisterEndPointsOutput, XmlParseError> {
        deserialize_elements::<_, DeregisterEndPointsOutput, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Instances" => {
                        obj.instances
                            .get_or_insert(vec![])
                            .extend(InstancesDeserializer::deserialize("Instances", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p>Contains the parameters for DescribeLoadBalancers.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
            params.put(&format!("{}{}", prefix, "Marker"), &field_value);
        }
        if let Some(ref field_value) = obj.page_size {
            params.put(&format!("{}{}", prefix, "PageSize"), &field_value);
        }
    }
}

/// <p>Contains the parameters for DescribeLoadBalancers.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DescribeAccessPointsOutput {
    /// <p>Information about the load balancers.</p>
    pub load_balancer_descriptions: Option<Vec<LoadBalancerDescription>>,
    /// <p>The marker to use when requesting the next set of results. If there are no additional results, the string is empty.</p>
    pub next_marker: Option<String>,
}

#[allow(dead_code)]
struct DescribeAccessPointsOutputDeserializer;
impl DescribeAccessPointsOutputDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeAccessPointsOutput, XmlParseError> {
        deserialize_elements::<_, DescribeAccessPointsOutput, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "LoadBalancerDescriptions" => {
                        obj.load_balancer_descriptions.get_or_insert(vec![]).extend(
                            LoadBalancerDescriptionsDeserializer::deserialize(
                                "LoadBalancerDescriptions",
                                stack,
                            )?,
                        );
                    }
                    "NextMarker" => {
                        obj.next_marker =
                            Some(MarkerDeserializer::deserialize("NextMarker", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
            params.put(&format!("{}{}", prefix, "Marker"), &field_value);
        }
        if let Some(ref field_value) = obj.page_size {
            params.put(&format!("{}{}", prefix, "PageSize"), &field_value);
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DescribeAccountLimitsOutput {
    /// <p>Information about the limits.</p>
    pub limits: Option<Vec<Limit>>,
    /// <p>The marker to use when requesting the next set of results. If there are no additional results, the string is empty.</p>
    pub next_marker: Option<String>,
}

#[allow(dead_code)]
struct DescribeAccountLimitsOutputDeserializer;
impl DescribeAccountLimitsOutputDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeAccountLimitsOutput, XmlParseError> {
        deserialize_elements::<_, DescribeAccountLimitsOutput, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Limits" => {
                        obj.limits
                            .get_or_insert(vec![])
                            .extend(LimitsDeserializer::deserialize("Limits", stack)?);
                    }
                    "NextMarker" => {
                        obj.next_marker =
                            Some(MarkerDeserializer::deserialize("NextMarker", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p>Contains the parameters for DescribeInstanceHealth.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
            &obj.load_balancer_name,
        );
    }
}

/// <p>Contains the output for DescribeInstanceHealth.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DescribeEndPointStateOutput {
    /// <p>Information about the health of the instances.</p>
    pub instance_states: Option<Vec<InstanceState>>,
}

#[allow(dead_code)]
struct DescribeEndPointStateOutputDeserializer;
impl DescribeEndPointStateOutputDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeEndPointStateOutput, XmlParseError> {
        deserialize_elements::<_, DescribeEndPointStateOutput, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "InstanceStates" => {
                        obj.instance_states.get_or_insert(vec![]).extend(
                            InstanceStatesDeserializer::deserialize("InstanceStates", stack)?,
                        );
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p>Contains the parameters for DescribeLoadBalancerAttributes.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
            &obj.load_balancer_name,
        );
    }
}

/// <p>Contains the output of DescribeLoadBalancerAttributes.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DescribeLoadBalancerAttributesOutput {
    /// <p>Information about the load balancer attributes.</p>
    pub load_balancer_attributes: Option<LoadBalancerAttributes>,
}

#[allow(dead_code)]
struct DescribeLoadBalancerAttributesOutputDeserializer;
impl DescribeLoadBalancerAttributesOutputDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeLoadBalancerAttributesOutput, XmlParseError> {
        deserialize_elements::<_, DescribeLoadBalancerAttributesOutput, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "LoadBalancerAttributes" => {
                        obj.load_balancer_attributes =
                            Some(LoadBalancerAttributesDeserializer::deserialize(
                                "LoadBalancerAttributes",
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
/// <p>Contains the parameters for DescribeLoadBalancerPolicies.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
            params.put(&format!("{}{}", prefix, "LoadBalancerName"), &field_value);
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
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DescribeLoadBalancerPoliciesOutput {
    /// <p>Information about the policies.</p>
    pub policy_descriptions: Option<Vec<PolicyDescription>>,
}

#[allow(dead_code)]
struct DescribeLoadBalancerPoliciesOutputDeserializer;
impl DescribeLoadBalancerPoliciesOutputDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeLoadBalancerPoliciesOutput, XmlParseError> {
        deserialize_elements::<_, DescribeLoadBalancerPoliciesOutput, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "PolicyDescriptions" => {
                        obj.policy_descriptions.get_or_insert(vec![]).extend(
                            PolicyDescriptionsDeserializer::deserialize(
                                "PolicyDescriptions",
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
/// <p>Contains the parameters for DescribeLoadBalancerPolicyTypes.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DescribeLoadBalancerPolicyTypesOutput {
    /// <p>Information about the policy types.</p>
    pub policy_type_descriptions: Option<Vec<PolicyTypeDescription>>,
}

#[allow(dead_code)]
struct DescribeLoadBalancerPolicyTypesOutputDeserializer;
impl DescribeLoadBalancerPolicyTypesOutputDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeLoadBalancerPolicyTypesOutput, XmlParseError> {
        deserialize_elements::<_, DescribeLoadBalancerPolicyTypesOutput, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "PolicyTypeDescriptions" => {
                        obj.policy_type_descriptions.get_or_insert(vec![]).extend(
                            PolicyTypeDescriptionsDeserializer::deserialize(
                                "PolicyTypeDescriptions",
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
/// <p>Contains the parameters for DescribeTags.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DescribeTagsOutput {
    /// <p>Information about the tags.</p>
    pub tag_descriptions: Option<Vec<TagDescription>>,
}

#[allow(dead_code)]
struct DescribeTagsOutputDeserializer;
impl DescribeTagsOutputDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeTagsOutput, XmlParseError> {
        deserialize_elements::<_, DescribeTagsOutput, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "TagDescriptions" => {
                    obj.tag_descriptions.get_or_insert(vec![]).extend(
                        TagDescriptionsDeserializer::deserialize("TagDescriptions", stack)?,
                    );
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
#[allow(dead_code)]
struct DescriptionDeserializer;
impl DescriptionDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}
/// <p>Contains the parameters for DetachLoadBalancerFromSubnets.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
            &obj.load_balancer_name,
        );
        SubnetsSerializer::serialize(params, &format!("{}{}", prefix, "Subnets"), &obj.subnets);
    }
}

/// <p>Contains the output of DetachLoadBalancerFromSubnets.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DetachLoadBalancerFromSubnetsOutput {
    /// <p>The IDs of the remaining subnets for the load balancer.</p>
    pub subnets: Option<Vec<String>>,
}

#[allow(dead_code)]
struct DetachLoadBalancerFromSubnetsOutputDeserializer;
impl DetachLoadBalancerFromSubnetsOutputDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DetachLoadBalancerFromSubnetsOutput, XmlParseError> {
        deserialize_elements::<_, DetachLoadBalancerFromSubnetsOutput, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Subnets" => {
                        obj.subnets
                            .get_or_insert(vec![])
                            .extend(SubnetsDeserializer::deserialize("Subnets", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p>Information about a health check.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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

#[allow(dead_code)]
struct HealthCheckDeserializer;
impl HealthCheckDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<HealthCheck, XmlParseError> {
        deserialize_elements::<_, HealthCheck, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "HealthyThreshold" => {
                    obj.healthy_threshold =
                        HealthyThresholdDeserializer::deserialize("HealthyThreshold", stack)?;
                }
                "Interval" => {
                    obj.interval = HealthCheckIntervalDeserializer::deserialize("Interval", stack)?;
                }
                "Target" => {
                    obj.target = HealthCheckTargetDeserializer::deserialize("Target", stack)?;
                }
                "Timeout" => {
                    obj.timeout = HealthCheckTimeoutDeserializer::deserialize("Timeout", stack)?;
                }
                "UnhealthyThreshold" => {
                    obj.unhealthy_threshold =
                        UnhealthyThresholdDeserializer::deserialize("UnhealthyThreshold", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
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
            &obj.healthy_threshold,
        );
        params.put(&format!("{}{}", prefix, "Interval"), &obj.interval);
        params.put(&format!("{}{}", prefix, "Target"), &obj.target);
        params.put(&format!("{}{}", prefix, "Timeout"), &obj.timeout);
        params.put(
            &format!("{}{}", prefix, "UnhealthyThreshold"),
            &obj.unhealthy_threshold,
        );
    }
}

#[allow(dead_code)]
struct HealthCheckIntervalDeserializer;
impl HealthCheckIntervalDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, |s| Ok(i64::from_str(&s).unwrap()))
    }
}
#[allow(dead_code)]
struct HealthCheckTargetDeserializer;
impl HealthCheckTargetDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}
#[allow(dead_code)]
struct HealthCheckTimeoutDeserializer;
impl HealthCheckTimeoutDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, |s| Ok(i64::from_str(&s).unwrap()))
    }
}
#[allow(dead_code)]
struct HealthyThresholdDeserializer;
impl HealthyThresholdDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, |s| Ok(i64::from_str(&s).unwrap()))
    }
}
#[allow(dead_code)]
struct IdleTimeoutDeserializer;
impl IdleTimeoutDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, |s| Ok(i64::from_str(&s).unwrap()))
    }
}
/// <p>The ID of an EC2 instance.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Instance {
    /// <p>The instance ID.</p>
    pub instance_id: Option<String>,
}

#[allow(dead_code)]
struct InstanceDeserializer;
impl InstanceDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Instance, XmlParseError> {
        deserialize_elements::<_, Instance, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "InstanceId" => {
                    obj.instance_id =
                        Some(InstanceIdDeserializer::deserialize("InstanceId", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
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
            params.put(&format!("{}{}", prefix, "InstanceId"), &field_value);
        }
    }
}

#[allow(dead_code)]
struct InstanceIdDeserializer;
impl InstanceIdDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}
#[allow(dead_code)]
struct InstancePortDeserializer;
impl InstancePortDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, |s| Ok(i64::from_str(&s).unwrap()))
    }
}
/// <p>Information about the state of an EC2 instance.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
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

#[allow(dead_code)]
struct InstanceStateDeserializer;
impl InstanceStateDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<InstanceState, XmlParseError> {
        deserialize_elements::<_, InstanceState, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Description" => {
                    obj.description =
                        Some(DescriptionDeserializer::deserialize("Description", stack)?);
                }
                "InstanceId" => {
                    obj.instance_id =
                        Some(InstanceIdDeserializer::deserialize("InstanceId", stack)?);
                }
                "ReasonCode" => {
                    obj.reason_code =
                        Some(ReasonCodeDeserializer::deserialize("ReasonCode", stack)?);
                }
                "State" => {
                    obj.state = Some(StateDeserializer::deserialize("State", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
#[allow(dead_code)]
struct InstanceStatesDeserializer;
impl InstanceStatesDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<InstanceState>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(InstanceStateDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
#[allow(dead_code)]
struct InstancesDeserializer;
impl InstancesDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Instance>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(InstanceDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
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

#[allow(dead_code)]
struct LBCookieStickinessPoliciesDeserializer;
impl LBCookieStickinessPoliciesDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<LBCookieStickinessPolicy>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(LBCookieStickinessPolicyDeserializer::deserialize(
                    "member", stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
/// <p>Information about a policy for duration-based session stickiness.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct LBCookieStickinessPolicy {
    /// <p>The time period, in seconds, after which the cookie should be considered stale. If this parameter is not specified, the stickiness session lasts for the duration of the browser session.</p>
    pub cookie_expiration_period: Option<i64>,
    /// <p>The name of the policy. This name must be unique within the set of policies for this load balancer.</p>
    pub policy_name: Option<String>,
}

#[allow(dead_code)]
struct LBCookieStickinessPolicyDeserializer;
impl LBCookieStickinessPolicyDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<LBCookieStickinessPolicy, XmlParseError> {
        deserialize_elements::<_, LBCookieStickinessPolicy, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "CookieExpirationPeriod" => {
                        obj.cookie_expiration_period =
                            Some(CookieExpirationPeriodDeserializer::deserialize(
                                "CookieExpirationPeriod",
                                stack,
                            )?);
                    }
                    "PolicyName" => {
                        obj.policy_name =
                            Some(PolicyNameDeserializer::deserialize("PolicyName", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p>Information about an Elastic Load Balancing resource limit for your AWS account.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct Limit {
    /// <p>The maximum value of the limit.</p>
    pub max: Option<String>,
    /// <p><p>The name of the limit. The possible values are:</p> <ul> <li> <p>classic-listeners</p> </li> <li> <p>classic-load-balancers</p> </li> <li> <p>classic-registered-instances</p> </li> </ul></p>
    pub name: Option<String>,
}

#[allow(dead_code)]
struct LimitDeserializer;
impl LimitDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Limit, XmlParseError> {
        deserialize_elements::<_, Limit, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Max" => {
                    obj.max = Some(MaxDeserializer::deserialize("Max", stack)?);
                }
                "Name" => {
                    obj.name = Some(NameDeserializer::deserialize("Name", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
#[allow(dead_code)]
struct LimitsDeserializer;
impl LimitsDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Limit>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(LimitDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
/// <p>Information about a listener.</p> <p>For information about the protocols and the ports supported by Elastic Load Balancing, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/classic/elb-listener-config.html">Listeners for Your Classic Load Balancer</a> in the <i>Classic Load Balancers Guide</i>.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Listener {
    /// <p>The port on which the instance is listening.</p>
    pub instance_port: i64,
    /// <p>The protocol to use for routing traffic to instances: HTTP, HTTPS, TCP, or SSL.</p> <p>If the front-end protocol is TCP or SSL, the back-end protocol must be TCP or SSL. If the front-end protocol is HTTP or HTTPS, the back-end protocol must be HTTP or HTTPS.</p> <p>If there is another listener with the same <code>InstancePort</code> whose <code>InstanceProtocol</code> is secure, (HTTPS or SSL), the listener's <code>InstanceProtocol</code> must also be secure.</p> <p>If there is another listener with the same <code>InstancePort</code> whose <code>InstanceProtocol</code> is HTTP or TCP, the listener's <code>InstanceProtocol</code> must be HTTP or TCP.</p>
    pub instance_protocol: Option<String>,
    /// <p>The port on which the load balancer is listening. On EC2-VPC, you can specify any port from the range 1-65535. On EC2-Classic, you can specify any port from the following list: 25, 80, 443, 465, 587, 1024-65535.</p>
    pub load_balancer_port: i64,
    /// <p>The load balancer transport protocol to use for routing: HTTP, HTTPS, TCP, or SSL.</p>
    pub protocol: String,
    /// <p>The Amazon Resource Name (ARN) of the server certificate.</p>
    pub ssl_certificate_id: Option<String>,
}

#[allow(dead_code)]
struct ListenerDeserializer;
impl ListenerDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Listener, XmlParseError> {
        deserialize_elements::<_, Listener, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "InstancePort" => {
                    obj.instance_port =
                        InstancePortDeserializer::deserialize("InstancePort", stack)?;
                }
                "InstanceProtocol" => {
                    obj.instance_protocol = Some(ProtocolDeserializer::deserialize(
                        "InstanceProtocol",
                        stack,
                    )?);
                }
                "LoadBalancerPort" => {
                    obj.load_balancer_port =
                        AccessPointPortDeserializer::deserialize("LoadBalancerPort", stack)?;
                }
                "Protocol" => {
                    obj.protocol = ProtocolDeserializer::deserialize("Protocol", stack)?;
                }
                "SSLCertificateId" => {
                    obj.ssl_certificate_id = Some(SSLCertificateIdDeserializer::deserialize(
                        "SSLCertificateId",
                        stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
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

        params.put(&format!("{}{}", prefix, "InstancePort"), &obj.instance_port);
        if let Some(ref field_value) = obj.instance_protocol {
            params.put(&format!("{}{}", prefix, "InstanceProtocol"), &field_value);
        }
        params.put(
            &format!("{}{}", prefix, "LoadBalancerPort"),
            &obj.load_balancer_port,
        );
        params.put(&format!("{}{}", prefix, "Protocol"), &obj.protocol);
        if let Some(ref field_value) = obj.ssl_certificate_id {
            params.put(&format!("{}{}", prefix, "SSLCertificateId"), &field_value);
        }
    }
}

/// <p>The policies enabled for a listener.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ListenerDescription {
    /// <p>The listener.</p>
    pub listener: Option<Listener>,
    /// <p>The policies. If there are no policies enabled, the list is empty.</p>
    pub policy_names: Option<Vec<String>>,
}

#[allow(dead_code)]
struct ListenerDescriptionDeserializer;
impl ListenerDescriptionDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListenerDescription, XmlParseError> {
        deserialize_elements::<_, ListenerDescription, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Listener" => {
                    obj.listener = Some(ListenerDeserializer::deserialize("Listener", stack)?);
                }
                "PolicyNames" => {
                    obj.policy_names
                        .get_or_insert(vec![])
                        .extend(PolicyNamesDeserializer::deserialize("PolicyNames", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
#[allow(dead_code)]
struct ListenerDescriptionsDeserializer;
impl ListenerDescriptionsDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<ListenerDescription>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(ListenerDescriptionDeserializer::deserialize(
                    "member", stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
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
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct LoadBalancerAttributes {
    /// <p>If enabled, the load balancer captures detailed information of all requests and delivers the information to the Amazon S3 bucket that you specify.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/classic/enable-access-logs.html">Enable Access Logs</a> in the <i>Classic Load Balancers Guide</i>.</p>
    pub access_log: Option<AccessLog>,
    /// <p>Any additional attributes.</p>
    pub additional_attributes: Option<Vec<AdditionalAttribute>>,
    /// <p>If enabled, the load balancer allows existing requests to complete before the load balancer shifts traffic away from a deregistered or unhealthy instance.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/classic/config-conn-drain.html">Configure Connection Draining</a> in the <i>Classic Load Balancers Guide</i>.</p>
    pub connection_draining: Option<ConnectionDraining>,
    /// <p>If enabled, the load balancer allows the connections to remain idle (no data is sent over the connection) for the specified duration.</p> <p>By default, Elastic Load Balancing maintains a 60-second idle connection timeout for both front-end and back-end connections of your load balancer. For more information, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/classic/config-idle-timeout.html">Configure Idle Connection Timeout</a> in the <i>Classic Load Balancers Guide</i>.</p>
    pub connection_settings: Option<ConnectionSettings>,
    /// <p>If enabled, the load balancer routes the request traffic evenly across all instances regardless of the Availability Zones.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/classic/enable-disable-crosszone-lb.html">Configure Cross-Zone Load Balancing</a> in the <i>Classic Load Balancers Guide</i>.</p>
    pub cross_zone_load_balancing: Option<CrossZoneLoadBalancing>,
}

#[allow(dead_code)]
struct LoadBalancerAttributesDeserializer;
impl LoadBalancerAttributesDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<LoadBalancerAttributes, XmlParseError> {
        deserialize_elements::<_, LoadBalancerAttributes, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "AccessLog" => {
                    obj.access_log = Some(AccessLogDeserializer::deserialize("AccessLog", stack)?);
                }
                "AdditionalAttributes" => {
                    obj.additional_attributes.get_or_insert(vec![]).extend(
                        AdditionalAttributesDeserializer::deserialize(
                            "AdditionalAttributes",
                            stack,
                        )?,
                    );
                }
                "ConnectionDraining" => {
                    obj.connection_draining = Some(ConnectionDrainingDeserializer::deserialize(
                        "ConnectionDraining",
                        stack,
                    )?);
                }
                "ConnectionSettings" => {
                    obj.connection_settings = Some(ConnectionSettingsDeserializer::deserialize(
                        "ConnectionSettings",
                        stack,
                    )?);
                }
                "CrossZoneLoadBalancing" => {
                    obj.cross_zone_load_balancing =
                        Some(CrossZoneLoadBalancingDeserializer::deserialize(
                            "CrossZoneLoadBalancing",
                            stack,
                        )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
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
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct LoadBalancerDescription {
    /// <p>The Availability Zones for the load balancer.</p>
    pub availability_zones: Option<Vec<String>>,
    /// <p>Information about your EC2 instances.</p>
    pub backend_server_descriptions: Option<Vec<BackendServerDescription>>,
    /// <p>The DNS name of the load balancer.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/classic/using-domain-names-with-elb.html">Configure a Custom Domain Name</a> in the <i>Classic Load Balancers Guide</i>.</p>
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

#[allow(dead_code)]
struct LoadBalancerDescriptionDeserializer;
impl LoadBalancerDescriptionDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<LoadBalancerDescription, XmlParseError> {
        deserialize_elements::<_, LoadBalancerDescription, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "AvailabilityZones" => {
                        obj.availability_zones.get_or_insert(vec![]).extend(
                            AvailabilityZonesDeserializer::deserialize("AvailabilityZones", stack)?,
                        );
                    }
                    "BackendServerDescriptions" => {
                        obj.backend_server_descriptions
                            .get_or_insert(vec![])
                            .extend(BackendServerDescriptionsDeserializer::deserialize(
                                "BackendServerDescriptions",
                                stack,
                            )?);
                    }
                    "CanonicalHostedZoneName" => {
                        obj.canonical_hosted_zone_name = Some(DNSNameDeserializer::deserialize(
                            "CanonicalHostedZoneName",
                            stack,
                        )?);
                    }
                    "CanonicalHostedZoneNameID" => {
                        obj.canonical_hosted_zone_name_id = Some(DNSNameDeserializer::deserialize(
                            "CanonicalHostedZoneNameID",
                            stack,
                        )?);
                    }
                    "CreatedTime" => {
                        obj.created_time =
                            Some(CreatedTimeDeserializer::deserialize("CreatedTime", stack)?);
                    }
                    "DNSName" => {
                        obj.dns_name = Some(DNSNameDeserializer::deserialize("DNSName", stack)?);
                    }
                    "HealthCheck" => {
                        obj.health_check =
                            Some(HealthCheckDeserializer::deserialize("HealthCheck", stack)?);
                    }
                    "Instances" => {
                        obj.instances
                            .get_or_insert(vec![])
                            .extend(InstancesDeserializer::deserialize("Instances", stack)?);
                    }
                    "ListenerDescriptions" => {
                        obj.listener_descriptions.get_or_insert(vec![]).extend(
                            ListenerDescriptionsDeserializer::deserialize(
                                "ListenerDescriptions",
                                stack,
                            )?,
                        );
                    }
                    "LoadBalancerName" => {
                        obj.load_balancer_name = Some(AccessPointNameDeserializer::deserialize(
                            "LoadBalancerName",
                            stack,
                        )?);
                    }
                    "Policies" => {
                        obj.policies = Some(PoliciesDeserializer::deserialize("Policies", stack)?);
                    }
                    "Scheme" => {
                        obj.scheme = Some(LoadBalancerSchemeDeserializer::deserialize(
                            "Scheme", stack,
                        )?);
                    }
                    "SecurityGroups" => {
                        obj.security_groups.get_or_insert(vec![]).extend(
                            SecurityGroupsDeserializer::deserialize("SecurityGroups", stack)?,
                        );
                    }
                    "SourceSecurityGroup" => {
                        obj.source_security_group =
                            Some(SourceSecurityGroupDeserializer::deserialize(
                                "SourceSecurityGroup",
                                stack,
                            )?);
                    }
                    "Subnets" => {
                        obj.subnets
                            .get_or_insert(vec![])
                            .extend(SubnetsDeserializer::deserialize("Subnets", stack)?);
                    }
                    "VPCId" => {
                        obj.vpc_id = Some(VPCIdDeserializer::deserialize("VPCId", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
#[allow(dead_code)]
struct LoadBalancerDescriptionsDeserializer;
impl LoadBalancerDescriptionsDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<LoadBalancerDescription>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(LoadBalancerDescriptionDeserializer::deserialize(
                    "member", stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
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

#[allow(dead_code)]
struct LoadBalancerSchemeDeserializer;
impl LoadBalancerSchemeDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}
#[allow(dead_code)]
struct MarkerDeserializer;
impl MarkerDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}
#[allow(dead_code)]
struct MaxDeserializer;
impl MaxDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}
/// <p>Contains the parameters for ModifyLoadBalancerAttributes.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
            &obj.load_balancer_name,
        );
    }
}

/// <p>Contains the output of ModifyLoadBalancerAttributes.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ModifyLoadBalancerAttributesOutput {
    /// <p>Information about the load balancer attributes.</p>
    pub load_balancer_attributes: Option<LoadBalancerAttributes>,
    /// <p>The name of the load balancer.</p>
    pub load_balancer_name: Option<String>,
}

#[allow(dead_code)]
struct ModifyLoadBalancerAttributesOutputDeserializer;
impl ModifyLoadBalancerAttributesOutputDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ModifyLoadBalancerAttributesOutput, XmlParseError> {
        deserialize_elements::<_, ModifyLoadBalancerAttributesOutput, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "LoadBalancerAttributes" => {
                        obj.load_balancer_attributes =
                            Some(LoadBalancerAttributesDeserializer::deserialize(
                                "LoadBalancerAttributes",
                                stack,
                            )?);
                    }
                    "LoadBalancerName" => {
                        obj.load_balancer_name = Some(AccessPointNameDeserializer::deserialize(
                            "LoadBalancerName",
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
#[allow(dead_code)]
struct NameDeserializer;
impl NameDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}
/// <p>The policies for a load balancer.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct Policies {
    /// <p>The stickiness policies created using <a>CreateAppCookieStickinessPolicy</a>.</p>
    pub app_cookie_stickiness_policies: Option<Vec<AppCookieStickinessPolicy>>,
    /// <p>The stickiness policies created using <a>CreateLBCookieStickinessPolicy</a>.</p>
    pub lb_cookie_stickiness_policies: Option<Vec<LBCookieStickinessPolicy>>,
    /// <p>The policies other than the stickiness policies.</p>
    pub other_policies: Option<Vec<String>>,
}

#[allow(dead_code)]
struct PoliciesDeserializer;
impl PoliciesDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Policies, XmlParseError> {
        deserialize_elements::<_, Policies, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "AppCookieStickinessPolicies" => {
                    obj.app_cookie_stickiness_policies
                        .get_or_insert(vec![])
                        .extend(AppCookieStickinessPoliciesDeserializer::deserialize(
                            "AppCookieStickinessPolicies",
                            stack,
                        )?);
                }
                "LBCookieStickinessPolicies" => {
                    obj.lb_cookie_stickiness_policies
                        .get_or_insert(vec![])
                        .extend(LBCookieStickinessPoliciesDeserializer::deserialize(
                            "LBCookieStickinessPolicies",
                            stack,
                        )?);
                }
                "OtherPolicies" => {
                    obj.other_policies.get_or_insert(vec![]).extend(
                        PolicyNamesDeserializer::deserialize("OtherPolicies", stack)?,
                    );
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
/// <p>Information about a policy attribute.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
            params.put(&format!("{}{}", prefix, "AttributeName"), &field_value);
        }
        if let Some(ref field_value) = obj.attribute_value {
            params.put(&format!("{}{}", prefix, "AttributeValue"), &field_value);
        }
    }
}

/// <p>Information about a policy attribute.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct PolicyAttributeDescription {
    /// <p>The name of the attribute.</p>
    pub attribute_name: Option<String>,
    /// <p>The value of the attribute.</p>
    pub attribute_value: Option<String>,
}

#[allow(dead_code)]
struct PolicyAttributeDescriptionDeserializer;
impl PolicyAttributeDescriptionDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PolicyAttributeDescription, XmlParseError> {
        deserialize_elements::<_, PolicyAttributeDescription, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "AttributeName" => {
                        obj.attribute_name = Some(AttributeNameDeserializer::deserialize(
                            "AttributeName",
                            stack,
                        )?);
                    }
                    "AttributeValue" => {
                        obj.attribute_value = Some(AttributeValueDeserializer::deserialize(
                            "AttributeValue",
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
#[allow(dead_code)]
struct PolicyAttributeDescriptionsDeserializer;
impl PolicyAttributeDescriptionsDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<PolicyAttributeDescription>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(PolicyAttributeDescriptionDeserializer::deserialize(
                    "member", stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
/// <p>Information about a policy attribute type.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
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

#[allow(dead_code)]
struct PolicyAttributeTypeDescriptionDeserializer;
impl PolicyAttributeTypeDescriptionDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PolicyAttributeTypeDescription, XmlParseError> {
        deserialize_elements::<_, PolicyAttributeTypeDescription, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "AttributeName" => {
                        obj.attribute_name = Some(AttributeNameDeserializer::deserialize(
                            "AttributeName",
                            stack,
                        )?);
                    }
                    "AttributeType" => {
                        obj.attribute_type = Some(AttributeTypeDeserializer::deserialize(
                            "AttributeType",
                            stack,
                        )?);
                    }
                    "Cardinality" => {
                        obj.cardinality =
                            Some(CardinalityDeserializer::deserialize("Cardinality", stack)?);
                    }
                    "DefaultValue" => {
                        obj.default_value = Some(DefaultValueDeserializer::deserialize(
                            "DefaultValue",
                            stack,
                        )?);
                    }
                    "Description" => {
                        obj.description =
                            Some(DescriptionDeserializer::deserialize("Description", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
#[allow(dead_code)]
struct PolicyAttributeTypeDescriptionsDeserializer;
impl PolicyAttributeTypeDescriptionsDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<PolicyAttributeTypeDescription>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(PolicyAttributeTypeDescriptionDeserializer::deserialize(
                    "member", stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
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
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct PolicyDescription {
    /// <p>The policy attributes.</p>
    pub policy_attribute_descriptions: Option<Vec<PolicyAttributeDescription>>,
    /// <p>The name of the policy.</p>
    pub policy_name: Option<String>,
    /// <p>The name of the policy type.</p>
    pub policy_type_name: Option<String>,
}

#[allow(dead_code)]
struct PolicyDescriptionDeserializer;
impl PolicyDescriptionDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PolicyDescription, XmlParseError> {
        deserialize_elements::<_, PolicyDescription, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "PolicyAttributeDescriptions" => {
                    obj.policy_attribute_descriptions
                        .get_or_insert(vec![])
                        .extend(PolicyAttributeDescriptionsDeserializer::deserialize(
                            "PolicyAttributeDescriptions",
                            stack,
                        )?);
                }
                "PolicyName" => {
                    obj.policy_name =
                        Some(PolicyNameDeserializer::deserialize("PolicyName", stack)?);
                }
                "PolicyTypeName" => {
                    obj.policy_type_name = Some(PolicyTypeNameDeserializer::deserialize(
                        "PolicyTypeName",
                        stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
#[allow(dead_code)]
struct PolicyDescriptionsDeserializer;
impl PolicyDescriptionsDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<PolicyDescription>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(PolicyDescriptionDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
#[allow(dead_code)]
struct PolicyNameDeserializer;
impl PolicyNameDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}
#[allow(dead_code)]
struct PolicyNamesDeserializer;
impl PolicyNamesDeserializer {
    #[allow(dead_code, unused_variables)]
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
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct PolicyTypeDescription {
    /// <p>A description of the policy type.</p>
    pub description: Option<String>,
    /// <p>The description of the policy attributes associated with the policies defined by Elastic Load Balancing.</p>
    pub policy_attribute_type_descriptions: Option<Vec<PolicyAttributeTypeDescription>>,
    /// <p>The name of the policy type.</p>
    pub policy_type_name: Option<String>,
}

#[allow(dead_code)]
struct PolicyTypeDescriptionDeserializer;
impl PolicyTypeDescriptionDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PolicyTypeDescription, XmlParseError> {
        deserialize_elements::<_, PolicyTypeDescription, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Description" => {
                    obj.description =
                        Some(DescriptionDeserializer::deserialize("Description", stack)?);
                }
                "PolicyAttributeTypeDescriptions" => {
                    obj.policy_attribute_type_descriptions
                        .get_or_insert(vec![])
                        .extend(PolicyAttributeTypeDescriptionsDeserializer::deserialize(
                            "PolicyAttributeTypeDescriptions",
                            stack,
                        )?);
                }
                "PolicyTypeName" => {
                    obj.policy_type_name = Some(PolicyTypeNameDeserializer::deserialize(
                        "PolicyTypeName",
                        stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
#[allow(dead_code)]
struct PolicyTypeDescriptionsDeserializer;
impl PolicyTypeDescriptionsDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<PolicyTypeDescription>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(PolicyTypeDescriptionDeserializer::deserialize(
                    "member", stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
#[allow(dead_code)]
struct PolicyTypeNameDeserializer;
impl PolicyTypeNameDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
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
            params.put(&key, &obj);
        }
    }
}

#[allow(dead_code)]
struct ProtocolDeserializer;
impl ProtocolDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}
#[allow(dead_code)]
struct ReasonCodeDeserializer;
impl ReasonCodeDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}
/// <p>Contains the parameters for RegisterInstancesWithLoadBalancer.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
            &obj.load_balancer_name,
        );
    }
}

/// <p>Contains the output of RegisterInstancesWithLoadBalancer.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct RegisterEndPointsOutput {
    /// <p>The updated list of instances for the load balancer.</p>
    pub instances: Option<Vec<Instance>>,
}

#[allow(dead_code)]
struct RegisterEndPointsOutputDeserializer;
impl RegisterEndPointsOutputDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<RegisterEndPointsOutput, XmlParseError> {
        deserialize_elements::<_, RegisterEndPointsOutput, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Instances" => {
                        obj.instances
                            .get_or_insert(vec![])
                            .extend(InstancesDeserializer::deserialize("Instances", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p>Contains the parameters for DisableAvailabilityZonesForLoadBalancer.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
            &obj.load_balancer_name,
        );
    }
}

/// <p>Contains the output for DisableAvailabilityZonesForLoadBalancer.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct RemoveAvailabilityZonesOutput {
    /// <p>The remaining Availability Zones for the load balancer.</p>
    pub availability_zones: Option<Vec<String>>,
}

#[allow(dead_code)]
struct RemoveAvailabilityZonesOutputDeserializer;
impl RemoveAvailabilityZonesOutputDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<RemoveAvailabilityZonesOutput, XmlParseError> {
        deserialize_elements::<_, RemoveAvailabilityZonesOutput, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "AvailabilityZones" => {
                        obj.availability_zones.get_or_insert(vec![]).extend(
                            AvailabilityZonesDeserializer::deserialize("AvailabilityZones", stack)?,
                        );
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p>Contains the parameters for RemoveTags.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct RemoveTagsOutput {}

#[allow(dead_code)]
struct RemoveTagsOutputDeserializer;
impl RemoveTagsOutputDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<RemoveTagsOutput, XmlParseError> {
        xml_util::start_element(tag_name, stack)?;

        let obj = RemoveTagsOutput::default();

        xml_util::end_element(tag_name, stack)?;

        Ok(obj)
    }
}
#[allow(dead_code)]
struct S3BucketNameDeserializer;
impl S3BucketNameDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}
#[allow(dead_code)]
struct SSLCertificateIdDeserializer;
impl SSLCertificateIdDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}
#[allow(dead_code)]
struct SecurityGroupIdDeserializer;
impl SecurityGroupIdDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}
#[allow(dead_code)]
struct SecurityGroupNameDeserializer;
impl SecurityGroupNameDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}
#[allow(dead_code)]
struct SecurityGroupOwnerAliasDeserializer;
impl SecurityGroupOwnerAliasDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}
#[allow(dead_code)]
struct SecurityGroupsDeserializer;
impl SecurityGroupsDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(SecurityGroupIdDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
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
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
            &obj.load_balancer_name,
        );
        params.put(
            &format!("{}{}", prefix, "LoadBalancerPort"),
            &obj.load_balancer_port,
        );
        params.put(
            &format!("{}{}", prefix, "SSLCertificateId"),
            &obj.ssl_certificate_id,
        );
    }
}

/// <p>Contains the output of SetLoadBalancerListenerSSLCertificate.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct SetLoadBalancerListenerSSLCertificateOutput {}

#[allow(dead_code)]
struct SetLoadBalancerListenerSSLCertificateOutputDeserializer;
impl SetLoadBalancerListenerSSLCertificateOutputDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<SetLoadBalancerListenerSSLCertificateOutput, XmlParseError> {
        xml_util::start_element(tag_name, stack)?;

        let obj = SetLoadBalancerListenerSSLCertificateOutput::default();

        xml_util::end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Contains the parameters for SetLoadBalancerPoliciesForBackendServer.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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

        params.put(&format!("{}{}", prefix, "InstancePort"), &obj.instance_port);
        params.put(
            &format!("{}{}", prefix, "LoadBalancerName"),
            &obj.load_balancer_name,
        );
        PolicyNamesSerializer::serialize(
            params,
            &format!("{}{}", prefix, "PolicyNames"),
            &obj.policy_names,
        );
    }
}

/// <p>Contains the output of SetLoadBalancerPoliciesForBackendServer.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct SetLoadBalancerPoliciesForBackendServerOutput {}

#[allow(dead_code)]
struct SetLoadBalancerPoliciesForBackendServerOutputDeserializer;
impl SetLoadBalancerPoliciesForBackendServerOutputDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<SetLoadBalancerPoliciesForBackendServerOutput, XmlParseError> {
        xml_util::start_element(tag_name, stack)?;

        let obj = SetLoadBalancerPoliciesForBackendServerOutput::default();

        xml_util::end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Contains the parameters for SetLoadBalancePoliciesOfListener.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
            &obj.load_balancer_name,
        );
        params.put(
            &format!("{}{}", prefix, "LoadBalancerPort"),
            &obj.load_balancer_port,
        );
        PolicyNamesSerializer::serialize(
            params,
            &format!("{}{}", prefix, "PolicyNames"),
            &obj.policy_names,
        );
    }
}

/// <p>Contains the output of SetLoadBalancePoliciesOfListener.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct SetLoadBalancerPoliciesOfListenerOutput {}

#[allow(dead_code)]
struct SetLoadBalancerPoliciesOfListenerOutputDeserializer;
impl SetLoadBalancerPoliciesOfListenerOutputDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<SetLoadBalancerPoliciesOfListenerOutput, XmlParseError> {
        xml_util::start_element(tag_name, stack)?;

        let obj = SetLoadBalancerPoliciesOfListenerOutput::default();

        xml_util::end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Information about a source security group.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct SourceSecurityGroup {
    /// <p>The name of the security group.</p>
    pub group_name: Option<String>,
    /// <p>The owner of the security group.</p>
    pub owner_alias: Option<String>,
}

#[allow(dead_code)]
struct SourceSecurityGroupDeserializer;
impl SourceSecurityGroupDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<SourceSecurityGroup, XmlParseError> {
        deserialize_elements::<_, SourceSecurityGroup, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "GroupName" => {
                    obj.group_name = Some(SecurityGroupNameDeserializer::deserialize(
                        "GroupName",
                        stack,
                    )?);
                }
                "OwnerAlias" => {
                    obj.owner_alias = Some(SecurityGroupOwnerAliasDeserializer::deserialize(
                        "OwnerAlias",
                        stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
#[allow(dead_code)]
struct StateDeserializer;
impl StateDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}
#[allow(dead_code)]
struct SubnetIdDeserializer;
impl SubnetIdDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}
#[allow(dead_code)]
struct SubnetsDeserializer;
impl SubnetsDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(SubnetIdDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
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
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Tag {
    /// <p>The key of the tag.</p>
    pub key: String,
    /// <p>The value of the tag.</p>
    pub value: Option<String>,
}

#[allow(dead_code)]
struct TagDeserializer;
impl TagDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Tag, XmlParseError> {
        deserialize_elements::<_, Tag, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Key" => {
                    obj.key = TagKeyDeserializer::deserialize("Key", stack)?;
                }
                "Value" => {
                    obj.value = Some(TagValueDeserializer::deserialize("Value", stack)?);
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
        if let Some(ref field_value) = obj.value {
            params.put(&format!("{}{}", prefix, "Value"), &field_value);
        }
    }
}

/// <p>The tags associated with a load balancer.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct TagDescription {
    /// <p>The name of the load balancer.</p>
    pub load_balancer_name: Option<String>,
    /// <p>The tags.</p>
    pub tags: Option<Vec<Tag>>,
}

#[allow(dead_code)]
struct TagDescriptionDeserializer;
impl TagDescriptionDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<TagDescription, XmlParseError> {
        deserialize_elements::<_, TagDescription, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "LoadBalancerName" => {
                    obj.load_balancer_name = Some(AccessPointNameDeserializer::deserialize(
                        "LoadBalancerName",
                        stack,
                    )?);
                }
                "Tags" => {
                    obj.tags
                        .get_or_insert(vec![])
                        .extend(TagListDeserializer::deserialize("Tags", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
#[allow(dead_code)]
struct TagDescriptionsDeserializer;
impl TagDescriptionsDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<TagDescription>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(TagDescriptionDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
#[allow(dead_code)]
struct TagKeyDeserializer;
impl TagKeyDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
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
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
            params.put(&format!("{}{}", prefix, "Key"), &field_value);
        }
    }
}

#[allow(dead_code)]
struct TagListDeserializer;
impl TagListDeserializer {
    #[allow(dead_code, unused_variables)]
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

#[allow(dead_code)]
struct TagValueDeserializer;
impl TagValueDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}
#[allow(dead_code)]
struct UnhealthyThresholdDeserializer;
impl UnhealthyThresholdDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, |s| Ok(i64::from_str(&s).unwrap()))
    }
}
#[allow(dead_code)]
struct VPCIdDeserializer;
impl VPCIdDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
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
}

impl AddTagsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AddTagsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "LoadBalancerNotFound" => {
                        return RusotoError::Service(AddTagsError::AccessPointNotFound(
                            parsed_error.message,
                        ))
                    }
                    "DuplicateTagKeys" => {
                        return RusotoError::Service(AddTagsError::DuplicateTagKeys(
                            parsed_error.message,
                        ))
                    }
                    "TooManyTags" => {
                        return RusotoError::Service(AddTagsError::TooManyTags(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn refine(err: RusotoError<std::convert::Infallible>) -> RusotoError<AddTagsError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for AddTagsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AddTagsError::AccessPointNotFound(ref cause) => write!(f, "{}", cause),
            AddTagsError::DuplicateTagKeys(ref cause) => write!(f, "{}", cause),
            AddTagsError::TooManyTags(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AddTagsError {}
/// Errors returned by ApplySecurityGroupsToLoadBalancer
#[derive(Debug, PartialEq)]
pub enum ApplySecurityGroupsToLoadBalancerError {
    /// <p>The specified load balancer does not exist.</p>
    AccessPointNotFound(String),
    /// <p>The requested configuration change is not valid.</p>
    InvalidConfigurationRequest(String),
    /// <p>One or more of the specified security groups do not exist.</p>
    InvalidSecurityGroup(String),
}

impl ApplySecurityGroupsToLoadBalancerError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ApplySecurityGroupsToLoadBalancerError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "LoadBalancerNotFound" => {
                        return RusotoError::Service(
                            ApplySecurityGroupsToLoadBalancerError::AccessPointNotFound(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidConfigurationRequest" => {
                        return RusotoError::Service(
                            ApplySecurityGroupsToLoadBalancerError::InvalidConfigurationRequest(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidSecurityGroup" => {
                        return RusotoError::Service(
                            ApplySecurityGroupsToLoadBalancerError::InvalidSecurityGroup(
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

    fn refine(
        err: RusotoError<std::convert::Infallible>,
    ) -> RusotoError<ApplySecurityGroupsToLoadBalancerError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for ApplySecurityGroupsToLoadBalancerError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ApplySecurityGroupsToLoadBalancerError::AccessPointNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            ApplySecurityGroupsToLoadBalancerError::InvalidConfigurationRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            ApplySecurityGroupsToLoadBalancerError::InvalidSecurityGroup(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ApplySecurityGroupsToLoadBalancerError {}
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
}

impl AttachLoadBalancerToSubnetsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<AttachLoadBalancerToSubnetsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "LoadBalancerNotFound" => {
                        return RusotoError::Service(
                            AttachLoadBalancerToSubnetsError::AccessPointNotFound(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidConfigurationRequest" => {
                        return RusotoError::Service(
                            AttachLoadBalancerToSubnetsError::InvalidConfigurationRequest(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidSubnet" => {
                        return RusotoError::Service(
                            AttachLoadBalancerToSubnetsError::InvalidSubnet(parsed_error.message),
                        )
                    }
                    "SubnetNotFound" => {
                        return RusotoError::Service(
                            AttachLoadBalancerToSubnetsError::SubnetNotFound(parsed_error.message),
                        )
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn refine(
        err: RusotoError<std::convert::Infallible>,
    ) -> RusotoError<AttachLoadBalancerToSubnetsError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for AttachLoadBalancerToSubnetsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AttachLoadBalancerToSubnetsError::AccessPointNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            AttachLoadBalancerToSubnetsError::InvalidConfigurationRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            AttachLoadBalancerToSubnetsError::InvalidSubnet(ref cause) => write!(f, "{}", cause),
            AttachLoadBalancerToSubnetsError::SubnetNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AttachLoadBalancerToSubnetsError {}
/// Errors returned by ConfigureHealthCheck
#[derive(Debug, PartialEq)]
pub enum ConfigureHealthCheckError {
    /// <p>The specified load balancer does not exist.</p>
    AccessPointNotFound(String),
}

impl ConfigureHealthCheckError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ConfigureHealthCheckError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "LoadBalancerNotFound" => {
                        return RusotoError::Service(
                            ConfigureHealthCheckError::AccessPointNotFound(parsed_error.message),
                        )
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn refine(
        err: RusotoError<std::convert::Infallible>,
    ) -> RusotoError<ConfigureHealthCheckError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for ConfigureHealthCheckError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ConfigureHealthCheckError::AccessPointNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ConfigureHealthCheckError {}
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
}

impl CreateAppCookieStickinessPolicyError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateAppCookieStickinessPolicyError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "LoadBalancerNotFound" => {
                        return RusotoError::Service(
                            CreateAppCookieStickinessPolicyError::AccessPointNotFound(
                                parsed_error.message,
                            ),
                        )
                    }
                    "DuplicatePolicyName" => {
                        return RusotoError::Service(
                            CreateAppCookieStickinessPolicyError::DuplicatePolicyName(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidConfigurationRequest" => {
                        return RusotoError::Service(
                            CreateAppCookieStickinessPolicyError::InvalidConfigurationRequest(
                                parsed_error.message,
                            ),
                        )
                    }
                    "TooManyPolicies" => {
                        return RusotoError::Service(
                            CreateAppCookieStickinessPolicyError::TooManyPolicies(
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

    fn refine(
        err: RusotoError<std::convert::Infallible>,
    ) -> RusotoError<CreateAppCookieStickinessPolicyError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for CreateAppCookieStickinessPolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateAppCookieStickinessPolicyError::AccessPointNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateAppCookieStickinessPolicyError::DuplicatePolicyName(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateAppCookieStickinessPolicyError::InvalidConfigurationRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateAppCookieStickinessPolicyError::TooManyPolicies(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for CreateAppCookieStickinessPolicyError {}
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
}

impl CreateLBCookieStickinessPolicyError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateLBCookieStickinessPolicyError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "LoadBalancerNotFound" => {
                        return RusotoError::Service(
                            CreateLBCookieStickinessPolicyError::AccessPointNotFound(
                                parsed_error.message,
                            ),
                        )
                    }
                    "DuplicatePolicyName" => {
                        return RusotoError::Service(
                            CreateLBCookieStickinessPolicyError::DuplicatePolicyName(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidConfigurationRequest" => {
                        return RusotoError::Service(
                            CreateLBCookieStickinessPolicyError::InvalidConfigurationRequest(
                                parsed_error.message,
                            ),
                        )
                    }
                    "TooManyPolicies" => {
                        return RusotoError::Service(
                            CreateLBCookieStickinessPolicyError::TooManyPolicies(
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

    fn refine(
        err: RusotoError<std::convert::Infallible>,
    ) -> RusotoError<CreateLBCookieStickinessPolicyError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for CreateLBCookieStickinessPolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateLBCookieStickinessPolicyError::AccessPointNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateLBCookieStickinessPolicyError::DuplicatePolicyName(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateLBCookieStickinessPolicyError::InvalidConfigurationRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateLBCookieStickinessPolicyError::TooManyPolicies(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for CreateLBCookieStickinessPolicyError {}
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
}

impl CreateLoadBalancerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateLoadBalancerError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "CertificateNotFound" => {
                        return RusotoError::Service(CreateLoadBalancerError::CertificateNotFound(
                            parsed_error.message,
                        ))
                    }
                    "DuplicateLoadBalancerName" => {
                        return RusotoError::Service(
                            CreateLoadBalancerError::DuplicateAccessPointName(parsed_error.message),
                        )
                    }
                    "DuplicateTagKeys" => {
                        return RusotoError::Service(CreateLoadBalancerError::DuplicateTagKeys(
                            parsed_error.message,
                        ))
                    }
                    "InvalidConfigurationRequest" => {
                        return RusotoError::Service(
                            CreateLoadBalancerError::InvalidConfigurationRequest(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidScheme" => {
                        return RusotoError::Service(CreateLoadBalancerError::InvalidScheme(
                            parsed_error.message,
                        ))
                    }
                    "InvalidSecurityGroup" => {
                        return RusotoError::Service(CreateLoadBalancerError::InvalidSecurityGroup(
                            parsed_error.message,
                        ))
                    }
                    "InvalidSubnet" => {
                        return RusotoError::Service(CreateLoadBalancerError::InvalidSubnet(
                            parsed_error.message,
                        ))
                    }
                    "OperationNotPermitted" => {
                        return RusotoError::Service(
                            CreateLoadBalancerError::OperationNotPermitted(parsed_error.message),
                        )
                    }
                    "SubnetNotFound" => {
                        return RusotoError::Service(CreateLoadBalancerError::SubnetNotFound(
                            parsed_error.message,
                        ))
                    }
                    "TooManyLoadBalancers" => {
                        return RusotoError::Service(CreateLoadBalancerError::TooManyAccessPoints(
                            parsed_error.message,
                        ))
                    }
                    "TooManyTags" => {
                        return RusotoError::Service(CreateLoadBalancerError::TooManyTags(
                            parsed_error.message,
                        ))
                    }
                    "UnsupportedProtocol" => {
                        return RusotoError::Service(CreateLoadBalancerError::UnsupportedProtocol(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn refine(err: RusotoError<std::convert::Infallible>) -> RusotoError<CreateLoadBalancerError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for CreateLoadBalancerError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateLoadBalancerError::CertificateNotFound(ref cause) => write!(f, "{}", cause),
            CreateLoadBalancerError::DuplicateAccessPointName(ref cause) => write!(f, "{}", cause),
            CreateLoadBalancerError::DuplicateTagKeys(ref cause) => write!(f, "{}", cause),
            CreateLoadBalancerError::InvalidConfigurationRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateLoadBalancerError::InvalidScheme(ref cause) => write!(f, "{}", cause),
            CreateLoadBalancerError::InvalidSecurityGroup(ref cause) => write!(f, "{}", cause),
            CreateLoadBalancerError::InvalidSubnet(ref cause) => write!(f, "{}", cause),
            CreateLoadBalancerError::OperationNotPermitted(ref cause) => write!(f, "{}", cause),
            CreateLoadBalancerError::SubnetNotFound(ref cause) => write!(f, "{}", cause),
            CreateLoadBalancerError::TooManyAccessPoints(ref cause) => write!(f, "{}", cause),
            CreateLoadBalancerError::TooManyTags(ref cause) => write!(f, "{}", cause),
            CreateLoadBalancerError::UnsupportedProtocol(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateLoadBalancerError {}
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
}

impl CreateLoadBalancerListenersError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateLoadBalancerListenersError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "LoadBalancerNotFound" => {
                        return RusotoError::Service(
                            CreateLoadBalancerListenersError::AccessPointNotFound(
                                parsed_error.message,
                            ),
                        )
                    }
                    "CertificateNotFound" => {
                        return RusotoError::Service(
                            CreateLoadBalancerListenersError::CertificateNotFound(
                                parsed_error.message,
                            ),
                        )
                    }
                    "DuplicateListener" => {
                        return RusotoError::Service(
                            CreateLoadBalancerListenersError::DuplicateListener(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidConfigurationRequest" => {
                        return RusotoError::Service(
                            CreateLoadBalancerListenersError::InvalidConfigurationRequest(
                                parsed_error.message,
                            ),
                        )
                    }
                    "UnsupportedProtocol" => {
                        return RusotoError::Service(
                            CreateLoadBalancerListenersError::UnsupportedProtocol(
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

    fn refine(
        err: RusotoError<std::convert::Infallible>,
    ) -> RusotoError<CreateLoadBalancerListenersError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for CreateLoadBalancerListenersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateLoadBalancerListenersError::AccessPointNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateLoadBalancerListenersError::CertificateNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateLoadBalancerListenersError::DuplicateListener(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateLoadBalancerListenersError::InvalidConfigurationRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateLoadBalancerListenersError::UnsupportedProtocol(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for CreateLoadBalancerListenersError {}
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
}

impl CreateLoadBalancerPolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateLoadBalancerPolicyError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "LoadBalancerNotFound" => {
                        return RusotoError::Service(
                            CreateLoadBalancerPolicyError::AccessPointNotFound(
                                parsed_error.message,
                            ),
                        )
                    }
                    "DuplicatePolicyName" => {
                        return RusotoError::Service(
                            CreateLoadBalancerPolicyError::DuplicatePolicyName(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidConfigurationRequest" => {
                        return RusotoError::Service(
                            CreateLoadBalancerPolicyError::InvalidConfigurationRequest(
                                parsed_error.message,
                            ),
                        )
                    }
                    "PolicyTypeNotFound" => {
                        return RusotoError::Service(
                            CreateLoadBalancerPolicyError::PolicyTypeNotFound(parsed_error.message),
                        )
                    }
                    "TooManyPolicies" => {
                        return RusotoError::Service(
                            CreateLoadBalancerPolicyError::TooManyPolicies(parsed_error.message),
                        )
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn refine(
        err: RusotoError<std::convert::Infallible>,
    ) -> RusotoError<CreateLoadBalancerPolicyError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for CreateLoadBalancerPolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateLoadBalancerPolicyError::AccessPointNotFound(ref cause) => write!(f, "{}", cause),
            CreateLoadBalancerPolicyError::DuplicatePolicyName(ref cause) => write!(f, "{}", cause),
            CreateLoadBalancerPolicyError::InvalidConfigurationRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateLoadBalancerPolicyError::PolicyTypeNotFound(ref cause) => write!(f, "{}", cause),
            CreateLoadBalancerPolicyError::TooManyPolicies(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateLoadBalancerPolicyError {}
/// Errors returned by DeleteLoadBalancer
#[derive(Debug, PartialEq)]
pub enum DeleteLoadBalancerError {}

impl DeleteLoadBalancerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteLoadBalancerError> {
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

    fn refine(err: RusotoError<std::convert::Infallible>) -> RusotoError<DeleteLoadBalancerError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DeleteLoadBalancerError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for DeleteLoadBalancerError {}
/// Errors returned by DeleteLoadBalancerListeners
#[derive(Debug, PartialEq)]
pub enum DeleteLoadBalancerListenersError {
    /// <p>The specified load balancer does not exist.</p>
    AccessPointNotFound(String),
}

impl DeleteLoadBalancerListenersError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteLoadBalancerListenersError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "LoadBalancerNotFound" => {
                        return RusotoError::Service(
                            DeleteLoadBalancerListenersError::AccessPointNotFound(
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

    fn refine(
        err: RusotoError<std::convert::Infallible>,
    ) -> RusotoError<DeleteLoadBalancerListenersError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DeleteLoadBalancerListenersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteLoadBalancerListenersError::AccessPointNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DeleteLoadBalancerListenersError {}
/// Errors returned by DeleteLoadBalancerPolicy
#[derive(Debug, PartialEq)]
pub enum DeleteLoadBalancerPolicyError {
    /// <p>The specified load balancer does not exist.</p>
    AccessPointNotFound(String),
    /// <p>The requested configuration change is not valid.</p>
    InvalidConfigurationRequest(String),
}

impl DeleteLoadBalancerPolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteLoadBalancerPolicyError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "LoadBalancerNotFound" => {
                        return RusotoError::Service(
                            DeleteLoadBalancerPolicyError::AccessPointNotFound(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidConfigurationRequest" => {
                        return RusotoError::Service(
                            DeleteLoadBalancerPolicyError::InvalidConfigurationRequest(
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

    fn refine(
        err: RusotoError<std::convert::Infallible>,
    ) -> RusotoError<DeleteLoadBalancerPolicyError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DeleteLoadBalancerPolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteLoadBalancerPolicyError::AccessPointNotFound(ref cause) => write!(f, "{}", cause),
            DeleteLoadBalancerPolicyError::InvalidConfigurationRequest(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DeleteLoadBalancerPolicyError {}
/// Errors returned by DeregisterInstancesFromLoadBalancer
#[derive(Debug, PartialEq)]
pub enum DeregisterInstancesFromLoadBalancerError {
    /// <p>The specified load balancer does not exist.</p>
    AccessPointNotFound(String),
    /// <p>The specified endpoint is not valid.</p>
    InvalidEndPoint(String),
}

impl DeregisterInstancesFromLoadBalancerError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeregisterInstancesFromLoadBalancerError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "LoadBalancerNotFound" => {
                        return RusotoError::Service(
                            DeregisterInstancesFromLoadBalancerError::AccessPointNotFound(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidInstance" => {
                        return RusotoError::Service(
                            DeregisterInstancesFromLoadBalancerError::InvalidEndPoint(
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

    fn refine(
        err: RusotoError<std::convert::Infallible>,
    ) -> RusotoError<DeregisterInstancesFromLoadBalancerError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DeregisterInstancesFromLoadBalancerError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeregisterInstancesFromLoadBalancerError::AccessPointNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            DeregisterInstancesFromLoadBalancerError::InvalidEndPoint(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DeregisterInstancesFromLoadBalancerError {}
/// Errors returned by DescribeAccountLimits
#[derive(Debug, PartialEq)]
pub enum DescribeAccountLimitsError {}

impl DescribeAccountLimitsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeAccountLimitsError> {
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

    fn refine(
        err: RusotoError<std::convert::Infallible>,
    ) -> RusotoError<DescribeAccountLimitsError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DescribeAccountLimitsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for DescribeAccountLimitsError {}
/// Errors returned by DescribeInstanceHealth
#[derive(Debug, PartialEq)]
pub enum DescribeInstanceHealthError {
    /// <p>The specified load balancer does not exist.</p>
    AccessPointNotFound(String),
    /// <p>The specified endpoint is not valid.</p>
    InvalidEndPoint(String),
}

impl DescribeInstanceHealthError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeInstanceHealthError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "LoadBalancerNotFound" => {
                        return RusotoError::Service(
                            DescribeInstanceHealthError::AccessPointNotFound(parsed_error.message),
                        )
                    }
                    "InvalidInstance" => {
                        return RusotoError::Service(DescribeInstanceHealthError::InvalidEndPoint(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn refine(
        err: RusotoError<std::convert::Infallible>,
    ) -> RusotoError<DescribeInstanceHealthError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DescribeInstanceHealthError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeInstanceHealthError::AccessPointNotFound(ref cause) => write!(f, "{}", cause),
            DescribeInstanceHealthError::InvalidEndPoint(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeInstanceHealthError {}
/// Errors returned by DescribeLoadBalancerAttributes
#[derive(Debug, PartialEq)]
pub enum DescribeLoadBalancerAttributesError {
    /// <p>The specified load balancer does not exist.</p>
    AccessPointNotFound(String),
    /// <p>The specified load balancer attribute does not exist.</p>
    LoadBalancerAttributeNotFound(String),
}

impl DescribeLoadBalancerAttributesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeLoadBalancerAttributesError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "LoadBalancerNotFound" => {
                        return RusotoError::Service(
                            DescribeLoadBalancerAttributesError::AccessPointNotFound(
                                parsed_error.message,
                            ),
                        )
                    }
                    "LoadBalancerAttributeNotFound" => {
                        return RusotoError::Service(
                            DescribeLoadBalancerAttributesError::LoadBalancerAttributeNotFound(
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

    fn refine(
        err: RusotoError<std::convert::Infallible>,
    ) -> RusotoError<DescribeLoadBalancerAttributesError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DescribeLoadBalancerAttributesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeLoadBalancerAttributesError::AccessPointNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeLoadBalancerAttributesError::LoadBalancerAttributeNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeLoadBalancerAttributesError {}
/// Errors returned by DescribeLoadBalancerPolicies
#[derive(Debug, PartialEq)]
pub enum DescribeLoadBalancerPoliciesError {
    /// <p>The specified load balancer does not exist.</p>
    AccessPointNotFound(String),
    /// <p>One or more of the specified policies do not exist.</p>
    PolicyNotFound(String),
}

impl DescribeLoadBalancerPoliciesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeLoadBalancerPoliciesError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "LoadBalancerNotFound" => {
                        return RusotoError::Service(
                            DescribeLoadBalancerPoliciesError::AccessPointNotFound(
                                parsed_error.message,
                            ),
                        )
                    }
                    "PolicyNotFound" => {
                        return RusotoError::Service(
                            DescribeLoadBalancerPoliciesError::PolicyNotFound(parsed_error.message),
                        )
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn refine(
        err: RusotoError<std::convert::Infallible>,
    ) -> RusotoError<DescribeLoadBalancerPoliciesError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DescribeLoadBalancerPoliciesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeLoadBalancerPoliciesError::AccessPointNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeLoadBalancerPoliciesError::PolicyNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeLoadBalancerPoliciesError {}
/// Errors returned by DescribeLoadBalancerPolicyTypes
#[derive(Debug, PartialEq)]
pub enum DescribeLoadBalancerPolicyTypesError {
    /// <p>One or more of the specified policy types do not exist.</p>
    PolicyTypeNotFound(String),
}

impl DescribeLoadBalancerPolicyTypesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeLoadBalancerPolicyTypesError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "PolicyTypeNotFound" => {
                        return RusotoError::Service(
                            DescribeLoadBalancerPolicyTypesError::PolicyTypeNotFound(
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

    fn refine(
        err: RusotoError<std::convert::Infallible>,
    ) -> RusotoError<DescribeLoadBalancerPolicyTypesError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DescribeLoadBalancerPolicyTypesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeLoadBalancerPolicyTypesError::PolicyTypeNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeLoadBalancerPolicyTypesError {}
/// Errors returned by DescribeLoadBalancers
#[derive(Debug, PartialEq)]
pub enum DescribeLoadBalancersError {
    /// <p>The specified load balancer does not exist.</p>
    AccessPointNotFound(String),
    /// <p>A request made by Elastic Load Balancing to another service exceeds the maximum request rate permitted for your account.</p>
    DependencyThrottle(String),
}

impl DescribeLoadBalancersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeLoadBalancersError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "LoadBalancerNotFound" => {
                        return RusotoError::Service(
                            DescribeLoadBalancersError::AccessPointNotFound(parsed_error.message),
                        )
                    }
                    "DependencyThrottle" => {
                        return RusotoError::Service(
                            DescribeLoadBalancersError::DependencyThrottle(parsed_error.message),
                        )
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn refine(
        err: RusotoError<std::convert::Infallible>,
    ) -> RusotoError<DescribeLoadBalancersError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DescribeLoadBalancersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeLoadBalancersError::AccessPointNotFound(ref cause) => write!(f, "{}", cause),
            DescribeLoadBalancersError::DependencyThrottle(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeLoadBalancersError {}
/// Errors returned by DescribeTags
#[derive(Debug, PartialEq)]
pub enum DescribeTagsError {
    /// <p>The specified load balancer does not exist.</p>
    AccessPointNotFound(String),
}

impl DescribeTagsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeTagsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "LoadBalancerNotFound" => {
                        return RusotoError::Service(DescribeTagsError::AccessPointNotFound(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn refine(err: RusotoError<std::convert::Infallible>) -> RusotoError<DescribeTagsError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DescribeTagsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeTagsError::AccessPointNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeTagsError {}
/// Errors returned by DetachLoadBalancerFromSubnets
#[derive(Debug, PartialEq)]
pub enum DetachLoadBalancerFromSubnetsError {
    /// <p>The specified load balancer does not exist.</p>
    AccessPointNotFound(String),
    /// <p>The requested configuration change is not valid.</p>
    InvalidConfigurationRequest(String),
}

impl DetachLoadBalancerFromSubnetsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DetachLoadBalancerFromSubnetsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "LoadBalancerNotFound" => {
                        return RusotoError::Service(
                            DetachLoadBalancerFromSubnetsError::AccessPointNotFound(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidConfigurationRequest" => {
                        return RusotoError::Service(
                            DetachLoadBalancerFromSubnetsError::InvalidConfigurationRequest(
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

    fn refine(
        err: RusotoError<std::convert::Infallible>,
    ) -> RusotoError<DetachLoadBalancerFromSubnetsError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DetachLoadBalancerFromSubnetsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DetachLoadBalancerFromSubnetsError::AccessPointNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            DetachLoadBalancerFromSubnetsError::InvalidConfigurationRequest(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DetachLoadBalancerFromSubnetsError {}
/// Errors returned by DisableAvailabilityZonesForLoadBalancer
#[derive(Debug, PartialEq)]
pub enum DisableAvailabilityZonesForLoadBalancerError {
    /// <p>The specified load balancer does not exist.</p>
    AccessPointNotFound(String),
    /// <p>The requested configuration change is not valid.</p>
    InvalidConfigurationRequest(String),
}

impl DisableAvailabilityZonesForLoadBalancerError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DisableAvailabilityZonesForLoadBalancerError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "LoadBalancerNotFound" => {
                        return RusotoError::Service(
                            DisableAvailabilityZonesForLoadBalancerError::AccessPointNotFound(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidConfigurationRequest" => return RusotoError::Service(
                        DisableAvailabilityZonesForLoadBalancerError::InvalidConfigurationRequest(
                            parsed_error.message,
                        ),
                    ),
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn refine(
        err: RusotoError<std::convert::Infallible>,
    ) -> RusotoError<DisableAvailabilityZonesForLoadBalancerError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DisableAvailabilityZonesForLoadBalancerError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisableAvailabilityZonesForLoadBalancerError::AccessPointNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            DisableAvailabilityZonesForLoadBalancerError::InvalidConfigurationRequest(
                ref cause,
            ) => write!(f, "{}", cause),
        }
    }
}
impl Error for DisableAvailabilityZonesForLoadBalancerError {}
/// Errors returned by EnableAvailabilityZonesForLoadBalancer
#[derive(Debug, PartialEq)]
pub enum EnableAvailabilityZonesForLoadBalancerError {
    /// <p>The specified load balancer does not exist.</p>
    AccessPointNotFound(String),
}

impl EnableAvailabilityZonesForLoadBalancerError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<EnableAvailabilityZonesForLoadBalancerError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "LoadBalancerNotFound" => {
                        return RusotoError::Service(
                            EnableAvailabilityZonesForLoadBalancerError::AccessPointNotFound(
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

    fn refine(
        err: RusotoError<std::convert::Infallible>,
    ) -> RusotoError<EnableAvailabilityZonesForLoadBalancerError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for EnableAvailabilityZonesForLoadBalancerError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EnableAvailabilityZonesForLoadBalancerError::AccessPointNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for EnableAvailabilityZonesForLoadBalancerError {}
/// Errors returned by ModifyLoadBalancerAttributes
#[derive(Debug, PartialEq)]
pub enum ModifyLoadBalancerAttributesError {
    /// <p>The specified load balancer does not exist.</p>
    AccessPointNotFound(String),
    /// <p>The requested configuration change is not valid.</p>
    InvalidConfigurationRequest(String),
    /// <p>The specified load balancer attribute does not exist.</p>
    LoadBalancerAttributeNotFound(String),
}

impl ModifyLoadBalancerAttributesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ModifyLoadBalancerAttributesError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "LoadBalancerNotFound" => {
                        return RusotoError::Service(
                            ModifyLoadBalancerAttributesError::AccessPointNotFound(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidConfigurationRequest" => {
                        return RusotoError::Service(
                            ModifyLoadBalancerAttributesError::InvalidConfigurationRequest(
                                parsed_error.message,
                            ),
                        )
                    }
                    "LoadBalancerAttributeNotFound" => {
                        return RusotoError::Service(
                            ModifyLoadBalancerAttributesError::LoadBalancerAttributeNotFound(
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

    fn refine(
        err: RusotoError<std::convert::Infallible>,
    ) -> RusotoError<ModifyLoadBalancerAttributesError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for ModifyLoadBalancerAttributesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ModifyLoadBalancerAttributesError::AccessPointNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            ModifyLoadBalancerAttributesError::InvalidConfigurationRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            ModifyLoadBalancerAttributesError::LoadBalancerAttributeNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ModifyLoadBalancerAttributesError {}
/// Errors returned by RegisterInstancesWithLoadBalancer
#[derive(Debug, PartialEq)]
pub enum RegisterInstancesWithLoadBalancerError {
    /// <p>The specified load balancer does not exist.</p>
    AccessPointNotFound(String),
    /// <p>The specified endpoint is not valid.</p>
    InvalidEndPoint(String),
}

impl RegisterInstancesWithLoadBalancerError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<RegisterInstancesWithLoadBalancerError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "LoadBalancerNotFound" => {
                        return RusotoError::Service(
                            RegisterInstancesWithLoadBalancerError::AccessPointNotFound(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidInstance" => {
                        return RusotoError::Service(
                            RegisterInstancesWithLoadBalancerError::InvalidEndPoint(
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

    fn refine(
        err: RusotoError<std::convert::Infallible>,
    ) -> RusotoError<RegisterInstancesWithLoadBalancerError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for RegisterInstancesWithLoadBalancerError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RegisterInstancesWithLoadBalancerError::AccessPointNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            RegisterInstancesWithLoadBalancerError::InvalidEndPoint(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for RegisterInstancesWithLoadBalancerError {}
/// Errors returned by RemoveTags
#[derive(Debug, PartialEq)]
pub enum RemoveTagsError {
    /// <p>The specified load balancer does not exist.</p>
    AccessPointNotFound(String),
}

impl RemoveTagsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RemoveTagsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "LoadBalancerNotFound" => {
                        return RusotoError::Service(RemoveTagsError::AccessPointNotFound(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn refine(err: RusotoError<std::convert::Infallible>) -> RusotoError<RemoveTagsError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for RemoveTagsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RemoveTagsError::AccessPointNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RemoveTagsError {}
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
}

impl SetLoadBalancerListenerSSLCertificateError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<SetLoadBalancerListenerSSLCertificateError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "LoadBalancerNotFound" => {
                        return RusotoError::Service(
                            SetLoadBalancerListenerSSLCertificateError::AccessPointNotFound(
                                parsed_error.message,
                            ),
                        )
                    }
                    "CertificateNotFound" => {
                        return RusotoError::Service(
                            SetLoadBalancerListenerSSLCertificateError::CertificateNotFound(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidConfigurationRequest" => {
                        return RusotoError::Service(
                            SetLoadBalancerListenerSSLCertificateError::InvalidConfigurationRequest(
                                parsed_error.message,
                            ),
                        )
                    }
                    "ListenerNotFound" => {
                        return RusotoError::Service(
                            SetLoadBalancerListenerSSLCertificateError::ListenerNotFound(
                                parsed_error.message,
                            ),
                        )
                    }
                    "UnsupportedProtocol" => {
                        return RusotoError::Service(
                            SetLoadBalancerListenerSSLCertificateError::UnsupportedProtocol(
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

    fn refine(
        err: RusotoError<std::convert::Infallible>,
    ) -> RusotoError<SetLoadBalancerListenerSSLCertificateError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for SetLoadBalancerListenerSSLCertificateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SetLoadBalancerListenerSSLCertificateError::AccessPointNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            SetLoadBalancerListenerSSLCertificateError::CertificateNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            SetLoadBalancerListenerSSLCertificateError::InvalidConfigurationRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            SetLoadBalancerListenerSSLCertificateError::ListenerNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            SetLoadBalancerListenerSSLCertificateError::UnsupportedProtocol(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for SetLoadBalancerListenerSSLCertificateError {}
/// Errors returned by SetLoadBalancerPoliciesForBackendServer
#[derive(Debug, PartialEq)]
pub enum SetLoadBalancerPoliciesForBackendServerError {
    /// <p>The specified load balancer does not exist.</p>
    AccessPointNotFound(String),
    /// <p>The requested configuration change is not valid.</p>
    InvalidConfigurationRequest(String),
    /// <p>One or more of the specified policies do not exist.</p>
    PolicyNotFound(String),
}

impl SetLoadBalancerPoliciesForBackendServerError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<SetLoadBalancerPoliciesForBackendServerError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "LoadBalancerNotFound" => {
                        return RusotoError::Service(
                            SetLoadBalancerPoliciesForBackendServerError::AccessPointNotFound(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidConfigurationRequest" => return RusotoError::Service(
                        SetLoadBalancerPoliciesForBackendServerError::InvalidConfigurationRequest(
                            parsed_error.message,
                        ),
                    ),
                    "PolicyNotFound" => {
                        return RusotoError::Service(
                            SetLoadBalancerPoliciesForBackendServerError::PolicyNotFound(
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

    fn refine(
        err: RusotoError<std::convert::Infallible>,
    ) -> RusotoError<SetLoadBalancerPoliciesForBackendServerError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for SetLoadBalancerPoliciesForBackendServerError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SetLoadBalancerPoliciesForBackendServerError::AccessPointNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            SetLoadBalancerPoliciesForBackendServerError::InvalidConfigurationRequest(
                ref cause,
            ) => write!(f, "{}", cause),
            SetLoadBalancerPoliciesForBackendServerError::PolicyNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for SetLoadBalancerPoliciesForBackendServerError {}
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
}

impl SetLoadBalancerPoliciesOfListenerError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<SetLoadBalancerPoliciesOfListenerError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "LoadBalancerNotFound" => {
                        return RusotoError::Service(
                            SetLoadBalancerPoliciesOfListenerError::AccessPointNotFound(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidConfigurationRequest" => {
                        return RusotoError::Service(
                            SetLoadBalancerPoliciesOfListenerError::InvalidConfigurationRequest(
                                parsed_error.message,
                            ),
                        )
                    }
                    "ListenerNotFound" => {
                        return RusotoError::Service(
                            SetLoadBalancerPoliciesOfListenerError::ListenerNotFound(
                                parsed_error.message,
                            ),
                        )
                    }
                    "PolicyNotFound" => {
                        return RusotoError::Service(
                            SetLoadBalancerPoliciesOfListenerError::PolicyNotFound(
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

    fn refine(
        err: RusotoError<std::convert::Infallible>,
    ) -> RusotoError<SetLoadBalancerPoliciesOfListenerError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for SetLoadBalancerPoliciesOfListenerError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SetLoadBalancerPoliciesOfListenerError::AccessPointNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            SetLoadBalancerPoliciesOfListenerError::InvalidConfigurationRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            SetLoadBalancerPoliciesOfListenerError::ListenerNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            SetLoadBalancerPoliciesOfListenerError::PolicyNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for SetLoadBalancerPoliciesOfListenerError {}
/// Trait representing the capabilities of the Elastic Load Balancing API. Elastic Load Balancing clients implement this trait.
#[async_trait]
pub trait Elb {
    /// <p>Adds the specified tags to the specified load balancer. Each load balancer can have a maximum of 10 tags.</p> <p>Each tag consists of a key and an optional value. If a tag with the same key is already associated with the load balancer, <code>AddTags</code> updates its value.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/classic/add-remove-tags.html">Tag Your Classic Load Balancer</a> in the <i>Classic Load Balancers Guide</i>.</p>
    async fn add_tags(
        &self,
        input: AddTagsInput,
    ) -> Result<AddTagsOutput, RusotoError<AddTagsError>>;

    /// <p>Associates one or more security groups with your load balancer in a virtual private cloud (VPC). The specified security groups override the previously associated security groups.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/classic/elb-security-groups.html#elb-vpc-security-groups">Security Groups for Load Balancers in a VPC</a> in the <i>Classic Load Balancers Guide</i>.</p>
    async fn apply_security_groups_to_load_balancer(
        &self,
        input: ApplySecurityGroupsToLoadBalancerInput,
    ) -> Result<
        ApplySecurityGroupsToLoadBalancerOutput,
        RusotoError<ApplySecurityGroupsToLoadBalancerError>,
    >;

    /// <p>Adds one or more subnets to the set of configured subnets for the specified load balancer.</p> <p>The load balancer evenly distributes requests across all registered subnets. For more information, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/classic/elb-manage-subnets.html">Add or Remove Subnets for Your Load Balancer in a VPC</a> in the <i>Classic Load Balancers Guide</i>.</p>
    async fn attach_load_balancer_to_subnets(
        &self,
        input: AttachLoadBalancerToSubnetsInput,
    ) -> Result<AttachLoadBalancerToSubnetsOutput, RusotoError<AttachLoadBalancerToSubnetsError>>;

    /// <p>Specifies the health check settings to use when evaluating the health state of your EC2 instances.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/classic/elb-healthchecks.html">Configure Health Checks for Your Load Balancer</a> in the <i>Classic Load Balancers Guide</i>.</p>
    async fn configure_health_check(
        &self,
        input: ConfigureHealthCheckInput,
    ) -> Result<ConfigureHealthCheckOutput, RusotoError<ConfigureHealthCheckError>>;

    /// <p>Generates a stickiness policy with sticky session lifetimes that follow that of an application-generated cookie. This policy can be associated only with HTTP/HTTPS listeners.</p> <p>This policy is similar to the policy created by <a>CreateLBCookieStickinessPolicy</a>, except that the lifetime of the special Elastic Load Balancing cookie, <code>AWSELB</code>, follows the lifetime of the application-generated cookie specified in the policy configuration. The load balancer only inserts a new stickiness cookie when the application response includes a new application cookie.</p> <p>If the application cookie is explicitly removed or expires, the session stops being sticky until a new application cookie is issued.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/classic/elb-sticky-sessions.html#enable-sticky-sessions-application">Application-Controlled Session Stickiness</a> in the <i>Classic Load Balancers Guide</i>.</p>
    async fn create_app_cookie_stickiness_policy(
        &self,
        input: CreateAppCookieStickinessPolicyInput,
    ) -> Result<
        CreateAppCookieStickinessPolicyOutput,
        RusotoError<CreateAppCookieStickinessPolicyError>,
    >;

    /// <p>Generates a stickiness policy with sticky session lifetimes controlled by the lifetime of the browser (user-agent) or a specified expiration period. This policy can be associated only with HTTP/HTTPS listeners.</p> <p>When a load balancer implements this policy, the load balancer uses a special cookie to track the instance for each request. When the load balancer receives a request, it first checks to see if this cookie is present in the request. If so, the load balancer sends the request to the application server specified in the cookie. If not, the load balancer sends the request to a server that is chosen based on the existing load-balancing algorithm.</p> <p>A cookie is inserted into the response for binding subsequent requests from the same user to that server. The validity of the cookie is based on the cookie expiration time, which is specified in the policy configuration.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/classic/elb-sticky-sessions.html#enable-sticky-sessions-duration">Duration-Based Session Stickiness</a> in the <i>Classic Load Balancers Guide</i>.</p>
    async fn create_lb_cookie_stickiness_policy(
        &self,
        input: CreateLBCookieStickinessPolicyInput,
    ) -> Result<
        CreateLBCookieStickinessPolicyOutput,
        RusotoError<CreateLBCookieStickinessPolicyError>,
    >;

    /// <p>Creates a Classic Load Balancer.</p> <p>You can add listeners, security groups, subnets, and tags when you create your load balancer, or you can add them later using <a>CreateLoadBalancerListeners</a>, <a>ApplySecurityGroupsToLoadBalancer</a>, <a>AttachLoadBalancerToSubnets</a>, and <a>AddTags</a>.</p> <p>To describe your current load balancers, see <a>DescribeLoadBalancers</a>. When you are finished with a load balancer, you can delete it using <a>DeleteLoadBalancer</a>.</p> <p>You can create up to 20 load balancers per region per account. You can request an increase for the number of load balancers for your account. For more information, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/classic/elb-limits.html">Limits for Your Classic Load Balancer</a> in the <i>Classic Load Balancers Guide</i>.</p>
    async fn create_load_balancer(
        &self,
        input: CreateAccessPointInput,
    ) -> Result<CreateAccessPointOutput, RusotoError<CreateLoadBalancerError>>;

    /// <p>Creates one or more listeners for the specified load balancer. If a listener with the specified port does not already exist, it is created; otherwise, the properties of the new listener must match the properties of the existing listener.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/classic/elb-listener-config.html">Listeners for Your Classic Load Balancer</a> in the <i>Classic Load Balancers Guide</i>.</p>
    async fn create_load_balancer_listeners(
        &self,
        input: CreateLoadBalancerListenerInput,
    ) -> Result<CreateLoadBalancerListenerOutput, RusotoError<CreateLoadBalancerListenersError>>;

    /// <p>Creates a policy with the specified attributes for the specified load balancer.</p> <p>Policies are settings that are saved for your load balancer and that can be applied to the listener or the application server, depending on the policy type.</p>
    async fn create_load_balancer_policy(
        &self,
        input: CreateLoadBalancerPolicyInput,
    ) -> Result<CreateLoadBalancerPolicyOutput, RusotoError<CreateLoadBalancerPolicyError>>;

    /// <p>Deletes the specified load balancer.</p> <p>If you are attempting to recreate a load balancer, you must reconfigure all settings. The DNS name associated with a deleted load balancer are no longer usable. The name and associated DNS record of the deleted load balancer no longer exist and traffic sent to any of its IP addresses is no longer delivered to your instances.</p> <p>If the load balancer does not exist or has already been deleted, the call to <code>DeleteLoadBalancer</code> still succeeds.</p>
    async fn delete_load_balancer(
        &self,
        input: DeleteAccessPointInput,
    ) -> Result<DeleteAccessPointOutput, RusotoError<DeleteLoadBalancerError>>;

    /// <p>Deletes the specified listeners from the specified load balancer.</p>
    async fn delete_load_balancer_listeners(
        &self,
        input: DeleteLoadBalancerListenerInput,
    ) -> Result<DeleteLoadBalancerListenerOutput, RusotoError<DeleteLoadBalancerListenersError>>;

    /// <p>Deletes the specified policy from the specified load balancer. This policy must not be enabled for any listeners.</p>
    async fn delete_load_balancer_policy(
        &self,
        input: DeleteLoadBalancerPolicyInput,
    ) -> Result<DeleteLoadBalancerPolicyOutput, RusotoError<DeleteLoadBalancerPolicyError>>;

    /// <p>Deregisters the specified instances from the specified load balancer. After the instance is deregistered, it no longer receives traffic from the load balancer.</p> <p>You can use <a>DescribeLoadBalancers</a> to verify that the instance is deregistered from the load balancer.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/classic/elb-deregister-register-instances.html">Register or De-Register EC2 Instances</a> in the <i>Classic Load Balancers Guide</i>.</p>
    async fn deregister_instances_from_load_balancer(
        &self,
        input: DeregisterEndPointsInput,
    ) -> Result<DeregisterEndPointsOutput, RusotoError<DeregisterInstancesFromLoadBalancerError>>;

    /// <p>Describes the current Elastic Load Balancing resource limits for your AWS account.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/classic/elb-limits.html">Limits for Your Classic Load Balancer</a> in the <i>Classic Load Balancers Guide</i>.</p>
    async fn describe_account_limits(
        &self,
        input: DescribeAccountLimitsInput,
    ) -> Result<DescribeAccountLimitsOutput, RusotoError<DescribeAccountLimitsError>>;

    /// <p>Describes the state of the specified instances with respect to the specified load balancer. If no instances are specified, the call describes the state of all instances that are currently registered with the load balancer. If instances are specified, their state is returned even if they are no longer registered with the load balancer. The state of terminated instances is not returned.</p>
    async fn describe_instance_health(
        &self,
        input: DescribeEndPointStateInput,
    ) -> Result<DescribeEndPointStateOutput, RusotoError<DescribeInstanceHealthError>>;

    /// <p>Describes the attributes for the specified load balancer.</p>
    async fn describe_load_balancer_attributes(
        &self,
        input: DescribeLoadBalancerAttributesInput,
    ) -> Result<
        DescribeLoadBalancerAttributesOutput,
        RusotoError<DescribeLoadBalancerAttributesError>,
    >;

    /// <p>Describes the specified policies.</p> <p>If you specify a load balancer name, the action returns the descriptions of all policies created for the load balancer. If you specify a policy name associated with your load balancer, the action returns the description of that policy. If you don't specify a load balancer name, the action returns descriptions of the specified sample policies, or descriptions of all sample policies. The names of the sample policies have the <code>ELBSample-</code> prefix.</p>
    async fn describe_load_balancer_policies(
        &self,
        input: DescribeLoadBalancerPoliciesInput,
    ) -> Result<DescribeLoadBalancerPoliciesOutput, RusotoError<DescribeLoadBalancerPoliciesError>>;

    /// <p>Describes the specified load balancer policy types or all load balancer policy types.</p> <p>The description of each type indicates how it can be used. For example, some policies can be used only with layer 7 listeners, some policies can be used only with layer 4 listeners, and some policies can be used only with your EC2 instances.</p> <p>You can use <a>CreateLoadBalancerPolicy</a> to create a policy configuration for any of these policy types. Then, depending on the policy type, use either <a>SetLoadBalancerPoliciesOfListener</a> or <a>SetLoadBalancerPoliciesForBackendServer</a> to set the policy.</p>
    async fn describe_load_balancer_policy_types(
        &self,
        input: DescribeLoadBalancerPolicyTypesInput,
    ) -> Result<
        DescribeLoadBalancerPolicyTypesOutput,
        RusotoError<DescribeLoadBalancerPolicyTypesError>,
    >;

    /// <p>Describes the specified the load balancers. If no load balancers are specified, the call describes all of your load balancers.</p>
    async fn describe_load_balancers(
        &self,
        input: DescribeAccessPointsInput,
    ) -> Result<DescribeAccessPointsOutput, RusotoError<DescribeLoadBalancersError>>;

    /// <p>Describes the tags associated with the specified load balancers.</p>
    async fn describe_tags(
        &self,
        input: DescribeTagsInput,
    ) -> Result<DescribeTagsOutput, RusotoError<DescribeTagsError>>;

    /// <p>Removes the specified subnets from the set of configured subnets for the load balancer.</p> <p>After a subnet is removed, all EC2 instances registered with the load balancer in the removed subnet go into the <code>OutOfService</code> state. Then, the load balancer balances the traffic among the remaining routable subnets.</p>
    async fn detach_load_balancer_from_subnets(
        &self,
        input: DetachLoadBalancerFromSubnetsInput,
    ) -> Result<DetachLoadBalancerFromSubnetsOutput, RusotoError<DetachLoadBalancerFromSubnetsError>>;

    /// <p>Removes the specified Availability Zones from the set of Availability Zones for the specified load balancer in EC2-Classic or a default VPC.</p> <p>For load balancers in a non-default VPC, use <a>DetachLoadBalancerFromSubnets</a>.</p> <p>There must be at least one Availability Zone registered with a load balancer at all times. After an Availability Zone is removed, all instances registered with the load balancer that are in the removed Availability Zone go into the <code>OutOfService</code> state. Then, the load balancer attempts to equally balance the traffic among its remaining Availability Zones.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/classic/enable-disable-az.html">Add or Remove Availability Zones</a> in the <i>Classic Load Balancers Guide</i>.</p>
    async fn disable_availability_zones_for_load_balancer(
        &self,
        input: RemoveAvailabilityZonesInput,
    ) -> Result<
        RemoveAvailabilityZonesOutput,
        RusotoError<DisableAvailabilityZonesForLoadBalancerError>,
    >;

    /// <p>Adds the specified Availability Zones to the set of Availability Zones for the specified load balancer in EC2-Classic or a default VPC.</p> <p>For load balancers in a non-default VPC, use <a>AttachLoadBalancerToSubnets</a>.</p> <p>The load balancer evenly distributes requests across all its registered Availability Zones that contain instances. For more information, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/classic/enable-disable-az.html">Add or Remove Availability Zones</a> in the <i>Classic Load Balancers Guide</i>.</p>
    async fn enable_availability_zones_for_load_balancer(
        &self,
        input: AddAvailabilityZonesInput,
    ) -> Result<AddAvailabilityZonesOutput, RusotoError<EnableAvailabilityZonesForLoadBalancerError>>;

    /// <p><p>Modifies the attributes of the specified load balancer.</p> <p>You can modify the load balancer attributes, such as <code>AccessLogs</code>, <code>ConnectionDraining</code>, and <code>CrossZoneLoadBalancing</code> by either enabling or disabling them. Or, you can modify the load balancer attribute <code>ConnectionSettings</code> by specifying an idle connection timeout value for your load balancer.</p> <p>For more information, see the following in the <i>Classic Load Balancers Guide</i>:</p> <ul> <li> <p> <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/classic/enable-disable-crosszone-lb.html">Cross-Zone Load Balancing</a> </p> </li> <li> <p> <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/classic/config-conn-drain.html">Connection Draining</a> </p> </li> <li> <p> <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/classic/access-log-collection.html">Access Logs</a> </p> </li> <li> <p> <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/classic/config-idle-timeout.html">Idle Connection Timeout</a> </p> </li> </ul></p>
    async fn modify_load_balancer_attributes(
        &self,
        input: ModifyLoadBalancerAttributesInput,
    ) -> Result<ModifyLoadBalancerAttributesOutput, RusotoError<ModifyLoadBalancerAttributesError>>;

    /// <p>Adds the specified instances to the specified load balancer.</p> <p>The instance must be a running instance in the same network as the load balancer (EC2-Classic or the same VPC). If you have EC2-Classic instances and a load balancer in a VPC with ClassicLink enabled, you can link the EC2-Classic instances to that VPC and then register the linked EC2-Classic instances with the load balancer in the VPC.</p> <p>Note that <code>RegisterInstanceWithLoadBalancer</code> completes when the request has been registered. Instance registration takes a little time to complete. To check the state of the registered instances, use <a>DescribeLoadBalancers</a> or <a>DescribeInstanceHealth</a>.</p> <p>After the instance is registered, it starts receiving traffic and requests from the load balancer. Any instance that is not in one of the Availability Zones registered for the load balancer is moved to the <code>OutOfService</code> state. If an Availability Zone is added to the load balancer later, any instances registered with the load balancer move to the <code>InService</code> state.</p> <p>To deregister instances from a load balancer, use <a>DeregisterInstancesFromLoadBalancer</a>.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/classic/elb-deregister-register-instances.html">Register or De-Register EC2 Instances</a> in the <i>Classic Load Balancers Guide</i>.</p>
    async fn register_instances_with_load_balancer(
        &self,
        input: RegisterEndPointsInput,
    ) -> Result<RegisterEndPointsOutput, RusotoError<RegisterInstancesWithLoadBalancerError>>;

    /// <p>Removes one or more tags from the specified load balancer.</p>
    async fn remove_tags(
        &self,
        input: RemoveTagsInput,
    ) -> Result<RemoveTagsOutput, RusotoError<RemoveTagsError>>;

    /// <p>Sets the certificate that terminates the specified listener's SSL connections. The specified certificate replaces any prior certificate that was used on the same load balancer and port.</p> <p>For more information about updating your SSL certificate, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/classic/elb-update-ssl-cert.html">Replace the SSL Certificate for Your Load Balancer</a> in the <i>Classic Load Balancers Guide</i>.</p>
    async fn set_load_balancer_listener_ssl_certificate(
        &self,
        input: SetLoadBalancerListenerSSLCertificateInput,
    ) -> Result<
        SetLoadBalancerListenerSSLCertificateOutput,
        RusotoError<SetLoadBalancerListenerSSLCertificateError>,
    >;

    /// <p>Replaces the set of policies associated with the specified port on which the EC2 instance is listening with a new set of policies. At this time, only the back-end server authentication policy type can be applied to the instance ports; this policy type is composed of multiple public key policies.</p> <p>Each time you use <code>SetLoadBalancerPoliciesForBackendServer</code> to enable the policies, use the <code>PolicyNames</code> parameter to list the policies that you want to enable.</p> <p>You can use <a>DescribeLoadBalancers</a> or <a>DescribeLoadBalancerPolicies</a> to verify that the policy is associated with the EC2 instance.</p> <p>For more information about enabling back-end instance authentication, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/classic/elb-create-https-ssl-load-balancer.html#configure_backendauth_clt">Configure Back-end Instance Authentication</a> in the <i>Classic Load Balancers Guide</i>. For more information about Proxy Protocol, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/classic/enable-proxy-protocol.html">Configure Proxy Protocol Support</a> in the <i>Classic Load Balancers Guide</i>.</p>
    async fn set_load_balancer_policies_for_backend_server(
        &self,
        input: SetLoadBalancerPoliciesForBackendServerInput,
    ) -> Result<
        SetLoadBalancerPoliciesForBackendServerOutput,
        RusotoError<SetLoadBalancerPoliciesForBackendServerError>,
    >;

    /// <p>Replaces the current set of policies for the specified load balancer port with the specified set of policies.</p> <p>To enable back-end server authentication, use <a>SetLoadBalancerPoliciesForBackendServer</a>.</p> <p>For more information about setting policies, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/classic/ssl-config-update.html">Update the SSL Negotiation Configuration</a>, <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/classic/elb-sticky-sessions.html#enable-sticky-sessions-duration">Duration-Based Session Stickiness</a>, and <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/classic/elb-sticky-sessions.html#enable-sticky-sessions-application">Application-Controlled Session Stickiness</a> in the <i>Classic Load Balancers Guide</i>.</p>
    async fn set_load_balancer_policies_of_listener(
        &self,
        input: SetLoadBalancerPoliciesOfListenerInput,
    ) -> Result<
        SetLoadBalancerPoliciesOfListenerOutput,
        RusotoError<SetLoadBalancerPoliciesOfListenerError>,
    >;
}
/// A client for the Elastic Load Balancing API.
#[derive(Clone)]
pub struct ElbClient {
    client: Client,
    region: region::Region,
}

impl ElbClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> ElbClient {
        ElbClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> ElbClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        ElbClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> ElbClient {
        ElbClient { client, region }
    }
}

#[async_trait]
impl Elb for ElbClient {
    /// <p>Adds the specified tags to the specified load balancer. Each load balancer can have a maximum of 10 tags.</p> <p>Each tag consists of a key and an optional value. If a tag with the same key is already associated with the load balancer, <code>AddTags</code> updates its value.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/classic/add-remove-tags.html">Tag Your Classic Load Balancer</a> in the <i>Classic Load Balancers Guide</i>.</p>
    async fn add_tags(
        &self,
        input: AddTagsInput,
    ) -> Result<AddTagsOutput, RusotoError<AddTagsError>> {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let params = self.new_params("AddTags");
        let mut params = params;
        AddTagsInputSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request)
            .await
            .map_err(AddTagsError::refine)?;

        let result = AddTagsOutput::default();

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p>Associates one or more security groups with your load balancer in a virtual private cloud (VPC). The specified security groups override the previously associated security groups.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/classic/elb-security-groups.html#elb-vpc-security-groups">Security Groups for Load Balancers in a VPC</a> in the <i>Classic Load Balancers Guide</i>.</p>
    async fn apply_security_groups_to_load_balancer(
        &self,
        input: ApplySecurityGroupsToLoadBalancerInput,
    ) -> Result<
        ApplySecurityGroupsToLoadBalancerOutput,
        RusotoError<ApplySecurityGroupsToLoadBalancerError>,
    > {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let params = self.new_params("ApplySecurityGroupsToLoadBalancer");
        let mut params = params;
        ApplySecurityGroupsToLoadBalancerInputSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request)
            .await
            .map_err(ApplySecurityGroupsToLoadBalancerError::refine)?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = ApplySecurityGroupsToLoadBalancerOutputDeserializer::deserialize(
                "ApplySecurityGroupsToLoadBalancerResult",
                stack,
            )?;
            skip_tree(stack);
            xml_util::end_element(actual_tag_name, stack)?;
            Ok(result)
        })
        .await?;

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p>Adds one or more subnets to the set of configured subnets for the specified load balancer.</p> <p>The load balancer evenly distributes requests across all registered subnets. For more information, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/classic/elb-manage-subnets.html">Add or Remove Subnets for Your Load Balancer in a VPC</a> in the <i>Classic Load Balancers Guide</i>.</p>
    async fn attach_load_balancer_to_subnets(
        &self,
        input: AttachLoadBalancerToSubnetsInput,
    ) -> Result<AttachLoadBalancerToSubnetsOutput, RusotoError<AttachLoadBalancerToSubnetsError>>
    {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let params = self.new_params("AttachLoadBalancerToSubnets");
        let mut params = params;
        AttachLoadBalancerToSubnetsInputSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request)
            .await
            .map_err(AttachLoadBalancerToSubnetsError::refine)?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = AttachLoadBalancerToSubnetsOutputDeserializer::deserialize(
                "AttachLoadBalancerToSubnetsResult",
                stack,
            )?;
            skip_tree(stack);
            xml_util::end_element(actual_tag_name, stack)?;
            Ok(result)
        })
        .await?;

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p>Specifies the health check settings to use when evaluating the health state of your EC2 instances.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/classic/elb-healthchecks.html">Configure Health Checks for Your Load Balancer</a> in the <i>Classic Load Balancers Guide</i>.</p>
    async fn configure_health_check(
        &self,
        input: ConfigureHealthCheckInput,
    ) -> Result<ConfigureHealthCheckOutput, RusotoError<ConfigureHealthCheckError>> {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let params = self.new_params("ConfigureHealthCheck");
        let mut params = params;
        ConfigureHealthCheckInputSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request)
            .await
            .map_err(ConfigureHealthCheckError::refine)?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = ConfigureHealthCheckOutputDeserializer::deserialize(
                "ConfigureHealthCheckResult",
                stack,
            )?;
            skip_tree(stack);
            xml_util::end_element(actual_tag_name, stack)?;
            Ok(result)
        })
        .await?;

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p>Generates a stickiness policy with sticky session lifetimes that follow that of an application-generated cookie. This policy can be associated only with HTTP/HTTPS listeners.</p> <p>This policy is similar to the policy created by <a>CreateLBCookieStickinessPolicy</a>, except that the lifetime of the special Elastic Load Balancing cookie, <code>AWSELB</code>, follows the lifetime of the application-generated cookie specified in the policy configuration. The load balancer only inserts a new stickiness cookie when the application response includes a new application cookie.</p> <p>If the application cookie is explicitly removed or expires, the session stops being sticky until a new application cookie is issued.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/classic/elb-sticky-sessions.html#enable-sticky-sessions-application">Application-Controlled Session Stickiness</a> in the <i>Classic Load Balancers Guide</i>.</p>
    async fn create_app_cookie_stickiness_policy(
        &self,
        input: CreateAppCookieStickinessPolicyInput,
    ) -> Result<
        CreateAppCookieStickinessPolicyOutput,
        RusotoError<CreateAppCookieStickinessPolicyError>,
    > {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let params = self.new_params("CreateAppCookieStickinessPolicy");
        let mut params = params;
        CreateAppCookieStickinessPolicyInputSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request)
            .await
            .map_err(CreateAppCookieStickinessPolicyError::refine)?;

        let result = CreateAppCookieStickinessPolicyOutput::default();

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p>Generates a stickiness policy with sticky session lifetimes controlled by the lifetime of the browser (user-agent) or a specified expiration period. This policy can be associated only with HTTP/HTTPS listeners.</p> <p>When a load balancer implements this policy, the load balancer uses a special cookie to track the instance for each request. When the load balancer receives a request, it first checks to see if this cookie is present in the request. If so, the load balancer sends the request to the application server specified in the cookie. If not, the load balancer sends the request to a server that is chosen based on the existing load-balancing algorithm.</p> <p>A cookie is inserted into the response for binding subsequent requests from the same user to that server. The validity of the cookie is based on the cookie expiration time, which is specified in the policy configuration.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/classic/elb-sticky-sessions.html#enable-sticky-sessions-duration">Duration-Based Session Stickiness</a> in the <i>Classic Load Balancers Guide</i>.</p>
    async fn create_lb_cookie_stickiness_policy(
        &self,
        input: CreateLBCookieStickinessPolicyInput,
    ) -> Result<
        CreateLBCookieStickinessPolicyOutput,
        RusotoError<CreateLBCookieStickinessPolicyError>,
    > {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let params = self.new_params("CreateLBCookieStickinessPolicy");
        let mut params = params;
        CreateLBCookieStickinessPolicyInputSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request)
            .await
            .map_err(CreateLBCookieStickinessPolicyError::refine)?;

        let result = CreateLBCookieStickinessPolicyOutput::default();

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p>Creates a Classic Load Balancer.</p> <p>You can add listeners, security groups, subnets, and tags when you create your load balancer, or you can add them later using <a>CreateLoadBalancerListeners</a>, <a>ApplySecurityGroupsToLoadBalancer</a>, <a>AttachLoadBalancerToSubnets</a>, and <a>AddTags</a>.</p> <p>To describe your current load balancers, see <a>DescribeLoadBalancers</a>. When you are finished with a load balancer, you can delete it using <a>DeleteLoadBalancer</a>.</p> <p>You can create up to 20 load balancers per region per account. You can request an increase for the number of load balancers for your account. For more information, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/classic/elb-limits.html">Limits for Your Classic Load Balancer</a> in the <i>Classic Load Balancers Guide</i>.</p>
    async fn create_load_balancer(
        &self,
        input: CreateAccessPointInput,
    ) -> Result<CreateAccessPointOutput, RusotoError<CreateLoadBalancerError>> {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let params = self.new_params("CreateLoadBalancer");
        let mut params = params;
        CreateAccessPointInputSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request)
            .await
            .map_err(CreateLoadBalancerError::refine)?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = CreateAccessPointOutputDeserializer::deserialize(
                "CreateLoadBalancerResult",
                stack,
            )?;
            skip_tree(stack);
            xml_util::end_element(actual_tag_name, stack)?;
            Ok(result)
        })
        .await?;

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p>Creates one or more listeners for the specified load balancer. If a listener with the specified port does not already exist, it is created; otherwise, the properties of the new listener must match the properties of the existing listener.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/classic/elb-listener-config.html">Listeners for Your Classic Load Balancer</a> in the <i>Classic Load Balancers Guide</i>.</p>
    async fn create_load_balancer_listeners(
        &self,
        input: CreateLoadBalancerListenerInput,
    ) -> Result<CreateLoadBalancerListenerOutput, RusotoError<CreateLoadBalancerListenersError>>
    {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let params = self.new_params("CreateLoadBalancerListeners");
        let mut params = params;
        CreateLoadBalancerListenerInputSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request)
            .await
            .map_err(CreateLoadBalancerListenersError::refine)?;

        let result = CreateLoadBalancerListenerOutput::default();

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p>Creates a policy with the specified attributes for the specified load balancer.</p> <p>Policies are settings that are saved for your load balancer and that can be applied to the listener or the application server, depending on the policy type.</p>
    async fn create_load_balancer_policy(
        &self,
        input: CreateLoadBalancerPolicyInput,
    ) -> Result<CreateLoadBalancerPolicyOutput, RusotoError<CreateLoadBalancerPolicyError>> {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let params = self.new_params("CreateLoadBalancerPolicy");
        let mut params = params;
        CreateLoadBalancerPolicyInputSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request)
            .await
            .map_err(CreateLoadBalancerPolicyError::refine)?;

        let result = CreateLoadBalancerPolicyOutput::default();

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p>Deletes the specified load balancer.</p> <p>If you are attempting to recreate a load balancer, you must reconfigure all settings. The DNS name associated with a deleted load balancer are no longer usable. The name and associated DNS record of the deleted load balancer no longer exist and traffic sent to any of its IP addresses is no longer delivered to your instances.</p> <p>If the load balancer does not exist or has already been deleted, the call to <code>DeleteLoadBalancer</code> still succeeds.</p>
    async fn delete_load_balancer(
        &self,
        input: DeleteAccessPointInput,
    ) -> Result<DeleteAccessPointOutput, RusotoError<DeleteLoadBalancerError>> {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let params = self.new_params("DeleteLoadBalancer");
        let mut params = params;
        DeleteAccessPointInputSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request)
            .await
            .map_err(DeleteLoadBalancerError::refine)?;

        let result = DeleteAccessPointOutput::default();

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p>Deletes the specified listeners from the specified load balancer.</p>
    async fn delete_load_balancer_listeners(
        &self,
        input: DeleteLoadBalancerListenerInput,
    ) -> Result<DeleteLoadBalancerListenerOutput, RusotoError<DeleteLoadBalancerListenersError>>
    {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let params = self.new_params("DeleteLoadBalancerListeners");
        let mut params = params;
        DeleteLoadBalancerListenerInputSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request)
            .await
            .map_err(DeleteLoadBalancerListenersError::refine)?;

        let result = DeleteLoadBalancerListenerOutput::default();

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p>Deletes the specified policy from the specified load balancer. This policy must not be enabled for any listeners.</p>
    async fn delete_load_balancer_policy(
        &self,
        input: DeleteLoadBalancerPolicyInput,
    ) -> Result<DeleteLoadBalancerPolicyOutput, RusotoError<DeleteLoadBalancerPolicyError>> {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let params = self.new_params("DeleteLoadBalancerPolicy");
        let mut params = params;
        DeleteLoadBalancerPolicyInputSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request)
            .await
            .map_err(DeleteLoadBalancerPolicyError::refine)?;

        let result = DeleteLoadBalancerPolicyOutput::default();

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p>Deregisters the specified instances from the specified load balancer. After the instance is deregistered, it no longer receives traffic from the load balancer.</p> <p>You can use <a>DescribeLoadBalancers</a> to verify that the instance is deregistered from the load balancer.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/classic/elb-deregister-register-instances.html">Register or De-Register EC2 Instances</a> in the <i>Classic Load Balancers Guide</i>.</p>
    async fn deregister_instances_from_load_balancer(
        &self,
        input: DeregisterEndPointsInput,
    ) -> Result<DeregisterEndPointsOutput, RusotoError<DeregisterInstancesFromLoadBalancerError>>
    {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let params = self.new_params("DeregisterInstancesFromLoadBalancer");
        let mut params = params;
        DeregisterEndPointsInputSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request)
            .await
            .map_err(DeregisterInstancesFromLoadBalancerError::refine)?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = DeregisterEndPointsOutputDeserializer::deserialize(
                "DeregisterInstancesFromLoadBalancerResult",
                stack,
            )?;
            skip_tree(stack);
            xml_util::end_element(actual_tag_name, stack)?;
            Ok(result)
        })
        .await?;

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p>Describes the current Elastic Load Balancing resource limits for your AWS account.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/classic/elb-limits.html">Limits for Your Classic Load Balancer</a> in the <i>Classic Load Balancers Guide</i>.</p>
    async fn describe_account_limits(
        &self,
        input: DescribeAccountLimitsInput,
    ) -> Result<DescribeAccountLimitsOutput, RusotoError<DescribeAccountLimitsError>> {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let params = self.new_params("DescribeAccountLimits");
        let mut params = params;
        DescribeAccountLimitsInputSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request)
            .await
            .map_err(DescribeAccountLimitsError::refine)?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = DescribeAccountLimitsOutputDeserializer::deserialize(
                "DescribeAccountLimitsResult",
                stack,
            )?;
            skip_tree(stack);
            xml_util::end_element(actual_tag_name, stack)?;
            Ok(result)
        })
        .await?;

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p>Describes the state of the specified instances with respect to the specified load balancer. If no instances are specified, the call describes the state of all instances that are currently registered with the load balancer. If instances are specified, their state is returned even if they are no longer registered with the load balancer. The state of terminated instances is not returned.</p>
    async fn describe_instance_health(
        &self,
        input: DescribeEndPointStateInput,
    ) -> Result<DescribeEndPointStateOutput, RusotoError<DescribeInstanceHealthError>> {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let params = self.new_params("DescribeInstanceHealth");
        let mut params = params;
        DescribeEndPointStateInputSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request)
            .await
            .map_err(DescribeInstanceHealthError::refine)?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = DescribeEndPointStateOutputDeserializer::deserialize(
                "DescribeInstanceHealthResult",
                stack,
            )?;
            skip_tree(stack);
            xml_util::end_element(actual_tag_name, stack)?;
            Ok(result)
        })
        .await?;

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p>Describes the attributes for the specified load balancer.</p>
    async fn describe_load_balancer_attributes(
        &self,
        input: DescribeLoadBalancerAttributesInput,
    ) -> Result<
        DescribeLoadBalancerAttributesOutput,
        RusotoError<DescribeLoadBalancerAttributesError>,
    > {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let params = self.new_params("DescribeLoadBalancerAttributes");
        let mut params = params;
        DescribeLoadBalancerAttributesInputSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request)
            .await
            .map_err(DescribeLoadBalancerAttributesError::refine)?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = DescribeLoadBalancerAttributesOutputDeserializer::deserialize(
                "DescribeLoadBalancerAttributesResult",
                stack,
            )?;
            skip_tree(stack);
            xml_util::end_element(actual_tag_name, stack)?;
            Ok(result)
        })
        .await?;

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p>Describes the specified policies.</p> <p>If you specify a load balancer name, the action returns the descriptions of all policies created for the load balancer. If you specify a policy name associated with your load balancer, the action returns the description of that policy. If you don't specify a load balancer name, the action returns descriptions of the specified sample policies, or descriptions of all sample policies. The names of the sample policies have the <code>ELBSample-</code> prefix.</p>
    async fn describe_load_balancer_policies(
        &self,
        input: DescribeLoadBalancerPoliciesInput,
    ) -> Result<DescribeLoadBalancerPoliciesOutput, RusotoError<DescribeLoadBalancerPoliciesError>>
    {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let params = self.new_params("DescribeLoadBalancerPolicies");
        let mut params = params;
        DescribeLoadBalancerPoliciesInputSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request)
            .await
            .map_err(DescribeLoadBalancerPoliciesError::refine)?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = DescribeLoadBalancerPoliciesOutputDeserializer::deserialize(
                "DescribeLoadBalancerPoliciesResult",
                stack,
            )?;
            skip_tree(stack);
            xml_util::end_element(actual_tag_name, stack)?;
            Ok(result)
        })
        .await?;

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p>Describes the specified load balancer policy types or all load balancer policy types.</p> <p>The description of each type indicates how it can be used. For example, some policies can be used only with layer 7 listeners, some policies can be used only with layer 4 listeners, and some policies can be used only with your EC2 instances.</p> <p>You can use <a>CreateLoadBalancerPolicy</a> to create a policy configuration for any of these policy types. Then, depending on the policy type, use either <a>SetLoadBalancerPoliciesOfListener</a> or <a>SetLoadBalancerPoliciesForBackendServer</a> to set the policy.</p>
    async fn describe_load_balancer_policy_types(
        &self,
        input: DescribeLoadBalancerPolicyTypesInput,
    ) -> Result<
        DescribeLoadBalancerPolicyTypesOutput,
        RusotoError<DescribeLoadBalancerPolicyTypesError>,
    > {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let params = self.new_params("DescribeLoadBalancerPolicyTypes");
        let mut params = params;
        DescribeLoadBalancerPolicyTypesInputSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request)
            .await
            .map_err(DescribeLoadBalancerPolicyTypesError::refine)?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = DescribeLoadBalancerPolicyTypesOutputDeserializer::deserialize(
                "DescribeLoadBalancerPolicyTypesResult",
                stack,
            )?;
            skip_tree(stack);
            xml_util::end_element(actual_tag_name, stack)?;
            Ok(result)
        })
        .await?;

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p>Describes the specified the load balancers. If no load balancers are specified, the call describes all of your load balancers.</p>
    async fn describe_load_balancers(
        &self,
        input: DescribeAccessPointsInput,
    ) -> Result<DescribeAccessPointsOutput, RusotoError<DescribeLoadBalancersError>> {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let params = self.new_params("DescribeLoadBalancers");
        let mut params = params;
        DescribeAccessPointsInputSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request)
            .await
            .map_err(DescribeLoadBalancersError::refine)?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = DescribeAccessPointsOutputDeserializer::deserialize(
                "DescribeLoadBalancersResult",
                stack,
            )?;
            skip_tree(stack);
            xml_util::end_element(actual_tag_name, stack)?;
            Ok(result)
        })
        .await?;

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p>Describes the tags associated with the specified load balancers.</p>
    async fn describe_tags(
        &self,
        input: DescribeTagsInput,
    ) -> Result<DescribeTagsOutput, RusotoError<DescribeTagsError>> {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let params = self.new_params("DescribeTags");
        let mut params = params;
        DescribeTagsInputSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request)
            .await
            .map_err(DescribeTagsError::refine)?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = DescribeTagsOutputDeserializer::deserialize("DescribeTagsResult", stack)?;
            skip_tree(stack);
            xml_util::end_element(actual_tag_name, stack)?;
            Ok(result)
        })
        .await?;

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p>Removes the specified subnets from the set of configured subnets for the load balancer.</p> <p>After a subnet is removed, all EC2 instances registered with the load balancer in the removed subnet go into the <code>OutOfService</code> state. Then, the load balancer balances the traffic among the remaining routable subnets.</p>
    async fn detach_load_balancer_from_subnets(
        &self,
        input: DetachLoadBalancerFromSubnetsInput,
    ) -> Result<DetachLoadBalancerFromSubnetsOutput, RusotoError<DetachLoadBalancerFromSubnetsError>>
    {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let params = self.new_params("DetachLoadBalancerFromSubnets");
        let mut params = params;
        DetachLoadBalancerFromSubnetsInputSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request)
            .await
            .map_err(DetachLoadBalancerFromSubnetsError::refine)?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = DetachLoadBalancerFromSubnetsOutputDeserializer::deserialize(
                "DetachLoadBalancerFromSubnetsResult",
                stack,
            )?;
            skip_tree(stack);
            xml_util::end_element(actual_tag_name, stack)?;
            Ok(result)
        })
        .await?;

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p>Removes the specified Availability Zones from the set of Availability Zones for the specified load balancer in EC2-Classic or a default VPC.</p> <p>For load balancers in a non-default VPC, use <a>DetachLoadBalancerFromSubnets</a>.</p> <p>There must be at least one Availability Zone registered with a load balancer at all times. After an Availability Zone is removed, all instances registered with the load balancer that are in the removed Availability Zone go into the <code>OutOfService</code> state. Then, the load balancer attempts to equally balance the traffic among its remaining Availability Zones.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/classic/enable-disable-az.html">Add or Remove Availability Zones</a> in the <i>Classic Load Balancers Guide</i>.</p>
    async fn disable_availability_zones_for_load_balancer(
        &self,
        input: RemoveAvailabilityZonesInput,
    ) -> Result<
        RemoveAvailabilityZonesOutput,
        RusotoError<DisableAvailabilityZonesForLoadBalancerError>,
    > {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let params = self.new_params("DisableAvailabilityZonesForLoadBalancer");
        let mut params = params;
        RemoveAvailabilityZonesInputSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request)
            .await
            .map_err(DisableAvailabilityZonesForLoadBalancerError::refine)?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = RemoveAvailabilityZonesOutputDeserializer::deserialize(
                "DisableAvailabilityZonesForLoadBalancerResult",
                stack,
            )?;
            skip_tree(stack);
            xml_util::end_element(actual_tag_name, stack)?;
            Ok(result)
        })
        .await?;

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p>Adds the specified Availability Zones to the set of Availability Zones for the specified load balancer in EC2-Classic or a default VPC.</p> <p>For load balancers in a non-default VPC, use <a>AttachLoadBalancerToSubnets</a>.</p> <p>The load balancer evenly distributes requests across all its registered Availability Zones that contain instances. For more information, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/classic/enable-disable-az.html">Add or Remove Availability Zones</a> in the <i>Classic Load Balancers Guide</i>.</p>
    async fn enable_availability_zones_for_load_balancer(
        &self,
        input: AddAvailabilityZonesInput,
    ) -> Result<AddAvailabilityZonesOutput, RusotoError<EnableAvailabilityZonesForLoadBalancerError>>
    {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let params = self.new_params("EnableAvailabilityZonesForLoadBalancer");
        let mut params = params;
        AddAvailabilityZonesInputSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request)
            .await
            .map_err(EnableAvailabilityZonesForLoadBalancerError::refine)?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = AddAvailabilityZonesOutputDeserializer::deserialize(
                "EnableAvailabilityZonesForLoadBalancerResult",
                stack,
            )?;
            skip_tree(stack);
            xml_util::end_element(actual_tag_name, stack)?;
            Ok(result)
        })
        .await?;

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p><p>Modifies the attributes of the specified load balancer.</p> <p>You can modify the load balancer attributes, such as <code>AccessLogs</code>, <code>ConnectionDraining</code>, and <code>CrossZoneLoadBalancing</code> by either enabling or disabling them. Or, you can modify the load balancer attribute <code>ConnectionSettings</code> by specifying an idle connection timeout value for your load balancer.</p> <p>For more information, see the following in the <i>Classic Load Balancers Guide</i>:</p> <ul> <li> <p> <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/classic/enable-disable-crosszone-lb.html">Cross-Zone Load Balancing</a> </p> </li> <li> <p> <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/classic/config-conn-drain.html">Connection Draining</a> </p> </li> <li> <p> <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/classic/access-log-collection.html">Access Logs</a> </p> </li> <li> <p> <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/classic/config-idle-timeout.html">Idle Connection Timeout</a> </p> </li> </ul></p>
    async fn modify_load_balancer_attributes(
        &self,
        input: ModifyLoadBalancerAttributesInput,
    ) -> Result<ModifyLoadBalancerAttributesOutput, RusotoError<ModifyLoadBalancerAttributesError>>
    {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let params = self.new_params("ModifyLoadBalancerAttributes");
        let mut params = params;
        ModifyLoadBalancerAttributesInputSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request)
            .await
            .map_err(ModifyLoadBalancerAttributesError::refine)?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = ModifyLoadBalancerAttributesOutputDeserializer::deserialize(
                "ModifyLoadBalancerAttributesResult",
                stack,
            )?;
            skip_tree(stack);
            xml_util::end_element(actual_tag_name, stack)?;
            Ok(result)
        })
        .await?;

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p>Adds the specified instances to the specified load balancer.</p> <p>The instance must be a running instance in the same network as the load balancer (EC2-Classic or the same VPC). If you have EC2-Classic instances and a load balancer in a VPC with ClassicLink enabled, you can link the EC2-Classic instances to that VPC and then register the linked EC2-Classic instances with the load balancer in the VPC.</p> <p>Note that <code>RegisterInstanceWithLoadBalancer</code> completes when the request has been registered. Instance registration takes a little time to complete. To check the state of the registered instances, use <a>DescribeLoadBalancers</a> or <a>DescribeInstanceHealth</a>.</p> <p>After the instance is registered, it starts receiving traffic and requests from the load balancer. Any instance that is not in one of the Availability Zones registered for the load balancer is moved to the <code>OutOfService</code> state. If an Availability Zone is added to the load balancer later, any instances registered with the load balancer move to the <code>InService</code> state.</p> <p>To deregister instances from a load balancer, use <a>DeregisterInstancesFromLoadBalancer</a>.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/classic/elb-deregister-register-instances.html">Register or De-Register EC2 Instances</a> in the <i>Classic Load Balancers Guide</i>.</p>
    async fn register_instances_with_load_balancer(
        &self,
        input: RegisterEndPointsInput,
    ) -> Result<RegisterEndPointsOutput, RusotoError<RegisterInstancesWithLoadBalancerError>> {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let params = self.new_params("RegisterInstancesWithLoadBalancer");
        let mut params = params;
        RegisterEndPointsInputSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request)
            .await
            .map_err(RegisterInstancesWithLoadBalancerError::refine)?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = RegisterEndPointsOutputDeserializer::deserialize(
                "RegisterInstancesWithLoadBalancerResult",
                stack,
            )?;
            skip_tree(stack);
            xml_util::end_element(actual_tag_name, stack)?;
            Ok(result)
        })
        .await?;

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p>Removes one or more tags from the specified load balancer.</p>
    async fn remove_tags(
        &self,
        input: RemoveTagsInput,
    ) -> Result<RemoveTagsOutput, RusotoError<RemoveTagsError>> {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let params = self.new_params("RemoveTags");
        let mut params = params;
        RemoveTagsInputSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request)
            .await
            .map_err(RemoveTagsError::refine)?;

        let result = RemoveTagsOutput::default();

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p>Sets the certificate that terminates the specified listener's SSL connections. The specified certificate replaces any prior certificate that was used on the same load balancer and port.</p> <p>For more information about updating your SSL certificate, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/classic/elb-update-ssl-cert.html">Replace the SSL Certificate for Your Load Balancer</a> in the <i>Classic Load Balancers Guide</i>.</p>
    async fn set_load_balancer_listener_ssl_certificate(
        &self,
        input: SetLoadBalancerListenerSSLCertificateInput,
    ) -> Result<
        SetLoadBalancerListenerSSLCertificateOutput,
        RusotoError<SetLoadBalancerListenerSSLCertificateError>,
    > {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let params = self.new_params("SetLoadBalancerListenerSSLCertificate");
        let mut params = params;
        SetLoadBalancerListenerSSLCertificateInputSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request)
            .await
            .map_err(SetLoadBalancerListenerSSLCertificateError::refine)?;

        let result = SetLoadBalancerListenerSSLCertificateOutput::default();

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p>Replaces the set of policies associated with the specified port on which the EC2 instance is listening with a new set of policies. At this time, only the back-end server authentication policy type can be applied to the instance ports; this policy type is composed of multiple public key policies.</p> <p>Each time you use <code>SetLoadBalancerPoliciesForBackendServer</code> to enable the policies, use the <code>PolicyNames</code> parameter to list the policies that you want to enable.</p> <p>You can use <a>DescribeLoadBalancers</a> or <a>DescribeLoadBalancerPolicies</a> to verify that the policy is associated with the EC2 instance.</p> <p>For more information about enabling back-end instance authentication, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/classic/elb-create-https-ssl-load-balancer.html#configure_backendauth_clt">Configure Back-end Instance Authentication</a> in the <i>Classic Load Balancers Guide</i>. For more information about Proxy Protocol, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/classic/enable-proxy-protocol.html">Configure Proxy Protocol Support</a> in the <i>Classic Load Balancers Guide</i>.</p>
    async fn set_load_balancer_policies_for_backend_server(
        &self,
        input: SetLoadBalancerPoliciesForBackendServerInput,
    ) -> Result<
        SetLoadBalancerPoliciesForBackendServerOutput,
        RusotoError<SetLoadBalancerPoliciesForBackendServerError>,
    > {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let params = self.new_params("SetLoadBalancerPoliciesForBackendServer");
        let mut params = params;
        SetLoadBalancerPoliciesForBackendServerInputSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request)
            .await
            .map_err(SetLoadBalancerPoliciesForBackendServerError::refine)?;

        let result = SetLoadBalancerPoliciesForBackendServerOutput::default();

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p>Replaces the current set of policies for the specified load balancer port with the specified set of policies.</p> <p>To enable back-end server authentication, use <a>SetLoadBalancerPoliciesForBackendServer</a>.</p> <p>For more information about setting policies, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/classic/ssl-config-update.html">Update the SSL Negotiation Configuration</a>, <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/classic/elb-sticky-sessions.html#enable-sticky-sessions-duration">Duration-Based Session Stickiness</a>, and <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/classic/elb-sticky-sessions.html#enable-sticky-sessions-application">Application-Controlled Session Stickiness</a> in the <i>Classic Load Balancers Guide</i>.</p>
    async fn set_load_balancer_policies_of_listener(
        &self,
        input: SetLoadBalancerPoliciesOfListenerInput,
    ) -> Result<
        SetLoadBalancerPoliciesOfListenerOutput,
        RusotoError<SetLoadBalancerPoliciesOfListenerError>,
    > {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let params = self.new_params("SetLoadBalancerPoliciesOfListener");
        let mut params = params;
        SetLoadBalancerPoliciesOfListenerInputSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request)
            .await
            .map_err(SetLoadBalancerPoliciesOfListenerError::refine)?;

        let result = SetLoadBalancerPoliciesOfListenerOutput::default();

        drop(response); // parse non-payload
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
    async fn test_parse_error_elb_describe_load_balancers() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/error",
            "elb-describe-load-balancers.xml",
        );
        let mock = MockRequestDispatcher::with_status(400).with_body(&mock_response);
        let client = ElbClient::new_with(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = DescribeAccessPointsInput::default();
        let result = client.describe_load_balancers(request).await;
        assert!(!result.is_ok(), "parse error: {:?}", result);
    }

    #[tokio::test]
    async fn test_parse_valid_elb_describe_load_balancer_policies() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "elb-describe-load-balancer-policies.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = ElbClient::new_with(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = DescribeLoadBalancerPoliciesInput::default();
        let result = client.describe_load_balancer_policies(request).await;
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[tokio::test]
    async fn test_parse_valid_elb_describe_load_balancer_policy_types() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "elb-describe-load-balancer-policy-types.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = ElbClient::new_with(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = DescribeLoadBalancerPolicyTypesInput::default();
        let result = client.describe_load_balancer_policy_types(request).await;
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[tokio::test]
    async fn test_parse_valid_elb_describe_load_balancers() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "elb-describe-load-balancers.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = ElbClient::new_with(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = DescribeAccessPointsInput::default();
        let result = client.describe_load_balancers(request).await;
        assert!(result.is_ok(), "parse error: {:?}", result);
    }
}
