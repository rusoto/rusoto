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
struct ActivitiesDeserializer;
impl ActivitiesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Activity>, XmlParseError> {
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
                        obj.push(try!(ActivityDeserializer::deserialize("member", stack)));
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
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ActivitiesType {
    /// <p>The scaling activities. Activities are sorted by start time. Activities still in progress are described first.</p>
    pub activities: Vec<Activity>,
    /// <p>The token to use when requesting the next set of items. If there are no additional items to return, the string is empty.</p>
    pub next_token: Option<String>,
}

struct ActivitiesTypeDeserializer;
impl ActivitiesTypeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ActivitiesType, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ActivitiesType::default();

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
                    "Activities" => {
                        obj.activities =
                            try!(ActivitiesDeserializer::deserialize("Activities", stack));
                    }
                    "NextToken" => {
                        obj.next_token =
                            Some(try!(XmlStringDeserializer::deserialize("NextToken", stack)));
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
/// <p>Describes scaling activity, which is a long-running process that represents a change to your Auto Scaling group, such as changing its size or replacing an instance.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Activity {
    /// <p>The ID of the activity.</p>
    pub activity_id: String,
    /// <p>The name of the Auto Scaling group.</p>
    pub auto_scaling_group_name: String,
    /// <p>The reason the activity began.</p>
    pub cause: String,
    /// <p>A friendly, more verbose description of the activity.</p>
    pub description: Option<String>,
    /// <p>The details about the activity.</p>
    pub details: Option<String>,
    /// <p>The end time of the activity.</p>
    pub end_time: Option<String>,
    /// <p>A value between 0 and 100 that indicates the progress of the activity.</p>
    pub progress: Option<i64>,
    /// <p>The start time of the activity.</p>
    pub start_time: String,
    /// <p>The current status of the activity.</p>
    pub status_code: String,
    /// <p>A friendly, more verbose description of the activity status.</p>
    pub status_message: Option<String>,
}

struct ActivityDeserializer;
impl ActivityDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Activity, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = Activity::default();

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
                    "ActivityId" => {
                        obj.activity_id =
                            try!(XmlStringDeserializer::deserialize("ActivityId", stack));
                    }
                    "AutoScalingGroupName" => {
                        obj.auto_scaling_group_name =
                            try!(XmlStringMaxLen255Deserializer::deserialize(
                                "AutoScalingGroupName",
                                stack
                            ));
                    }
                    "Cause" => {
                        obj.cause =
                            try!(XmlStringMaxLen1023Deserializer::deserialize("Cause", stack));
                    }
                    "Description" => {
                        obj.description = Some(try!(XmlStringDeserializer::deserialize(
                            "Description",
                            stack
                        )));
                    }
                    "Details" => {
                        obj.details =
                            Some(try!(XmlStringDeserializer::deserialize("Details", stack)));
                    }
                    "EndTime" => {
                        obj.end_time = Some(try!(TimestampTypeDeserializer::deserialize(
                            "EndTime", stack
                        )));
                    }
                    "Progress" => {
                        obj.progress =
                            Some(try!(ProgressDeserializer::deserialize("Progress", stack)));
                    }
                    "StartTime" => {
                        obj.start_time =
                            try!(TimestampTypeDeserializer::deserialize("StartTime", stack));
                    }
                    "StatusCode" => {
                        obj.status_code = try!(ScalingActivityStatusCodeDeserializer::deserialize(
                            "StatusCode",
                            stack
                        ));
                    }
                    "StatusMessage" => {
                        obj.status_message = Some(try!(
                            XmlStringMaxLen255Deserializer::deserialize("StatusMessage", stack)
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

/// Serialize `ActivityIds` contents to a `SignedRequest`.
struct ActivityIdsSerializer;
impl ActivityIdsSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct ActivityType {
    /// <p>A scaling activity.</p>
    pub activity: Option<Activity>,
}

struct ActivityTypeDeserializer;
impl ActivityTypeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ActivityType, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ActivityType::default();

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
                    "Activity" => {
                        obj.activity =
                            Some(try!(ActivityDeserializer::deserialize("Activity", stack)));
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
/// <p>Describes a policy adjustment type.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/AutoScaling/latest/DeveloperGuide/as-scale-based-on-demand.html">Dynamic Scaling</a> in the <i>Auto Scaling User Guide</i>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct AdjustmentType {
    /// <p>The policy adjustment type. The valid values are <code>ChangeInCapacity</code>, <code>ExactCapacity</code>, and <code>PercentChangeInCapacity</code>.</p>
    pub adjustment_type: Option<String>,
}

struct AdjustmentTypeDeserializer;
impl AdjustmentTypeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AdjustmentType, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = AdjustmentType::default();

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
                    "AdjustmentType" => {
                        obj.adjustment_type = Some(try!(
                            XmlStringMaxLen255Deserializer::deserialize("AdjustmentType", stack)
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
struct AdjustmentTypesDeserializer;
impl AdjustmentTypesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<AdjustmentType>, XmlParseError> {
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
                        obj.push(try!(AdjustmentTypeDeserializer::deserialize(
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
/// <p>Describes an alarm.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Alarm {
    /// <p>The Amazon Resource Name (ARN) of the alarm.</p>
    pub alarm_arn: Option<String>,
    /// <p>The name of the alarm.</p>
    pub alarm_name: Option<String>,
}

struct AlarmDeserializer;
impl AlarmDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Alarm, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = Alarm::default();

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
                    "AlarmARN" => {
                        obj.alarm_arn = Some(try!(ResourceNameDeserializer::deserialize(
                            "AlarmARN", stack
                        )));
                    }
                    "AlarmName" => {
                        obj.alarm_name = Some(try!(XmlStringMaxLen255Deserializer::deserialize(
                            "AlarmName",
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
struct AlarmsDeserializer;
impl AlarmsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Alarm>, XmlParseError> {
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
                        obj.push(try!(AlarmDeserializer::deserialize("member", stack)));
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
struct AsciiStringMaxLen255Deserializer;
impl AsciiStringMaxLen255Deserializer {
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
struct AssociatePublicIpAddressDeserializer;
impl AssociatePublicIpAddressDeserializer {
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
pub struct AttachInstancesQuery {
    /// <p>The name of the Auto Scaling group.</p>
    pub auto_scaling_group_name: String,
    /// <p>The IDs of the instances. You can specify up to 20 instances.</p>
    pub instance_ids: Option<Vec<String>>,
}

/// Serialize `AttachInstancesQuery` contents to a `SignedRequest`.
struct AttachInstancesQuerySerializer;
impl AttachInstancesQuerySerializer {
    fn serialize(params: &mut Params, name: &str, obj: &AttachInstancesQuery) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "AutoScalingGroupName"),
            &obj.auto_scaling_group_name.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.instance_ids {
            InstanceIdsSerializer::serialize(
                params,
                &format!("{}{}", prefix, "InstanceIds"),
                field_value,
            );
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct AttachLoadBalancerTargetGroupsResultType {}

struct AttachLoadBalancerTargetGroupsResultTypeDeserializer;
impl AttachLoadBalancerTargetGroupsResultTypeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AttachLoadBalancerTargetGroupsResultType, XmlParseError> {
        try!(start_element(tag_name, stack));

        let obj = AttachLoadBalancerTargetGroupsResultType::default();

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct AttachLoadBalancerTargetGroupsType {
    /// <p>The name of the Auto Scaling group.</p>
    pub auto_scaling_group_name: String,
    /// <p>The Amazon Resource Names (ARN) of the target groups. You can specify up to 10 target groups.</p>
    pub target_group_ar_ns: Vec<String>,
}

/// Serialize `AttachLoadBalancerTargetGroupsType` contents to a `SignedRequest`.
struct AttachLoadBalancerTargetGroupsTypeSerializer;
impl AttachLoadBalancerTargetGroupsTypeSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &AttachLoadBalancerTargetGroupsType) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "AutoScalingGroupName"),
            &obj.auto_scaling_group_name.replace("+", "%2B"),
        );
        TargetGroupARNsSerializer::serialize(
            params,
            &format!("{}{}", prefix, "TargetGroupARNs"),
            &obj.target_group_ar_ns,
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct AttachLoadBalancersResultType {}

struct AttachLoadBalancersResultTypeDeserializer;
impl AttachLoadBalancersResultTypeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AttachLoadBalancersResultType, XmlParseError> {
        try!(start_element(tag_name, stack));

        let obj = AttachLoadBalancersResultType::default();

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct AttachLoadBalancersType {
    /// <p>The name of the Auto Scaling group.</p>
    pub auto_scaling_group_name: String,
    /// <p>The names of the load balancers. You can specify up to 10 load balancers.</p>
    pub load_balancer_names: Vec<String>,
}

/// Serialize `AttachLoadBalancersType` contents to a `SignedRequest`.
struct AttachLoadBalancersTypeSerializer;
impl AttachLoadBalancersTypeSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &AttachLoadBalancersType) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "AutoScalingGroupName"),
            &obj.auto_scaling_group_name.replace("+", "%2B"),
        );
        LoadBalancerNamesSerializer::serialize(
            params,
            &format!("{}{}", prefix, "LoadBalancerNames"),
            &obj.load_balancer_names,
        );
    }
}

/// <p>Describes an Auto Scaling group.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct AutoScalingGroup {
    /// <p>The Amazon Resource Name (ARN) of the Auto Scaling group.</p>
    pub auto_scaling_group_arn: Option<String>,
    /// <p>The name of the Auto Scaling group.</p>
    pub auto_scaling_group_name: String,
    /// <p>One or more Availability Zones for the group.</p>
    pub availability_zones: Vec<String>,
    /// <p>The date and time the group was created.</p>
    pub created_time: String,
    /// <p>The amount of time, in seconds, after a scaling activity completes before another scaling activity can start.</p>
    pub default_cooldown: i64,
    /// <p>The desired size of the group.</p>
    pub desired_capacity: i64,
    /// <p>The metrics enabled for the group.</p>
    pub enabled_metrics: Option<Vec<EnabledMetric>>,
    /// <p>The amount of time, in seconds, that Auto Scaling waits before checking the health status of an EC2 instance that has come into service.</p>
    pub health_check_grace_period: Option<i64>,
    /// <p>The service to use for the health checks. The valid values are <code>EC2</code> and <code>ELB</code>.</p>
    pub health_check_type: String,
    /// <p>The EC2 instances associated with the group.</p>
    pub instances: Option<Vec<Instance>>,
    /// <p>The name of the associated launch configuration.</p>
    pub launch_configuration_name: Option<String>,
    /// <p>The launch template for the group.</p>
    pub launch_template: Option<LaunchTemplateSpecification>,
    /// <p>One or more load balancers associated with the group.</p>
    pub load_balancer_names: Option<Vec<String>>,
    /// <p>The maximum size of the group.</p>
    pub max_size: i64,
    /// <p>The minimum size of the group.</p>
    pub min_size: i64,
    /// <p>Indicates whether newly launched instances are protected from termination by Auto Scaling when scaling in.</p>
    pub new_instances_protected_from_scale_in: Option<bool>,
    /// <p>The name of the placement group into which you'll launch your instances, if any. For more information, see <a href="http://docs.aws.amazon.com/AWSEC2/latest/UserGuide/placement-groups.html">Placement Groups</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
    pub placement_group: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the service-linked role that the Auto Scaling group uses to call other AWS services on your behalf.</p>
    pub service_linked_role_arn: Option<String>,
    /// <p>The current state of the group when <a>DeleteAutoScalingGroup</a> is in progress.</p>
    pub status: Option<String>,
    /// <p>The suspended processes associated with the group.</p>
    pub suspended_processes: Option<Vec<SuspendedProcess>>,
    /// <p>The tags for the group.</p>
    pub tags: Option<Vec<TagDescription>>,
    /// <p>The Amazon Resource Names (ARN) of the target groups for your load balancer.</p>
    pub target_group_ar_ns: Option<Vec<String>>,
    /// <p>The termination policies for the group.</p>
    pub termination_policies: Option<Vec<String>>,
    /// <p>One or more subnet IDs, if applicable, separated by commas.</p> <p>If you specify <code>VPCZoneIdentifier</code> and <code>AvailabilityZones</code>, ensure that the Availability Zones of the subnets match the values for <code>AvailabilityZones</code>.</p>
    pub vpc_zone_identifier: Option<String>,
}

struct AutoScalingGroupDeserializer;
impl AutoScalingGroupDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AutoScalingGroup, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = AutoScalingGroup::default();

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
                    "AutoScalingGroupARN" => {
                        obj.auto_scaling_group_arn = Some(try!(
                            ResourceNameDeserializer::deserialize("AutoScalingGroupARN", stack)
                        ));
                    }
                    "AutoScalingGroupName" => {
                        obj.auto_scaling_group_name =
                            try!(XmlStringMaxLen255Deserializer::deserialize(
                                "AutoScalingGroupName",
                                stack
                            ));
                    }
                    "AvailabilityZones" => {
                        obj.availability_zones = try!(AvailabilityZonesDeserializer::deserialize(
                            "AvailabilityZones",
                            stack
                        ));
                    }
                    "CreatedTime" => {
                        obj.created_time =
                            try!(TimestampTypeDeserializer::deserialize("CreatedTime", stack));
                    }
                    "DefaultCooldown" => {
                        obj.default_cooldown =
                            try!(CooldownDeserializer::deserialize("DefaultCooldown", stack));
                    }
                    "DesiredCapacity" => {
                        obj.desired_capacity =
                            try!(AutoScalingGroupDesiredCapacityDeserializer::deserialize(
                                "DesiredCapacity",
                                stack
                            ));
                    }
                    "EnabledMetrics" => {
                        obj.enabled_metrics = Some(try!(EnabledMetricsDeserializer::deserialize(
                            "EnabledMetrics",
                            stack
                        )));
                    }
                    "HealthCheckGracePeriod" => {
                        obj.health_check_grace_period =
                            Some(try!(HealthCheckGracePeriodDeserializer::deserialize(
                                "HealthCheckGracePeriod",
                                stack
                            )));
                    }
                    "HealthCheckType" => {
                        obj.health_check_type = try!(XmlStringMaxLen32Deserializer::deserialize(
                            "HealthCheckType",
                            stack
                        ));
                    }
                    "Instances" => {
                        obj.instances =
                            Some(try!(InstancesDeserializer::deserialize("Instances", stack)));
                    }
                    "LaunchConfigurationName" => {
                        obj.launch_configuration_name =
                            Some(try!(XmlStringMaxLen255Deserializer::deserialize(
                                "LaunchConfigurationName",
                                stack
                            )));
                    }
                    "LaunchTemplate" => {
                        obj.launch_template =
                            Some(try!(LaunchTemplateSpecificationDeserializer::deserialize(
                                "LaunchTemplate",
                                stack
                            )));
                    }
                    "LoadBalancerNames" => {
                        obj.load_balancer_names = Some(try!(
                            LoadBalancerNamesDeserializer::deserialize("LoadBalancerNames", stack)
                        ));
                    }
                    "MaxSize" => {
                        obj.max_size = try!(AutoScalingGroupMaxSizeDeserializer::deserialize(
                            "MaxSize", stack
                        ));
                    }
                    "MinSize" => {
                        obj.min_size = try!(AutoScalingGroupMinSizeDeserializer::deserialize(
                            "MinSize", stack
                        ));
                    }
                    "NewInstancesProtectedFromScaleIn" => {
                        obj.new_instances_protected_from_scale_in =
                            Some(try!(InstanceProtectedDeserializer::deserialize(
                                "NewInstancesProtectedFromScaleIn",
                                stack
                            )));
                    }
                    "PlacementGroup" => {
                        obj.placement_group = Some(try!(
                            XmlStringMaxLen255Deserializer::deserialize("PlacementGroup", stack)
                        ));
                    }
                    "ServiceLinkedRoleARN" => {
                        obj.service_linked_role_arn = Some(try!(
                            ResourceNameDeserializer::deserialize("ServiceLinkedRoleARN", stack)
                        ));
                    }
                    "Status" => {
                        obj.status = Some(try!(XmlStringMaxLen255Deserializer::deserialize(
                            "Status", stack
                        )));
                    }
                    "SuspendedProcesses" => {
                        obj.suspended_processes =
                            Some(try!(SuspendedProcessesDeserializer::deserialize(
                                "SuspendedProcesses",
                                stack
                            )));
                    }
                    "Tags" => {
                        obj.tags = Some(try!(TagDescriptionListDeserializer::deserialize(
                            "Tags", stack
                        )));
                    }
                    "TargetGroupARNs" => {
                        obj.target_group_ar_ns = Some(try!(
                            TargetGroupARNsDeserializer::deserialize("TargetGroupARNs", stack)
                        ));
                    }
                    "TerminationPolicies" => {
                        obj.termination_policies =
                            Some(try!(TerminationPoliciesDeserializer::deserialize(
                                "TerminationPolicies",
                                stack
                            )));
                    }
                    "VPCZoneIdentifier" => {
                        obj.vpc_zone_identifier =
                            Some(try!(XmlStringMaxLen2047Deserializer::deserialize(
                                "VPCZoneIdentifier",
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
struct AutoScalingGroupDesiredCapacityDeserializer;
impl AutoScalingGroupDesiredCapacityDeserializer {
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
struct AutoScalingGroupMaxSizeDeserializer;
impl AutoScalingGroupMaxSizeDeserializer {
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
struct AutoScalingGroupMinSizeDeserializer;
impl AutoScalingGroupMinSizeDeserializer {
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

/// Serialize `AutoScalingGroupNames` contents to a `SignedRequest`.
struct AutoScalingGroupNamesSerializer;
impl AutoScalingGroupNamesSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct AutoScalingGroupNamesType {
    /// <p>The names of the Auto Scaling groups. If you omit this parameter, all Auto Scaling groups are described.</p>
    pub auto_scaling_group_names: Option<Vec<String>>,
    /// <p>The maximum number of items to return with this call. The default value is 50 and the maximum value is 100.</p>
    pub max_records: Option<i64>,
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    pub next_token: Option<String>,
}

/// Serialize `AutoScalingGroupNamesType` contents to a `SignedRequest`.
struct AutoScalingGroupNamesTypeSerializer;
impl AutoScalingGroupNamesTypeSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &AutoScalingGroupNamesType) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.auto_scaling_group_names {
            AutoScalingGroupNamesSerializer::serialize(
                params,
                &format!("{}{}", prefix, "AutoScalingGroupNames"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.max_records {
            params.put(
                &format!("{}{}", prefix, "MaxRecords"),
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

struct AutoScalingGroupsDeserializer;
impl AutoScalingGroupsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<AutoScalingGroup>, XmlParseError> {
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
                        obj.push(try!(AutoScalingGroupDeserializer::deserialize(
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
#[derive(Default, Debug, Clone, PartialEq)]
pub struct AutoScalingGroupsType {
    /// <p>The groups.</p>
    pub auto_scaling_groups: Vec<AutoScalingGroup>,
    /// <p>The token to use when requesting the next set of items. If there are no additional items to return, the string is empty.</p>
    pub next_token: Option<String>,
}

struct AutoScalingGroupsTypeDeserializer;
impl AutoScalingGroupsTypeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AutoScalingGroupsType, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = AutoScalingGroupsType::default();

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
                    "AutoScalingGroups" => {
                        obj.auto_scaling_groups = try!(AutoScalingGroupsDeserializer::deserialize(
                            "AutoScalingGroups",
                            stack
                        ));
                    }
                    "NextToken" => {
                        obj.next_token =
                            Some(try!(XmlStringDeserializer::deserialize("NextToken", stack)));
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
/// <p>Describes an EC2 instance associated with an Auto Scaling group.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct AutoScalingInstanceDetails {
    /// <p>The name of the Auto Scaling group for the instance.</p>
    pub auto_scaling_group_name: String,
    /// <p>The Availability Zone for the instance.</p>
    pub availability_zone: String,
    /// <p>The last reported health status of this instance. "Healthy" means that the instance is healthy and should remain in service. "Unhealthy" means that the instance is unhealthy and Auto Scaling should terminate and replace it.</p>
    pub health_status: String,
    /// <p>The ID of the instance.</p>
    pub instance_id: String,
    /// <p>The launch configuration used to launch the instance. This value is not available if you attached the instance to the Auto Scaling group.</p>
    pub launch_configuration_name: Option<String>,
    /// <p>The launch template for the instance.</p>
    pub launch_template: Option<LaunchTemplateSpecification>,
    /// <p>The lifecycle state for the instance. For more information, see <a href="http://docs.aws.amazon.com/autoscaling/latest/userguide/AutoScalingGroupLifecycle.html">Auto Scaling Lifecycle</a> in the <i>Auto Scaling User Guide</i>.</p>
    pub lifecycle_state: String,
    /// <p>Indicates whether the instance is protected from termination by Auto Scaling when scaling in.</p>
    pub protected_from_scale_in: bool,
}

struct AutoScalingInstanceDetailsDeserializer;
impl AutoScalingInstanceDetailsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AutoScalingInstanceDetails, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = AutoScalingInstanceDetails::default();

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
                    "AutoScalingGroupName" => {
                        obj.auto_scaling_group_name =
                            try!(XmlStringMaxLen255Deserializer::deserialize(
                                "AutoScalingGroupName",
                                stack
                            ));
                    }
                    "AvailabilityZone" => {
                        obj.availability_zone = try!(XmlStringMaxLen255Deserializer::deserialize(
                            "AvailabilityZone",
                            stack
                        ));
                    }
                    "HealthStatus" => {
                        obj.health_status = try!(XmlStringMaxLen32Deserializer::deserialize(
                            "HealthStatus",
                            stack
                        ));
                    }
                    "InstanceId" => {
                        obj.instance_id = try!(XmlStringMaxLen19Deserializer::deserialize(
                            "InstanceId",
                            stack
                        ));
                    }
                    "LaunchConfigurationName" => {
                        obj.launch_configuration_name =
                            Some(try!(XmlStringMaxLen255Deserializer::deserialize(
                                "LaunchConfigurationName",
                                stack
                            )));
                    }
                    "LaunchTemplate" => {
                        obj.launch_template =
                            Some(try!(LaunchTemplateSpecificationDeserializer::deserialize(
                                "LaunchTemplate",
                                stack
                            )));
                    }
                    "LifecycleState" => {
                        obj.lifecycle_state = try!(XmlStringMaxLen32Deserializer::deserialize(
                            "LifecycleState",
                            stack
                        ));
                    }
                    "ProtectedFromScaleIn" => {
                        obj.protected_from_scale_in =
                            try!(InstanceProtectedDeserializer::deserialize(
                                "ProtectedFromScaleIn",
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
struct AutoScalingInstancesDeserializer;
impl AutoScalingInstancesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<AutoScalingInstanceDetails>, XmlParseError> {
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
                        obj.push(try!(AutoScalingInstanceDetailsDeserializer::deserialize(
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
#[derive(Default, Debug, Clone, PartialEq)]
pub struct AutoScalingInstancesType {
    /// <p>The instances.</p>
    pub auto_scaling_instances: Option<Vec<AutoScalingInstanceDetails>>,
    /// <p>The token to use when requesting the next set of items. If there are no additional items to return, the string is empty.</p>
    pub next_token: Option<String>,
}

struct AutoScalingInstancesTypeDeserializer;
impl AutoScalingInstancesTypeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AutoScalingInstancesType, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = AutoScalingInstancesType::default();

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
                    "AutoScalingInstances" => {
                        obj.auto_scaling_instances =
                            Some(try!(AutoScalingInstancesDeserializer::deserialize(
                                "AutoScalingInstances",
                                stack
                            )));
                    }
                    "NextToken" => {
                        obj.next_token =
                            Some(try!(XmlStringDeserializer::deserialize("NextToken", stack)));
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
struct AutoScalingNotificationTypesDeserializer;
impl AutoScalingNotificationTypesDeserializer {
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
                        obj.push(try!(XmlStringMaxLen255Deserializer::deserialize(
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

/// Serialize `AutoScalingNotificationTypes` contents to a `SignedRequest`.
struct AutoScalingNotificationTypesSerializer;
impl AutoScalingNotificationTypesSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
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
                        obj.push(try!(XmlStringMaxLen255Deserializer::deserialize(
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

struct BlockDeviceEbsDeleteOnTerminationDeserializer;
impl BlockDeviceEbsDeleteOnTerminationDeserializer {
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
struct BlockDeviceEbsEncryptedDeserializer;
impl BlockDeviceEbsEncryptedDeserializer {
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
struct BlockDeviceEbsIopsDeserializer;
impl BlockDeviceEbsIopsDeserializer {
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
struct BlockDeviceEbsVolumeSizeDeserializer;
impl BlockDeviceEbsVolumeSizeDeserializer {
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
struct BlockDeviceEbsVolumeTypeDeserializer;
impl BlockDeviceEbsVolumeTypeDeserializer {
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
/// <p>Describes a block device mapping.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct BlockDeviceMapping {
    /// <p>The device name exposed to the EC2 instance (for example, <code>/dev/sdh</code> or <code>xvdh</code>).</p>
    pub device_name: String,
    /// <p>The information about the Amazon EBS volume.</p>
    pub ebs: Option<Ebs>,
    /// <p>Suppresses a device mapping.</p> <p>If this parameter is true for the root device, the instance might fail the EC2 health check. Auto Scaling launches a replacement instance if the instance fails the health check.</p>
    pub no_device: Option<bool>,
    /// <p>The name of the virtual device (for example, <code>ephemeral0</code>).</p>
    pub virtual_name: Option<String>,
}

struct BlockDeviceMappingDeserializer;
impl BlockDeviceMappingDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<BlockDeviceMapping, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = BlockDeviceMapping::default();

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
                    "DeviceName" => {
                        obj.device_name = try!(XmlStringMaxLen255Deserializer::deserialize(
                            "DeviceName",
                            stack
                        ));
                    }
                    "Ebs" => {
                        obj.ebs = Some(try!(EbsDeserializer::deserialize("Ebs", stack)));
                    }
                    "NoDevice" => {
                        obj.no_device =
                            Some(try!(NoDeviceDeserializer::deserialize("NoDevice", stack)));
                    }
                    "VirtualName" => {
                        obj.virtual_name = Some(try!(XmlStringMaxLen255Deserializer::deserialize(
                            "VirtualName",
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

/// Serialize `BlockDeviceMapping` contents to a `SignedRequest`.
struct BlockDeviceMappingSerializer;
impl BlockDeviceMappingSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &BlockDeviceMapping) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "DeviceName"),
            &obj.device_name.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.ebs {
            EbsSerializer::serialize(params, &format!("{}{}", prefix, "Ebs"), field_value);
        }
        if let Some(ref field_value) = obj.no_device {
            params.put(
                &format!("{}{}", prefix, "NoDevice"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.virtual_name {
            params.put(
                &format!("{}{}", prefix, "VirtualName"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

struct BlockDeviceMappingsDeserializer;
impl BlockDeviceMappingsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<BlockDeviceMapping>, XmlParseError> {
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
                        obj.push(try!(BlockDeviceMappingDeserializer::deserialize(
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

/// Serialize `BlockDeviceMappings` contents to a `SignedRequest`.
struct BlockDeviceMappingsSerializer;
impl BlockDeviceMappingsSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<BlockDeviceMapping>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            BlockDeviceMappingSerializer::serialize(params, &key, obj);
        }
    }
}

struct ClassicLinkVPCSecurityGroupsDeserializer;
impl ClassicLinkVPCSecurityGroupsDeserializer {
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
                        obj.push(try!(XmlStringMaxLen255Deserializer::deserialize(
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

/// Serialize `ClassicLinkVPCSecurityGroups` contents to a `SignedRequest`.
struct ClassicLinkVPCSecurityGroupsSerializer;
impl ClassicLinkVPCSecurityGroupsSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct CompleteLifecycleActionAnswer {}

struct CompleteLifecycleActionAnswerDeserializer;
impl CompleteLifecycleActionAnswerDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CompleteLifecycleActionAnswer, XmlParseError> {
        try!(start_element(tag_name, stack));

        let obj = CompleteLifecycleActionAnswer::default();

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CompleteLifecycleActionType {
    /// <p>The name of the Auto Scaling group.</p>
    pub auto_scaling_group_name: String,
    /// <p>The ID of the instance.</p>
    pub instance_id: Option<String>,
    /// <p>The action for the group to take. This parameter can be either <code>CONTINUE</code> or <code>ABANDON</code>.</p>
    pub lifecycle_action_result: String,
    /// <p>A universally unique identifier (UUID) that identifies a specific lifecycle action associated with an instance. Auto Scaling sends this token to the notification target you specified when you created the lifecycle hook.</p>
    pub lifecycle_action_token: Option<String>,
    /// <p>The name of the lifecycle hook.</p>
    pub lifecycle_hook_name: String,
}

/// Serialize `CompleteLifecycleActionType` contents to a `SignedRequest`.
struct CompleteLifecycleActionTypeSerializer;
impl CompleteLifecycleActionTypeSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CompleteLifecycleActionType) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "AutoScalingGroupName"),
            &obj.auto_scaling_group_name.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.instance_id {
            params.put(
                &format!("{}{}", prefix, "InstanceId"),
                &field_value.replace("+", "%2B"),
            );
        }
        params.put(
            &format!("{}{}", prefix, "LifecycleActionResult"),
            &obj.lifecycle_action_result.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.lifecycle_action_token {
            params.put(
                &format!("{}{}", prefix, "LifecycleActionToken"),
                &field_value.replace("+", "%2B"),
            );
        }
        params.put(
            &format!("{}{}", prefix, "LifecycleHookName"),
            &obj.lifecycle_hook_name.replace("+", "%2B"),
        );
    }
}

struct CooldownDeserializer;
impl CooldownDeserializer {
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
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateAutoScalingGroupType {
    /// <p>The name of the Auto Scaling group. This name must be unique within the scope of your AWS account.</p>
    pub auto_scaling_group_name: String,
    /// <p>One or more Availability Zones for the group. This parameter is optional if you specify one or more subnets.</p>
    pub availability_zones: Option<Vec<String>>,
    /// <p>The amount of time, in seconds, after a scaling activity completes before another scaling activity can start. The default is 300.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/autoscaling/latest/userguide/Cooldown.html">Auto Scaling Cooldowns</a> in the <i>Auto Scaling User Guide</i>.</p>
    pub default_cooldown: Option<i64>,
    /// <p>The number of EC2 instances that should be running in the group. This number must be greater than or equal to the minimum size of the group and less than or equal to the maximum size of the group. If you do not specify a desired capacity, the default is the minimum size of the group.</p>
    pub desired_capacity: Option<i64>,
    /// <p>The amount of time, in seconds, that Auto Scaling waits before checking the health status of an EC2 instance that has come into service. During this time, any health check failures for the instance are ignored. The default is 0.</p> <p>This parameter is required if you are adding an <code>ELB</code> health check.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/autoscaling/latest/userguide/healthcheck.html">Health Checks</a> in the <i>Auto Scaling User Guide</i>.</p>
    pub health_check_grace_period: Option<i64>,
    /// <p>The service to use for the health checks. The valid values are <code>EC2</code> and <code>ELB</code>.</p> <p>By default, health checks use Amazon EC2 instance status checks to determine the health of an instance. For more information, see <a href="http://docs.aws.amazon.com/autoscaling/latest/userguide/healthcheck.html">Health Checks</a> in the <i>Auto Scaling User Guide</i>.</p>
    pub health_check_type: Option<String>,
    /// <p>The ID of the instance used to create a launch configuration for the group. You must specify one of the following: an EC2 instance, a launch configuration, or a launch template.</p> <p>When you specify an ID of an instance, Auto Scaling creates a new launch configuration and associates it with the group. This launch configuration derives its attributes from the specified instance, with the exception of the block device mapping.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/autoscaling/latest/userguide/create-asg-from-instance.html">Create an Auto Scaling Group Using an EC2 Instance</a> in the <i>Auto Scaling User Guide</i>.</p>
    pub instance_id: Option<String>,
    /// <p>The name of the launch configuration. You must specify one of the following: a launch configuration, a launch template, or an EC2 instance.</p>
    pub launch_configuration_name: Option<String>,
    /// <p>The launch template to use to launch instances. You must specify one of the following: a launch template, a launch configuration, or an EC2 instance.</p>
    pub launch_template: Option<LaunchTemplateSpecification>,
    /// <p>One or more lifecycle hooks.</p>
    pub lifecycle_hook_specification_list: Option<Vec<LifecycleHookSpecification>>,
    /// <p>One or more Classic Load Balancers. To specify an Application Load Balancer, use <code>TargetGroupARNs</code> instead.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/autoscaling/latest/userguide/create-asg-from-instance.html">Using a Load Balancer With an Auto Scaling Group</a> in the <i>Auto Scaling User Guide</i>.</p>
    pub load_balancer_names: Option<Vec<String>>,
    /// <p>The maximum size of the group.</p>
    pub max_size: i64,
    /// <p>The minimum size of the group.</p>
    pub min_size: i64,
    /// <p>Indicates whether newly launched instances are protected from termination by Auto Scaling when scaling in.</p>
    pub new_instances_protected_from_scale_in: Option<bool>,
    /// <p>The name of the placement group into which you'll launch your instances, if any. For more information, see <a href="http://docs.aws.amazon.com/AWSEC2/latest/UserGuide/placement-groups.html">Placement Groups</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
    pub placement_group: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the service-linked role that the Auto Scaling group uses to call other AWS services on your behalf. By default, Auto Scaling uses a service-linked role named AWSServiceRoleForAutoScaling, which it creates if it does not exist.</p>
    pub service_linked_role_arn: Option<String>,
    /// <p>One or more tags.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/autoscaling/latest/userguide/autoscaling-tagging.html">Tagging Auto Scaling Groups and Instances</a> in the <i>Auto Scaling User Guide</i>.</p>
    pub tags: Option<Vec<Tag>>,
    /// <p>The Amazon Resource Names (ARN) of the target groups.</p>
    pub target_group_ar_ns: Option<Vec<String>>,
    /// <p>One or more termination policies used to select the instance to terminate. These policies are executed in the order that they are listed.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/autoscaling/latest/userguide/as-instance-termination.html">Controlling Which Instances Auto Scaling Terminates During Scale In</a> in the <i>Auto Scaling User Guide</i>.</p>
    pub termination_policies: Option<Vec<String>>,
    /// <p>A comma-separated list of subnet identifiers for your virtual private cloud (VPC).</p> <p>If you specify subnets and Availability Zones with this call, ensure that the subnets' Availability Zones match the Availability Zones specified.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/autoscaling/latest/userguide/asg-in-vpc.html">Launching Auto Scaling Instances in a VPC</a> in the <i>Auto Scaling User Guide</i>.</p>
    pub vpc_zone_identifier: Option<String>,
}

/// Serialize `CreateAutoScalingGroupType` contents to a `SignedRequest`.
struct CreateAutoScalingGroupTypeSerializer;
impl CreateAutoScalingGroupTypeSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CreateAutoScalingGroupType) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "AutoScalingGroupName"),
            &obj.auto_scaling_group_name.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.availability_zones {
            AvailabilityZonesSerializer::serialize(
                params,
                &format!("{}{}", prefix, "AvailabilityZones"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.default_cooldown {
            params.put(
                &format!("{}{}", prefix, "DefaultCooldown"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.desired_capacity {
            params.put(
                &format!("{}{}", prefix, "DesiredCapacity"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.health_check_grace_period {
            params.put(
                &format!("{}{}", prefix, "HealthCheckGracePeriod"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.health_check_type {
            params.put(
                &format!("{}{}", prefix, "HealthCheckType"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.instance_id {
            params.put(
                &format!("{}{}", prefix, "InstanceId"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.launch_configuration_name {
            params.put(
                &format!("{}{}", prefix, "LaunchConfigurationName"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.launch_template {
            LaunchTemplateSpecificationSerializer::serialize(
                params,
                &format!("{}{}", prefix, "LaunchTemplate"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.lifecycle_hook_specification_list {
            LifecycleHookSpecificationsSerializer::serialize(
                params,
                &format!("{}{}", prefix, "LifecycleHookSpecificationList"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.load_balancer_names {
            LoadBalancerNamesSerializer::serialize(
                params,
                &format!("{}{}", prefix, "LoadBalancerNames"),
                field_value,
            );
        }
        params.put(
            &format!("{}{}", prefix, "MaxSize"),
            &obj.max_size.to_string().replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "MinSize"),
            &obj.min_size.to_string().replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.new_instances_protected_from_scale_in {
            params.put(
                &format!("{}{}", prefix, "NewInstancesProtectedFromScaleIn"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.placement_group {
            params.put(
                &format!("{}{}", prefix, "PlacementGroup"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.service_linked_role_arn {
            params.put(
                &format!("{}{}", prefix, "ServiceLinkedRoleARN"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.tags {
            TagsSerializer::serialize(params, &format!("{}{}", prefix, "Tags"), field_value);
        }
        if let Some(ref field_value) = obj.target_group_ar_ns {
            TargetGroupARNsSerializer::serialize(
                params,
                &format!("{}{}", prefix, "TargetGroupARNs"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.termination_policies {
            TerminationPoliciesSerializer::serialize(
                params,
                &format!("{}{}", prefix, "TerminationPolicies"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.vpc_zone_identifier {
            params.put(
                &format!("{}{}", prefix, "VPCZoneIdentifier"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateLaunchConfigurationType {
    /// <p>Used for groups that launch instances into a virtual private cloud (VPC). Specifies whether to assign a public IP address to each instance. For more information, see <a href="http://docs.aws.amazon.com/autoscaling/latest/userguide/asg-in-vpc.html">Launching Auto Scaling Instances in a VPC</a> in the <i>Auto Scaling User Guide</i>.</p> <p>If you specify this parameter, be sure to specify at least one subnet when you create your group.</p> <p>Default: If the instance is launched into a default subnet, the default is to assign a public IP address. If the instance is launched into a nondefault subnet, the default is not to assign a public IP address.</p>
    pub associate_public_ip_address: Option<bool>,
    /// <p>One or more mappings that specify how block devices are exposed to the instance. For more information, see <a href="http://docs.aws.amazon.com/AWSEC2/latest/UserGuide/block-device-mapping-concepts.html">Block Device Mapping</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
    pub block_device_mappings: Option<Vec<BlockDeviceMapping>>,
    /// <p>The ID of a ClassicLink-enabled VPC to link your EC2-Classic instances to. This parameter is supported only if you are launching EC2-Classic instances. For more information, see <a href="http://docs.aws.amazon.com/AWSEC2/latest/UserGuide/vpc-classiclink.html">ClassicLink</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
    pub classic_link_vpc_id: Option<String>,
    /// <p>The IDs of one or more security groups for the specified ClassicLink-enabled VPC. This parameter is required if you specify a ClassicLink-enabled VPC, and is not supported otherwise. For more information, see <a href="http://docs.aws.amazon.com/AWSEC2/latest/UserGuide/vpc-classiclink.html">ClassicLink</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
    pub classic_link_vpc_security_groups: Option<Vec<String>>,
    /// <p>Indicates whether the instance is optimized for Amazon EBS I/O. By default, the instance is not optimized for EBS I/O. The optimization provides dedicated throughput to Amazon EBS and an optimized configuration stack to provide optimal I/O performance. This optimization is not available with all instance types. Additional usage charges apply. For more information, see <a href="http://docs.aws.amazon.com/AWSEC2/latest/UserGuide/EBSOptimized.html">Amazon EBS-Optimized Instances</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
    pub ebs_optimized: Option<bool>,
    /// <p>The name or the Amazon Resource Name (ARN) of the instance profile associated with the IAM role for the instance.</p> <p>EC2 instances launched with an IAM role will automatically have AWS security credentials available. You can use IAM roles with Auto Scaling to automatically enable applications running on your EC2 instances to securely access other AWS resources. For more information, see <a href="http://docs.aws.amazon.com/autoscaling/latest/userguide/us-iam-role.html">Launch Auto Scaling Instances with an IAM Role</a> in the <i>Auto Scaling User Guide</i>.</p>
    pub iam_instance_profile: Option<String>,
    /// <p>The ID of the Amazon Machine Image (AMI) to use to launch your EC2 instances.</p> <p>If you do not specify <code>InstanceId</code>, you must specify <code>ImageId</code>.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/AWSEC2/latest/UserGuide/finding-an-ami.html">Finding an AMI</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
    pub image_id: Option<String>,
    /// <p>The ID of the instance to use to create the launch configuration. The new launch configuration derives attributes from the instance, with the exception of the block device mapping.</p> <p>If you do not specify <code>InstanceId</code>, you must specify both <code>ImageId</code> and <code>InstanceType</code>.</p> <p>To create a launch configuration with a block device mapping or override any other instance attributes, specify them as part of the same request.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/autoscaling/latest/userguide/create-lc-with-instanceID.html">Create a Launch Configuration Using an EC2 Instance</a> in the <i>Auto Scaling User Guide</i>.</p>
    pub instance_id: Option<String>,
    /// <p>Enables detailed monitoring (<code>true</code>) or basic monitoring (<code>false</code>) for the Auto Scaling instances. The default is <code>true</code>.</p>
    pub instance_monitoring: Option<InstanceMonitoring>,
    /// <p>The instance type of the EC2 instance.</p> <p>If you do not specify <code>InstanceId</code>, you must specify <code>InstanceType</code>.</p> <p>For information about available instance types, see <a href="http://docs.aws.amazon.com/AWSEC2/latest/UserGuide/instance-types.html#AvailableInstanceTypes">Available Instance Types</a> in the <i>Amazon Elastic Compute Cloud User Guide.</i> </p>
    pub instance_type: Option<String>,
    /// <p>The ID of the kernel associated with the AMI.</p>
    pub kernel_id: Option<String>,
    /// <p>The name of the key pair. For more information, see <a href="http://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-key-pairs.html">Amazon EC2 Key Pairs</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
    pub key_name: Option<String>,
    /// <p>The name of the launch configuration. This name must be unique within the scope of your AWS account.</p>
    pub launch_configuration_name: String,
    /// <p>The tenancy of the instance. An instance with a tenancy of <code>dedicated</code> runs on single-tenant hardware and can only be launched into a VPC.</p> <p>You must set the value of this parameter to <code>dedicated</code> if want to launch Dedicated Instances into a shared tenancy VPC (VPC with instance placement tenancy attribute set to <code>default</code>).</p> <p>If you specify this parameter, be sure to specify at least one subnet when you create your group.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/autoscaling/latest/userguide/asg-in-vpc.html">Launching Auto Scaling Instances in a VPC</a> in the <i>Auto Scaling User Guide</i>.</p> <p>Valid values: <code>default</code> | <code>dedicated</code> </p>
    pub placement_tenancy: Option<String>,
    /// <p>The ID of the RAM disk associated with the AMI.</p>
    pub ramdisk_id: Option<String>,
    /// <p>One or more security groups with which to associate the instances.</p> <p>If your instances are launched in EC2-Classic, you can either specify security group names or the security group IDs. For more information about security groups for EC2-Classic, see <a href="http://docs.aws.amazon.com/AWSEC2/latest/UserGuide/using-network-security.html">Amazon EC2 Security Groups</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p> <p>If your instances are launched into a VPC, specify security group IDs. For more information, see <a href="http://docs.aws.amazon.com/AmazonVPC/latest/UserGuide/VPC_SecurityGroups.html">Security Groups for Your VPC</a> in the <i>Amazon Virtual Private Cloud User Guide</i>.</p>
    pub security_groups: Option<Vec<String>>,
    /// <p>The maximum hourly price to be paid for any Spot Instance launched to fulfill the request. Spot Instances are launched when the price you specify exceeds the current Spot market price. For more information, see <a href="http://docs.aws.amazon.com/autoscaling/latest/userguide/US-SpotInstances.html">Launching Spot Instances in Your Auto Scaling Group</a> in the <i>Auto Scaling User Guide</i>.</p>
    pub spot_price: Option<String>,
    /// <p>The user data to make available to the launched EC2 instances. For more information, see <a href="http://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-instance-metadata.html">Instance Metadata and User Data</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
    pub user_data: Option<String>,
}

/// Serialize `CreateLaunchConfigurationType` contents to a `SignedRequest`.
struct CreateLaunchConfigurationTypeSerializer;
impl CreateLaunchConfigurationTypeSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CreateLaunchConfigurationType) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.associate_public_ip_address {
            params.put(
                &format!("{}{}", prefix, "AssociatePublicIpAddress"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.block_device_mappings {
            BlockDeviceMappingsSerializer::serialize(
                params,
                &format!("{}{}", prefix, "BlockDeviceMappings"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.classic_link_vpc_id {
            params.put(
                &format!("{}{}", prefix, "ClassicLinkVPCId"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.classic_link_vpc_security_groups {
            ClassicLinkVPCSecurityGroupsSerializer::serialize(
                params,
                &format!("{}{}", prefix, "ClassicLinkVPCSecurityGroups"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.ebs_optimized {
            params.put(
                &format!("{}{}", prefix, "EbsOptimized"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.iam_instance_profile {
            params.put(
                &format!("{}{}", prefix, "IamInstanceProfile"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.image_id {
            params.put(
                &format!("{}{}", prefix, "ImageId"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.instance_id {
            params.put(
                &format!("{}{}", prefix, "InstanceId"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.instance_monitoring {
            InstanceMonitoringSerializer::serialize(
                params,
                &format!("{}{}", prefix, "InstanceMonitoring"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.instance_type {
            params.put(
                &format!("{}{}", prefix, "InstanceType"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.kernel_id {
            params.put(
                &format!("{}{}", prefix, "KernelId"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.key_name {
            params.put(
                &format!("{}{}", prefix, "KeyName"),
                &field_value.replace("+", "%2B"),
            );
        }
        params.put(
            &format!("{}{}", prefix, "LaunchConfigurationName"),
            &obj.launch_configuration_name.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.placement_tenancy {
            params.put(
                &format!("{}{}", prefix, "PlacementTenancy"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.ramdisk_id {
            params.put(
                &format!("{}{}", prefix, "RamdiskId"),
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
        if let Some(ref field_value) = obj.spot_price {
            params.put(
                &format!("{}{}", prefix, "SpotPrice"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.user_data {
            params.put(
                &format!("{}{}", prefix, "UserData"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateOrUpdateTagsType {
    /// <p>One or more tags.</p>
    pub tags: Vec<Tag>,
}

/// Serialize `CreateOrUpdateTagsType` contents to a `SignedRequest`.
struct CreateOrUpdateTagsTypeSerializer;
impl CreateOrUpdateTagsTypeSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CreateOrUpdateTagsType) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        TagsSerializer::serialize(params, &format!("{}{}", prefix, "Tags"), &obj.tags);
    }
}

/// <p>Configures a customized metric for a target tracking policy.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CustomizedMetricSpecification {
    /// <p>The dimensions of the metric.</p>
    pub dimensions: Option<Vec<MetricDimension>>,
    /// <p>The name of the metric.</p>
    pub metric_name: String,
    /// <p>The namespace of the metric.</p>
    pub namespace: String,
    /// <p>The statistic of the metric.</p>
    pub statistic: String,
    /// <p>The unit of the metric.</p>
    pub unit: Option<String>,
}

struct CustomizedMetricSpecificationDeserializer;
impl CustomizedMetricSpecificationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CustomizedMetricSpecification, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CustomizedMetricSpecification::default();

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
                    "Dimensions" => {
                        obj.dimensions = Some(try!(MetricDimensionsDeserializer::deserialize(
                            "Dimensions",
                            stack
                        )));
                    }
                    "MetricName" => {
                        obj.metric_name =
                            try!(MetricNameDeserializer::deserialize("MetricName", stack));
                    }
                    "Namespace" => {
                        obj.namespace =
                            try!(MetricNamespaceDeserializer::deserialize("Namespace", stack));
                    }
                    "Statistic" => {
                        obj.statistic =
                            try!(MetricStatisticDeserializer::deserialize("Statistic", stack));
                    }
                    "Unit" => {
                        obj.unit = Some(try!(MetricUnitDeserializer::deserialize("Unit", stack)));
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

/// Serialize `CustomizedMetricSpecification` contents to a `SignedRequest`.
struct CustomizedMetricSpecificationSerializer;
impl CustomizedMetricSpecificationSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CustomizedMetricSpecification) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.dimensions {
            MetricDimensionsSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Dimensions"),
                field_value,
            );
        }
        params.put(
            &format!("{}{}", prefix, "MetricName"),
            &obj.metric_name.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "Namespace"),
            &obj.namespace.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "Statistic"),
            &obj.statistic.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.unit {
            params.put(
                &format!("{}{}", prefix, "Unit"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteAutoScalingGroupType {
    /// <p>The name of the Auto Scaling group.</p>
    pub auto_scaling_group_name: String,
    /// <p>Specifies that the group will be deleted along with all instances associated with the group, without waiting for all instances to be terminated. This parameter also deletes any lifecycle actions associated with the group.</p>
    pub force_delete: Option<bool>,
}

/// Serialize `DeleteAutoScalingGroupType` contents to a `SignedRequest`.
struct DeleteAutoScalingGroupTypeSerializer;
impl DeleteAutoScalingGroupTypeSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DeleteAutoScalingGroupType) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "AutoScalingGroupName"),
            &obj.auto_scaling_group_name.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.force_delete {
            params.put(
                &format!("{}{}", prefix, "ForceDelete"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteLifecycleHookAnswer {}

struct DeleteLifecycleHookAnswerDeserializer;
impl DeleteLifecycleHookAnswerDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteLifecycleHookAnswer, XmlParseError> {
        try!(start_element(tag_name, stack));

        let obj = DeleteLifecycleHookAnswer::default();

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteLifecycleHookType {
    /// <p>The name of the Auto Scaling group.</p>
    pub auto_scaling_group_name: String,
    /// <p>The name of the lifecycle hook.</p>
    pub lifecycle_hook_name: String,
}

/// Serialize `DeleteLifecycleHookType` contents to a `SignedRequest`.
struct DeleteLifecycleHookTypeSerializer;
impl DeleteLifecycleHookTypeSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DeleteLifecycleHookType) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "AutoScalingGroupName"),
            &obj.auto_scaling_group_name.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "LifecycleHookName"),
            &obj.lifecycle_hook_name.replace("+", "%2B"),
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteNotificationConfigurationType {
    /// <p>The name of the Auto Scaling group.</p>
    pub auto_scaling_group_name: String,
    /// <p>The Amazon Resource Name (ARN) of the Amazon Simple Notification Service (SNS) topic.</p>
    pub topic_arn: String,
}

/// Serialize `DeleteNotificationConfigurationType` contents to a `SignedRequest`.
struct DeleteNotificationConfigurationTypeSerializer;
impl DeleteNotificationConfigurationTypeSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DeleteNotificationConfigurationType) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "AutoScalingGroupName"),
            &obj.auto_scaling_group_name.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "TopicARN"),
            &obj.topic_arn.replace("+", "%2B"),
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeletePolicyType {
    /// <p>The name of the Auto Scaling group.</p>
    pub auto_scaling_group_name: Option<String>,
    /// <p>The name or Amazon Resource Name (ARN) of the policy.</p>
    pub policy_name: String,
}

/// Serialize `DeletePolicyType` contents to a `SignedRequest`.
struct DeletePolicyTypeSerializer;
impl DeletePolicyTypeSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DeletePolicyType) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.auto_scaling_group_name {
            params.put(
                &format!("{}{}", prefix, "AutoScalingGroupName"),
                &field_value.replace("+", "%2B"),
            );
        }
        params.put(
            &format!("{}{}", prefix, "PolicyName"),
            &obj.policy_name.replace("+", "%2B"),
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteScheduledActionType {
    /// <p>The name of the Auto Scaling group.</p>
    pub auto_scaling_group_name: String,
    /// <p>The name of the action to delete.</p>
    pub scheduled_action_name: String,
}

/// Serialize `DeleteScheduledActionType` contents to a `SignedRequest`.
struct DeleteScheduledActionTypeSerializer;
impl DeleteScheduledActionTypeSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DeleteScheduledActionType) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "AutoScalingGroupName"),
            &obj.auto_scaling_group_name.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "ScheduledActionName"),
            &obj.scheduled_action_name.replace("+", "%2B"),
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteTagsType {
    /// <p>One or more tags.</p>
    pub tags: Vec<Tag>,
}

/// Serialize `DeleteTagsType` contents to a `SignedRequest`.
struct DeleteTagsTypeSerializer;
impl DeleteTagsTypeSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DeleteTagsType) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        TagsSerializer::serialize(params, &format!("{}{}", prefix, "Tags"), &obj.tags);
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeAccountLimitsAnswer {
    /// <p>The maximum number of groups allowed for your AWS account. The default limit is 20 per region.</p>
    pub max_number_of_auto_scaling_groups: Option<i64>,
    /// <p>The maximum number of launch configurations allowed for your AWS account. The default limit is 100 per region.</p>
    pub max_number_of_launch_configurations: Option<i64>,
    /// <p>The current number of groups for your AWS account.</p>
    pub number_of_auto_scaling_groups: Option<i64>,
    /// <p>The current number of launch configurations for your AWS account.</p>
    pub number_of_launch_configurations: Option<i64>,
}

struct DescribeAccountLimitsAnswerDeserializer;
impl DescribeAccountLimitsAnswerDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeAccountLimitsAnswer, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DescribeAccountLimitsAnswer::default();

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
                    "MaxNumberOfAutoScalingGroups" => {
                        obj.max_number_of_auto_scaling_groups =
                            Some(try!(MaxNumberOfAutoScalingGroupsDeserializer::deserialize(
                                "MaxNumberOfAutoScalingGroups",
                                stack
                            )));
                    }
                    "MaxNumberOfLaunchConfigurations" => {
                        obj.max_number_of_launch_configurations = Some(try!(
                            MaxNumberOfLaunchConfigurationsDeserializer::deserialize(
                                "MaxNumberOfLaunchConfigurations",
                                stack
                            )
                        ));
                    }
                    "NumberOfAutoScalingGroups" => {
                        obj.number_of_auto_scaling_groups =
                            Some(try!(NumberOfAutoScalingGroupsDeserializer::deserialize(
                                "NumberOfAutoScalingGroups",
                                stack
                            )));
                    }
                    "NumberOfLaunchConfigurations" => {
                        obj.number_of_launch_configurations =
                            Some(try!(NumberOfLaunchConfigurationsDeserializer::deserialize(
                                "NumberOfLaunchConfigurations",
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
pub struct DescribeAdjustmentTypesAnswer {
    /// <p>The policy adjustment types.</p>
    pub adjustment_types: Option<Vec<AdjustmentType>>,
}

struct DescribeAdjustmentTypesAnswerDeserializer;
impl DescribeAdjustmentTypesAnswerDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeAdjustmentTypesAnswer, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DescribeAdjustmentTypesAnswer::default();

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
                    "AdjustmentTypes" => {
                        obj.adjustment_types = Some(try!(
                            AdjustmentTypesDeserializer::deserialize("AdjustmentTypes", stack)
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
pub struct DescribeAutoScalingInstancesType {
    /// <p>The instances to describe; up to 50 instance IDs. If you omit this parameter, all Auto Scaling instances are described. If you specify an ID that does not exist, it is ignored with no error.</p>
    pub instance_ids: Option<Vec<String>>,
    /// <p>The maximum number of items to return with this call. The default value is 50 and the maximum value is 50.</p>
    pub max_records: Option<i64>,
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    pub next_token: Option<String>,
}

/// Serialize `DescribeAutoScalingInstancesType` contents to a `SignedRequest`.
struct DescribeAutoScalingInstancesTypeSerializer;
impl DescribeAutoScalingInstancesTypeSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeAutoScalingInstancesType) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.instance_ids {
            InstanceIdsSerializer::serialize(
                params,
                &format!("{}{}", prefix, "InstanceIds"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.max_records {
            params.put(
                &format!("{}{}", prefix, "MaxRecords"),
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
pub struct DescribeAutoScalingNotificationTypesAnswer {
    /// <p>The notification types.</p>
    pub auto_scaling_notification_types: Option<Vec<String>>,
}

struct DescribeAutoScalingNotificationTypesAnswerDeserializer;
impl DescribeAutoScalingNotificationTypesAnswerDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeAutoScalingNotificationTypesAnswer, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DescribeAutoScalingNotificationTypesAnswer::default();

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
                    "AutoScalingNotificationTypes" => {
                        obj.auto_scaling_notification_types =
                            Some(try!(AutoScalingNotificationTypesDeserializer::deserialize(
                                "AutoScalingNotificationTypes",
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
pub struct DescribeLifecycleHookTypesAnswer {
    /// <p>The lifecycle hook types.</p>
    pub lifecycle_hook_types: Option<Vec<String>>,
}

struct DescribeLifecycleHookTypesAnswerDeserializer;
impl DescribeLifecycleHookTypesAnswerDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeLifecycleHookTypesAnswer, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DescribeLifecycleHookTypesAnswer::default();

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
                    "LifecycleHookTypes" => {
                        obj.lifecycle_hook_types =
                            Some(try!(AutoScalingNotificationTypesDeserializer::deserialize(
                                "LifecycleHookTypes",
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
pub struct DescribeLifecycleHooksAnswer {
    /// <p>The lifecycle hooks for the specified group.</p>
    pub lifecycle_hooks: Option<Vec<LifecycleHook>>,
}

struct DescribeLifecycleHooksAnswerDeserializer;
impl DescribeLifecycleHooksAnswerDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeLifecycleHooksAnswer, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DescribeLifecycleHooksAnswer::default();

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
                    "LifecycleHooks" => {
                        obj.lifecycle_hooks = Some(try!(LifecycleHooksDeserializer::deserialize(
                            "LifecycleHooks",
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
pub struct DescribeLifecycleHooksType {
    /// <p>The name of the Auto Scaling group.</p>
    pub auto_scaling_group_name: String,
    /// <p>The names of one or more lifecycle hooks. If you omit this parameter, all lifecycle hooks are described.</p>
    pub lifecycle_hook_names: Option<Vec<String>>,
}

/// Serialize `DescribeLifecycleHooksType` contents to a `SignedRequest`.
struct DescribeLifecycleHooksTypeSerializer;
impl DescribeLifecycleHooksTypeSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeLifecycleHooksType) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "AutoScalingGroupName"),
            &obj.auto_scaling_group_name.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.lifecycle_hook_names {
            LifecycleHookNamesSerializer::serialize(
                params,
                &format!("{}{}", prefix, "LifecycleHookNames"),
                field_value,
            );
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeLoadBalancerTargetGroupsRequest {
    /// <p>The name of the Auto Scaling group.</p>
    pub auto_scaling_group_name: String,
    /// <p>The maximum number of items to return with this call. The default value is 100 and the maximum value is 100.</p>
    pub max_records: Option<i64>,
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    pub next_token: Option<String>,
}

/// Serialize `DescribeLoadBalancerTargetGroupsRequest` contents to a `SignedRequest`.
struct DescribeLoadBalancerTargetGroupsRequestSerializer;
impl DescribeLoadBalancerTargetGroupsRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeLoadBalancerTargetGroupsRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "AutoScalingGroupName"),
            &obj.auto_scaling_group_name.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.max_records {
            params.put(
                &format!("{}{}", prefix, "MaxRecords"),
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
pub struct DescribeLoadBalancerTargetGroupsResponse {
    /// <p>Information about the target groups.</p>
    pub load_balancer_target_groups: Option<Vec<LoadBalancerTargetGroupState>>,
    /// <p>The token to use when requesting the next set of items. If there are no additional items to return, the string is empty.</p>
    pub next_token: Option<String>,
}

struct DescribeLoadBalancerTargetGroupsResponseDeserializer;
impl DescribeLoadBalancerTargetGroupsResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeLoadBalancerTargetGroupsResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DescribeLoadBalancerTargetGroupsResponse::default();

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
                    "LoadBalancerTargetGroups" => {
                        obj.load_balancer_target_groups = Some(try!(
                            LoadBalancerTargetGroupStatesDeserializer::deserialize(
                                "LoadBalancerTargetGroups",
                                stack
                            )
                        ));
                    }
                    "NextToken" => {
                        obj.next_token =
                            Some(try!(XmlStringDeserializer::deserialize("NextToken", stack)));
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
pub struct DescribeLoadBalancersRequest {
    /// <p>The name of the Auto Scaling group.</p>
    pub auto_scaling_group_name: String,
    /// <p>The maximum number of items to return with this call. The default value is 100 and the maximum value is 100.</p>
    pub max_records: Option<i64>,
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    pub next_token: Option<String>,
}

/// Serialize `DescribeLoadBalancersRequest` contents to a `SignedRequest`.
struct DescribeLoadBalancersRequestSerializer;
impl DescribeLoadBalancersRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeLoadBalancersRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "AutoScalingGroupName"),
            &obj.auto_scaling_group_name.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.max_records {
            params.put(
                &format!("{}{}", prefix, "MaxRecords"),
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
pub struct DescribeLoadBalancersResponse {
    /// <p>The load balancers.</p>
    pub load_balancers: Option<Vec<LoadBalancerState>>,
    /// <p>The token to use when requesting the next set of items. If there are no additional items to return, the string is empty.</p>
    pub next_token: Option<String>,
}

struct DescribeLoadBalancersResponseDeserializer;
impl DescribeLoadBalancersResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeLoadBalancersResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DescribeLoadBalancersResponse::default();

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
                        obj.load_balancers = Some(try!(
                            LoadBalancerStatesDeserializer::deserialize("LoadBalancers", stack)
                        ));
                    }
                    "NextToken" => {
                        obj.next_token =
                            Some(try!(XmlStringDeserializer::deserialize("NextToken", stack)));
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
pub struct DescribeMetricCollectionTypesAnswer {
    /// <p>The granularities for the metrics.</p>
    pub granularities: Option<Vec<MetricGranularityType>>,
    /// <p>One or more metrics.</p>
    pub metrics: Option<Vec<MetricCollectionType>>,
}

struct DescribeMetricCollectionTypesAnswerDeserializer;
impl DescribeMetricCollectionTypesAnswerDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeMetricCollectionTypesAnswer, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DescribeMetricCollectionTypesAnswer::default();

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
                    "Granularities" => {
                        obj.granularities = Some(try!(
                            MetricGranularityTypesDeserializer::deserialize("Granularities", stack)
                        ));
                    }
                    "Metrics" => {
                        obj.metrics = Some(try!(MetricCollectionTypesDeserializer::deserialize(
                            "Metrics", stack
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
pub struct DescribeNotificationConfigurationsAnswer {
    /// <p>The token to use when requesting the next set of items. If there are no additional items to return, the string is empty.</p>
    pub next_token: Option<String>,
    /// <p>The notification configurations.</p>
    pub notification_configurations: Vec<NotificationConfiguration>,
}

struct DescribeNotificationConfigurationsAnswerDeserializer;
impl DescribeNotificationConfigurationsAnswerDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeNotificationConfigurationsAnswer, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DescribeNotificationConfigurationsAnswer::default();

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
                            Some(try!(XmlStringDeserializer::deserialize("NextToken", stack)));
                    }
                    "NotificationConfigurations" => {
                        obj.notification_configurations =
                            try!(NotificationConfigurationsDeserializer::deserialize(
                                "NotificationConfigurations",
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
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeNotificationConfigurationsType {
    /// <p>The name of the Auto Scaling group.</p>
    pub auto_scaling_group_names: Option<Vec<String>>,
    /// <p>The maximum number of items to return with this call. The default value is 50 and the maximum value is 100.</p>
    pub max_records: Option<i64>,
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    pub next_token: Option<String>,
}

/// Serialize `DescribeNotificationConfigurationsType` contents to a `SignedRequest`.
struct DescribeNotificationConfigurationsTypeSerializer;
impl DescribeNotificationConfigurationsTypeSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeNotificationConfigurationsType) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.auto_scaling_group_names {
            AutoScalingGroupNamesSerializer::serialize(
                params,
                &format!("{}{}", prefix, "AutoScalingGroupNames"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.max_records {
            params.put(
                &format!("{}{}", prefix, "MaxRecords"),
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
pub struct DescribePoliciesType {
    /// <p>The name of the Auto Scaling group.</p>
    pub auto_scaling_group_name: Option<String>,
    /// <p>The maximum number of items to be returned with each call. The default value is 50 and the maximum value is 100.</p>
    pub max_records: Option<i64>,
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    pub next_token: Option<String>,
    /// <p>The names of one or more policies. If you omit this parameter, all policies are described. If an group name is provided, the results are limited to that group. This list is limited to 50 items. If you specify an unknown policy name, it is ignored with no error.</p>
    pub policy_names: Option<Vec<String>>,
    /// <p>One or more policy types. Valid values are <code>SimpleScaling</code> and <code>StepScaling</code>.</p>
    pub policy_types: Option<Vec<String>>,
}

/// Serialize `DescribePoliciesType` contents to a `SignedRequest`.
struct DescribePoliciesTypeSerializer;
impl DescribePoliciesTypeSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribePoliciesType) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.auto_scaling_group_name {
            params.put(
                &format!("{}{}", prefix, "AutoScalingGroupName"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.max_records {
            params.put(
                &format!("{}{}", prefix, "MaxRecords"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.next_token {
            params.put(
                &format!("{}{}", prefix, "NextToken"),
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
        if let Some(ref field_value) = obj.policy_types {
            PolicyTypesSerializer::serialize(
                params,
                &format!("{}{}", prefix, "PolicyTypes"),
                field_value,
            );
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeScalingActivitiesType {
    /// <p>The activity IDs of the desired scaling activities. If you omit this parameter, all activities for the past six weeks are described. If you specify an Auto Scaling group, the results are limited to that group. The list of requested activities cannot contain more than 50 items. If unknown activities are requested, they are ignored with no error.</p>
    pub activity_ids: Option<Vec<String>>,
    /// <p>The name of the Auto Scaling group.</p>
    pub auto_scaling_group_name: Option<String>,
    /// <p>The maximum number of items to return with this call. The default value is 100 and the maximum value is 100.</p>
    pub max_records: Option<i64>,
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    pub next_token: Option<String>,
}

/// Serialize `DescribeScalingActivitiesType` contents to a `SignedRequest`.
struct DescribeScalingActivitiesTypeSerializer;
impl DescribeScalingActivitiesTypeSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeScalingActivitiesType) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.activity_ids {
            ActivityIdsSerializer::serialize(
                params,
                &format!("{}{}", prefix, "ActivityIds"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.auto_scaling_group_name {
            params.put(
                &format!("{}{}", prefix, "AutoScalingGroupName"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.max_records {
            params.put(
                &format!("{}{}", prefix, "MaxRecords"),
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
pub struct DescribeScheduledActionsType {
    /// <p>The name of the Auto Scaling group.</p>
    pub auto_scaling_group_name: Option<String>,
    /// <p>The latest scheduled start time to return. If scheduled action names are provided, this parameter is ignored.</p>
    pub end_time: Option<String>,
    /// <p>The maximum number of items to return with this call. The default value is 50 and the maximum value is 100.</p>
    pub max_records: Option<i64>,
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    pub next_token: Option<String>,
    /// <p>Describes one or more scheduled actions. If you omit this parameter, all scheduled actions are described. If you specify an unknown scheduled action, it is ignored with no error.</p> <p>You can describe up to a maximum of 50 instances with a single call. If there are more items to return, the call returns a token. To get the next set of items, repeat the call with the returned token.</p>
    pub scheduled_action_names: Option<Vec<String>>,
    /// <p>The earliest scheduled start time to return. If scheduled action names are provided, this parameter is ignored.</p>
    pub start_time: Option<String>,
}

/// Serialize `DescribeScheduledActionsType` contents to a `SignedRequest`.
struct DescribeScheduledActionsTypeSerializer;
impl DescribeScheduledActionsTypeSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeScheduledActionsType) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.auto_scaling_group_name {
            params.put(
                &format!("{}{}", prefix, "AutoScalingGroupName"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.end_time {
            params.put(
                &format!("{}{}", prefix, "EndTime"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.max_records {
            params.put(
                &format!("{}{}", prefix, "MaxRecords"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.next_token {
            params.put(
                &format!("{}{}", prefix, "NextToken"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.scheduled_action_names {
            ScheduledActionNamesSerializer::serialize(
                params,
                &format!("{}{}", prefix, "ScheduledActionNames"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.start_time {
            params.put(
                &format!("{}{}", prefix, "StartTime"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeTagsType {
    /// <p>A filter used to scope the tags to return.</p>
    pub filters: Option<Vec<Filter>>,
    /// <p>The maximum number of items to return with this call. The default value is 50 and the maximum value is 100.</p>
    pub max_records: Option<i64>,
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    pub next_token: Option<String>,
}

/// Serialize `DescribeTagsType` contents to a `SignedRequest`.
struct DescribeTagsTypeSerializer;
impl DescribeTagsTypeSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeTagsType) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.filters {
            FiltersSerializer::serialize(params, &format!("{}{}", prefix, "Filters"), field_value);
        }
        if let Some(ref field_value) = obj.max_records {
            params.put(
                &format!("{}{}", prefix, "MaxRecords"),
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
pub struct DescribeTerminationPolicyTypesAnswer {
    /// <p>The termination policies supported by Auto Scaling (<code>OldestInstance</code>, <code>OldestLaunchConfiguration</code>, <code>NewestInstance</code>, <code>ClosestToNextInstanceHour</code>, and <code>Default</code>).</p>
    pub termination_policy_types: Option<Vec<String>>,
}

struct DescribeTerminationPolicyTypesAnswerDeserializer;
impl DescribeTerminationPolicyTypesAnswerDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeTerminationPolicyTypesAnswer, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DescribeTerminationPolicyTypesAnswer::default();

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
                    "TerminationPolicyTypes" => {
                        obj.termination_policy_types =
                            Some(try!(TerminationPoliciesDeserializer::deserialize(
                                "TerminationPolicyTypes",
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
pub struct DetachInstancesAnswer {
    /// <p>The activities related to detaching the instances from the Auto Scaling group.</p>
    pub activities: Option<Vec<Activity>>,
}

struct DetachInstancesAnswerDeserializer;
impl DetachInstancesAnswerDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DetachInstancesAnswer, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DetachInstancesAnswer::default();

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
                    "Activities" => {
                        obj.activities = Some(try!(ActivitiesDeserializer::deserialize(
                            "Activities",
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
pub struct DetachInstancesQuery {
    /// <p>The name of the Auto Scaling group.</p>
    pub auto_scaling_group_name: String,
    /// <p>The IDs of the instances. You can specify up to 20 instances.</p>
    pub instance_ids: Option<Vec<String>>,
    /// <p>Indicates whether the Auto Scaling group decrements the desired capacity value by the number of instances detached.</p>
    pub should_decrement_desired_capacity: bool,
}

/// Serialize `DetachInstancesQuery` contents to a `SignedRequest`.
struct DetachInstancesQuerySerializer;
impl DetachInstancesQuerySerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DetachInstancesQuery) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "AutoScalingGroupName"),
            &obj.auto_scaling_group_name.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.instance_ids {
            InstanceIdsSerializer::serialize(
                params,
                &format!("{}{}", prefix, "InstanceIds"),
                field_value,
            );
        }
        params.put(
            &format!("{}{}", prefix, "ShouldDecrementDesiredCapacity"),
            &obj.should_decrement_desired_capacity
                .to_string()
                .replace("+", "%2B"),
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DetachLoadBalancerTargetGroupsResultType {}

struct DetachLoadBalancerTargetGroupsResultTypeDeserializer;
impl DetachLoadBalancerTargetGroupsResultTypeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DetachLoadBalancerTargetGroupsResultType, XmlParseError> {
        try!(start_element(tag_name, stack));

        let obj = DetachLoadBalancerTargetGroupsResultType::default();

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DetachLoadBalancerTargetGroupsType {
    /// <p>The name of the Auto Scaling group.</p>
    pub auto_scaling_group_name: String,
    /// <p>The Amazon Resource Names (ARN) of the target groups. You can specify up to 10 target groups.</p>
    pub target_group_ar_ns: Vec<String>,
}

/// Serialize `DetachLoadBalancerTargetGroupsType` contents to a `SignedRequest`.
struct DetachLoadBalancerTargetGroupsTypeSerializer;
impl DetachLoadBalancerTargetGroupsTypeSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DetachLoadBalancerTargetGroupsType) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "AutoScalingGroupName"),
            &obj.auto_scaling_group_name.replace("+", "%2B"),
        );
        TargetGroupARNsSerializer::serialize(
            params,
            &format!("{}{}", prefix, "TargetGroupARNs"),
            &obj.target_group_ar_ns,
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DetachLoadBalancersResultType {}

struct DetachLoadBalancersResultTypeDeserializer;
impl DetachLoadBalancersResultTypeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DetachLoadBalancersResultType, XmlParseError> {
        try!(start_element(tag_name, stack));

        let obj = DetachLoadBalancersResultType::default();

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DetachLoadBalancersType {
    /// <p>The name of the Auto Scaling group.</p>
    pub auto_scaling_group_name: String,
    /// <p>The names of the load balancers. You can specify up to 10 load balancers.</p>
    pub load_balancer_names: Vec<String>,
}

/// Serialize `DetachLoadBalancersType` contents to a `SignedRequest`.
struct DetachLoadBalancersTypeSerializer;
impl DetachLoadBalancersTypeSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DetachLoadBalancersType) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "AutoScalingGroupName"),
            &obj.auto_scaling_group_name.replace("+", "%2B"),
        );
        LoadBalancerNamesSerializer::serialize(
            params,
            &format!("{}{}", prefix, "LoadBalancerNames"),
            &obj.load_balancer_names,
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DisableMetricsCollectionQuery {
    /// <p>The name of the Auto Scaling group.</p>
    pub auto_scaling_group_name: String,
    /// <p><p>One or more of the following metrics. If you omit this parameter, all metrics are disabled.</p> <ul> <li> <p> <code>GroupMinSize</code> </p> </li> <li> <p> <code>GroupMaxSize</code> </p> </li> <li> <p> <code>GroupDesiredCapacity</code> </p> </li> <li> <p> <code>GroupInServiceInstances</code> </p> </li> <li> <p> <code>GroupPendingInstances</code> </p> </li> <li> <p> <code>GroupStandbyInstances</code> </p> </li> <li> <p> <code>GroupTerminatingInstances</code> </p> </li> <li> <p> <code>GroupTotalInstances</code> </p> </li> </ul></p>
    pub metrics: Option<Vec<String>>,
}

/// Serialize `DisableMetricsCollectionQuery` contents to a `SignedRequest`.
struct DisableMetricsCollectionQuerySerializer;
impl DisableMetricsCollectionQuerySerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DisableMetricsCollectionQuery) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "AutoScalingGroupName"),
            &obj.auto_scaling_group_name.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.metrics {
            MetricsSerializer::serialize(params, &format!("{}{}", prefix, "Metrics"), field_value);
        }
    }
}

struct DisableScaleInDeserializer;
impl DisableScaleInDeserializer {
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
/// <p>Describes an Amazon EBS volume.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Ebs {
    /// <p>Indicates whether the volume is deleted on instance termination. The default is <code>true</code>.</p>
    pub delete_on_termination: Option<bool>,
    /// <p>Indicates whether the volume should be encrypted. Encrypted EBS volumes must be attached to instances that support Amazon EBS encryption. Volumes that are created from encrypted snapshots are automatically encrypted. There is no way to create an encrypted volume from an unencrypted snapshot or an unencrypted volume from an encrypted snapshot. For more information, see <a href="http://docs.aws.amazon.com/AWSEC2/latest/UserGuide/EBSEncryption.html">Amazon EBS Encryption</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
    pub encrypted: Option<bool>,
    /// <p>The number of I/O operations per second (IOPS) to provision for the volume.</p> <p>Constraint: Required when the volume type is <code>io1</code>.</p>
    pub iops: Option<i64>,
    /// <p>The ID of the snapshot.</p>
    pub snapshot_id: Option<String>,
    /// <p>The volume size, in GiB. For <code>standard</code> volumes, specify a value from 1 to 1,024. For <code>io1</code> volumes, specify a value from 4 to 16,384. For <code>gp2</code> volumes, specify a value from 1 to 16,384. If you specify a snapshot, the volume size must be equal to or larger than the snapshot size.</p> <p>Default: If you create a volume from a snapshot and you don't specify a volume size, the default is the snapshot size.</p>
    pub volume_size: Option<i64>,
    /// <p>The volume type. For more information, see <a href="http://docs.aws.amazon.com/AWSEC2/latest/UserGuide/EBSVolumeTypes.html">Amazon EBS Volume Types</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p> <p>Valid values: <code>standard</code> | <code>io1</code> | <code>gp2</code> </p> <p>Default: <code>standard</code> </p>
    pub volume_type: Option<String>,
}

struct EbsDeserializer;
impl EbsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Ebs, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = Ebs::default();

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
                    "DeleteOnTermination" => {
                        obj.delete_on_termination = Some(try!(
                            BlockDeviceEbsDeleteOnTerminationDeserializer::deserialize(
                                "DeleteOnTermination",
                                stack
                            )
                        ));
                    }
                    "Encrypted" => {
                        obj.encrypted = Some(try!(
                            BlockDeviceEbsEncryptedDeserializer::deserialize("Encrypted", stack)
                        ));
                    }
                    "Iops" => {
                        obj.iops = Some(try!(BlockDeviceEbsIopsDeserializer::deserialize(
                            "Iops", stack
                        )));
                    }
                    "SnapshotId" => {
                        obj.snapshot_id = Some(try!(XmlStringMaxLen255Deserializer::deserialize(
                            "SnapshotId",
                            stack
                        )));
                    }
                    "VolumeSize" => {
                        obj.volume_size = Some(try!(
                            BlockDeviceEbsVolumeSizeDeserializer::deserialize("VolumeSize", stack)
                        ));
                    }
                    "VolumeType" => {
                        obj.volume_type = Some(try!(
                            BlockDeviceEbsVolumeTypeDeserializer::deserialize("VolumeType", stack)
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

/// Serialize `Ebs` contents to a `SignedRequest`.
struct EbsSerializer;
impl EbsSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Ebs) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.delete_on_termination {
            params.put(
                &format!("{}{}", prefix, "DeleteOnTermination"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.encrypted {
            params.put(
                &format!("{}{}", prefix, "Encrypted"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.iops {
            params.put(
                &format!("{}{}", prefix, "Iops"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.snapshot_id {
            params.put(
                &format!("{}{}", prefix, "SnapshotId"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.volume_size {
            params.put(
                &format!("{}{}", prefix, "VolumeSize"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.volume_type {
            params.put(
                &format!("{}{}", prefix, "VolumeType"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

struct EbsOptimizedDeserializer;
impl EbsOptimizedDeserializer {
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
pub struct EnableMetricsCollectionQuery {
    /// <p>The name of the Auto Scaling group.</p>
    pub auto_scaling_group_name: String,
    /// <p>The granularity to associate with the metrics to collect. The only valid value is <code>1Minute</code>.</p>
    pub granularity: String,
    /// <p><p>One or more of the following metrics. If you omit this parameter, all metrics are enabled.</p> <ul> <li> <p> <code>GroupMinSize</code> </p> </li> <li> <p> <code>GroupMaxSize</code> </p> </li> <li> <p> <code>GroupDesiredCapacity</code> </p> </li> <li> <p> <code>GroupInServiceInstances</code> </p> </li> <li> <p> <code>GroupPendingInstances</code> </p> </li> <li> <p> <code>GroupStandbyInstances</code> </p> </li> <li> <p> <code>GroupTerminatingInstances</code> </p> </li> <li> <p> <code>GroupTotalInstances</code> </p> </li> </ul></p>
    pub metrics: Option<Vec<String>>,
}

/// Serialize `EnableMetricsCollectionQuery` contents to a `SignedRequest`.
struct EnableMetricsCollectionQuerySerializer;
impl EnableMetricsCollectionQuerySerializer {
    fn serialize(params: &mut Params, name: &str, obj: &EnableMetricsCollectionQuery) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "AutoScalingGroupName"),
            &obj.auto_scaling_group_name.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "Granularity"),
            &obj.granularity.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.metrics {
            MetricsSerializer::serialize(params, &format!("{}{}", prefix, "Metrics"), field_value);
        }
    }
}

/// <p>Describes an enabled metric.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct EnabledMetric {
    /// <p>The granularity of the metric. The only valid value is <code>1Minute</code>.</p>
    pub granularity: Option<String>,
    /// <p><p>One of the following metrics:</p> <ul> <li> <p> <code>GroupMinSize</code> </p> </li> <li> <p> <code>GroupMaxSize</code> </p> </li> <li> <p> <code>GroupDesiredCapacity</code> </p> </li> <li> <p> <code>GroupInServiceInstances</code> </p> </li> <li> <p> <code>GroupPendingInstances</code> </p> </li> <li> <p> <code>GroupStandbyInstances</code> </p> </li> <li> <p> <code>GroupTerminatingInstances</code> </p> </li> <li> <p> <code>GroupTotalInstances</code> </p> </li> </ul></p>
    pub metric: Option<String>,
}

struct EnabledMetricDeserializer;
impl EnabledMetricDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<EnabledMetric, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = EnabledMetric::default();

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
                    "Granularity" => {
                        obj.granularity = Some(try!(XmlStringMaxLen255Deserializer::deserialize(
                            "Granularity",
                            stack
                        )));
                    }
                    "Metric" => {
                        obj.metric = Some(try!(XmlStringMaxLen255Deserializer::deserialize(
                            "Metric", stack
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
struct EnabledMetricsDeserializer;
impl EnabledMetricsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<EnabledMetric>, XmlParseError> {
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
                        obj.push(try!(EnabledMetricDeserializer::deserialize(
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
#[derive(Default, Debug, Clone, PartialEq)]
pub struct EnterStandbyAnswer {
    /// <p>The activities related to moving instances into <code>Standby</code> mode.</p>
    pub activities: Option<Vec<Activity>>,
}

struct EnterStandbyAnswerDeserializer;
impl EnterStandbyAnswerDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<EnterStandbyAnswer, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = EnterStandbyAnswer::default();

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
                    "Activities" => {
                        obj.activities = Some(try!(ActivitiesDeserializer::deserialize(
                            "Activities",
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
pub struct EnterStandbyQuery {
    /// <p>The name of the Auto Scaling group.</p>
    pub auto_scaling_group_name: String,
    /// <p>The IDs of the instances. You can specify up to 20 instances.</p>
    pub instance_ids: Option<Vec<String>>,
    /// <p>Indicates whether to decrement the desired capacity of the Auto Scaling group by the number of instances moved to <code>Standby</code> mode.</p>
    pub should_decrement_desired_capacity: bool,
}

/// Serialize `EnterStandbyQuery` contents to a `SignedRequest`.
struct EnterStandbyQuerySerializer;
impl EnterStandbyQuerySerializer {
    fn serialize(params: &mut Params, name: &str, obj: &EnterStandbyQuery) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "AutoScalingGroupName"),
            &obj.auto_scaling_group_name.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.instance_ids {
            InstanceIdsSerializer::serialize(
                params,
                &format!("{}{}", prefix, "InstanceIds"),
                field_value,
            );
        }
        params.put(
            &format!("{}{}", prefix, "ShouldDecrementDesiredCapacity"),
            &obj.should_decrement_desired_capacity
                .to_string()
                .replace("+", "%2B"),
        );
    }
}

struct EstimatedInstanceWarmupDeserializer;
impl EstimatedInstanceWarmupDeserializer {
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
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ExecutePolicyType {
    /// <p>The name of the Auto Scaling group.</p>
    pub auto_scaling_group_name: Option<String>,
    /// <p>The breach threshold for the alarm.</p> <p>This parameter is required if the policy type is <code>StepScaling</code> and not supported otherwise.</p>
    pub breach_threshold: Option<f64>,
    /// <p>Indicates whether Auto Scaling waits for the cooldown period to complete before executing the policy.</p> <p>This parameter is not supported if the policy type is <code>StepScaling</code>.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/autoscaling/latest/userguide/Cooldown.html">Auto Scaling Cooldowns</a> in the <i>Auto Scaling User Guide</i>.</p>
    pub honor_cooldown: Option<bool>,
    /// <p>The metric value to compare to <code>BreachThreshold</code>. This enables you to execute a policy of type <code>StepScaling</code> and determine which step adjustment to use. For example, if the breach threshold is 50 and you want to use a step adjustment with a lower bound of 0 and an upper bound of 10, you can set the metric value to 59.</p> <p>If you specify a metric value that doesn't correspond to a step adjustment for the policy, the call returns an error.</p> <p>This parameter is required if the policy type is <code>StepScaling</code> and not supported otherwise.</p>
    pub metric_value: Option<f64>,
    /// <p>The name or ARN of the policy.</p>
    pub policy_name: String,
}

/// Serialize `ExecutePolicyType` contents to a `SignedRequest`.
struct ExecutePolicyTypeSerializer;
impl ExecutePolicyTypeSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ExecutePolicyType) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.auto_scaling_group_name {
            params.put(
                &format!("{}{}", prefix, "AutoScalingGroupName"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.breach_threshold {
            params.put(
                &format!("{}{}", prefix, "BreachThreshold"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.honor_cooldown {
            params.put(
                &format!("{}{}", prefix, "HonorCooldown"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.metric_value {
            params.put(
                &format!("{}{}", prefix, "MetricValue"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        params.put(
            &format!("{}{}", prefix, "PolicyName"),
            &obj.policy_name.replace("+", "%2B"),
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct ExitStandbyAnswer {
    /// <p>The activities related to moving instances out of <code>Standby</code> mode.</p>
    pub activities: Option<Vec<Activity>>,
}

struct ExitStandbyAnswerDeserializer;
impl ExitStandbyAnswerDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ExitStandbyAnswer, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ExitStandbyAnswer::default();

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
                    "Activities" => {
                        obj.activities = Some(try!(ActivitiesDeserializer::deserialize(
                            "Activities",
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
pub struct ExitStandbyQuery {
    /// <p>The name of the Auto Scaling group.</p>
    pub auto_scaling_group_name: String,
    /// <p>The IDs of the instances. You can specify up to 20 instances.</p>
    pub instance_ids: Option<Vec<String>>,
}

/// Serialize `ExitStandbyQuery` contents to a `SignedRequest`.
struct ExitStandbyQuerySerializer;
impl ExitStandbyQuerySerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ExitStandbyQuery) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "AutoScalingGroupName"),
            &obj.auto_scaling_group_name.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.instance_ids {
            InstanceIdsSerializer::serialize(
                params,
                &format!("{}{}", prefix, "InstanceIds"),
                field_value,
            );
        }
    }
}

/// <p>Describes a filter.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Filter {
    /// <p>The name of the filter. The valid values are: <code>"auto-scaling-group"</code>, <code>"key"</code>, <code>"value"</code>, and <code>"propagate-at-launch"</code>.</p>
    pub name: Option<String>,
    /// <p>The value of the filter.</p>
    pub values: Option<Vec<String>>,
}

/// Serialize `Filter` contents to a `SignedRequest`.
struct FilterSerializer;
impl FilterSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Filter) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.name {
            params.put(
                &format!("{}{}", prefix, "Name"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.values {
            ValuesSerializer::serialize(params, &format!("{}{}", prefix, "Values"), field_value);
        }
    }
}

/// Serialize `Filters` contents to a `SignedRequest`.
struct FiltersSerializer;
impl FiltersSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<Filter>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            FilterSerializer::serialize(params, &key, obj);
        }
    }
}

struct GlobalTimeoutDeserializer;
impl GlobalTimeoutDeserializer {
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
struct HealthCheckGracePeriodDeserializer;
impl HealthCheckGracePeriodDeserializer {
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
struct HeartbeatTimeoutDeserializer;
impl HeartbeatTimeoutDeserializer {
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
/// <p>Describes an EC2 instance.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Instance {
    /// <p>The Availability Zone in which the instance is running.</p>
    pub availability_zone: String,
    /// <p>The last reported health status of the instance. "Healthy" means that the instance is healthy and should remain in service. "Unhealthy" means that the instance is unhealthy and Auto Scaling should terminate and replace it.</p>
    pub health_status: String,
    /// <p>The ID of the instance.</p>
    pub instance_id: String,
    /// <p>The launch configuration associated with the instance.</p>
    pub launch_configuration_name: Option<String>,
    /// <p>The launch template for the instance.</p>
    pub launch_template: Option<LaunchTemplateSpecification>,
    /// <p>A description of the current lifecycle state. Note that the <code>Quarantined</code> state is not used.</p>
    pub lifecycle_state: String,
    /// <p>Indicates whether the instance is protected from termination by Auto Scaling when scaling in.</p>
    pub protected_from_scale_in: bool,
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
                    "AvailabilityZone" => {
                        obj.availability_zone = try!(XmlStringMaxLen255Deserializer::deserialize(
                            "AvailabilityZone",
                            stack
                        ));
                    }
                    "HealthStatus" => {
                        obj.health_status = try!(XmlStringMaxLen32Deserializer::deserialize(
                            "HealthStatus",
                            stack
                        ));
                    }
                    "InstanceId" => {
                        obj.instance_id = try!(XmlStringMaxLen19Deserializer::deserialize(
                            "InstanceId",
                            stack
                        ));
                    }
                    "LaunchConfigurationName" => {
                        obj.launch_configuration_name =
                            Some(try!(XmlStringMaxLen255Deserializer::deserialize(
                                "LaunchConfigurationName",
                                stack
                            )));
                    }
                    "LaunchTemplate" => {
                        obj.launch_template =
                            Some(try!(LaunchTemplateSpecificationDeserializer::deserialize(
                                "LaunchTemplate",
                                stack
                            )));
                    }
                    "LifecycleState" => {
                        obj.lifecycle_state = try!(LifecycleStateDeserializer::deserialize(
                            "LifecycleState",
                            stack
                        ));
                    }
                    "ProtectedFromScaleIn" => {
                        obj.protected_from_scale_in =
                            try!(InstanceProtectedDeserializer::deserialize(
                                "ProtectedFromScaleIn",
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

/// Serialize `InstanceIds` contents to a `SignedRequest`.
struct InstanceIdsSerializer;
impl InstanceIdsSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

/// <p>Describes whether detailed monitoring is enabled for the Auto Scaling instances.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct InstanceMonitoring {
    /// <p>If <code>true</code>, detailed monitoring is enabled. Otherwise, basic monitoring is enabled.</p>
    pub enabled: Option<bool>,
}

struct InstanceMonitoringDeserializer;
impl InstanceMonitoringDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<InstanceMonitoring, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = InstanceMonitoring::default();

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
                        obj.enabled = Some(try!(MonitoringEnabledDeserializer::deserialize(
                            "Enabled", stack
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

/// Serialize `InstanceMonitoring` contents to a `SignedRequest`.
struct InstanceMonitoringSerializer;
impl InstanceMonitoringSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &InstanceMonitoring) {
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

struct InstanceProtectedDeserializer;
impl InstanceProtectedDeserializer {
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
/// <p>Describes a launch configuration.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct LaunchConfiguration {
    /// <p>[EC2-VPC] Indicates whether to assign a public IP address to each instance.</p>
    pub associate_public_ip_address: Option<bool>,
    /// <p>A block device mapping, which specifies the block devices for the instance.</p>
    pub block_device_mappings: Option<Vec<BlockDeviceMapping>>,
    /// <p>The ID of a ClassicLink-enabled VPC to link your EC2-Classic instances to. This parameter can only be used if you are launching EC2-Classic instances. For more information, see <a href="http://docs.aws.amazon.com/AWSEC2/latest/UserGuide/vpc-classiclink.html">ClassicLink</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
    pub classic_link_vpc_id: Option<String>,
    /// <p>The IDs of one or more security groups for the VPC specified in <code>ClassicLinkVPCId</code>. This parameter is required if you specify a ClassicLink-enabled VPC, and cannot be used otherwise. For more information, see <a href="http://docs.aws.amazon.com/AWSEC2/latest/UserGuide/vpc-classiclink.html">ClassicLink</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
    pub classic_link_vpc_security_groups: Option<Vec<String>>,
    /// <p>The creation date and time for the launch configuration.</p>
    pub created_time: String,
    /// <p>Controls whether the instance is optimized for EBS I/O (<code>true</code>) or not (<code>false</code>).</p>
    pub ebs_optimized: Option<bool>,
    /// <p>The name or Amazon Resource Name (ARN) of the instance profile associated with the IAM role for the instance.</p>
    pub iam_instance_profile: Option<String>,
    /// <p>The ID of the Amazon Machine Image (AMI).</p>
    pub image_id: String,
    /// <p>Controls whether instances in this group are launched with detailed (<code>true</code>) or basic (<code>false</code>) monitoring.</p>
    pub instance_monitoring: Option<InstanceMonitoring>,
    /// <p>The instance type for the instances.</p>
    pub instance_type: String,
    /// <p>The ID of the kernel associated with the AMI.</p>
    pub kernel_id: Option<String>,
    /// <p>The name of the key pair.</p>
    pub key_name: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the launch configuration.</p>
    pub launch_configuration_arn: Option<String>,
    /// <p>The name of the launch configuration.</p>
    pub launch_configuration_name: String,
    /// <p>The tenancy of the instance, either <code>default</code> or <code>dedicated</code>. An instance with <code>dedicated</code> tenancy runs in an isolated, single-tenant hardware and can only be launched into a VPC.</p>
    pub placement_tenancy: Option<String>,
    /// <p>The ID of the RAM disk associated with the AMI.</p>
    pub ramdisk_id: Option<String>,
    /// <p>The security groups to associate with the instances.</p>
    pub security_groups: Option<Vec<String>>,
    /// <p>The price to bid when launching Spot Instances.</p>
    pub spot_price: Option<String>,
    /// <p>The user data available to the instances.</p>
    pub user_data: Option<String>,
}

struct LaunchConfigurationDeserializer;
impl LaunchConfigurationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<LaunchConfiguration, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = LaunchConfiguration::default();

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
                    "AssociatePublicIpAddress" => {
                        obj.associate_public_ip_address =
                            Some(try!(AssociatePublicIpAddressDeserializer::deserialize(
                                "AssociatePublicIpAddress",
                                stack
                            )));
                    }
                    "BlockDeviceMappings" => {
                        obj.block_device_mappings =
                            Some(try!(BlockDeviceMappingsDeserializer::deserialize(
                                "BlockDeviceMappings",
                                stack
                            )));
                    }
                    "ClassicLinkVPCId" => {
                        obj.classic_link_vpc_id = Some(try!(
                            XmlStringMaxLen255Deserializer::deserialize("ClassicLinkVPCId", stack)
                        ));
                    }
                    "ClassicLinkVPCSecurityGroups" => {
                        obj.classic_link_vpc_security_groups =
                            Some(try!(ClassicLinkVPCSecurityGroupsDeserializer::deserialize(
                                "ClassicLinkVPCSecurityGroups",
                                stack
                            )));
                    }
                    "CreatedTime" => {
                        obj.created_time =
                            try!(TimestampTypeDeserializer::deserialize("CreatedTime", stack));
                    }
                    "EbsOptimized" => {
                        obj.ebs_optimized = Some(try!(EbsOptimizedDeserializer::deserialize(
                            "EbsOptimized",
                            stack
                        )));
                    }
                    "IamInstanceProfile" => {
                        obj.iam_instance_profile =
                            Some(try!(XmlStringMaxLen1600Deserializer::deserialize(
                                "IamInstanceProfile",
                                stack
                            )));
                    }
                    "ImageId" => {
                        obj.image_id = try!(XmlStringMaxLen255Deserializer::deserialize(
                            "ImageId", stack
                        ));
                    }
                    "InstanceMonitoring" => {
                        obj.instance_monitoring =
                            Some(try!(InstanceMonitoringDeserializer::deserialize(
                                "InstanceMonitoring",
                                stack
                            )));
                    }
                    "InstanceType" => {
                        obj.instance_type = try!(XmlStringMaxLen255Deserializer::deserialize(
                            "InstanceType",
                            stack
                        ));
                    }
                    "KernelId" => {
                        obj.kernel_id = Some(try!(XmlStringMaxLen255Deserializer::deserialize(
                            "KernelId", stack
                        )));
                    }
                    "KeyName" => {
                        obj.key_name = Some(try!(XmlStringMaxLen255Deserializer::deserialize(
                            "KeyName", stack
                        )));
                    }
                    "LaunchConfigurationARN" => {
                        obj.launch_configuration_arn = Some(try!(
                            ResourceNameDeserializer::deserialize("LaunchConfigurationARN", stack)
                        ));
                    }
                    "LaunchConfigurationName" => {
                        obj.launch_configuration_name =
                            try!(XmlStringMaxLen255Deserializer::deserialize(
                                "LaunchConfigurationName",
                                stack
                            ));
                    }
                    "PlacementTenancy" => {
                        obj.placement_tenancy = Some(try!(
                            XmlStringMaxLen64Deserializer::deserialize("PlacementTenancy", stack)
                        ));
                    }
                    "RamdiskId" => {
                        obj.ramdisk_id = Some(try!(XmlStringMaxLen255Deserializer::deserialize(
                            "RamdiskId",
                            stack
                        )));
                    }
                    "SecurityGroups" => {
                        obj.security_groups = Some(try!(SecurityGroupsDeserializer::deserialize(
                            "SecurityGroups",
                            stack
                        )));
                    }
                    "SpotPrice" => {
                        obj.spot_price =
                            Some(try!(SpotPriceDeserializer::deserialize("SpotPrice", stack)));
                    }
                    "UserData" => {
                        obj.user_data = Some(try!(XmlStringUserDataDeserializer::deserialize(
                            "UserData", stack
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
pub struct LaunchConfigurationNameType {
    /// <p>The name of the launch configuration.</p>
    pub launch_configuration_name: String,
}

/// Serialize `LaunchConfigurationNameType` contents to a `SignedRequest`.
struct LaunchConfigurationNameTypeSerializer;
impl LaunchConfigurationNameTypeSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &LaunchConfigurationNameType) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "LaunchConfigurationName"),
            &obj.launch_configuration_name.replace("+", "%2B"),
        );
    }
}

/// Serialize `LaunchConfigurationNames` contents to a `SignedRequest`.
struct LaunchConfigurationNamesSerializer;
impl LaunchConfigurationNamesSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct LaunchConfigurationNamesType {
    /// <p>The launch configuration names. If you omit this parameter, all launch configurations are described.</p>
    pub launch_configuration_names: Option<Vec<String>>,
    /// <p>The maximum number of items to return with this call. The default value is 50 and the maximum value is 100.</p>
    pub max_records: Option<i64>,
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    pub next_token: Option<String>,
}

/// Serialize `LaunchConfigurationNamesType` contents to a `SignedRequest`.
struct LaunchConfigurationNamesTypeSerializer;
impl LaunchConfigurationNamesTypeSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &LaunchConfigurationNamesType) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.launch_configuration_names {
            LaunchConfigurationNamesSerializer::serialize(
                params,
                &format!("{}{}", prefix, "LaunchConfigurationNames"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.max_records {
            params.put(
                &format!("{}{}", prefix, "MaxRecords"),
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

struct LaunchConfigurationsDeserializer;
impl LaunchConfigurationsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<LaunchConfiguration>, XmlParseError> {
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
                        obj.push(try!(LaunchConfigurationDeserializer::deserialize(
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
#[derive(Default, Debug, Clone, PartialEq)]
pub struct LaunchConfigurationsType {
    /// <p>The launch configurations.</p>
    pub launch_configurations: Vec<LaunchConfiguration>,
    /// <p>The token to use when requesting the next set of items. If there are no additional items to return, the string is empty.</p>
    pub next_token: Option<String>,
}

struct LaunchConfigurationsTypeDeserializer;
impl LaunchConfigurationsTypeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<LaunchConfigurationsType, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = LaunchConfigurationsType::default();

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
                    "LaunchConfigurations" => {
                        obj.launch_configurations =
                            try!(LaunchConfigurationsDeserializer::deserialize(
                                "LaunchConfigurations",
                                stack
                            ));
                    }
                    "NextToken" => {
                        obj.next_token =
                            Some(try!(XmlStringDeserializer::deserialize("NextToken", stack)));
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
struct LaunchTemplateNameDeserializer;
impl LaunchTemplateNameDeserializer {
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
/// <p>Describes a launch template.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct LaunchTemplateSpecification {
    /// <p>The ID of the launch template. You must specify either a template ID or a template name.</p>
    pub launch_template_id: Option<String>,
    /// <p>The name of the launch template. You must specify either a template name or a template ID.</p>
    pub launch_template_name: Option<String>,
    /// <p>The version number, <code>$Latest</code>, or <code>$Default</code>. If the value is <code>$Latest</code>, Auto Scaling selects the latest version of the launch template when launching instances. If the value is <code>$Default</code>, Auto Scaling selects the default version of the launch template when launching instances. The default value is <code>$Default</code>.</p>
    pub version: Option<String>,
}

struct LaunchTemplateSpecificationDeserializer;
impl LaunchTemplateSpecificationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<LaunchTemplateSpecification, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = LaunchTemplateSpecification::default();

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
                    "LaunchTemplateId" => {
                        obj.launch_template_id = Some(try!(
                            XmlStringMaxLen255Deserializer::deserialize("LaunchTemplateId", stack)
                        ));
                    }
                    "LaunchTemplateName" => {
                        obj.launch_template_name =
                            Some(try!(LaunchTemplateNameDeserializer::deserialize(
                                "LaunchTemplateName",
                                stack
                            )));
                    }
                    "Version" => {
                        obj.version = Some(try!(XmlStringMaxLen255Deserializer::deserialize(
                            "Version", stack
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

/// Serialize `LaunchTemplateSpecification` contents to a `SignedRequest`.
struct LaunchTemplateSpecificationSerializer;
impl LaunchTemplateSpecificationSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &LaunchTemplateSpecification) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.launch_template_id {
            params.put(
                &format!("{}{}", prefix, "LaunchTemplateId"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.launch_template_name {
            params.put(
                &format!("{}{}", prefix, "LaunchTemplateName"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.version {
            params.put(
                &format!("{}{}", prefix, "Version"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

struct LifecycleActionResultDeserializer;
impl LifecycleActionResultDeserializer {
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
/// <p>Describes a lifecycle hook, which tells Auto Scaling that you want to perform an action whenever it launches instances or whenever it terminates instances.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/autoscaling/latest/userguide/lifecycle-hooks.html">Auto Scaling Lifecycle Hooks</a> in the <i>Auto Scaling User Guide</i>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct LifecycleHook {
    /// <p>The name of the Auto Scaling group for the lifecycle hook.</p>
    pub auto_scaling_group_name: Option<String>,
    /// <p>Defines the action the Auto Scaling group should take when the lifecycle hook timeout elapses or if an unexpected failure occurs. The valid values are <code>CONTINUE</code> and <code>ABANDON</code>. The default value is <code>CONTINUE</code>.</p>
    pub default_result: Option<String>,
    /// <p>The maximum time, in seconds, that an instance can remain in a <code>Pending:Wait</code> or <code>Terminating:Wait</code> state. The maximum is 172800 seconds (48 hours) or 100 times <code>HeartbeatTimeout</code>, whichever is smaller.</p>
    pub global_timeout: Option<i64>,
    /// <p>The maximum time, in seconds, that can elapse before the lifecycle hook times out. If the lifecycle hook times out, Auto Scaling performs the default action. You can prevent the lifecycle hook from timing out by calling <a>RecordLifecycleActionHeartbeat</a>.</p>
    pub heartbeat_timeout: Option<i64>,
    /// <p>The name of the lifecycle hook.</p>
    pub lifecycle_hook_name: Option<String>,
    /// <p>The state of the EC2 instance to which you want to attach the lifecycle hook. For a list of lifecycle hook types, see <a>DescribeLifecycleHookTypes</a>.</p>
    pub lifecycle_transition: Option<String>,
    /// <p>Additional information that you want to include any time Auto Scaling sends a message to the notification target.</p>
    pub notification_metadata: Option<String>,
    /// <p>The ARN of the target that Auto Scaling sends notifications to when an instance is in the transition state for the lifecycle hook. The notification target can be either an SQS queue or an SNS topic.</p>
    pub notification_target_arn: Option<String>,
    /// <p>The ARN of the IAM role that allows the Auto Scaling group to publish to the specified notification target.</p>
    pub role_arn: Option<String>,
}

struct LifecycleHookDeserializer;
impl LifecycleHookDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<LifecycleHook, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = LifecycleHook::default();

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
                    "AutoScalingGroupName" => {
                        obj.auto_scaling_group_name = Some(try!(
                            ResourceNameDeserializer::deserialize("AutoScalingGroupName", stack)
                        ));
                    }
                    "DefaultResult" => {
                        obj.default_result = Some(try!(
                            LifecycleActionResultDeserializer::deserialize("DefaultResult", stack)
                        ));
                    }
                    "GlobalTimeout" => {
                        obj.global_timeout = Some(try!(GlobalTimeoutDeserializer::deserialize(
                            "GlobalTimeout",
                            stack
                        )));
                    }
                    "HeartbeatTimeout" => {
                        obj.heartbeat_timeout = Some(try!(
                            HeartbeatTimeoutDeserializer::deserialize("HeartbeatTimeout", stack)
                        ));
                    }
                    "LifecycleHookName" => {
                        obj.lifecycle_hook_name =
                            Some(try!(AsciiStringMaxLen255Deserializer::deserialize(
                                "LifecycleHookName",
                                stack
                            )));
                    }
                    "LifecycleTransition" => {
                        obj.lifecycle_transition =
                            Some(try!(LifecycleTransitionDeserializer::deserialize(
                                "LifecycleTransition",
                                stack
                            )));
                    }
                    "NotificationMetadata" => {
                        obj.notification_metadata =
                            Some(try!(XmlStringMaxLen1023Deserializer::deserialize(
                                "NotificationMetadata",
                                stack
                            )));
                    }
                    "NotificationTargetARN" => {
                        obj.notification_target_arn = Some(try!(
                            ResourceNameDeserializer::deserialize("NotificationTargetARN", stack)
                        ));
                    }
                    "RoleARN" => {
                        obj.role_arn = Some(try!(ResourceNameDeserializer::deserialize(
                            "RoleARN", stack
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

/// Serialize `LifecycleHookNames` contents to a `SignedRequest`.
struct LifecycleHookNamesSerializer;
impl LifecycleHookNamesSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

/// <p>Describes a lifecycle hook, which tells Auto Scaling that you want to perform an action whenever it launches instances or whenever it terminates instances.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/autoscaling/latest/userguide/lifecycle-hooks.html">Auto Scaling Lifecycle Hooks</a> in the <i>Auto Scaling User Guide</i>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct LifecycleHookSpecification {
    /// <p>Defines the action the Auto Scaling group should take when the lifecycle hook timeout elapses or if an unexpected failure occurs. The valid values are <code>CONTINUE</code> and <code>ABANDON</code>.</p>
    pub default_result: Option<String>,
    /// <p>The maximum time, in seconds, that can elapse before the lifecycle hook times out. If the lifecycle hook times out, Auto Scaling performs the default action. You can prevent the lifecycle hook from timing out by calling <a>RecordLifecycleActionHeartbeat</a>.</p>
    pub heartbeat_timeout: Option<i64>,
    /// <p>The name of the lifecycle hook.</p>
    pub lifecycle_hook_name: String,
    /// <p>The state of the EC2 instance to which you want to attach the lifecycle hook. For a list of lifecycle hook types, see <a>DescribeLifecycleHookTypes</a>.</p>
    pub lifecycle_transition: String,
    /// <p>Additional information that you want to include any time Auto Scaling sends a message to the notification target.</p>
    pub notification_metadata: Option<String>,
    /// <p>The ARN of the target that Auto Scaling sends notifications to when an instance is in the transition state for the lifecycle hook. The notification target can be either an SQS queue or an SNS topic.</p>
    pub notification_target_arn: Option<String>,
    /// <p>The ARN of the IAM role that allows the Auto Scaling group to publish to the specified notification target.</p>
    pub role_arn: Option<String>,
}

/// Serialize `LifecycleHookSpecification` contents to a `SignedRequest`.
struct LifecycleHookSpecificationSerializer;
impl LifecycleHookSpecificationSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &LifecycleHookSpecification) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.default_result {
            params.put(
                &format!("{}{}", prefix, "DefaultResult"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.heartbeat_timeout {
            params.put(
                &format!("{}{}", prefix, "HeartbeatTimeout"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        params.put(
            &format!("{}{}", prefix, "LifecycleHookName"),
            &obj.lifecycle_hook_name.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "LifecycleTransition"),
            &obj.lifecycle_transition.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.notification_metadata {
            params.put(
                &format!("{}{}", prefix, "NotificationMetadata"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.notification_target_arn {
            params.put(
                &format!("{}{}", prefix, "NotificationTargetARN"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.role_arn {
            params.put(
                &format!("{}{}", prefix, "RoleARN"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

/// Serialize `LifecycleHookSpecifications` contents to a `SignedRequest`.
struct LifecycleHookSpecificationsSerializer;
impl LifecycleHookSpecificationsSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<LifecycleHookSpecification>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            LifecycleHookSpecificationSerializer::serialize(params, &key, obj);
        }
    }
}

struct LifecycleHooksDeserializer;
impl LifecycleHooksDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<LifecycleHook>, XmlParseError> {
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
                        obj.push(try!(LifecycleHookDeserializer::deserialize(
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
struct LifecycleStateDeserializer;
impl LifecycleStateDeserializer {
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
struct LifecycleTransitionDeserializer;
impl LifecycleTransitionDeserializer {
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
struct LoadBalancerNamesDeserializer;
impl LoadBalancerNamesDeserializer {
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
                        obj.push(try!(XmlStringMaxLen255Deserializer::deserialize(
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

/// <p>Describes the state of a Classic Load Balancer.</p> <p>If you specify a load balancer when creating the Auto Scaling group, the state of the load balancer is <code>InService</code>.</p> <p>If you attach a load balancer to an existing Auto Scaling group, the initial state is <code>Adding</code>. The state transitions to <code>Added</code> after all instances in the group are registered with the load balancer. If ELB health checks are enabled for the load balancer, the state transitions to <code>InService</code> after at least one instance in the group passes the health check. If EC2 health checks are enabled instead, the load balancer remains in the <code>Added</code> state.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct LoadBalancerState {
    /// <p>The name of the load balancer.</p>
    pub load_balancer_name: Option<String>,
    /// <p><p>One of the following load balancer states:</p> <ul> <li> <p> <code>Adding</code> - The instances in the group are being registered with the load balancer.</p> </li> <li> <p> <code>Added</code> - All instances in the group are registered with the load balancer.</p> </li> <li> <p> <code>InService</code> - At least one instance in the group passed an ELB health check.</p> </li> <li> <p> <code>Removing</code> - The instances in the group are being deregistered from the load balancer. If connection draining is enabled, Elastic Load Balancing waits for in-flight requests to complete before deregistering the instances.</p> </li> <li> <p> <code>Removed</code> - All instances in the group are deregistered from the load balancer.</p> </li> </ul></p>
    pub state: Option<String>,
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
                    "LoadBalancerName" => {
                        obj.load_balancer_name = Some(try!(
                            XmlStringMaxLen255Deserializer::deserialize("LoadBalancerName", stack)
                        ));
                    }
                    "State" => {
                        obj.state = Some(try!(XmlStringMaxLen255Deserializer::deserialize(
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
struct LoadBalancerStatesDeserializer;
impl LoadBalancerStatesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<LoadBalancerState>, XmlParseError> {
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
                        obj.push(try!(LoadBalancerStateDeserializer::deserialize(
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
/// <p>Describes the state of a target group.</p> <p>If you attach a target group to an existing Auto Scaling group, the initial state is <code>Adding</code>. The state transitions to <code>Added</code> after all Auto Scaling instances are registered with the target group. If ELB health checks are enabled, the state transitions to <code>InService</code> after at least one Auto Scaling instance passes the health check. If EC2 health checks are enabled instead, the target group remains in the <code>Added</code> state.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct LoadBalancerTargetGroupState {
    /// <p>The Amazon Resource Name (ARN) of the target group.</p>
    pub load_balancer_target_group_arn: Option<String>,
    /// <p><p>The state of the target group.</p> <ul> <li> <p> <code>Adding</code> - The Auto Scaling instances are being registered with the target group.</p> </li> <li> <p> <code>Added</code> - All Auto Scaling instances are registered with the target group.</p> </li> <li> <p> <code>InService</code> - At least one Auto Scaling instance passed an ELB health check.</p> </li> <li> <p> <code>Removing</code> - The Auto Scaling instances are being deregistered from the target group. If connection draining is enabled, Elastic Load Balancing waits for in-flight requests to complete before deregistering the instances.</p> </li> <li> <p> <code>Removed</code> - All Auto Scaling instances are deregistered from the target group.</p> </li> </ul></p>
    pub state: Option<String>,
}

struct LoadBalancerTargetGroupStateDeserializer;
impl LoadBalancerTargetGroupStateDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<LoadBalancerTargetGroupState, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = LoadBalancerTargetGroupState::default();

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
                    "LoadBalancerTargetGroupARN" => {
                        obj.load_balancer_target_group_arn =
                            Some(try!(XmlStringMaxLen511Deserializer::deserialize(
                                "LoadBalancerTargetGroupARN",
                                stack
                            )));
                    }
                    "State" => {
                        obj.state = Some(try!(XmlStringMaxLen255Deserializer::deserialize(
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
struct LoadBalancerTargetGroupStatesDeserializer;
impl LoadBalancerTargetGroupStatesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<LoadBalancerTargetGroupState>, XmlParseError> {
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
                        obj.push(try!(LoadBalancerTargetGroupStateDeserializer::deserialize(
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
struct MaxNumberOfAutoScalingGroupsDeserializer;
impl MaxNumberOfAutoScalingGroupsDeserializer {
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
struct MaxNumberOfLaunchConfigurationsDeserializer;
impl MaxNumberOfLaunchConfigurationsDeserializer {
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
/// <p>Describes a metric.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct MetricCollectionType {
    /// <p><p>One of the following metrics:</p> <ul> <li> <p> <code>GroupMinSize</code> </p> </li> <li> <p> <code>GroupMaxSize</code> </p> </li> <li> <p> <code>GroupDesiredCapacity</code> </p> </li> <li> <p> <code>GroupInServiceInstances</code> </p> </li> <li> <p> <code>GroupPendingInstances</code> </p> </li> <li> <p> <code>GroupStandbyInstances</code> </p> </li> <li> <p> <code>GroupTerminatingInstances</code> </p> </li> <li> <p> <code>GroupTotalInstances</code> </p> </li> </ul></p>
    pub metric: Option<String>,
}

struct MetricCollectionTypeDeserializer;
impl MetricCollectionTypeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<MetricCollectionType, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = MetricCollectionType::default();

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
                    "Metric" => {
                        obj.metric = Some(try!(XmlStringMaxLen255Deserializer::deserialize(
                            "Metric", stack
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
struct MetricCollectionTypesDeserializer;
impl MetricCollectionTypesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<MetricCollectionType>, XmlParseError> {
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
                        obj.push(try!(MetricCollectionTypeDeserializer::deserialize(
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
/// <p>Describes the dimension of a metric.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct MetricDimension {
    /// <p>The name of the dimension.</p>
    pub name: String,
    /// <p>The value of the dimension.</p>
    pub value: String,
}

struct MetricDimensionDeserializer;
impl MetricDimensionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<MetricDimension, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = MetricDimension::default();

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
                            try!(MetricDimensionNameDeserializer::deserialize("Name", stack));
                    }
                    "Value" => {
                        obj.value = try!(MetricDimensionValueDeserializer::deserialize(
                            "Value", stack
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

/// Serialize `MetricDimension` contents to a `SignedRequest`.
struct MetricDimensionSerializer;
impl MetricDimensionSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &MetricDimension) {
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

struct MetricDimensionNameDeserializer;
impl MetricDimensionNameDeserializer {
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
struct MetricDimensionValueDeserializer;
impl MetricDimensionValueDeserializer {
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
struct MetricDimensionsDeserializer;
impl MetricDimensionsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<MetricDimension>, XmlParseError> {
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
                        obj.push(try!(MetricDimensionDeserializer::deserialize(
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

/// Serialize `MetricDimensions` contents to a `SignedRequest`.
struct MetricDimensionsSerializer;
impl MetricDimensionsSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<MetricDimension>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            MetricDimensionSerializer::serialize(params, &key, obj);
        }
    }
}

/// <p>Describes a granularity of a metric.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct MetricGranularityType {
    /// <p>The granularity. The only valid value is <code>1Minute</code>.</p>
    pub granularity: Option<String>,
}

struct MetricGranularityTypeDeserializer;
impl MetricGranularityTypeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<MetricGranularityType, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = MetricGranularityType::default();

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
                    "Granularity" => {
                        obj.granularity = Some(try!(XmlStringMaxLen255Deserializer::deserialize(
                            "Granularity",
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
struct MetricGranularityTypesDeserializer;
impl MetricGranularityTypesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<MetricGranularityType>, XmlParseError> {
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
                        obj.push(try!(MetricGranularityTypeDeserializer::deserialize(
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
struct MetricNameDeserializer;
impl MetricNameDeserializer {
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
struct MetricNamespaceDeserializer;
impl MetricNamespaceDeserializer {
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
struct MetricScaleDeserializer;
impl MetricScaleDeserializer {
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
struct MetricStatisticDeserializer;
impl MetricStatisticDeserializer {
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
struct MetricTypeDeserializer;
impl MetricTypeDeserializer {
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
struct MetricUnitDeserializer;
impl MetricUnitDeserializer {
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

/// Serialize `Metrics` contents to a `SignedRequest`.
struct MetricsSerializer;
impl MetricsSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

struct MinAdjustmentMagnitudeDeserializer;
impl MinAdjustmentMagnitudeDeserializer {
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
struct MinAdjustmentStepDeserializer;
impl MinAdjustmentStepDeserializer {
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
struct MonitoringEnabledDeserializer;
impl MonitoringEnabledDeserializer {
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
struct NoDeviceDeserializer;
impl NoDeviceDeserializer {
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
/// <p>Describes a notification.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct NotificationConfiguration {
    /// <p>The name of the Auto Scaling group.</p>
    pub auto_scaling_group_name: Option<String>,
    /// <p><p>One of the following event notification types:</p> <ul> <li> <p> <code>autoscaling:EC2<em>INSTANCE</em>LAUNCH</code> </p> </li> <li> <p> <code>autoscaling:EC2<em>INSTANCE</em>LAUNCH<em>ERROR</code> </p> </li> <li> <p> <code>autoscaling:EC2</em>INSTANCE<em>TERMINATE</code> </p> </li> <li> <p> <code>autoscaling:EC2</em>INSTANCE<em>TERMINATE</em>ERROR</code> </p> </li> <li> <p> <code>autoscaling:TEST_NOTIFICATION</code> </p> </li> </ul></p>
    pub notification_type: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the Amazon Simple Notification Service (SNS) topic.</p>
    pub topic_arn: Option<String>,
}

struct NotificationConfigurationDeserializer;
impl NotificationConfigurationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<NotificationConfiguration, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = NotificationConfiguration::default();

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
                    "AutoScalingGroupName" => {
                        obj.auto_scaling_group_name = Some(try!(
                            ResourceNameDeserializer::deserialize("AutoScalingGroupName", stack)
                        ));
                    }
                    "NotificationType" => {
                        obj.notification_type = Some(try!(
                            XmlStringMaxLen255Deserializer::deserialize("NotificationType", stack)
                        ));
                    }
                    "TopicARN" => {
                        obj.topic_arn = Some(try!(ResourceNameDeserializer::deserialize(
                            "TopicARN", stack
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
struct NotificationConfigurationsDeserializer;
impl NotificationConfigurationsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<NotificationConfiguration>, XmlParseError> {
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
                        obj.push(try!(NotificationConfigurationDeserializer::deserialize(
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
struct NumberOfAutoScalingGroupsDeserializer;
impl NumberOfAutoScalingGroupsDeserializer {
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
struct NumberOfLaunchConfigurationsDeserializer;
impl NumberOfLaunchConfigurationsDeserializer {
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
#[derive(Default, Debug, Clone, PartialEq)]
pub struct PoliciesType {
    /// <p>The token to use when requesting the next set of items. If there are no additional items to return, the string is empty.</p>
    pub next_token: Option<String>,
    /// <p>The scaling policies.</p>
    pub scaling_policies: Option<Vec<ScalingPolicy>>,
}

struct PoliciesTypeDeserializer;
impl PoliciesTypeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PoliciesType, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = PoliciesType::default();

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
                            Some(try!(XmlStringDeserializer::deserialize("NextToken", stack)));
                    }
                    "ScalingPolicies" => {
                        obj.scaling_policies = Some(try!(
                            ScalingPoliciesDeserializer::deserialize("ScalingPolicies", stack)
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
/// <p>Contains the output of PutScalingPolicy.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct PolicyARNType {
    /// <p>The CloudWatch alarms created for the target tracking policy.</p>
    pub alarms: Option<Vec<Alarm>>,
    /// <p>The Amazon Resource Name (ARN) of the policy.</p>
    pub policy_arn: Option<String>,
}

struct PolicyARNTypeDeserializer;
impl PolicyARNTypeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PolicyARNType, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = PolicyARNType::default();

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
                    "Alarms" => {
                        obj.alarms = Some(try!(AlarmsDeserializer::deserialize("Alarms", stack)));
                    }
                    "PolicyARN" => {
                        obj.policy_arn = Some(try!(ResourceNameDeserializer::deserialize(
                            "PolicyARN",
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
struct PolicyIncrementDeserializer;
impl PolicyIncrementDeserializer {
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

/// Serialize `PolicyTypes` contents to a `SignedRequest`.
struct PolicyTypesSerializer;
impl PolicyTypesSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

/// <p>Configures a predefined metric for a target tracking policy.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct PredefinedMetricSpecification {
    /// <p>The metric type.</p>
    pub predefined_metric_type: String,
    /// <p>Identifies the resource associated with the metric type. The following predefined metrics are available:</p> <ul> <li> <p> <code>ASGAverageCPUUtilization</code> - average CPU utilization of the Auto Scaling group</p> </li> <li> <p> <code>ASGAverageNetworkIn</code> - average number of bytes received on all network interfaces by the Auto Scaling group</p> </li> <li> <p> <code>ASGAverageNetworkOut</code> - average number of bytes sent out on all network interfaces by the Auto Scaling group</p> </li> <li> <p> <code>ALBRequestCountPerTarget</code> - number of requests completed per target in an Application Load Balancer target group</p> </li> </ul> <p>For predefined metric types <code>ASGAverageCPUUtilization</code>, <code>ASGAverageNetworkIn</code>, and <code>ASGAverageNetworkOut</code>, the parameter must not be specified as the resource associated with the metric type is the Auto Scaling group. For predefined metric type <code>ALBRequestCountPerTarget</code>, the parameter must be specified in the format: <code>app/<i>load-balancer-name</i>/<i>load-balancer-id</i>/targetgroup/<i>target-group-name</i>/<i>target-group-id</i> </code>, where <code>app/<i>load-balancer-name</i>/<i>load-balancer-id</i> </code> is the final portion of the load balancer ARN, and <code>targetgroup/<i>target-group-name</i>/<i>target-group-id</i> </code> is the final portion of the target group ARN. The target group must be attached to the Auto Scaling group.</p>
    pub resource_label: Option<String>,
}

struct PredefinedMetricSpecificationDeserializer;
impl PredefinedMetricSpecificationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PredefinedMetricSpecification, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = PredefinedMetricSpecification::default();

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
                    "PredefinedMetricType" => {
                        obj.predefined_metric_type = try!(MetricTypeDeserializer::deserialize(
                            "PredefinedMetricType",
                            stack
                        ));
                    }
                    "ResourceLabel" => {
                        obj.resource_label = Some(try!(
                            XmlStringMaxLen1023Deserializer::deserialize("ResourceLabel", stack)
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

/// Serialize `PredefinedMetricSpecification` contents to a `SignedRequest`.
struct PredefinedMetricSpecificationSerializer;
impl PredefinedMetricSpecificationSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &PredefinedMetricSpecification) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "PredefinedMetricType"),
            &obj.predefined_metric_type.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.resource_label {
            params.put(
                &format!("{}{}", prefix, "ResourceLabel"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

/// Serialize `ProcessNames` contents to a `SignedRequest`.
struct ProcessNamesSerializer;
impl ProcessNamesSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

/// <p>Describes a process type.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/autoscaling/latest/userguide/as-suspend-resume-processes.html#process-types">Auto Scaling Processes</a> in the <i>Auto Scaling User Guide</i>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ProcessType {
    /// <p><p>One of the following processes:</p> <ul> <li> <p> <code>Launch</code> </p> </li> <li> <p> <code>Terminate</code> </p> </li> <li> <p> <code>AddToLoadBalancer</code> </p> </li> <li> <p> <code>AlarmNotification</code> </p> </li> <li> <p> <code>AZRebalance</code> </p> </li> <li> <p> <code>HealthCheck</code> </p> </li> <li> <p> <code>ReplaceUnhealthy</code> </p> </li> <li> <p> <code>ScheduledActions</code> </p> </li> </ul></p>
    pub process_name: String,
}

struct ProcessTypeDeserializer;
impl ProcessTypeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ProcessType, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ProcessType::default();

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
                    "ProcessName" => {
                        obj.process_name = try!(XmlStringMaxLen255Deserializer::deserialize(
                            "ProcessName",
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
struct ProcessesDeserializer;
impl ProcessesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<ProcessType>, XmlParseError> {
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
                        obj.push(try!(ProcessTypeDeserializer::deserialize("member", stack)));
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
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ProcessesType {
    /// <p>The names of the process types.</p>
    pub processes: Option<Vec<ProcessType>>,
}

struct ProcessesTypeDeserializer;
impl ProcessesTypeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ProcessesType, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ProcessesType::default();

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
                    "Processes" => {
                        obj.processes =
                            Some(try!(ProcessesDeserializer::deserialize("Processes", stack)));
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
struct ProgressDeserializer;
impl ProgressDeserializer {
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
struct PropagateAtLaunchDeserializer;
impl PropagateAtLaunchDeserializer {
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
pub struct PutLifecycleHookAnswer {}

struct PutLifecycleHookAnswerDeserializer;
impl PutLifecycleHookAnswerDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PutLifecycleHookAnswer, XmlParseError> {
        try!(start_element(tag_name, stack));

        let obj = PutLifecycleHookAnswer::default();

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct PutLifecycleHookType {
    /// <p>The name of the Auto Scaling group.</p>
    pub auto_scaling_group_name: String,
    /// <p>Defines the action the Auto Scaling group should take when the lifecycle hook timeout elapses or if an unexpected failure occurs. This parameter can be either <code>CONTINUE</code> or <code>ABANDON</code>. The default value is <code>ABANDON</code>.</p>
    pub default_result: Option<String>,
    /// <p>The maximum time, in seconds, that can elapse before the lifecycle hook times out. The range is from 30 to 7200 seconds. The default is 3600 seconds (1 hour).</p> <p>If the lifecycle hook times out, Auto Scaling performs the default action. You can prevent the lifecycle hook from timing out by calling <a>RecordLifecycleActionHeartbeat</a>.</p>
    pub heartbeat_timeout: Option<i64>,
    /// <p>The name of the lifecycle hook.</p>
    pub lifecycle_hook_name: String,
    /// <p>The instance state to which you want to attach the lifecycle hook. For a list of lifecycle hook types, see <a>DescribeLifecycleHookTypes</a>.</p> <p>This parameter is required for new lifecycle hooks, but optional when updating existing hooks.</p>
    pub lifecycle_transition: Option<String>,
    /// <p>Contains additional information that you want to include any time Auto Scaling sends a message to the notification target.</p>
    pub notification_metadata: Option<String>,
    /// <p>The ARN of the notification target that Auto Scaling will use to notify you when an instance is in the transition state for the lifecycle hook. This target can be either an SQS queue or an SNS topic. If you specify an empty string, this overrides the current ARN.</p> <p>This operation uses the JSON format when sending notifications to an Amazon SQS queue, and an email key/value pair format when sending notifications to an Amazon SNS topic.</p> <p>When you specify a notification target, Auto Scaling sends it a test message. Test messages contains the following additional key/value pair: <code>"Event": "autoscaling:TEST_NOTIFICATION"</code>.</p>
    pub notification_target_arn: Option<String>,
    /// <p>The ARN of the IAM role that allows the Auto Scaling group to publish to the specified notification target.</p> <p>This parameter is required for new lifecycle hooks, but optional when updating existing hooks.</p>
    pub role_arn: Option<String>,
}

/// Serialize `PutLifecycleHookType` contents to a `SignedRequest`.
struct PutLifecycleHookTypeSerializer;
impl PutLifecycleHookTypeSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &PutLifecycleHookType) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "AutoScalingGroupName"),
            &obj.auto_scaling_group_name.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.default_result {
            params.put(
                &format!("{}{}", prefix, "DefaultResult"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.heartbeat_timeout {
            params.put(
                &format!("{}{}", prefix, "HeartbeatTimeout"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        params.put(
            &format!("{}{}", prefix, "LifecycleHookName"),
            &obj.lifecycle_hook_name.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.lifecycle_transition {
            params.put(
                &format!("{}{}", prefix, "LifecycleTransition"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.notification_metadata {
            params.put(
                &format!("{}{}", prefix, "NotificationMetadata"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.notification_target_arn {
            params.put(
                &format!("{}{}", prefix, "NotificationTargetARN"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.role_arn {
            params.put(
                &format!("{}{}", prefix, "RoleARN"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct PutNotificationConfigurationType {
    /// <p>The name of the Auto Scaling group.</p>
    pub auto_scaling_group_name: String,
    /// <p>The type of event that will cause the notification to be sent. For details about notification types supported by Auto Scaling, see <a>DescribeAutoScalingNotificationTypes</a>.</p>
    pub notification_types: Vec<String>,
    /// <p>The Amazon Resource Name (ARN) of the Amazon Simple Notification Service (SNS) topic.</p>
    pub topic_arn: String,
}

/// Serialize `PutNotificationConfigurationType` contents to a `SignedRequest`.
struct PutNotificationConfigurationTypeSerializer;
impl PutNotificationConfigurationTypeSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &PutNotificationConfigurationType) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "AutoScalingGroupName"),
            &obj.auto_scaling_group_name.replace("+", "%2B"),
        );
        AutoScalingNotificationTypesSerializer::serialize(
            params,
            &format!("{}{}", prefix, "NotificationTypes"),
            &obj.notification_types,
        );
        params.put(
            &format!("{}{}", prefix, "TopicARN"),
            &obj.topic_arn.replace("+", "%2B"),
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct PutScalingPolicyType {
    /// <p>The adjustment type. The valid values are <code>ChangeInCapacity</code>, <code>ExactCapacity</code>, and <code>PercentChangeInCapacity</code>.</p> <p>This parameter is supported if the policy type is <code>SimpleScaling</code> or <code>StepScaling</code>.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/autoscaling/latest/userguide/as-scale-based-on-demand.html">Dynamic Scaling</a> in the <i>Auto Scaling User Guide</i>.</p>
    pub adjustment_type: Option<String>,
    /// <p>The name of the Auto Scaling group.</p>
    pub auto_scaling_group_name: String,
    /// <p>The amount of time, in seconds, after a scaling activity completes and before the next scaling activity can start. If this parameter is not specified, the default cooldown period for the group applies.</p> <p>This parameter is supported if the policy type is <code>SimpleScaling</code>.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/autoscaling/latest/userguide/Cooldown.html">Auto Scaling Cooldowns</a> in the <i>Auto Scaling User Guide</i>.</p>
    pub cooldown: Option<i64>,
    /// <p>The estimated time, in seconds, until a newly launched instance can contribute to the CloudWatch metrics. The default is to use the value specified for the default cooldown period for the group.</p> <p>This parameter is supported if the policy type is <code>StepScaling</code> or <code>TargetTrackingScaling</code>.</p>
    pub estimated_instance_warmup: Option<i64>,
    /// <p>The aggregation type for the CloudWatch metrics. The valid values are <code>Minimum</code>, <code>Maximum</code>, and <code>Average</code>. If the aggregation type is null, the value is treated as <code>Average</code>.</p> <p>This parameter is supported if the policy type is <code>StepScaling</code>.</p>
    pub metric_aggregation_type: Option<String>,
    /// <p>The minimum number of instances to scale. If the value of <code>AdjustmentType</code> is <code>PercentChangeInCapacity</code>, the scaling policy changes the <code>DesiredCapacity</code> of the Auto Scaling group by at least this many instances. Otherwise, the error is <code>ValidationError</code>.</p> <p>This parameter is supported if the policy type is <code>SimpleScaling</code> or <code>StepScaling</code>.</p>
    pub min_adjustment_magnitude: Option<i64>,
    /// <p>Available for backward compatibility. Use <code>MinAdjustmentMagnitude</code> instead.</p>
    pub min_adjustment_step: Option<i64>,
    /// <p>The name of the policy.</p>
    pub policy_name: String,
    /// <p>The policy type. The valid values are <code>SimpleScaling</code>, <code>StepScaling</code>, and <code>TargetTrackingScaling</code>. If the policy type is null, the value is treated as <code>SimpleScaling</code>.</p>
    pub policy_type: Option<String>,
    /// <p>The amount by which to scale, based on the specified adjustment type. A positive value adds to the current capacity while a negative number removes from the current capacity.</p> <p>This parameter is required if the policy type is <code>SimpleScaling</code> and not supported otherwise.</p>
    pub scaling_adjustment: Option<i64>,
    /// <p>A set of adjustments that enable you to scale based on the size of the alarm breach.</p> <p>This parameter is required if the policy type is <code>StepScaling</code> and not supported otherwise.</p>
    pub step_adjustments: Option<Vec<StepAdjustment>>,
    /// <p>A target tracking policy.</p> <p>This parameter is required if the policy type is <code>TargetTrackingScaling</code> and not supported otherwise.</p>
    pub target_tracking_configuration: Option<TargetTrackingConfiguration>,
}

/// Serialize `PutScalingPolicyType` contents to a `SignedRequest`.
struct PutScalingPolicyTypeSerializer;
impl PutScalingPolicyTypeSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &PutScalingPolicyType) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.adjustment_type {
            params.put(
                &format!("{}{}", prefix, "AdjustmentType"),
                &field_value.replace("+", "%2B"),
            );
        }
        params.put(
            &format!("{}{}", prefix, "AutoScalingGroupName"),
            &obj.auto_scaling_group_name.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.cooldown {
            params.put(
                &format!("{}{}", prefix, "Cooldown"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.estimated_instance_warmup {
            params.put(
                &format!("{}{}", prefix, "EstimatedInstanceWarmup"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.metric_aggregation_type {
            params.put(
                &format!("{}{}", prefix, "MetricAggregationType"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.min_adjustment_magnitude {
            params.put(
                &format!("{}{}", prefix, "MinAdjustmentMagnitude"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.min_adjustment_step {
            params.put(
                &format!("{}{}", prefix, "MinAdjustmentStep"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        params.put(
            &format!("{}{}", prefix, "PolicyName"),
            &obj.policy_name.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.policy_type {
            params.put(
                &format!("{}{}", prefix, "PolicyType"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.scaling_adjustment {
            params.put(
                &format!("{}{}", prefix, "ScalingAdjustment"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.step_adjustments {
            StepAdjustmentsSerializer::serialize(
                params,
                &format!("{}{}", prefix, "StepAdjustments"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.target_tracking_configuration {
            TargetTrackingConfigurationSerializer::serialize(
                params,
                &format!("{}{}", prefix, "TargetTrackingConfiguration"),
                field_value,
            );
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct PutScheduledUpdateGroupActionType {
    /// <p>The name of the Auto Scaling group.</p>
    pub auto_scaling_group_name: String,
    /// <p>The number of EC2 instances that should be running in the group.</p>
    pub desired_capacity: Option<i64>,
    /// <p>The time for the recurring schedule to end. Auto Scaling does not perform the action after this time.</p>
    pub end_time: Option<String>,
    /// <p>The maximum size for the Auto Scaling group.</p>
    pub max_size: Option<i64>,
    /// <p>The minimum size for the Auto Scaling group.</p>
    pub min_size: Option<i64>,
    /// <p>The recurring schedule for this action, in Unix cron syntax format. For more information, see <a href="http://en.wikipedia.org/wiki/Cron">Cron</a> in Wikipedia.</p>
    pub recurrence: Option<String>,
    /// <p>The name of this scaling action.</p>
    pub scheduled_action_name: String,
    /// <p>The time for this action to start, in "YYYY-MM-DDThh:mm:ssZ" format in UTC/GMT only (for example, <code>2014-06-01T00:00:00Z</code>).</p> <p>If you specify <code>Recurrence</code> and <code>StartTime</code>, Auto Scaling performs the action at this time, and then performs the action based on the specified recurrence.</p> <p>If you try to schedule your action in the past, Auto Scaling returns an error message.</p>
    pub start_time: Option<String>,
    /// <p>This parameter is deprecated.</p>
    pub time: Option<String>,
}

/// Serialize `PutScheduledUpdateGroupActionType` contents to a `SignedRequest`.
struct PutScheduledUpdateGroupActionTypeSerializer;
impl PutScheduledUpdateGroupActionTypeSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &PutScheduledUpdateGroupActionType) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "AutoScalingGroupName"),
            &obj.auto_scaling_group_name.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.desired_capacity {
            params.put(
                &format!("{}{}", prefix, "DesiredCapacity"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.end_time {
            params.put(
                &format!("{}{}", prefix, "EndTime"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.max_size {
            params.put(
                &format!("{}{}", prefix, "MaxSize"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.min_size {
            params.put(
                &format!("{}{}", prefix, "MinSize"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.recurrence {
            params.put(
                &format!("{}{}", prefix, "Recurrence"),
                &field_value.replace("+", "%2B"),
            );
        }
        params.put(
            &format!("{}{}", prefix, "ScheduledActionName"),
            &obj.scheduled_action_name.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.start_time {
            params.put(
                &format!("{}{}", prefix, "StartTime"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.time {
            params.put(
                &format!("{}{}", prefix, "Time"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct RecordLifecycleActionHeartbeatAnswer {}

struct RecordLifecycleActionHeartbeatAnswerDeserializer;
impl RecordLifecycleActionHeartbeatAnswerDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<RecordLifecycleActionHeartbeatAnswer, XmlParseError> {
        try!(start_element(tag_name, stack));

        let obj = RecordLifecycleActionHeartbeatAnswer::default();

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct RecordLifecycleActionHeartbeatType {
    /// <p>The name of the Auto Scaling group.</p>
    pub auto_scaling_group_name: String,
    /// <p>The ID of the instance.</p>
    pub instance_id: Option<String>,
    /// <p>A token that uniquely identifies a specific lifecycle action associated with an instance. Auto Scaling sends this token to the notification target you specified when you created the lifecycle hook.</p>
    pub lifecycle_action_token: Option<String>,
    /// <p>The name of the lifecycle hook.</p>
    pub lifecycle_hook_name: String,
}

/// Serialize `RecordLifecycleActionHeartbeatType` contents to a `SignedRequest`.
struct RecordLifecycleActionHeartbeatTypeSerializer;
impl RecordLifecycleActionHeartbeatTypeSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &RecordLifecycleActionHeartbeatType) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "AutoScalingGroupName"),
            &obj.auto_scaling_group_name.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.instance_id {
            params.put(
                &format!("{}{}", prefix, "InstanceId"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.lifecycle_action_token {
            params.put(
                &format!("{}{}", prefix, "LifecycleActionToken"),
                &field_value.replace("+", "%2B"),
            );
        }
        params.put(
            &format!("{}{}", prefix, "LifecycleHookName"),
            &obj.lifecycle_hook_name.replace("+", "%2B"),
        );
    }
}

struct ResourceNameDeserializer;
impl ResourceNameDeserializer {
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
struct ScalingActivityStatusCodeDeserializer;
impl ScalingActivityStatusCodeDeserializer {
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
struct ScalingPoliciesDeserializer;
impl ScalingPoliciesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<ScalingPolicy>, XmlParseError> {
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
                        obj.push(try!(ScalingPolicyDeserializer::deserialize(
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
/// <p>Describes a scaling policy.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ScalingPolicy {
    /// <p>The adjustment type, which specifies how <code>ScalingAdjustment</code> is interpreted. Valid values are <code>ChangeInCapacity</code>, <code>ExactCapacity</code>, and <code>PercentChangeInCapacity</code>.</p>
    pub adjustment_type: Option<String>,
    /// <p>The CloudWatch alarms related to the policy.</p>
    pub alarms: Option<Vec<Alarm>>,
    /// <p>The name of the Auto Scaling group.</p>
    pub auto_scaling_group_name: Option<String>,
    /// <p>The amount of time, in seconds, after a scaling activity completes before any further dynamic scaling activities can start.</p>
    pub cooldown: Option<i64>,
    /// <p>The estimated time, in seconds, until a newly launched instance can contribute to the CloudWatch metrics.</p>
    pub estimated_instance_warmup: Option<i64>,
    /// <p>The aggregation type for the CloudWatch metrics. Valid values are <code>Minimum</code>, <code>Maximum</code>, and <code>Average</code>.</p>
    pub metric_aggregation_type: Option<String>,
    /// <p>The minimum number of instances to scale. If the value of <code>AdjustmentType</code> is <code>PercentChangeInCapacity</code>, the scaling policy changes the <code>DesiredCapacity</code> of the Auto Scaling group by at least this many instances. Otherwise, the error is <code>ValidationError</code>.</p>
    pub min_adjustment_magnitude: Option<i64>,
    /// <p>Available for backward compatibility. Use <code>MinAdjustmentMagnitude</code> instead.</p>
    pub min_adjustment_step: Option<i64>,
    /// <p>The Amazon Resource Name (ARN) of the policy.</p>
    pub policy_arn: Option<String>,
    /// <p>The name of the scaling policy.</p>
    pub policy_name: Option<String>,
    /// <p>The policy type. Valid values are <code>SimpleScaling</code> and <code>StepScaling</code>.</p>
    pub policy_type: Option<String>,
    /// <p>The amount by which to scale, based on the specified adjustment type. A positive value adds to the current capacity while a negative number removes from the current capacity.</p>
    pub scaling_adjustment: Option<i64>,
    /// <p>A set of adjustments that enable you to scale based on the size of the alarm breach.</p>
    pub step_adjustments: Option<Vec<StepAdjustment>>,
    /// <p>A target tracking policy.</p>
    pub target_tracking_configuration: Option<TargetTrackingConfiguration>,
}

struct ScalingPolicyDeserializer;
impl ScalingPolicyDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ScalingPolicy, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ScalingPolicy::default();

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
                    "AdjustmentType" => {
                        obj.adjustment_type = Some(try!(
                            XmlStringMaxLen255Deserializer::deserialize("AdjustmentType", stack)
                        ));
                    }
                    "Alarms" => {
                        obj.alarms = Some(try!(AlarmsDeserializer::deserialize("Alarms", stack)));
                    }
                    "AutoScalingGroupName" => {
                        obj.auto_scaling_group_name =
                            Some(try!(XmlStringMaxLen255Deserializer::deserialize(
                                "AutoScalingGroupName",
                                stack
                            )));
                    }
                    "Cooldown" => {
                        obj.cooldown =
                            Some(try!(CooldownDeserializer::deserialize("Cooldown", stack)));
                    }
                    "EstimatedInstanceWarmup" => {
                        obj.estimated_instance_warmup =
                            Some(try!(EstimatedInstanceWarmupDeserializer::deserialize(
                                "EstimatedInstanceWarmup",
                                stack
                            )));
                    }
                    "MetricAggregationType" => {
                        obj.metric_aggregation_type =
                            Some(try!(XmlStringMaxLen32Deserializer::deserialize(
                                "MetricAggregationType",
                                stack
                            )));
                    }
                    "MinAdjustmentMagnitude" => {
                        obj.min_adjustment_magnitude =
                            Some(try!(MinAdjustmentMagnitudeDeserializer::deserialize(
                                "MinAdjustmentMagnitude",
                                stack
                            )));
                    }
                    "MinAdjustmentStep" => {
                        obj.min_adjustment_step = Some(try!(
                            MinAdjustmentStepDeserializer::deserialize("MinAdjustmentStep", stack)
                        ));
                    }
                    "PolicyARN" => {
                        obj.policy_arn = Some(try!(ResourceNameDeserializer::deserialize(
                            "PolicyARN",
                            stack
                        )));
                    }
                    "PolicyName" => {
                        obj.policy_name = Some(try!(XmlStringMaxLen255Deserializer::deserialize(
                            "PolicyName",
                            stack
                        )));
                    }
                    "PolicyType" => {
                        obj.policy_type = Some(try!(XmlStringMaxLen64Deserializer::deserialize(
                            "PolicyType",
                            stack
                        )));
                    }
                    "ScalingAdjustment" => {
                        obj.scaling_adjustment = Some(try!(
                            PolicyIncrementDeserializer::deserialize("ScalingAdjustment", stack)
                        ));
                    }
                    "StepAdjustments" => {
                        obj.step_adjustments = Some(try!(
                            StepAdjustmentsDeserializer::deserialize("StepAdjustments", stack)
                        ));
                    }
                    "TargetTrackingConfiguration" => {
                        obj.target_tracking_configuration =
                            Some(try!(TargetTrackingConfigurationDeserializer::deserialize(
                                "TargetTrackingConfiguration",
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
pub struct ScalingProcessQuery {
    /// <p>The name of the Auto Scaling group.</p>
    pub auto_scaling_group_name: String,
    /// <p><p>One or more of the following processes. If you omit this parameter, all processes are specified.</p> <ul> <li> <p> <code>Launch</code> </p> </li> <li> <p> <code>Terminate</code> </p> </li> <li> <p> <code>HealthCheck</code> </p> </li> <li> <p> <code>ReplaceUnhealthy</code> </p> </li> <li> <p> <code>AZRebalance</code> </p> </li> <li> <p> <code>AlarmNotification</code> </p> </li> <li> <p> <code>ScheduledActions</code> </p> </li> <li> <p> <code>AddToLoadBalancer</code> </p> </li> </ul></p>
    pub scaling_processes: Option<Vec<String>>,
}

/// Serialize `ScalingProcessQuery` contents to a `SignedRequest`.
struct ScalingProcessQuerySerializer;
impl ScalingProcessQuerySerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ScalingProcessQuery) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "AutoScalingGroupName"),
            &obj.auto_scaling_group_name.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.scaling_processes {
            ProcessNamesSerializer::serialize(
                params,
                &format!("{}{}", prefix, "ScalingProcesses"),
                field_value,
            );
        }
    }
}

/// Serialize `ScheduledActionNames` contents to a `SignedRequest`.
struct ScheduledActionNamesSerializer;
impl ScheduledActionNamesSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct ScheduledActionsType {
    /// <p>The token to use when requesting the next set of items. If there are no additional items to return, the string is empty.</p>
    pub next_token: Option<String>,
    /// <p>The scheduled actions.</p>
    pub scheduled_update_group_actions: Option<Vec<ScheduledUpdateGroupAction>>,
}

struct ScheduledActionsTypeDeserializer;
impl ScheduledActionsTypeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ScheduledActionsType, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ScheduledActionsType::default();

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
                            Some(try!(XmlStringDeserializer::deserialize("NextToken", stack)));
                    }
                    "ScheduledUpdateGroupActions" => {
                        obj.scheduled_update_group_actions =
                            Some(try!(ScheduledUpdateGroupActionsDeserializer::deserialize(
                                "ScheduledUpdateGroupActions",
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
/// <p>Describes a scheduled update to an Auto Scaling group.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ScheduledUpdateGroupAction {
    /// <p>The name of the Auto Scaling group.</p>
    pub auto_scaling_group_name: Option<String>,
    /// <p>The number of instances you prefer to maintain in the group.</p>
    pub desired_capacity: Option<i64>,
    /// <p>The date and time that the action is scheduled to end. This date and time can be up to one month in the future.</p>
    pub end_time: Option<String>,
    /// <p>The maximum size of the group.</p>
    pub max_size: Option<i64>,
    /// <p>The minimum size of the group.</p>
    pub min_size: Option<i64>,
    /// <p>The recurring schedule for the action.</p>
    pub recurrence: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the scheduled action.</p>
    pub scheduled_action_arn: Option<String>,
    /// <p>The name of the scheduled action.</p>
    pub scheduled_action_name: Option<String>,
    /// <p>The date and time that the action is scheduled to begin. This date and time can be up to one month in the future.</p> <p>When <code>StartTime</code> and <code>EndTime</code> are specified with <code>Recurrence</code>, they form the boundaries of when the recurring action will start and stop.</p>
    pub start_time: Option<String>,
    /// <p>This parameter is deprecated.</p>
    pub time: Option<String>,
}

struct ScheduledUpdateGroupActionDeserializer;
impl ScheduledUpdateGroupActionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ScheduledUpdateGroupAction, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ScheduledUpdateGroupAction::default();

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
                    "AutoScalingGroupName" => {
                        obj.auto_scaling_group_name =
                            Some(try!(XmlStringMaxLen255Deserializer::deserialize(
                                "AutoScalingGroupName",
                                stack
                            )));
                    }
                    "DesiredCapacity" => {
                        obj.desired_capacity = Some(try!(
                            AutoScalingGroupDesiredCapacityDeserializer::deserialize(
                                "DesiredCapacity",
                                stack
                            )
                        ));
                    }
                    "EndTime" => {
                        obj.end_time = Some(try!(TimestampTypeDeserializer::deserialize(
                            "EndTime", stack
                        )));
                    }
                    "MaxSize" => {
                        obj.max_size = Some(try!(
                            AutoScalingGroupMaxSizeDeserializer::deserialize("MaxSize", stack)
                        ));
                    }
                    "MinSize" => {
                        obj.min_size = Some(try!(
                            AutoScalingGroupMinSizeDeserializer::deserialize("MinSize", stack)
                        ));
                    }
                    "Recurrence" => {
                        obj.recurrence = Some(try!(XmlStringMaxLen255Deserializer::deserialize(
                            "Recurrence",
                            stack
                        )));
                    }
                    "ScheduledActionARN" => {
                        obj.scheduled_action_arn = Some(try!(
                            ResourceNameDeserializer::deserialize("ScheduledActionARN", stack)
                        ));
                    }
                    "ScheduledActionName" => {
                        obj.scheduled_action_name =
                            Some(try!(XmlStringMaxLen255Deserializer::deserialize(
                                "ScheduledActionName",
                                stack
                            )));
                    }
                    "StartTime" => {
                        obj.start_time = Some(try!(TimestampTypeDeserializer::deserialize(
                            "StartTime",
                            stack
                        )));
                    }
                    "Time" => {
                        obj.time =
                            Some(try!(TimestampTypeDeserializer::deserialize("Time", stack)));
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
struct ScheduledUpdateGroupActionsDeserializer;
impl ScheduledUpdateGroupActionsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<ScheduledUpdateGroupAction>, XmlParseError> {
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
                        obj.push(try!(ScheduledUpdateGroupActionDeserializer::deserialize(
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
                        obj.push(try!(XmlStringDeserializer::deserialize("member", stack)));
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
pub struct SetDesiredCapacityType {
    /// <p>The name of the Auto Scaling group.</p>
    pub auto_scaling_group_name: String,
    /// <p>The number of EC2 instances that should be running in the Auto Scaling group.</p>
    pub desired_capacity: i64,
    /// <p>Indicates whether Auto Scaling waits for the cooldown period to complete before initiating a scaling activity to set your Auto Scaling group to its new capacity. By default, Auto Scaling does not honor the cooldown period during manual scaling activities.</p>
    pub honor_cooldown: Option<bool>,
}

/// Serialize `SetDesiredCapacityType` contents to a `SignedRequest`.
struct SetDesiredCapacityTypeSerializer;
impl SetDesiredCapacityTypeSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &SetDesiredCapacityType) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "AutoScalingGroupName"),
            &obj.auto_scaling_group_name.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "DesiredCapacity"),
            &obj.desired_capacity.to_string().replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.honor_cooldown {
            params.put(
                &format!("{}{}", prefix, "HonorCooldown"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct SetInstanceHealthQuery {
    /// <p>The health status of the instance. Set to <code>Healthy</code> if you want the instance to remain in service. Set to <code>Unhealthy</code> if you want the instance to be out of service. Auto Scaling will terminate and replace the unhealthy instance.</p>
    pub health_status: String,
    /// <p>The ID of the instance.</p>
    pub instance_id: String,
    /// <p>If the Auto Scaling group of the specified instance has a <code>HealthCheckGracePeriod</code> specified for the group, by default, this call will respect the grace period. Set this to <code>False</code>, if you do not want the call to respect the grace period associated with the group.</p> <p>For more information, see the description of the health check grace period for <a>CreateAutoScalingGroup</a>.</p>
    pub should_respect_grace_period: Option<bool>,
}

/// Serialize `SetInstanceHealthQuery` contents to a `SignedRequest`.
struct SetInstanceHealthQuerySerializer;
impl SetInstanceHealthQuerySerializer {
    fn serialize(params: &mut Params, name: &str, obj: &SetInstanceHealthQuery) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "HealthStatus"),
            &obj.health_status.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "InstanceId"),
            &obj.instance_id.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.should_respect_grace_period {
            params.put(
                &format!("{}{}", prefix, "ShouldRespectGracePeriod"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct SetInstanceProtectionAnswer {}

struct SetInstanceProtectionAnswerDeserializer;
impl SetInstanceProtectionAnswerDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<SetInstanceProtectionAnswer, XmlParseError> {
        try!(start_element(tag_name, stack));

        let obj = SetInstanceProtectionAnswer::default();

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SetInstanceProtectionQuery {
    /// <p>The name of the Auto Scaling group.</p>
    pub auto_scaling_group_name: String,
    /// <p>One or more instance IDs.</p>
    pub instance_ids: Vec<String>,
    /// <p>Indicates whether the instance is protected from termination by Auto Scaling when scaling in.</p>
    pub protected_from_scale_in: bool,
}

/// Serialize `SetInstanceProtectionQuery` contents to a `SignedRequest`.
struct SetInstanceProtectionQuerySerializer;
impl SetInstanceProtectionQuerySerializer {
    fn serialize(params: &mut Params, name: &str, obj: &SetInstanceProtectionQuery) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "AutoScalingGroupName"),
            &obj.auto_scaling_group_name.replace("+", "%2B"),
        );
        InstanceIdsSerializer::serialize(
            params,
            &format!("{}{}", prefix, "InstanceIds"),
            &obj.instance_ids,
        );
        params.put(
            &format!("{}{}", prefix, "ProtectedFromScaleIn"),
            &obj.protected_from_scale_in.to_string().replace("+", "%2B"),
        );
    }
}

struct SpotPriceDeserializer;
impl SpotPriceDeserializer {
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
/// <p><p>Describes an adjustment based on the difference between the value of the aggregated CloudWatch metric and the breach threshold that you&#39;ve defined for the alarm.</p> <p>For the following examples, suppose that you have an alarm with a breach threshold of 50:</p> <ul> <li> <p>If you want the adjustment to be triggered when the metric is greater than or equal to 50 and less than 60, specify a lower bound of 0 and an upper bound of 10.</p> </li> <li> <p>If you want the adjustment to be triggered when the metric is greater than 40 and less than or equal to 50, specify a lower bound of -10 and an upper bound of 0.</p> </li> </ul> <p>There are a few rules for the step adjustments for your step policy:</p> <ul> <li> <p>The ranges of your step adjustments can&#39;t overlap or have a gap.</p> </li> <li> <p>At most one step adjustment can have a null lower bound. If one step adjustment has a negative lower bound, then there must be a step adjustment with a null lower bound.</p> </li> <li> <p>At most one step adjustment can have a null upper bound. If one step adjustment has a positive upper bound, then there must be a step adjustment with a null upper bound.</p> </li> <li> <p>The upper and lower bound can&#39;t be null in the same step adjustment.</p> </li> </ul></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct StepAdjustment {
    /// <p>The lower bound for the difference between the alarm threshold and the CloudWatch metric. If the metric value is above the breach threshold, the lower bound is inclusive (the metric must be greater than or equal to the threshold plus the lower bound). Otherwise, it is exclusive (the metric must be greater than the threshold plus the lower bound). A null value indicates negative infinity.</p>
    pub metric_interval_lower_bound: Option<f64>,
    /// <p>The upper bound for the difference between the alarm threshold and the CloudWatch metric. If the metric value is above the breach threshold, the upper bound is exclusive (the metric must be less than the threshold plus the upper bound). Otherwise, it is inclusive (the metric must be less than or equal to the threshold plus the upper bound). A null value indicates positive infinity.</p> <p>The upper bound must be greater than the lower bound.</p>
    pub metric_interval_upper_bound: Option<f64>,
    /// <p>The amount by which to scale, based on the specified adjustment type. A positive value adds to the current capacity while a negative number removes from the current capacity.</p>
    pub scaling_adjustment: i64,
}

struct StepAdjustmentDeserializer;
impl StepAdjustmentDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<StepAdjustment, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = StepAdjustment::default();

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
                    "MetricIntervalLowerBound" => {
                        obj.metric_interval_lower_bound = Some(try!(
                            MetricScaleDeserializer::deserialize("MetricIntervalLowerBound", stack)
                        ));
                    }
                    "MetricIntervalUpperBound" => {
                        obj.metric_interval_upper_bound = Some(try!(
                            MetricScaleDeserializer::deserialize("MetricIntervalUpperBound", stack)
                        ));
                    }
                    "ScalingAdjustment" => {
                        obj.scaling_adjustment = try!(PolicyIncrementDeserializer::deserialize(
                            "ScalingAdjustment",
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

/// Serialize `StepAdjustment` contents to a `SignedRequest`.
struct StepAdjustmentSerializer;
impl StepAdjustmentSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &StepAdjustment) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.metric_interval_lower_bound {
            params.put(
                &format!("{}{}", prefix, "MetricIntervalLowerBound"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.metric_interval_upper_bound {
            params.put(
                &format!("{}{}", prefix, "MetricIntervalUpperBound"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        params.put(
            &format!("{}{}", prefix, "ScalingAdjustment"),
            &obj.scaling_adjustment.to_string().replace("+", "%2B"),
        );
    }
}

struct StepAdjustmentsDeserializer;
impl StepAdjustmentsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<StepAdjustment>, XmlParseError> {
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
                        obj.push(try!(StepAdjustmentDeserializer::deserialize(
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

/// Serialize `StepAdjustments` contents to a `SignedRequest`.
struct StepAdjustmentsSerializer;
impl StepAdjustmentsSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<StepAdjustment>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            StepAdjustmentSerializer::serialize(params, &key, obj);
        }
    }
}

/// <p>Describes an Auto Scaling process that has been suspended. For more information, see <a>ProcessType</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SuspendedProcess {
    /// <p>The name of the suspended process.</p>
    pub process_name: Option<String>,
    /// <p>The reason that the process was suspended.</p>
    pub suspension_reason: Option<String>,
}

struct SuspendedProcessDeserializer;
impl SuspendedProcessDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<SuspendedProcess, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = SuspendedProcess::default();

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
                    "ProcessName" => {
                        obj.process_name = Some(try!(XmlStringMaxLen255Deserializer::deserialize(
                            "ProcessName",
                            stack
                        )));
                    }
                    "SuspensionReason" => {
                        obj.suspension_reason = Some(try!(
                            XmlStringMaxLen255Deserializer::deserialize("SuspensionReason", stack)
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
struct SuspendedProcessesDeserializer;
impl SuspendedProcessesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<SuspendedProcess>, XmlParseError> {
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
                        obj.push(try!(SuspendedProcessDeserializer::deserialize(
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
/// <p>Describes a tag for an Auto Scaling group.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Tag {
    /// <p>The tag key.</p>
    pub key: String,
    /// <p>Determines whether the tag is added to new instances as they are launched in the group.</p>
    pub propagate_at_launch: Option<bool>,
    /// <p>The name of the group.</p>
    pub resource_id: Option<String>,
    /// <p>The type of resource. The only supported value is <code>auto-scaling-group</code>.</p>
    pub resource_type: Option<String>,
    /// <p>The tag value.</p>
    pub value: Option<String>,
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
        if let Some(ref field_value) = obj.propagate_at_launch {
            params.put(
                &format!("{}{}", prefix, "PropagateAtLaunch"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.resource_id {
            params.put(
                &format!("{}{}", prefix, "ResourceId"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.resource_type {
            params.put(
                &format!("{}{}", prefix, "ResourceType"),
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

/// <p>Describes a tag for an Auto Scaling group.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct TagDescription {
    /// <p>The tag key.</p>
    pub key: Option<String>,
    /// <p>Determines whether the tag is added to new instances as they are launched in the group.</p>
    pub propagate_at_launch: Option<bool>,
    /// <p>The name of the group.</p>
    pub resource_id: Option<String>,
    /// <p>The type of resource. The only supported value is <code>auto-scaling-group</code>.</p>
    pub resource_type: Option<String>,
    /// <p>The tag value.</p>
    pub value: Option<String>,
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
                    "Key" => {
                        obj.key = Some(try!(TagKeyDeserializer::deserialize("Key", stack)));
                    }
                    "PropagateAtLaunch" => {
                        obj.propagate_at_launch = Some(try!(
                            PropagateAtLaunchDeserializer::deserialize("PropagateAtLaunch", stack)
                        ));
                    }
                    "ResourceId" => {
                        obj.resource_id = Some(try!(XmlStringDeserializer::deserialize(
                            "ResourceId",
                            stack
                        )));
                    }
                    "ResourceType" => {
                        obj.resource_type = Some(try!(XmlStringDeserializer::deserialize(
                            "ResourceType",
                            stack
                        )));
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
struct TagDescriptionListDeserializer;
impl TagDescriptionListDeserializer {
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

/// Serialize `Tags` contents to a `SignedRequest`.
struct TagsSerializer;
impl TagsSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<Tag>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            TagSerializer::serialize(params, &key, obj);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct TagsType {
    /// <p>The token to use when requesting the next set of items. If there are no additional items to return, the string is empty.</p>
    pub next_token: Option<String>,
    /// <p>One or more tags.</p>
    pub tags: Option<Vec<TagDescription>>,
}

struct TagsTypeDeserializer;
impl TagsTypeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<TagsType, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = TagsType::default();

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
                            Some(try!(XmlStringDeserializer::deserialize("NextToken", stack)));
                    }
                    "Tags" => {
                        obj.tags = Some(try!(TagDescriptionListDeserializer::deserialize(
                            "Tags", stack
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
struct TargetGroupARNsDeserializer;
impl TargetGroupARNsDeserializer {
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
                        obj.push(try!(XmlStringMaxLen511Deserializer::deserialize(
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

/// Serialize `TargetGroupARNs` contents to a `SignedRequest`.
struct TargetGroupARNsSerializer;
impl TargetGroupARNsSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

/// <p>Represents a target tracking policy configuration.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct TargetTrackingConfiguration {
    /// <p>A customized metric.</p>
    pub customized_metric_specification: Option<CustomizedMetricSpecification>,
    /// <p>Indicates whether scale in by the target tracking policy is disabled. If scale in is disabled, the target tracking policy won't remove instances from the Auto Scaling group. Otherwise, the target tracking policy can remove instances from the Auto Scaling group. The default is disabled.</p>
    pub disable_scale_in: Option<bool>,
    /// <p>A predefined metric. You can specify either a predefined metric or a customized metric.</p>
    pub predefined_metric_specification: Option<PredefinedMetricSpecification>,
    /// <p>The target value for the metric.</p>
    pub target_value: f64,
}

struct TargetTrackingConfigurationDeserializer;
impl TargetTrackingConfigurationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<TargetTrackingConfiguration, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = TargetTrackingConfiguration::default();

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
                    "CustomizedMetricSpecification" => {
                        obj.customized_metric_specification = Some(try!(
                            CustomizedMetricSpecificationDeserializer::deserialize(
                                "CustomizedMetricSpecification",
                                stack
                            )
                        ));
                    }
                    "DisableScaleIn" => {
                        obj.disable_scale_in = Some(try!(DisableScaleInDeserializer::deserialize(
                            "DisableScaleIn",
                            stack
                        )));
                    }
                    "PredefinedMetricSpecification" => {
                        obj.predefined_metric_specification = Some(try!(
                            PredefinedMetricSpecificationDeserializer::deserialize(
                                "PredefinedMetricSpecification",
                                stack
                            )
                        ));
                    }
                    "TargetValue" => {
                        obj.target_value =
                            try!(MetricScaleDeserializer::deserialize("TargetValue", stack));
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

/// Serialize `TargetTrackingConfiguration` contents to a `SignedRequest`.
struct TargetTrackingConfigurationSerializer;
impl TargetTrackingConfigurationSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &TargetTrackingConfiguration) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.customized_metric_specification {
            CustomizedMetricSpecificationSerializer::serialize(
                params,
                &format!("{}{}", prefix, "CustomizedMetricSpecification"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.disable_scale_in {
            params.put(
                &format!("{}{}", prefix, "DisableScaleIn"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.predefined_metric_specification {
            PredefinedMetricSpecificationSerializer::serialize(
                params,
                &format!("{}{}", prefix, "PredefinedMetricSpecification"),
                field_value,
            );
        }
        params.put(
            &format!("{}{}", prefix, "TargetValue"),
            &obj.target_value.to_string().replace("+", "%2B"),
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct TerminateInstanceInAutoScalingGroupType {
    /// <p>The ID of the instance.</p>
    pub instance_id: String,
    /// <p>Indicates whether terminating the instance also decrements the size of the Auto Scaling group.</p>
    pub should_decrement_desired_capacity: bool,
}

/// Serialize `TerminateInstanceInAutoScalingGroupType` contents to a `SignedRequest`.
struct TerminateInstanceInAutoScalingGroupTypeSerializer;
impl TerminateInstanceInAutoScalingGroupTypeSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &TerminateInstanceInAutoScalingGroupType) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "InstanceId"),
            &obj.instance_id.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "ShouldDecrementDesiredCapacity"),
            &obj.should_decrement_desired_capacity
                .to_string()
                .replace("+", "%2B"),
        );
    }
}

struct TerminationPoliciesDeserializer;
impl TerminationPoliciesDeserializer {
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
                        obj.push(try!(XmlStringMaxLen1600Deserializer::deserialize(
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

/// Serialize `TerminationPolicies` contents to a `SignedRequest`.
struct TerminationPoliciesSerializer;
impl TerminationPoliciesSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

struct TimestampTypeDeserializer;
impl TimestampTypeDeserializer {
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
pub struct UpdateAutoScalingGroupType {
    /// <p>The name of the Auto Scaling group.</p>
    pub auto_scaling_group_name: String,
    /// <p>One or more Availability Zones for the group.</p>
    pub availability_zones: Option<Vec<String>>,
    /// <p>The amount of time, in seconds, after a scaling activity completes before another scaling activity can start. The default is 300.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/autoscaling/latest/userguide/Cooldown.html">Auto Scaling Cooldowns</a> in the <i>Auto Scaling User Guide</i>.</p>
    pub default_cooldown: Option<i64>,
    /// <p>The number of EC2 instances that should be running in the Auto Scaling group. This number must be greater than or equal to the minimum size of the group and less than or equal to the maximum size of the group.</p>
    pub desired_capacity: Option<i64>,
    /// <p>The amount of time, in seconds, that Auto Scaling waits before checking the health status of an EC2 instance that has come into service. The default is 0.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/autoscaling/latest/userguide/healthcheck.html">Health Checks</a> in the <i>Auto Scaling User Guide</i>.</p>
    pub health_check_grace_period: Option<i64>,
    /// <p>The service to use for the health checks. The valid values are <code>EC2</code> and <code>ELB</code>.</p>
    pub health_check_type: Option<String>,
    /// <p>The name of the launch configuration. If you specify a launch configuration, you can't specify a launch template.</p>
    pub launch_configuration_name: Option<String>,
    /// <p>The launch template to use to specify the updates. If you specify a launch template, you can't specify a launch configuration.</p>
    pub launch_template: Option<LaunchTemplateSpecification>,
    /// <p>The maximum size of the Auto Scaling group.</p>
    pub max_size: Option<i64>,
    /// <p>The minimum size of the Auto Scaling group.</p>
    pub min_size: Option<i64>,
    /// <p>Indicates whether newly launched instances are protected from termination by Auto Scaling when scaling in.</p>
    pub new_instances_protected_from_scale_in: Option<bool>,
    /// <p>The name of the placement group into which you'll launch your instances, if any. For more information, see <a href="http://docs.aws.amazon.com/AWSEC2/latest/UserGuide/placement-groups.html">Placement Groups</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
    pub placement_group: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the service-linked role that the Auto Scaling group uses to call other AWS services on your behalf.</p>
    pub service_linked_role_arn: Option<String>,
    /// <p>A standalone termination policy or a list of termination policies used to select the instance to terminate. The policies are executed in the order that they are listed.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/autoscaling/latest/userguide/as-instance-termination.html">Controlling Which Instances Auto Scaling Terminates During Scale In</a> in the <i>Auto Scaling User Guide</i>.</p>
    pub termination_policies: Option<Vec<String>>,
    /// <p>The ID of the subnet, if you are launching into a VPC. You can specify several subnets in a comma-separated list.</p> <p>When you specify <code>VPCZoneIdentifier</code> with <code>AvailabilityZones</code>, ensure that the subnets' Availability Zones match the values you specify for <code>AvailabilityZones</code>.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/autoscaling/latest/userguide/asg-in-vpc.html">Launching Auto Scaling Instances in a VPC</a> in the <i>Auto Scaling User Guide</i>.</p>
    pub vpc_zone_identifier: Option<String>,
}

/// Serialize `UpdateAutoScalingGroupType` contents to a `SignedRequest`.
struct UpdateAutoScalingGroupTypeSerializer;
impl UpdateAutoScalingGroupTypeSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &UpdateAutoScalingGroupType) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "AutoScalingGroupName"),
            &obj.auto_scaling_group_name.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.availability_zones {
            AvailabilityZonesSerializer::serialize(
                params,
                &format!("{}{}", prefix, "AvailabilityZones"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.default_cooldown {
            params.put(
                &format!("{}{}", prefix, "DefaultCooldown"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.desired_capacity {
            params.put(
                &format!("{}{}", prefix, "DesiredCapacity"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.health_check_grace_period {
            params.put(
                &format!("{}{}", prefix, "HealthCheckGracePeriod"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.health_check_type {
            params.put(
                &format!("{}{}", prefix, "HealthCheckType"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.launch_configuration_name {
            params.put(
                &format!("{}{}", prefix, "LaunchConfigurationName"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.launch_template {
            LaunchTemplateSpecificationSerializer::serialize(
                params,
                &format!("{}{}", prefix, "LaunchTemplate"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.max_size {
            params.put(
                &format!("{}{}", prefix, "MaxSize"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.min_size {
            params.put(
                &format!("{}{}", prefix, "MinSize"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.new_instances_protected_from_scale_in {
            params.put(
                &format!("{}{}", prefix, "NewInstancesProtectedFromScaleIn"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.placement_group {
            params.put(
                &format!("{}{}", prefix, "PlacementGroup"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.service_linked_role_arn {
            params.put(
                &format!("{}{}", prefix, "ServiceLinkedRoleARN"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.termination_policies {
            TerminationPoliciesSerializer::serialize(
                params,
                &format!("{}{}", prefix, "TerminationPolicies"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.vpc_zone_identifier {
            params.put(
                &format!("{}{}", prefix, "VPCZoneIdentifier"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

/// Serialize `Values` contents to a `SignedRequest`.
struct ValuesSerializer;
impl ValuesSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

struct XmlStringDeserializer;
impl XmlStringDeserializer {
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
struct XmlStringMaxLen1023Deserializer;
impl XmlStringMaxLen1023Deserializer {
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
struct XmlStringMaxLen1600Deserializer;
impl XmlStringMaxLen1600Deserializer {
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
struct XmlStringMaxLen19Deserializer;
impl XmlStringMaxLen19Deserializer {
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
struct XmlStringMaxLen2047Deserializer;
impl XmlStringMaxLen2047Deserializer {
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
struct XmlStringMaxLen255Deserializer;
impl XmlStringMaxLen255Deserializer {
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
struct XmlStringMaxLen32Deserializer;
impl XmlStringMaxLen32Deserializer {
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
struct XmlStringMaxLen511Deserializer;
impl XmlStringMaxLen511Deserializer {
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
struct XmlStringMaxLen64Deserializer;
impl XmlStringMaxLen64Deserializer {
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
struct XmlStringUserDataDeserializer;
impl XmlStringUserDataDeserializer {
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
/// Errors returned by AttachInstances
#[derive(Debug, PartialEq)]
pub enum AttachInstancesError {
    /// <p>You already have a pending update to an Auto Scaling resource (for example, a group, instance, or load balancer).</p>
    ResourceContentionFault(String),
    /// <p>The service-linked role is not yet ready for use.</p>
    ServiceLinkedRoleFailure(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl AttachInstancesError {
    pub fn from_body(body: &str) -> AttachInstancesError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ResourceContention" => AttachInstancesError::ResourceContentionFault(
                    String::from(parsed_error.message),
                ),
                "ServiceLinkedRoleFailure" => AttachInstancesError::ServiceLinkedRoleFailure(
                    String::from(parsed_error.message),
                ),
                _ => AttachInstancesError::Unknown(String::from(body)),
            },
            Err(_) => AttachInstancesError::Unknown(body.to_string()),
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

impl From<XmlParseError> for AttachInstancesError {
    fn from(err: XmlParseError) -> AttachInstancesError {
        let XmlParseError(message) = err;
        AttachInstancesError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for AttachInstancesError {
    fn from(err: CredentialsError) -> AttachInstancesError {
        AttachInstancesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AttachInstancesError {
    fn from(err: HttpDispatchError) -> AttachInstancesError {
        AttachInstancesError::HttpDispatch(err)
    }
}
impl From<io::Error> for AttachInstancesError {
    fn from(err: io::Error) -> AttachInstancesError {
        AttachInstancesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AttachInstancesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AttachInstancesError {
    fn description(&self) -> &str {
        match *self {
            AttachInstancesError::ResourceContentionFault(ref cause) => cause,
            AttachInstancesError::ServiceLinkedRoleFailure(ref cause) => cause,
            AttachInstancesError::Validation(ref cause) => cause,
            AttachInstancesError::Credentials(ref err) => err.description(),
            AttachInstancesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            AttachInstancesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by AttachLoadBalancerTargetGroups
#[derive(Debug, PartialEq)]
pub enum AttachLoadBalancerTargetGroupsError {
    /// <p>You already have a pending update to an Auto Scaling resource (for example, a group, instance, or load balancer).</p>
    ResourceContentionFault(String),
    /// <p>The service-linked role is not yet ready for use.</p>
    ServiceLinkedRoleFailure(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl AttachLoadBalancerTargetGroupsError {
    pub fn from_body(body: &str) -> AttachLoadBalancerTargetGroupsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ResourceContention" => {
                    AttachLoadBalancerTargetGroupsError::ResourceContentionFault(String::from(
                        parsed_error.message,
                    ))
                }
                "ServiceLinkedRoleFailure" => {
                    AttachLoadBalancerTargetGroupsError::ServiceLinkedRoleFailure(String::from(
                        parsed_error.message,
                    ))
                }
                _ => AttachLoadBalancerTargetGroupsError::Unknown(String::from(body)),
            },
            Err(_) => AttachLoadBalancerTargetGroupsError::Unknown(body.to_string()),
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

impl From<XmlParseError> for AttachLoadBalancerTargetGroupsError {
    fn from(err: XmlParseError) -> AttachLoadBalancerTargetGroupsError {
        let XmlParseError(message) = err;
        AttachLoadBalancerTargetGroupsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for AttachLoadBalancerTargetGroupsError {
    fn from(err: CredentialsError) -> AttachLoadBalancerTargetGroupsError {
        AttachLoadBalancerTargetGroupsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AttachLoadBalancerTargetGroupsError {
    fn from(err: HttpDispatchError) -> AttachLoadBalancerTargetGroupsError {
        AttachLoadBalancerTargetGroupsError::HttpDispatch(err)
    }
}
impl From<io::Error> for AttachLoadBalancerTargetGroupsError {
    fn from(err: io::Error) -> AttachLoadBalancerTargetGroupsError {
        AttachLoadBalancerTargetGroupsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AttachLoadBalancerTargetGroupsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AttachLoadBalancerTargetGroupsError {
    fn description(&self) -> &str {
        match *self {
            AttachLoadBalancerTargetGroupsError::ResourceContentionFault(ref cause) => cause,
            AttachLoadBalancerTargetGroupsError::ServiceLinkedRoleFailure(ref cause) => cause,
            AttachLoadBalancerTargetGroupsError::Validation(ref cause) => cause,
            AttachLoadBalancerTargetGroupsError::Credentials(ref err) => err.description(),
            AttachLoadBalancerTargetGroupsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AttachLoadBalancerTargetGroupsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by AttachLoadBalancers
#[derive(Debug, PartialEq)]
pub enum AttachLoadBalancersError {
    /// <p>You already have a pending update to an Auto Scaling resource (for example, a group, instance, or load balancer).</p>
    ResourceContentionFault(String),
    /// <p>The service-linked role is not yet ready for use.</p>
    ServiceLinkedRoleFailure(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl AttachLoadBalancersError {
    pub fn from_body(body: &str) -> AttachLoadBalancersError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ResourceContention" => AttachLoadBalancersError::ResourceContentionFault(
                    String::from(parsed_error.message),
                ),
                "ServiceLinkedRoleFailure" => AttachLoadBalancersError::ServiceLinkedRoleFailure(
                    String::from(parsed_error.message),
                ),
                _ => AttachLoadBalancersError::Unknown(String::from(body)),
            },
            Err(_) => AttachLoadBalancersError::Unknown(body.to_string()),
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

impl From<XmlParseError> for AttachLoadBalancersError {
    fn from(err: XmlParseError) -> AttachLoadBalancersError {
        let XmlParseError(message) = err;
        AttachLoadBalancersError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for AttachLoadBalancersError {
    fn from(err: CredentialsError) -> AttachLoadBalancersError {
        AttachLoadBalancersError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AttachLoadBalancersError {
    fn from(err: HttpDispatchError) -> AttachLoadBalancersError {
        AttachLoadBalancersError::HttpDispatch(err)
    }
}
impl From<io::Error> for AttachLoadBalancersError {
    fn from(err: io::Error) -> AttachLoadBalancersError {
        AttachLoadBalancersError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AttachLoadBalancersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AttachLoadBalancersError {
    fn description(&self) -> &str {
        match *self {
            AttachLoadBalancersError::ResourceContentionFault(ref cause) => cause,
            AttachLoadBalancersError::ServiceLinkedRoleFailure(ref cause) => cause,
            AttachLoadBalancersError::Validation(ref cause) => cause,
            AttachLoadBalancersError::Credentials(ref err) => err.description(),
            AttachLoadBalancersError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AttachLoadBalancersError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CompleteLifecycleAction
#[derive(Debug, PartialEq)]
pub enum CompleteLifecycleActionError {
    /// <p>You already have a pending update to an Auto Scaling resource (for example, a group, instance, or load balancer).</p>
    ResourceContentionFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CompleteLifecycleActionError {
    pub fn from_body(body: &str) -> CompleteLifecycleActionError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ResourceContention" => CompleteLifecycleActionError::ResourceContentionFault(
                    String::from(parsed_error.message),
                ),
                _ => CompleteLifecycleActionError::Unknown(String::from(body)),
            },
            Err(_) => CompleteLifecycleActionError::Unknown(body.to_string()),
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

impl From<XmlParseError> for CompleteLifecycleActionError {
    fn from(err: XmlParseError) -> CompleteLifecycleActionError {
        let XmlParseError(message) = err;
        CompleteLifecycleActionError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for CompleteLifecycleActionError {
    fn from(err: CredentialsError) -> CompleteLifecycleActionError {
        CompleteLifecycleActionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CompleteLifecycleActionError {
    fn from(err: HttpDispatchError) -> CompleteLifecycleActionError {
        CompleteLifecycleActionError::HttpDispatch(err)
    }
}
impl From<io::Error> for CompleteLifecycleActionError {
    fn from(err: io::Error) -> CompleteLifecycleActionError {
        CompleteLifecycleActionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CompleteLifecycleActionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CompleteLifecycleActionError {
    fn description(&self) -> &str {
        match *self {
            CompleteLifecycleActionError::ResourceContentionFault(ref cause) => cause,
            CompleteLifecycleActionError::Validation(ref cause) => cause,
            CompleteLifecycleActionError::Credentials(ref err) => err.description(),
            CompleteLifecycleActionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CompleteLifecycleActionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateAutoScalingGroup
#[derive(Debug, PartialEq)]
pub enum CreateAutoScalingGroupError {
    /// <p>You already have an Auto Scaling group or launch configuration with this name.</p>
    AlreadyExistsFault(String),
    /// <p>You have already reached a limit for your Auto Scaling resources (for example, groups, launch configurations, or lifecycle hooks). For more information, see <a>DescribeAccountLimits</a>.</p>
    LimitExceededFault(String),
    /// <p>You already have a pending update to an Auto Scaling resource (for example, a group, instance, or load balancer).</p>
    ResourceContentionFault(String),
    /// <p>The service-linked role is not yet ready for use.</p>
    ServiceLinkedRoleFailure(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateAutoScalingGroupError {
    pub fn from_body(body: &str) -> CreateAutoScalingGroupError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "AlreadyExists" => CreateAutoScalingGroupError::AlreadyExistsFault(String::from(
                    parsed_error.message,
                )),
                "LimitExceeded" => CreateAutoScalingGroupError::LimitExceededFault(String::from(
                    parsed_error.message,
                )),
                "ResourceContention" => CreateAutoScalingGroupError::ResourceContentionFault(
                    String::from(parsed_error.message),
                ),
                "ServiceLinkedRoleFailure" => {
                    CreateAutoScalingGroupError::ServiceLinkedRoleFailure(String::from(
                        parsed_error.message,
                    ))
                }
                _ => CreateAutoScalingGroupError::Unknown(String::from(body)),
            },
            Err(_) => CreateAutoScalingGroupError::Unknown(body.to_string()),
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

impl From<XmlParseError> for CreateAutoScalingGroupError {
    fn from(err: XmlParseError) -> CreateAutoScalingGroupError {
        let XmlParseError(message) = err;
        CreateAutoScalingGroupError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for CreateAutoScalingGroupError {
    fn from(err: CredentialsError) -> CreateAutoScalingGroupError {
        CreateAutoScalingGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateAutoScalingGroupError {
    fn from(err: HttpDispatchError) -> CreateAutoScalingGroupError {
        CreateAutoScalingGroupError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateAutoScalingGroupError {
    fn from(err: io::Error) -> CreateAutoScalingGroupError {
        CreateAutoScalingGroupError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateAutoScalingGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateAutoScalingGroupError {
    fn description(&self) -> &str {
        match *self {
            CreateAutoScalingGroupError::AlreadyExistsFault(ref cause) => cause,
            CreateAutoScalingGroupError::LimitExceededFault(ref cause) => cause,
            CreateAutoScalingGroupError::ResourceContentionFault(ref cause) => cause,
            CreateAutoScalingGroupError::ServiceLinkedRoleFailure(ref cause) => cause,
            CreateAutoScalingGroupError::Validation(ref cause) => cause,
            CreateAutoScalingGroupError::Credentials(ref err) => err.description(),
            CreateAutoScalingGroupError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateAutoScalingGroupError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateLaunchConfiguration
#[derive(Debug, PartialEq)]
pub enum CreateLaunchConfigurationError {
    /// <p>You already have an Auto Scaling group or launch configuration with this name.</p>
    AlreadyExistsFault(String),
    /// <p>You have already reached a limit for your Auto Scaling resources (for example, groups, launch configurations, or lifecycle hooks). For more information, see <a>DescribeAccountLimits</a>.</p>
    LimitExceededFault(String),
    /// <p>You already have a pending update to an Auto Scaling resource (for example, a group, instance, or load balancer).</p>
    ResourceContentionFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateLaunchConfigurationError {
    pub fn from_body(body: &str) -> CreateLaunchConfigurationError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "AlreadyExists" => CreateLaunchConfigurationError::AlreadyExistsFault(
                    String::from(parsed_error.message),
                ),
                "LimitExceeded" => CreateLaunchConfigurationError::LimitExceededFault(
                    String::from(parsed_error.message),
                ),
                "ResourceContention" => CreateLaunchConfigurationError::ResourceContentionFault(
                    String::from(parsed_error.message),
                ),
                _ => CreateLaunchConfigurationError::Unknown(String::from(body)),
            },
            Err(_) => CreateLaunchConfigurationError::Unknown(body.to_string()),
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

impl From<XmlParseError> for CreateLaunchConfigurationError {
    fn from(err: XmlParseError) -> CreateLaunchConfigurationError {
        let XmlParseError(message) = err;
        CreateLaunchConfigurationError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for CreateLaunchConfigurationError {
    fn from(err: CredentialsError) -> CreateLaunchConfigurationError {
        CreateLaunchConfigurationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateLaunchConfigurationError {
    fn from(err: HttpDispatchError) -> CreateLaunchConfigurationError {
        CreateLaunchConfigurationError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateLaunchConfigurationError {
    fn from(err: io::Error) -> CreateLaunchConfigurationError {
        CreateLaunchConfigurationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateLaunchConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateLaunchConfigurationError {
    fn description(&self) -> &str {
        match *self {
            CreateLaunchConfigurationError::AlreadyExistsFault(ref cause) => cause,
            CreateLaunchConfigurationError::LimitExceededFault(ref cause) => cause,
            CreateLaunchConfigurationError::ResourceContentionFault(ref cause) => cause,
            CreateLaunchConfigurationError::Validation(ref cause) => cause,
            CreateLaunchConfigurationError::Credentials(ref err) => err.description(),
            CreateLaunchConfigurationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateLaunchConfigurationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateOrUpdateTags
#[derive(Debug, PartialEq)]
pub enum CreateOrUpdateTagsError {
    /// <p>You already have an Auto Scaling group or launch configuration with this name.</p>
    AlreadyExistsFault(String),
    /// <p>You have already reached a limit for your Auto Scaling resources (for example, groups, launch configurations, or lifecycle hooks). For more information, see <a>DescribeAccountLimits</a>.</p>
    LimitExceededFault(String),
    /// <p>You already have a pending update to an Auto Scaling resource (for example, a group, instance, or load balancer).</p>
    ResourceContentionFault(String),
    /// <p>The operation can't be performed because the resource is in use.</p>
    ResourceInUseFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateOrUpdateTagsError {
    pub fn from_body(body: &str) -> CreateOrUpdateTagsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "AlreadyExists" => {
                    CreateOrUpdateTagsError::AlreadyExistsFault(String::from(parsed_error.message))
                }
                "LimitExceeded" => {
                    CreateOrUpdateTagsError::LimitExceededFault(String::from(parsed_error.message))
                }
                "ResourceContention" => CreateOrUpdateTagsError::ResourceContentionFault(
                    String::from(parsed_error.message),
                ),
                "ResourceInUse" => {
                    CreateOrUpdateTagsError::ResourceInUseFault(String::from(parsed_error.message))
                }
                _ => CreateOrUpdateTagsError::Unknown(String::from(body)),
            },
            Err(_) => CreateOrUpdateTagsError::Unknown(body.to_string()),
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

impl From<XmlParseError> for CreateOrUpdateTagsError {
    fn from(err: XmlParseError) -> CreateOrUpdateTagsError {
        let XmlParseError(message) = err;
        CreateOrUpdateTagsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for CreateOrUpdateTagsError {
    fn from(err: CredentialsError) -> CreateOrUpdateTagsError {
        CreateOrUpdateTagsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateOrUpdateTagsError {
    fn from(err: HttpDispatchError) -> CreateOrUpdateTagsError {
        CreateOrUpdateTagsError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateOrUpdateTagsError {
    fn from(err: io::Error) -> CreateOrUpdateTagsError {
        CreateOrUpdateTagsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateOrUpdateTagsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateOrUpdateTagsError {
    fn description(&self) -> &str {
        match *self {
            CreateOrUpdateTagsError::AlreadyExistsFault(ref cause) => cause,
            CreateOrUpdateTagsError::LimitExceededFault(ref cause) => cause,
            CreateOrUpdateTagsError::ResourceContentionFault(ref cause) => cause,
            CreateOrUpdateTagsError::ResourceInUseFault(ref cause) => cause,
            CreateOrUpdateTagsError::Validation(ref cause) => cause,
            CreateOrUpdateTagsError::Credentials(ref err) => err.description(),
            CreateOrUpdateTagsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateOrUpdateTagsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteAutoScalingGroup
#[derive(Debug, PartialEq)]
pub enum DeleteAutoScalingGroupError {
    /// <p>You already have a pending update to an Auto Scaling resource (for example, a group, instance, or load balancer).</p>
    ResourceContentionFault(String),
    /// <p>The operation can't be performed because the resource is in use.</p>
    ResourceInUseFault(String),
    /// <p>The operation can't be performed because there are scaling activities in progress.</p>
    ScalingActivityInProgressFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteAutoScalingGroupError {
    pub fn from_body(body: &str) -> DeleteAutoScalingGroupError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ResourceContention" => DeleteAutoScalingGroupError::ResourceContentionFault(
                    String::from(parsed_error.message),
                ),
                "ResourceInUse" => DeleteAutoScalingGroupError::ResourceInUseFault(String::from(
                    parsed_error.message,
                )),
                "ScalingActivityInProgress" => {
                    DeleteAutoScalingGroupError::ScalingActivityInProgressFault(String::from(
                        parsed_error.message,
                    ))
                }
                _ => DeleteAutoScalingGroupError::Unknown(String::from(body)),
            },
            Err(_) => DeleteAutoScalingGroupError::Unknown(body.to_string()),
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

impl From<XmlParseError> for DeleteAutoScalingGroupError {
    fn from(err: XmlParseError) -> DeleteAutoScalingGroupError {
        let XmlParseError(message) = err;
        DeleteAutoScalingGroupError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeleteAutoScalingGroupError {
    fn from(err: CredentialsError) -> DeleteAutoScalingGroupError {
        DeleteAutoScalingGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteAutoScalingGroupError {
    fn from(err: HttpDispatchError) -> DeleteAutoScalingGroupError {
        DeleteAutoScalingGroupError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteAutoScalingGroupError {
    fn from(err: io::Error) -> DeleteAutoScalingGroupError {
        DeleteAutoScalingGroupError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteAutoScalingGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteAutoScalingGroupError {
    fn description(&self) -> &str {
        match *self {
            DeleteAutoScalingGroupError::ResourceContentionFault(ref cause) => cause,
            DeleteAutoScalingGroupError::ResourceInUseFault(ref cause) => cause,
            DeleteAutoScalingGroupError::ScalingActivityInProgressFault(ref cause) => cause,
            DeleteAutoScalingGroupError::Validation(ref cause) => cause,
            DeleteAutoScalingGroupError::Credentials(ref err) => err.description(),
            DeleteAutoScalingGroupError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteAutoScalingGroupError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteLaunchConfiguration
#[derive(Debug, PartialEq)]
pub enum DeleteLaunchConfigurationError {
    /// <p>You already have a pending update to an Auto Scaling resource (for example, a group, instance, or load balancer).</p>
    ResourceContentionFault(String),
    /// <p>The operation can't be performed because the resource is in use.</p>
    ResourceInUseFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteLaunchConfigurationError {
    pub fn from_body(body: &str) -> DeleteLaunchConfigurationError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ResourceContention" => DeleteLaunchConfigurationError::ResourceContentionFault(
                    String::from(parsed_error.message),
                ),
                "ResourceInUse" => DeleteLaunchConfigurationError::ResourceInUseFault(
                    String::from(parsed_error.message),
                ),
                _ => DeleteLaunchConfigurationError::Unknown(String::from(body)),
            },
            Err(_) => DeleteLaunchConfigurationError::Unknown(body.to_string()),
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

impl From<XmlParseError> for DeleteLaunchConfigurationError {
    fn from(err: XmlParseError) -> DeleteLaunchConfigurationError {
        let XmlParseError(message) = err;
        DeleteLaunchConfigurationError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeleteLaunchConfigurationError {
    fn from(err: CredentialsError) -> DeleteLaunchConfigurationError {
        DeleteLaunchConfigurationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteLaunchConfigurationError {
    fn from(err: HttpDispatchError) -> DeleteLaunchConfigurationError {
        DeleteLaunchConfigurationError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteLaunchConfigurationError {
    fn from(err: io::Error) -> DeleteLaunchConfigurationError {
        DeleteLaunchConfigurationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteLaunchConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteLaunchConfigurationError {
    fn description(&self) -> &str {
        match *self {
            DeleteLaunchConfigurationError::ResourceContentionFault(ref cause) => cause,
            DeleteLaunchConfigurationError::ResourceInUseFault(ref cause) => cause,
            DeleteLaunchConfigurationError::Validation(ref cause) => cause,
            DeleteLaunchConfigurationError::Credentials(ref err) => err.description(),
            DeleteLaunchConfigurationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteLaunchConfigurationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteLifecycleHook
#[derive(Debug, PartialEq)]
pub enum DeleteLifecycleHookError {
    /// <p>You already have a pending update to an Auto Scaling resource (for example, a group, instance, or load balancer).</p>
    ResourceContentionFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteLifecycleHookError {
    pub fn from_body(body: &str) -> DeleteLifecycleHookError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ResourceContention" => DeleteLifecycleHookError::ResourceContentionFault(
                    String::from(parsed_error.message),
                ),
                _ => DeleteLifecycleHookError::Unknown(String::from(body)),
            },
            Err(_) => DeleteLifecycleHookError::Unknown(body.to_string()),
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

impl From<XmlParseError> for DeleteLifecycleHookError {
    fn from(err: XmlParseError) -> DeleteLifecycleHookError {
        let XmlParseError(message) = err;
        DeleteLifecycleHookError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeleteLifecycleHookError {
    fn from(err: CredentialsError) -> DeleteLifecycleHookError {
        DeleteLifecycleHookError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteLifecycleHookError {
    fn from(err: HttpDispatchError) -> DeleteLifecycleHookError {
        DeleteLifecycleHookError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteLifecycleHookError {
    fn from(err: io::Error) -> DeleteLifecycleHookError {
        DeleteLifecycleHookError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteLifecycleHookError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteLifecycleHookError {
    fn description(&self) -> &str {
        match *self {
            DeleteLifecycleHookError::ResourceContentionFault(ref cause) => cause,
            DeleteLifecycleHookError::Validation(ref cause) => cause,
            DeleteLifecycleHookError::Credentials(ref err) => err.description(),
            DeleteLifecycleHookError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteLifecycleHookError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteNotificationConfiguration
#[derive(Debug, PartialEq)]
pub enum DeleteNotificationConfigurationError {
    /// <p>You already have a pending update to an Auto Scaling resource (for example, a group, instance, or load balancer).</p>
    ResourceContentionFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteNotificationConfigurationError {
    pub fn from_body(body: &str) -> DeleteNotificationConfigurationError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ResourceContention" => {
                    DeleteNotificationConfigurationError::ResourceContentionFault(String::from(
                        parsed_error.message,
                    ))
                }
                _ => DeleteNotificationConfigurationError::Unknown(String::from(body)),
            },
            Err(_) => DeleteNotificationConfigurationError::Unknown(body.to_string()),
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

impl From<XmlParseError> for DeleteNotificationConfigurationError {
    fn from(err: XmlParseError) -> DeleteNotificationConfigurationError {
        let XmlParseError(message) = err;
        DeleteNotificationConfigurationError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeleteNotificationConfigurationError {
    fn from(err: CredentialsError) -> DeleteNotificationConfigurationError {
        DeleteNotificationConfigurationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteNotificationConfigurationError {
    fn from(err: HttpDispatchError) -> DeleteNotificationConfigurationError {
        DeleteNotificationConfigurationError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteNotificationConfigurationError {
    fn from(err: io::Error) -> DeleteNotificationConfigurationError {
        DeleteNotificationConfigurationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteNotificationConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteNotificationConfigurationError {
    fn description(&self) -> &str {
        match *self {
            DeleteNotificationConfigurationError::ResourceContentionFault(ref cause) => cause,
            DeleteNotificationConfigurationError::Validation(ref cause) => cause,
            DeleteNotificationConfigurationError::Credentials(ref err) => err.description(),
            DeleteNotificationConfigurationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteNotificationConfigurationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeletePolicy
#[derive(Debug, PartialEq)]
pub enum DeletePolicyError {
    /// <p>You already have a pending update to an Auto Scaling resource (for example, a group, instance, or load balancer).</p>
    ResourceContentionFault(String),
    /// <p>The service-linked role is not yet ready for use.</p>
    ServiceLinkedRoleFailure(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeletePolicyError {
    pub fn from_body(body: &str) -> DeletePolicyError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ResourceContention" => {
                    DeletePolicyError::ResourceContentionFault(String::from(parsed_error.message))
                }
                "ServiceLinkedRoleFailure" => {
                    DeletePolicyError::ServiceLinkedRoleFailure(String::from(parsed_error.message))
                }
                _ => DeletePolicyError::Unknown(String::from(body)),
            },
            Err(_) => DeletePolicyError::Unknown(body.to_string()),
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

impl From<XmlParseError> for DeletePolicyError {
    fn from(err: XmlParseError) -> DeletePolicyError {
        let XmlParseError(message) = err;
        DeletePolicyError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeletePolicyError {
    fn from(err: CredentialsError) -> DeletePolicyError {
        DeletePolicyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeletePolicyError {
    fn from(err: HttpDispatchError) -> DeletePolicyError {
        DeletePolicyError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeletePolicyError {
    fn from(err: io::Error) -> DeletePolicyError {
        DeletePolicyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeletePolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeletePolicyError {
    fn description(&self) -> &str {
        match *self {
            DeletePolicyError::ResourceContentionFault(ref cause) => cause,
            DeletePolicyError::ServiceLinkedRoleFailure(ref cause) => cause,
            DeletePolicyError::Validation(ref cause) => cause,
            DeletePolicyError::Credentials(ref err) => err.description(),
            DeletePolicyError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeletePolicyError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteScheduledAction
#[derive(Debug, PartialEq)]
pub enum DeleteScheduledActionError {
    /// <p>You already have a pending update to an Auto Scaling resource (for example, a group, instance, or load balancer).</p>
    ResourceContentionFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteScheduledActionError {
    pub fn from_body(body: &str) -> DeleteScheduledActionError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ResourceContention" => DeleteScheduledActionError::ResourceContentionFault(
                    String::from(parsed_error.message),
                ),
                _ => DeleteScheduledActionError::Unknown(String::from(body)),
            },
            Err(_) => DeleteScheduledActionError::Unknown(body.to_string()),
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

impl From<XmlParseError> for DeleteScheduledActionError {
    fn from(err: XmlParseError) -> DeleteScheduledActionError {
        let XmlParseError(message) = err;
        DeleteScheduledActionError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeleteScheduledActionError {
    fn from(err: CredentialsError) -> DeleteScheduledActionError {
        DeleteScheduledActionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteScheduledActionError {
    fn from(err: HttpDispatchError) -> DeleteScheduledActionError {
        DeleteScheduledActionError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteScheduledActionError {
    fn from(err: io::Error) -> DeleteScheduledActionError {
        DeleteScheduledActionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteScheduledActionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteScheduledActionError {
    fn description(&self) -> &str {
        match *self {
            DeleteScheduledActionError::ResourceContentionFault(ref cause) => cause,
            DeleteScheduledActionError::Validation(ref cause) => cause,
            DeleteScheduledActionError::Credentials(ref err) => err.description(),
            DeleteScheduledActionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteScheduledActionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteTags
#[derive(Debug, PartialEq)]
pub enum DeleteTagsError {
    /// <p>You already have a pending update to an Auto Scaling resource (for example, a group, instance, or load balancer).</p>
    ResourceContentionFault(String),
    /// <p>The operation can't be performed because the resource is in use.</p>
    ResourceInUseFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteTagsError {
    pub fn from_body(body: &str) -> DeleteTagsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ResourceContention" => {
                    DeleteTagsError::ResourceContentionFault(String::from(parsed_error.message))
                }
                "ResourceInUse" => {
                    DeleteTagsError::ResourceInUseFault(String::from(parsed_error.message))
                }
                _ => DeleteTagsError::Unknown(String::from(body)),
            },
            Err(_) => DeleteTagsError::Unknown(body.to_string()),
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

impl From<XmlParseError> for DeleteTagsError {
    fn from(err: XmlParseError) -> DeleteTagsError {
        let XmlParseError(message) = err;
        DeleteTagsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeleteTagsError {
    fn from(err: CredentialsError) -> DeleteTagsError {
        DeleteTagsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteTagsError {
    fn from(err: HttpDispatchError) -> DeleteTagsError {
        DeleteTagsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteTagsError {
    fn from(err: io::Error) -> DeleteTagsError {
        DeleteTagsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteTagsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteTagsError {
    fn description(&self) -> &str {
        match *self {
            DeleteTagsError::ResourceContentionFault(ref cause) => cause,
            DeleteTagsError::ResourceInUseFault(ref cause) => cause,
            DeleteTagsError::Validation(ref cause) => cause,
            DeleteTagsError::Credentials(ref err) => err.description(),
            DeleteTagsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteTagsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeAccountLimits
#[derive(Debug, PartialEq)]
pub enum DescribeAccountLimitsError {
    /// <p>You already have a pending update to an Auto Scaling resource (for example, a group, instance, or load balancer).</p>
    ResourceContentionFault(String),
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
                "ResourceContention" => DescribeAccountLimitsError::ResourceContentionFault(
                    String::from(parsed_error.message),
                ),
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
            DescribeAccountLimitsError::ResourceContentionFault(ref cause) => cause,
            DescribeAccountLimitsError::Validation(ref cause) => cause,
            DescribeAccountLimitsError::Credentials(ref err) => err.description(),
            DescribeAccountLimitsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeAccountLimitsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeAdjustmentTypes
#[derive(Debug, PartialEq)]
pub enum DescribeAdjustmentTypesError {
    /// <p>You already have a pending update to an Auto Scaling resource (for example, a group, instance, or load balancer).</p>
    ResourceContentionFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeAdjustmentTypesError {
    pub fn from_body(body: &str) -> DescribeAdjustmentTypesError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ResourceContention" => DescribeAdjustmentTypesError::ResourceContentionFault(
                    String::from(parsed_error.message),
                ),
                _ => DescribeAdjustmentTypesError::Unknown(String::from(body)),
            },
            Err(_) => DescribeAdjustmentTypesError::Unknown(body.to_string()),
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

impl From<XmlParseError> for DescribeAdjustmentTypesError {
    fn from(err: XmlParseError) -> DescribeAdjustmentTypesError {
        let XmlParseError(message) = err;
        DescribeAdjustmentTypesError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DescribeAdjustmentTypesError {
    fn from(err: CredentialsError) -> DescribeAdjustmentTypesError {
        DescribeAdjustmentTypesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeAdjustmentTypesError {
    fn from(err: HttpDispatchError) -> DescribeAdjustmentTypesError {
        DescribeAdjustmentTypesError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeAdjustmentTypesError {
    fn from(err: io::Error) -> DescribeAdjustmentTypesError {
        DescribeAdjustmentTypesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeAdjustmentTypesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeAdjustmentTypesError {
    fn description(&self) -> &str {
        match *self {
            DescribeAdjustmentTypesError::ResourceContentionFault(ref cause) => cause,
            DescribeAdjustmentTypesError::Validation(ref cause) => cause,
            DescribeAdjustmentTypesError::Credentials(ref err) => err.description(),
            DescribeAdjustmentTypesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeAdjustmentTypesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeAutoScalingGroups
#[derive(Debug, PartialEq)]
pub enum DescribeAutoScalingGroupsError {
    /// <p>The <code>NextToken</code> value is not valid.</p>
    InvalidNextToken(String),
    /// <p>You already have a pending update to an Auto Scaling resource (for example, a group, instance, or load balancer).</p>
    ResourceContentionFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeAutoScalingGroupsError {
    pub fn from_body(body: &str) -> DescribeAutoScalingGroupsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "InvalidNextToken" => DescribeAutoScalingGroupsError::InvalidNextToken(
                    String::from(parsed_error.message),
                ),
                "ResourceContention" => DescribeAutoScalingGroupsError::ResourceContentionFault(
                    String::from(parsed_error.message),
                ),
                _ => DescribeAutoScalingGroupsError::Unknown(String::from(body)),
            },
            Err(_) => DescribeAutoScalingGroupsError::Unknown(body.to_string()),
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

impl From<XmlParseError> for DescribeAutoScalingGroupsError {
    fn from(err: XmlParseError) -> DescribeAutoScalingGroupsError {
        let XmlParseError(message) = err;
        DescribeAutoScalingGroupsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DescribeAutoScalingGroupsError {
    fn from(err: CredentialsError) -> DescribeAutoScalingGroupsError {
        DescribeAutoScalingGroupsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeAutoScalingGroupsError {
    fn from(err: HttpDispatchError) -> DescribeAutoScalingGroupsError {
        DescribeAutoScalingGroupsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeAutoScalingGroupsError {
    fn from(err: io::Error) -> DescribeAutoScalingGroupsError {
        DescribeAutoScalingGroupsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeAutoScalingGroupsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeAutoScalingGroupsError {
    fn description(&self) -> &str {
        match *self {
            DescribeAutoScalingGroupsError::InvalidNextToken(ref cause) => cause,
            DescribeAutoScalingGroupsError::ResourceContentionFault(ref cause) => cause,
            DescribeAutoScalingGroupsError::Validation(ref cause) => cause,
            DescribeAutoScalingGroupsError::Credentials(ref err) => err.description(),
            DescribeAutoScalingGroupsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeAutoScalingGroupsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeAutoScalingInstances
#[derive(Debug, PartialEq)]
pub enum DescribeAutoScalingInstancesError {
    /// <p>The <code>NextToken</code> value is not valid.</p>
    InvalidNextToken(String),
    /// <p>You already have a pending update to an Auto Scaling resource (for example, a group, instance, or load balancer).</p>
    ResourceContentionFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeAutoScalingInstancesError {
    pub fn from_body(body: &str) -> DescribeAutoScalingInstancesError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "InvalidNextToken" => DescribeAutoScalingInstancesError::InvalidNextToken(
                    String::from(parsed_error.message),
                ),
                "ResourceContention" => DescribeAutoScalingInstancesError::ResourceContentionFault(
                    String::from(parsed_error.message),
                ),
                _ => DescribeAutoScalingInstancesError::Unknown(String::from(body)),
            },
            Err(_) => DescribeAutoScalingInstancesError::Unknown(body.to_string()),
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

impl From<XmlParseError> for DescribeAutoScalingInstancesError {
    fn from(err: XmlParseError) -> DescribeAutoScalingInstancesError {
        let XmlParseError(message) = err;
        DescribeAutoScalingInstancesError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DescribeAutoScalingInstancesError {
    fn from(err: CredentialsError) -> DescribeAutoScalingInstancesError {
        DescribeAutoScalingInstancesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeAutoScalingInstancesError {
    fn from(err: HttpDispatchError) -> DescribeAutoScalingInstancesError {
        DescribeAutoScalingInstancesError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeAutoScalingInstancesError {
    fn from(err: io::Error) -> DescribeAutoScalingInstancesError {
        DescribeAutoScalingInstancesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeAutoScalingInstancesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeAutoScalingInstancesError {
    fn description(&self) -> &str {
        match *self {
            DescribeAutoScalingInstancesError::InvalidNextToken(ref cause) => cause,
            DescribeAutoScalingInstancesError::ResourceContentionFault(ref cause) => cause,
            DescribeAutoScalingInstancesError::Validation(ref cause) => cause,
            DescribeAutoScalingInstancesError::Credentials(ref err) => err.description(),
            DescribeAutoScalingInstancesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeAutoScalingInstancesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeAutoScalingNotificationTypes
#[derive(Debug, PartialEq)]
pub enum DescribeAutoScalingNotificationTypesError {
    /// <p>You already have a pending update to an Auto Scaling resource (for example, a group, instance, or load balancer).</p>
    ResourceContentionFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeAutoScalingNotificationTypesError {
    pub fn from_body(body: &str) -> DescribeAutoScalingNotificationTypesError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ResourceContention" => {
                    DescribeAutoScalingNotificationTypesError::ResourceContentionFault(
                        String::from(parsed_error.message),
                    )
                }
                _ => DescribeAutoScalingNotificationTypesError::Unknown(String::from(body)),
            },
            Err(_) => DescribeAutoScalingNotificationTypesError::Unknown(body.to_string()),
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

impl From<XmlParseError> for DescribeAutoScalingNotificationTypesError {
    fn from(err: XmlParseError) -> DescribeAutoScalingNotificationTypesError {
        let XmlParseError(message) = err;
        DescribeAutoScalingNotificationTypesError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DescribeAutoScalingNotificationTypesError {
    fn from(err: CredentialsError) -> DescribeAutoScalingNotificationTypesError {
        DescribeAutoScalingNotificationTypesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeAutoScalingNotificationTypesError {
    fn from(err: HttpDispatchError) -> DescribeAutoScalingNotificationTypesError {
        DescribeAutoScalingNotificationTypesError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeAutoScalingNotificationTypesError {
    fn from(err: io::Error) -> DescribeAutoScalingNotificationTypesError {
        DescribeAutoScalingNotificationTypesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeAutoScalingNotificationTypesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeAutoScalingNotificationTypesError {
    fn description(&self) -> &str {
        match *self {
            DescribeAutoScalingNotificationTypesError::ResourceContentionFault(ref cause) => cause,
            DescribeAutoScalingNotificationTypesError::Validation(ref cause) => cause,
            DescribeAutoScalingNotificationTypesError::Credentials(ref err) => err.description(),
            DescribeAutoScalingNotificationTypesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeAutoScalingNotificationTypesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeLaunchConfigurations
#[derive(Debug, PartialEq)]
pub enum DescribeLaunchConfigurationsError {
    /// <p>The <code>NextToken</code> value is not valid.</p>
    InvalidNextToken(String),
    /// <p>You already have a pending update to an Auto Scaling resource (for example, a group, instance, or load balancer).</p>
    ResourceContentionFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeLaunchConfigurationsError {
    pub fn from_body(body: &str) -> DescribeLaunchConfigurationsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "InvalidNextToken" => DescribeLaunchConfigurationsError::InvalidNextToken(
                    String::from(parsed_error.message),
                ),
                "ResourceContention" => DescribeLaunchConfigurationsError::ResourceContentionFault(
                    String::from(parsed_error.message),
                ),
                _ => DescribeLaunchConfigurationsError::Unknown(String::from(body)),
            },
            Err(_) => DescribeLaunchConfigurationsError::Unknown(body.to_string()),
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

impl From<XmlParseError> for DescribeLaunchConfigurationsError {
    fn from(err: XmlParseError) -> DescribeLaunchConfigurationsError {
        let XmlParseError(message) = err;
        DescribeLaunchConfigurationsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DescribeLaunchConfigurationsError {
    fn from(err: CredentialsError) -> DescribeLaunchConfigurationsError {
        DescribeLaunchConfigurationsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeLaunchConfigurationsError {
    fn from(err: HttpDispatchError) -> DescribeLaunchConfigurationsError {
        DescribeLaunchConfigurationsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeLaunchConfigurationsError {
    fn from(err: io::Error) -> DescribeLaunchConfigurationsError {
        DescribeLaunchConfigurationsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeLaunchConfigurationsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeLaunchConfigurationsError {
    fn description(&self) -> &str {
        match *self {
            DescribeLaunchConfigurationsError::InvalidNextToken(ref cause) => cause,
            DescribeLaunchConfigurationsError::ResourceContentionFault(ref cause) => cause,
            DescribeLaunchConfigurationsError::Validation(ref cause) => cause,
            DescribeLaunchConfigurationsError::Credentials(ref err) => err.description(),
            DescribeLaunchConfigurationsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeLaunchConfigurationsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeLifecycleHookTypes
#[derive(Debug, PartialEq)]
pub enum DescribeLifecycleHookTypesError {
    /// <p>You already have a pending update to an Auto Scaling resource (for example, a group, instance, or load balancer).</p>
    ResourceContentionFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeLifecycleHookTypesError {
    pub fn from_body(body: &str) -> DescribeLifecycleHookTypesError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ResourceContention" => DescribeLifecycleHookTypesError::ResourceContentionFault(
                    String::from(parsed_error.message),
                ),
                _ => DescribeLifecycleHookTypesError::Unknown(String::from(body)),
            },
            Err(_) => DescribeLifecycleHookTypesError::Unknown(body.to_string()),
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

impl From<XmlParseError> for DescribeLifecycleHookTypesError {
    fn from(err: XmlParseError) -> DescribeLifecycleHookTypesError {
        let XmlParseError(message) = err;
        DescribeLifecycleHookTypesError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DescribeLifecycleHookTypesError {
    fn from(err: CredentialsError) -> DescribeLifecycleHookTypesError {
        DescribeLifecycleHookTypesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeLifecycleHookTypesError {
    fn from(err: HttpDispatchError) -> DescribeLifecycleHookTypesError {
        DescribeLifecycleHookTypesError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeLifecycleHookTypesError {
    fn from(err: io::Error) -> DescribeLifecycleHookTypesError {
        DescribeLifecycleHookTypesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeLifecycleHookTypesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeLifecycleHookTypesError {
    fn description(&self) -> &str {
        match *self {
            DescribeLifecycleHookTypesError::ResourceContentionFault(ref cause) => cause,
            DescribeLifecycleHookTypesError::Validation(ref cause) => cause,
            DescribeLifecycleHookTypesError::Credentials(ref err) => err.description(),
            DescribeLifecycleHookTypesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeLifecycleHookTypesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeLifecycleHooks
#[derive(Debug, PartialEq)]
pub enum DescribeLifecycleHooksError {
    /// <p>You already have a pending update to an Auto Scaling resource (for example, a group, instance, or load balancer).</p>
    ResourceContentionFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeLifecycleHooksError {
    pub fn from_body(body: &str) -> DescribeLifecycleHooksError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ResourceContention" => DescribeLifecycleHooksError::ResourceContentionFault(
                    String::from(parsed_error.message),
                ),
                _ => DescribeLifecycleHooksError::Unknown(String::from(body)),
            },
            Err(_) => DescribeLifecycleHooksError::Unknown(body.to_string()),
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

impl From<XmlParseError> for DescribeLifecycleHooksError {
    fn from(err: XmlParseError) -> DescribeLifecycleHooksError {
        let XmlParseError(message) = err;
        DescribeLifecycleHooksError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DescribeLifecycleHooksError {
    fn from(err: CredentialsError) -> DescribeLifecycleHooksError {
        DescribeLifecycleHooksError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeLifecycleHooksError {
    fn from(err: HttpDispatchError) -> DescribeLifecycleHooksError {
        DescribeLifecycleHooksError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeLifecycleHooksError {
    fn from(err: io::Error) -> DescribeLifecycleHooksError {
        DescribeLifecycleHooksError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeLifecycleHooksError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeLifecycleHooksError {
    fn description(&self) -> &str {
        match *self {
            DescribeLifecycleHooksError::ResourceContentionFault(ref cause) => cause,
            DescribeLifecycleHooksError::Validation(ref cause) => cause,
            DescribeLifecycleHooksError::Credentials(ref err) => err.description(),
            DescribeLifecycleHooksError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeLifecycleHooksError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeLoadBalancerTargetGroups
#[derive(Debug, PartialEq)]
pub enum DescribeLoadBalancerTargetGroupsError {
    /// <p>You already have a pending update to an Auto Scaling resource (for example, a group, instance, or load balancer).</p>
    ResourceContentionFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeLoadBalancerTargetGroupsError {
    pub fn from_body(body: &str) -> DescribeLoadBalancerTargetGroupsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ResourceContention" => {
                    DescribeLoadBalancerTargetGroupsError::ResourceContentionFault(String::from(
                        parsed_error.message,
                    ))
                }
                _ => DescribeLoadBalancerTargetGroupsError::Unknown(String::from(body)),
            },
            Err(_) => DescribeLoadBalancerTargetGroupsError::Unknown(body.to_string()),
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

impl From<XmlParseError> for DescribeLoadBalancerTargetGroupsError {
    fn from(err: XmlParseError) -> DescribeLoadBalancerTargetGroupsError {
        let XmlParseError(message) = err;
        DescribeLoadBalancerTargetGroupsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DescribeLoadBalancerTargetGroupsError {
    fn from(err: CredentialsError) -> DescribeLoadBalancerTargetGroupsError {
        DescribeLoadBalancerTargetGroupsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeLoadBalancerTargetGroupsError {
    fn from(err: HttpDispatchError) -> DescribeLoadBalancerTargetGroupsError {
        DescribeLoadBalancerTargetGroupsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeLoadBalancerTargetGroupsError {
    fn from(err: io::Error) -> DescribeLoadBalancerTargetGroupsError {
        DescribeLoadBalancerTargetGroupsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeLoadBalancerTargetGroupsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeLoadBalancerTargetGroupsError {
    fn description(&self) -> &str {
        match *self {
            DescribeLoadBalancerTargetGroupsError::ResourceContentionFault(ref cause) => cause,
            DescribeLoadBalancerTargetGroupsError::Validation(ref cause) => cause,
            DescribeLoadBalancerTargetGroupsError::Credentials(ref err) => err.description(),
            DescribeLoadBalancerTargetGroupsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeLoadBalancerTargetGroupsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeLoadBalancers
#[derive(Debug, PartialEq)]
pub enum DescribeLoadBalancersError {
    /// <p>You already have a pending update to an Auto Scaling resource (for example, a group, instance, or load balancer).</p>
    ResourceContentionFault(String),
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
                "ResourceContention" => DescribeLoadBalancersError::ResourceContentionFault(
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
            DescribeLoadBalancersError::ResourceContentionFault(ref cause) => cause,
            DescribeLoadBalancersError::Validation(ref cause) => cause,
            DescribeLoadBalancersError::Credentials(ref err) => err.description(),
            DescribeLoadBalancersError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeLoadBalancersError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeMetricCollectionTypes
#[derive(Debug, PartialEq)]
pub enum DescribeMetricCollectionTypesError {
    /// <p>You already have a pending update to an Auto Scaling resource (for example, a group, instance, or load balancer).</p>
    ResourceContentionFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeMetricCollectionTypesError {
    pub fn from_body(body: &str) -> DescribeMetricCollectionTypesError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ResourceContention" => {
                    DescribeMetricCollectionTypesError::ResourceContentionFault(String::from(
                        parsed_error.message,
                    ))
                }
                _ => DescribeMetricCollectionTypesError::Unknown(String::from(body)),
            },
            Err(_) => DescribeMetricCollectionTypesError::Unknown(body.to_string()),
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

impl From<XmlParseError> for DescribeMetricCollectionTypesError {
    fn from(err: XmlParseError) -> DescribeMetricCollectionTypesError {
        let XmlParseError(message) = err;
        DescribeMetricCollectionTypesError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DescribeMetricCollectionTypesError {
    fn from(err: CredentialsError) -> DescribeMetricCollectionTypesError {
        DescribeMetricCollectionTypesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeMetricCollectionTypesError {
    fn from(err: HttpDispatchError) -> DescribeMetricCollectionTypesError {
        DescribeMetricCollectionTypesError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeMetricCollectionTypesError {
    fn from(err: io::Error) -> DescribeMetricCollectionTypesError {
        DescribeMetricCollectionTypesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeMetricCollectionTypesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeMetricCollectionTypesError {
    fn description(&self) -> &str {
        match *self {
            DescribeMetricCollectionTypesError::ResourceContentionFault(ref cause) => cause,
            DescribeMetricCollectionTypesError::Validation(ref cause) => cause,
            DescribeMetricCollectionTypesError::Credentials(ref err) => err.description(),
            DescribeMetricCollectionTypesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeMetricCollectionTypesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeNotificationConfigurations
#[derive(Debug, PartialEq)]
pub enum DescribeNotificationConfigurationsError {
    /// <p>The <code>NextToken</code> value is not valid.</p>
    InvalidNextToken(String),
    /// <p>You already have a pending update to an Auto Scaling resource (for example, a group, instance, or load balancer).</p>
    ResourceContentionFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeNotificationConfigurationsError {
    pub fn from_body(body: &str) -> DescribeNotificationConfigurationsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "InvalidNextToken" => DescribeNotificationConfigurationsError::InvalidNextToken(
                    String::from(parsed_error.message),
                ),
                "ResourceContention" => {
                    DescribeNotificationConfigurationsError::ResourceContentionFault(String::from(
                        parsed_error.message,
                    ))
                }
                _ => DescribeNotificationConfigurationsError::Unknown(String::from(body)),
            },
            Err(_) => DescribeNotificationConfigurationsError::Unknown(body.to_string()),
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

impl From<XmlParseError> for DescribeNotificationConfigurationsError {
    fn from(err: XmlParseError) -> DescribeNotificationConfigurationsError {
        let XmlParseError(message) = err;
        DescribeNotificationConfigurationsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DescribeNotificationConfigurationsError {
    fn from(err: CredentialsError) -> DescribeNotificationConfigurationsError {
        DescribeNotificationConfigurationsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeNotificationConfigurationsError {
    fn from(err: HttpDispatchError) -> DescribeNotificationConfigurationsError {
        DescribeNotificationConfigurationsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeNotificationConfigurationsError {
    fn from(err: io::Error) -> DescribeNotificationConfigurationsError {
        DescribeNotificationConfigurationsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeNotificationConfigurationsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeNotificationConfigurationsError {
    fn description(&self) -> &str {
        match *self {
            DescribeNotificationConfigurationsError::InvalidNextToken(ref cause) => cause,
            DescribeNotificationConfigurationsError::ResourceContentionFault(ref cause) => cause,
            DescribeNotificationConfigurationsError::Validation(ref cause) => cause,
            DescribeNotificationConfigurationsError::Credentials(ref err) => err.description(),
            DescribeNotificationConfigurationsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeNotificationConfigurationsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribePolicies
#[derive(Debug, PartialEq)]
pub enum DescribePoliciesError {
    /// <p>The <code>NextToken</code> value is not valid.</p>
    InvalidNextToken(String),
    /// <p>You already have a pending update to an Auto Scaling resource (for example, a group, instance, or load balancer).</p>
    ResourceContentionFault(String),
    /// <p>The service-linked role is not yet ready for use.</p>
    ServiceLinkedRoleFailure(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribePoliciesError {
    pub fn from_body(body: &str) -> DescribePoliciesError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "InvalidNextToken" => {
                    DescribePoliciesError::InvalidNextToken(String::from(parsed_error.message))
                }
                "ResourceContention" => DescribePoliciesError::ResourceContentionFault(
                    String::from(parsed_error.message),
                ),
                "ServiceLinkedRoleFailure" => DescribePoliciesError::ServiceLinkedRoleFailure(
                    String::from(parsed_error.message),
                ),
                _ => DescribePoliciesError::Unknown(String::from(body)),
            },
            Err(_) => DescribePoliciesError::Unknown(body.to_string()),
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

impl From<XmlParseError> for DescribePoliciesError {
    fn from(err: XmlParseError) -> DescribePoliciesError {
        let XmlParseError(message) = err;
        DescribePoliciesError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DescribePoliciesError {
    fn from(err: CredentialsError) -> DescribePoliciesError {
        DescribePoliciesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribePoliciesError {
    fn from(err: HttpDispatchError) -> DescribePoliciesError {
        DescribePoliciesError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribePoliciesError {
    fn from(err: io::Error) -> DescribePoliciesError {
        DescribePoliciesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribePoliciesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribePoliciesError {
    fn description(&self) -> &str {
        match *self {
            DescribePoliciesError::InvalidNextToken(ref cause) => cause,
            DescribePoliciesError::ResourceContentionFault(ref cause) => cause,
            DescribePoliciesError::ServiceLinkedRoleFailure(ref cause) => cause,
            DescribePoliciesError::Validation(ref cause) => cause,
            DescribePoliciesError::Credentials(ref err) => err.description(),
            DescribePoliciesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribePoliciesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeScalingActivities
#[derive(Debug, PartialEq)]
pub enum DescribeScalingActivitiesError {
    /// <p>The <code>NextToken</code> value is not valid.</p>
    InvalidNextToken(String),
    /// <p>You already have a pending update to an Auto Scaling resource (for example, a group, instance, or load balancer).</p>
    ResourceContentionFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeScalingActivitiesError {
    pub fn from_body(body: &str) -> DescribeScalingActivitiesError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "InvalidNextToken" => DescribeScalingActivitiesError::InvalidNextToken(
                    String::from(parsed_error.message),
                ),
                "ResourceContention" => DescribeScalingActivitiesError::ResourceContentionFault(
                    String::from(parsed_error.message),
                ),
                _ => DescribeScalingActivitiesError::Unknown(String::from(body)),
            },
            Err(_) => DescribeScalingActivitiesError::Unknown(body.to_string()),
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

impl From<XmlParseError> for DescribeScalingActivitiesError {
    fn from(err: XmlParseError) -> DescribeScalingActivitiesError {
        let XmlParseError(message) = err;
        DescribeScalingActivitiesError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DescribeScalingActivitiesError {
    fn from(err: CredentialsError) -> DescribeScalingActivitiesError {
        DescribeScalingActivitiesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeScalingActivitiesError {
    fn from(err: HttpDispatchError) -> DescribeScalingActivitiesError {
        DescribeScalingActivitiesError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeScalingActivitiesError {
    fn from(err: io::Error) -> DescribeScalingActivitiesError {
        DescribeScalingActivitiesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeScalingActivitiesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeScalingActivitiesError {
    fn description(&self) -> &str {
        match *self {
            DescribeScalingActivitiesError::InvalidNextToken(ref cause) => cause,
            DescribeScalingActivitiesError::ResourceContentionFault(ref cause) => cause,
            DescribeScalingActivitiesError::Validation(ref cause) => cause,
            DescribeScalingActivitiesError::Credentials(ref err) => err.description(),
            DescribeScalingActivitiesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeScalingActivitiesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeScalingProcessTypes
#[derive(Debug, PartialEq)]
pub enum DescribeScalingProcessTypesError {
    /// <p>You already have a pending update to an Auto Scaling resource (for example, a group, instance, or load balancer).</p>
    ResourceContentionFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeScalingProcessTypesError {
    pub fn from_body(body: &str) -> DescribeScalingProcessTypesError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ResourceContention" => DescribeScalingProcessTypesError::ResourceContentionFault(
                    String::from(parsed_error.message),
                ),
                _ => DescribeScalingProcessTypesError::Unknown(String::from(body)),
            },
            Err(_) => DescribeScalingProcessTypesError::Unknown(body.to_string()),
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

impl From<XmlParseError> for DescribeScalingProcessTypesError {
    fn from(err: XmlParseError) -> DescribeScalingProcessTypesError {
        let XmlParseError(message) = err;
        DescribeScalingProcessTypesError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DescribeScalingProcessTypesError {
    fn from(err: CredentialsError) -> DescribeScalingProcessTypesError {
        DescribeScalingProcessTypesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeScalingProcessTypesError {
    fn from(err: HttpDispatchError) -> DescribeScalingProcessTypesError {
        DescribeScalingProcessTypesError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeScalingProcessTypesError {
    fn from(err: io::Error) -> DescribeScalingProcessTypesError {
        DescribeScalingProcessTypesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeScalingProcessTypesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeScalingProcessTypesError {
    fn description(&self) -> &str {
        match *self {
            DescribeScalingProcessTypesError::ResourceContentionFault(ref cause) => cause,
            DescribeScalingProcessTypesError::Validation(ref cause) => cause,
            DescribeScalingProcessTypesError::Credentials(ref err) => err.description(),
            DescribeScalingProcessTypesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeScalingProcessTypesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeScheduledActions
#[derive(Debug, PartialEq)]
pub enum DescribeScheduledActionsError {
    /// <p>The <code>NextToken</code> value is not valid.</p>
    InvalidNextToken(String),
    /// <p>You already have a pending update to an Auto Scaling resource (for example, a group, instance, or load balancer).</p>
    ResourceContentionFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeScheduledActionsError {
    pub fn from_body(body: &str) -> DescribeScheduledActionsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "InvalidNextToken" => DescribeScheduledActionsError::InvalidNextToken(
                    String::from(parsed_error.message),
                ),
                "ResourceContention" => DescribeScheduledActionsError::ResourceContentionFault(
                    String::from(parsed_error.message),
                ),
                _ => DescribeScheduledActionsError::Unknown(String::from(body)),
            },
            Err(_) => DescribeScheduledActionsError::Unknown(body.to_string()),
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

impl From<XmlParseError> for DescribeScheduledActionsError {
    fn from(err: XmlParseError) -> DescribeScheduledActionsError {
        let XmlParseError(message) = err;
        DescribeScheduledActionsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DescribeScheduledActionsError {
    fn from(err: CredentialsError) -> DescribeScheduledActionsError {
        DescribeScheduledActionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeScheduledActionsError {
    fn from(err: HttpDispatchError) -> DescribeScheduledActionsError {
        DescribeScheduledActionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeScheduledActionsError {
    fn from(err: io::Error) -> DescribeScheduledActionsError {
        DescribeScheduledActionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeScheduledActionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeScheduledActionsError {
    fn description(&self) -> &str {
        match *self {
            DescribeScheduledActionsError::InvalidNextToken(ref cause) => cause,
            DescribeScheduledActionsError::ResourceContentionFault(ref cause) => cause,
            DescribeScheduledActionsError::Validation(ref cause) => cause,
            DescribeScheduledActionsError::Credentials(ref err) => err.description(),
            DescribeScheduledActionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeScheduledActionsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeTags
#[derive(Debug, PartialEq)]
pub enum DescribeTagsError {
    /// <p>The <code>NextToken</code> value is not valid.</p>
    InvalidNextToken(String),
    /// <p>You already have a pending update to an Auto Scaling resource (for example, a group, instance, or load balancer).</p>
    ResourceContentionFault(String),
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
                "InvalidNextToken" => {
                    DescribeTagsError::InvalidNextToken(String::from(parsed_error.message))
                }
                "ResourceContention" => {
                    DescribeTagsError::ResourceContentionFault(String::from(parsed_error.message))
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
            DescribeTagsError::InvalidNextToken(ref cause) => cause,
            DescribeTagsError::ResourceContentionFault(ref cause) => cause,
            DescribeTagsError::Validation(ref cause) => cause,
            DescribeTagsError::Credentials(ref err) => err.description(),
            DescribeTagsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeTagsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeTerminationPolicyTypes
#[derive(Debug, PartialEq)]
pub enum DescribeTerminationPolicyTypesError {
    /// <p>You already have a pending update to an Auto Scaling resource (for example, a group, instance, or load balancer).</p>
    ResourceContentionFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeTerminationPolicyTypesError {
    pub fn from_body(body: &str) -> DescribeTerminationPolicyTypesError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ResourceContention" => {
                    DescribeTerminationPolicyTypesError::ResourceContentionFault(String::from(
                        parsed_error.message,
                    ))
                }
                _ => DescribeTerminationPolicyTypesError::Unknown(String::from(body)),
            },
            Err(_) => DescribeTerminationPolicyTypesError::Unknown(body.to_string()),
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

impl From<XmlParseError> for DescribeTerminationPolicyTypesError {
    fn from(err: XmlParseError) -> DescribeTerminationPolicyTypesError {
        let XmlParseError(message) = err;
        DescribeTerminationPolicyTypesError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DescribeTerminationPolicyTypesError {
    fn from(err: CredentialsError) -> DescribeTerminationPolicyTypesError {
        DescribeTerminationPolicyTypesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeTerminationPolicyTypesError {
    fn from(err: HttpDispatchError) -> DescribeTerminationPolicyTypesError {
        DescribeTerminationPolicyTypesError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeTerminationPolicyTypesError {
    fn from(err: io::Error) -> DescribeTerminationPolicyTypesError {
        DescribeTerminationPolicyTypesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeTerminationPolicyTypesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeTerminationPolicyTypesError {
    fn description(&self) -> &str {
        match *self {
            DescribeTerminationPolicyTypesError::ResourceContentionFault(ref cause) => cause,
            DescribeTerminationPolicyTypesError::Validation(ref cause) => cause,
            DescribeTerminationPolicyTypesError::Credentials(ref err) => err.description(),
            DescribeTerminationPolicyTypesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeTerminationPolicyTypesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DetachInstances
#[derive(Debug, PartialEq)]
pub enum DetachInstancesError {
    /// <p>You already have a pending update to an Auto Scaling resource (for example, a group, instance, or load balancer).</p>
    ResourceContentionFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DetachInstancesError {
    pub fn from_body(body: &str) -> DetachInstancesError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ResourceContention" => DetachInstancesError::ResourceContentionFault(
                    String::from(parsed_error.message),
                ),
                _ => DetachInstancesError::Unknown(String::from(body)),
            },
            Err(_) => DetachInstancesError::Unknown(body.to_string()),
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

impl From<XmlParseError> for DetachInstancesError {
    fn from(err: XmlParseError) -> DetachInstancesError {
        let XmlParseError(message) = err;
        DetachInstancesError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DetachInstancesError {
    fn from(err: CredentialsError) -> DetachInstancesError {
        DetachInstancesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DetachInstancesError {
    fn from(err: HttpDispatchError) -> DetachInstancesError {
        DetachInstancesError::HttpDispatch(err)
    }
}
impl From<io::Error> for DetachInstancesError {
    fn from(err: io::Error) -> DetachInstancesError {
        DetachInstancesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DetachInstancesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DetachInstancesError {
    fn description(&self) -> &str {
        match *self {
            DetachInstancesError::ResourceContentionFault(ref cause) => cause,
            DetachInstancesError::Validation(ref cause) => cause,
            DetachInstancesError::Credentials(ref err) => err.description(),
            DetachInstancesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DetachInstancesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DetachLoadBalancerTargetGroups
#[derive(Debug, PartialEq)]
pub enum DetachLoadBalancerTargetGroupsError {
    /// <p>You already have a pending update to an Auto Scaling resource (for example, a group, instance, or load balancer).</p>
    ResourceContentionFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DetachLoadBalancerTargetGroupsError {
    pub fn from_body(body: &str) -> DetachLoadBalancerTargetGroupsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ResourceContention" => {
                    DetachLoadBalancerTargetGroupsError::ResourceContentionFault(String::from(
                        parsed_error.message,
                    ))
                }
                _ => DetachLoadBalancerTargetGroupsError::Unknown(String::from(body)),
            },
            Err(_) => DetachLoadBalancerTargetGroupsError::Unknown(body.to_string()),
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

impl From<XmlParseError> for DetachLoadBalancerTargetGroupsError {
    fn from(err: XmlParseError) -> DetachLoadBalancerTargetGroupsError {
        let XmlParseError(message) = err;
        DetachLoadBalancerTargetGroupsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DetachLoadBalancerTargetGroupsError {
    fn from(err: CredentialsError) -> DetachLoadBalancerTargetGroupsError {
        DetachLoadBalancerTargetGroupsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DetachLoadBalancerTargetGroupsError {
    fn from(err: HttpDispatchError) -> DetachLoadBalancerTargetGroupsError {
        DetachLoadBalancerTargetGroupsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DetachLoadBalancerTargetGroupsError {
    fn from(err: io::Error) -> DetachLoadBalancerTargetGroupsError {
        DetachLoadBalancerTargetGroupsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DetachLoadBalancerTargetGroupsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DetachLoadBalancerTargetGroupsError {
    fn description(&self) -> &str {
        match *self {
            DetachLoadBalancerTargetGroupsError::ResourceContentionFault(ref cause) => cause,
            DetachLoadBalancerTargetGroupsError::Validation(ref cause) => cause,
            DetachLoadBalancerTargetGroupsError::Credentials(ref err) => err.description(),
            DetachLoadBalancerTargetGroupsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DetachLoadBalancerTargetGroupsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DetachLoadBalancers
#[derive(Debug, PartialEq)]
pub enum DetachLoadBalancersError {
    /// <p>You already have a pending update to an Auto Scaling resource (for example, a group, instance, or load balancer).</p>
    ResourceContentionFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DetachLoadBalancersError {
    pub fn from_body(body: &str) -> DetachLoadBalancersError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ResourceContention" => DetachLoadBalancersError::ResourceContentionFault(
                    String::from(parsed_error.message),
                ),
                _ => DetachLoadBalancersError::Unknown(String::from(body)),
            },
            Err(_) => DetachLoadBalancersError::Unknown(body.to_string()),
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

impl From<XmlParseError> for DetachLoadBalancersError {
    fn from(err: XmlParseError) -> DetachLoadBalancersError {
        let XmlParseError(message) = err;
        DetachLoadBalancersError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DetachLoadBalancersError {
    fn from(err: CredentialsError) -> DetachLoadBalancersError {
        DetachLoadBalancersError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DetachLoadBalancersError {
    fn from(err: HttpDispatchError) -> DetachLoadBalancersError {
        DetachLoadBalancersError::HttpDispatch(err)
    }
}
impl From<io::Error> for DetachLoadBalancersError {
    fn from(err: io::Error) -> DetachLoadBalancersError {
        DetachLoadBalancersError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DetachLoadBalancersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DetachLoadBalancersError {
    fn description(&self) -> &str {
        match *self {
            DetachLoadBalancersError::ResourceContentionFault(ref cause) => cause,
            DetachLoadBalancersError::Validation(ref cause) => cause,
            DetachLoadBalancersError::Credentials(ref err) => err.description(),
            DetachLoadBalancersError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DetachLoadBalancersError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DisableMetricsCollection
#[derive(Debug, PartialEq)]
pub enum DisableMetricsCollectionError {
    /// <p>You already have a pending update to an Auto Scaling resource (for example, a group, instance, or load balancer).</p>
    ResourceContentionFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DisableMetricsCollectionError {
    pub fn from_body(body: &str) -> DisableMetricsCollectionError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ResourceContention" => DisableMetricsCollectionError::ResourceContentionFault(
                    String::from(parsed_error.message),
                ),
                _ => DisableMetricsCollectionError::Unknown(String::from(body)),
            },
            Err(_) => DisableMetricsCollectionError::Unknown(body.to_string()),
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

impl From<XmlParseError> for DisableMetricsCollectionError {
    fn from(err: XmlParseError) -> DisableMetricsCollectionError {
        let XmlParseError(message) = err;
        DisableMetricsCollectionError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DisableMetricsCollectionError {
    fn from(err: CredentialsError) -> DisableMetricsCollectionError {
        DisableMetricsCollectionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DisableMetricsCollectionError {
    fn from(err: HttpDispatchError) -> DisableMetricsCollectionError {
        DisableMetricsCollectionError::HttpDispatch(err)
    }
}
impl From<io::Error> for DisableMetricsCollectionError {
    fn from(err: io::Error) -> DisableMetricsCollectionError {
        DisableMetricsCollectionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DisableMetricsCollectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DisableMetricsCollectionError {
    fn description(&self) -> &str {
        match *self {
            DisableMetricsCollectionError::ResourceContentionFault(ref cause) => cause,
            DisableMetricsCollectionError::Validation(ref cause) => cause,
            DisableMetricsCollectionError::Credentials(ref err) => err.description(),
            DisableMetricsCollectionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DisableMetricsCollectionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by EnableMetricsCollection
#[derive(Debug, PartialEq)]
pub enum EnableMetricsCollectionError {
    /// <p>You already have a pending update to an Auto Scaling resource (for example, a group, instance, or load balancer).</p>
    ResourceContentionFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl EnableMetricsCollectionError {
    pub fn from_body(body: &str) -> EnableMetricsCollectionError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ResourceContention" => EnableMetricsCollectionError::ResourceContentionFault(
                    String::from(parsed_error.message),
                ),
                _ => EnableMetricsCollectionError::Unknown(String::from(body)),
            },
            Err(_) => EnableMetricsCollectionError::Unknown(body.to_string()),
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

impl From<XmlParseError> for EnableMetricsCollectionError {
    fn from(err: XmlParseError) -> EnableMetricsCollectionError {
        let XmlParseError(message) = err;
        EnableMetricsCollectionError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for EnableMetricsCollectionError {
    fn from(err: CredentialsError) -> EnableMetricsCollectionError {
        EnableMetricsCollectionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for EnableMetricsCollectionError {
    fn from(err: HttpDispatchError) -> EnableMetricsCollectionError {
        EnableMetricsCollectionError::HttpDispatch(err)
    }
}
impl From<io::Error> for EnableMetricsCollectionError {
    fn from(err: io::Error) -> EnableMetricsCollectionError {
        EnableMetricsCollectionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for EnableMetricsCollectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for EnableMetricsCollectionError {
    fn description(&self) -> &str {
        match *self {
            EnableMetricsCollectionError::ResourceContentionFault(ref cause) => cause,
            EnableMetricsCollectionError::Validation(ref cause) => cause,
            EnableMetricsCollectionError::Credentials(ref err) => err.description(),
            EnableMetricsCollectionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            EnableMetricsCollectionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by EnterStandby
#[derive(Debug, PartialEq)]
pub enum EnterStandbyError {
    /// <p>You already have a pending update to an Auto Scaling resource (for example, a group, instance, or load balancer).</p>
    ResourceContentionFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl EnterStandbyError {
    pub fn from_body(body: &str) -> EnterStandbyError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ResourceContention" => {
                    EnterStandbyError::ResourceContentionFault(String::from(parsed_error.message))
                }
                _ => EnterStandbyError::Unknown(String::from(body)),
            },
            Err(_) => EnterStandbyError::Unknown(body.to_string()),
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

impl From<XmlParseError> for EnterStandbyError {
    fn from(err: XmlParseError) -> EnterStandbyError {
        let XmlParseError(message) = err;
        EnterStandbyError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for EnterStandbyError {
    fn from(err: CredentialsError) -> EnterStandbyError {
        EnterStandbyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for EnterStandbyError {
    fn from(err: HttpDispatchError) -> EnterStandbyError {
        EnterStandbyError::HttpDispatch(err)
    }
}
impl From<io::Error> for EnterStandbyError {
    fn from(err: io::Error) -> EnterStandbyError {
        EnterStandbyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for EnterStandbyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for EnterStandbyError {
    fn description(&self) -> &str {
        match *self {
            EnterStandbyError::ResourceContentionFault(ref cause) => cause,
            EnterStandbyError::Validation(ref cause) => cause,
            EnterStandbyError::Credentials(ref err) => err.description(),
            EnterStandbyError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            EnterStandbyError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ExecutePolicy
#[derive(Debug, PartialEq)]
pub enum ExecutePolicyError {
    /// <p>You already have a pending update to an Auto Scaling resource (for example, a group, instance, or load balancer).</p>
    ResourceContentionFault(String),
    /// <p>The operation can't be performed because there are scaling activities in progress.</p>
    ScalingActivityInProgressFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ExecutePolicyError {
    pub fn from_body(body: &str) -> ExecutePolicyError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ResourceContention" => {
                    ExecutePolicyError::ResourceContentionFault(String::from(parsed_error.message))
                }
                "ScalingActivityInProgress" => ExecutePolicyError::ScalingActivityInProgressFault(
                    String::from(parsed_error.message),
                ),
                _ => ExecutePolicyError::Unknown(String::from(body)),
            },
            Err(_) => ExecutePolicyError::Unknown(body.to_string()),
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

impl From<XmlParseError> for ExecutePolicyError {
    fn from(err: XmlParseError) -> ExecutePolicyError {
        let XmlParseError(message) = err;
        ExecutePolicyError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ExecutePolicyError {
    fn from(err: CredentialsError) -> ExecutePolicyError {
        ExecutePolicyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ExecutePolicyError {
    fn from(err: HttpDispatchError) -> ExecutePolicyError {
        ExecutePolicyError::HttpDispatch(err)
    }
}
impl From<io::Error> for ExecutePolicyError {
    fn from(err: io::Error) -> ExecutePolicyError {
        ExecutePolicyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ExecutePolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ExecutePolicyError {
    fn description(&self) -> &str {
        match *self {
            ExecutePolicyError::ResourceContentionFault(ref cause) => cause,
            ExecutePolicyError::ScalingActivityInProgressFault(ref cause) => cause,
            ExecutePolicyError::Validation(ref cause) => cause,
            ExecutePolicyError::Credentials(ref err) => err.description(),
            ExecutePolicyError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ExecutePolicyError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ExitStandby
#[derive(Debug, PartialEq)]
pub enum ExitStandbyError {
    /// <p>You already have a pending update to an Auto Scaling resource (for example, a group, instance, or load balancer).</p>
    ResourceContentionFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ExitStandbyError {
    pub fn from_body(body: &str) -> ExitStandbyError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ResourceContention" => {
                    ExitStandbyError::ResourceContentionFault(String::from(parsed_error.message))
                }
                _ => ExitStandbyError::Unknown(String::from(body)),
            },
            Err(_) => ExitStandbyError::Unknown(body.to_string()),
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

impl From<XmlParseError> for ExitStandbyError {
    fn from(err: XmlParseError) -> ExitStandbyError {
        let XmlParseError(message) = err;
        ExitStandbyError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ExitStandbyError {
    fn from(err: CredentialsError) -> ExitStandbyError {
        ExitStandbyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ExitStandbyError {
    fn from(err: HttpDispatchError) -> ExitStandbyError {
        ExitStandbyError::HttpDispatch(err)
    }
}
impl From<io::Error> for ExitStandbyError {
    fn from(err: io::Error) -> ExitStandbyError {
        ExitStandbyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ExitStandbyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ExitStandbyError {
    fn description(&self) -> &str {
        match *self {
            ExitStandbyError::ResourceContentionFault(ref cause) => cause,
            ExitStandbyError::Validation(ref cause) => cause,
            ExitStandbyError::Credentials(ref err) => err.description(),
            ExitStandbyError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ExitStandbyError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PutLifecycleHook
#[derive(Debug, PartialEq)]
pub enum PutLifecycleHookError {
    /// <p>You have already reached a limit for your Auto Scaling resources (for example, groups, launch configurations, or lifecycle hooks). For more information, see <a>DescribeAccountLimits</a>.</p>
    LimitExceededFault(String),
    /// <p>You already have a pending update to an Auto Scaling resource (for example, a group, instance, or load balancer).</p>
    ResourceContentionFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl PutLifecycleHookError {
    pub fn from_body(body: &str) -> PutLifecycleHookError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "LimitExceeded" => {
                    PutLifecycleHookError::LimitExceededFault(String::from(parsed_error.message))
                }
                "ResourceContention" => PutLifecycleHookError::ResourceContentionFault(
                    String::from(parsed_error.message),
                ),
                _ => PutLifecycleHookError::Unknown(String::from(body)),
            },
            Err(_) => PutLifecycleHookError::Unknown(body.to_string()),
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

impl From<XmlParseError> for PutLifecycleHookError {
    fn from(err: XmlParseError) -> PutLifecycleHookError {
        let XmlParseError(message) = err;
        PutLifecycleHookError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for PutLifecycleHookError {
    fn from(err: CredentialsError) -> PutLifecycleHookError {
        PutLifecycleHookError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutLifecycleHookError {
    fn from(err: HttpDispatchError) -> PutLifecycleHookError {
        PutLifecycleHookError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutLifecycleHookError {
    fn from(err: io::Error) -> PutLifecycleHookError {
        PutLifecycleHookError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutLifecycleHookError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutLifecycleHookError {
    fn description(&self) -> &str {
        match *self {
            PutLifecycleHookError::LimitExceededFault(ref cause) => cause,
            PutLifecycleHookError::ResourceContentionFault(ref cause) => cause,
            PutLifecycleHookError::Validation(ref cause) => cause,
            PutLifecycleHookError::Credentials(ref err) => err.description(),
            PutLifecycleHookError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            PutLifecycleHookError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PutNotificationConfiguration
#[derive(Debug, PartialEq)]
pub enum PutNotificationConfigurationError {
    /// <p>You have already reached a limit for your Auto Scaling resources (for example, groups, launch configurations, or lifecycle hooks). For more information, see <a>DescribeAccountLimits</a>.</p>
    LimitExceededFault(String),
    /// <p>You already have a pending update to an Auto Scaling resource (for example, a group, instance, or load balancer).</p>
    ResourceContentionFault(String),
    /// <p>The service-linked role is not yet ready for use.</p>
    ServiceLinkedRoleFailure(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl PutNotificationConfigurationError {
    pub fn from_body(body: &str) -> PutNotificationConfigurationError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "LimitExceeded" => PutNotificationConfigurationError::LimitExceededFault(
                    String::from(parsed_error.message),
                ),
                "ResourceContention" => PutNotificationConfigurationError::ResourceContentionFault(
                    String::from(parsed_error.message),
                ),
                "ServiceLinkedRoleFailure" => {
                    PutNotificationConfigurationError::ServiceLinkedRoleFailure(String::from(
                        parsed_error.message,
                    ))
                }
                _ => PutNotificationConfigurationError::Unknown(String::from(body)),
            },
            Err(_) => PutNotificationConfigurationError::Unknown(body.to_string()),
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

impl From<XmlParseError> for PutNotificationConfigurationError {
    fn from(err: XmlParseError) -> PutNotificationConfigurationError {
        let XmlParseError(message) = err;
        PutNotificationConfigurationError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for PutNotificationConfigurationError {
    fn from(err: CredentialsError) -> PutNotificationConfigurationError {
        PutNotificationConfigurationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutNotificationConfigurationError {
    fn from(err: HttpDispatchError) -> PutNotificationConfigurationError {
        PutNotificationConfigurationError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutNotificationConfigurationError {
    fn from(err: io::Error) -> PutNotificationConfigurationError {
        PutNotificationConfigurationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutNotificationConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutNotificationConfigurationError {
    fn description(&self) -> &str {
        match *self {
            PutNotificationConfigurationError::LimitExceededFault(ref cause) => cause,
            PutNotificationConfigurationError::ResourceContentionFault(ref cause) => cause,
            PutNotificationConfigurationError::ServiceLinkedRoleFailure(ref cause) => cause,
            PutNotificationConfigurationError::Validation(ref cause) => cause,
            PutNotificationConfigurationError::Credentials(ref err) => err.description(),
            PutNotificationConfigurationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            PutNotificationConfigurationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PutScalingPolicy
#[derive(Debug, PartialEq)]
pub enum PutScalingPolicyError {
    /// <p>You have already reached a limit for your Auto Scaling resources (for example, groups, launch configurations, or lifecycle hooks). For more information, see <a>DescribeAccountLimits</a>.</p>
    LimitExceededFault(String),
    /// <p>You already have a pending update to an Auto Scaling resource (for example, a group, instance, or load balancer).</p>
    ResourceContentionFault(String),
    /// <p>The service-linked role is not yet ready for use.</p>
    ServiceLinkedRoleFailure(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl PutScalingPolicyError {
    pub fn from_body(body: &str) -> PutScalingPolicyError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "LimitExceeded" => {
                    PutScalingPolicyError::LimitExceededFault(String::from(parsed_error.message))
                }
                "ResourceContention" => PutScalingPolicyError::ResourceContentionFault(
                    String::from(parsed_error.message),
                ),
                "ServiceLinkedRoleFailure" => PutScalingPolicyError::ServiceLinkedRoleFailure(
                    String::from(parsed_error.message),
                ),
                _ => PutScalingPolicyError::Unknown(String::from(body)),
            },
            Err(_) => PutScalingPolicyError::Unknown(body.to_string()),
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

impl From<XmlParseError> for PutScalingPolicyError {
    fn from(err: XmlParseError) -> PutScalingPolicyError {
        let XmlParseError(message) = err;
        PutScalingPolicyError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for PutScalingPolicyError {
    fn from(err: CredentialsError) -> PutScalingPolicyError {
        PutScalingPolicyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutScalingPolicyError {
    fn from(err: HttpDispatchError) -> PutScalingPolicyError {
        PutScalingPolicyError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutScalingPolicyError {
    fn from(err: io::Error) -> PutScalingPolicyError {
        PutScalingPolicyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutScalingPolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutScalingPolicyError {
    fn description(&self) -> &str {
        match *self {
            PutScalingPolicyError::LimitExceededFault(ref cause) => cause,
            PutScalingPolicyError::ResourceContentionFault(ref cause) => cause,
            PutScalingPolicyError::ServiceLinkedRoleFailure(ref cause) => cause,
            PutScalingPolicyError::Validation(ref cause) => cause,
            PutScalingPolicyError::Credentials(ref err) => err.description(),
            PutScalingPolicyError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            PutScalingPolicyError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PutScheduledUpdateGroupAction
#[derive(Debug, PartialEq)]
pub enum PutScheduledUpdateGroupActionError {
    /// <p>You already have an Auto Scaling group or launch configuration with this name.</p>
    AlreadyExistsFault(String),
    /// <p>You have already reached a limit for your Auto Scaling resources (for example, groups, launch configurations, or lifecycle hooks). For more information, see <a>DescribeAccountLimits</a>.</p>
    LimitExceededFault(String),
    /// <p>You already have a pending update to an Auto Scaling resource (for example, a group, instance, or load balancer).</p>
    ResourceContentionFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl PutScheduledUpdateGroupActionError {
    pub fn from_body(body: &str) -> PutScheduledUpdateGroupActionError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "AlreadyExists" => PutScheduledUpdateGroupActionError::AlreadyExistsFault(
                    String::from(parsed_error.message),
                ),
                "LimitExceeded" => PutScheduledUpdateGroupActionError::LimitExceededFault(
                    String::from(parsed_error.message),
                ),
                "ResourceContention" => {
                    PutScheduledUpdateGroupActionError::ResourceContentionFault(String::from(
                        parsed_error.message,
                    ))
                }
                _ => PutScheduledUpdateGroupActionError::Unknown(String::from(body)),
            },
            Err(_) => PutScheduledUpdateGroupActionError::Unknown(body.to_string()),
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

impl From<XmlParseError> for PutScheduledUpdateGroupActionError {
    fn from(err: XmlParseError) -> PutScheduledUpdateGroupActionError {
        let XmlParseError(message) = err;
        PutScheduledUpdateGroupActionError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for PutScheduledUpdateGroupActionError {
    fn from(err: CredentialsError) -> PutScheduledUpdateGroupActionError {
        PutScheduledUpdateGroupActionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutScheduledUpdateGroupActionError {
    fn from(err: HttpDispatchError) -> PutScheduledUpdateGroupActionError {
        PutScheduledUpdateGroupActionError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutScheduledUpdateGroupActionError {
    fn from(err: io::Error) -> PutScheduledUpdateGroupActionError {
        PutScheduledUpdateGroupActionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutScheduledUpdateGroupActionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutScheduledUpdateGroupActionError {
    fn description(&self) -> &str {
        match *self {
            PutScheduledUpdateGroupActionError::AlreadyExistsFault(ref cause) => cause,
            PutScheduledUpdateGroupActionError::LimitExceededFault(ref cause) => cause,
            PutScheduledUpdateGroupActionError::ResourceContentionFault(ref cause) => cause,
            PutScheduledUpdateGroupActionError::Validation(ref cause) => cause,
            PutScheduledUpdateGroupActionError::Credentials(ref err) => err.description(),
            PutScheduledUpdateGroupActionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            PutScheduledUpdateGroupActionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RecordLifecycleActionHeartbeat
#[derive(Debug, PartialEq)]
pub enum RecordLifecycleActionHeartbeatError {
    /// <p>You already have a pending update to an Auto Scaling resource (for example, a group, instance, or load balancer).</p>
    ResourceContentionFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl RecordLifecycleActionHeartbeatError {
    pub fn from_body(body: &str) -> RecordLifecycleActionHeartbeatError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ResourceContention" => {
                    RecordLifecycleActionHeartbeatError::ResourceContentionFault(String::from(
                        parsed_error.message,
                    ))
                }
                _ => RecordLifecycleActionHeartbeatError::Unknown(String::from(body)),
            },
            Err(_) => RecordLifecycleActionHeartbeatError::Unknown(body.to_string()),
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

impl From<XmlParseError> for RecordLifecycleActionHeartbeatError {
    fn from(err: XmlParseError) -> RecordLifecycleActionHeartbeatError {
        let XmlParseError(message) = err;
        RecordLifecycleActionHeartbeatError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for RecordLifecycleActionHeartbeatError {
    fn from(err: CredentialsError) -> RecordLifecycleActionHeartbeatError {
        RecordLifecycleActionHeartbeatError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RecordLifecycleActionHeartbeatError {
    fn from(err: HttpDispatchError) -> RecordLifecycleActionHeartbeatError {
        RecordLifecycleActionHeartbeatError::HttpDispatch(err)
    }
}
impl From<io::Error> for RecordLifecycleActionHeartbeatError {
    fn from(err: io::Error) -> RecordLifecycleActionHeartbeatError {
        RecordLifecycleActionHeartbeatError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RecordLifecycleActionHeartbeatError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RecordLifecycleActionHeartbeatError {
    fn description(&self) -> &str {
        match *self {
            RecordLifecycleActionHeartbeatError::ResourceContentionFault(ref cause) => cause,
            RecordLifecycleActionHeartbeatError::Validation(ref cause) => cause,
            RecordLifecycleActionHeartbeatError::Credentials(ref err) => err.description(),
            RecordLifecycleActionHeartbeatError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            RecordLifecycleActionHeartbeatError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ResumeProcesses
#[derive(Debug, PartialEq)]
pub enum ResumeProcessesError {
    /// <p>You already have a pending update to an Auto Scaling resource (for example, a group, instance, or load balancer).</p>
    ResourceContentionFault(String),
    /// <p>The operation can't be performed because the resource is in use.</p>
    ResourceInUseFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ResumeProcessesError {
    pub fn from_body(body: &str) -> ResumeProcessesError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ResourceContention" => ResumeProcessesError::ResourceContentionFault(
                    String::from(parsed_error.message),
                ),
                "ResourceInUse" => {
                    ResumeProcessesError::ResourceInUseFault(String::from(parsed_error.message))
                }
                _ => ResumeProcessesError::Unknown(String::from(body)),
            },
            Err(_) => ResumeProcessesError::Unknown(body.to_string()),
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

impl From<XmlParseError> for ResumeProcessesError {
    fn from(err: XmlParseError) -> ResumeProcessesError {
        let XmlParseError(message) = err;
        ResumeProcessesError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ResumeProcessesError {
    fn from(err: CredentialsError) -> ResumeProcessesError {
        ResumeProcessesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ResumeProcessesError {
    fn from(err: HttpDispatchError) -> ResumeProcessesError {
        ResumeProcessesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ResumeProcessesError {
    fn from(err: io::Error) -> ResumeProcessesError {
        ResumeProcessesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ResumeProcessesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ResumeProcessesError {
    fn description(&self) -> &str {
        match *self {
            ResumeProcessesError::ResourceContentionFault(ref cause) => cause,
            ResumeProcessesError::ResourceInUseFault(ref cause) => cause,
            ResumeProcessesError::Validation(ref cause) => cause,
            ResumeProcessesError::Credentials(ref err) => err.description(),
            ResumeProcessesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ResumeProcessesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by SetDesiredCapacity
#[derive(Debug, PartialEq)]
pub enum SetDesiredCapacityError {
    /// <p>You already have a pending update to an Auto Scaling resource (for example, a group, instance, or load balancer).</p>
    ResourceContentionFault(String),
    /// <p>The operation can't be performed because there are scaling activities in progress.</p>
    ScalingActivityInProgressFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl SetDesiredCapacityError {
    pub fn from_body(body: &str) -> SetDesiredCapacityError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ResourceContention" => SetDesiredCapacityError::ResourceContentionFault(
                    String::from(parsed_error.message),
                ),
                "ScalingActivityInProgress" => {
                    SetDesiredCapacityError::ScalingActivityInProgressFault(String::from(
                        parsed_error.message,
                    ))
                }
                _ => SetDesiredCapacityError::Unknown(String::from(body)),
            },
            Err(_) => SetDesiredCapacityError::Unknown(body.to_string()),
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

impl From<XmlParseError> for SetDesiredCapacityError {
    fn from(err: XmlParseError) -> SetDesiredCapacityError {
        let XmlParseError(message) = err;
        SetDesiredCapacityError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for SetDesiredCapacityError {
    fn from(err: CredentialsError) -> SetDesiredCapacityError {
        SetDesiredCapacityError::Credentials(err)
    }
}
impl From<HttpDispatchError> for SetDesiredCapacityError {
    fn from(err: HttpDispatchError) -> SetDesiredCapacityError {
        SetDesiredCapacityError::HttpDispatch(err)
    }
}
impl From<io::Error> for SetDesiredCapacityError {
    fn from(err: io::Error) -> SetDesiredCapacityError {
        SetDesiredCapacityError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for SetDesiredCapacityError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SetDesiredCapacityError {
    fn description(&self) -> &str {
        match *self {
            SetDesiredCapacityError::ResourceContentionFault(ref cause) => cause,
            SetDesiredCapacityError::ScalingActivityInProgressFault(ref cause) => cause,
            SetDesiredCapacityError::Validation(ref cause) => cause,
            SetDesiredCapacityError::Credentials(ref err) => err.description(),
            SetDesiredCapacityError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            SetDesiredCapacityError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by SetInstanceHealth
#[derive(Debug, PartialEq)]
pub enum SetInstanceHealthError {
    /// <p>You already have a pending update to an Auto Scaling resource (for example, a group, instance, or load balancer).</p>
    ResourceContentionFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl SetInstanceHealthError {
    pub fn from_body(body: &str) -> SetInstanceHealthError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ResourceContention" => SetInstanceHealthError::ResourceContentionFault(
                    String::from(parsed_error.message),
                ),
                _ => SetInstanceHealthError::Unknown(String::from(body)),
            },
            Err(_) => SetInstanceHealthError::Unknown(body.to_string()),
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

impl From<XmlParseError> for SetInstanceHealthError {
    fn from(err: XmlParseError) -> SetInstanceHealthError {
        let XmlParseError(message) = err;
        SetInstanceHealthError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for SetInstanceHealthError {
    fn from(err: CredentialsError) -> SetInstanceHealthError {
        SetInstanceHealthError::Credentials(err)
    }
}
impl From<HttpDispatchError> for SetInstanceHealthError {
    fn from(err: HttpDispatchError) -> SetInstanceHealthError {
        SetInstanceHealthError::HttpDispatch(err)
    }
}
impl From<io::Error> for SetInstanceHealthError {
    fn from(err: io::Error) -> SetInstanceHealthError {
        SetInstanceHealthError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for SetInstanceHealthError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SetInstanceHealthError {
    fn description(&self) -> &str {
        match *self {
            SetInstanceHealthError::ResourceContentionFault(ref cause) => cause,
            SetInstanceHealthError::Validation(ref cause) => cause,
            SetInstanceHealthError::Credentials(ref err) => err.description(),
            SetInstanceHealthError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            SetInstanceHealthError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by SetInstanceProtection
#[derive(Debug, PartialEq)]
pub enum SetInstanceProtectionError {
    /// <p>You have already reached a limit for your Auto Scaling resources (for example, groups, launch configurations, or lifecycle hooks). For more information, see <a>DescribeAccountLimits</a>.</p>
    LimitExceededFault(String),
    /// <p>You already have a pending update to an Auto Scaling resource (for example, a group, instance, or load balancer).</p>
    ResourceContentionFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl SetInstanceProtectionError {
    pub fn from_body(body: &str) -> SetInstanceProtectionError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "LimitExceeded" => SetInstanceProtectionError::LimitExceededFault(String::from(
                    parsed_error.message,
                )),
                "ResourceContention" => SetInstanceProtectionError::ResourceContentionFault(
                    String::from(parsed_error.message),
                ),
                _ => SetInstanceProtectionError::Unknown(String::from(body)),
            },
            Err(_) => SetInstanceProtectionError::Unknown(body.to_string()),
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

impl From<XmlParseError> for SetInstanceProtectionError {
    fn from(err: XmlParseError) -> SetInstanceProtectionError {
        let XmlParseError(message) = err;
        SetInstanceProtectionError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for SetInstanceProtectionError {
    fn from(err: CredentialsError) -> SetInstanceProtectionError {
        SetInstanceProtectionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for SetInstanceProtectionError {
    fn from(err: HttpDispatchError) -> SetInstanceProtectionError {
        SetInstanceProtectionError::HttpDispatch(err)
    }
}
impl From<io::Error> for SetInstanceProtectionError {
    fn from(err: io::Error) -> SetInstanceProtectionError {
        SetInstanceProtectionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for SetInstanceProtectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SetInstanceProtectionError {
    fn description(&self) -> &str {
        match *self {
            SetInstanceProtectionError::LimitExceededFault(ref cause) => cause,
            SetInstanceProtectionError::ResourceContentionFault(ref cause) => cause,
            SetInstanceProtectionError::Validation(ref cause) => cause,
            SetInstanceProtectionError::Credentials(ref err) => err.description(),
            SetInstanceProtectionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            SetInstanceProtectionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by SuspendProcesses
#[derive(Debug, PartialEq)]
pub enum SuspendProcessesError {
    /// <p>You already have a pending update to an Auto Scaling resource (for example, a group, instance, or load balancer).</p>
    ResourceContentionFault(String),
    /// <p>The operation can't be performed because the resource is in use.</p>
    ResourceInUseFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl SuspendProcessesError {
    pub fn from_body(body: &str) -> SuspendProcessesError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ResourceContention" => SuspendProcessesError::ResourceContentionFault(
                    String::from(parsed_error.message),
                ),
                "ResourceInUse" => {
                    SuspendProcessesError::ResourceInUseFault(String::from(parsed_error.message))
                }
                _ => SuspendProcessesError::Unknown(String::from(body)),
            },
            Err(_) => SuspendProcessesError::Unknown(body.to_string()),
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

impl From<XmlParseError> for SuspendProcessesError {
    fn from(err: XmlParseError) -> SuspendProcessesError {
        let XmlParseError(message) = err;
        SuspendProcessesError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for SuspendProcessesError {
    fn from(err: CredentialsError) -> SuspendProcessesError {
        SuspendProcessesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for SuspendProcessesError {
    fn from(err: HttpDispatchError) -> SuspendProcessesError {
        SuspendProcessesError::HttpDispatch(err)
    }
}
impl From<io::Error> for SuspendProcessesError {
    fn from(err: io::Error) -> SuspendProcessesError {
        SuspendProcessesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for SuspendProcessesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SuspendProcessesError {
    fn description(&self) -> &str {
        match *self {
            SuspendProcessesError::ResourceContentionFault(ref cause) => cause,
            SuspendProcessesError::ResourceInUseFault(ref cause) => cause,
            SuspendProcessesError::Validation(ref cause) => cause,
            SuspendProcessesError::Credentials(ref err) => err.description(),
            SuspendProcessesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            SuspendProcessesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by TerminateInstanceInAutoScalingGroup
#[derive(Debug, PartialEq)]
pub enum TerminateInstanceInAutoScalingGroupError {
    /// <p>You already have a pending update to an Auto Scaling resource (for example, a group, instance, or load balancer).</p>
    ResourceContentionFault(String),
    /// <p>The operation can't be performed because there are scaling activities in progress.</p>
    ScalingActivityInProgressFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl TerminateInstanceInAutoScalingGroupError {
    pub fn from_body(body: &str) -> TerminateInstanceInAutoScalingGroupError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ResourceContention" => {
                    TerminateInstanceInAutoScalingGroupError::ResourceContentionFault(String::from(
                        parsed_error.message,
                    ))
                }
                "ScalingActivityInProgress" => {
                    TerminateInstanceInAutoScalingGroupError::ScalingActivityInProgressFault(
                        String::from(parsed_error.message),
                    )
                }
                _ => TerminateInstanceInAutoScalingGroupError::Unknown(String::from(body)),
            },
            Err(_) => TerminateInstanceInAutoScalingGroupError::Unknown(body.to_string()),
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

impl From<XmlParseError> for TerminateInstanceInAutoScalingGroupError {
    fn from(err: XmlParseError) -> TerminateInstanceInAutoScalingGroupError {
        let XmlParseError(message) = err;
        TerminateInstanceInAutoScalingGroupError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for TerminateInstanceInAutoScalingGroupError {
    fn from(err: CredentialsError) -> TerminateInstanceInAutoScalingGroupError {
        TerminateInstanceInAutoScalingGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for TerminateInstanceInAutoScalingGroupError {
    fn from(err: HttpDispatchError) -> TerminateInstanceInAutoScalingGroupError {
        TerminateInstanceInAutoScalingGroupError::HttpDispatch(err)
    }
}
impl From<io::Error> for TerminateInstanceInAutoScalingGroupError {
    fn from(err: io::Error) -> TerminateInstanceInAutoScalingGroupError {
        TerminateInstanceInAutoScalingGroupError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for TerminateInstanceInAutoScalingGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for TerminateInstanceInAutoScalingGroupError {
    fn description(&self) -> &str {
        match *self {
            TerminateInstanceInAutoScalingGroupError::ResourceContentionFault(ref cause) => cause,
            TerminateInstanceInAutoScalingGroupError::ScalingActivityInProgressFault(ref cause) => {
                cause
            }
            TerminateInstanceInAutoScalingGroupError::Validation(ref cause) => cause,
            TerminateInstanceInAutoScalingGroupError::Credentials(ref err) => err.description(),
            TerminateInstanceInAutoScalingGroupError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            TerminateInstanceInAutoScalingGroupError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateAutoScalingGroup
#[derive(Debug, PartialEq)]
pub enum UpdateAutoScalingGroupError {
    /// <p>You already have a pending update to an Auto Scaling resource (for example, a group, instance, or load balancer).</p>
    ResourceContentionFault(String),
    /// <p>The operation can't be performed because there are scaling activities in progress.</p>
    ScalingActivityInProgressFault(String),
    /// <p>The service-linked role is not yet ready for use.</p>
    ServiceLinkedRoleFailure(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateAutoScalingGroupError {
    pub fn from_body(body: &str) -> UpdateAutoScalingGroupError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ResourceContention" => UpdateAutoScalingGroupError::ResourceContentionFault(
                    String::from(parsed_error.message),
                ),
                "ScalingActivityInProgress" => {
                    UpdateAutoScalingGroupError::ScalingActivityInProgressFault(String::from(
                        parsed_error.message,
                    ))
                }
                "ServiceLinkedRoleFailure" => {
                    UpdateAutoScalingGroupError::ServiceLinkedRoleFailure(String::from(
                        parsed_error.message,
                    ))
                }
                _ => UpdateAutoScalingGroupError::Unknown(String::from(body)),
            },
            Err(_) => UpdateAutoScalingGroupError::Unknown(body.to_string()),
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

impl From<XmlParseError> for UpdateAutoScalingGroupError {
    fn from(err: XmlParseError) -> UpdateAutoScalingGroupError {
        let XmlParseError(message) = err;
        UpdateAutoScalingGroupError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for UpdateAutoScalingGroupError {
    fn from(err: CredentialsError) -> UpdateAutoScalingGroupError {
        UpdateAutoScalingGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateAutoScalingGroupError {
    fn from(err: HttpDispatchError) -> UpdateAutoScalingGroupError {
        UpdateAutoScalingGroupError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateAutoScalingGroupError {
    fn from(err: io::Error) -> UpdateAutoScalingGroupError {
        UpdateAutoScalingGroupError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateAutoScalingGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateAutoScalingGroupError {
    fn description(&self) -> &str {
        match *self {
            UpdateAutoScalingGroupError::ResourceContentionFault(ref cause) => cause,
            UpdateAutoScalingGroupError::ScalingActivityInProgressFault(ref cause) => cause,
            UpdateAutoScalingGroupError::ServiceLinkedRoleFailure(ref cause) => cause,
            UpdateAutoScalingGroupError::Validation(ref cause) => cause,
            UpdateAutoScalingGroupError::Credentials(ref err) => err.description(),
            UpdateAutoScalingGroupError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateAutoScalingGroupError::Unknown(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the Auto Scaling API. Auto Scaling clients implement this trait.
pub trait Autoscaling {
    /// <p>Attaches one or more EC2 instances to the specified Auto Scaling group.</p> <p>When you attach instances, Auto Scaling increases the desired capacity of the group by the number of instances being attached. If the number of instances being attached plus the desired capacity of the group exceeds the maximum size of the group, the operation fails.</p> <p>If there is a Classic Load Balancer attached to your Auto Scaling group, the instances are also registered with the load balancer. If there are target groups attached to your Auto Scaling group, the instances are also registered with the target groups.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/autoscaling/latest/userguide/attach-instance-asg.html">Attach EC2 Instances to Your Auto Scaling Group</a> in the <i>Auto Scaling User Guide</i>.</p>
    fn attach_instances(
        &self,
        input: AttachInstancesQuery,
    ) -> RusotoFuture<(), AttachInstancesError>;

    /// <p>Attaches one or more target groups to the specified Auto Scaling group.</p> <p>To describe the target groups for an Auto Scaling group, use <a>DescribeLoadBalancerTargetGroups</a>. To detach the target group from the Auto Scaling group, use <a>DetachLoadBalancerTargetGroups</a>.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/autoscaling/latest/userguide/attach-load-balancer-asg.html">Attach a Load Balancer to Your Auto Scaling Group</a> in the <i>Auto Scaling User Guide</i>.</p>
    fn attach_load_balancer_target_groups(
        &self,
        input: AttachLoadBalancerTargetGroupsType,
    ) -> RusotoFuture<AttachLoadBalancerTargetGroupsResultType, AttachLoadBalancerTargetGroupsError>;

    /// <p>Attaches one or more Classic Load Balancers to the specified Auto Scaling group.</p> <p>To attach an Application Load Balancer instead, see <a>AttachLoadBalancerTargetGroups</a>.</p> <p>To describe the load balancers for an Auto Scaling group, use <a>DescribeLoadBalancers</a>. To detach the load balancer from the Auto Scaling group, use <a>DetachLoadBalancers</a>.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/autoscaling/latest/userguide/attach-load-balancer-asg.html">Attach a Load Balancer to Your Auto Scaling Group</a> in the <i>Auto Scaling User Guide</i>.</p>
    fn attach_load_balancers(
        &self,
        input: AttachLoadBalancersType,
    ) -> RusotoFuture<AttachLoadBalancersResultType, AttachLoadBalancersError>;

    /// <p>Completes the lifecycle action for the specified token or instance with the specified result.</p> <p>This step is a part of the procedure for adding a lifecycle hook to an Auto Scaling group:</p> <ol> <li> <p>(Optional) Create a Lambda function and a rule that allows CloudWatch Events to invoke your Lambda function when Auto Scaling launches or terminates instances.</p> </li> <li> <p>(Optional) Create a notification target and an IAM role. The target can be either an Amazon SQS queue or an Amazon SNS topic. The role allows Auto Scaling to publish lifecycle notifications to the target.</p> </li> <li> <p>Create the lifecycle hook. Specify whether the hook is used when the instances launch or terminate.</p> </li> <li> <p>If you need more time, record the lifecycle action heartbeat to keep the instance in a pending state.</p> </li> <li> <p> <b>If you finish before the timeout period ends, complete the lifecycle action.</b> </p> </li> </ol> <p>For more information, see <a href="http://docs.aws.amazon.com/autoscaling/latest/userguide/AutoScalingGroupLifecycle.html">Auto Scaling Lifecycle</a> in the <i>Auto Scaling User Guide</i>.</p>
    fn complete_lifecycle_action(
        &self,
        input: CompleteLifecycleActionType,
    ) -> RusotoFuture<CompleteLifecycleActionAnswer, CompleteLifecycleActionError>;

    /// <p>Creates an Auto Scaling group with the specified name and attributes.</p> <p>If you exceed your maximum limit of Auto Scaling groups, the call fails. For information about viewing this limit, see <a>DescribeAccountLimits</a>. For information about updating this limit, see <a href="http://docs.aws.amazon.com/autoscaling/latest/userguide/as-account-limits.html">Auto Scaling Limits</a> in the <i>Auto Scaling User Guide</i>.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/autoscaling/latest/userguide/AutoScalingGroup.html">Auto Scaling Groups</a> in the <i>Auto Scaling User Guide</i>.</p>
    fn create_auto_scaling_group(
        &self,
        input: CreateAutoScalingGroupType,
    ) -> RusotoFuture<(), CreateAutoScalingGroupError>;

    /// <p>Creates a launch configuration.</p> <p>If you exceed your maximum limit of launch configurations, the call fails. For information about viewing this limit, see <a>DescribeAccountLimits</a>. For information about updating this limit, see <a href="http://docs.aws.amazon.com/autoscaling/latest/userguide/as-account-limits.html">Auto Scaling Limits</a> in the <i>Auto Scaling User Guide</i>.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/autoscaling/latest/userguide/LaunchConfiguration.html">Launch Configurations</a> in the <i>Auto Scaling User Guide</i>.</p>
    fn create_launch_configuration(
        &self,
        input: CreateLaunchConfigurationType,
    ) -> RusotoFuture<(), CreateLaunchConfigurationError>;

    /// <p>Creates or updates tags for the specified Auto Scaling group.</p> <p>When you specify a tag with a key that already exists, the operation overwrites the previous tag definition, and you do not get an error message.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/autoscaling/latest/userguide/autoscaling-tagging.html">Tagging Auto Scaling Groups and Instances</a> in the <i>Auto Scaling User Guide</i>.</p>
    fn create_or_update_tags(
        &self,
        input: CreateOrUpdateTagsType,
    ) -> RusotoFuture<(), CreateOrUpdateTagsError>;

    /// <p>Deletes the specified Auto Scaling group.</p> <p>If the group has instances or scaling activities in progress, you must specify the option to force the deletion in order for it to succeed.</p> <p>If the group has policies, deleting the group deletes the policies, the underlying alarm actions, and any alarm that no longer has an associated action.</p> <p>To remove instances from the Auto Scaling group before deleting it, call <a>DetachInstances</a> with the list of instances and the option to decrement the desired capacity so that Auto Scaling does not launch replacement instances.</p> <p>To terminate all instances before deleting the Auto Scaling group, call <a>UpdateAutoScalingGroup</a> and set the minimum size and desired capacity of the Auto Scaling group to zero.</p>
    fn delete_auto_scaling_group(
        &self,
        input: DeleteAutoScalingGroupType,
    ) -> RusotoFuture<(), DeleteAutoScalingGroupError>;

    /// <p>Deletes the specified launch configuration.</p> <p>The launch configuration must not be attached to an Auto Scaling group. When this call completes, the launch configuration is no longer available for use.</p>
    fn delete_launch_configuration(
        &self,
        input: LaunchConfigurationNameType,
    ) -> RusotoFuture<(), DeleteLaunchConfigurationError>;

    /// <p>Deletes the specified lifecycle hook.</p> <p>If there are any outstanding lifecycle actions, they are completed first (<code>ABANDON</code> for launching instances, <code>CONTINUE</code> for terminating instances).</p>
    fn delete_lifecycle_hook(
        &self,
        input: DeleteLifecycleHookType,
    ) -> RusotoFuture<DeleteLifecycleHookAnswer, DeleteLifecycleHookError>;

    /// <p>Deletes the specified notification.</p>
    fn delete_notification_configuration(
        &self,
        input: DeleteNotificationConfigurationType,
    ) -> RusotoFuture<(), DeleteNotificationConfigurationError>;

    /// <p>Deletes the specified Auto Scaling policy.</p> <p>Deleting a policy deletes the underlying alarm action, but does not delete the alarm, even if it no longer has an associated action.</p>
    fn delete_policy(&self, input: DeletePolicyType) -> RusotoFuture<(), DeletePolicyError>;

    /// <p>Deletes the specified scheduled action.</p>
    fn delete_scheduled_action(
        &self,
        input: DeleteScheduledActionType,
    ) -> RusotoFuture<(), DeleteScheduledActionError>;

    /// <p>Deletes the specified tags.</p>
    fn delete_tags(&self, input: DeleteTagsType) -> RusotoFuture<(), DeleteTagsError>;

    /// <p>Describes the current Auto Scaling resource limits for your AWS account.</p> <p>For information about requesting an increase in these limits, see <a href="http://docs.aws.amazon.com/autoscaling/latest/userguide/as-account-limits.html">Auto Scaling Limits</a> in the <i>Auto Scaling User Guide</i>.</p>
    fn describe_account_limits(
        &self,
    ) -> RusotoFuture<DescribeAccountLimitsAnswer, DescribeAccountLimitsError>;

    /// <p>Describes the policy adjustment types for use with <a>PutScalingPolicy</a>.</p>
    fn describe_adjustment_types(
        &self,
    ) -> RusotoFuture<DescribeAdjustmentTypesAnswer, DescribeAdjustmentTypesError>;

    /// <p>Describes one or more Auto Scaling groups.</p>
    fn describe_auto_scaling_groups(
        &self,
        input: AutoScalingGroupNamesType,
    ) -> RusotoFuture<AutoScalingGroupsType, DescribeAutoScalingGroupsError>;

    /// <p>Describes one or more Auto Scaling instances.</p>
    fn describe_auto_scaling_instances(
        &self,
        input: DescribeAutoScalingInstancesType,
    ) -> RusotoFuture<AutoScalingInstancesType, DescribeAutoScalingInstancesError>;

    /// <p>Describes the notification types that are supported by Auto Scaling.</p>
    fn describe_auto_scaling_notification_types(
        &self,
    ) -> RusotoFuture<
        DescribeAutoScalingNotificationTypesAnswer,
        DescribeAutoScalingNotificationTypesError,
    >;

    /// <p>Describes one or more launch configurations.</p>
    fn describe_launch_configurations(
        &self,
        input: LaunchConfigurationNamesType,
    ) -> RusotoFuture<LaunchConfigurationsType, DescribeLaunchConfigurationsError>;

    /// <p>Describes the available types of lifecycle hooks.</p>
    fn describe_lifecycle_hook_types(
        &self,
    ) -> RusotoFuture<DescribeLifecycleHookTypesAnswer, DescribeLifecycleHookTypesError>;

    /// <p>Describes the lifecycle hooks for the specified Auto Scaling group.</p>
    fn describe_lifecycle_hooks(
        &self,
        input: DescribeLifecycleHooksType,
    ) -> RusotoFuture<DescribeLifecycleHooksAnswer, DescribeLifecycleHooksError>;

    /// <p>Describes the target groups for the specified Auto Scaling group.</p>
    fn describe_load_balancer_target_groups(
        &self,
        input: DescribeLoadBalancerTargetGroupsRequest,
    ) -> RusotoFuture<DescribeLoadBalancerTargetGroupsResponse, DescribeLoadBalancerTargetGroupsError>;

    /// <p>Describes the load balancers for the specified Auto Scaling group.</p> <p>Note that this operation describes only Classic Load Balancers. If you have Application Load Balancers, use <a>DescribeLoadBalancerTargetGroups</a> instead.</p>
    fn describe_load_balancers(
        &self,
        input: DescribeLoadBalancersRequest,
    ) -> RusotoFuture<DescribeLoadBalancersResponse, DescribeLoadBalancersError>;

    /// <p>Describes the available CloudWatch metrics for Auto Scaling.</p> <p>Note that the <code>GroupStandbyInstances</code> metric is not returned by default. You must explicitly request this metric when calling <a>EnableMetricsCollection</a>.</p>
    fn describe_metric_collection_types(
        &self,
    ) -> RusotoFuture<DescribeMetricCollectionTypesAnswer, DescribeMetricCollectionTypesError>;

    /// <p>Describes the notification actions associated with the specified Auto Scaling group.</p>
    fn describe_notification_configurations(
        &self,
        input: DescribeNotificationConfigurationsType,
    ) -> RusotoFuture<
        DescribeNotificationConfigurationsAnswer,
        DescribeNotificationConfigurationsError,
    >;

    /// <p>Describes the policies for the specified Auto Scaling group.</p>
    fn describe_policies(
        &self,
        input: DescribePoliciesType,
    ) -> RusotoFuture<PoliciesType, DescribePoliciesError>;

    /// <p>Describes one or more scaling activities for the specified Auto Scaling group.</p>
    fn describe_scaling_activities(
        &self,
        input: DescribeScalingActivitiesType,
    ) -> RusotoFuture<ActivitiesType, DescribeScalingActivitiesError>;

    /// <p>Describes the scaling process types for use with <a>ResumeProcesses</a> and <a>SuspendProcesses</a>.</p>
    fn describe_scaling_process_types(
        &self,
    ) -> RusotoFuture<ProcessesType, DescribeScalingProcessTypesError>;

    /// <p>Describes the actions scheduled for your Auto Scaling group that haven't run. To describe the actions that have already run, use <a>DescribeScalingActivities</a>.</p>
    fn describe_scheduled_actions(
        &self,
        input: DescribeScheduledActionsType,
    ) -> RusotoFuture<ScheduledActionsType, DescribeScheduledActionsError>;

    /// <p>Describes the specified tags.</p> <p>You can use filters to limit the results. For example, you can query for the tags for a specific Auto Scaling group. You can specify multiple values for a filter. A tag must match at least one of the specified values for it to be included in the results.</p> <p>You can also specify multiple filters. The result includes information for a particular tag only if it matches all the filters. If there's no match, no special message is returned.</p>
    fn describe_tags(&self, input: DescribeTagsType) -> RusotoFuture<TagsType, DescribeTagsError>;

    /// <p>Describes the termination policies supported by Auto Scaling.</p>
    fn describe_termination_policy_types(
        &self,
    ) -> RusotoFuture<DescribeTerminationPolicyTypesAnswer, DescribeTerminationPolicyTypesError>;

    /// <p>Removes one or more instances from the specified Auto Scaling group.</p> <p>After the instances are detached, you can manage them independent of the Auto Scaling group.</p> <p>If you do not specify the option to decrement the desired capacity, Auto Scaling launches instances to replace the ones that are detached.</p> <p>If there is a Classic Load Balancer attached to the Auto Scaling group, the instances are deregistered from the load balancer. If there are target groups attached to the Auto Scaling group, the instances are deregistered from the target groups.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/autoscaling/latest/userguide/detach-instance-asg.html">Detach EC2 Instances from Your Auto Scaling Group</a> in the <i>Auto Scaling User Guide</i>.</p>
    fn detach_instances(
        &self,
        input: DetachInstancesQuery,
    ) -> RusotoFuture<DetachInstancesAnswer, DetachInstancesError>;

    /// <p>Detaches one or more target groups from the specified Auto Scaling group.</p>
    fn detach_load_balancer_target_groups(
        &self,
        input: DetachLoadBalancerTargetGroupsType,
    ) -> RusotoFuture<DetachLoadBalancerTargetGroupsResultType, DetachLoadBalancerTargetGroupsError>;

    /// <p>Detaches one or more Classic Load Balancers from the specified Auto Scaling group.</p> <p>Note that this operation detaches only Classic Load Balancers. If you have Application Load Balancers, use <a>DetachLoadBalancerTargetGroups</a> instead.</p> <p>When you detach a load balancer, it enters the <code>Removing</code> state while deregistering the instances in the group. When all instances are deregistered, then you can no longer describe the load balancer using <a>DescribeLoadBalancers</a>. Note that the instances remain running.</p>
    fn detach_load_balancers(
        &self,
        input: DetachLoadBalancersType,
    ) -> RusotoFuture<DetachLoadBalancersResultType, DetachLoadBalancersError>;

    /// <p>Disables group metrics for the specified Auto Scaling group.</p>
    fn disable_metrics_collection(
        &self,
        input: DisableMetricsCollectionQuery,
    ) -> RusotoFuture<(), DisableMetricsCollectionError>;

    /// <p>Enables group metrics for the specified Auto Scaling group. For more information, see <a href="http://docs.aws.amazon.com/autoscaling/latest/userguide/as-instance-monitoring.html">Monitoring Your Auto Scaling Groups and Instances</a> in the <i>Auto Scaling User Guide</i>.</p>
    fn enable_metrics_collection(
        &self,
        input: EnableMetricsCollectionQuery,
    ) -> RusotoFuture<(), EnableMetricsCollectionError>;

    /// <p>Moves the specified instances into the standby state.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/autoscaling/latest/userguide/as-enter-exit-standby.html">Temporarily Removing Instances from Your Auto Scaling Group</a> in the <i>Auto Scaling User Guide</i>.</p>
    fn enter_standby(
        &self,
        input: EnterStandbyQuery,
    ) -> RusotoFuture<EnterStandbyAnswer, EnterStandbyError>;

    /// <p>Executes the specified policy.</p>
    fn execute_policy(&self, input: ExecutePolicyType) -> RusotoFuture<(), ExecutePolicyError>;

    /// <p>Moves the specified instances out of the standby state.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/autoscaling/latest/userguide/as-enter-exit-standby.html">Temporarily Removing Instances from Your Auto Scaling Group</a> in the <i>Auto Scaling User Guide</i>.</p>
    fn exit_standby(
        &self,
        input: ExitStandbyQuery,
    ) -> RusotoFuture<ExitStandbyAnswer, ExitStandbyError>;

    /// <p>Creates or updates a lifecycle hook for the specified Auto Scaling Group.</p> <p>A lifecycle hook tells Auto Scaling that you want to perform an action on an instance that is not actively in service; for example, either when the instance launches or before the instance terminates.</p> <p>This step is a part of the procedure for adding a lifecycle hook to an Auto Scaling group:</p> <ol> <li> <p>(Optional) Create a Lambda function and a rule that allows CloudWatch Events to invoke your Lambda function when Auto Scaling launches or terminates instances.</p> </li> <li> <p>(Optional) Create a notification target and an IAM role. The target can be either an Amazon SQS queue or an Amazon SNS topic. The role allows Auto Scaling to publish lifecycle notifications to the target.</p> </li> <li> <p> <b>Create the lifecycle hook. Specify whether the hook is used when the instances launch or terminate.</b> </p> </li> <li> <p>If you need more time, record the lifecycle action heartbeat to keep the instance in a pending state.</p> </li> <li> <p>If you finish before the timeout period ends, complete the lifecycle action.</p> </li> </ol> <p>For more information, see <a href="http://docs.aws.amazon.com/autoscaling/latest/userguide/lifecycle-hooks.html">Auto Scaling Lifecycle Hooks</a> in the <i>Auto Scaling User Guide</i>.</p> <p>If you exceed your maximum limit of lifecycle hooks, which by default is 50 per Auto Scaling group, the call fails. For information about updating this limit, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html">AWS Service Limits</a> in the <i>Amazon Web Services General Reference</i>.</p>
    fn put_lifecycle_hook(
        &self,
        input: PutLifecycleHookType,
    ) -> RusotoFuture<PutLifecycleHookAnswer, PutLifecycleHookError>;

    /// <p>Configures an Auto Scaling group to send notifications when specified events take place. Subscribers to the specified topic can have messages delivered to an endpoint such as a web server or an email address.</p> <p>This configuration overwrites any existing configuration.</p> <p>For more information see <a href="http://docs.aws.amazon.com/autoscaling/latest/userguide/ASGettingNotifications.html">Getting SNS Notifications When Your Auto Scaling Group Scales</a> in the <i>Auto Scaling User Guide</i>.</p>
    fn put_notification_configuration(
        &self,
        input: PutNotificationConfigurationType,
    ) -> RusotoFuture<(), PutNotificationConfigurationError>;

    /// <p>Creates or updates a policy for an Auto Scaling group. To update an existing policy, use the existing policy name and set the parameters you want to change. Any existing parameter not changed in an update to an existing policy is not changed in this update request.</p> <p>If you exceed your maximum limit of step adjustments, which by default is 20 per region, the call fails. For information about updating this limit, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html">AWS Service Limits</a> in the <i>Amazon Web Services General Reference</i>.</p>
    fn put_scaling_policy(
        &self,
        input: PutScalingPolicyType,
    ) -> RusotoFuture<PolicyARNType, PutScalingPolicyError>;

    /// <p>Creates or updates a scheduled scaling action for an Auto Scaling group. When updating a scheduled scaling action, if you leave a parameter unspecified, the corresponding value remains unchanged.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/autoscaling/latest/userguide/schedule_time.html">Scheduled Scaling</a> in the <i>Auto Scaling User Guide</i>.</p>
    fn put_scheduled_update_group_action(
        &self,
        input: PutScheduledUpdateGroupActionType,
    ) -> RusotoFuture<(), PutScheduledUpdateGroupActionError>;

    /// <p>Records a heartbeat for the lifecycle action associated with the specified token or instance. This extends the timeout by the length of time defined using <a>PutLifecycleHook</a>.</p> <p>This step is a part of the procedure for adding a lifecycle hook to an Auto Scaling group:</p> <ol> <li> <p>(Optional) Create a Lambda function and a rule that allows CloudWatch Events to invoke your Lambda function when Auto Scaling launches or terminates instances.</p> </li> <li> <p>(Optional) Create a notification target and an IAM role. The target can be either an Amazon SQS queue or an Amazon SNS topic. The role allows Auto Scaling to publish lifecycle notifications to the target.</p> </li> <li> <p>Create the lifecycle hook. Specify whether the hook is used when the instances launch or terminate.</p> </li> <li> <p> <b>If you need more time, record the lifecycle action heartbeat to keep the instance in a pending state.</b> </p> </li> <li> <p>If you finish before the timeout period ends, complete the lifecycle action.</p> </li> </ol> <p>For more information, see <a href="http://docs.aws.amazon.com/autoscaling/latest/userguide/AutoScalingGroupLifecycle.html">Auto Scaling Lifecycle</a> in the <i>Auto Scaling User Guide</i>.</p>
    fn record_lifecycle_action_heartbeat(
        &self,
        input: RecordLifecycleActionHeartbeatType,
    ) -> RusotoFuture<RecordLifecycleActionHeartbeatAnswer, RecordLifecycleActionHeartbeatError>;

    /// <p>Resumes the specified suspended Auto Scaling processes, or all suspended process, for the specified Auto Scaling group.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/autoscaling/latest/userguide/as-suspend-resume-processes.html">Suspending and Resuming Auto Scaling Processes</a> in the <i>Auto Scaling User Guide</i>.</p>
    fn resume_processes(
        &self,
        input: ScalingProcessQuery,
    ) -> RusotoFuture<(), ResumeProcessesError>;

    /// <p>Sets the size of the specified Auto Scaling group.</p> <p>For more information about desired capacity, see <a href="http://docs.aws.amazon.com/autoscaling/latest/userguide/WhatIsAutoScaling.html">What Is Auto Scaling?</a> in the <i>Auto Scaling User Guide</i>.</p>
    fn set_desired_capacity(
        &self,
        input: SetDesiredCapacityType,
    ) -> RusotoFuture<(), SetDesiredCapacityError>;

    /// <p>Sets the health status of the specified instance.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/autoscaling/latest/userguide/healthcheck.html">Health Checks</a> in the <i>Auto Scaling User Guide</i>.</p>
    fn set_instance_health(
        &self,
        input: SetInstanceHealthQuery,
    ) -> RusotoFuture<(), SetInstanceHealthError>;

    /// <p>Updates the instance protection settings of the specified instances.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/autoscaling/latest/userguide/as-instance-termination.html#instance-protection">Instance Protection</a> in the <i>Auto Scaling User Guide</i>.</p>
    fn set_instance_protection(
        &self,
        input: SetInstanceProtectionQuery,
    ) -> RusotoFuture<SetInstanceProtectionAnswer, SetInstanceProtectionError>;

    /// <p>Suspends the specified Auto Scaling processes, or all processes, for the specified Auto Scaling group.</p> <p>Note that if you suspend either the <code>Launch</code> or <code>Terminate</code> process types, it can prevent other process types from functioning properly.</p> <p>To resume processes that have been suspended, use <a>ResumeProcesses</a>.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/autoscaling/latest/userguide/as-suspend-resume-processes.html">Suspending and Resuming Auto Scaling Processes</a> in the <i>Auto Scaling User Guide</i>.</p>
    fn suspend_processes(
        &self,
        input: ScalingProcessQuery,
    ) -> RusotoFuture<(), SuspendProcessesError>;

    /// <p>Terminates the specified instance and optionally adjusts the desired group size.</p> <p>This call simply makes a termination request. The instance is not terminated immediately.</p>
    fn terminate_instance_in_auto_scaling_group(
        &self,
        input: TerminateInstanceInAutoScalingGroupType,
    ) -> RusotoFuture<ActivityType, TerminateInstanceInAutoScalingGroupError>;

    /// <p><p>Updates the configuration for the specified Auto Scaling group.</p> <p>The new settings take effect on any scaling activities after this call returns. Scaling activities that are currently in progress aren&#39;t affected.</p> <p>To update an Auto Scaling group with a launch configuration with <code>InstanceMonitoring</code> set to <code>false</code>, you must first disable the collection of group metrics. Otherwise, you will get an error. If you have previously enabled the collection of group metrics, you can disable it using <a>DisableMetricsCollection</a>.</p> <p>Note the following:</p> <ul> <li> <p>If you specify a new value for <code>MinSize</code> without specifying a value for <code>DesiredCapacity</code>, and the new <code>MinSize</code> is larger than the current size of the group, we implicitly call <a>SetDesiredCapacity</a> to set the size of the group to the new value of <code>MinSize</code>.</p> </li> <li> <p>If you specify a new value for <code>MaxSize</code> without specifying a value for <code>DesiredCapacity</code>, and the new <code>MaxSize</code> is smaller than the current size of the group, we implicitly call <a>SetDesiredCapacity</a> to set the size of the group to the new value of <code>MaxSize</code>.</p> </li> <li> <p>All other optional parameters are left unchanged if not specified.</p> </li> </ul></p>
    fn update_auto_scaling_group(
        &self,
        input: UpdateAutoScalingGroupType,
    ) -> RusotoFuture<(), UpdateAutoScalingGroupError>;
}
/// A client for the Auto Scaling API.
pub struct AutoscalingClient<P = CredentialsProvider, D = RequestDispatcher>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    inner: ClientInner<P, D>,
    region: region::Region,
}

impl AutoscalingClient {
    /// Creates a simple client backed by an implicit event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    ///
    /// See the `rusoto_core::reactor` module for more details.
    pub fn simple(region: region::Region) -> AutoscalingClient {
        AutoscalingClient::new(
            RequestDispatcher::default(),
            CredentialsProvider::default(),
            region,
        )
    }
}

impl<P, D> AutoscalingClient<P, D>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
        AutoscalingClient {
            inner: ClientInner::new(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl<P, D> Autoscaling for AutoscalingClient<P, D>
where
    P: ProvideAwsCredentials + 'static,
    D: DispatchSignedRequest + 'static,
{
    /// <p>Attaches one or more EC2 instances to the specified Auto Scaling group.</p> <p>When you attach instances, Auto Scaling increases the desired capacity of the group by the number of instances being attached. If the number of instances being attached plus the desired capacity of the group exceeds the maximum size of the group, the operation fails.</p> <p>If there is a Classic Load Balancer attached to your Auto Scaling group, the instances are also registered with the load balancer. If there are target groups attached to your Auto Scaling group, the instances are also registered with the target groups.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/autoscaling/latest/userguide/attach-instance-asg.html">Attach EC2 Instances to Your Auto Scaling Group</a> in the <i>Auto Scaling User Guide</i>.</p>
    fn attach_instances(
        &self,
        input: AttachInstancesQuery,
    ) -> RusotoFuture<(), AttachInstancesError> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "AttachInstances");
        params.put("Version", "2011-01-01");
        AttachInstancesQuerySerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(AttachInstancesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(future::ok(::std::mem::drop(response)))
        });

        RusotoFuture::new(future)
    }

    /// <p>Attaches one or more target groups to the specified Auto Scaling group.</p> <p>To describe the target groups for an Auto Scaling group, use <a>DescribeLoadBalancerTargetGroups</a>. To detach the target group from the Auto Scaling group, use <a>DetachLoadBalancerTargetGroups</a>.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/autoscaling/latest/userguide/attach-load-balancer-asg.html">Attach a Load Balancer to Your Auto Scaling Group</a> in the <i>Auto Scaling User Guide</i>.</p>
    fn attach_load_balancer_target_groups(
        &self,
        input: AttachLoadBalancerTargetGroupsType,
    ) -> RusotoFuture<AttachLoadBalancerTargetGroupsResultType, AttachLoadBalancerTargetGroupsError>
    {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "AttachLoadBalancerTargetGroups");
        params.put("Version", "2011-01-01");
        AttachLoadBalancerTargetGroupsTypeSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(AttachLoadBalancerTargetGroupsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = AttachLoadBalancerTargetGroupsResultType::default();
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
                        AttachLoadBalancerTargetGroupsResultTypeDeserializer::deserialize(
                            "AttachLoadBalancerTargetGroupsResult",
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

    /// <p>Attaches one or more Classic Load Balancers to the specified Auto Scaling group.</p> <p>To attach an Application Load Balancer instead, see <a>AttachLoadBalancerTargetGroups</a>.</p> <p>To describe the load balancers for an Auto Scaling group, use <a>DescribeLoadBalancers</a>. To detach the load balancer from the Auto Scaling group, use <a>DetachLoadBalancers</a>.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/autoscaling/latest/userguide/attach-load-balancer-asg.html">Attach a Load Balancer to Your Auto Scaling Group</a> in the <i>Auto Scaling User Guide</i>.</p>
    fn attach_load_balancers(
        &self,
        input: AttachLoadBalancersType,
    ) -> RusotoFuture<AttachLoadBalancersResultType, AttachLoadBalancersError> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "AttachLoadBalancers");
        params.put("Version", "2011-01-01");
        AttachLoadBalancersTypeSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(AttachLoadBalancersError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = AttachLoadBalancersResultType::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(AttachLoadBalancersResultTypeDeserializer::deserialize(
                        "AttachLoadBalancersResult",
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

    /// <p>Completes the lifecycle action for the specified token or instance with the specified result.</p> <p>This step is a part of the procedure for adding a lifecycle hook to an Auto Scaling group:</p> <ol> <li> <p>(Optional) Create a Lambda function and a rule that allows CloudWatch Events to invoke your Lambda function when Auto Scaling launches or terminates instances.</p> </li> <li> <p>(Optional) Create a notification target and an IAM role. The target can be either an Amazon SQS queue or an Amazon SNS topic. The role allows Auto Scaling to publish lifecycle notifications to the target.</p> </li> <li> <p>Create the lifecycle hook. Specify whether the hook is used when the instances launch or terminate.</p> </li> <li> <p>If you need more time, record the lifecycle action heartbeat to keep the instance in a pending state.</p> </li> <li> <p> <b>If you finish before the timeout period ends, complete the lifecycle action.</b> </p> </li> </ol> <p>For more information, see <a href="http://docs.aws.amazon.com/autoscaling/latest/userguide/AutoScalingGroupLifecycle.html">Auto Scaling Lifecycle</a> in the <i>Auto Scaling User Guide</i>.</p>
    fn complete_lifecycle_action(
        &self,
        input: CompleteLifecycleActionType,
    ) -> RusotoFuture<CompleteLifecycleActionAnswer, CompleteLifecycleActionError> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CompleteLifecycleAction");
        params.put("Version", "2011-01-01");
        CompleteLifecycleActionTypeSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CompleteLifecycleActionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = CompleteLifecycleActionAnswer::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(CompleteLifecycleActionAnswerDeserializer::deserialize(
                        "CompleteLifecycleActionResult",
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

    /// <p>Creates an Auto Scaling group with the specified name and attributes.</p> <p>If you exceed your maximum limit of Auto Scaling groups, the call fails. For information about viewing this limit, see <a>DescribeAccountLimits</a>. For information about updating this limit, see <a href="http://docs.aws.amazon.com/autoscaling/latest/userguide/as-account-limits.html">Auto Scaling Limits</a> in the <i>Auto Scaling User Guide</i>.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/autoscaling/latest/userguide/AutoScalingGroup.html">Auto Scaling Groups</a> in the <i>Auto Scaling User Guide</i>.</p>
    fn create_auto_scaling_group(
        &self,
        input: CreateAutoScalingGroupType,
    ) -> RusotoFuture<(), CreateAutoScalingGroupError> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CreateAutoScalingGroup");
        params.put("Version", "2011-01-01");
        CreateAutoScalingGroupTypeSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateAutoScalingGroupError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(future::ok(::std::mem::drop(response)))
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates a launch configuration.</p> <p>If you exceed your maximum limit of launch configurations, the call fails. For information about viewing this limit, see <a>DescribeAccountLimits</a>. For information about updating this limit, see <a href="http://docs.aws.amazon.com/autoscaling/latest/userguide/as-account-limits.html">Auto Scaling Limits</a> in the <i>Auto Scaling User Guide</i>.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/autoscaling/latest/userguide/LaunchConfiguration.html">Launch Configurations</a> in the <i>Auto Scaling User Guide</i>.</p>
    fn create_launch_configuration(
        &self,
        input: CreateLaunchConfigurationType,
    ) -> RusotoFuture<(), CreateLaunchConfigurationError> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CreateLaunchConfiguration");
        params.put("Version", "2011-01-01");
        CreateLaunchConfigurationTypeSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateLaunchConfigurationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(future::ok(::std::mem::drop(response)))
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates or updates tags for the specified Auto Scaling group.</p> <p>When you specify a tag with a key that already exists, the operation overwrites the previous tag definition, and you do not get an error message.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/autoscaling/latest/userguide/autoscaling-tagging.html">Tagging Auto Scaling Groups and Instances</a> in the <i>Auto Scaling User Guide</i>.</p>
    fn create_or_update_tags(
        &self,
        input: CreateOrUpdateTagsType,
    ) -> RusotoFuture<(), CreateOrUpdateTagsError> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CreateOrUpdateTags");
        params.put("Version", "2011-01-01");
        CreateOrUpdateTagsTypeSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateOrUpdateTagsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(future::ok(::std::mem::drop(response)))
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes the specified Auto Scaling group.</p> <p>If the group has instances or scaling activities in progress, you must specify the option to force the deletion in order for it to succeed.</p> <p>If the group has policies, deleting the group deletes the policies, the underlying alarm actions, and any alarm that no longer has an associated action.</p> <p>To remove instances from the Auto Scaling group before deleting it, call <a>DetachInstances</a> with the list of instances and the option to decrement the desired capacity so that Auto Scaling does not launch replacement instances.</p> <p>To terminate all instances before deleting the Auto Scaling group, call <a>UpdateAutoScalingGroup</a> and set the minimum size and desired capacity of the Auto Scaling group to zero.</p>
    fn delete_auto_scaling_group(
        &self,
        input: DeleteAutoScalingGroupType,
    ) -> RusotoFuture<(), DeleteAutoScalingGroupError> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteAutoScalingGroup");
        params.put("Version", "2011-01-01");
        DeleteAutoScalingGroupTypeSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteAutoScalingGroupError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(future::ok(::std::mem::drop(response)))
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes the specified launch configuration.</p> <p>The launch configuration must not be attached to an Auto Scaling group. When this call completes, the launch configuration is no longer available for use.</p>
    fn delete_launch_configuration(
        &self,
        input: LaunchConfigurationNameType,
    ) -> RusotoFuture<(), DeleteLaunchConfigurationError> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteLaunchConfiguration");
        params.put("Version", "2011-01-01");
        LaunchConfigurationNameTypeSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteLaunchConfigurationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(future::ok(::std::mem::drop(response)))
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes the specified lifecycle hook.</p> <p>If there are any outstanding lifecycle actions, they are completed first (<code>ABANDON</code> for launching instances, <code>CONTINUE</code> for terminating instances).</p>
    fn delete_lifecycle_hook(
        &self,
        input: DeleteLifecycleHookType,
    ) -> RusotoFuture<DeleteLifecycleHookAnswer, DeleteLifecycleHookError> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteLifecycleHook");
        params.put("Version", "2011-01-01");
        DeleteLifecycleHookTypeSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteLifecycleHookError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DeleteLifecycleHookAnswer::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DeleteLifecycleHookAnswerDeserializer::deserialize(
                        "DeleteLifecycleHookResult",
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

    /// <p>Deletes the specified notification.</p>
    fn delete_notification_configuration(
        &self,
        input: DeleteNotificationConfigurationType,
    ) -> RusotoFuture<(), DeleteNotificationConfigurationError> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteNotificationConfiguration");
        params.put("Version", "2011-01-01");
        DeleteNotificationConfigurationTypeSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteNotificationConfigurationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(future::ok(::std::mem::drop(response)))
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes the specified Auto Scaling policy.</p> <p>Deleting a policy deletes the underlying alarm action, but does not delete the alarm, even if it no longer has an associated action.</p>
    fn delete_policy(&self, input: DeletePolicyType) -> RusotoFuture<(), DeletePolicyError> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeletePolicy");
        params.put("Version", "2011-01-01");
        DeletePolicyTypeSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeletePolicyError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(future::ok(::std::mem::drop(response)))
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes the specified scheduled action.</p>
    fn delete_scheduled_action(
        &self,
        input: DeleteScheduledActionType,
    ) -> RusotoFuture<(), DeleteScheduledActionError> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteScheduledAction");
        params.put("Version", "2011-01-01");
        DeleteScheduledActionTypeSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteScheduledActionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(future::ok(::std::mem::drop(response)))
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes the specified tags.</p>
    fn delete_tags(&self, input: DeleteTagsType) -> RusotoFuture<(), DeleteTagsError> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteTags");
        params.put("Version", "2011-01-01");
        DeleteTagsTypeSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteTagsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(future::ok(::std::mem::drop(response)))
        });

        RusotoFuture::new(future)
    }

    /// <p>Describes the current Auto Scaling resource limits for your AWS account.</p> <p>For information about requesting an increase in these limits, see <a href="http://docs.aws.amazon.com/autoscaling/latest/userguide/as-account-limits.html">Auto Scaling Limits</a> in the <i>Auto Scaling User Guide</i>.</p>
    fn describe_account_limits(
        &self,
    ) -> RusotoFuture<DescribeAccountLimitsAnswer, DescribeAccountLimitsError> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeAccountLimits");
        params.put("Version", "2011-01-01");

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
                    result = DescribeAccountLimitsAnswer::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DescribeAccountLimitsAnswerDeserializer::deserialize(
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

    /// <p>Describes the policy adjustment types for use with <a>PutScalingPolicy</a>.</p>
    fn describe_adjustment_types(
        &self,
    ) -> RusotoFuture<DescribeAdjustmentTypesAnswer, DescribeAdjustmentTypesError> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeAdjustmentTypes");
        params.put("Version", "2011-01-01");

        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeAdjustmentTypesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DescribeAdjustmentTypesAnswer::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DescribeAdjustmentTypesAnswerDeserializer::deserialize(
                        "DescribeAdjustmentTypesResult",
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

    /// <p>Describes one or more Auto Scaling groups.</p>
    fn describe_auto_scaling_groups(
        &self,
        input: AutoScalingGroupNamesType,
    ) -> RusotoFuture<AutoScalingGroupsType, DescribeAutoScalingGroupsError> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeAutoScalingGroups");
        params.put("Version", "2011-01-01");
        AutoScalingGroupNamesTypeSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeAutoScalingGroupsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = AutoScalingGroupsType::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(AutoScalingGroupsTypeDeserializer::deserialize(
                        "DescribeAutoScalingGroupsResult",
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

    /// <p>Describes one or more Auto Scaling instances.</p>
    fn describe_auto_scaling_instances(
        &self,
        input: DescribeAutoScalingInstancesType,
    ) -> RusotoFuture<AutoScalingInstancesType, DescribeAutoScalingInstancesError> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeAutoScalingInstances");
        params.put("Version", "2011-01-01");
        DescribeAutoScalingInstancesTypeSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeAutoScalingInstancesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = AutoScalingInstancesType::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(AutoScalingInstancesTypeDeserializer::deserialize(
                        "DescribeAutoScalingInstancesResult",
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

    /// <p>Describes the notification types that are supported by Auto Scaling.</p>
    fn describe_auto_scaling_notification_types(
        &self,
    ) -> RusotoFuture<
        DescribeAutoScalingNotificationTypesAnswer,
        DescribeAutoScalingNotificationTypesError,
    > {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeAutoScalingNotificationTypes");
        params.put("Version", "2011-01-01");

        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeAutoScalingNotificationTypesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DescribeAutoScalingNotificationTypesAnswer::default();
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
                        DescribeAutoScalingNotificationTypesAnswerDeserializer::deserialize(
                            "DescribeAutoScalingNotificationTypesResult",
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

    /// <p>Describes one or more launch configurations.</p>
    fn describe_launch_configurations(
        &self,
        input: LaunchConfigurationNamesType,
    ) -> RusotoFuture<LaunchConfigurationsType, DescribeLaunchConfigurationsError> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeLaunchConfigurations");
        params.put("Version", "2011-01-01");
        LaunchConfigurationNamesTypeSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeLaunchConfigurationsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = LaunchConfigurationsType::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(LaunchConfigurationsTypeDeserializer::deserialize(
                        "DescribeLaunchConfigurationsResult",
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

    /// <p>Describes the available types of lifecycle hooks.</p>
    fn describe_lifecycle_hook_types(
        &self,
    ) -> RusotoFuture<DescribeLifecycleHookTypesAnswer, DescribeLifecycleHookTypesError> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeLifecycleHookTypes");
        params.put("Version", "2011-01-01");

        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeLifecycleHookTypesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DescribeLifecycleHookTypesAnswer::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DescribeLifecycleHookTypesAnswerDeserializer::deserialize(
                        "DescribeLifecycleHookTypesResult",
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

    /// <p>Describes the lifecycle hooks for the specified Auto Scaling group.</p>
    fn describe_lifecycle_hooks(
        &self,
        input: DescribeLifecycleHooksType,
    ) -> RusotoFuture<DescribeLifecycleHooksAnswer, DescribeLifecycleHooksError> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeLifecycleHooks");
        params.put("Version", "2011-01-01");
        DescribeLifecycleHooksTypeSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeLifecycleHooksError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DescribeLifecycleHooksAnswer::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DescribeLifecycleHooksAnswerDeserializer::deserialize(
                        "DescribeLifecycleHooksResult",
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

    /// <p>Describes the target groups for the specified Auto Scaling group.</p>
    fn describe_load_balancer_target_groups(
        &self,
        input: DescribeLoadBalancerTargetGroupsRequest,
    ) -> RusotoFuture<DescribeLoadBalancerTargetGroupsResponse, DescribeLoadBalancerTargetGroupsError>
    {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeLoadBalancerTargetGroups");
        params.put("Version", "2011-01-01");
        DescribeLoadBalancerTargetGroupsRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeLoadBalancerTargetGroupsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DescribeLoadBalancerTargetGroupsResponse::default();
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
                        DescribeLoadBalancerTargetGroupsResponseDeserializer::deserialize(
                            "DescribeLoadBalancerTargetGroupsResult",
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

    /// <p>Describes the load balancers for the specified Auto Scaling group.</p> <p>Note that this operation describes only Classic Load Balancers. If you have Application Load Balancers, use <a>DescribeLoadBalancerTargetGroups</a> instead.</p>
    fn describe_load_balancers(
        &self,
        input: DescribeLoadBalancersRequest,
    ) -> RusotoFuture<DescribeLoadBalancersResponse, DescribeLoadBalancersError> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeLoadBalancers");
        params.put("Version", "2011-01-01");
        DescribeLoadBalancersRequestSerializer::serialize(&mut params, "", &input);
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
                    result = DescribeLoadBalancersResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DescribeLoadBalancersResponseDeserializer::deserialize(
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

    /// <p>Describes the available CloudWatch metrics for Auto Scaling.</p> <p>Note that the <code>GroupStandbyInstances</code> metric is not returned by default. You must explicitly request this metric when calling <a>EnableMetricsCollection</a>.</p>
    fn describe_metric_collection_types(
        &self,
    ) -> RusotoFuture<DescribeMetricCollectionTypesAnswer, DescribeMetricCollectionTypesError> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeMetricCollectionTypes");
        params.put("Version", "2011-01-01");

        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeMetricCollectionTypesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DescribeMetricCollectionTypesAnswer::default();
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
                        DescribeMetricCollectionTypesAnswerDeserializer::deserialize(
                            "DescribeMetricCollectionTypesResult",
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

    /// <p>Describes the notification actions associated with the specified Auto Scaling group.</p>
    fn describe_notification_configurations(
        &self,
        input: DescribeNotificationConfigurationsType,
    ) -> RusotoFuture<
        DescribeNotificationConfigurationsAnswer,
        DescribeNotificationConfigurationsError,
    > {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeNotificationConfigurations");
        params.put("Version", "2011-01-01");
        DescribeNotificationConfigurationsTypeSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeNotificationConfigurationsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DescribeNotificationConfigurationsAnswer::default();
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
                        DescribeNotificationConfigurationsAnswerDeserializer::deserialize(
                            "DescribeNotificationConfigurationsResult",
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

    /// <p>Describes the policies for the specified Auto Scaling group.</p>
    fn describe_policies(
        &self,
        input: DescribePoliciesType,
    ) -> RusotoFuture<PoliciesType, DescribePoliciesError> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribePolicies");
        params.put("Version", "2011-01-01");
        DescribePoliciesTypeSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribePoliciesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = PoliciesType::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(PoliciesTypeDeserializer::deserialize(
                        "DescribePoliciesResult",
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

    /// <p>Describes one or more scaling activities for the specified Auto Scaling group.</p>
    fn describe_scaling_activities(
        &self,
        input: DescribeScalingActivitiesType,
    ) -> RusotoFuture<ActivitiesType, DescribeScalingActivitiesError> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeScalingActivities");
        params.put("Version", "2011-01-01");
        DescribeScalingActivitiesTypeSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeScalingActivitiesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ActivitiesType::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(ActivitiesTypeDeserializer::deserialize(
                        "DescribeScalingActivitiesResult",
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

    /// <p>Describes the scaling process types for use with <a>ResumeProcesses</a> and <a>SuspendProcesses</a>.</p>
    fn describe_scaling_process_types(
        &self,
    ) -> RusotoFuture<ProcessesType, DescribeScalingProcessTypesError> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeScalingProcessTypes");
        params.put("Version", "2011-01-01");

        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeScalingProcessTypesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ProcessesType::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(ProcessesTypeDeserializer::deserialize(
                        "DescribeScalingProcessTypesResult",
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

    /// <p>Describes the actions scheduled for your Auto Scaling group that haven't run. To describe the actions that have already run, use <a>DescribeScalingActivities</a>.</p>
    fn describe_scheduled_actions(
        &self,
        input: DescribeScheduledActionsType,
    ) -> RusotoFuture<ScheduledActionsType, DescribeScheduledActionsError> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeScheduledActions");
        params.put("Version", "2011-01-01");
        DescribeScheduledActionsTypeSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeScheduledActionsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ScheduledActionsType::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(ScheduledActionsTypeDeserializer::deserialize(
                        "DescribeScheduledActionsResult",
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

    /// <p>Describes the specified tags.</p> <p>You can use filters to limit the results. For example, you can query for the tags for a specific Auto Scaling group. You can specify multiple values for a filter. A tag must match at least one of the specified values for it to be included in the results.</p> <p>You can also specify multiple filters. The result includes information for a particular tag only if it matches all the filters. If there's no match, no special message is returned.</p>
    fn describe_tags(&self, input: DescribeTagsType) -> RusotoFuture<TagsType, DescribeTagsError> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeTags");
        params.put("Version", "2011-01-01");
        DescribeTagsTypeSerializer::serialize(&mut params, "", &input);
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
                    result = TagsType::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(TagsTypeDeserializer::deserialize(
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

    /// <p>Describes the termination policies supported by Auto Scaling.</p>
    fn describe_termination_policy_types(
        &self,
    ) -> RusotoFuture<DescribeTerminationPolicyTypesAnswer, DescribeTerminationPolicyTypesError>
    {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeTerminationPolicyTypes");
        params.put("Version", "2011-01-01");

        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeTerminationPolicyTypesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DescribeTerminationPolicyTypesAnswer::default();
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
                        DescribeTerminationPolicyTypesAnswerDeserializer::deserialize(
                            "DescribeTerminationPolicyTypesResult",
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

    /// <p>Removes one or more instances from the specified Auto Scaling group.</p> <p>After the instances are detached, you can manage them independent of the Auto Scaling group.</p> <p>If you do not specify the option to decrement the desired capacity, Auto Scaling launches instances to replace the ones that are detached.</p> <p>If there is a Classic Load Balancer attached to the Auto Scaling group, the instances are deregistered from the load balancer. If there are target groups attached to the Auto Scaling group, the instances are deregistered from the target groups.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/autoscaling/latest/userguide/detach-instance-asg.html">Detach EC2 Instances from Your Auto Scaling Group</a> in the <i>Auto Scaling User Guide</i>.</p>
    fn detach_instances(
        &self,
        input: DetachInstancesQuery,
    ) -> RusotoFuture<DetachInstancesAnswer, DetachInstancesError> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DetachInstances");
        params.put("Version", "2011-01-01");
        DetachInstancesQuerySerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DetachInstancesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DetachInstancesAnswer::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DetachInstancesAnswerDeserializer::deserialize(
                        "DetachInstancesResult",
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

    /// <p>Detaches one or more target groups from the specified Auto Scaling group.</p>
    fn detach_load_balancer_target_groups(
        &self,
        input: DetachLoadBalancerTargetGroupsType,
    ) -> RusotoFuture<DetachLoadBalancerTargetGroupsResultType, DetachLoadBalancerTargetGroupsError>
    {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DetachLoadBalancerTargetGroups");
        params.put("Version", "2011-01-01");
        DetachLoadBalancerTargetGroupsTypeSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DetachLoadBalancerTargetGroupsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DetachLoadBalancerTargetGroupsResultType::default();
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
                        DetachLoadBalancerTargetGroupsResultTypeDeserializer::deserialize(
                            "DetachLoadBalancerTargetGroupsResult",
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

    /// <p>Detaches one or more Classic Load Balancers from the specified Auto Scaling group.</p> <p>Note that this operation detaches only Classic Load Balancers. If you have Application Load Balancers, use <a>DetachLoadBalancerTargetGroups</a> instead.</p> <p>When you detach a load balancer, it enters the <code>Removing</code> state while deregistering the instances in the group. When all instances are deregistered, then you can no longer describe the load balancer using <a>DescribeLoadBalancers</a>. Note that the instances remain running.</p>
    fn detach_load_balancers(
        &self,
        input: DetachLoadBalancersType,
    ) -> RusotoFuture<DetachLoadBalancersResultType, DetachLoadBalancersError> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DetachLoadBalancers");
        params.put("Version", "2011-01-01");
        DetachLoadBalancersTypeSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DetachLoadBalancersError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DetachLoadBalancersResultType::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DetachLoadBalancersResultTypeDeserializer::deserialize(
                        "DetachLoadBalancersResult",
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

    /// <p>Disables group metrics for the specified Auto Scaling group.</p>
    fn disable_metrics_collection(
        &self,
        input: DisableMetricsCollectionQuery,
    ) -> RusotoFuture<(), DisableMetricsCollectionError> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DisableMetricsCollection");
        params.put("Version", "2011-01-01");
        DisableMetricsCollectionQuerySerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DisableMetricsCollectionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(future::ok(::std::mem::drop(response)))
        });

        RusotoFuture::new(future)
    }

    /// <p>Enables group metrics for the specified Auto Scaling group. For more information, see <a href="http://docs.aws.amazon.com/autoscaling/latest/userguide/as-instance-monitoring.html">Monitoring Your Auto Scaling Groups and Instances</a> in the <i>Auto Scaling User Guide</i>.</p>
    fn enable_metrics_collection(
        &self,
        input: EnableMetricsCollectionQuery,
    ) -> RusotoFuture<(), EnableMetricsCollectionError> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "EnableMetricsCollection");
        params.put("Version", "2011-01-01");
        EnableMetricsCollectionQuerySerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(EnableMetricsCollectionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(future::ok(::std::mem::drop(response)))
        });

        RusotoFuture::new(future)
    }

    /// <p>Moves the specified instances into the standby state.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/autoscaling/latest/userguide/as-enter-exit-standby.html">Temporarily Removing Instances from Your Auto Scaling Group</a> in the <i>Auto Scaling User Guide</i>.</p>
    fn enter_standby(
        &self,
        input: EnterStandbyQuery,
    ) -> RusotoFuture<EnterStandbyAnswer, EnterStandbyError> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "EnterStandby");
        params.put("Version", "2011-01-01");
        EnterStandbyQuerySerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(EnterStandbyError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = EnterStandbyAnswer::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(EnterStandbyAnswerDeserializer::deserialize(
                        "EnterStandbyResult",
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

    /// <p>Executes the specified policy.</p>
    fn execute_policy(&self, input: ExecutePolicyType) -> RusotoFuture<(), ExecutePolicyError> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ExecutePolicy");
        params.put("Version", "2011-01-01");
        ExecutePolicyTypeSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ExecutePolicyError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(future::ok(::std::mem::drop(response)))
        });

        RusotoFuture::new(future)
    }

    /// <p>Moves the specified instances out of the standby state.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/autoscaling/latest/userguide/as-enter-exit-standby.html">Temporarily Removing Instances from Your Auto Scaling Group</a> in the <i>Auto Scaling User Guide</i>.</p>
    fn exit_standby(
        &self,
        input: ExitStandbyQuery,
    ) -> RusotoFuture<ExitStandbyAnswer, ExitStandbyError> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ExitStandby");
        params.put("Version", "2011-01-01");
        ExitStandbyQuerySerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ExitStandbyError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ExitStandbyAnswer::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(ExitStandbyAnswerDeserializer::deserialize(
                        "ExitStandbyResult",
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

    /// <p>Creates or updates a lifecycle hook for the specified Auto Scaling Group.</p> <p>A lifecycle hook tells Auto Scaling that you want to perform an action on an instance that is not actively in service; for example, either when the instance launches or before the instance terminates.</p> <p>This step is a part of the procedure for adding a lifecycle hook to an Auto Scaling group:</p> <ol> <li> <p>(Optional) Create a Lambda function and a rule that allows CloudWatch Events to invoke your Lambda function when Auto Scaling launches or terminates instances.</p> </li> <li> <p>(Optional) Create a notification target and an IAM role. The target can be either an Amazon SQS queue or an Amazon SNS topic. The role allows Auto Scaling to publish lifecycle notifications to the target.</p> </li> <li> <p> <b>Create the lifecycle hook. Specify whether the hook is used when the instances launch or terminate.</b> </p> </li> <li> <p>If you need more time, record the lifecycle action heartbeat to keep the instance in a pending state.</p> </li> <li> <p>If you finish before the timeout period ends, complete the lifecycle action.</p> </li> </ol> <p>For more information, see <a href="http://docs.aws.amazon.com/autoscaling/latest/userguide/lifecycle-hooks.html">Auto Scaling Lifecycle Hooks</a> in the <i>Auto Scaling User Guide</i>.</p> <p>If you exceed your maximum limit of lifecycle hooks, which by default is 50 per Auto Scaling group, the call fails. For information about updating this limit, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html">AWS Service Limits</a> in the <i>Amazon Web Services General Reference</i>.</p>
    fn put_lifecycle_hook(
        &self,
        input: PutLifecycleHookType,
    ) -> RusotoFuture<PutLifecycleHookAnswer, PutLifecycleHookError> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "PutLifecycleHook");
        params.put("Version", "2011-01-01");
        PutLifecycleHookTypeSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(PutLifecycleHookError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = PutLifecycleHookAnswer::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(PutLifecycleHookAnswerDeserializer::deserialize(
                        "PutLifecycleHookResult",
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

    /// <p>Configures an Auto Scaling group to send notifications when specified events take place. Subscribers to the specified topic can have messages delivered to an endpoint such as a web server or an email address.</p> <p>This configuration overwrites any existing configuration.</p> <p>For more information see <a href="http://docs.aws.amazon.com/autoscaling/latest/userguide/ASGettingNotifications.html">Getting SNS Notifications When Your Auto Scaling Group Scales</a> in the <i>Auto Scaling User Guide</i>.</p>
    fn put_notification_configuration(
        &self,
        input: PutNotificationConfigurationType,
    ) -> RusotoFuture<(), PutNotificationConfigurationError> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "PutNotificationConfiguration");
        params.put("Version", "2011-01-01");
        PutNotificationConfigurationTypeSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(PutNotificationConfigurationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(future::ok(::std::mem::drop(response)))
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates or updates a policy for an Auto Scaling group. To update an existing policy, use the existing policy name and set the parameters you want to change. Any existing parameter not changed in an update to an existing policy is not changed in this update request.</p> <p>If you exceed your maximum limit of step adjustments, which by default is 20 per region, the call fails. For information about updating this limit, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html">AWS Service Limits</a> in the <i>Amazon Web Services General Reference</i>.</p>
    fn put_scaling_policy(
        &self,
        input: PutScalingPolicyType,
    ) -> RusotoFuture<PolicyARNType, PutScalingPolicyError> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "PutScalingPolicy");
        params.put("Version", "2011-01-01");
        PutScalingPolicyTypeSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(PutScalingPolicyError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = PolicyARNType::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(PolicyARNTypeDeserializer::deserialize(
                        "PutScalingPolicyResult",
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

    /// <p>Creates or updates a scheduled scaling action for an Auto Scaling group. When updating a scheduled scaling action, if you leave a parameter unspecified, the corresponding value remains unchanged.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/autoscaling/latest/userguide/schedule_time.html">Scheduled Scaling</a> in the <i>Auto Scaling User Guide</i>.</p>
    fn put_scheduled_update_group_action(
        &self,
        input: PutScheduledUpdateGroupActionType,
    ) -> RusotoFuture<(), PutScheduledUpdateGroupActionError> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "PutScheduledUpdateGroupAction");
        params.put("Version", "2011-01-01");
        PutScheduledUpdateGroupActionTypeSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(PutScheduledUpdateGroupActionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(future::ok(::std::mem::drop(response)))
        });

        RusotoFuture::new(future)
    }

    /// <p>Records a heartbeat for the lifecycle action associated with the specified token or instance. This extends the timeout by the length of time defined using <a>PutLifecycleHook</a>.</p> <p>This step is a part of the procedure for adding a lifecycle hook to an Auto Scaling group:</p> <ol> <li> <p>(Optional) Create a Lambda function and a rule that allows CloudWatch Events to invoke your Lambda function when Auto Scaling launches or terminates instances.</p> </li> <li> <p>(Optional) Create a notification target and an IAM role. The target can be either an Amazon SQS queue or an Amazon SNS topic. The role allows Auto Scaling to publish lifecycle notifications to the target.</p> </li> <li> <p>Create the lifecycle hook. Specify whether the hook is used when the instances launch or terminate.</p> </li> <li> <p> <b>If you need more time, record the lifecycle action heartbeat to keep the instance in a pending state.</b> </p> </li> <li> <p>If you finish before the timeout period ends, complete the lifecycle action.</p> </li> </ol> <p>For more information, see <a href="http://docs.aws.amazon.com/autoscaling/latest/userguide/AutoScalingGroupLifecycle.html">Auto Scaling Lifecycle</a> in the <i>Auto Scaling User Guide</i>.</p>
    fn record_lifecycle_action_heartbeat(
        &self,
        input: RecordLifecycleActionHeartbeatType,
    ) -> RusotoFuture<RecordLifecycleActionHeartbeatAnswer, RecordLifecycleActionHeartbeatError>
    {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "RecordLifecycleActionHeartbeat");
        params.put("Version", "2011-01-01");
        RecordLifecycleActionHeartbeatTypeSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(RecordLifecycleActionHeartbeatError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = RecordLifecycleActionHeartbeatAnswer::default();
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
                        RecordLifecycleActionHeartbeatAnswerDeserializer::deserialize(
                            "RecordLifecycleActionHeartbeatResult",
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

    /// <p>Resumes the specified suspended Auto Scaling processes, or all suspended process, for the specified Auto Scaling group.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/autoscaling/latest/userguide/as-suspend-resume-processes.html">Suspending and Resuming Auto Scaling Processes</a> in the <i>Auto Scaling User Guide</i>.</p>
    fn resume_processes(
        &self,
        input: ScalingProcessQuery,
    ) -> RusotoFuture<(), ResumeProcessesError> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ResumeProcesses");
        params.put("Version", "2011-01-01");
        ScalingProcessQuerySerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ResumeProcessesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(future::ok(::std::mem::drop(response)))
        });

        RusotoFuture::new(future)
    }

    /// <p>Sets the size of the specified Auto Scaling group.</p> <p>For more information about desired capacity, see <a href="http://docs.aws.amazon.com/autoscaling/latest/userguide/WhatIsAutoScaling.html">What Is Auto Scaling?</a> in the <i>Auto Scaling User Guide</i>.</p>
    fn set_desired_capacity(
        &self,
        input: SetDesiredCapacityType,
    ) -> RusotoFuture<(), SetDesiredCapacityError> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "SetDesiredCapacity");
        params.put("Version", "2011-01-01");
        SetDesiredCapacityTypeSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(SetDesiredCapacityError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(future::ok(::std::mem::drop(response)))
        });

        RusotoFuture::new(future)
    }

    /// <p>Sets the health status of the specified instance.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/autoscaling/latest/userguide/healthcheck.html">Health Checks</a> in the <i>Auto Scaling User Guide</i>.</p>
    fn set_instance_health(
        &self,
        input: SetInstanceHealthQuery,
    ) -> RusotoFuture<(), SetInstanceHealthError> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "SetInstanceHealth");
        params.put("Version", "2011-01-01");
        SetInstanceHealthQuerySerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(SetInstanceHealthError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(future::ok(::std::mem::drop(response)))
        });

        RusotoFuture::new(future)
    }

    /// <p>Updates the instance protection settings of the specified instances.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/autoscaling/latest/userguide/as-instance-termination.html#instance-protection">Instance Protection</a> in the <i>Auto Scaling User Guide</i>.</p>
    fn set_instance_protection(
        &self,
        input: SetInstanceProtectionQuery,
    ) -> RusotoFuture<SetInstanceProtectionAnswer, SetInstanceProtectionError> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "SetInstanceProtection");
        params.put("Version", "2011-01-01");
        SetInstanceProtectionQuerySerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(SetInstanceProtectionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = SetInstanceProtectionAnswer::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(SetInstanceProtectionAnswerDeserializer::deserialize(
                        "SetInstanceProtectionResult",
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

    /// <p>Suspends the specified Auto Scaling processes, or all processes, for the specified Auto Scaling group.</p> <p>Note that if you suspend either the <code>Launch</code> or <code>Terminate</code> process types, it can prevent other process types from functioning properly.</p> <p>To resume processes that have been suspended, use <a>ResumeProcesses</a>.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/autoscaling/latest/userguide/as-suspend-resume-processes.html">Suspending and Resuming Auto Scaling Processes</a> in the <i>Auto Scaling User Guide</i>.</p>
    fn suspend_processes(
        &self,
        input: ScalingProcessQuery,
    ) -> RusotoFuture<(), SuspendProcessesError> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "SuspendProcesses");
        params.put("Version", "2011-01-01");
        ScalingProcessQuerySerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(SuspendProcessesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(future::ok(::std::mem::drop(response)))
        });

        RusotoFuture::new(future)
    }

    /// <p>Terminates the specified instance and optionally adjusts the desired group size.</p> <p>This call simply makes a termination request. The instance is not terminated immediately.</p>
    fn terminate_instance_in_auto_scaling_group(
        &self,
        input: TerminateInstanceInAutoScalingGroupType,
    ) -> RusotoFuture<ActivityType, TerminateInstanceInAutoScalingGroupError> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "TerminateInstanceInAutoScalingGroup");
        params.put("Version", "2011-01-01");
        TerminateInstanceInAutoScalingGroupTypeSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(TerminateInstanceInAutoScalingGroupError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ActivityType::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(ActivityTypeDeserializer::deserialize(
                        "TerminateInstanceInAutoScalingGroupResult",
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

    /// <p><p>Updates the configuration for the specified Auto Scaling group.</p> <p>The new settings take effect on any scaling activities after this call returns. Scaling activities that are currently in progress aren&#39;t affected.</p> <p>To update an Auto Scaling group with a launch configuration with <code>InstanceMonitoring</code> set to <code>false</code>, you must first disable the collection of group metrics. Otherwise, you will get an error. If you have previously enabled the collection of group metrics, you can disable it using <a>DisableMetricsCollection</a>.</p> <p>Note the following:</p> <ul> <li> <p>If you specify a new value for <code>MinSize</code> without specifying a value for <code>DesiredCapacity</code>, and the new <code>MinSize</code> is larger than the current size of the group, we implicitly call <a>SetDesiredCapacity</a> to set the size of the group to the new value of <code>MinSize</code>.</p> </li> <li> <p>If you specify a new value for <code>MaxSize</code> without specifying a value for <code>DesiredCapacity</code>, and the new <code>MaxSize</code> is smaller than the current size of the group, we implicitly call <a>SetDesiredCapacity</a> to set the size of the group to the new value of <code>MaxSize</code>.</p> </li> <li> <p>All other optional parameters are left unchanged if not specified.</p> </li> </ul></p>
    fn update_auto_scaling_group(
        &self,
        input: UpdateAutoScalingGroupType,
    ) -> RusotoFuture<(), UpdateAutoScalingGroupError> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "UpdateAutoScalingGroup");
        params.put("Version", "2011-01-01");
        UpdateAutoScalingGroupTypeSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateAutoScalingGroupError::from_body(
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
    fn test_parse_error_autoscaling_delete_policy() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/error",
            "autoscaling-delete-policy.xml",
        );
        let mock = MockRequestDispatcher::with_status(400).with_body(&mock_response);
        let client = AutoscalingClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = DeletePolicyType::default();
        let result = client.delete_policy(request).sync();
        assert!(!result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_autoscaling_describe_adjustment_types() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "autoscaling-describe-adjustment-types.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = AutoscalingClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);

        let result = client.describe_adjustment_types().sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_autoscaling_describe_auto_scaling_groups() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "autoscaling-describe-auto-scaling-groups.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = AutoscalingClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = AutoScalingGroupNamesType::default();
        let result = client.describe_auto_scaling_groups(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_autoscaling_describe_auto_scaling_instances() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "autoscaling-describe-auto-scaling-instances.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = AutoscalingClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = DescribeAutoScalingInstancesType::default();
        let result = client.describe_auto_scaling_instances(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_autoscaling_describe_auto_scaling_notification_types() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "autoscaling-describe-auto-scaling-notification-types.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = AutoscalingClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);

        let result = client.describe_auto_scaling_notification_types().sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_autoscaling_describe_launch_configurations() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "autoscaling-describe-launch-configurations.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = AutoscalingClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = LaunchConfigurationNamesType::default();
        let result = client.describe_launch_configurations(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_autoscaling_describe_metric_collection_types() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "autoscaling-describe-metric-collection-types.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = AutoscalingClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);

        let result = client.describe_metric_collection_types().sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_autoscaling_describe_notification_configurations() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "autoscaling-describe-notification-configurations.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = AutoscalingClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = DescribeNotificationConfigurationsType::default();
        let result = client.describe_notification_configurations(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_autoscaling_describe_policies() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "autoscaling-describe-policies.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = AutoscalingClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = DescribePoliciesType::default();
        let result = client.describe_policies(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_autoscaling_describe_scaling_activities() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "autoscaling-describe-scaling-activities.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = AutoscalingClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = DescribeScalingActivitiesType::default();
        let result = client.describe_scaling_activities(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_autoscaling_describe_scaling_process_types() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "autoscaling-describe-scaling-process-types.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = AutoscalingClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);

        let result = client.describe_scaling_process_types().sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_autoscaling_describe_scheduled_actions() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "autoscaling-describe-scheduled-actions.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = AutoscalingClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = DescribeScheduledActionsType::default();
        let result = client.describe_scheduled_actions(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_autoscaling_describe_tags() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "autoscaling-describe-tags.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = AutoscalingClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = DescribeTagsType::default();
        let result = client.describe_tags(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_autoscaling_describe_termination_policy_types() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "autoscaling-describe-termination-policy-types.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = AutoscalingClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);

        let result = client.describe_termination_policy_types().sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }
}
