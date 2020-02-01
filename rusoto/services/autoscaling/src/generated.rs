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

struct ActivitiesDeserializer;
impl ActivitiesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Activity>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(ActivityDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ActivitiesType {
    /// <p>The scaling activities. Activities are sorted by start time. Activities still in progress are described first.</p>
    pub activities: Vec<Activity>,
    /// <p>A string that indicates that the response contains more items than can be returned in a single response. To receive additional items, specify this string for the <code>NextToken</code> value when requesting the next set of items. This value is null when there are no more items to return.</p>
    pub next_token: Option<String>,
}

struct ActivitiesTypeDeserializer;
impl ActivitiesTypeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ActivitiesType, XmlParseError> {
        deserialize_elements::<_, ActivitiesType, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Activities" => {
                    obj.activities
                        .extend(ActivitiesDeserializer::deserialize("Activities", stack)?);
                }
                "NextToken" => {
                    obj.next_token = Some(XmlStringDeserializer::deserialize("NextToken", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
/// <p>Describes scaling activity, which is a long-running process that represents a change to your Auto Scaling group, such as changing its size or replacing an instance.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
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
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Activity, XmlParseError> {
        deserialize_elements::<_, Activity, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "ActivityId" => {
                    obj.activity_id = XmlStringDeserializer::deserialize("ActivityId", stack)?;
                }
                "AutoScalingGroupName" => {
                    obj.auto_scaling_group_name =
                        XmlStringMaxLen255Deserializer::deserialize("AutoScalingGroupName", stack)?;
                }
                "Cause" => {
                    obj.cause = XmlStringMaxLen1023Deserializer::deserialize("Cause", stack)?;
                }
                "Description" => {
                    obj.description =
                        Some(XmlStringDeserializer::deserialize("Description", stack)?);
                }
                "Details" => {
                    obj.details = Some(XmlStringDeserializer::deserialize("Details", stack)?);
                }
                "EndTime" => {
                    obj.end_time = Some(TimestampTypeDeserializer::deserialize("EndTime", stack)?);
                }
                "Progress" => {
                    obj.progress = Some(ProgressDeserializer::deserialize("Progress", stack)?);
                }
                "StartTime" => {
                    obj.start_time = TimestampTypeDeserializer::deserialize("StartTime", stack)?;
                }
                "StatusCode" => {
                    obj.status_code =
                        ScalingActivityStatusCodeDeserializer::deserialize("StatusCode", stack)?;
                }
                "StatusMessage" => {
                    obj.status_message = Some(XmlStringMaxLen255Deserializer::deserialize(
                        "StatusMessage",
                        stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
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
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ActivityType {
    /// <p>A scaling activity.</p>
    pub activity: Option<Activity>,
}

struct ActivityTypeDeserializer;
impl ActivityTypeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ActivityType, XmlParseError> {
        deserialize_elements::<_, ActivityType, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Activity" => {
                    obj.activity = Some(ActivityDeserializer::deserialize("Activity", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
/// <p>Describes a policy adjustment type.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct AdjustmentType {
    /// <p>The policy adjustment type. The valid values are <code>ChangeInCapacity</code>, <code>ExactCapacity</code>, and <code>PercentChangeInCapacity</code>.</p>
    pub adjustment_type: Option<String>,
}

struct AdjustmentTypeDeserializer;
impl AdjustmentTypeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AdjustmentType, XmlParseError> {
        deserialize_elements::<_, AdjustmentType, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "AdjustmentType" => {
                    obj.adjustment_type = Some(XmlStringMaxLen255Deserializer::deserialize(
                        "AdjustmentType",
                        stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct AdjustmentTypesDeserializer;
impl AdjustmentTypesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<AdjustmentType>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(AdjustmentTypeDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
/// <p>Describes an alarm.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct Alarm {
    /// <p>The Amazon Resource Name (ARN) of the alarm.</p>
    pub alarm_arn: Option<String>,
    /// <p>The name of the alarm.</p>
    pub alarm_name: Option<String>,
}

struct AlarmDeserializer;
impl AlarmDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Alarm, XmlParseError> {
        deserialize_elements::<_, Alarm, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "AlarmARN" => {
                    obj.alarm_arn = Some(ResourceNameDeserializer::deserialize("AlarmARN", stack)?);
                }
                "AlarmName" => {
                    obj.alarm_name = Some(XmlStringMaxLen255Deserializer::deserialize(
                        "AlarmName",
                        stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct AlarmsDeserializer;
impl AlarmsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Alarm>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(AlarmDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
struct AsciiStringMaxLen255Deserializer;
impl AsciiStringMaxLen255Deserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct AssociatePublicIpAddressDeserializer;
impl AssociatePublicIpAddressDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<bool, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = bool::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
            &obj.auto_scaling_group_name,
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
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct AttachLoadBalancerTargetGroupsResultType {}

struct AttachLoadBalancerTargetGroupsResultTypeDeserializer;
impl AttachLoadBalancerTargetGroupsResultTypeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AttachLoadBalancerTargetGroupsResultType, XmlParseError> {
        start_element(tag_name, stack)?;

        let obj = AttachLoadBalancerTargetGroupsResultType::default();

        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
            &obj.auto_scaling_group_name,
        );
        TargetGroupARNsSerializer::serialize(
            params,
            &format!("{}{}", prefix, "TargetGroupARNs"),
            &obj.target_group_ar_ns,
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct AttachLoadBalancersResultType {}

struct AttachLoadBalancersResultTypeDeserializer;
impl AttachLoadBalancersResultTypeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AttachLoadBalancersResultType, XmlParseError> {
        start_element(tag_name, stack)?;

        let obj = AttachLoadBalancersResultType::default();

        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
            &obj.auto_scaling_group_name,
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
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
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
    /// <p>The amount of time, in seconds, that Amazon EC2 Auto Scaling waits before checking the health status of an EC2 instance that has come into service.</p>
    pub health_check_grace_period: Option<i64>,
    /// <p>The service to use for the health checks. The valid values are <code>EC2</code> and <code>ELB</code>. If you configure an Auto Scaling group to use ELB health checks, it considers the instance unhealthy if it fails either the EC2 status checks or the load balancer health checks.</p>
    pub health_check_type: String,
    /// <p>The EC2 instances associated with the group.</p>
    pub instances: Option<Vec<Instance>>,
    /// <p>The name of the associated launch configuration.</p>
    pub launch_configuration_name: Option<String>,
    /// <p>The launch template for the group.</p>
    pub launch_template: Option<LaunchTemplateSpecification>,
    /// <p>One or more load balancers associated with the group.</p>
    pub load_balancer_names: Option<Vec<String>>,
    /// <p>The maximum amount of time, in seconds, that an instance can be in service.</p> <p>Valid Range: Minimum value of 604800.</p>
    pub max_instance_lifetime: Option<i64>,
    /// <p>The maximum size of the group.</p>
    pub max_size: i64,
    /// <p>The minimum size of the group.</p>
    pub min_size: i64,
    /// <p>The mixed instances policy for the group.</p>
    pub mixed_instances_policy: Option<MixedInstancesPolicy>,
    /// <p>Indicates whether newly launched instances are protected from termination by Amazon EC2 Auto Scaling when scaling in.</p>
    pub new_instances_protected_from_scale_in: Option<bool>,
    /// <p>The name of the placement group into which to launch your instances, if any.</p>
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
    /// <p>One or more subnet IDs, if applicable, separated by commas.</p>
    pub vpc_zone_identifier: Option<String>,
}

struct AutoScalingGroupDeserializer;
impl AutoScalingGroupDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AutoScalingGroup, XmlParseError> {
        deserialize_elements::<_, AutoScalingGroup, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "AutoScalingGroupARN" => {
                    obj.auto_scaling_group_arn = Some(ResourceNameDeserializer::deserialize(
                        "AutoScalingGroupARN",
                        stack,
                    )?);
                }
                "AutoScalingGroupName" => {
                    obj.auto_scaling_group_name =
                        XmlStringMaxLen255Deserializer::deserialize("AutoScalingGroupName", stack)?;
                }
                "AvailabilityZones" => {
                    obj.availability_zones
                        .extend(AvailabilityZonesDeserializer::deserialize(
                            "AvailabilityZones",
                            stack,
                        )?);
                }
                "CreatedTime" => {
                    obj.created_time =
                        TimestampTypeDeserializer::deserialize("CreatedTime", stack)?;
                }
                "DefaultCooldown" => {
                    obj.default_cooldown =
                        CooldownDeserializer::deserialize("DefaultCooldown", stack)?;
                }
                "DesiredCapacity" => {
                    obj.desired_capacity =
                        AutoScalingGroupDesiredCapacityDeserializer::deserialize(
                            "DesiredCapacity",
                            stack,
                        )?;
                }
                "EnabledMetrics" => {
                    obj.enabled_metrics.get_or_insert(vec![]).extend(
                        EnabledMetricsDeserializer::deserialize("EnabledMetrics", stack)?,
                    );
                }
                "HealthCheckGracePeriod" => {
                    obj.health_check_grace_period =
                        Some(HealthCheckGracePeriodDeserializer::deserialize(
                            "HealthCheckGracePeriod",
                            stack,
                        )?);
                }
                "HealthCheckType" => {
                    obj.health_check_type =
                        XmlStringMaxLen32Deserializer::deserialize("HealthCheckType", stack)?;
                }
                "Instances" => {
                    obj.instances
                        .get_or_insert(vec![])
                        .extend(InstancesDeserializer::deserialize("Instances", stack)?);
                }
                "LaunchConfigurationName" => {
                    obj.launch_configuration_name =
                        Some(XmlStringMaxLen255Deserializer::deserialize(
                            "LaunchConfigurationName",
                            stack,
                        )?);
                }
                "LaunchTemplate" => {
                    obj.launch_template =
                        Some(LaunchTemplateSpecificationDeserializer::deserialize(
                            "LaunchTemplate",
                            stack,
                        )?);
                }
                "LoadBalancerNames" => {
                    obj.load_balancer_names.get_or_insert(vec![]).extend(
                        LoadBalancerNamesDeserializer::deserialize("LoadBalancerNames", stack)?,
                    );
                }
                "MaxInstanceLifetime" => {
                    obj.max_instance_lifetime = Some(MaxInstanceLifetimeDeserializer::deserialize(
                        "MaxInstanceLifetime",
                        stack,
                    )?);
                }
                "MaxSize" => {
                    obj.max_size =
                        AutoScalingGroupMaxSizeDeserializer::deserialize("MaxSize", stack)?;
                }
                "MinSize" => {
                    obj.min_size =
                        AutoScalingGroupMinSizeDeserializer::deserialize("MinSize", stack)?;
                }
                "MixedInstancesPolicy" => {
                    obj.mixed_instances_policy =
                        Some(MixedInstancesPolicyDeserializer::deserialize(
                            "MixedInstancesPolicy",
                            stack,
                        )?);
                }
                "NewInstancesProtectedFromScaleIn" => {
                    obj.new_instances_protected_from_scale_in =
                        Some(InstanceProtectedDeserializer::deserialize(
                            "NewInstancesProtectedFromScaleIn",
                            stack,
                        )?);
                }
                "PlacementGroup" => {
                    obj.placement_group = Some(XmlStringMaxLen255Deserializer::deserialize(
                        "PlacementGroup",
                        stack,
                    )?);
                }
                "ServiceLinkedRoleARN" => {
                    obj.service_linked_role_arn = Some(ResourceNameDeserializer::deserialize(
                        "ServiceLinkedRoleARN",
                        stack,
                    )?);
                }
                "Status" => {
                    obj.status = Some(XmlStringMaxLen255Deserializer::deserialize(
                        "Status", stack,
                    )?);
                }
                "SuspendedProcesses" => {
                    obj.suspended_processes.get_or_insert(vec![]).extend(
                        SuspendedProcessesDeserializer::deserialize("SuspendedProcesses", stack)?,
                    );
                }
                "Tags" => {
                    obj.tags
                        .get_or_insert(vec![])
                        .extend(TagDescriptionListDeserializer::deserialize("Tags", stack)?);
                }
                "TargetGroupARNs" => {
                    obj.target_group_ar_ns.get_or_insert(vec![]).extend(
                        TargetGroupARNsDeserializer::deserialize("TargetGroupARNs", stack)?,
                    );
                }
                "TerminationPolicies" => {
                    obj.termination_policies.get_or_insert(vec![]).extend(
                        TerminationPoliciesDeserializer::deserialize("TerminationPolicies", stack)?,
                    );
                }
                "VPCZoneIdentifier" => {
                    obj.vpc_zone_identifier = Some(XmlStringMaxLen2047Deserializer::deserialize(
                        "VPCZoneIdentifier",
                        stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct AutoScalingGroupDesiredCapacityDeserializer;
impl AutoScalingGroupDesiredCapacityDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = i64::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct AutoScalingGroupMaxSizeDeserializer;
impl AutoScalingGroupMaxSizeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = i64::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct AutoScalingGroupMinSizeDeserializer;
impl AutoScalingGroupMinSizeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = i64::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AutoScalingGroupNamesType {
    /// <p>The names of the Auto Scaling groups. Each name can be a maximum of 1600 characters. By default, you can only specify up to 50 names. You can optionally increase this limit using the <code>MaxRecords</code> parameter.</p> <p>If you omit this parameter, all Auto Scaling groups are described.</p>
    pub auto_scaling_group_names: Option<Vec<String>>,
    /// <p>The maximum number of items to return with this call. The default value is <code>50</code> and the maximum value is <code>100</code>.</p>
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
            params.put(&format!("{}{}", prefix, "MaxRecords"), &field_value);
        }
        if let Some(ref field_value) = obj.next_token {
            params.put(&format!("{}{}", prefix, "NextToken"), &field_value);
        }
    }
}

struct AutoScalingGroupsDeserializer;
impl AutoScalingGroupsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<AutoScalingGroup>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(AutoScalingGroupDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct AutoScalingGroupsType {
    /// <p>The groups.</p>
    pub auto_scaling_groups: Vec<AutoScalingGroup>,
    /// <p>A string that indicates that the response contains more items than can be returned in a single response. To receive additional items, specify this string for the <code>NextToken</code> value when requesting the next set of items. This value is null when there are no more items to return.</p>
    pub next_token: Option<String>,
}

struct AutoScalingGroupsTypeDeserializer;
impl AutoScalingGroupsTypeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AutoScalingGroupsType, XmlParseError> {
        deserialize_elements::<_, AutoScalingGroupsType, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "AutoScalingGroups" => {
                    obj.auto_scaling_groups
                        .extend(AutoScalingGroupsDeserializer::deserialize(
                            "AutoScalingGroups",
                            stack,
                        )?);
                }
                "NextToken" => {
                    obj.next_token = Some(XmlStringDeserializer::deserialize("NextToken", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
/// <p>Describes an EC2 instance associated with an Auto Scaling group.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct AutoScalingInstanceDetails {
    /// <p>The name of the Auto Scaling group for the instance.</p>
    pub auto_scaling_group_name: String,
    /// <p>The Availability Zone for the instance.</p>
    pub availability_zone: String,
    /// <p>The last reported health status of this instance. "Healthy" means that the instance is healthy and should remain in service. "Unhealthy" means that the instance is unhealthy and Amazon EC2 Auto Scaling should terminate and replace it.</p>
    pub health_status: String,
    /// <p>The ID of the instance.</p>
    pub instance_id: String,
    /// <p>The instance type of the EC2 instance.</p>
    pub instance_type: Option<String>,
    /// <p>The launch configuration used to launch the instance. This value is not available if you attached the instance to the Auto Scaling group.</p>
    pub launch_configuration_name: Option<String>,
    /// <p>The launch template for the instance.</p>
    pub launch_template: Option<LaunchTemplateSpecification>,
    /// <p>The lifecycle state for the instance.</p>
    pub lifecycle_state: String,
    /// <p>Indicates whether the instance is protected from termination by Amazon EC2 Auto Scaling when scaling in.</p>
    pub protected_from_scale_in: bool,
    /// <p>The number of capacity units contributed by the instance based on its instance type.</p> <p>Valid Range: Minimum value of 1. Maximum value of 999.</p>
    pub weighted_capacity: Option<String>,
}

struct AutoScalingInstanceDetailsDeserializer;
impl AutoScalingInstanceDetailsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AutoScalingInstanceDetails, XmlParseError> {
        deserialize_elements::<_, AutoScalingInstanceDetails, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "AutoScalingGroupName" => {
                        obj.auto_scaling_group_name = XmlStringMaxLen255Deserializer::deserialize(
                            "AutoScalingGroupName",
                            stack,
                        )?;
                    }
                    "AvailabilityZone" => {
                        obj.availability_zone =
                            XmlStringMaxLen255Deserializer::deserialize("AvailabilityZone", stack)?;
                    }
                    "HealthStatus" => {
                        obj.health_status =
                            XmlStringMaxLen32Deserializer::deserialize("HealthStatus", stack)?;
                    }
                    "InstanceId" => {
                        obj.instance_id =
                            XmlStringMaxLen19Deserializer::deserialize("InstanceId", stack)?;
                    }
                    "InstanceType" => {
                        obj.instance_type = Some(XmlStringMaxLen255Deserializer::deserialize(
                            "InstanceType",
                            stack,
                        )?);
                    }
                    "LaunchConfigurationName" => {
                        obj.launch_configuration_name =
                            Some(XmlStringMaxLen255Deserializer::deserialize(
                                "LaunchConfigurationName",
                                stack,
                            )?);
                    }
                    "LaunchTemplate" => {
                        obj.launch_template =
                            Some(LaunchTemplateSpecificationDeserializer::deserialize(
                                "LaunchTemplate",
                                stack,
                            )?);
                    }
                    "LifecycleState" => {
                        obj.lifecycle_state =
                            XmlStringMaxLen32Deserializer::deserialize("LifecycleState", stack)?;
                    }
                    "ProtectedFromScaleIn" => {
                        obj.protected_from_scale_in = InstanceProtectedDeserializer::deserialize(
                            "ProtectedFromScaleIn",
                            stack,
                        )?;
                    }
                    "WeightedCapacity" => {
                        obj.weighted_capacity = Some(XmlStringMaxLen32Deserializer::deserialize(
                            "WeightedCapacity",
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
struct AutoScalingInstancesDeserializer;
impl AutoScalingInstancesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<AutoScalingInstanceDetails>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(AutoScalingInstanceDetailsDeserializer::deserialize(
                    "member", stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct AutoScalingInstancesType {
    /// <p>The instances.</p>
    pub auto_scaling_instances: Option<Vec<AutoScalingInstanceDetails>>,
    /// <p>A string that indicates that the response contains more items than can be returned in a single response. To receive additional items, specify this string for the <code>NextToken</code> value when requesting the next set of items. This value is null when there are no more items to return.</p>
    pub next_token: Option<String>,
}

struct AutoScalingInstancesTypeDeserializer;
impl AutoScalingInstancesTypeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AutoScalingInstancesType, XmlParseError> {
        deserialize_elements::<_, AutoScalingInstancesType, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "AutoScalingInstances" => {
                        obj.auto_scaling_instances.get_or_insert(vec![]).extend(
                            AutoScalingInstancesDeserializer::deserialize(
                                "AutoScalingInstances",
                                stack,
                            )?,
                        );
                    }
                    "NextToken" => {
                        obj.next_token =
                            Some(XmlStringDeserializer::deserialize("NextToken", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
struct AutoScalingNotificationTypesDeserializer;
impl AutoScalingNotificationTypesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(XmlStringMaxLen255Deserializer::deserialize(
                    "member", stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
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
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(XmlStringMaxLen255Deserializer::deserialize(
                    "member", stack,
                )?);
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

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct BatchDeleteScheduledActionAnswer {
    /// <p>The names of the scheduled actions that could not be deleted, including an error message.</p>
    pub failed_scheduled_actions: Option<Vec<FailedScheduledUpdateGroupActionRequest>>,
}

struct BatchDeleteScheduledActionAnswerDeserializer;
impl BatchDeleteScheduledActionAnswerDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<BatchDeleteScheduledActionAnswer, XmlParseError> {
        deserialize_elements::<_, BatchDeleteScheduledActionAnswer, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "FailedScheduledActions" => {
                        obj.failed_scheduled_actions.get_or_insert(vec![]).extend(
                            FailedScheduledUpdateGroupActionRequestsDeserializer::deserialize(
                                "FailedScheduledActions",
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
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BatchDeleteScheduledActionType {
    /// <p>The name of the Auto Scaling group.</p>
    pub auto_scaling_group_name: String,
    /// <p>The names of the scheduled actions to delete. The maximum number allowed is 50. </p>
    pub scheduled_action_names: Vec<String>,
}

/// Serialize `BatchDeleteScheduledActionType` contents to a `SignedRequest`.
struct BatchDeleteScheduledActionTypeSerializer;
impl BatchDeleteScheduledActionTypeSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &BatchDeleteScheduledActionType) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "AutoScalingGroupName"),
            &obj.auto_scaling_group_name,
        );
        ScheduledActionNamesSerializer::serialize(
            params,
            &format!("{}{}", prefix, "ScheduledActionNames"),
            &obj.scheduled_action_names,
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct BatchPutScheduledUpdateGroupActionAnswer {
    /// <p>The names of the scheduled actions that could not be created or updated, including an error message.</p>
    pub failed_scheduled_update_group_actions: Option<Vec<FailedScheduledUpdateGroupActionRequest>>,
}

struct BatchPutScheduledUpdateGroupActionAnswerDeserializer;
impl BatchPutScheduledUpdateGroupActionAnswerDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<BatchPutScheduledUpdateGroupActionAnswer, XmlParseError> {
        deserialize_elements::<_, BatchPutScheduledUpdateGroupActionAnswer, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "FailedScheduledUpdateGroupActions" => {
                        obj.failed_scheduled_update_group_actions
                            .get_or_insert(vec![])
                            .extend(
                                FailedScheduledUpdateGroupActionRequestsDeserializer::deserialize(
                                    "FailedScheduledUpdateGroupActions",
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
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BatchPutScheduledUpdateGroupActionType {
    /// <p>The name of the Auto Scaling group.</p>
    pub auto_scaling_group_name: String,
    /// <p>One or more scheduled actions. The maximum number allowed is 50.</p>
    pub scheduled_update_group_actions: Vec<ScheduledUpdateGroupActionRequest>,
}

/// Serialize `BatchPutScheduledUpdateGroupActionType` contents to a `SignedRequest`.
struct BatchPutScheduledUpdateGroupActionTypeSerializer;
impl BatchPutScheduledUpdateGroupActionTypeSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &BatchPutScheduledUpdateGroupActionType) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "AutoScalingGroupName"),
            &obj.auto_scaling_group_name,
        );
        ScheduledUpdateGroupActionRequestsSerializer::serialize(
            params,
            &format!("{}{}", prefix, "ScheduledUpdateGroupActions"),
            &obj.scheduled_update_group_actions,
        );
    }
}

struct BlockDeviceEbsDeleteOnTerminationDeserializer;
impl BlockDeviceEbsDeleteOnTerminationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<bool, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = bool::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct BlockDeviceEbsEncryptedDeserializer;
impl BlockDeviceEbsEncryptedDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<bool, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = bool::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct BlockDeviceEbsIopsDeserializer;
impl BlockDeviceEbsIopsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = i64::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct BlockDeviceEbsVolumeSizeDeserializer;
impl BlockDeviceEbsVolumeSizeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = i64::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct BlockDeviceEbsVolumeTypeDeserializer;
impl BlockDeviceEbsVolumeTypeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Describes a block device mapping.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BlockDeviceMapping {
    /// <p>The device name exposed to the EC2 instance (for example, <code>/dev/sdh</code> or <code>xvdh</code>). For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/device_naming.html">Device Naming on Linux Instances</a> in the <i>Amazon EC2 User Guide for Linux Instances</i>.</p>
    pub device_name: String,
    /// <p>The information about the Amazon EBS volume.</p>
    pub ebs: Option<Ebs>,
    /// <p>Suppresses a device mapping.</p> <p>If this parameter is true for the root device, the instance might fail the EC2 health check. In that case, Amazon EC2 Auto Scaling launches a replacement instance.</p>
    pub no_device: Option<bool>,
    /// <p>The name of the virtual device (for example, <code>ephemeral0</code>).</p>
    pub virtual_name: Option<String>,
}

struct BlockDeviceMappingDeserializer;
impl BlockDeviceMappingDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<BlockDeviceMapping, XmlParseError> {
        deserialize_elements::<_, BlockDeviceMapping, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "DeviceName" => {
                    obj.device_name =
                        XmlStringMaxLen255Deserializer::deserialize("DeviceName", stack)?;
                }
                "Ebs" => {
                    obj.ebs = Some(EbsDeserializer::deserialize("Ebs", stack)?);
                }
                "NoDevice" => {
                    obj.no_device = Some(NoDeviceDeserializer::deserialize("NoDevice", stack)?);
                }
                "VirtualName" => {
                    obj.virtual_name = Some(XmlStringMaxLen255Deserializer::deserialize(
                        "VirtualName",
                        stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
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

        params.put(&format!("{}{}", prefix, "DeviceName"), &obj.device_name);
        if let Some(ref field_value) = obj.ebs {
            EbsSerializer::serialize(params, &format!("{}{}", prefix, "Ebs"), field_value);
        }
        if let Some(ref field_value) = obj.no_device {
            params.put(&format!("{}{}", prefix, "NoDevice"), &field_value);
        }
        if let Some(ref field_value) = obj.virtual_name {
            params.put(&format!("{}{}", prefix, "VirtualName"), &field_value);
        }
    }
}

struct BlockDeviceMappingsDeserializer;
impl BlockDeviceMappingsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<BlockDeviceMapping>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(BlockDeviceMappingDeserializer::deserialize(
                    "member", stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
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
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(XmlStringMaxLen255Deserializer::deserialize(
                    "member", stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
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
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CompleteLifecycleActionAnswer {}

struct CompleteLifecycleActionAnswerDeserializer;
impl CompleteLifecycleActionAnswerDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CompleteLifecycleActionAnswer, XmlParseError> {
        start_element(tag_name, stack)?;

        let obj = CompleteLifecycleActionAnswer::default();

        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CompleteLifecycleActionType {
    /// <p>The name of the Auto Scaling group.</p>
    pub auto_scaling_group_name: String,
    /// <p>The ID of the instance.</p>
    pub instance_id: Option<String>,
    /// <p>The action for the group to take. This parameter can be either <code>CONTINUE</code> or <code>ABANDON</code>.</p>
    pub lifecycle_action_result: String,
    /// <p>A universally unique identifier (UUID) that identifies a specific lifecycle action associated with an instance. Amazon EC2 Auto Scaling sends this token to the notification target you specified when you created the lifecycle hook.</p>
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
            &obj.auto_scaling_group_name,
        );
        if let Some(ref field_value) = obj.instance_id {
            params.put(&format!("{}{}", prefix, "InstanceId"), &field_value);
        }
        params.put(
            &format!("{}{}", prefix, "LifecycleActionResult"),
            &obj.lifecycle_action_result,
        );
        if let Some(ref field_value) = obj.lifecycle_action_token {
            params.put(
                &format!("{}{}", prefix, "LifecycleActionToken"),
                &field_value,
            );
        }
        params.put(
            &format!("{}{}", prefix, "LifecycleHookName"),
            &obj.lifecycle_hook_name,
        );
    }
}

struct CooldownDeserializer;
impl CooldownDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = i64::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateAutoScalingGroupType {
    /// <p>The name of the Auto Scaling group. This name must be unique per Region per account.</p>
    pub auto_scaling_group_name: String,
    /// <p>One or more Availability Zones for the group. This parameter is optional if you specify one or more subnets for <code>VPCZoneIdentifier</code>.</p> <p>Conditional: If your account supports EC2-Classic and VPC, this parameter is required to launch instances into EC2-Classic.</p>
    pub availability_zones: Option<Vec<String>>,
    /// <p>The amount of time, in seconds, after a scaling activity completes before another scaling activity can start. The default value is <code>300</code>.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/Cooldown.html">Scaling Cooldowns</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    pub default_cooldown: Option<i64>,
    /// <p>The number of Amazon EC2 instances that the Auto Scaling group attempts to maintain. This number must be greater than or equal to the minimum size of the group and less than or equal to the maximum size of the group. If you do not specify a desired capacity, the default is the minimum size of the group.</p>
    pub desired_capacity: Option<i64>,
    /// <p>The amount of time, in seconds, that Amazon EC2 Auto Scaling waits before checking the health status of an EC2 instance that has come into service. During this time, any health check failures for the instance are ignored. The default value is <code>0</code>.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/healthcheck.html#health-check-grace-period">Health Check Grace Period</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p> <p>Conditional: This parameter is required if you are adding an <code>ELB</code> health check.</p>
    pub health_check_grace_period: Option<i64>,
    /// <p>The service to use for the health checks. The valid values are <code>EC2</code> and <code>ELB</code>. The default value is <code>EC2</code>. If you configure an Auto Scaling group to use ELB health checks, it considers the instance unhealthy if it fails either the EC2 status checks or the load balancer health checks.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/healthcheck.html">Health Checks for Auto Scaling Instances</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    pub health_check_type: Option<String>,
    /// <p>The ID of the instance used to create a launch configuration for the group.</p> <p>When you specify an ID of an instance, Amazon EC2 Auto Scaling creates a new launch configuration and associates it with the group. This launch configuration derives its attributes from the specified instance, except for the block device mapping.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/create-asg-from-instance.html">Create an Auto Scaling Group Using an EC2 Instance</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p> <p>You must specify one of the following parameters in your request: <code>LaunchConfigurationName</code>, <code>LaunchTemplate</code>, <code>InstanceId</code>, or <code>MixedInstancesPolicy</code>.</p>
    pub instance_id: Option<String>,
    /// <p>The name of the launch configuration.</p> <p>If you do not specify <code>LaunchConfigurationName</code>, you must specify one of the following parameters: <code>InstanceId</code>, <code>LaunchTemplate</code>, or <code>MixedInstancesPolicy</code>.</p>
    pub launch_configuration_name: Option<String>,
    /// <p>The launch template to use to launch instances.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/APIReference/API_LaunchTemplateSpecification.html">LaunchTemplateSpecification</a> in the <i>Amazon EC2 Auto Scaling API Reference</i>.</p> <p>If you do not specify <code>LaunchTemplate</code>, you must specify one of the following parameters: <code>InstanceId</code>, <code>LaunchConfigurationName</code>, or <code>MixedInstancesPolicy</code>.</p>
    pub launch_template: Option<LaunchTemplateSpecification>,
    /// <p>One or more lifecycle hooks.</p>
    pub lifecycle_hook_specification_list: Option<Vec<LifecycleHookSpecification>>,
    /// <p>A list of Classic Load Balancers associated with this Auto Scaling group. For Application Load Balancers and Network Load Balancers, specify a list of target groups using the <code>TargetGroupARNs</code> property instead.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/autoscaling-load-balancer.html">Using a Load Balancer with an Auto Scaling Group</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    pub load_balancer_names: Option<Vec<String>>,
    /// <p>The maximum amount of time, in seconds, that an instance can be in service.</p> <p>Valid Range: Minimum value of 604800.</p>
    pub max_instance_lifetime: Option<i64>,
    /// <p>The maximum size of the group.</p>
    pub max_size: i64,
    /// <p>The minimum size of the group.</p>
    pub min_size: i64,
    /// <p>An embedded object that specifies a mixed instances policy. The required parameters must be specified. If optional parameters are unspecified, their default values are used.</p> <p>The policy includes parameters that not only define the distribution of On-Demand Instances and Spot Instances, the maximum price to pay for Spot Instances, and how the Auto Scaling group allocates instance types to fulfill On-Demand and Spot capacity, but also the parameters that specify the instance configuration information—the launch template and instance types.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/APIReference/API_MixedInstancesPolicy.html">MixedInstancesPolicy</a> in the <i>Amazon EC2 Auto Scaling API Reference</i> and <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/asg-purchase-options.html">Auto Scaling Groups with Multiple Instance Types and Purchase Options</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p> <p>You must specify one of the following parameters in your request: <code>LaunchConfigurationName</code>, <code>LaunchTemplate</code>, <code>InstanceId</code>, or <code>MixedInstancesPolicy</code>.</p>
    pub mixed_instances_policy: Option<MixedInstancesPolicy>,
    /// <p>Indicates whether newly launched instances are protected from termination by Amazon EC2 Auto Scaling when scaling in.</p> <p>For more information about preventing instances from terminating on scale in, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/as-instance-termination.html#instance-protection">Instance Protection</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    pub new_instances_protected_from_scale_in: Option<bool>,
    /// <p>The name of the placement group into which to launch your instances, if any. A placement group is a logical grouping of instances within a single Availability Zone. You cannot specify multiple Availability Zones and a placement group. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/placement-groups.html">Placement Groups</a> in the <i>Amazon EC2 User Guide for Linux Instances</i>.</p>
    pub placement_group: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the service-linked role that the Auto Scaling group uses to call other AWS services on your behalf. By default, Amazon EC2 Auto Scaling uses a service-linked role named AWSServiceRoleForAutoScaling, which it creates if it does not exist. For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/autoscaling-service-linked-role.html">Service-Linked Roles</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    pub service_linked_role_arn: Option<String>,
    /// <p>One or more tags.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/autoscaling-tagging.html">Tagging Auto Scaling Groups and Instances</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    pub tags: Option<Vec<Tag>>,
    /// <p>The Amazon Resource Names (ARN) of the target groups to associate with the Auto Scaling group. Instances are registered as targets in a target group, and traffic is routed to the target group.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/autoscaling-load-balancer.html">Using a Load Balancer with an Auto Scaling Group</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    pub target_group_ar_ns: Option<Vec<String>>,
    /// <p>One or more termination policies used to select the instance to terminate. These policies are executed in the order that they are listed.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/as-instance-termination.html">Controlling Which Instances Auto Scaling Terminates During Scale In</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    pub termination_policies: Option<Vec<String>>,
    /// <p>A comma-separated list of subnet IDs for your virtual private cloud (VPC).</p> <p>If you specify <code>VPCZoneIdentifier</code> with <code>AvailabilityZones</code>, the subnets that you specify for this parameter must reside in those Availability Zones.</p> <p>Conditional: If your account supports EC2-Classic and VPC, this parameter is required to launch instances into a VPC.</p>
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
            &obj.auto_scaling_group_name,
        );
        if let Some(ref field_value) = obj.availability_zones {
            AvailabilityZonesSerializer::serialize(
                params,
                &format!("{}{}", prefix, "AvailabilityZones"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.default_cooldown {
            params.put(&format!("{}{}", prefix, "DefaultCooldown"), &field_value);
        }
        if let Some(ref field_value) = obj.desired_capacity {
            params.put(&format!("{}{}", prefix, "DesiredCapacity"), &field_value);
        }
        if let Some(ref field_value) = obj.health_check_grace_period {
            params.put(
                &format!("{}{}", prefix, "HealthCheckGracePeriod"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.health_check_type {
            params.put(&format!("{}{}", prefix, "HealthCheckType"), &field_value);
        }
        if let Some(ref field_value) = obj.instance_id {
            params.put(&format!("{}{}", prefix, "InstanceId"), &field_value);
        }
        if let Some(ref field_value) = obj.launch_configuration_name {
            params.put(
                &format!("{}{}", prefix, "LaunchConfigurationName"),
                &field_value,
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
        if let Some(ref field_value) = obj.max_instance_lifetime {
            params.put(
                &format!("{}{}", prefix, "MaxInstanceLifetime"),
                &field_value,
            );
        }
        params.put(&format!("{}{}", prefix, "MaxSize"), &obj.max_size);
        params.put(&format!("{}{}", prefix, "MinSize"), &obj.min_size);
        if let Some(ref field_value) = obj.mixed_instances_policy {
            MixedInstancesPolicySerializer::serialize(
                params,
                &format!("{}{}", prefix, "MixedInstancesPolicy"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.new_instances_protected_from_scale_in {
            params.put(
                &format!("{}{}", prefix, "NewInstancesProtectedFromScaleIn"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.placement_group {
            params.put(&format!("{}{}", prefix, "PlacementGroup"), &field_value);
        }
        if let Some(ref field_value) = obj.service_linked_role_arn {
            params.put(
                &format!("{}{}", prefix, "ServiceLinkedRoleARN"),
                &field_value,
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
            params.put(&format!("{}{}", prefix, "VPCZoneIdentifier"), &field_value);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateLaunchConfigurationType {
    /// <p><p>For Auto Scaling groups that are running in a virtual private cloud (VPC), specifies whether to assign a public IP address to the group&#39;s instances. If you specify <code>true</code>, each instance in the Auto Scaling group receives a unique public IP address. For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/asg-in-vpc.html">Launching Auto Scaling Instances in a VPC</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p> <p>If you specify this parameter, you must specify at least one subnet for <code>VPCZoneIdentifier</code> when you create your group.</p> <note> <p>If the instance is launched into a default subnet, the default is to assign a public IP address, unless you disabled the option to assign a public IP address on the subnet. If the instance is launched into a nondefault subnet, the default is not to assign a public IP address, unless you enabled the option to assign a public IP address on the subnet.</p> </note></p>
    pub associate_public_ip_address: Option<bool>,
    /// <p>A block device mapping, which specifies the block devices for the instance. You can specify virtual devices and EBS volumes. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/block-device-mapping-concepts.html">Block Device Mapping</a> in the <i>Amazon EC2 User Guide for Linux Instances</i>.</p>
    pub block_device_mappings: Option<Vec<BlockDeviceMapping>>,
    /// <p>The ID of a ClassicLink-enabled VPC to link your EC2-Classic instances to. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/vpc-classiclink.html">ClassicLink</a> in the <i>Amazon EC2 User Guide for Linux Instances</i> and <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/asg-in-vpc.html#as-ClassicLink">Linking EC2-Classic Instances to a VPC</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p> <p>This parameter can only be used if you are launching EC2-Classic instances.</p>
    pub classic_link_vpc_id: Option<String>,
    /// <p>The IDs of one or more security groups for the specified ClassicLink-enabled VPC. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/vpc-classiclink.html">ClassicLink</a> in the <i>Amazon EC2 User Guide for Linux Instances</i> and <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/asg-in-vpc.html#as-ClassicLink">Linking EC2-Classic Instances to a VPC</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p> <p>If you specify the <code>ClassicLinkVPCId</code> parameter, you must specify this parameter.</p>
    pub classic_link_vpc_security_groups: Option<Vec<String>>,
    /// <p>Specifies whether the launch configuration is optimized for EBS I/O (<code>true</code>) or not (<code>false</code>). The optimization provides dedicated throughput to Amazon EBS and an optimized configuration stack to provide optimal I/O performance. This optimization is not available with all instance types. Additional fees are incurred when you enable EBS optimization for an instance type that is not EBS-optimized by default. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/EBSOptimized.html">Amazon EBS-Optimized Instances</a> in the <i>Amazon EC2 User Guide for Linux Instances</i>.</p> <p>The default value is <code>false</code>.</p>
    pub ebs_optimized: Option<bool>,
    /// <p>The name or the Amazon Resource Name (ARN) of the instance profile associated with the IAM role for the instance. The instance profile contains the IAM role.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/us-iam-role.html">IAM Role for Applications That Run on Amazon EC2 Instances</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    pub iam_instance_profile: Option<String>,
    /// <p>The ID of the Amazon Machine Image (AMI) that was assigned during registration. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/finding-an-ami.html">Finding an AMI</a> in the <i>Amazon EC2 User Guide for Linux Instances</i>.</p> <p>If you do not specify <code>InstanceId</code>, you must specify <code>ImageId</code>.</p>
    pub image_id: Option<String>,
    /// <p>The ID of the instance to use to create the launch configuration. The new launch configuration derives attributes from the instance, except for the block device mapping.</p> <p>To create a launch configuration with a block device mapping or override any other instance attributes, specify them as part of the same request.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/create-lc-with-instanceID.html">Create a Launch Configuration Using an EC2 Instance</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p> <p>If you do not specify <code>InstanceId</code>, you must specify both <code>ImageId</code> and <code>InstanceType</code>.</p>
    pub instance_id: Option<String>,
    /// <p><p>Controls whether instances in this group are launched with detailed (<code>true</code>) or basic (<code>false</code>) monitoring.</p> <p>The default value is <code>true</code> (enabled).</p> <important> <p>When detailed monitoring is enabled, Amazon CloudWatch generates metrics every minute and your account is charged a fee. When you disable detailed monitoring, CloudWatch generates metrics every 5 minutes. For more information, see <a href="https://docs.aws.amazon.com/autoscaling/latest/userguide/as-instance-monitoring.html#enable-as-instance-metrics">Configure Monitoring for Auto Scaling Instances</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p> </important></p>
    pub instance_monitoring: Option<InstanceMonitoring>,
    /// <p>Specifies the instance type of the EC2 instance.</p> <p>For information about available instance types, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/instance-types.html#AvailableInstanceTypes">Available Instance Types</a> in the <i>Amazon EC2 User Guide for Linux Instances.</i> </p> <p>If you do not specify <code>InstanceId</code>, you must specify <code>InstanceType</code>.</p>
    pub instance_type: Option<String>,
    /// <p>The ID of the kernel associated with the AMI.</p>
    pub kernel_id: Option<String>,
    /// <p>The name of the key pair. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-key-pairs.html">Amazon EC2 Key Pairs</a> in the <i>Amazon EC2 User Guide for Linux Instances</i>.</p>
    pub key_name: Option<String>,
    /// <p>The name of the launch configuration. This name must be unique per Region per account.</p>
    pub launch_configuration_name: String,
    /// <p>The tenancy of the instance. An instance with <code>dedicated</code> tenancy runs on isolated, single-tenant hardware and can only be launched into a VPC.</p> <p>To launch dedicated instances into a shared tenancy VPC (a VPC with the instance placement tenancy attribute set to <code>default</code>), you must set the value of this parameter to <code>dedicated</code>.</p> <p>If you specify <code>PlacementTenancy</code>, you must specify at least one subnet for <code>VPCZoneIdentifier</code> when you create your group.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/asg-in-vpc.html#as-vpc-tenancy">Instance Placement Tenancy</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p> <p>Valid Values: <code>default</code> | <code>dedicated</code> </p>
    pub placement_tenancy: Option<String>,
    /// <p>The ID of the RAM disk to select.</p>
    pub ramdisk_id: Option<String>,
    /// <p>A list that contains the security groups to assign to the instances in the Auto Scaling group.</p> <p>[EC2-VPC] Specify the security group IDs. For more information, see <a href="https://docs.aws.amazon.com/AmazonVPC/latest/UserGuide/VPC_SecurityGroups.html">Security Groups for Your VPC</a> in the <i>Amazon Virtual Private Cloud User Guide</i>.</p> <p>[EC2-Classic] Specify either the security group names or the security group IDs. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/using-network-security.html">Amazon EC2 Security Groups</a> in the <i>Amazon EC2 User Guide for Linux Instances</i>.</p>
    pub security_groups: Option<Vec<String>>,
    /// <p><p>The maximum hourly price to be paid for any Spot Instance launched to fulfill the request. Spot Instances are launched when the price you specify exceeds the current Spot price. For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/asg-launch-spot-instances.html">Launching Spot Instances in Your Auto Scaling Group</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p> <note> <p>When you change your maximum price by creating a new launch configuration, running instances will continue to run as long as the maximum price for those running instances is higher than the current Spot price.</p> </note></p>
    pub spot_price: Option<String>,
    /// <p>The Base64-encoded user data to make available to the launched EC2 instances. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-instance-metadata.html">Instance Metadata and User Data</a> in the <i>Amazon EC2 User Guide for Linux Instances</i>.</p>
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
                &field_value,
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
            params.put(&format!("{}{}", prefix, "ClassicLinkVPCId"), &field_value);
        }
        if let Some(ref field_value) = obj.classic_link_vpc_security_groups {
            ClassicLinkVPCSecurityGroupsSerializer::serialize(
                params,
                &format!("{}{}", prefix, "ClassicLinkVPCSecurityGroups"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.ebs_optimized {
            params.put(&format!("{}{}", prefix, "EbsOptimized"), &field_value);
        }
        if let Some(ref field_value) = obj.iam_instance_profile {
            params.put(&format!("{}{}", prefix, "IamInstanceProfile"), &field_value);
        }
        if let Some(ref field_value) = obj.image_id {
            params.put(&format!("{}{}", prefix, "ImageId"), &field_value);
        }
        if let Some(ref field_value) = obj.instance_id {
            params.put(&format!("{}{}", prefix, "InstanceId"), &field_value);
        }
        if let Some(ref field_value) = obj.instance_monitoring {
            InstanceMonitoringSerializer::serialize(
                params,
                &format!("{}{}", prefix, "InstanceMonitoring"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.instance_type {
            params.put(&format!("{}{}", prefix, "InstanceType"), &field_value);
        }
        if let Some(ref field_value) = obj.kernel_id {
            params.put(&format!("{}{}", prefix, "KernelId"), &field_value);
        }
        if let Some(ref field_value) = obj.key_name {
            params.put(&format!("{}{}", prefix, "KeyName"), &field_value);
        }
        params.put(
            &format!("{}{}", prefix, "LaunchConfigurationName"),
            &obj.launch_configuration_name,
        );
        if let Some(ref field_value) = obj.placement_tenancy {
            params.put(&format!("{}{}", prefix, "PlacementTenancy"), &field_value);
        }
        if let Some(ref field_value) = obj.ramdisk_id {
            params.put(&format!("{}{}", prefix, "RamdiskId"), &field_value);
        }
        if let Some(ref field_value) = obj.security_groups {
            SecurityGroupsSerializer::serialize(
                params,
                &format!("{}{}", prefix, "SecurityGroups"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.spot_price {
            params.put(&format!("{}{}", prefix, "SpotPrice"), &field_value);
        }
        if let Some(ref field_value) = obj.user_data {
            params.put(&format!("{}{}", prefix, "UserData"), &field_value);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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

/// <p>Represents a CloudWatch metric of your choosing for a target tracking scaling policy to use with Amazon EC2 Auto Scaling.</p> <p>To create your customized metric specification:</p> <ul> <li> <p>Add values for each required parameter from CloudWatch. You can use an existing metric, or a new metric that you create. To use your own metric, you must first publish the metric to CloudWatch. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/publishingMetrics.html">Publish Custom Metrics</a> in the <i>Amazon CloudWatch User Guide</i>.</p> </li> <li> <p>Choose a metric that changes proportionally with capacity. The value of the metric should increase or decrease in inverse proportion to the number of capacity units. That is, the value of the metric should decrease when capacity increases.</p> </li> </ul> <p>For more information about CloudWatch, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/cloudwatch_concepts.html">Amazon CloudWatch Concepts</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CustomizedMetricSpecification {
    /// <p>The dimensions of the metric.</p> <p>Conditional: If you published your metric with dimensions, you must specify the same dimensions in your scaling policy.</p>
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
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CustomizedMetricSpecification, XmlParseError> {
        deserialize_elements::<_, CustomizedMetricSpecification, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Dimensions" => {
                        obj.dimensions.get_or_insert(vec![]).extend(
                            MetricDimensionsDeserializer::deserialize("Dimensions", stack)?,
                        );
                    }
                    "MetricName" => {
                        obj.metric_name = MetricNameDeserializer::deserialize("MetricName", stack)?;
                    }
                    "Namespace" => {
                        obj.namespace =
                            MetricNamespaceDeserializer::deserialize("Namespace", stack)?;
                    }
                    "Statistic" => {
                        obj.statistic =
                            MetricStatisticDeserializer::deserialize("Statistic", stack)?;
                    }
                    "Unit" => {
                        obj.unit = Some(MetricUnitDeserializer::deserialize("Unit", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
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
        params.put(&format!("{}{}", prefix, "MetricName"), &obj.metric_name);
        params.put(&format!("{}{}", prefix, "Namespace"), &obj.namespace);
        params.put(&format!("{}{}", prefix, "Statistic"), &obj.statistic);
        if let Some(ref field_value) = obj.unit {
            params.put(&format!("{}{}", prefix, "Unit"), &field_value);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteAutoScalingGroupType {
    /// <p>The name of the Auto Scaling group.</p>
    pub auto_scaling_group_name: String,
    /// <p>Specifies that the group is to be deleted along with all instances associated with the group, without waiting for all instances to be terminated. This parameter also deletes any lifecycle actions associated with the group.</p>
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
            &obj.auto_scaling_group_name,
        );
        if let Some(ref field_value) = obj.force_delete {
            params.put(&format!("{}{}", prefix, "ForceDelete"), &field_value);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DeleteLifecycleHookAnswer {}

struct DeleteLifecycleHookAnswerDeserializer;
impl DeleteLifecycleHookAnswerDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteLifecycleHookAnswer, XmlParseError> {
        start_element(tag_name, stack)?;

        let obj = DeleteLifecycleHookAnswer::default();

        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
            &obj.auto_scaling_group_name,
        );
        params.put(
            &format!("{}{}", prefix, "LifecycleHookName"),
            &obj.lifecycle_hook_name,
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteNotificationConfigurationType {
    /// <p>The name of the Auto Scaling group.</p>
    pub auto_scaling_group_name: String,
    /// <p>The Amazon Resource Name (ARN) of the Amazon Simple Notification Service (Amazon SNS) topic.</p>
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
            &obj.auto_scaling_group_name,
        );
        params.put(&format!("{}{}", prefix, "TopicARN"), &obj.topic_arn);
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
                &field_value,
            );
        }
        params.put(&format!("{}{}", prefix, "PolicyName"), &obj.policy_name);
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
            &obj.auto_scaling_group_name,
        );
        params.put(
            &format!("{}{}", prefix, "ScheduledActionName"),
            &obj.scheduled_action_name,
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DescribeAccountLimitsAnswer {
    /// <p>The maximum number of groups allowed for your AWS account. The default limit is 200 per AWS Region.</p>
    pub max_number_of_auto_scaling_groups: Option<i64>,
    /// <p>The maximum number of launch configurations allowed for your AWS account. The default limit is 200 per AWS Region.</p>
    pub max_number_of_launch_configurations: Option<i64>,
    /// <p>The current number of groups for your AWS account.</p>
    pub number_of_auto_scaling_groups: Option<i64>,
    /// <p>The current number of launch configurations for your AWS account.</p>
    pub number_of_launch_configurations: Option<i64>,
}

struct DescribeAccountLimitsAnswerDeserializer;
impl DescribeAccountLimitsAnswerDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeAccountLimitsAnswer, XmlParseError> {
        deserialize_elements::<_, DescribeAccountLimitsAnswer, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "MaxNumberOfAutoScalingGroups" => {
                        obj.max_number_of_auto_scaling_groups =
                            Some(MaxNumberOfAutoScalingGroupsDeserializer::deserialize(
                                "MaxNumberOfAutoScalingGroups",
                                stack,
                            )?);
                    }
                    "MaxNumberOfLaunchConfigurations" => {
                        obj.max_number_of_launch_configurations =
                            Some(MaxNumberOfLaunchConfigurationsDeserializer::deserialize(
                                "MaxNumberOfLaunchConfigurations",
                                stack,
                            )?);
                    }
                    "NumberOfAutoScalingGroups" => {
                        obj.number_of_auto_scaling_groups =
                            Some(NumberOfAutoScalingGroupsDeserializer::deserialize(
                                "NumberOfAutoScalingGroups",
                                stack,
                            )?);
                    }
                    "NumberOfLaunchConfigurations" => {
                        obj.number_of_launch_configurations =
                            Some(NumberOfLaunchConfigurationsDeserializer::deserialize(
                                "NumberOfLaunchConfigurations",
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
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DescribeAdjustmentTypesAnswer {
    /// <p>The policy adjustment types.</p>
    pub adjustment_types: Option<Vec<AdjustmentType>>,
}

struct DescribeAdjustmentTypesAnswerDeserializer;
impl DescribeAdjustmentTypesAnswerDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeAdjustmentTypesAnswer, XmlParseError> {
        deserialize_elements::<_, DescribeAdjustmentTypesAnswer, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "AdjustmentTypes" => {
                        obj.adjustment_types.get_or_insert(vec![]).extend(
                            AdjustmentTypesDeserializer::deserialize("AdjustmentTypes", stack)?,
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
pub struct DescribeAutoScalingInstancesType {
    /// <p>The IDs of the instances. You can specify up to <code>MaxRecords</code> IDs. If you omit this parameter, all Auto Scaling instances are described. If you specify an ID that does not exist, it is ignored with no error.</p>
    pub instance_ids: Option<Vec<String>>,
    /// <p>The maximum number of items to return with this call. The default value is <code>50</code> and the maximum value is <code>50</code>.</p>
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
            params.put(&format!("{}{}", prefix, "MaxRecords"), &field_value);
        }
        if let Some(ref field_value) = obj.next_token {
            params.put(&format!("{}{}", prefix, "NextToken"), &field_value);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DescribeAutoScalingNotificationTypesAnswer {
    /// <p>The notification types.</p>
    pub auto_scaling_notification_types: Option<Vec<String>>,
}

struct DescribeAutoScalingNotificationTypesAnswerDeserializer;
impl DescribeAutoScalingNotificationTypesAnswerDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeAutoScalingNotificationTypesAnswer, XmlParseError> {
        deserialize_elements::<_, DescribeAutoScalingNotificationTypesAnswer, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "AutoScalingNotificationTypes" => {
                        obj.auto_scaling_notification_types
                            .get_or_insert(vec![])
                            .extend(AutoScalingNotificationTypesDeserializer::deserialize(
                                "AutoScalingNotificationTypes",
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
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DescribeLifecycleHookTypesAnswer {
    /// <p>The lifecycle hook types.</p>
    pub lifecycle_hook_types: Option<Vec<String>>,
}

struct DescribeLifecycleHookTypesAnswerDeserializer;
impl DescribeLifecycleHookTypesAnswerDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeLifecycleHookTypesAnswer, XmlParseError> {
        deserialize_elements::<_, DescribeLifecycleHookTypesAnswer, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "LifecycleHookTypes" => {
                        obj.lifecycle_hook_types.get_or_insert(vec![]).extend(
                            AutoScalingNotificationTypesDeserializer::deserialize(
                                "LifecycleHookTypes",
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
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DescribeLifecycleHooksAnswer {
    /// <p>The lifecycle hooks for the specified group.</p>
    pub lifecycle_hooks: Option<Vec<LifecycleHook>>,
}

struct DescribeLifecycleHooksAnswerDeserializer;
impl DescribeLifecycleHooksAnswerDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeLifecycleHooksAnswer, XmlParseError> {
        deserialize_elements::<_, DescribeLifecycleHooksAnswer, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "LifecycleHooks" => {
                        obj.lifecycle_hooks.get_or_insert(vec![]).extend(
                            LifecycleHooksDeserializer::deserialize("LifecycleHooks", stack)?,
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
            &obj.auto_scaling_group_name,
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeLoadBalancerTargetGroupsRequest {
    /// <p>The name of the Auto Scaling group.</p>
    pub auto_scaling_group_name: String,
    /// <p>The maximum number of items to return with this call. The default value is <code>100</code> and the maximum value is <code>100</code>.</p>
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
            &obj.auto_scaling_group_name,
        );
        if let Some(ref field_value) = obj.max_records {
            params.put(&format!("{}{}", prefix, "MaxRecords"), &field_value);
        }
        if let Some(ref field_value) = obj.next_token {
            params.put(&format!("{}{}", prefix, "NextToken"), &field_value);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DescribeLoadBalancerTargetGroupsResponse {
    /// <p>Information about the target groups.</p>
    pub load_balancer_target_groups: Option<Vec<LoadBalancerTargetGroupState>>,
    /// <p>A string that indicates that the response contains more items than can be returned in a single response. To receive additional items, specify this string for the <code>NextToken</code> value when requesting the next set of items. This value is null when there are no more items to return.</p>
    pub next_token: Option<String>,
}

struct DescribeLoadBalancerTargetGroupsResponseDeserializer;
impl DescribeLoadBalancerTargetGroupsResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeLoadBalancerTargetGroupsResponse, XmlParseError> {
        deserialize_elements::<_, DescribeLoadBalancerTargetGroupsResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "LoadBalancerTargetGroups" => {
                        obj.load_balancer_target_groups
                            .get_or_insert(vec![])
                            .extend(LoadBalancerTargetGroupStatesDeserializer::deserialize(
                                "LoadBalancerTargetGroups",
                                stack,
                            )?);
                    }
                    "NextToken" => {
                        obj.next_token =
                            Some(XmlStringDeserializer::deserialize("NextToken", stack)?);
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
pub struct DescribeLoadBalancersRequest {
    /// <p>The name of the Auto Scaling group.</p>
    pub auto_scaling_group_name: String,
    /// <p>The maximum number of items to return with this call. The default value is <code>100</code> and the maximum value is <code>100</code>.</p>
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
            &obj.auto_scaling_group_name,
        );
        if let Some(ref field_value) = obj.max_records {
            params.put(&format!("{}{}", prefix, "MaxRecords"), &field_value);
        }
        if let Some(ref field_value) = obj.next_token {
            params.put(&format!("{}{}", prefix, "NextToken"), &field_value);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DescribeLoadBalancersResponse {
    /// <p>The load balancers.</p>
    pub load_balancers: Option<Vec<LoadBalancerState>>,
    /// <p>A string that indicates that the response contains more items than can be returned in a single response. To receive additional items, specify this string for the <code>NextToken</code> value when requesting the next set of items. This value is null when there are no more items to return.</p>
    pub next_token: Option<String>,
}

struct DescribeLoadBalancersResponseDeserializer;
impl DescribeLoadBalancersResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeLoadBalancersResponse, XmlParseError> {
        deserialize_elements::<_, DescribeLoadBalancersResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "LoadBalancers" => {
                        obj.load_balancers.get_or_insert(vec![]).extend(
                            LoadBalancerStatesDeserializer::deserialize("LoadBalancers", stack)?,
                        );
                    }
                    "NextToken" => {
                        obj.next_token =
                            Some(XmlStringDeserializer::deserialize("NextToken", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DescribeMetricCollectionTypesAnswer {
    /// <p>The granularities for the metrics.</p>
    pub granularities: Option<Vec<MetricGranularityType>>,
    /// <p>One or more metrics.</p>
    pub metrics: Option<Vec<MetricCollectionType>>,
}

struct DescribeMetricCollectionTypesAnswerDeserializer;
impl DescribeMetricCollectionTypesAnswerDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeMetricCollectionTypesAnswer, XmlParseError> {
        deserialize_elements::<_, DescribeMetricCollectionTypesAnswer, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Granularities" => {
                        obj.granularities.get_or_insert(vec![]).extend(
                            MetricGranularityTypesDeserializer::deserialize(
                                "Granularities",
                                stack,
                            )?,
                        );
                    }
                    "Metrics" => {
                        obj.metrics.get_or_insert(vec![]).extend(
                            MetricCollectionTypesDeserializer::deserialize("Metrics", stack)?,
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
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DescribeNotificationConfigurationsAnswer {
    /// <p>A string that indicates that the response contains more items than can be returned in a single response. To receive additional items, specify this string for the <code>NextToken</code> value when requesting the next set of items. This value is null when there are no more items to return.</p>
    pub next_token: Option<String>,
    /// <p>The notification configurations.</p>
    pub notification_configurations: Vec<NotificationConfiguration>,
}

struct DescribeNotificationConfigurationsAnswerDeserializer;
impl DescribeNotificationConfigurationsAnswerDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeNotificationConfigurationsAnswer, XmlParseError> {
        deserialize_elements::<_, DescribeNotificationConfigurationsAnswer, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "NextToken" => {
                        obj.next_token =
                            Some(XmlStringDeserializer::deserialize("NextToken", stack)?);
                    }
                    "NotificationConfigurations" => {
                        obj.notification_configurations.extend(
                            NotificationConfigurationsDeserializer::deserialize(
                                "NotificationConfigurations",
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
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeNotificationConfigurationsType {
    /// <p>The name of the Auto Scaling group.</p>
    pub auto_scaling_group_names: Option<Vec<String>>,
    /// <p>The maximum number of items to return with this call. The default value is <code>50</code> and the maximum value is <code>100</code>.</p>
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
            params.put(&format!("{}{}", prefix, "MaxRecords"), &field_value);
        }
        if let Some(ref field_value) = obj.next_token {
            params.put(&format!("{}{}", prefix, "NextToken"), &field_value);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribePoliciesType {
    /// <p>The name of the Auto Scaling group.</p>
    pub auto_scaling_group_name: Option<String>,
    /// <p>The maximum number of items to be returned with each call. The default value is <code>50</code> and the maximum value is <code>100</code>.</p>
    pub max_records: Option<i64>,
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    pub next_token: Option<String>,
    /// <p>The names of one or more policies. If you omit this parameter, all policies are described. If a group name is provided, the results are limited to that group. This list is limited to 50 items. If you specify an unknown policy name, it is ignored with no error.</p>
    pub policy_names: Option<Vec<String>>,
    /// <p>One or more policy types. The valid values are <code>SimpleScaling</code>, <code>StepScaling</code>, and <code>TargetTrackingScaling</code>.</p>
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
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.max_records {
            params.put(&format!("{}{}", prefix, "MaxRecords"), &field_value);
        }
        if let Some(ref field_value) = obj.next_token {
            params.put(&format!("{}{}", prefix, "NextToken"), &field_value);
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeScalingActivitiesType {
    /// <p>The activity IDs of the desired scaling activities. You can specify up to 50 IDs. If you omit this parameter, all activities for the past six weeks are described. If unknown activities are requested, they are ignored with no error. If you specify an Auto Scaling group, the results are limited to that group.</p>
    pub activity_ids: Option<Vec<String>>,
    /// <p>The name of the Auto Scaling group.</p>
    pub auto_scaling_group_name: Option<String>,
    /// <p>The maximum number of items to return with this call. The default value is <code>100</code> and the maximum value is <code>100</code>.</p>
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
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.max_records {
            params.put(&format!("{}{}", prefix, "MaxRecords"), &field_value);
        }
        if let Some(ref field_value) = obj.next_token {
            params.put(&format!("{}{}", prefix, "NextToken"), &field_value);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeScheduledActionsType {
    /// <p>The name of the Auto Scaling group.</p>
    pub auto_scaling_group_name: Option<String>,
    /// <p>The latest scheduled start time to return. If scheduled action names are provided, this parameter is ignored.</p>
    pub end_time: Option<String>,
    /// <p>The maximum number of items to return with this call. The default value is <code>50</code> and the maximum value is <code>100</code>.</p>
    pub max_records: Option<i64>,
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    pub next_token: Option<String>,
    /// <p>The names of one or more scheduled actions. You can specify up to 50 actions. If you omit this parameter, all scheduled actions are described. If you specify an unknown scheduled action, it is ignored with no error.</p>
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
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.end_time {
            params.put(&format!("{}{}", prefix, "EndTime"), &field_value);
        }
        if let Some(ref field_value) = obj.max_records {
            params.put(&format!("{}{}", prefix, "MaxRecords"), &field_value);
        }
        if let Some(ref field_value) = obj.next_token {
            params.put(&format!("{}{}", prefix, "NextToken"), &field_value);
        }
        if let Some(ref field_value) = obj.scheduled_action_names {
            ScheduledActionNamesSerializer::serialize(
                params,
                &format!("{}{}", prefix, "ScheduledActionNames"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.start_time {
            params.put(&format!("{}{}", prefix, "StartTime"), &field_value);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeTagsType {
    /// <p>One or more filters to scope the tags to return. The maximum number of filters per filter type (for example, <code>auto-scaling-group</code>) is 1000.</p>
    pub filters: Option<Vec<Filter>>,
    /// <p>The maximum number of items to return with this call. The default value is <code>50</code> and the maximum value is <code>100</code>.</p>
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
            params.put(&format!("{}{}", prefix, "MaxRecords"), &field_value);
        }
        if let Some(ref field_value) = obj.next_token {
            params.put(&format!("{}{}", prefix, "NextToken"), &field_value);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DescribeTerminationPolicyTypesAnswer {
    /// <p>The termination policies supported by Amazon EC2 Auto Scaling: <code>OldestInstance</code>, <code>OldestLaunchConfiguration</code>, <code>NewestInstance</code>, <code>ClosestToNextInstanceHour</code>, <code>Default</code>, <code>OldestLaunchTemplate</code>, and <code>AllocationStrategy</code>.</p>
    pub termination_policy_types: Option<Vec<String>>,
}

struct DescribeTerminationPolicyTypesAnswerDeserializer;
impl DescribeTerminationPolicyTypesAnswerDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeTerminationPolicyTypesAnswer, XmlParseError> {
        deserialize_elements::<_, DescribeTerminationPolicyTypesAnswer, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "TerminationPolicyTypes" => {
                        obj.termination_policy_types.get_or_insert(vec![]).extend(
                            TerminationPoliciesDeserializer::deserialize(
                                "TerminationPolicyTypes",
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
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DetachInstancesAnswer {
    /// <p>The activities related to detaching the instances from the Auto Scaling group.</p>
    pub activities: Option<Vec<Activity>>,
}

struct DetachInstancesAnswerDeserializer;
impl DetachInstancesAnswerDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DetachInstancesAnswer, XmlParseError> {
        deserialize_elements::<_, DetachInstancesAnswer, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Activities" => {
                    obj.activities
                        .get_or_insert(vec![])
                        .extend(ActivitiesDeserializer::deserialize("Activities", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
            &obj.auto_scaling_group_name,
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
            &obj.should_decrement_desired_capacity,
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DetachLoadBalancerTargetGroupsResultType {}

struct DetachLoadBalancerTargetGroupsResultTypeDeserializer;
impl DetachLoadBalancerTargetGroupsResultTypeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DetachLoadBalancerTargetGroupsResultType, XmlParseError> {
        start_element(tag_name, stack)?;

        let obj = DetachLoadBalancerTargetGroupsResultType::default();

        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
            &obj.auto_scaling_group_name,
        );
        TargetGroupARNsSerializer::serialize(
            params,
            &format!("{}{}", prefix, "TargetGroupARNs"),
            &obj.target_group_ar_ns,
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DetachLoadBalancersResultType {}

struct DetachLoadBalancersResultTypeDeserializer;
impl DetachLoadBalancersResultTypeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DetachLoadBalancersResultType, XmlParseError> {
        start_element(tag_name, stack)?;

        let obj = DetachLoadBalancersResultType::default();

        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
            &obj.auto_scaling_group_name,
        );
        LoadBalancerNamesSerializer::serialize(
            params,
            &format!("{}{}", prefix, "LoadBalancerNames"),
            &obj.load_balancer_names,
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
            &obj.auto_scaling_group_name,
        );
        if let Some(ref field_value) = obj.metrics {
            MetricsSerializer::serialize(params, &format!("{}{}", prefix, "Metrics"), field_value);
        }
    }
}

struct DisableScaleInDeserializer;
impl DisableScaleInDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<bool, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = bool::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Describes an Amazon EBS volume. Used in combination with <a>BlockDeviceMapping</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Ebs {
    /// <p>Indicates whether the volume is deleted on instance termination. For Amazon EC2 Auto Scaling, the default value is <code>true</code>.</p>
    pub delete_on_termination: Option<bool>,
    /// <p>Specifies whether the volume should be encrypted. Encrypted EBS volumes can only be attached to instances that support Amazon EBS encryption. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/EBSEncryption.html#EBSEncryption_supported_instances">Supported Instance Types</a>. If your AMI uses encrypted volumes, you can also only launch it on supported instance types.</p> <note> <p>If you are creating a volume from a snapshot, you cannot specify an encryption value. Volumes that are created from encrypted snapshots are automatically encrypted, and volumes that are created from unencrypted snapshots are automatically unencrypted. By default, encrypted snapshots use the AWS managed CMK that is used for EBS encryption, but you can specify a custom CMK when you create the snapshot. The ability to encrypt a snapshot during copying also allows you to apply a new CMK to an already-encrypted snapshot. Volumes restored from the resulting copy are only accessible using the new CMK.</p> <p>Enabling <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/EBSEncryption.html#encryption-by-default">encryption by default</a> results in all EBS volumes being encrypted with the AWS managed CMK or a customer managed CMK, whether or not the snapshot was encrypted.</p> </note> <p>For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/AMIEncryption.html">Using Encryption with EBS-Backed AMIs</a> in the <i>Amazon EC2 User Guide for Linux Instances</i> and <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/key-policy-requirements-EBS-encryption.html">Required CMK Key Policy for Use with Encrypted Volumes</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    pub encrypted: Option<bool>,
    /// <p>The number of I/O operations per second (IOPS) to provision for the volume. The maximum ratio of IOPS to volume size (in GiB) is 50:1. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/EBSVolumeTypes.html">Amazon EBS Volume Types</a> in the <i>Amazon EC2 User Guide for Linux Instances</i>.</p> <p>Conditional: This parameter is required when the volume type is <code>io1</code>. (Not used with <code>standard</code>, <code>gp2</code>, <code>st1</code>, or <code>sc1</code> volumes.) </p>
    pub iops: Option<i64>,
    /// <p>The snapshot ID of the volume to use.</p> <p>Conditional: This parameter is optional if you specify a volume size. If you specify both <code>SnapshotId</code> and <code>VolumeSize</code>, <code>VolumeSize</code> must be equal or greater than the size of the snapshot.</p>
    pub snapshot_id: Option<String>,
    /// <p><p>The volume size, in Gibibytes (GiB).</p> <p>This can be a number from 1-1,024 for <code>standard</code>, 4-16,384 for <code>io1</code>, 1-16,384 for <code>gp2</code>, and 500-16,384 for <code>st1</code> and <code>sc1</code>. If you specify a snapshot, the volume size must be equal to or larger than the snapshot size.</p> <p>Default: If you create a volume from a snapshot and you don&#39;t specify a volume size, the default is the snapshot size.</p> <note> <p>At least one of VolumeSize or SnapshotId is required.</p> </note></p>
    pub volume_size: Option<i64>,
    /// <p>The volume type, which can be <code>standard</code> for Magnetic, <code>io1</code> for Provisioned IOPS SSD, <code>gp2</code> for General Purpose SSD, <code>st1</code> for Throughput Optimized HDD, or <code>sc1</code> for Cold HDD. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/EBSVolumeTypes.html">Amazon EBS Volume Types</a> in the <i>Amazon EC2 User Guide for Linux Instances</i>.</p> <p>Valid Values: <code>standard</code> | <code>io1</code> | <code>gp2</code> | <code>st1</code> | <code>sc1</code> </p>
    pub volume_type: Option<String>,
}

struct EbsDeserializer;
impl EbsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Ebs, XmlParseError> {
        deserialize_elements::<_, Ebs, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "DeleteOnTermination" => {
                    obj.delete_on_termination =
                        Some(BlockDeviceEbsDeleteOnTerminationDeserializer::deserialize(
                            "DeleteOnTermination",
                            stack,
                        )?);
                }
                "Encrypted" => {
                    obj.encrypted = Some(BlockDeviceEbsEncryptedDeserializer::deserialize(
                        "Encrypted",
                        stack,
                    )?);
                }
                "Iops" => {
                    obj.iops = Some(BlockDeviceEbsIopsDeserializer::deserialize("Iops", stack)?);
                }
                "SnapshotId" => {
                    obj.snapshot_id = Some(XmlStringMaxLen255Deserializer::deserialize(
                        "SnapshotId",
                        stack,
                    )?);
                }
                "VolumeSize" => {
                    obj.volume_size = Some(BlockDeviceEbsVolumeSizeDeserializer::deserialize(
                        "VolumeSize",
                        stack,
                    )?);
                }
                "VolumeType" => {
                    obj.volume_type = Some(BlockDeviceEbsVolumeTypeDeserializer::deserialize(
                        "VolumeType",
                        stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
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
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.encrypted {
            params.put(&format!("{}{}", prefix, "Encrypted"), &field_value);
        }
        if let Some(ref field_value) = obj.iops {
            params.put(&format!("{}{}", prefix, "Iops"), &field_value);
        }
        if let Some(ref field_value) = obj.snapshot_id {
            params.put(&format!("{}{}", prefix, "SnapshotId"), &field_value);
        }
        if let Some(ref field_value) = obj.volume_size {
            params.put(&format!("{}{}", prefix, "VolumeSize"), &field_value);
        }
        if let Some(ref field_value) = obj.volume_type {
            params.put(&format!("{}{}", prefix, "VolumeType"), &field_value);
        }
    }
}

struct EbsOptimizedDeserializer;
impl EbsOptimizedDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<bool, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = bool::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
            &obj.auto_scaling_group_name,
        );
        params.put(&format!("{}{}", prefix, "Granularity"), &obj.granularity);
        if let Some(ref field_value) = obj.metrics {
            MetricsSerializer::serialize(params, &format!("{}{}", prefix, "Metrics"), field_value);
        }
    }
}

/// <p>Describes an enabled metric.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct EnabledMetric {
    /// <p>The granularity of the metric. The only valid value is <code>1Minute</code>.</p>
    pub granularity: Option<String>,
    /// <p><p>One of the following metrics:</p> <ul> <li> <p> <code>GroupMinSize</code> </p> </li> <li> <p> <code>GroupMaxSize</code> </p> </li> <li> <p> <code>GroupDesiredCapacity</code> </p> </li> <li> <p> <code>GroupInServiceInstances</code> </p> </li> <li> <p> <code>GroupPendingInstances</code> </p> </li> <li> <p> <code>GroupStandbyInstances</code> </p> </li> <li> <p> <code>GroupTerminatingInstances</code> </p> </li> <li> <p> <code>GroupTotalInstances</code> </p> </li> </ul></p>
    pub metric: Option<String>,
}

struct EnabledMetricDeserializer;
impl EnabledMetricDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<EnabledMetric, XmlParseError> {
        deserialize_elements::<_, EnabledMetric, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Granularity" => {
                    obj.granularity = Some(XmlStringMaxLen255Deserializer::deserialize(
                        "Granularity",
                        stack,
                    )?);
                }
                "Metric" => {
                    obj.metric = Some(XmlStringMaxLen255Deserializer::deserialize(
                        "Metric", stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct EnabledMetricsDeserializer;
impl EnabledMetricsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<EnabledMetric>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(EnabledMetricDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct EnterStandbyAnswer {
    /// <p>The activities related to moving instances into <code>Standby</code> mode.</p>
    pub activities: Option<Vec<Activity>>,
}

struct EnterStandbyAnswerDeserializer;
impl EnterStandbyAnswerDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<EnterStandbyAnswer, XmlParseError> {
        deserialize_elements::<_, EnterStandbyAnswer, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Activities" => {
                    obj.activities
                        .get_or_insert(vec![])
                        .extend(ActivitiesDeserializer::deserialize("Activities", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
            &obj.auto_scaling_group_name,
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
            &obj.should_decrement_desired_capacity,
        );
    }
}

struct EstimatedInstanceWarmupDeserializer;
impl EstimatedInstanceWarmupDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = i64::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ExecutePolicyType {
    /// <p>The name of the Auto Scaling group.</p>
    pub auto_scaling_group_name: Option<String>,
    /// <p>The breach threshold for the alarm.</p> <p>Conditional: This parameter is required if the policy type is <code>StepScaling</code> and not supported otherwise.</p>
    pub breach_threshold: Option<f64>,
    /// <p>Indicates whether Amazon EC2 Auto Scaling waits for the cooldown period to complete before executing the policy.</p> <p>This parameter is not supported if the policy type is <code>StepScaling</code> or <code>TargetTrackingScaling</code>.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/Cooldown.html">Scaling Cooldowns</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    pub honor_cooldown: Option<bool>,
    /// <p>The metric value to compare to <code>BreachThreshold</code>. This enables you to execute a policy of type <code>StepScaling</code> and determine which step adjustment to use. For example, if the breach threshold is 50 and you want to use a step adjustment with a lower bound of 0 and an upper bound of 10, you can set the metric value to 59.</p> <p>If you specify a metric value that doesn't correspond to a step adjustment for the policy, the call returns an error.</p> <p>Conditional: This parameter is required if the policy type is <code>StepScaling</code> and not supported otherwise.</p>
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
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.breach_threshold {
            params.put(&format!("{}{}", prefix, "BreachThreshold"), &field_value);
        }
        if let Some(ref field_value) = obj.honor_cooldown {
            params.put(&format!("{}{}", prefix, "HonorCooldown"), &field_value);
        }
        if let Some(ref field_value) = obj.metric_value {
            params.put(&format!("{}{}", prefix, "MetricValue"), &field_value);
        }
        params.put(&format!("{}{}", prefix, "PolicyName"), &obj.policy_name);
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ExitStandbyAnswer {
    /// <p>The activities related to moving instances out of <code>Standby</code> mode.</p>
    pub activities: Option<Vec<Activity>>,
}

struct ExitStandbyAnswerDeserializer;
impl ExitStandbyAnswerDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ExitStandbyAnswer, XmlParseError> {
        deserialize_elements::<_, ExitStandbyAnswer, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Activities" => {
                    obj.activities
                        .get_or_insert(vec![])
                        .extend(ActivitiesDeserializer::deserialize("Activities", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
            &obj.auto_scaling_group_name,
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

/// <p>Describes a scheduled action that could not be created, updated, or deleted.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct FailedScheduledUpdateGroupActionRequest {
    /// <p>The error code.</p>
    pub error_code: Option<String>,
    /// <p>The error message accompanying the error code.</p>
    pub error_message: Option<String>,
    /// <p>The name of the scheduled action.</p>
    pub scheduled_action_name: String,
}

struct FailedScheduledUpdateGroupActionRequestDeserializer;
impl FailedScheduledUpdateGroupActionRequestDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<FailedScheduledUpdateGroupActionRequest, XmlParseError> {
        deserialize_elements::<_, FailedScheduledUpdateGroupActionRequest, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "ErrorCode" => {
                        obj.error_code = Some(XmlStringMaxLen64Deserializer::deserialize(
                            "ErrorCode",
                            stack,
                        )?);
                    }
                    "ErrorMessage" => {
                        obj.error_message =
                            Some(XmlStringDeserializer::deserialize("ErrorMessage", stack)?);
                    }
                    "ScheduledActionName" => {
                        obj.scheduled_action_name = XmlStringMaxLen255Deserializer::deserialize(
                            "ScheduledActionName",
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
struct FailedScheduledUpdateGroupActionRequestsDeserializer;
impl FailedScheduledUpdateGroupActionRequestsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<FailedScheduledUpdateGroupActionRequest>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(
                    FailedScheduledUpdateGroupActionRequestDeserializer::deserialize(
                        "member", stack,
                    )?,
                );
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
/// <p>Describes a filter.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
            params.put(&format!("{}{}", prefix, "Name"), &field_value);
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
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = i64::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct HealthCheckGracePeriodDeserializer;
impl HealthCheckGracePeriodDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = i64::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct HeartbeatTimeoutDeserializer;
impl HeartbeatTimeoutDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = i64::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Describes an EC2 instance.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct Instance {
    /// <p>The Availability Zone in which the instance is running.</p>
    pub availability_zone: String,
    /// <p>The last reported health status of the instance. "Healthy" means that the instance is healthy and should remain in service. "Unhealthy" means that the instance is unhealthy and that Amazon EC2 Auto Scaling should terminate and replace it.</p>
    pub health_status: String,
    /// <p>The ID of the instance.</p>
    pub instance_id: String,
    /// <p>The instance type of the EC2 instance.</p>
    pub instance_type: Option<String>,
    /// <p>The launch configuration associated with the instance.</p>
    pub launch_configuration_name: Option<String>,
    /// <p>The launch template for the instance.</p>
    pub launch_template: Option<LaunchTemplateSpecification>,
    /// <p>A description of the current lifecycle state. The <code>Quarantined</code> state is not used.</p>
    pub lifecycle_state: String,
    /// <p>Indicates whether the instance is protected from termination by Amazon EC2 Auto Scaling when scaling in.</p>
    pub protected_from_scale_in: bool,
    /// <p>The number of capacity units contributed by the instance based on its instance type.</p> <p>Valid Range: Minimum value of 1. Maximum value of 999.</p>
    pub weighted_capacity: Option<String>,
}

struct InstanceDeserializer;
impl InstanceDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Instance, XmlParseError> {
        deserialize_elements::<_, Instance, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "AvailabilityZone" => {
                    obj.availability_zone =
                        XmlStringMaxLen255Deserializer::deserialize("AvailabilityZone", stack)?;
                }
                "HealthStatus" => {
                    obj.health_status =
                        XmlStringMaxLen32Deserializer::deserialize("HealthStatus", stack)?;
                }
                "InstanceId" => {
                    obj.instance_id =
                        XmlStringMaxLen19Deserializer::deserialize("InstanceId", stack)?;
                }
                "InstanceType" => {
                    obj.instance_type = Some(XmlStringMaxLen255Deserializer::deserialize(
                        "InstanceType",
                        stack,
                    )?);
                }
                "LaunchConfigurationName" => {
                    obj.launch_configuration_name =
                        Some(XmlStringMaxLen255Deserializer::deserialize(
                            "LaunchConfigurationName",
                            stack,
                        )?);
                }
                "LaunchTemplate" => {
                    obj.launch_template =
                        Some(LaunchTemplateSpecificationDeserializer::deserialize(
                            "LaunchTemplate",
                            stack,
                        )?);
                }
                "LifecycleState" => {
                    obj.lifecycle_state =
                        LifecycleStateDeserializer::deserialize("LifecycleState", stack)?;
                }
                "ProtectedFromScaleIn" => {
                    obj.protected_from_scale_in =
                        InstanceProtectedDeserializer::deserialize("ProtectedFromScaleIn", stack)?;
                }
                "WeightedCapacity" => {
                    obj.weighted_capacity = Some(XmlStringMaxLen32Deserializer::deserialize(
                        "WeightedCapacity",
                        stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
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
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct InstanceMonitoring {
    /// <p>If <code>true</code>, detailed monitoring is enabled. Otherwise, basic monitoring is enabled.</p>
    pub enabled: Option<bool>,
}

struct InstanceMonitoringDeserializer;
impl InstanceMonitoringDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<InstanceMonitoring, XmlParseError> {
        deserialize_elements::<_, InstanceMonitoring, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Enabled" => {
                    obj.enabled = Some(MonitoringEnabledDeserializer::deserialize(
                        "Enabled", stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
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
            params.put(&format!("{}{}", prefix, "Enabled"), &field_value);
        }
    }
}

struct InstanceProtectedDeserializer;
impl InstanceProtectedDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<bool, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = bool::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct InstancesDeserializer;
impl InstancesDeserializer {
    #[allow(unused_variables)]
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
/// <p>Describes an instances distribution for an Auto Scaling group with <a>MixedInstancesPolicy</a>.</p> <p>The instances distribution specifies the distribution of On-Demand Instances and Spot Instances, the maximum price to pay for Spot Instances, and how the Auto Scaling group allocates instance types to fulfill On-Demand and Spot capacity.</p> <p>When you update <code>SpotAllocationStrategy</code>, <code>SpotInstancePools</code>, or <code>SpotMaxPrice</code>, this update action does not deploy any changes across the running Amazon EC2 instances in the group. Your existing Spot Instances continue to run as long as the maximum price for those instances is higher than the current Spot price. When scale out occurs, Amazon EC2 Auto Scaling launches instances based on the new settings. When scale in occurs, Amazon EC2 Auto Scaling terminates instances according to the group's termination policies.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct InstancesDistribution {
    /// <p>Indicates how to allocate instance types to fulfill On-Demand capacity.</p> <p>The only valid value is <code>prioritized</code>, which is also the default value. This strategy uses the order of instance type overrides for the <a>LaunchTemplate</a> to define the launch priority of each instance type. The first instance type in the array is prioritized higher than the last. If all your On-Demand capacity cannot be fulfilled using your highest priority instance, then the Auto Scaling groups launches the remaining capacity using the second priority instance type, and so on.</p>
    pub on_demand_allocation_strategy: Option<String>,
    /// <p><p>The minimum amount of the Auto Scaling group&#39;s capacity that must be fulfilled by On-Demand Instances. This base portion is provisioned first as your group scales.</p> <p>Default if not set is 0. If you leave it set to 0, On-Demand Instances are launched as a percentage of the Auto Scaling group&#39;s desired capacity, per the <code>OnDemandPercentageAboveBaseCapacity</code> setting.</p> <note> <p>An update to this setting means a gradual replacement of instances to maintain the specified number of On-Demand Instances for your base capacity. When replacing instances, Amazon EC2 Auto Scaling launches new instances before terminating the old ones.</p> </note></p>
    pub on_demand_base_capacity: Option<i64>,
    /// <p>Controls the percentages of On-Demand Instances and Spot Instances for your additional capacity beyond <code>OnDemandBaseCapacity</code>.</p> <p>Default if not set is 100. If you leave it set to 100, the percentages are 100% for On-Demand Instances and 0% for Spot Instances.</p> <note> <p>An update to this setting means a gradual replacement of instances to maintain the percentage of On-Demand Instances for your additional capacity above the base capacity. When replacing instances, Amazon EC2 Auto Scaling launches new instances before terminating the old ones.</p> </note> <p>Valid Range: Minimum value of 0. Maximum value of 100.</p>
    pub on_demand_percentage_above_base_capacity: Option<i64>,
    /// <p>Indicates how to allocate instances across Spot Instance pools.</p> <p>If the allocation strategy is <code>lowest-price</code>, the Auto Scaling group launches instances using the Spot pools with the lowest price, and evenly allocates your instances across the number of Spot pools that you specify. If the allocation strategy is <code>capacity-optimized</code>, the Auto Scaling group launches instances using Spot pools that are optimally chosen based on the available Spot capacity.</p> <p>The default Spot allocation strategy for calls that you make through the API, the AWS CLI, or the AWS SDKs is <code>lowest-price</code>. The default Spot allocation strategy for the AWS Management Console is <code>capacity-optimized</code>.</p> <p>Valid values: <code>lowest-price</code> | <code>capacity-optimized</code> </p>
    pub spot_allocation_strategy: Option<String>,
    /// <p>The number of Spot Instance pools across which to allocate your Spot Instances. The Spot pools are determined from the different instance types in the Overrides array of <a>LaunchTemplate</a>. Default if not set is 2.</p> <p>Used only when the Spot allocation strategy is <code>lowest-price</code>.</p> <p>Valid Range: Minimum value of 1. Maximum value of 20.</p>
    pub spot_instance_pools: Option<i64>,
    /// <p>The maximum price per unit hour that you are willing to pay for a Spot Instance. If you leave the value of this parameter blank (which is the default), the maximum Spot price is set at the On-Demand price.</p> <p>To remove a value that you previously set, include the parameter but leave the value blank.</p>
    pub spot_max_price: Option<String>,
}

struct InstancesDistributionDeserializer;
impl InstancesDistributionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<InstancesDistribution, XmlParseError> {
        deserialize_elements::<_, InstancesDistribution, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "OnDemandAllocationStrategy" => {
                    obj.on_demand_allocation_strategy = Some(XmlStringDeserializer::deserialize(
                        "OnDemandAllocationStrategy",
                        stack,
                    )?);
                }
                "OnDemandBaseCapacity" => {
                    obj.on_demand_base_capacity =
                        Some(OnDemandBaseCapacityDeserializer::deserialize(
                            "OnDemandBaseCapacity",
                            stack,
                        )?);
                }
                "OnDemandPercentageAboveBaseCapacity" => {
                    obj.on_demand_percentage_above_base_capacity = Some(
                        OnDemandPercentageAboveBaseCapacityDeserializer::deserialize(
                            "OnDemandPercentageAboveBaseCapacity",
                            stack,
                        )?,
                    );
                }
                "SpotAllocationStrategy" => {
                    obj.spot_allocation_strategy = Some(XmlStringDeserializer::deserialize(
                        "SpotAllocationStrategy",
                        stack,
                    )?);
                }
                "SpotInstancePools" => {
                    obj.spot_instance_pools = Some(SpotInstancePoolsDeserializer::deserialize(
                        "SpotInstancePools",
                        stack,
                    )?);
                }
                "SpotMaxPrice" => {
                    obj.spot_max_price = Some(MixedInstanceSpotPriceDeserializer::deserialize(
                        "SpotMaxPrice",
                        stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

/// Serialize `InstancesDistribution` contents to a `SignedRequest`.
struct InstancesDistributionSerializer;
impl InstancesDistributionSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &InstancesDistribution) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.on_demand_allocation_strategy {
            params.put(
                &format!("{}{}", prefix, "OnDemandAllocationStrategy"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.on_demand_base_capacity {
            params.put(
                &format!("{}{}", prefix, "OnDemandBaseCapacity"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.on_demand_percentage_above_base_capacity {
            params.put(
                &format!("{}{}", prefix, "OnDemandPercentageAboveBaseCapacity"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.spot_allocation_strategy {
            params.put(
                &format!("{}{}", prefix, "SpotAllocationStrategy"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.spot_instance_pools {
            params.put(&format!("{}{}", prefix, "SpotInstancePools"), &field_value);
        }
        if let Some(ref field_value) = obj.spot_max_price {
            params.put(&format!("{}{}", prefix, "SpotMaxPrice"), &field_value);
        }
    }
}

/// <p>Describes a launch configuration.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct LaunchConfiguration {
    /// <p>For Auto Scaling groups that are running in a VPC, specifies whether to assign a public IP address to the group's instances.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/asg-in-vpc.html">Launching Auto Scaling Instances in a VPC</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    pub associate_public_ip_address: Option<bool>,
    /// <p>A block device mapping, which specifies the block devices for the instance.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/block-device-mapping-concepts.html">Block Device Mapping</a> in the <i>Amazon EC2 User Guide for Linux Instances</i>.</p>
    pub block_device_mappings: Option<Vec<BlockDeviceMapping>>,
    /// <p>The ID of a ClassicLink-enabled VPC to link your EC2-Classic instances to.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/vpc-classiclink.html">ClassicLink</a> in the <i>Amazon EC2 User Guide for Linux Instances</i> and <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/asg-in-vpc.html#as-ClassicLink">Linking EC2-Classic Instances to a VPC</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    pub classic_link_vpc_id: Option<String>,
    /// <p>The IDs of one or more security groups for the VPC specified in <code>ClassicLinkVPCId</code>.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/vpc-classiclink.html">ClassicLink</a> in the <i>Amazon EC2 User Guide for Linux Instances</i> and <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/asg-in-vpc.html#as-ClassicLink">Linking EC2-Classic Instances to a VPC</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    pub classic_link_vpc_security_groups: Option<Vec<String>>,
    /// <p>The creation date and time for the launch configuration.</p>
    pub created_time: String,
    /// <p>Specifies whether the launch configuration is optimized for EBS I/O (<code>true</code>) or not (<code>false</code>).</p> <p>For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/EBSOptimized.html">Amazon EBS-Optimized Instances</a> in the <i>Amazon EC2 User Guide for Linux Instances</i>.</p>
    pub ebs_optimized: Option<bool>,
    /// <p>The name or the Amazon Resource Name (ARN) of the instance profile associated with the IAM role for the instance. The instance profile contains the IAM role.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/us-iam-role.html">IAM Role for Applications That Run on Amazon EC2 Instances</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    pub iam_instance_profile: Option<String>,
    /// <p>The ID of the Amazon Machine Image (AMI) to use to launch your EC2 instances.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/finding-an-ami.html">Finding an AMI</a> in the <i>Amazon EC2 User Guide for Linux Instances</i>.</p>
    pub image_id: String,
    /// <p>Controls whether instances in this group are launched with detailed (<code>true</code>) or basic (<code>false</code>) monitoring.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/latest/userguide/as-instance-monitoring.html#enable-as-instance-metrics">Configure Monitoring for Auto Scaling Instances</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    pub instance_monitoring: Option<InstanceMonitoring>,
    /// <p>The instance type for the instances.</p> <p>For information about available instance types, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/instance-types.html#AvailableInstanceTypes">Available Instance Types</a> in the <i>Amazon EC2 User Guide for Linux Instances.</i> </p>
    pub instance_type: String,
    /// <p>The ID of the kernel associated with the AMI.</p>
    pub kernel_id: Option<String>,
    /// <p>The name of the key pair.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-key-pairs.html">Amazon EC2 Key Pairs</a> in the <i>Amazon EC2 User Guide for Linux Instances</i>.</p>
    pub key_name: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the launch configuration.</p>
    pub launch_configuration_arn: Option<String>,
    /// <p>The name of the launch configuration.</p>
    pub launch_configuration_name: String,
    /// <p>The tenancy of the instance, either <code>default</code> or <code>dedicated</code>. An instance with <code>dedicated</code> tenancy runs on isolated, single-tenant hardware and can only be launched into a VPC.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/asg-in-vpc.html#as-vpc-tenancy">Instance Placement Tenancy</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    pub placement_tenancy: Option<String>,
    /// <p>The ID of the RAM disk associated with the AMI.</p>
    pub ramdisk_id: Option<String>,
    /// <p>A list that contains the security groups to assign to the instances in the Auto Scaling group.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonVPC/latest/UserGuide/VPC_SecurityGroups.html">Security Groups for Your VPC</a> in the <i>Amazon Virtual Private Cloud User Guide</i>.</p>
    pub security_groups: Option<Vec<String>>,
    /// <p>The maximum hourly price to be paid for any Spot Instance launched to fulfill the request. Spot Instances are launched when the price you specify exceeds the current Spot price.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/asg-launch-spot-instances.html">Launching Spot Instances in Your Auto Scaling Group</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    pub spot_price: Option<String>,
    /// <p>The Base64-encoded user data to make available to the launched EC2 instances.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-instance-metadata.html">Instance Metadata and User Data</a> in the <i>Amazon EC2 User Guide for Linux Instances</i>.</p>
    pub user_data: Option<String>,
}

struct LaunchConfigurationDeserializer;
impl LaunchConfigurationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<LaunchConfiguration, XmlParseError> {
        deserialize_elements::<_, LaunchConfiguration, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "AssociatePublicIpAddress" => {
                    obj.associate_public_ip_address =
                        Some(AssociatePublicIpAddressDeserializer::deserialize(
                            "AssociatePublicIpAddress",
                            stack,
                        )?);
                }
                "BlockDeviceMappings" => {
                    obj.block_device_mappings.get_or_insert(vec![]).extend(
                        BlockDeviceMappingsDeserializer::deserialize("BlockDeviceMappings", stack)?,
                    );
                }
                "ClassicLinkVPCId" => {
                    obj.classic_link_vpc_id = Some(XmlStringMaxLen255Deserializer::deserialize(
                        "ClassicLinkVPCId",
                        stack,
                    )?);
                }
                "ClassicLinkVPCSecurityGroups" => {
                    obj.classic_link_vpc_security_groups
                        .get_or_insert(vec![])
                        .extend(ClassicLinkVPCSecurityGroupsDeserializer::deserialize(
                            "ClassicLinkVPCSecurityGroups",
                            stack,
                        )?);
                }
                "CreatedTime" => {
                    obj.created_time =
                        TimestampTypeDeserializer::deserialize("CreatedTime", stack)?;
                }
                "EbsOptimized" => {
                    obj.ebs_optimized = Some(EbsOptimizedDeserializer::deserialize(
                        "EbsOptimized",
                        stack,
                    )?);
                }
                "IamInstanceProfile" => {
                    obj.iam_instance_profile = Some(XmlStringMaxLen1600Deserializer::deserialize(
                        "IamInstanceProfile",
                        stack,
                    )?);
                }
                "ImageId" => {
                    obj.image_id = XmlStringMaxLen255Deserializer::deserialize("ImageId", stack)?;
                }
                "InstanceMonitoring" => {
                    obj.instance_monitoring = Some(InstanceMonitoringDeserializer::deserialize(
                        "InstanceMonitoring",
                        stack,
                    )?);
                }
                "InstanceType" => {
                    obj.instance_type =
                        XmlStringMaxLen255Deserializer::deserialize("InstanceType", stack)?;
                }
                "KernelId" => {
                    obj.kernel_id = Some(XmlStringMaxLen255Deserializer::deserialize(
                        "KernelId", stack,
                    )?);
                }
                "KeyName" => {
                    obj.key_name = Some(XmlStringMaxLen255Deserializer::deserialize(
                        "KeyName", stack,
                    )?);
                }
                "LaunchConfigurationARN" => {
                    obj.launch_configuration_arn = Some(ResourceNameDeserializer::deserialize(
                        "LaunchConfigurationARN",
                        stack,
                    )?);
                }
                "LaunchConfigurationName" => {
                    obj.launch_configuration_name = XmlStringMaxLen255Deserializer::deserialize(
                        "LaunchConfigurationName",
                        stack,
                    )?;
                }
                "PlacementTenancy" => {
                    obj.placement_tenancy = Some(XmlStringMaxLen64Deserializer::deserialize(
                        "PlacementTenancy",
                        stack,
                    )?);
                }
                "RamdiskId" => {
                    obj.ramdisk_id = Some(XmlStringMaxLen255Deserializer::deserialize(
                        "RamdiskId",
                        stack,
                    )?);
                }
                "SecurityGroups" => {
                    obj.security_groups.get_or_insert(vec![]).extend(
                        SecurityGroupsDeserializer::deserialize("SecurityGroups", stack)?,
                    );
                }
                "SpotPrice" => {
                    obj.spot_price = Some(SpotPriceDeserializer::deserialize("SpotPrice", stack)?);
                }
                "UserData" => {
                    obj.user_data = Some(XmlStringUserDataDeserializer::deserialize(
                        "UserData", stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
            &obj.launch_configuration_name,
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct LaunchConfigurationNamesType {
    /// <p>The launch configuration names. If you omit this parameter, all launch configurations are described.</p>
    pub launch_configuration_names: Option<Vec<String>>,
    /// <p>The maximum number of items to return with this call. The default value is <code>50</code> and the maximum value is <code>100</code>.</p>
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
            params.put(&format!("{}{}", prefix, "MaxRecords"), &field_value);
        }
        if let Some(ref field_value) = obj.next_token {
            params.put(&format!("{}{}", prefix, "NextToken"), &field_value);
        }
    }
}

struct LaunchConfigurationsDeserializer;
impl LaunchConfigurationsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<LaunchConfiguration>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(LaunchConfigurationDeserializer::deserialize(
                    "member", stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct LaunchConfigurationsType {
    /// <p>The launch configurations.</p>
    pub launch_configurations: Vec<LaunchConfiguration>,
    /// <p>A string that indicates that the response contains more items than can be returned in a single response. To receive additional items, specify this string for the <code>NextToken</code> value when requesting the next set of items. This value is null when there are no more items to return.</p>
    pub next_token: Option<String>,
}

struct LaunchConfigurationsTypeDeserializer;
impl LaunchConfigurationsTypeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<LaunchConfigurationsType, XmlParseError> {
        deserialize_elements::<_, LaunchConfigurationsType, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "LaunchConfigurations" => {
                        obj.launch_configurations.extend(
                            LaunchConfigurationsDeserializer::deserialize(
                                "LaunchConfigurations",
                                stack,
                            )?,
                        );
                    }
                    "NextToken" => {
                        obj.next_token =
                            Some(XmlStringDeserializer::deserialize("NextToken", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p>Describes a launch template and overrides.</p> <p>The overrides are used to override the instance type specified by the launch template with multiple instance types that can be used to launch On-Demand Instances and Spot Instances.</p> <p>When you update the launch template or overrides, existing Amazon EC2 instances continue to run. When scale out occurs, Amazon EC2 Auto Scaling launches instances to match the new settings. When scale in occurs, Amazon EC2 Auto Scaling terminates instances according to the group's termination policies.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct LaunchTemplate {
    /// <p>The launch template to use. You must specify either the launch template ID or launch template name in the request.</p>
    pub launch_template_specification: Option<LaunchTemplateSpecification>,
    /// <p>An optional setting. Any parameters that you specify override the same parameters in the launch template. Currently, the only supported override is instance type. You can specify between 1 and 20 instance types.</p>
    pub overrides: Option<Vec<LaunchTemplateOverrides>>,
}

struct LaunchTemplateDeserializer;
impl LaunchTemplateDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<LaunchTemplate, XmlParseError> {
        deserialize_elements::<_, LaunchTemplate, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "LaunchTemplateSpecification" => {
                    obj.launch_template_specification =
                        Some(LaunchTemplateSpecificationDeserializer::deserialize(
                            "LaunchTemplateSpecification",
                            stack,
                        )?);
                }
                "Overrides" => {
                    obj.overrides
                        .get_or_insert(vec![])
                        .extend(OverridesDeserializer::deserialize("Overrides", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

/// Serialize `LaunchTemplate` contents to a `SignedRequest`.
struct LaunchTemplateSerializer;
impl LaunchTemplateSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &LaunchTemplate) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.launch_template_specification {
            LaunchTemplateSpecificationSerializer::serialize(
                params,
                &format!("{}{}", prefix, "LaunchTemplateSpecification"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.overrides {
            OverridesSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Overrides"),
                field_value,
            );
        }
    }
}

struct LaunchTemplateNameDeserializer;
impl LaunchTemplateNameDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Describes an override for a launch template.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct LaunchTemplateOverrides {
    /// <p>The instance type.</p> <p>For information about available instance types, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/instance-types.html#AvailableInstanceTypes">Available Instance Types</a> in the <i>Amazon Elastic Compute Cloud User Guide.</i> </p>
    pub instance_type: Option<String>,
    /// <p>The number of capacity units, which gives the instance type a proportional weight to other instance types. For example, larger instance types are generally weighted more than smaller instance types. These are the same units that you chose to set the desired capacity in terms of instances, or a performance attribute such as vCPUs, memory, or I/O.</p> <p>Valid Range: Minimum value of 1. Maximum value of 999.</p>
    pub weighted_capacity: Option<String>,
}

struct LaunchTemplateOverridesDeserializer;
impl LaunchTemplateOverridesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<LaunchTemplateOverrides, XmlParseError> {
        deserialize_elements::<_, LaunchTemplateOverrides, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "InstanceType" => {
                        obj.instance_type = Some(XmlStringMaxLen255Deserializer::deserialize(
                            "InstanceType",
                            stack,
                        )?);
                    }
                    "WeightedCapacity" => {
                        obj.weighted_capacity = Some(XmlStringMaxLen32Deserializer::deserialize(
                            "WeightedCapacity",
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

/// Serialize `LaunchTemplateOverrides` contents to a `SignedRequest`.
struct LaunchTemplateOverridesSerializer;
impl LaunchTemplateOverridesSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &LaunchTemplateOverrides) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.instance_type {
            params.put(&format!("{}{}", prefix, "InstanceType"), &field_value);
        }
        if let Some(ref field_value) = obj.weighted_capacity {
            params.put(&format!("{}{}", prefix, "WeightedCapacity"), &field_value);
        }
    }
}

/// <p>Describes a launch template and the launch template version.</p> <p>The launch template that is specified must be configured for use with an Auto Scaling group. For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/create-launch-template.html">Creating a Launch Template for an Auto Scaling Group</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct LaunchTemplateSpecification {
    /// <p>The ID of the launch template. You must specify either a template ID or a template name.</p>
    pub launch_template_id: Option<String>,
    /// <p>The name of the launch template. You must specify either a template name or a template ID.</p>
    pub launch_template_name: Option<String>,
    /// <p>The version number, <code>$Latest</code>, or <code>$Default</code>. If the value is <code>$Latest</code>, Amazon EC2 Auto Scaling selects the latest version of the launch template when launching instances. If the value is <code>$Default</code>, Amazon EC2 Auto Scaling selects the default version of the launch template when launching instances. The default value is <code>$Default</code>.</p>
    pub version: Option<String>,
}

struct LaunchTemplateSpecificationDeserializer;
impl LaunchTemplateSpecificationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<LaunchTemplateSpecification, XmlParseError> {
        deserialize_elements::<_, LaunchTemplateSpecification, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "LaunchTemplateId" => {
                        obj.launch_template_id = Some(XmlStringMaxLen255Deserializer::deserialize(
                            "LaunchTemplateId",
                            stack,
                        )?);
                    }
                    "LaunchTemplateName" => {
                        obj.launch_template_name =
                            Some(LaunchTemplateNameDeserializer::deserialize(
                                "LaunchTemplateName",
                                stack,
                            )?);
                    }
                    "Version" => {
                        obj.version = Some(XmlStringMaxLen255Deserializer::deserialize(
                            "Version", stack,
                        )?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
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
            params.put(&format!("{}{}", prefix, "LaunchTemplateId"), &field_value);
        }
        if let Some(ref field_value) = obj.launch_template_name {
            params.put(&format!("{}{}", prefix, "LaunchTemplateName"), &field_value);
        }
        if let Some(ref field_value) = obj.version {
            params.put(&format!("{}{}", prefix, "Version"), &field_value);
        }
    }
}

struct LifecycleActionResultDeserializer;
impl LifecycleActionResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Describes a lifecycle hook, which tells Amazon EC2 Auto Scaling that you want to perform an action whenever it launches instances or terminates instances. Used in response to <a>DescribeLifecycleHooks</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct LifecycleHook {
    /// <p>The name of the Auto Scaling group for the lifecycle hook.</p>
    pub auto_scaling_group_name: Option<String>,
    /// <p>Defines the action the Auto Scaling group should take when the lifecycle hook timeout elapses or if an unexpected failure occurs. The possible values are <code>CONTINUE</code> and <code>ABANDON</code>.</p>
    pub default_result: Option<String>,
    /// <p>The maximum time, in seconds, that an instance can remain in a <code>Pending:Wait</code> or <code>Terminating:Wait</code> state. The maximum is 172800 seconds (48 hours) or 100 times <code>HeartbeatTimeout</code>, whichever is smaller.</p>
    pub global_timeout: Option<i64>,
    /// <p>The maximum time, in seconds, that can elapse before the lifecycle hook times out. If the lifecycle hook times out, Amazon EC2 Auto Scaling performs the action that you specified in the <code>DefaultResult</code> parameter.</p>
    pub heartbeat_timeout: Option<i64>,
    /// <p>The name of the lifecycle hook.</p>
    pub lifecycle_hook_name: Option<String>,
    /// <p><p>The state of the EC2 instance to which to attach the lifecycle hook. The following are possible values:</p> <ul> <li> <p>autoscaling:EC2<em>INSTANCE</em>LAUNCHING</p> </li> <li> <p>autoscaling:EC2<em>INSTANCE</em>TERMINATING</p> </li> </ul></p>
    pub lifecycle_transition: Option<String>,
    /// <p>Additional information that is included any time Amazon EC2 Auto Scaling sends a message to the notification target.</p>
    pub notification_metadata: Option<String>,
    /// <p>The ARN of the target that Amazon EC2 Auto Scaling sends notifications to when an instance is in the transition state for the lifecycle hook. The notification target can be either an SQS queue or an SNS topic.</p>
    pub notification_target_arn: Option<String>,
    /// <p>The ARN of the IAM role that allows the Auto Scaling group to publish to the specified notification target.</p>
    pub role_arn: Option<String>,
}

struct LifecycleHookDeserializer;
impl LifecycleHookDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<LifecycleHook, XmlParseError> {
        deserialize_elements::<_, LifecycleHook, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "AutoScalingGroupName" => {
                    obj.auto_scaling_group_name = Some(ResourceNameDeserializer::deserialize(
                        "AutoScalingGroupName",
                        stack,
                    )?);
                }
                "DefaultResult" => {
                    obj.default_result = Some(LifecycleActionResultDeserializer::deserialize(
                        "DefaultResult",
                        stack,
                    )?);
                }
                "GlobalTimeout" => {
                    obj.global_timeout = Some(GlobalTimeoutDeserializer::deserialize(
                        "GlobalTimeout",
                        stack,
                    )?);
                }
                "HeartbeatTimeout" => {
                    obj.heartbeat_timeout = Some(HeartbeatTimeoutDeserializer::deserialize(
                        "HeartbeatTimeout",
                        stack,
                    )?);
                }
                "LifecycleHookName" => {
                    obj.lifecycle_hook_name = Some(AsciiStringMaxLen255Deserializer::deserialize(
                        "LifecycleHookName",
                        stack,
                    )?);
                }
                "LifecycleTransition" => {
                    obj.lifecycle_transition = Some(LifecycleTransitionDeserializer::deserialize(
                        "LifecycleTransition",
                        stack,
                    )?);
                }
                "NotificationMetadata" => {
                    obj.notification_metadata = Some(XmlStringMaxLen1023Deserializer::deserialize(
                        "NotificationMetadata",
                        stack,
                    )?);
                }
                "NotificationTargetARN" => {
                    obj.notification_target_arn = Some(ResourceNameDeserializer::deserialize(
                        "NotificationTargetARN",
                        stack,
                    )?);
                }
                "RoleARN" => {
                    obj.role_arn = Some(ResourceNameDeserializer::deserialize("RoleARN", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
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

/// <p>Describes a lifecycle hook. Used in combination with <a>CreateAutoScalingGroup</a>.</p> <p>A lifecycle hook tells Amazon EC2 Auto Scaling to perform an action on an instance when the instance launches (before it is put into service) or as the instance terminates (before it is fully terminated).</p> <p>This step is a part of the procedure for creating a lifecycle hook for an Auto Scaling group:</p> <ol> <li> <p>(Optional) Create a Lambda function and a rule that allows CloudWatch Events to invoke your Lambda function when Amazon EC2 Auto Scaling launches or terminates instances.</p> </li> <li> <p>(Optional) Create a notification target and an IAM role. The target can be either an Amazon SQS queue or an Amazon SNS topic. The role allows Amazon EC2 Auto Scaling to publish lifecycle notifications to the target.</p> </li> <li> <p> <b>Create the lifecycle hook. Specify whether the hook is used when the instances launch or terminate.</b> </p> </li> <li> <p>If you need more time, record the lifecycle action heartbeat to keep the instance in a pending state using <a>RecordLifecycleActionHeartbeat</a>.</p> </li> <li> <p>If you finish before the timeout period ends, complete the lifecycle action using <a>CompleteLifecycleAction</a>.</p> </li> </ol> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/lifecycle-hooks.html">Amazon EC2 Auto Scaling Lifecycle Hooks</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p> <p>You can view the lifecycle hooks for an Auto Scaling group using <a>DescribeLifecycleHooks</a>. You can modify an existing lifecycle hook or create new lifecycle hooks using <a>PutLifecycleHook</a>. If you are no longer using a lifecycle hook, you can delete it using <a>DeleteLifecycleHook</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct LifecycleHookSpecification {
    /// <p>Defines the action the Auto Scaling group should take when the lifecycle hook timeout elapses or if an unexpected failure occurs. The valid values are <code>CONTINUE</code> and <code>ABANDON</code>. The default value is <code>ABANDON</code>.</p>
    pub default_result: Option<String>,
    /// <p>The maximum time, in seconds, that can elapse before the lifecycle hook times out.</p> <p>If the lifecycle hook times out, Amazon EC2 Auto Scaling performs the action that you specified in the <code>DefaultResult</code> parameter. You can prevent the lifecycle hook from timing out by calling <a>RecordLifecycleActionHeartbeat</a>.</p>
    pub heartbeat_timeout: Option<i64>,
    /// <p>The name of the lifecycle hook.</p>
    pub lifecycle_hook_name: String,
    /// <p><p>The state of the EC2 instance to which you want to attach the lifecycle hook. The valid values are:</p> <ul> <li> <p>autoscaling:EC2<em>INSTANCE</em>LAUNCHING</p> </li> <li> <p>autoscaling:EC2<em>INSTANCE</em>TERMINATING</p> </li> </ul></p>
    pub lifecycle_transition: String,
    /// <p>Additional information that you want to include any time Amazon EC2 Auto Scaling sends a message to the notification target.</p>
    pub notification_metadata: Option<String>,
    /// <p>The ARN of the target that Amazon EC2 Auto Scaling sends notifications to when an instance is in the transition state for the lifecycle hook. The notification target can be either an SQS queue or an SNS topic.</p>
    pub notification_target_arn: Option<String>,
    /// <p>The ARN of the IAM role that allows the Auto Scaling group to publish to the specified notification target, for example, an Amazon SNS topic or an Amazon SQS queue.</p>
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
            params.put(&format!("{}{}", prefix, "DefaultResult"), &field_value);
        }
        if let Some(ref field_value) = obj.heartbeat_timeout {
            params.put(&format!("{}{}", prefix, "HeartbeatTimeout"), &field_value);
        }
        params.put(
            &format!("{}{}", prefix, "LifecycleHookName"),
            &obj.lifecycle_hook_name,
        );
        params.put(
            &format!("{}{}", prefix, "LifecycleTransition"),
            &obj.lifecycle_transition,
        );
        if let Some(ref field_value) = obj.notification_metadata {
            params.put(
                &format!("{}{}", prefix, "NotificationMetadata"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.notification_target_arn {
            params.put(
                &format!("{}{}", prefix, "NotificationTargetARN"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.role_arn {
            params.put(&format!("{}{}", prefix, "RoleARN"), &field_value);
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
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<LifecycleHook>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(LifecycleHookDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
struct LifecycleStateDeserializer;
impl LifecycleStateDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct LifecycleTransitionDeserializer;
impl LifecycleTransitionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct LoadBalancerNamesDeserializer;
impl LoadBalancerNamesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(XmlStringMaxLen255Deserializer::deserialize(
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

/// <p>Describes the state of a Classic Load Balancer.</p> <p>If you specify a load balancer when creating the Auto Scaling group, the state of the load balancer is <code>InService</code>.</p> <p>If you attach a load balancer to an existing Auto Scaling group, the initial state is <code>Adding</code>. The state transitions to <code>Added</code> after all instances in the group are registered with the load balancer. If Elastic Load Balancing health checks are enabled for the load balancer, the state transitions to <code>InService</code> after at least one instance in the group passes the health check. If EC2 health checks are enabled instead, the load balancer remains in the <code>Added</code> state.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct LoadBalancerState {
    /// <p>The name of the load balancer.</p>
    pub load_balancer_name: Option<String>,
    /// <p><p>One of the following load balancer states:</p> <ul> <li> <p> <code>Adding</code> - The instances in the group are being registered with the load balancer.</p> </li> <li> <p> <code>Added</code> - All instances in the group are registered with the load balancer.</p> </li> <li> <p> <code>InService</code> - At least one instance in the group passed an ELB health check.</p> </li> <li> <p> <code>Removing</code> - The instances in the group are being deregistered from the load balancer. If connection draining is enabled, Elastic Load Balancing waits for in-flight requests to complete before deregistering the instances.</p> </li> <li> <p> <code>Removed</code> - All instances in the group are deregistered from the load balancer.</p> </li> </ul></p>
    pub state: Option<String>,
}

struct LoadBalancerStateDeserializer;
impl LoadBalancerStateDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<LoadBalancerState, XmlParseError> {
        deserialize_elements::<_, LoadBalancerState, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "LoadBalancerName" => {
                    obj.load_balancer_name = Some(XmlStringMaxLen255Deserializer::deserialize(
                        "LoadBalancerName",
                        stack,
                    )?);
                }
                "State" => {
                    obj.state = Some(XmlStringMaxLen255Deserializer::deserialize("State", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct LoadBalancerStatesDeserializer;
impl LoadBalancerStatesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<LoadBalancerState>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(LoadBalancerStateDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
/// <p>Describes the state of a target group.</p> <p>If you attach a target group to an existing Auto Scaling group, the initial state is <code>Adding</code>. The state transitions to <code>Added</code> after all Auto Scaling instances are registered with the target group. If Elastic Load Balancing health checks are enabled, the state transitions to <code>InService</code> after at least one Auto Scaling instance passes the health check. If EC2 health checks are enabled instead, the target group remains in the <code>Added</code> state.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct LoadBalancerTargetGroupState {
    /// <p>The Amazon Resource Name (ARN) of the target group.</p>
    pub load_balancer_target_group_arn: Option<String>,
    /// <p><p>The state of the target group.</p> <ul> <li> <p> <code>Adding</code> - The Auto Scaling instances are being registered with the target group.</p> </li> <li> <p> <code>Added</code> - All Auto Scaling instances are registered with the target group.</p> </li> <li> <p> <code>InService</code> - At least one Auto Scaling instance passed an ELB health check.</p> </li> <li> <p> <code>Removing</code> - The Auto Scaling instances are being deregistered from the target group. If connection draining is enabled, Elastic Load Balancing waits for in-flight requests to complete before deregistering the instances.</p> </li> <li> <p> <code>Removed</code> - All Auto Scaling instances are deregistered from the target group.</p> </li> </ul></p>
    pub state: Option<String>,
}

struct LoadBalancerTargetGroupStateDeserializer;
impl LoadBalancerTargetGroupStateDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<LoadBalancerTargetGroupState, XmlParseError> {
        deserialize_elements::<_, LoadBalancerTargetGroupState, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "LoadBalancerTargetGroupARN" => {
                        obj.load_balancer_target_group_arn =
                            Some(XmlStringMaxLen511Deserializer::deserialize(
                                "LoadBalancerTargetGroupARN",
                                stack,
                            )?);
                    }
                    "State" => {
                        obj.state =
                            Some(XmlStringMaxLen255Deserializer::deserialize("State", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
struct LoadBalancerTargetGroupStatesDeserializer;
impl LoadBalancerTargetGroupStatesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<LoadBalancerTargetGroupState>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(LoadBalancerTargetGroupStateDeserializer::deserialize(
                    "member", stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
struct MaxInstanceLifetimeDeserializer;
impl MaxInstanceLifetimeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = i64::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct MaxNumberOfAutoScalingGroupsDeserializer;
impl MaxNumberOfAutoScalingGroupsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = i64::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct MaxNumberOfLaunchConfigurationsDeserializer;
impl MaxNumberOfLaunchConfigurationsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = i64::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Describes a metric.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct MetricCollectionType {
    /// <p><p>One of the following metrics:</p> <ul> <li> <p> <code>GroupMinSize</code> </p> </li> <li> <p> <code>GroupMaxSize</code> </p> </li> <li> <p> <code>GroupDesiredCapacity</code> </p> </li> <li> <p> <code>GroupInServiceInstances</code> </p> </li> <li> <p> <code>GroupPendingInstances</code> </p> </li> <li> <p> <code>GroupStandbyInstances</code> </p> </li> <li> <p> <code>GroupTerminatingInstances</code> </p> </li> <li> <p> <code>GroupTotalInstances</code> </p> </li> </ul></p>
    pub metric: Option<String>,
}

struct MetricCollectionTypeDeserializer;
impl MetricCollectionTypeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<MetricCollectionType, XmlParseError> {
        deserialize_elements::<_, MetricCollectionType, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Metric" => {
                    obj.metric = Some(XmlStringMaxLen255Deserializer::deserialize(
                        "Metric", stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct MetricCollectionTypesDeserializer;
impl MetricCollectionTypesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<MetricCollectionType>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(MetricCollectionTypeDeserializer::deserialize(
                    "member", stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
/// <p>Describes the dimension of a metric.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct MetricDimension {
    /// <p>The name of the dimension.</p>
    pub name: String,
    /// <p>The value of the dimension.</p>
    pub value: String,
}

struct MetricDimensionDeserializer;
impl MetricDimensionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<MetricDimension, XmlParseError> {
        deserialize_elements::<_, MetricDimension, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Name" => {
                    obj.name = MetricDimensionNameDeserializer::deserialize("Name", stack)?;
                }
                "Value" => {
                    obj.value = MetricDimensionValueDeserializer::deserialize("Value", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
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

        params.put(&format!("{}{}", prefix, "Name"), &obj.name);
        params.put(&format!("{}{}", prefix, "Value"), &obj.value);
    }
}

struct MetricDimensionNameDeserializer;
impl MetricDimensionNameDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct MetricDimensionValueDeserializer;
impl MetricDimensionValueDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct MetricDimensionsDeserializer;
impl MetricDimensionsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<MetricDimension>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(MetricDimensionDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
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
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct MetricGranularityType {
    /// <p>The granularity. The only valid value is <code>1Minute</code>.</p>
    pub granularity: Option<String>,
}

struct MetricGranularityTypeDeserializer;
impl MetricGranularityTypeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<MetricGranularityType, XmlParseError> {
        deserialize_elements::<_, MetricGranularityType, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Granularity" => {
                    obj.granularity = Some(XmlStringMaxLen255Deserializer::deserialize(
                        "Granularity",
                        stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct MetricGranularityTypesDeserializer;
impl MetricGranularityTypesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<MetricGranularityType>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(MetricGranularityTypeDeserializer::deserialize(
                    "member", stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
struct MetricNameDeserializer;
impl MetricNameDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct MetricNamespaceDeserializer;
impl MetricNamespaceDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct MetricScaleDeserializer;
impl MetricScaleDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<f64, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = f64::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct MetricStatisticDeserializer;
impl MetricStatisticDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct MetricTypeDeserializer;
impl MetricTypeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct MetricUnitDeserializer;
impl MetricUnitDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

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
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = i64::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct MinAdjustmentStepDeserializer;
impl MinAdjustmentStepDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = i64::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct MixedInstanceSpotPriceDeserializer;
impl MixedInstanceSpotPriceDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Describes a mixed instances policy for an Auto Scaling group. With mixed instances, your Auto Scaling group can provision a combination of On-Demand Instances and Spot Instances across multiple instance types. For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/asg-purchase-options.html">Auto Scaling Groups with Multiple Instance Types and Purchase Options</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p> <p>You can create a mixed instances policy for a new Auto Scaling group, or you can create it for an existing group by updating the group to specify <code>MixedInstancesPolicy</code> as the top-level parameter instead of a launch configuration or template. For more information, see <a>CreateAutoScalingGroup</a> and <a>UpdateAutoScalingGroup</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct MixedInstancesPolicy {
    /// <p>The instances distribution to use.</p> <p>If you leave this parameter unspecified, the value for each parameter in <code>InstancesDistribution</code> uses a default value.</p>
    pub instances_distribution: Option<InstancesDistribution>,
    /// <p>The launch template and instance types (overrides).</p> <p>This parameter must be specified when creating a mixed instances policy.</p>
    pub launch_template: Option<LaunchTemplate>,
}

struct MixedInstancesPolicyDeserializer;
impl MixedInstancesPolicyDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<MixedInstancesPolicy, XmlParseError> {
        deserialize_elements::<_, MixedInstancesPolicy, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "InstancesDistribution" => {
                    obj.instances_distribution =
                        Some(InstancesDistributionDeserializer::deserialize(
                            "InstancesDistribution",
                            stack,
                        )?);
                }
                "LaunchTemplate" => {
                    obj.launch_template = Some(LaunchTemplateDeserializer::deserialize(
                        "LaunchTemplate",
                        stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

/// Serialize `MixedInstancesPolicy` contents to a `SignedRequest`.
struct MixedInstancesPolicySerializer;
impl MixedInstancesPolicySerializer {
    fn serialize(params: &mut Params, name: &str, obj: &MixedInstancesPolicy) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.instances_distribution {
            InstancesDistributionSerializer::serialize(
                params,
                &format!("{}{}", prefix, "InstancesDistribution"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.launch_template {
            LaunchTemplateSerializer::serialize(
                params,
                &format!("{}{}", prefix, "LaunchTemplate"),
                field_value,
            );
        }
    }
}

struct MonitoringEnabledDeserializer;
impl MonitoringEnabledDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<bool, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = bool::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct NoDeviceDeserializer;
impl NoDeviceDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<bool, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = bool::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Describes a notification.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct NotificationConfiguration {
    /// <p>The name of the Auto Scaling group.</p>
    pub auto_scaling_group_name: Option<String>,
    /// <p><p>One of the following event notification types:</p> <ul> <li> <p> <code>autoscaling:EC2<em>INSTANCE</em>LAUNCH</code> </p> </li> <li> <p> <code>autoscaling:EC2<em>INSTANCE</em>LAUNCH<em>ERROR</code> </p> </li> <li> <p> <code>autoscaling:EC2</em>INSTANCE<em>TERMINATE</code> </p> </li> <li> <p> <code>autoscaling:EC2</em>INSTANCE<em>TERMINATE</em>ERROR</code> </p> </li> <li> <p> <code>autoscaling:TEST_NOTIFICATION</code> </p> </li> </ul></p>
    pub notification_type: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the Amazon Simple Notification Service (Amazon SNS) topic.</p>
    pub topic_arn: Option<String>,
}

struct NotificationConfigurationDeserializer;
impl NotificationConfigurationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<NotificationConfiguration, XmlParseError> {
        deserialize_elements::<_, NotificationConfiguration, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "AutoScalingGroupName" => {
                        obj.auto_scaling_group_name = Some(ResourceNameDeserializer::deserialize(
                            "AutoScalingGroupName",
                            stack,
                        )?);
                    }
                    "NotificationType" => {
                        obj.notification_type = Some(XmlStringMaxLen255Deserializer::deserialize(
                            "NotificationType",
                            stack,
                        )?);
                    }
                    "TopicARN" => {
                        obj.topic_arn =
                            Some(ResourceNameDeserializer::deserialize("TopicARN", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
struct NotificationConfigurationsDeserializer;
impl NotificationConfigurationsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<NotificationConfiguration>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(NotificationConfigurationDeserializer::deserialize(
                    "member", stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
struct NumberOfAutoScalingGroupsDeserializer;
impl NumberOfAutoScalingGroupsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = i64::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct NumberOfLaunchConfigurationsDeserializer;
impl NumberOfLaunchConfigurationsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = i64::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct OnDemandBaseCapacityDeserializer;
impl OnDemandBaseCapacityDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = i64::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct OnDemandPercentageAboveBaseCapacityDeserializer;
impl OnDemandPercentageAboveBaseCapacityDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = i64::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct OverridesDeserializer;
impl OverridesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<LaunchTemplateOverrides>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(LaunchTemplateOverridesDeserializer::deserialize(
                    "member", stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}

/// Serialize `Overrides` contents to a `SignedRequest`.
struct OverridesSerializer;
impl OverridesSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<LaunchTemplateOverrides>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            LaunchTemplateOverridesSerializer::serialize(params, &key, obj);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct PoliciesType {
    /// <p>A string that indicates that the response contains more items than can be returned in a single response. To receive additional items, specify this string for the <code>NextToken</code> value when requesting the next set of items. This value is null when there are no more items to return.</p>
    pub next_token: Option<String>,
    /// <p>The scaling policies.</p>
    pub scaling_policies: Option<Vec<ScalingPolicy>>,
}

struct PoliciesTypeDeserializer;
impl PoliciesTypeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PoliciesType, XmlParseError> {
        deserialize_elements::<_, PoliciesType, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "NextToken" => {
                    obj.next_token = Some(XmlStringDeserializer::deserialize("NextToken", stack)?);
                }
                "ScalingPolicies" => {
                    obj.scaling_policies.get_or_insert(vec![]).extend(
                        ScalingPoliciesDeserializer::deserialize("ScalingPolicies", stack)?,
                    );
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
/// <p>Contains the output of PutScalingPolicy.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct PolicyARNType {
    /// <p>The CloudWatch alarms created for the target tracking scaling policy.</p>
    pub alarms: Option<Vec<Alarm>>,
    /// <p>The Amazon Resource Name (ARN) of the policy.</p>
    pub policy_arn: Option<String>,
}

struct PolicyARNTypeDeserializer;
impl PolicyARNTypeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PolicyARNType, XmlParseError> {
        deserialize_elements::<_, PolicyARNType, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Alarms" => {
                    obj.alarms
                        .get_or_insert(vec![])
                        .extend(AlarmsDeserializer::deserialize("Alarms", stack)?);
                }
                "PolicyARN" => {
                    obj.policy_arn =
                        Some(ResourceNameDeserializer::deserialize("PolicyARN", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct PolicyIncrementDeserializer;
impl PolicyIncrementDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = i64::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

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

/// <p>Represents a predefined metric for a target tracking scaling policy to use with Amazon EC2 Auto Scaling.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PredefinedMetricSpecification {
    /// <p><p>The metric type. The following predefined metrics are available:</p> <ul> <li> <p> <code>ASGAverageCPUUtilization</code> - Average CPU utilization of the Auto Scaling group.</p> </li> <li> <p> <code>ASGAverageNetworkIn</code> - Average number of bytes received on all network interfaces by the Auto Scaling group.</p> </li> <li> <p> <code>ASGAverageNetworkOut</code> - Average number of bytes sent out on all network interfaces by the Auto Scaling group.</p> </li> <li> <p> <code>ALBRequestCountPerTarget</code> - Number of requests completed per target in an Application Load Balancer target group.</p> </li> </ul></p>
    pub predefined_metric_type: String,
    /// <p><p>Identifies the resource associated with the metric type. You can&#39;t specify a resource label unless the metric type is <code>ALBRequestCountPerTarget</code> and there is a target group attached to the Auto Scaling group.</p> <p>The format is <code>app/<i>load-balancer-name</i>/<i>load-balancer-id</i>/targetgroup/<i>target-group-name</i>/<i>target-group-id</i> </code>, where </p> <ul> <li> <p> <code>app/<i>load-balancer-name</i>/<i>load-balancer-id</i> </code> is the final portion of the load balancer ARN, and</p> </li> <li> <p> <code>targetgroup/<i>target-group-name</i>/<i>target-group-id</i> </code> is the final portion of the target group ARN.</p> </li> </ul></p>
    pub resource_label: Option<String>,
}

struct PredefinedMetricSpecificationDeserializer;
impl PredefinedMetricSpecificationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PredefinedMetricSpecification, XmlParseError> {
        deserialize_elements::<_, PredefinedMetricSpecification, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "PredefinedMetricType" => {
                        obj.predefined_metric_type =
                            MetricTypeDeserializer::deserialize("PredefinedMetricType", stack)?;
                    }
                    "ResourceLabel" => {
                        obj.resource_label = Some(XmlStringMaxLen1023Deserializer::deserialize(
                            "ResourceLabel",
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
            &obj.predefined_metric_type,
        );
        if let Some(ref field_value) = obj.resource_label {
            params.put(&format!("{}{}", prefix, "ResourceLabel"), &field_value);
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

/// <p>Describes a process type.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/as-suspend-resume-processes.html#process-types">Scaling Processes</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ProcessType {
    /// <p><p>One of the following processes:</p> <ul> <li> <p> <code>Launch</code> </p> </li> <li> <p> <code>Terminate</code> </p> </li> <li> <p> <code>AddToLoadBalancer</code> </p> </li> <li> <p> <code>AlarmNotification</code> </p> </li> <li> <p> <code>AZRebalance</code> </p> </li> <li> <p> <code>HealthCheck</code> </p> </li> <li> <p> <code>ReplaceUnhealthy</code> </p> </li> <li> <p> <code>ScheduledActions</code> </p> </li> </ul></p>
    pub process_name: String,
}

struct ProcessTypeDeserializer;
impl ProcessTypeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ProcessType, XmlParseError> {
        deserialize_elements::<_, ProcessType, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "ProcessName" => {
                    obj.process_name =
                        XmlStringMaxLen255Deserializer::deserialize("ProcessName", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct ProcessesDeserializer;
impl ProcessesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<ProcessType>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(ProcessTypeDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ProcessesType {
    /// <p>The names of the process types.</p>
    pub processes: Option<Vec<ProcessType>>,
}

struct ProcessesTypeDeserializer;
impl ProcessesTypeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ProcessesType, XmlParseError> {
        deserialize_elements::<_, ProcessesType, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Processes" => {
                    obj.processes
                        .get_or_insert(vec![])
                        .extend(ProcessesDeserializer::deserialize("Processes", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct ProgressDeserializer;
impl ProgressDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = i64::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct PropagateAtLaunchDeserializer;
impl PropagateAtLaunchDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<bool, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = bool::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct PutLifecycleHookAnswer {}

struct PutLifecycleHookAnswerDeserializer;
impl PutLifecycleHookAnswerDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PutLifecycleHookAnswer, XmlParseError> {
        start_element(tag_name, stack)?;

        let obj = PutLifecycleHookAnswer::default();

        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutLifecycleHookType {
    /// <p>The name of the Auto Scaling group.</p>
    pub auto_scaling_group_name: String,
    /// <p>Defines the action the Auto Scaling group should take when the lifecycle hook timeout elapses or if an unexpected failure occurs. This parameter can be either <code>CONTINUE</code> or <code>ABANDON</code>. The default value is <code>ABANDON</code>.</p>
    pub default_result: Option<String>,
    /// <p>The maximum time, in seconds, that can elapse before the lifecycle hook times out. The range is from <code>30</code> to <code>7200</code> seconds. The default value is <code>3600</code> seconds (1 hour).</p> <p>If the lifecycle hook times out, Amazon EC2 Auto Scaling performs the action that you specified in the <code>DefaultResult</code> parameter. You can prevent the lifecycle hook from timing out by calling <a>RecordLifecycleActionHeartbeat</a>.</p>
    pub heartbeat_timeout: Option<i64>,
    /// <p>The name of the lifecycle hook.</p>
    pub lifecycle_hook_name: String,
    /// <p>The instance state to which you want to attach the lifecycle hook. The valid values are:</p> <ul> <li> <p>autoscaling:EC2_INSTANCE_LAUNCHING</p> </li> <li> <p>autoscaling:EC2_INSTANCE_TERMINATING</p> </li> </ul> <p>Conditional: This parameter is required for new lifecycle hooks, but optional when updating existing hooks.</p>
    pub lifecycle_transition: Option<String>,
    /// <p>Additional information that you want to include any time Amazon EC2 Auto Scaling sends a message to the notification target.</p>
    pub notification_metadata: Option<String>,
    /// <p>The ARN of the notification target that Amazon EC2 Auto Scaling uses to notify you when an instance is in the transition state for the lifecycle hook. This target can be either an SQS queue or an SNS topic.</p> <p>If you specify an empty string, this overrides the current ARN.</p> <p>This operation uses the JSON format when sending notifications to an Amazon SQS queue, and an email key-value pair format when sending notifications to an Amazon SNS topic.</p> <p>When you specify a notification target, Amazon EC2 Auto Scaling sends it a test message. Test messages contain the following additional key-value pair: <code>"Event": "autoscaling:TEST_NOTIFICATION"</code>.</p>
    pub notification_target_arn: Option<String>,
    /// <p>The ARN of the IAM role that allows the Auto Scaling group to publish to the specified notification target, for example, an Amazon SNS topic or an Amazon SQS queue.</p> <p>Conditional: This parameter is required for new lifecycle hooks, but optional when updating existing hooks.</p>
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
            &obj.auto_scaling_group_name,
        );
        if let Some(ref field_value) = obj.default_result {
            params.put(&format!("{}{}", prefix, "DefaultResult"), &field_value);
        }
        if let Some(ref field_value) = obj.heartbeat_timeout {
            params.put(&format!("{}{}", prefix, "HeartbeatTimeout"), &field_value);
        }
        params.put(
            &format!("{}{}", prefix, "LifecycleHookName"),
            &obj.lifecycle_hook_name,
        );
        if let Some(ref field_value) = obj.lifecycle_transition {
            params.put(
                &format!("{}{}", prefix, "LifecycleTransition"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.notification_metadata {
            params.put(
                &format!("{}{}", prefix, "NotificationMetadata"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.notification_target_arn {
            params.put(
                &format!("{}{}", prefix, "NotificationTargetARN"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.role_arn {
            params.put(&format!("{}{}", prefix, "RoleARN"), &field_value);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutNotificationConfigurationType {
    /// <p>The name of the Auto Scaling group.</p>
    pub auto_scaling_group_name: String,
    /// <p>The type of event that causes the notification to be sent. For more information about notification types supported by Amazon EC2 Auto Scaling, see <a>DescribeAutoScalingNotificationTypes</a>.</p>
    pub notification_types: Vec<String>,
    /// <p>The Amazon Resource Name (ARN) of the Amazon Simple Notification Service (Amazon SNS) topic.</p>
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
            &obj.auto_scaling_group_name,
        );
        AutoScalingNotificationTypesSerializer::serialize(
            params,
            &format!("{}{}", prefix, "NotificationTypes"),
            &obj.notification_types,
        );
        params.put(&format!("{}{}", prefix, "TopicARN"), &obj.topic_arn);
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutScalingPolicyType {
    /// <p>Specifies whether the <code>ScalingAdjustment</code> parameter is an absolute number or a percentage of the current capacity. The valid values are <code>ChangeInCapacity</code>, <code>ExactCapacity</code>, and <code>PercentChangeInCapacity</code>.</p> <p>Valid only if the policy type is <code>StepScaling</code> or <code>SimpleScaling</code>. For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/as-scaling-simple-step.html#as-scaling-adjustment">Scaling Adjustment Types</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    pub adjustment_type: Option<String>,
    /// <p>The name of the Auto Scaling group.</p>
    pub auto_scaling_group_name: String,
    /// <p>The amount of time, in seconds, after a scaling activity completes before any further dynamic scaling activities can start. If this parameter is not specified, the default cooldown period for the group applies.</p> <p>Valid only if the policy type is <code>SimpleScaling</code>. For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/Cooldown.html">Scaling Cooldowns</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    pub cooldown: Option<i64>,
    /// <p>The estimated time, in seconds, until a newly launched instance can contribute to the CloudWatch metrics. The default is to use the value specified for the default cooldown period for the group.</p> <p>Valid only if the policy type is <code>StepScaling</code> or <code>TargetTrackingScaling</code>.</p>
    pub estimated_instance_warmup: Option<i64>,
    /// <p>The aggregation type for the CloudWatch metrics. The valid values are <code>Minimum</code>, <code>Maximum</code>, and <code>Average</code>. If the aggregation type is null, the value is treated as <code>Average</code>.</p> <p>Valid only if the policy type is <code>StepScaling</code>.</p>
    pub metric_aggregation_type: Option<String>,
    /// <p>The minimum number of instances to scale. If the value of <code>AdjustmentType</code> is <code>PercentChangeInCapacity</code>, the scaling policy changes the <code>DesiredCapacity</code> of the Auto Scaling group by at least this many instances. Otherwise, the error is <code>ValidationError</code>.</p> <p>This property replaces the <code>MinAdjustmentStep</code> property. For example, suppose that you create a step scaling policy to scale out an Auto Scaling group by 25 percent and you specify a <code>MinAdjustmentMagnitude</code> of 2. If the group has 4 instances and the scaling policy is performed, 25 percent of 4 is 1. However, because you specified a <code>MinAdjustmentMagnitude</code> of 2, Amazon EC2 Auto Scaling scales out the group by 2 instances.</p> <p>Valid only if the policy type is <code>SimpleScaling</code> or <code>StepScaling</code>.</p>
    pub min_adjustment_magnitude: Option<i64>,
    /// <p>Available for backward compatibility. Use <code>MinAdjustmentMagnitude</code> instead.</p>
    pub min_adjustment_step: Option<i64>,
    /// <p>The name of the policy.</p>
    pub policy_name: String,
    /// <p>The policy type. The valid values are <code>SimpleScaling</code>, <code>StepScaling</code>, and <code>TargetTrackingScaling</code>. If the policy type is null, the value is treated as <code>SimpleScaling</code>.</p>
    pub policy_type: Option<String>,
    /// <p>The amount by which a simple scaling policy scales the Auto Scaling group in response to an alarm breach. The adjustment is based on the value that you specified in the <code>AdjustmentType</code> parameter (either an absolute number or a percentage). A positive value adds to the current capacity and a negative value subtracts from the current capacity. For exact capacity, you must specify a positive value.</p> <p>Conditional: If you specify <code>SimpleScaling</code> for the policy type, you must specify this parameter. (Not used with any other policy type.) </p>
    pub scaling_adjustment: Option<i64>,
    /// <p>A set of adjustments that enable you to scale based on the size of the alarm breach.</p> <p>Conditional: If you specify <code>StepScaling</code> for the policy type, you must specify this parameter. (Not used with any other policy type.) </p>
    pub step_adjustments: Option<Vec<StepAdjustment>>,
    /// <p>A target tracking scaling policy. Includes support for predefined or customized metrics.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/APIReference/API_TargetTrackingConfiguration.html">TargetTrackingConfiguration</a> in the <i>Amazon EC2 Auto Scaling API Reference</i>.</p> <p>Conditional: If you specify <code>TargetTrackingScaling</code> for the policy type, you must specify this parameter. (Not used with any other policy type.) </p>
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
            params.put(&format!("{}{}", prefix, "AdjustmentType"), &field_value);
        }
        params.put(
            &format!("{}{}", prefix, "AutoScalingGroupName"),
            &obj.auto_scaling_group_name,
        );
        if let Some(ref field_value) = obj.cooldown {
            params.put(&format!("{}{}", prefix, "Cooldown"), &field_value);
        }
        if let Some(ref field_value) = obj.estimated_instance_warmup {
            params.put(
                &format!("{}{}", prefix, "EstimatedInstanceWarmup"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.metric_aggregation_type {
            params.put(
                &format!("{}{}", prefix, "MetricAggregationType"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.min_adjustment_magnitude {
            params.put(
                &format!("{}{}", prefix, "MinAdjustmentMagnitude"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.min_adjustment_step {
            params.put(&format!("{}{}", prefix, "MinAdjustmentStep"), &field_value);
        }
        params.put(&format!("{}{}", prefix, "PolicyName"), &obj.policy_name);
        if let Some(ref field_value) = obj.policy_type {
            params.put(&format!("{}{}", prefix, "PolicyType"), &field_value);
        }
        if let Some(ref field_value) = obj.scaling_adjustment {
            params.put(&format!("{}{}", prefix, "ScalingAdjustment"), &field_value);
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutScheduledUpdateGroupActionType {
    /// <p>The name of the Auto Scaling group.</p>
    pub auto_scaling_group_name: String,
    /// <p>The number of EC2 instances that should be running in the Auto Scaling group.</p>
    pub desired_capacity: Option<i64>,
    /// <p>The date and time for the recurring schedule to end. Amazon EC2 Auto Scaling does not perform the action after this time.</p>
    pub end_time: Option<String>,
    /// <p>The maximum number of instances in the Auto Scaling group.</p>
    pub max_size: Option<i64>,
    /// <p>The minimum number of instances in the Auto Scaling group.</p>
    pub min_size: Option<i64>,
    /// <p>The recurring schedule for this action, in Unix cron syntax format. This format consists of five fields separated by white spaces: [Minute] [Hour] [Day_of_Month] [Month_of_Year] [Day_of_Week]. The value must be in quotes (for example, <code>"30 0 1 1,6,12 *"</code>). For more information about this format, see <a href="http://crontab.org">Crontab</a>.</p> <p>When <code>StartTime</code> and <code>EndTime</code> are specified with <code>Recurrence</code>, they form the boundaries of when the recurring action starts and stops.</p>
    pub recurrence: Option<String>,
    /// <p>The name of this scaling action.</p>
    pub scheduled_action_name: String,
    /// <p>The date and time for this action to start, in YYYY-MM-DDThh:mm:ssZ format in UTC/GMT only and in quotes (for example, <code>"2019-06-01T00:00:00Z"</code>).</p> <p>If you specify <code>Recurrence</code> and <code>StartTime</code>, Amazon EC2 Auto Scaling performs the action at this time, and then performs the action based on the specified recurrence.</p> <p>If you try to schedule your action in the past, Amazon EC2 Auto Scaling returns an error message.</p>
    pub start_time: Option<String>,
    /// <p>This parameter is no longer used.</p>
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
            &obj.auto_scaling_group_name,
        );
        if let Some(ref field_value) = obj.desired_capacity {
            params.put(&format!("{}{}", prefix, "DesiredCapacity"), &field_value);
        }
        if let Some(ref field_value) = obj.end_time {
            params.put(&format!("{}{}", prefix, "EndTime"), &field_value);
        }
        if let Some(ref field_value) = obj.max_size {
            params.put(&format!("{}{}", prefix, "MaxSize"), &field_value);
        }
        if let Some(ref field_value) = obj.min_size {
            params.put(&format!("{}{}", prefix, "MinSize"), &field_value);
        }
        if let Some(ref field_value) = obj.recurrence {
            params.put(&format!("{}{}", prefix, "Recurrence"), &field_value);
        }
        params.put(
            &format!("{}{}", prefix, "ScheduledActionName"),
            &obj.scheduled_action_name,
        );
        if let Some(ref field_value) = obj.start_time {
            params.put(&format!("{}{}", prefix, "StartTime"), &field_value);
        }
        if let Some(ref field_value) = obj.time {
            params.put(&format!("{}{}", prefix, "Time"), &field_value);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct RecordLifecycleActionHeartbeatAnswer {}

struct RecordLifecycleActionHeartbeatAnswerDeserializer;
impl RecordLifecycleActionHeartbeatAnswerDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<RecordLifecycleActionHeartbeatAnswer, XmlParseError> {
        start_element(tag_name, stack)?;

        let obj = RecordLifecycleActionHeartbeatAnswer::default();

        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RecordLifecycleActionHeartbeatType {
    /// <p>The name of the Auto Scaling group.</p>
    pub auto_scaling_group_name: String,
    /// <p>The ID of the instance.</p>
    pub instance_id: Option<String>,
    /// <p>A token that uniquely identifies a specific lifecycle action associated with an instance. Amazon EC2 Auto Scaling sends this token to the notification target that you specified when you created the lifecycle hook.</p>
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
            &obj.auto_scaling_group_name,
        );
        if let Some(ref field_value) = obj.instance_id {
            params.put(&format!("{}{}", prefix, "InstanceId"), &field_value);
        }
        if let Some(ref field_value) = obj.lifecycle_action_token {
            params.put(
                &format!("{}{}", prefix, "LifecycleActionToken"),
                &field_value,
            );
        }
        params.put(
            &format!("{}{}", prefix, "LifecycleHookName"),
            &obj.lifecycle_hook_name,
        );
    }
}

struct ResourceNameDeserializer;
impl ResourceNameDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct ScalingActivityStatusCodeDeserializer;
impl ScalingActivityStatusCodeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct ScalingPoliciesDeserializer;
impl ScalingPoliciesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<ScalingPolicy>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(ScalingPolicyDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
/// <p>Describes a scaling policy.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ScalingPolicy {
    /// <p>The adjustment type, which specifies how <code>ScalingAdjustment</code> is interpreted. The valid values are <code>ChangeInCapacity</code>, <code>ExactCapacity</code>, and <code>PercentChangeInCapacity</code>.</p>
    pub adjustment_type: Option<String>,
    /// <p>The CloudWatch alarms related to the policy.</p>
    pub alarms: Option<Vec<Alarm>>,
    /// <p>The name of the Auto Scaling group.</p>
    pub auto_scaling_group_name: Option<String>,
    /// <p>The amount of time, in seconds, after a scaling activity completes before any further dynamic scaling activities can start.</p>
    pub cooldown: Option<i64>,
    /// <p>The estimated time, in seconds, until a newly launched instance can contribute to the CloudWatch metrics.</p>
    pub estimated_instance_warmup: Option<i64>,
    /// <p>The aggregation type for the CloudWatch metrics. The valid values are <code>Minimum</code>, <code>Maximum</code>, and <code>Average</code>.</p>
    pub metric_aggregation_type: Option<String>,
    /// <p>The minimum number of instances to scale. If the value of <code>AdjustmentType</code> is <code>PercentChangeInCapacity</code>, the scaling policy changes the <code>DesiredCapacity</code> of the Auto Scaling group by at least this many instances. Otherwise, the error is <code>ValidationError</code>.</p>
    pub min_adjustment_magnitude: Option<i64>,
    /// <p>Available for backward compatibility. Use <code>MinAdjustmentMagnitude</code> instead.</p>
    pub min_adjustment_step: Option<i64>,
    /// <p>The Amazon Resource Name (ARN) of the policy.</p>
    pub policy_arn: Option<String>,
    /// <p>The name of the scaling policy.</p>
    pub policy_name: Option<String>,
    /// <p>The policy type. The valid values are <code>SimpleScaling</code>, <code>StepScaling</code>, and <code>TargetTrackingScaling</code>.</p>
    pub policy_type: Option<String>,
    /// <p>The amount by which to scale, based on the specified adjustment type. A positive value adds to the current capacity while a negative number removes from the current capacity.</p>
    pub scaling_adjustment: Option<i64>,
    /// <p>A set of adjustments that enable you to scale based on the size of the alarm breach.</p>
    pub step_adjustments: Option<Vec<StepAdjustment>>,
    /// <p>A target tracking scaling policy.</p>
    pub target_tracking_configuration: Option<TargetTrackingConfiguration>,
}

struct ScalingPolicyDeserializer;
impl ScalingPolicyDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ScalingPolicy, XmlParseError> {
        deserialize_elements::<_, ScalingPolicy, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "AdjustmentType" => {
                    obj.adjustment_type = Some(XmlStringMaxLen255Deserializer::deserialize(
                        "AdjustmentType",
                        stack,
                    )?);
                }
                "Alarms" => {
                    obj.alarms
                        .get_or_insert(vec![])
                        .extend(AlarmsDeserializer::deserialize("Alarms", stack)?);
                }
                "AutoScalingGroupName" => {
                    obj.auto_scaling_group_name = Some(
                        XmlStringMaxLen255Deserializer::deserialize("AutoScalingGroupName", stack)?,
                    );
                }
                "Cooldown" => {
                    obj.cooldown = Some(CooldownDeserializer::deserialize("Cooldown", stack)?);
                }
                "EstimatedInstanceWarmup" => {
                    obj.estimated_instance_warmup =
                        Some(EstimatedInstanceWarmupDeserializer::deserialize(
                            "EstimatedInstanceWarmup",
                            stack,
                        )?);
                }
                "MetricAggregationType" => {
                    obj.metric_aggregation_type = Some(XmlStringMaxLen32Deserializer::deserialize(
                        "MetricAggregationType",
                        stack,
                    )?);
                }
                "MinAdjustmentMagnitude" => {
                    obj.min_adjustment_magnitude =
                        Some(MinAdjustmentMagnitudeDeserializer::deserialize(
                            "MinAdjustmentMagnitude",
                            stack,
                        )?);
                }
                "MinAdjustmentStep" => {
                    obj.min_adjustment_step = Some(MinAdjustmentStepDeserializer::deserialize(
                        "MinAdjustmentStep",
                        stack,
                    )?);
                }
                "PolicyARN" => {
                    obj.policy_arn =
                        Some(ResourceNameDeserializer::deserialize("PolicyARN", stack)?);
                }
                "PolicyName" => {
                    obj.policy_name = Some(XmlStringMaxLen255Deserializer::deserialize(
                        "PolicyName",
                        stack,
                    )?);
                }
                "PolicyType" => {
                    obj.policy_type = Some(XmlStringMaxLen64Deserializer::deserialize(
                        "PolicyType",
                        stack,
                    )?);
                }
                "ScalingAdjustment" => {
                    obj.scaling_adjustment = Some(PolicyIncrementDeserializer::deserialize(
                        "ScalingAdjustment",
                        stack,
                    )?);
                }
                "StepAdjustments" => {
                    obj.step_adjustments.get_or_insert(vec![]).extend(
                        StepAdjustmentsDeserializer::deserialize("StepAdjustments", stack)?,
                    );
                }
                "TargetTrackingConfiguration" => {
                    obj.target_tracking_configuration =
                        Some(TargetTrackingConfigurationDeserializer::deserialize(
                            "TargetTrackingConfiguration",
                            stack,
                        )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
            &obj.auto_scaling_group_name,
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
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ScheduledActionsType {
    /// <p>A string that indicates that the response contains more items than can be returned in a single response. To receive additional items, specify this string for the <code>NextToken</code> value when requesting the next set of items. This value is null when there are no more items to return.</p>
    pub next_token: Option<String>,
    /// <p>The scheduled actions.</p>
    pub scheduled_update_group_actions: Option<Vec<ScheduledUpdateGroupAction>>,
}

struct ScheduledActionsTypeDeserializer;
impl ScheduledActionsTypeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ScheduledActionsType, XmlParseError> {
        deserialize_elements::<_, ScheduledActionsType, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "NextToken" => {
                    obj.next_token = Some(XmlStringDeserializer::deserialize("NextToken", stack)?);
                }
                "ScheduledUpdateGroupActions" => {
                    obj.scheduled_update_group_actions
                        .get_or_insert(vec![])
                        .extend(ScheduledUpdateGroupActionsDeserializer::deserialize(
                            "ScheduledUpdateGroupActions",
                            stack,
                        )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
/// <p>Describes a scheduled scaling action. Used in response to <a>DescribeScheduledActions</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ScheduledUpdateGroupAction {
    /// <p>The name of the Auto Scaling group.</p>
    pub auto_scaling_group_name: Option<String>,
    /// <p>The number of instances you prefer to maintain in the group.</p>
    pub desired_capacity: Option<i64>,
    /// <p>The date and time in UTC for the recurring schedule to end. For example, <code>"2019-06-01T00:00:00Z"</code>. </p>
    pub end_time: Option<String>,
    /// <p>The maximum number of instances in the Auto Scaling group.</p>
    pub max_size: Option<i64>,
    /// <p>The minimum number of instances in the Auto Scaling group.</p>
    pub min_size: Option<i64>,
    /// <p>The recurring schedule for the action, in Unix cron syntax format.</p> <p>When <code>StartTime</code> and <code>EndTime</code> are specified with <code>Recurrence</code>, they form the boundaries of when the recurring action starts and stops.</p>
    pub recurrence: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the scheduled action.</p>
    pub scheduled_action_arn: Option<String>,
    /// <p>The name of the scheduled action.</p>
    pub scheduled_action_name: Option<String>,
    /// <p>The date and time in UTC for this action to start. For example, <code>"2019-06-01T00:00:00Z"</code>. </p>
    pub start_time: Option<String>,
    /// <p>This parameter is no longer used.</p>
    pub time: Option<String>,
}

struct ScheduledUpdateGroupActionDeserializer;
impl ScheduledUpdateGroupActionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ScheduledUpdateGroupAction, XmlParseError> {
        deserialize_elements::<_, ScheduledUpdateGroupAction, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "AutoScalingGroupName" => {
                        obj.auto_scaling_group_name =
                            Some(XmlStringMaxLen255Deserializer::deserialize(
                                "AutoScalingGroupName",
                                stack,
                            )?);
                    }
                    "DesiredCapacity" => {
                        obj.desired_capacity =
                            Some(AutoScalingGroupDesiredCapacityDeserializer::deserialize(
                                "DesiredCapacity",
                                stack,
                            )?);
                    }
                    "EndTime" => {
                        obj.end_time =
                            Some(TimestampTypeDeserializer::deserialize("EndTime", stack)?);
                    }
                    "MaxSize" => {
                        obj.max_size = Some(AutoScalingGroupMaxSizeDeserializer::deserialize(
                            "MaxSize", stack,
                        )?);
                    }
                    "MinSize" => {
                        obj.min_size = Some(AutoScalingGroupMinSizeDeserializer::deserialize(
                            "MinSize", stack,
                        )?);
                    }
                    "Recurrence" => {
                        obj.recurrence = Some(XmlStringMaxLen255Deserializer::deserialize(
                            "Recurrence",
                            stack,
                        )?);
                    }
                    "ScheduledActionARN" => {
                        obj.scheduled_action_arn = Some(ResourceNameDeserializer::deserialize(
                            "ScheduledActionARN",
                            stack,
                        )?);
                    }
                    "ScheduledActionName" => {
                        obj.scheduled_action_name =
                            Some(XmlStringMaxLen255Deserializer::deserialize(
                                "ScheduledActionName",
                                stack,
                            )?);
                    }
                    "StartTime" => {
                        obj.start_time =
                            Some(TimestampTypeDeserializer::deserialize("StartTime", stack)?);
                    }
                    "Time" => {
                        obj.time = Some(TimestampTypeDeserializer::deserialize("Time", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p>Describes one or more scheduled scaling action updates for a specified Auto Scaling group. Used in combination with <a>BatchPutScheduledUpdateGroupAction</a>.</p> <p>When updating a scheduled scaling action, all optional parameters are left unchanged if not specified.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ScheduledUpdateGroupActionRequest {
    /// <p>The number of EC2 instances that should be running in the group.</p>
    pub desired_capacity: Option<i64>,
    /// <p>The date and time for the recurring schedule to end. Amazon EC2 Auto Scaling does not perform the action after this time.</p>
    pub end_time: Option<String>,
    /// <p>The maximum number of instances in the Auto Scaling group.</p>
    pub max_size: Option<i64>,
    /// <p>The minimum number of instances in the Auto Scaling group.</p>
    pub min_size: Option<i64>,
    /// <p>The recurring schedule for the action, in Unix cron syntax format. This format consists of five fields separated by white spaces: [Minute] [Hour] [Day_of_Month] [Month_of_Year] [Day_of_Week]. The value must be in quotes (for example, <code>"30 0 1 1,6,12 *"</code>). For more information about this format, see <a href="http://crontab.org">Crontab</a>.</p> <p>When <code>StartTime</code> and <code>EndTime</code> are specified with <code>Recurrence</code>, they form the boundaries of when the recurring action starts and stops.</p>
    pub recurrence: Option<String>,
    /// <p>The name of the scaling action.</p>
    pub scheduled_action_name: String,
    /// <p>The date and time for the action to start, in YYYY-MM-DDThh:mm:ssZ format in UTC/GMT only and in quotes (for example, <code>"2019-06-01T00:00:00Z"</code>).</p> <p>If you specify <code>Recurrence</code> and <code>StartTime</code>, Amazon EC2 Auto Scaling performs the action at this time, and then performs the action based on the specified recurrence.</p> <p>If you try to schedule the action in the past, Amazon EC2 Auto Scaling returns an error message.</p>
    pub start_time: Option<String>,
}

/// Serialize `ScheduledUpdateGroupActionRequest` contents to a `SignedRequest`.
struct ScheduledUpdateGroupActionRequestSerializer;
impl ScheduledUpdateGroupActionRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ScheduledUpdateGroupActionRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.desired_capacity {
            params.put(&format!("{}{}", prefix, "DesiredCapacity"), &field_value);
        }
        if let Some(ref field_value) = obj.end_time {
            params.put(&format!("{}{}", prefix, "EndTime"), &field_value);
        }
        if let Some(ref field_value) = obj.max_size {
            params.put(&format!("{}{}", prefix, "MaxSize"), &field_value);
        }
        if let Some(ref field_value) = obj.min_size {
            params.put(&format!("{}{}", prefix, "MinSize"), &field_value);
        }
        if let Some(ref field_value) = obj.recurrence {
            params.put(&format!("{}{}", prefix, "Recurrence"), &field_value);
        }
        params.put(
            &format!("{}{}", prefix, "ScheduledActionName"),
            &obj.scheduled_action_name,
        );
        if let Some(ref field_value) = obj.start_time {
            params.put(&format!("{}{}", prefix, "StartTime"), &field_value);
        }
    }
}

/// Serialize `ScheduledUpdateGroupActionRequests` contents to a `SignedRequest`.
struct ScheduledUpdateGroupActionRequestsSerializer;
impl ScheduledUpdateGroupActionRequestsSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<ScheduledUpdateGroupActionRequest>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            ScheduledUpdateGroupActionRequestSerializer::serialize(params, &key, obj);
        }
    }
}

struct ScheduledUpdateGroupActionsDeserializer;
impl ScheduledUpdateGroupActionsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<ScheduledUpdateGroupAction>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(ScheduledUpdateGroupActionDeserializer::deserialize(
                    "member", stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
struct SecurityGroupsDeserializer;
impl SecurityGroupsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(XmlStringDeserializer::deserialize("member", stack)?);
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

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SetDesiredCapacityType {
    /// <p>The name of the Auto Scaling group.</p>
    pub auto_scaling_group_name: String,
    /// <p>The number of EC2 instances that should be running in the Auto Scaling group.</p>
    pub desired_capacity: i64,
    /// <p>Indicates whether Amazon EC2 Auto Scaling waits for the cooldown period to complete before initiating a scaling activity to set your Auto Scaling group to its new capacity. By default, Amazon EC2 Auto Scaling does not honor the cooldown period during manual scaling activities.</p>
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
            &obj.auto_scaling_group_name,
        );
        params.put(
            &format!("{}{}", prefix, "DesiredCapacity"),
            &obj.desired_capacity,
        );
        if let Some(ref field_value) = obj.honor_cooldown {
            params.put(&format!("{}{}", prefix, "HonorCooldown"), &field_value);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SetInstanceHealthQuery {
    /// <p>The health status of the instance. Set to <code>Healthy</code> to have the instance remain in service. Set to <code>Unhealthy</code> to have the instance be out of service. Amazon EC2 Auto Scaling terminates and replaces the unhealthy instance.</p>
    pub health_status: String,
    /// <p>The ID of the instance.</p>
    pub instance_id: String,
    /// <p>If the Auto Scaling group of the specified instance has a <code>HealthCheckGracePeriod</code> specified for the group, by default, this call respects the grace period. Set this to <code>False</code>, to have the call not respect the grace period associated with the group.</p> <p>For more information about the health check grace period, see <a>CreateAutoScalingGroup</a>.</p>
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

        params.put(&format!("{}{}", prefix, "HealthStatus"), &obj.health_status);
        params.put(&format!("{}{}", prefix, "InstanceId"), &obj.instance_id);
        if let Some(ref field_value) = obj.should_respect_grace_period {
            params.put(
                &format!("{}{}", prefix, "ShouldRespectGracePeriod"),
                &field_value,
            );
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct SetInstanceProtectionAnswer {}

struct SetInstanceProtectionAnswerDeserializer;
impl SetInstanceProtectionAnswerDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<SetInstanceProtectionAnswer, XmlParseError> {
        start_element(tag_name, stack)?;

        let obj = SetInstanceProtectionAnswer::default();

        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SetInstanceProtectionQuery {
    /// <p>The name of the Auto Scaling group.</p>
    pub auto_scaling_group_name: String,
    /// <p>One or more instance IDs.</p>
    pub instance_ids: Vec<String>,
    /// <p>Indicates whether the instance is protected from termination by Amazon EC2 Auto Scaling when scaling in.</p>
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
            &obj.auto_scaling_group_name,
        );
        InstanceIdsSerializer::serialize(
            params,
            &format!("{}{}", prefix, "InstanceIds"),
            &obj.instance_ids,
        );
        params.put(
            &format!("{}{}", prefix, "ProtectedFromScaleIn"),
            &obj.protected_from_scale_in,
        );
    }
}

struct SpotInstancePoolsDeserializer;
impl SpotInstancePoolsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = i64::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct SpotPriceDeserializer;
impl SpotPriceDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p><p>Describes an adjustment based on the difference between the value of the aggregated CloudWatch metric and the breach threshold that you&#39;ve defined for the alarm. Used in combination with <a>PutScalingPolicy</a>.</p> <p>For the following examples, suppose that you have an alarm with a breach threshold of 50:</p> <ul> <li> <p>To trigger the adjustment when the metric is greater than or equal to 50 and less than 60, specify a lower bound of 0 and an upper bound of 10.</p> </li> <li> <p>To trigger the adjustment when the metric is greater than 40 and less than or equal to 50, specify a lower bound of -10 and an upper bound of 0.</p> </li> </ul> <p>There are a few rules for the step adjustments for your step policy:</p> <ul> <li> <p>The ranges of your step adjustments can&#39;t overlap or have a gap.</p> </li> <li> <p>At most, one step adjustment can have a null lower bound. If one step adjustment has a negative lower bound, then there must be a step adjustment with a null lower bound.</p> </li> <li> <p>At most, one step adjustment can have a null upper bound. If one step adjustment has a positive upper bound, then there must be a step adjustment with a null upper bound.</p> </li> <li> <p>The upper and lower bound can&#39;t be null in the same step adjustment.</p> </li> </ul></p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<StepAdjustment, XmlParseError> {
        deserialize_elements::<_, StepAdjustment, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "MetricIntervalLowerBound" => {
                    obj.metric_interval_lower_bound = Some(MetricScaleDeserializer::deserialize(
                        "MetricIntervalLowerBound",
                        stack,
                    )?);
                }
                "MetricIntervalUpperBound" => {
                    obj.metric_interval_upper_bound = Some(MetricScaleDeserializer::deserialize(
                        "MetricIntervalUpperBound",
                        stack,
                    )?);
                }
                "ScalingAdjustment" => {
                    obj.scaling_adjustment =
                        PolicyIncrementDeserializer::deserialize("ScalingAdjustment", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
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
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.metric_interval_upper_bound {
            params.put(
                &format!("{}{}", prefix, "MetricIntervalUpperBound"),
                &field_value,
            );
        }
        params.put(
            &format!("{}{}", prefix, "ScalingAdjustment"),
            &obj.scaling_adjustment,
        );
    }
}

struct StepAdjustmentsDeserializer;
impl StepAdjustmentsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<StepAdjustment>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(StepAdjustmentDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
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

/// <p>Describes an automatic scaling process that has been suspended. For more information, see <a>ProcessType</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct SuspendedProcess {
    /// <p>The name of the suspended process.</p>
    pub process_name: Option<String>,
    /// <p>The reason that the process was suspended.</p>
    pub suspension_reason: Option<String>,
}

struct SuspendedProcessDeserializer;
impl SuspendedProcessDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<SuspendedProcess, XmlParseError> {
        deserialize_elements::<_, SuspendedProcess, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "ProcessName" => {
                    obj.process_name = Some(XmlStringMaxLen255Deserializer::deserialize(
                        "ProcessName",
                        stack,
                    )?);
                }
                "SuspensionReason" => {
                    obj.suspension_reason = Some(XmlStringMaxLen255Deserializer::deserialize(
                        "SuspensionReason",
                        stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct SuspendedProcessesDeserializer;
impl SuspendedProcessesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<SuspendedProcess>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(SuspendedProcessDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
/// <p>Describes a tag for an Auto Scaling group.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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

        params.put(&format!("{}{}", prefix, "Key"), &obj.key);
        if let Some(ref field_value) = obj.propagate_at_launch {
            params.put(&format!("{}{}", prefix, "PropagateAtLaunch"), &field_value);
        }
        if let Some(ref field_value) = obj.resource_id {
            params.put(&format!("{}{}", prefix, "ResourceId"), &field_value);
        }
        if let Some(ref field_value) = obj.resource_type {
            params.put(&format!("{}{}", prefix, "ResourceType"), &field_value);
        }
        if let Some(ref field_value) = obj.value {
            params.put(&format!("{}{}", prefix, "Value"), &field_value);
        }
    }
}

/// <p>Describes a tag for an Auto Scaling group.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
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
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<TagDescription, XmlParseError> {
        deserialize_elements::<_, TagDescription, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Key" => {
                    obj.key = Some(TagKeyDeserializer::deserialize("Key", stack)?);
                }
                "PropagateAtLaunch" => {
                    obj.propagate_at_launch = Some(PropagateAtLaunchDeserializer::deserialize(
                        "PropagateAtLaunch",
                        stack,
                    )?);
                }
                "ResourceId" => {
                    obj.resource_id =
                        Some(XmlStringDeserializer::deserialize("ResourceId", stack)?);
                }
                "ResourceType" => {
                    obj.resource_type =
                        Some(XmlStringDeserializer::deserialize("ResourceType", stack)?);
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
struct TagDescriptionListDeserializer;
impl TagDescriptionListDeserializer {
    #[allow(unused_variables)]
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
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct TagsType {
    /// <p>A string that indicates that the response contains more items than can be returned in a single response. To receive additional items, specify this string for the <code>NextToken</code> value when requesting the next set of items. This value is null when there are no more items to return.</p>
    pub next_token: Option<String>,
    /// <p>One or more tags.</p>
    pub tags: Option<Vec<TagDescription>>,
}

struct TagsTypeDeserializer;
impl TagsTypeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<TagsType, XmlParseError> {
        deserialize_elements::<_, TagsType, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "NextToken" => {
                    obj.next_token = Some(XmlStringDeserializer::deserialize("NextToken", stack)?);
                }
                "Tags" => {
                    obj.tags
                        .get_or_insert(vec![])
                        .extend(TagDescriptionListDeserializer::deserialize("Tags", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct TargetGroupARNsDeserializer;
impl TargetGroupARNsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(XmlStringMaxLen511Deserializer::deserialize(
                    "member", stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
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

/// <p>Represents a target tracking scaling policy configuration to use with Amazon EC2 Auto Scaling.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TargetTrackingConfiguration {
    /// <p>A customized metric. You must specify either a predefined metric or a customized metric.</p>
    pub customized_metric_specification: Option<CustomizedMetricSpecification>,
    /// <p>Indicates whether scaling in by the target tracking scaling policy is disabled. If scaling in is disabled, the target tracking scaling policy doesn't remove instances from the Auto Scaling group. Otherwise, the target tracking scaling policy can remove instances from the Auto Scaling group. The default is <code>false</code>.</p>
    pub disable_scale_in: Option<bool>,
    /// <p>A predefined metric. You must specify either a predefined metric or a customized metric.</p>
    pub predefined_metric_specification: Option<PredefinedMetricSpecification>,
    /// <p>The target value for the metric.</p>
    pub target_value: f64,
}

struct TargetTrackingConfigurationDeserializer;
impl TargetTrackingConfigurationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<TargetTrackingConfiguration, XmlParseError> {
        deserialize_elements::<_, TargetTrackingConfiguration, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "CustomizedMetricSpecification" => {
                        obj.customized_metric_specification =
                            Some(CustomizedMetricSpecificationDeserializer::deserialize(
                                "CustomizedMetricSpecification",
                                stack,
                            )?);
                    }
                    "DisableScaleIn" => {
                        obj.disable_scale_in = Some(DisableScaleInDeserializer::deserialize(
                            "DisableScaleIn",
                            stack,
                        )?);
                    }
                    "PredefinedMetricSpecification" => {
                        obj.predefined_metric_specification =
                            Some(PredefinedMetricSpecificationDeserializer::deserialize(
                                "PredefinedMetricSpecification",
                                stack,
                            )?);
                    }
                    "TargetValue" => {
                        obj.target_value =
                            MetricScaleDeserializer::deserialize("TargetValue", stack)?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
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
            params.put(&format!("{}{}", prefix, "DisableScaleIn"), &field_value);
        }
        if let Some(ref field_value) = obj.predefined_metric_specification {
            PredefinedMetricSpecificationSerializer::serialize(
                params,
                &format!("{}{}", prefix, "PredefinedMetricSpecification"),
                field_value,
            );
        }
        params.put(&format!("{}{}", prefix, "TargetValue"), &obj.target_value);
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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

        params.put(&format!("{}{}", prefix, "InstanceId"), &obj.instance_id);
        params.put(
            &format!("{}{}", prefix, "ShouldDecrementDesiredCapacity"),
            &obj.should_decrement_desired_capacity,
        );
    }
}

struct TerminationPoliciesDeserializer;
impl TerminationPoliciesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(XmlStringMaxLen1600Deserializer::deserialize(
                    "member", stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
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
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateAutoScalingGroupType {
    /// <p>The name of the Auto Scaling group.</p>
    pub auto_scaling_group_name: String,
    /// <p>One or more Availability Zones for the group.</p>
    pub availability_zones: Option<Vec<String>>,
    /// <p>The amount of time, in seconds, after a scaling activity completes before another scaling activity can start. The default value is <code>300</code>. This cooldown period is not used when a scaling-specific cooldown is specified.</p> <p>Cooldown periods are not supported for target tracking scaling policies, step scaling policies, or scheduled scaling. For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/Cooldown.html">Scaling Cooldowns</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    pub default_cooldown: Option<i64>,
    /// <p>The number of EC2 instances that should be running in the Auto Scaling group. This number must be greater than or equal to the minimum size of the group and less than or equal to the maximum size of the group.</p>
    pub desired_capacity: Option<i64>,
    /// <p>The amount of time, in seconds, that Amazon EC2 Auto Scaling waits before checking the health status of an EC2 instance that has come into service. The default value is <code>0</code>.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/healthcheck.html#health-check-grace-period">Health Check Grace Period</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p> <p>Conditional: This parameter is required if you are adding an <code>ELB</code> health check.</p>
    pub health_check_grace_period: Option<i64>,
    /// <p>The service to use for the health checks. The valid values are <code>EC2</code> and <code>ELB</code>. If you configure an Auto Scaling group to use ELB health checks, it considers the instance unhealthy if it fails either the EC2 status checks or the load balancer health checks.</p>
    pub health_check_type: Option<String>,
    /// <p>The name of the launch configuration. If you specify <code>LaunchConfigurationName</code> in your update request, you can't specify <code>LaunchTemplate</code> or <code>MixedInstancesPolicy</code>.</p>
    pub launch_configuration_name: Option<String>,
    /// <p>The launch template and version to use to specify the updates. If you specify <code>LaunchTemplate</code> in your update request, you can't specify <code>LaunchConfigurationName</code> or <code>MixedInstancesPolicy</code>.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/APIReference/API_LaunchTemplateSpecification.html">LaunchTemplateSpecification</a> in the <i>Amazon EC2 Auto Scaling API Reference</i>.</p>
    pub launch_template: Option<LaunchTemplateSpecification>,
    /// <p>The maximum amount of time, in seconds, that an instance can be in service.</p> <p>Valid Range: Minimum value of 604800.</p>
    pub max_instance_lifetime: Option<i64>,
    /// <p>The maximum size of the Auto Scaling group.</p>
    pub max_size: Option<i64>,
    /// <p>The minimum size of the Auto Scaling group.</p>
    pub min_size: Option<i64>,
    /// <p>An embedded object that specifies a mixed instances policy.</p> <p>In your call to <code>UpdateAutoScalingGroup</code>, you can make changes to the policy that is specified. All optional parameters are left unchanged if not specified.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/APIReference/API_MixedInstancesPolicy.html">MixedInstancesPolicy</a> in the <i>Amazon EC2 Auto Scaling API Reference</i> and <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/asg-purchase-options.html">Auto Scaling Groups with Multiple Instance Types and Purchase Options</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    pub mixed_instances_policy: Option<MixedInstancesPolicy>,
    /// <p>Indicates whether newly launched instances are protected from termination by Amazon EC2 Auto Scaling when scaling in.</p> <p>For more information about preventing instances from terminating on scale in, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/as-instance-termination.html#instance-protection">Instance Protection</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    pub new_instances_protected_from_scale_in: Option<bool>,
    /// <p>The name of the placement group into which to launch your instances, if any. A placement group is a logical grouping of instances within a single Availability Zone. You cannot specify multiple Availability Zones and a placement group. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/placement-groups.html">Placement Groups</a> in the <i>Amazon EC2 User Guide for Linux Instances</i>.</p>
    pub placement_group: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the service-linked role that the Auto Scaling group uses to call other AWS services on your behalf. For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/autoscaling-service-linked-role.html">Service-Linked Roles</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    pub service_linked_role_arn: Option<String>,
    /// <p>A standalone termination policy or a list of termination policies used to select the instance to terminate. The policies are executed in the order that they are listed.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/as-instance-termination.html">Controlling Which Instances Auto Scaling Terminates During Scale In</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    pub termination_policies: Option<Vec<String>>,
    /// <p>A comma-separated list of subnet IDs for virtual private cloud (VPC).</p> <p>If you specify <code>VPCZoneIdentifier</code> with <code>AvailabilityZones</code>, the subnets that you specify for this parameter must reside in those Availability Zones.</p>
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
            &obj.auto_scaling_group_name,
        );
        if let Some(ref field_value) = obj.availability_zones {
            AvailabilityZonesSerializer::serialize(
                params,
                &format!("{}{}", prefix, "AvailabilityZones"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.default_cooldown {
            params.put(&format!("{}{}", prefix, "DefaultCooldown"), &field_value);
        }
        if let Some(ref field_value) = obj.desired_capacity {
            params.put(&format!("{}{}", prefix, "DesiredCapacity"), &field_value);
        }
        if let Some(ref field_value) = obj.health_check_grace_period {
            params.put(
                &format!("{}{}", prefix, "HealthCheckGracePeriod"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.health_check_type {
            params.put(&format!("{}{}", prefix, "HealthCheckType"), &field_value);
        }
        if let Some(ref field_value) = obj.launch_configuration_name {
            params.put(
                &format!("{}{}", prefix, "LaunchConfigurationName"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.launch_template {
            LaunchTemplateSpecificationSerializer::serialize(
                params,
                &format!("{}{}", prefix, "LaunchTemplate"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.max_instance_lifetime {
            params.put(
                &format!("{}{}", prefix, "MaxInstanceLifetime"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.max_size {
            params.put(&format!("{}{}", prefix, "MaxSize"), &field_value);
        }
        if let Some(ref field_value) = obj.min_size {
            params.put(&format!("{}{}", prefix, "MinSize"), &field_value);
        }
        if let Some(ref field_value) = obj.mixed_instances_policy {
            MixedInstancesPolicySerializer::serialize(
                params,
                &format!("{}{}", prefix, "MixedInstancesPolicy"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.new_instances_protected_from_scale_in {
            params.put(
                &format!("{}{}", prefix, "NewInstancesProtectedFromScaleIn"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.placement_group {
            params.put(&format!("{}{}", prefix, "PlacementGroup"), &field_value);
        }
        if let Some(ref field_value) = obj.service_linked_role_arn {
            params.put(
                &format!("{}{}", prefix, "ServiceLinkedRoleARN"),
                &field_value,
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
            params.put(&format!("{}{}", prefix, "VPCZoneIdentifier"), &field_value);
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
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct XmlStringMaxLen1023Deserializer;
impl XmlStringMaxLen1023Deserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct XmlStringMaxLen1600Deserializer;
impl XmlStringMaxLen1600Deserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct XmlStringMaxLen19Deserializer;
impl XmlStringMaxLen19Deserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct XmlStringMaxLen2047Deserializer;
impl XmlStringMaxLen2047Deserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct XmlStringMaxLen255Deserializer;
impl XmlStringMaxLen255Deserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct XmlStringMaxLen32Deserializer;
impl XmlStringMaxLen32Deserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct XmlStringMaxLen511Deserializer;
impl XmlStringMaxLen511Deserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct XmlStringMaxLen64Deserializer;
impl XmlStringMaxLen64Deserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct XmlStringUserDataDeserializer;
impl XmlStringUserDataDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// Errors returned by AttachInstances
#[derive(Debug, PartialEq)]
pub enum AttachInstancesError {
    /// <p>You already have a pending update to an Amazon EC2 Auto Scaling resource (for example, an Auto Scaling group, instance, or load balancer).</p>
    ResourceContentionFault(String),
    /// <p>The service-linked role is not yet ready for use.</p>
    ServiceLinkedRoleFailure(String),
}

impl AttachInstancesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AttachInstancesError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "ResourceContention" => {
                        return RusotoError::Service(AttachInstancesError::ResourceContentionFault(
                            parsed_error.message,
                        ))
                    }
                    "ServiceLinkedRoleFailure" => {
                        return RusotoError::Service(
                            AttachInstancesError::ServiceLinkedRoleFailure(parsed_error.message),
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
impl fmt::Display for AttachInstancesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AttachInstancesError::ResourceContentionFault(ref cause) => write!(f, "{}", cause),
            AttachInstancesError::ServiceLinkedRoleFailure(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AttachInstancesError {}
/// Errors returned by AttachLoadBalancerTargetGroups
#[derive(Debug, PartialEq)]
pub enum AttachLoadBalancerTargetGroupsError {
    /// <p>You already have a pending update to an Amazon EC2 Auto Scaling resource (for example, an Auto Scaling group, instance, or load balancer).</p>
    ResourceContentionFault(String),
    /// <p>The service-linked role is not yet ready for use.</p>
    ServiceLinkedRoleFailure(String),
}

impl AttachLoadBalancerTargetGroupsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<AttachLoadBalancerTargetGroupsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "ResourceContention" => {
                        return RusotoError::Service(
                            AttachLoadBalancerTargetGroupsError::ResourceContentionFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "ServiceLinkedRoleFailure" => {
                        return RusotoError::Service(
                            AttachLoadBalancerTargetGroupsError::ServiceLinkedRoleFailure(
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
impl fmt::Display for AttachLoadBalancerTargetGroupsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AttachLoadBalancerTargetGroupsError::ResourceContentionFault(ref cause) => {
                write!(f, "{}", cause)
            }
            AttachLoadBalancerTargetGroupsError::ServiceLinkedRoleFailure(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for AttachLoadBalancerTargetGroupsError {}
/// Errors returned by AttachLoadBalancers
#[derive(Debug, PartialEq)]
pub enum AttachLoadBalancersError {
    /// <p>You already have a pending update to an Amazon EC2 Auto Scaling resource (for example, an Auto Scaling group, instance, or load balancer).</p>
    ResourceContentionFault(String),
    /// <p>The service-linked role is not yet ready for use.</p>
    ServiceLinkedRoleFailure(String),
}

impl AttachLoadBalancersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AttachLoadBalancersError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "ResourceContention" => {
                        return RusotoError::Service(
                            AttachLoadBalancersError::ResourceContentionFault(parsed_error.message),
                        )
                    }
                    "ServiceLinkedRoleFailure" => {
                        return RusotoError::Service(
                            AttachLoadBalancersError::ServiceLinkedRoleFailure(
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
impl fmt::Display for AttachLoadBalancersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AttachLoadBalancersError::ResourceContentionFault(ref cause) => write!(f, "{}", cause),
            AttachLoadBalancersError::ServiceLinkedRoleFailure(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AttachLoadBalancersError {}
/// Errors returned by BatchDeleteScheduledAction
#[derive(Debug, PartialEq)]
pub enum BatchDeleteScheduledActionError {
    /// <p>You already have a pending update to an Amazon EC2 Auto Scaling resource (for example, an Auto Scaling group, instance, or load balancer).</p>
    ResourceContentionFault(String),
}

impl BatchDeleteScheduledActionError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<BatchDeleteScheduledActionError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "ResourceContention" => {
                        return RusotoError::Service(
                            BatchDeleteScheduledActionError::ResourceContentionFault(
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
impl fmt::Display for BatchDeleteScheduledActionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BatchDeleteScheduledActionError::ResourceContentionFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for BatchDeleteScheduledActionError {}
/// Errors returned by BatchPutScheduledUpdateGroupAction
#[derive(Debug, PartialEq)]
pub enum BatchPutScheduledUpdateGroupActionError {
    /// <p>You already have an Auto Scaling group or launch configuration with this name.</p>
    AlreadyExistsFault(String),
    /// <p>You have already reached a limit for your Amazon EC2 Auto Scaling resources (for example, Auto Scaling groups, launch configurations, or lifecycle hooks). For more information, see <a>DescribeAccountLimits</a>.</p>
    LimitExceededFault(String),
    /// <p>You already have a pending update to an Amazon EC2 Auto Scaling resource (for example, an Auto Scaling group, instance, or load balancer).</p>
    ResourceContentionFault(String),
}

impl BatchPutScheduledUpdateGroupActionError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<BatchPutScheduledUpdateGroupActionError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AlreadyExists" => {
                        return RusotoError::Service(
                            BatchPutScheduledUpdateGroupActionError::AlreadyExistsFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "LimitExceeded" => {
                        return RusotoError::Service(
                            BatchPutScheduledUpdateGroupActionError::LimitExceededFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "ResourceContention" => {
                        return RusotoError::Service(
                            BatchPutScheduledUpdateGroupActionError::ResourceContentionFault(
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
impl fmt::Display for BatchPutScheduledUpdateGroupActionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BatchPutScheduledUpdateGroupActionError::AlreadyExistsFault(ref cause) => {
                write!(f, "{}", cause)
            }
            BatchPutScheduledUpdateGroupActionError::LimitExceededFault(ref cause) => {
                write!(f, "{}", cause)
            }
            BatchPutScheduledUpdateGroupActionError::ResourceContentionFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for BatchPutScheduledUpdateGroupActionError {}
/// Errors returned by CompleteLifecycleAction
#[derive(Debug, PartialEq)]
pub enum CompleteLifecycleActionError {
    /// <p>You already have a pending update to an Amazon EC2 Auto Scaling resource (for example, an Auto Scaling group, instance, or load balancer).</p>
    ResourceContentionFault(String),
}

impl CompleteLifecycleActionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CompleteLifecycleActionError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "ResourceContention" => {
                        return RusotoError::Service(
                            CompleteLifecycleActionError::ResourceContentionFault(
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
impl fmt::Display for CompleteLifecycleActionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CompleteLifecycleActionError::ResourceContentionFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for CompleteLifecycleActionError {}
/// Errors returned by CreateAutoScalingGroup
#[derive(Debug, PartialEq)]
pub enum CreateAutoScalingGroupError {
    /// <p>You already have an Auto Scaling group or launch configuration with this name.</p>
    AlreadyExistsFault(String),
    /// <p>You have already reached a limit for your Amazon EC2 Auto Scaling resources (for example, Auto Scaling groups, launch configurations, or lifecycle hooks). For more information, see <a>DescribeAccountLimits</a>.</p>
    LimitExceededFault(String),
    /// <p>You already have a pending update to an Amazon EC2 Auto Scaling resource (for example, an Auto Scaling group, instance, or load balancer).</p>
    ResourceContentionFault(String),
    /// <p>The service-linked role is not yet ready for use.</p>
    ServiceLinkedRoleFailure(String),
}

impl CreateAutoScalingGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateAutoScalingGroupError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AlreadyExists" => {
                        return RusotoError::Service(
                            CreateAutoScalingGroupError::AlreadyExistsFault(parsed_error.message),
                        )
                    }
                    "LimitExceeded" => {
                        return RusotoError::Service(
                            CreateAutoScalingGroupError::LimitExceededFault(parsed_error.message),
                        )
                    }
                    "ResourceContention" => {
                        return RusotoError::Service(
                            CreateAutoScalingGroupError::ResourceContentionFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "ServiceLinkedRoleFailure" => {
                        return RusotoError::Service(
                            CreateAutoScalingGroupError::ServiceLinkedRoleFailure(
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
impl fmt::Display for CreateAutoScalingGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateAutoScalingGroupError::AlreadyExistsFault(ref cause) => write!(f, "{}", cause),
            CreateAutoScalingGroupError::LimitExceededFault(ref cause) => write!(f, "{}", cause),
            CreateAutoScalingGroupError::ResourceContentionFault(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateAutoScalingGroupError::ServiceLinkedRoleFailure(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for CreateAutoScalingGroupError {}
/// Errors returned by CreateLaunchConfiguration
#[derive(Debug, PartialEq)]
pub enum CreateLaunchConfigurationError {
    /// <p>You already have an Auto Scaling group or launch configuration with this name.</p>
    AlreadyExistsFault(String),
    /// <p>You have already reached a limit for your Amazon EC2 Auto Scaling resources (for example, Auto Scaling groups, launch configurations, or lifecycle hooks). For more information, see <a>DescribeAccountLimits</a>.</p>
    LimitExceededFault(String),
    /// <p>You already have a pending update to an Amazon EC2 Auto Scaling resource (for example, an Auto Scaling group, instance, or load balancer).</p>
    ResourceContentionFault(String),
}

impl CreateLaunchConfigurationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateLaunchConfigurationError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AlreadyExists" => {
                        return RusotoError::Service(
                            CreateLaunchConfigurationError::AlreadyExistsFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "LimitExceeded" => {
                        return RusotoError::Service(
                            CreateLaunchConfigurationError::LimitExceededFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "ResourceContention" => {
                        return RusotoError::Service(
                            CreateLaunchConfigurationError::ResourceContentionFault(
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
impl fmt::Display for CreateLaunchConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateLaunchConfigurationError::AlreadyExistsFault(ref cause) => write!(f, "{}", cause),
            CreateLaunchConfigurationError::LimitExceededFault(ref cause) => write!(f, "{}", cause),
            CreateLaunchConfigurationError::ResourceContentionFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for CreateLaunchConfigurationError {}
/// Errors returned by CreateOrUpdateTags
#[derive(Debug, PartialEq)]
pub enum CreateOrUpdateTagsError {
    /// <p>You already have an Auto Scaling group or launch configuration with this name.</p>
    AlreadyExistsFault(String),
    /// <p>You have already reached a limit for your Amazon EC2 Auto Scaling resources (for example, Auto Scaling groups, launch configurations, or lifecycle hooks). For more information, see <a>DescribeAccountLimits</a>.</p>
    LimitExceededFault(String),
    /// <p>You already have a pending update to an Amazon EC2 Auto Scaling resource (for example, an Auto Scaling group, instance, or load balancer).</p>
    ResourceContentionFault(String),
    /// <p>The operation can't be performed because the resource is in use.</p>
    ResourceInUseFault(String),
}

impl CreateOrUpdateTagsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateOrUpdateTagsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AlreadyExists" => {
                        return RusotoError::Service(CreateOrUpdateTagsError::AlreadyExistsFault(
                            parsed_error.message,
                        ))
                    }
                    "LimitExceeded" => {
                        return RusotoError::Service(CreateOrUpdateTagsError::LimitExceededFault(
                            parsed_error.message,
                        ))
                    }
                    "ResourceContention" => {
                        return RusotoError::Service(
                            CreateOrUpdateTagsError::ResourceContentionFault(parsed_error.message),
                        )
                    }
                    "ResourceInUse" => {
                        return RusotoError::Service(CreateOrUpdateTagsError::ResourceInUseFault(
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
impl fmt::Display for CreateOrUpdateTagsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateOrUpdateTagsError::AlreadyExistsFault(ref cause) => write!(f, "{}", cause),
            CreateOrUpdateTagsError::LimitExceededFault(ref cause) => write!(f, "{}", cause),
            CreateOrUpdateTagsError::ResourceContentionFault(ref cause) => write!(f, "{}", cause),
            CreateOrUpdateTagsError::ResourceInUseFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateOrUpdateTagsError {}
/// Errors returned by DeleteAutoScalingGroup
#[derive(Debug, PartialEq)]
pub enum DeleteAutoScalingGroupError {
    /// <p>You already have a pending update to an Amazon EC2 Auto Scaling resource (for example, an Auto Scaling group, instance, or load balancer).</p>
    ResourceContentionFault(String),
    /// <p>The operation can't be performed because the resource is in use.</p>
    ResourceInUseFault(String),
    /// <p>The operation can't be performed because there are scaling activities in progress.</p>
    ScalingActivityInProgressFault(String),
}

impl DeleteAutoScalingGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteAutoScalingGroupError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "ResourceContention" => {
                        return RusotoError::Service(
                            DeleteAutoScalingGroupError::ResourceContentionFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "ResourceInUse" => {
                        return RusotoError::Service(
                            DeleteAutoScalingGroupError::ResourceInUseFault(parsed_error.message),
                        )
                    }
                    "ScalingActivityInProgress" => {
                        return RusotoError::Service(
                            DeleteAutoScalingGroupError::ScalingActivityInProgressFault(
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
impl fmt::Display for DeleteAutoScalingGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteAutoScalingGroupError::ResourceContentionFault(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteAutoScalingGroupError::ResourceInUseFault(ref cause) => write!(f, "{}", cause),
            DeleteAutoScalingGroupError::ScalingActivityInProgressFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DeleteAutoScalingGroupError {}
/// Errors returned by DeleteLaunchConfiguration
#[derive(Debug, PartialEq)]
pub enum DeleteLaunchConfigurationError {
    /// <p>You already have a pending update to an Amazon EC2 Auto Scaling resource (for example, an Auto Scaling group, instance, or load balancer).</p>
    ResourceContentionFault(String),
    /// <p>The operation can't be performed because the resource is in use.</p>
    ResourceInUseFault(String),
}

impl DeleteLaunchConfigurationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteLaunchConfigurationError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "ResourceContention" => {
                        return RusotoError::Service(
                            DeleteLaunchConfigurationError::ResourceContentionFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "ResourceInUse" => {
                        return RusotoError::Service(
                            DeleteLaunchConfigurationError::ResourceInUseFault(
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
impl fmt::Display for DeleteLaunchConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteLaunchConfigurationError::ResourceContentionFault(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteLaunchConfigurationError::ResourceInUseFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteLaunchConfigurationError {}
/// Errors returned by DeleteLifecycleHook
#[derive(Debug, PartialEq)]
pub enum DeleteLifecycleHookError {
    /// <p>You already have a pending update to an Amazon EC2 Auto Scaling resource (for example, an Auto Scaling group, instance, or load balancer).</p>
    ResourceContentionFault(String),
}

impl DeleteLifecycleHookError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteLifecycleHookError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "ResourceContention" => {
                        return RusotoError::Service(
                            DeleteLifecycleHookError::ResourceContentionFault(parsed_error.message),
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
impl fmt::Display for DeleteLifecycleHookError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteLifecycleHookError::ResourceContentionFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteLifecycleHookError {}
/// Errors returned by DeleteNotificationConfiguration
#[derive(Debug, PartialEq)]
pub enum DeleteNotificationConfigurationError {
    /// <p>You already have a pending update to an Amazon EC2 Auto Scaling resource (for example, an Auto Scaling group, instance, or load balancer).</p>
    ResourceContentionFault(String),
}

impl DeleteNotificationConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteNotificationConfigurationError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "ResourceContention" => {
                        return RusotoError::Service(
                            DeleteNotificationConfigurationError::ResourceContentionFault(
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
impl fmt::Display for DeleteNotificationConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteNotificationConfigurationError::ResourceContentionFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DeleteNotificationConfigurationError {}
/// Errors returned by DeletePolicy
#[derive(Debug, PartialEq)]
pub enum DeletePolicyError {
    /// <p>You already have a pending update to an Amazon EC2 Auto Scaling resource (for example, an Auto Scaling group, instance, or load balancer).</p>
    ResourceContentionFault(String),
    /// <p>The service-linked role is not yet ready for use.</p>
    ServiceLinkedRoleFailure(String),
}

impl DeletePolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeletePolicyError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "ResourceContention" => {
                        return RusotoError::Service(DeletePolicyError::ResourceContentionFault(
                            parsed_error.message,
                        ))
                    }
                    "ServiceLinkedRoleFailure" => {
                        return RusotoError::Service(DeletePolicyError::ServiceLinkedRoleFailure(
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
impl fmt::Display for DeletePolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeletePolicyError::ResourceContentionFault(ref cause) => write!(f, "{}", cause),
            DeletePolicyError::ServiceLinkedRoleFailure(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeletePolicyError {}
/// Errors returned by DeleteScheduledAction
#[derive(Debug, PartialEq)]
pub enum DeleteScheduledActionError {
    /// <p>You already have a pending update to an Amazon EC2 Auto Scaling resource (for example, an Auto Scaling group, instance, or load balancer).</p>
    ResourceContentionFault(String),
}

impl DeleteScheduledActionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteScheduledActionError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "ResourceContention" => {
                        return RusotoError::Service(
                            DeleteScheduledActionError::ResourceContentionFault(
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
impl fmt::Display for DeleteScheduledActionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteScheduledActionError::ResourceContentionFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DeleteScheduledActionError {}
/// Errors returned by DeleteTags
#[derive(Debug, PartialEq)]
pub enum DeleteTagsError {
    /// <p>You already have a pending update to an Amazon EC2 Auto Scaling resource (for example, an Auto Scaling group, instance, or load balancer).</p>
    ResourceContentionFault(String),
    /// <p>The operation can't be performed because the resource is in use.</p>
    ResourceInUseFault(String),
}

impl DeleteTagsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteTagsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "ResourceContention" => {
                        return RusotoError::Service(DeleteTagsError::ResourceContentionFault(
                            parsed_error.message,
                        ))
                    }
                    "ResourceInUse" => {
                        return RusotoError::Service(DeleteTagsError::ResourceInUseFault(
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
impl fmt::Display for DeleteTagsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteTagsError::ResourceContentionFault(ref cause) => write!(f, "{}", cause),
            DeleteTagsError::ResourceInUseFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteTagsError {}
/// Errors returned by DescribeAccountLimits
#[derive(Debug, PartialEq)]
pub enum DescribeAccountLimitsError {
    /// <p>You already have a pending update to an Amazon EC2 Auto Scaling resource (for example, an Auto Scaling group, instance, or load balancer).</p>
    ResourceContentionFault(String),
}

impl DescribeAccountLimitsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeAccountLimitsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "ResourceContention" => {
                        return RusotoError::Service(
                            DescribeAccountLimitsError::ResourceContentionFault(
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
impl fmt::Display for DescribeAccountLimitsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeAccountLimitsError::ResourceContentionFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeAccountLimitsError {}
/// Errors returned by DescribeAdjustmentTypes
#[derive(Debug, PartialEq)]
pub enum DescribeAdjustmentTypesError {
    /// <p>You already have a pending update to an Amazon EC2 Auto Scaling resource (for example, an Auto Scaling group, instance, or load balancer).</p>
    ResourceContentionFault(String),
}

impl DescribeAdjustmentTypesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeAdjustmentTypesError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "ResourceContention" => {
                        return RusotoError::Service(
                            DescribeAdjustmentTypesError::ResourceContentionFault(
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
impl fmt::Display for DescribeAdjustmentTypesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeAdjustmentTypesError::ResourceContentionFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeAdjustmentTypesError {}
/// Errors returned by DescribeAutoScalingGroups
#[derive(Debug, PartialEq)]
pub enum DescribeAutoScalingGroupsError {
    /// <p>The <code>NextToken</code> value is not valid.</p>
    InvalidNextToken(String),
    /// <p>You already have a pending update to an Amazon EC2 Auto Scaling resource (for example, an Auto Scaling group, instance, or load balancer).</p>
    ResourceContentionFault(String),
}

impl DescribeAutoScalingGroupsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeAutoScalingGroupsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidNextToken" => {
                        return RusotoError::Service(
                            DescribeAutoScalingGroupsError::InvalidNextToken(parsed_error.message),
                        )
                    }
                    "ResourceContention" => {
                        return RusotoError::Service(
                            DescribeAutoScalingGroupsError::ResourceContentionFault(
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
impl fmt::Display for DescribeAutoScalingGroupsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeAutoScalingGroupsError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            DescribeAutoScalingGroupsError::ResourceContentionFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeAutoScalingGroupsError {}
/// Errors returned by DescribeAutoScalingInstances
#[derive(Debug, PartialEq)]
pub enum DescribeAutoScalingInstancesError {
    /// <p>The <code>NextToken</code> value is not valid.</p>
    InvalidNextToken(String),
    /// <p>You already have a pending update to an Amazon EC2 Auto Scaling resource (for example, an Auto Scaling group, instance, or load balancer).</p>
    ResourceContentionFault(String),
}

impl DescribeAutoScalingInstancesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeAutoScalingInstancesError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidNextToken" => {
                        return RusotoError::Service(
                            DescribeAutoScalingInstancesError::InvalidNextToken(
                                parsed_error.message,
                            ),
                        )
                    }
                    "ResourceContention" => {
                        return RusotoError::Service(
                            DescribeAutoScalingInstancesError::ResourceContentionFault(
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
impl fmt::Display for DescribeAutoScalingInstancesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeAutoScalingInstancesError::InvalidNextToken(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeAutoScalingInstancesError::ResourceContentionFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeAutoScalingInstancesError {}
/// Errors returned by DescribeAutoScalingNotificationTypes
#[derive(Debug, PartialEq)]
pub enum DescribeAutoScalingNotificationTypesError {
    /// <p>You already have a pending update to an Amazon EC2 Auto Scaling resource (for example, an Auto Scaling group, instance, or load balancer).</p>
    ResourceContentionFault(String),
}

impl DescribeAutoScalingNotificationTypesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeAutoScalingNotificationTypesError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "ResourceContention" => {
                        return RusotoError::Service(
                            DescribeAutoScalingNotificationTypesError::ResourceContentionFault(
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
impl fmt::Display for DescribeAutoScalingNotificationTypesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeAutoScalingNotificationTypesError::ResourceContentionFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeAutoScalingNotificationTypesError {}
/// Errors returned by DescribeLaunchConfigurations
#[derive(Debug, PartialEq)]
pub enum DescribeLaunchConfigurationsError {
    /// <p>The <code>NextToken</code> value is not valid.</p>
    InvalidNextToken(String),
    /// <p>You already have a pending update to an Amazon EC2 Auto Scaling resource (for example, an Auto Scaling group, instance, or load balancer).</p>
    ResourceContentionFault(String),
}

impl DescribeLaunchConfigurationsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeLaunchConfigurationsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidNextToken" => {
                        return RusotoError::Service(
                            DescribeLaunchConfigurationsError::InvalidNextToken(
                                parsed_error.message,
                            ),
                        )
                    }
                    "ResourceContention" => {
                        return RusotoError::Service(
                            DescribeLaunchConfigurationsError::ResourceContentionFault(
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
impl fmt::Display for DescribeLaunchConfigurationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeLaunchConfigurationsError::InvalidNextToken(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeLaunchConfigurationsError::ResourceContentionFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeLaunchConfigurationsError {}
/// Errors returned by DescribeLifecycleHookTypes
#[derive(Debug, PartialEq)]
pub enum DescribeLifecycleHookTypesError {
    /// <p>You already have a pending update to an Amazon EC2 Auto Scaling resource (for example, an Auto Scaling group, instance, or load balancer).</p>
    ResourceContentionFault(String),
}

impl DescribeLifecycleHookTypesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeLifecycleHookTypesError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "ResourceContention" => {
                        return RusotoError::Service(
                            DescribeLifecycleHookTypesError::ResourceContentionFault(
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
impl fmt::Display for DescribeLifecycleHookTypesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeLifecycleHookTypesError::ResourceContentionFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeLifecycleHookTypesError {}
/// Errors returned by DescribeLifecycleHooks
#[derive(Debug, PartialEq)]
pub enum DescribeLifecycleHooksError {
    /// <p>You already have a pending update to an Amazon EC2 Auto Scaling resource (for example, an Auto Scaling group, instance, or load balancer).</p>
    ResourceContentionFault(String),
}

impl DescribeLifecycleHooksError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeLifecycleHooksError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "ResourceContention" => {
                        return RusotoError::Service(
                            DescribeLifecycleHooksError::ResourceContentionFault(
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
impl fmt::Display for DescribeLifecycleHooksError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeLifecycleHooksError::ResourceContentionFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeLifecycleHooksError {}
/// Errors returned by DescribeLoadBalancerTargetGroups
#[derive(Debug, PartialEq)]
pub enum DescribeLoadBalancerTargetGroupsError {
    /// <p>You already have a pending update to an Amazon EC2 Auto Scaling resource (for example, an Auto Scaling group, instance, or load balancer).</p>
    ResourceContentionFault(String),
}

impl DescribeLoadBalancerTargetGroupsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeLoadBalancerTargetGroupsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "ResourceContention" => {
                        return RusotoError::Service(
                            DescribeLoadBalancerTargetGroupsError::ResourceContentionFault(
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
impl fmt::Display for DescribeLoadBalancerTargetGroupsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeLoadBalancerTargetGroupsError::ResourceContentionFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeLoadBalancerTargetGroupsError {}
/// Errors returned by DescribeLoadBalancers
#[derive(Debug, PartialEq)]
pub enum DescribeLoadBalancersError {
    /// <p>You already have a pending update to an Amazon EC2 Auto Scaling resource (for example, an Auto Scaling group, instance, or load balancer).</p>
    ResourceContentionFault(String),
}

impl DescribeLoadBalancersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeLoadBalancersError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "ResourceContention" => {
                        return RusotoError::Service(
                            DescribeLoadBalancersError::ResourceContentionFault(
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
impl fmt::Display for DescribeLoadBalancersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeLoadBalancersError::ResourceContentionFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeLoadBalancersError {}
/// Errors returned by DescribeMetricCollectionTypes
#[derive(Debug, PartialEq)]
pub enum DescribeMetricCollectionTypesError {
    /// <p>You already have a pending update to an Amazon EC2 Auto Scaling resource (for example, an Auto Scaling group, instance, or load balancer).</p>
    ResourceContentionFault(String),
}

impl DescribeMetricCollectionTypesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeMetricCollectionTypesError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "ResourceContention" => {
                        return RusotoError::Service(
                            DescribeMetricCollectionTypesError::ResourceContentionFault(
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
impl fmt::Display for DescribeMetricCollectionTypesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeMetricCollectionTypesError::ResourceContentionFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeMetricCollectionTypesError {}
/// Errors returned by DescribeNotificationConfigurations
#[derive(Debug, PartialEq)]
pub enum DescribeNotificationConfigurationsError {
    /// <p>The <code>NextToken</code> value is not valid.</p>
    InvalidNextToken(String),
    /// <p>You already have a pending update to an Amazon EC2 Auto Scaling resource (for example, an Auto Scaling group, instance, or load balancer).</p>
    ResourceContentionFault(String),
}

impl DescribeNotificationConfigurationsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeNotificationConfigurationsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidNextToken" => {
                        return RusotoError::Service(
                            DescribeNotificationConfigurationsError::InvalidNextToken(
                                parsed_error.message,
                            ),
                        )
                    }
                    "ResourceContention" => {
                        return RusotoError::Service(
                            DescribeNotificationConfigurationsError::ResourceContentionFault(
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
impl fmt::Display for DescribeNotificationConfigurationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeNotificationConfigurationsError::InvalidNextToken(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeNotificationConfigurationsError::ResourceContentionFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeNotificationConfigurationsError {}
/// Errors returned by DescribePolicies
#[derive(Debug, PartialEq)]
pub enum DescribePoliciesError {
    /// <p>The <code>NextToken</code> value is not valid.</p>
    InvalidNextToken(String),
    /// <p>You already have a pending update to an Amazon EC2 Auto Scaling resource (for example, an Auto Scaling group, instance, or load balancer).</p>
    ResourceContentionFault(String),
    /// <p>The service-linked role is not yet ready for use.</p>
    ServiceLinkedRoleFailure(String),
}

impl DescribePoliciesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribePoliciesError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidNextToken" => {
                        return RusotoError::Service(DescribePoliciesError::InvalidNextToken(
                            parsed_error.message,
                        ))
                    }
                    "ResourceContention" => {
                        return RusotoError::Service(
                            DescribePoliciesError::ResourceContentionFault(parsed_error.message),
                        )
                    }
                    "ServiceLinkedRoleFailure" => {
                        return RusotoError::Service(
                            DescribePoliciesError::ServiceLinkedRoleFailure(parsed_error.message),
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
impl fmt::Display for DescribePoliciesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribePoliciesError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            DescribePoliciesError::ResourceContentionFault(ref cause) => write!(f, "{}", cause),
            DescribePoliciesError::ServiceLinkedRoleFailure(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribePoliciesError {}
/// Errors returned by DescribeScalingActivities
#[derive(Debug, PartialEq)]
pub enum DescribeScalingActivitiesError {
    /// <p>The <code>NextToken</code> value is not valid.</p>
    InvalidNextToken(String),
    /// <p>You already have a pending update to an Amazon EC2 Auto Scaling resource (for example, an Auto Scaling group, instance, or load balancer).</p>
    ResourceContentionFault(String),
}

impl DescribeScalingActivitiesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeScalingActivitiesError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidNextToken" => {
                        return RusotoError::Service(
                            DescribeScalingActivitiesError::InvalidNextToken(parsed_error.message),
                        )
                    }
                    "ResourceContention" => {
                        return RusotoError::Service(
                            DescribeScalingActivitiesError::ResourceContentionFault(
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
impl fmt::Display for DescribeScalingActivitiesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeScalingActivitiesError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            DescribeScalingActivitiesError::ResourceContentionFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeScalingActivitiesError {}
/// Errors returned by DescribeScalingProcessTypes
#[derive(Debug, PartialEq)]
pub enum DescribeScalingProcessTypesError {
    /// <p>You already have a pending update to an Amazon EC2 Auto Scaling resource (for example, an Auto Scaling group, instance, or load balancer).</p>
    ResourceContentionFault(String),
}

impl DescribeScalingProcessTypesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeScalingProcessTypesError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "ResourceContention" => {
                        return RusotoError::Service(
                            DescribeScalingProcessTypesError::ResourceContentionFault(
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
impl fmt::Display for DescribeScalingProcessTypesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeScalingProcessTypesError::ResourceContentionFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeScalingProcessTypesError {}
/// Errors returned by DescribeScheduledActions
#[derive(Debug, PartialEq)]
pub enum DescribeScheduledActionsError {
    /// <p>The <code>NextToken</code> value is not valid.</p>
    InvalidNextToken(String),
    /// <p>You already have a pending update to an Amazon EC2 Auto Scaling resource (for example, an Auto Scaling group, instance, or load balancer).</p>
    ResourceContentionFault(String),
}

impl DescribeScheduledActionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeScheduledActionsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidNextToken" => {
                        return RusotoError::Service(
                            DescribeScheduledActionsError::InvalidNextToken(parsed_error.message),
                        )
                    }
                    "ResourceContention" => {
                        return RusotoError::Service(
                            DescribeScheduledActionsError::ResourceContentionFault(
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
impl fmt::Display for DescribeScheduledActionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeScheduledActionsError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            DescribeScheduledActionsError::ResourceContentionFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeScheduledActionsError {}
/// Errors returned by DescribeTags
#[derive(Debug, PartialEq)]
pub enum DescribeTagsError {
    /// <p>The <code>NextToken</code> value is not valid.</p>
    InvalidNextToken(String),
    /// <p>You already have a pending update to an Amazon EC2 Auto Scaling resource (for example, an Auto Scaling group, instance, or load balancer).</p>
    ResourceContentionFault(String),
}

impl DescribeTagsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeTagsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidNextToken" => {
                        return RusotoError::Service(DescribeTagsError::InvalidNextToken(
                            parsed_error.message,
                        ))
                    }
                    "ResourceContention" => {
                        return RusotoError::Service(DescribeTagsError::ResourceContentionFault(
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
impl fmt::Display for DescribeTagsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeTagsError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            DescribeTagsError::ResourceContentionFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeTagsError {}
/// Errors returned by DescribeTerminationPolicyTypes
#[derive(Debug, PartialEq)]
pub enum DescribeTerminationPolicyTypesError {
    /// <p>You already have a pending update to an Amazon EC2 Auto Scaling resource (for example, an Auto Scaling group, instance, or load balancer).</p>
    ResourceContentionFault(String),
}

impl DescribeTerminationPolicyTypesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeTerminationPolicyTypesError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "ResourceContention" => {
                        return RusotoError::Service(
                            DescribeTerminationPolicyTypesError::ResourceContentionFault(
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
impl fmt::Display for DescribeTerminationPolicyTypesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeTerminationPolicyTypesError::ResourceContentionFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeTerminationPolicyTypesError {}
/// Errors returned by DetachInstances
#[derive(Debug, PartialEq)]
pub enum DetachInstancesError {
    /// <p>You already have a pending update to an Amazon EC2 Auto Scaling resource (for example, an Auto Scaling group, instance, or load balancer).</p>
    ResourceContentionFault(String),
}

impl DetachInstancesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DetachInstancesError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "ResourceContention" => {
                        return RusotoError::Service(DetachInstancesError::ResourceContentionFault(
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
impl fmt::Display for DetachInstancesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DetachInstancesError::ResourceContentionFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DetachInstancesError {}
/// Errors returned by DetachLoadBalancerTargetGroups
#[derive(Debug, PartialEq)]
pub enum DetachLoadBalancerTargetGroupsError {
    /// <p>You already have a pending update to an Amazon EC2 Auto Scaling resource (for example, an Auto Scaling group, instance, or load balancer).</p>
    ResourceContentionFault(String),
}

impl DetachLoadBalancerTargetGroupsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DetachLoadBalancerTargetGroupsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "ResourceContention" => {
                        return RusotoError::Service(
                            DetachLoadBalancerTargetGroupsError::ResourceContentionFault(
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
impl fmt::Display for DetachLoadBalancerTargetGroupsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DetachLoadBalancerTargetGroupsError::ResourceContentionFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DetachLoadBalancerTargetGroupsError {}
/// Errors returned by DetachLoadBalancers
#[derive(Debug, PartialEq)]
pub enum DetachLoadBalancersError {
    /// <p>You already have a pending update to an Amazon EC2 Auto Scaling resource (for example, an Auto Scaling group, instance, or load balancer).</p>
    ResourceContentionFault(String),
}

impl DetachLoadBalancersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DetachLoadBalancersError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "ResourceContention" => {
                        return RusotoError::Service(
                            DetachLoadBalancersError::ResourceContentionFault(parsed_error.message),
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
impl fmt::Display for DetachLoadBalancersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DetachLoadBalancersError::ResourceContentionFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DetachLoadBalancersError {}
/// Errors returned by DisableMetricsCollection
#[derive(Debug, PartialEq)]
pub enum DisableMetricsCollectionError {
    /// <p>You already have a pending update to an Amazon EC2 Auto Scaling resource (for example, an Auto Scaling group, instance, or load balancer).</p>
    ResourceContentionFault(String),
}

impl DisableMetricsCollectionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DisableMetricsCollectionError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "ResourceContention" => {
                        return RusotoError::Service(
                            DisableMetricsCollectionError::ResourceContentionFault(
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
impl fmt::Display for DisableMetricsCollectionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisableMetricsCollectionError::ResourceContentionFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DisableMetricsCollectionError {}
/// Errors returned by EnableMetricsCollection
#[derive(Debug, PartialEq)]
pub enum EnableMetricsCollectionError {
    /// <p>You already have a pending update to an Amazon EC2 Auto Scaling resource (for example, an Auto Scaling group, instance, or load balancer).</p>
    ResourceContentionFault(String),
}

impl EnableMetricsCollectionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<EnableMetricsCollectionError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "ResourceContention" => {
                        return RusotoError::Service(
                            EnableMetricsCollectionError::ResourceContentionFault(
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
impl fmt::Display for EnableMetricsCollectionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EnableMetricsCollectionError::ResourceContentionFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for EnableMetricsCollectionError {}
/// Errors returned by EnterStandby
#[derive(Debug, PartialEq)]
pub enum EnterStandbyError {
    /// <p>You already have a pending update to an Amazon EC2 Auto Scaling resource (for example, an Auto Scaling group, instance, or load balancer).</p>
    ResourceContentionFault(String),
}

impl EnterStandbyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<EnterStandbyError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "ResourceContention" => {
                        return RusotoError::Service(EnterStandbyError::ResourceContentionFault(
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
impl fmt::Display for EnterStandbyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EnterStandbyError::ResourceContentionFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for EnterStandbyError {}
/// Errors returned by ExecutePolicy
#[derive(Debug, PartialEq)]
pub enum ExecutePolicyError {
    /// <p>You already have a pending update to an Amazon EC2 Auto Scaling resource (for example, an Auto Scaling group, instance, or load balancer).</p>
    ResourceContentionFault(String),
    /// <p>The operation can't be performed because there are scaling activities in progress.</p>
    ScalingActivityInProgressFault(String),
}

impl ExecutePolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ExecutePolicyError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "ResourceContention" => {
                        return RusotoError::Service(ExecutePolicyError::ResourceContentionFault(
                            parsed_error.message,
                        ))
                    }
                    "ScalingActivityInProgress" => {
                        return RusotoError::Service(
                            ExecutePolicyError::ScalingActivityInProgressFault(
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
impl fmt::Display for ExecutePolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ExecutePolicyError::ResourceContentionFault(ref cause) => write!(f, "{}", cause),
            ExecutePolicyError::ScalingActivityInProgressFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ExecutePolicyError {}
/// Errors returned by ExitStandby
#[derive(Debug, PartialEq)]
pub enum ExitStandbyError {
    /// <p>You already have a pending update to an Amazon EC2 Auto Scaling resource (for example, an Auto Scaling group, instance, or load balancer).</p>
    ResourceContentionFault(String),
}

impl ExitStandbyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ExitStandbyError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "ResourceContention" => {
                        return RusotoError::Service(ExitStandbyError::ResourceContentionFault(
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
impl fmt::Display for ExitStandbyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ExitStandbyError::ResourceContentionFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ExitStandbyError {}
/// Errors returned by PutLifecycleHook
#[derive(Debug, PartialEq)]
pub enum PutLifecycleHookError {
    /// <p>You have already reached a limit for your Amazon EC2 Auto Scaling resources (for example, Auto Scaling groups, launch configurations, or lifecycle hooks). For more information, see <a>DescribeAccountLimits</a>.</p>
    LimitExceededFault(String),
    /// <p>You already have a pending update to an Amazon EC2 Auto Scaling resource (for example, an Auto Scaling group, instance, or load balancer).</p>
    ResourceContentionFault(String),
}

impl PutLifecycleHookError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutLifecycleHookError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "LimitExceeded" => {
                        return RusotoError::Service(PutLifecycleHookError::LimitExceededFault(
                            parsed_error.message,
                        ))
                    }
                    "ResourceContention" => {
                        return RusotoError::Service(
                            PutLifecycleHookError::ResourceContentionFault(parsed_error.message),
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
impl fmt::Display for PutLifecycleHookError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutLifecycleHookError::LimitExceededFault(ref cause) => write!(f, "{}", cause),
            PutLifecycleHookError::ResourceContentionFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutLifecycleHookError {}
/// Errors returned by PutNotificationConfiguration
#[derive(Debug, PartialEq)]
pub enum PutNotificationConfigurationError {
    /// <p>You have already reached a limit for your Amazon EC2 Auto Scaling resources (for example, Auto Scaling groups, launch configurations, or lifecycle hooks). For more information, see <a>DescribeAccountLimits</a>.</p>
    LimitExceededFault(String),
    /// <p>You already have a pending update to an Amazon EC2 Auto Scaling resource (for example, an Auto Scaling group, instance, or load balancer).</p>
    ResourceContentionFault(String),
    /// <p>The service-linked role is not yet ready for use.</p>
    ServiceLinkedRoleFailure(String),
}

impl PutNotificationConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<PutNotificationConfigurationError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "LimitExceeded" => {
                        return RusotoError::Service(
                            PutNotificationConfigurationError::LimitExceededFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "ResourceContention" => {
                        return RusotoError::Service(
                            PutNotificationConfigurationError::ResourceContentionFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "ServiceLinkedRoleFailure" => {
                        return RusotoError::Service(
                            PutNotificationConfigurationError::ServiceLinkedRoleFailure(
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
impl fmt::Display for PutNotificationConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutNotificationConfigurationError::LimitExceededFault(ref cause) => {
                write!(f, "{}", cause)
            }
            PutNotificationConfigurationError::ResourceContentionFault(ref cause) => {
                write!(f, "{}", cause)
            }
            PutNotificationConfigurationError::ServiceLinkedRoleFailure(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for PutNotificationConfigurationError {}
/// Errors returned by PutScalingPolicy
#[derive(Debug, PartialEq)]
pub enum PutScalingPolicyError {
    /// <p>You have already reached a limit for your Amazon EC2 Auto Scaling resources (for example, Auto Scaling groups, launch configurations, or lifecycle hooks). For more information, see <a>DescribeAccountLimits</a>.</p>
    LimitExceededFault(String),
    /// <p>You already have a pending update to an Amazon EC2 Auto Scaling resource (for example, an Auto Scaling group, instance, or load balancer).</p>
    ResourceContentionFault(String),
    /// <p>The service-linked role is not yet ready for use.</p>
    ServiceLinkedRoleFailure(String),
}

impl PutScalingPolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutScalingPolicyError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "LimitExceeded" => {
                        return RusotoError::Service(PutScalingPolicyError::LimitExceededFault(
                            parsed_error.message,
                        ))
                    }
                    "ResourceContention" => {
                        return RusotoError::Service(
                            PutScalingPolicyError::ResourceContentionFault(parsed_error.message),
                        )
                    }
                    "ServiceLinkedRoleFailure" => {
                        return RusotoError::Service(
                            PutScalingPolicyError::ServiceLinkedRoleFailure(parsed_error.message),
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
impl fmt::Display for PutScalingPolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutScalingPolicyError::LimitExceededFault(ref cause) => write!(f, "{}", cause),
            PutScalingPolicyError::ResourceContentionFault(ref cause) => write!(f, "{}", cause),
            PutScalingPolicyError::ServiceLinkedRoleFailure(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutScalingPolicyError {}
/// Errors returned by PutScheduledUpdateGroupAction
#[derive(Debug, PartialEq)]
pub enum PutScheduledUpdateGroupActionError {
    /// <p>You already have an Auto Scaling group or launch configuration with this name.</p>
    AlreadyExistsFault(String),
    /// <p>You have already reached a limit for your Amazon EC2 Auto Scaling resources (for example, Auto Scaling groups, launch configurations, or lifecycle hooks). For more information, see <a>DescribeAccountLimits</a>.</p>
    LimitExceededFault(String),
    /// <p>You already have a pending update to an Amazon EC2 Auto Scaling resource (for example, an Auto Scaling group, instance, or load balancer).</p>
    ResourceContentionFault(String),
}

impl PutScheduledUpdateGroupActionError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<PutScheduledUpdateGroupActionError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AlreadyExists" => {
                        return RusotoError::Service(
                            PutScheduledUpdateGroupActionError::AlreadyExistsFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "LimitExceeded" => {
                        return RusotoError::Service(
                            PutScheduledUpdateGroupActionError::LimitExceededFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "ResourceContention" => {
                        return RusotoError::Service(
                            PutScheduledUpdateGroupActionError::ResourceContentionFault(
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
impl fmt::Display for PutScheduledUpdateGroupActionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutScheduledUpdateGroupActionError::AlreadyExistsFault(ref cause) => {
                write!(f, "{}", cause)
            }
            PutScheduledUpdateGroupActionError::LimitExceededFault(ref cause) => {
                write!(f, "{}", cause)
            }
            PutScheduledUpdateGroupActionError::ResourceContentionFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for PutScheduledUpdateGroupActionError {}
/// Errors returned by RecordLifecycleActionHeartbeat
#[derive(Debug, PartialEq)]
pub enum RecordLifecycleActionHeartbeatError {
    /// <p>You already have a pending update to an Amazon EC2 Auto Scaling resource (for example, an Auto Scaling group, instance, or load balancer).</p>
    ResourceContentionFault(String),
}

impl RecordLifecycleActionHeartbeatError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<RecordLifecycleActionHeartbeatError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "ResourceContention" => {
                        return RusotoError::Service(
                            RecordLifecycleActionHeartbeatError::ResourceContentionFault(
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
impl fmt::Display for RecordLifecycleActionHeartbeatError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RecordLifecycleActionHeartbeatError::ResourceContentionFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for RecordLifecycleActionHeartbeatError {}
/// Errors returned by ResumeProcesses
#[derive(Debug, PartialEq)]
pub enum ResumeProcessesError {
    /// <p>You already have a pending update to an Amazon EC2 Auto Scaling resource (for example, an Auto Scaling group, instance, or load balancer).</p>
    ResourceContentionFault(String),
    /// <p>The operation can't be performed because the resource is in use.</p>
    ResourceInUseFault(String),
}

impl ResumeProcessesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ResumeProcessesError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "ResourceContention" => {
                        return RusotoError::Service(ResumeProcessesError::ResourceContentionFault(
                            parsed_error.message,
                        ))
                    }
                    "ResourceInUse" => {
                        return RusotoError::Service(ResumeProcessesError::ResourceInUseFault(
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
impl fmt::Display for ResumeProcessesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ResumeProcessesError::ResourceContentionFault(ref cause) => write!(f, "{}", cause),
            ResumeProcessesError::ResourceInUseFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ResumeProcessesError {}
/// Errors returned by SetDesiredCapacity
#[derive(Debug, PartialEq)]
pub enum SetDesiredCapacityError {
    /// <p>You already have a pending update to an Amazon EC2 Auto Scaling resource (for example, an Auto Scaling group, instance, or load balancer).</p>
    ResourceContentionFault(String),
    /// <p>The operation can't be performed because there are scaling activities in progress.</p>
    ScalingActivityInProgressFault(String),
}

impl SetDesiredCapacityError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SetDesiredCapacityError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "ResourceContention" => {
                        return RusotoError::Service(
                            SetDesiredCapacityError::ResourceContentionFault(parsed_error.message),
                        )
                    }
                    "ScalingActivityInProgress" => {
                        return RusotoError::Service(
                            SetDesiredCapacityError::ScalingActivityInProgressFault(
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
impl fmt::Display for SetDesiredCapacityError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SetDesiredCapacityError::ResourceContentionFault(ref cause) => write!(f, "{}", cause),
            SetDesiredCapacityError::ScalingActivityInProgressFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for SetDesiredCapacityError {}
/// Errors returned by SetInstanceHealth
#[derive(Debug, PartialEq)]
pub enum SetInstanceHealthError {
    /// <p>You already have a pending update to an Amazon EC2 Auto Scaling resource (for example, an Auto Scaling group, instance, or load balancer).</p>
    ResourceContentionFault(String),
}

impl SetInstanceHealthError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SetInstanceHealthError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "ResourceContention" => {
                        return RusotoError::Service(
                            SetInstanceHealthError::ResourceContentionFault(parsed_error.message),
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
impl fmt::Display for SetInstanceHealthError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SetInstanceHealthError::ResourceContentionFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for SetInstanceHealthError {}
/// Errors returned by SetInstanceProtection
#[derive(Debug, PartialEq)]
pub enum SetInstanceProtectionError {
    /// <p>You have already reached a limit for your Amazon EC2 Auto Scaling resources (for example, Auto Scaling groups, launch configurations, or lifecycle hooks). For more information, see <a>DescribeAccountLimits</a>.</p>
    LimitExceededFault(String),
    /// <p>You already have a pending update to an Amazon EC2 Auto Scaling resource (for example, an Auto Scaling group, instance, or load balancer).</p>
    ResourceContentionFault(String),
}

impl SetInstanceProtectionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SetInstanceProtectionError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "LimitExceeded" => {
                        return RusotoError::Service(
                            SetInstanceProtectionError::LimitExceededFault(parsed_error.message),
                        )
                    }
                    "ResourceContention" => {
                        return RusotoError::Service(
                            SetInstanceProtectionError::ResourceContentionFault(
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
impl fmt::Display for SetInstanceProtectionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SetInstanceProtectionError::LimitExceededFault(ref cause) => write!(f, "{}", cause),
            SetInstanceProtectionError::ResourceContentionFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for SetInstanceProtectionError {}
/// Errors returned by SuspendProcesses
#[derive(Debug, PartialEq)]
pub enum SuspendProcessesError {
    /// <p>You already have a pending update to an Amazon EC2 Auto Scaling resource (for example, an Auto Scaling group, instance, or load balancer).</p>
    ResourceContentionFault(String),
    /// <p>The operation can't be performed because the resource is in use.</p>
    ResourceInUseFault(String),
}

impl SuspendProcessesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SuspendProcessesError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "ResourceContention" => {
                        return RusotoError::Service(
                            SuspendProcessesError::ResourceContentionFault(parsed_error.message),
                        )
                    }
                    "ResourceInUse" => {
                        return RusotoError::Service(SuspendProcessesError::ResourceInUseFault(
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
impl fmt::Display for SuspendProcessesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SuspendProcessesError::ResourceContentionFault(ref cause) => write!(f, "{}", cause),
            SuspendProcessesError::ResourceInUseFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for SuspendProcessesError {}
/// Errors returned by TerminateInstanceInAutoScalingGroup
#[derive(Debug, PartialEq)]
pub enum TerminateInstanceInAutoScalingGroupError {
    /// <p>You already have a pending update to an Amazon EC2 Auto Scaling resource (for example, an Auto Scaling group, instance, or load balancer).</p>
    ResourceContentionFault(String),
    /// <p>The operation can't be performed because there are scaling activities in progress.</p>
    ScalingActivityInProgressFault(String),
}

impl TerminateInstanceInAutoScalingGroupError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<TerminateInstanceInAutoScalingGroupError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "ResourceContention" => {
                        return RusotoError::Service(
                            TerminateInstanceInAutoScalingGroupError::ResourceContentionFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "ScalingActivityInProgress" => return RusotoError::Service(
                        TerminateInstanceInAutoScalingGroupError::ScalingActivityInProgressFault(
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
impl fmt::Display for TerminateInstanceInAutoScalingGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TerminateInstanceInAutoScalingGroupError::ResourceContentionFault(ref cause) => {
                write!(f, "{}", cause)
            }
            TerminateInstanceInAutoScalingGroupError::ScalingActivityInProgressFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for TerminateInstanceInAutoScalingGroupError {}
/// Errors returned by UpdateAutoScalingGroup
#[derive(Debug, PartialEq)]
pub enum UpdateAutoScalingGroupError {
    /// <p>You already have a pending update to an Amazon EC2 Auto Scaling resource (for example, an Auto Scaling group, instance, or load balancer).</p>
    ResourceContentionFault(String),
    /// <p>The operation can't be performed because there are scaling activities in progress.</p>
    ScalingActivityInProgressFault(String),
    /// <p>The service-linked role is not yet ready for use.</p>
    ServiceLinkedRoleFailure(String),
}

impl UpdateAutoScalingGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateAutoScalingGroupError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "ResourceContention" => {
                        return RusotoError::Service(
                            UpdateAutoScalingGroupError::ResourceContentionFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "ScalingActivityInProgress" => {
                        return RusotoError::Service(
                            UpdateAutoScalingGroupError::ScalingActivityInProgressFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "ServiceLinkedRoleFailure" => {
                        return RusotoError::Service(
                            UpdateAutoScalingGroupError::ServiceLinkedRoleFailure(
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
impl fmt::Display for UpdateAutoScalingGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateAutoScalingGroupError::ResourceContentionFault(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateAutoScalingGroupError::ScalingActivityInProgressFault(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateAutoScalingGroupError::ServiceLinkedRoleFailure(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for UpdateAutoScalingGroupError {}
/// Trait representing the capabilities of the Auto Scaling API. Auto Scaling clients implement this trait.
#[async_trait]
pub trait Autoscaling {
    /// <p>Attaches one or more EC2 instances to the specified Auto Scaling group.</p> <p>When you attach instances, Amazon EC2 Auto Scaling increases the desired capacity of the group by the number of instances being attached. If the number of instances being attached plus the desired capacity of the group exceeds the maximum size of the group, the operation fails.</p> <p>If there is a Classic Load Balancer attached to your Auto Scaling group, the instances are also registered with the load balancer. If there are target groups attached to your Auto Scaling group, the instances are also registered with the target groups.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/attach-instance-asg.html">Attach EC2 Instances to Your Auto Scaling Group</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    async fn attach_instances(
        &self,
        input: AttachInstancesQuery,
    ) -> Result<(), RusotoError<AttachInstancesError>>;

    /// <p>Attaches one or more target groups to the specified Auto Scaling group.</p> <p>To describe the target groups for an Auto Scaling group, use <a>DescribeLoadBalancerTargetGroups</a>. To detach the target group from the Auto Scaling group, use <a>DetachLoadBalancerTargetGroups</a>.</p> <p>With Application Load Balancers and Network Load Balancers, instances are registered as targets with a target group. With Classic Load Balancers, instances are registered with the load balancer. For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/attach-load-balancer-asg.html">Attaching a Load Balancer to Your Auto Scaling Group</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    async fn attach_load_balancer_target_groups(
        &self,
        input: AttachLoadBalancerTargetGroupsType,
    ) -> Result<
        AttachLoadBalancerTargetGroupsResultType,
        RusotoError<AttachLoadBalancerTargetGroupsError>,
    >;

    /// <p>Attaches one or more Classic Load Balancers to the specified Auto Scaling group.</p> <p>To attach an Application Load Balancer or a Network Load Balancer instead, see <a>AttachLoadBalancerTargetGroups</a>.</p> <p>To describe the load balancers for an Auto Scaling group, use <a>DescribeLoadBalancers</a>. To detach the load balancer from the Auto Scaling group, use <a>DetachLoadBalancers</a>.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/attach-load-balancer-asg.html">Attaching a Load Balancer to Your Auto Scaling Group</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    async fn attach_load_balancers(
        &self,
        input: AttachLoadBalancersType,
    ) -> Result<AttachLoadBalancersResultType, RusotoError<AttachLoadBalancersError>>;

    /// <p>Deletes one or more scheduled actions for the specified Auto Scaling group.</p>
    async fn batch_delete_scheduled_action(
        &self,
        input: BatchDeleteScheduledActionType,
    ) -> Result<BatchDeleteScheduledActionAnswer, RusotoError<BatchDeleteScheduledActionError>>;

    /// <p>Creates or updates one or more scheduled scaling actions for an Auto Scaling group. If you leave a parameter unspecified when updating a scheduled scaling action, the corresponding value remains unchanged.</p>
    async fn batch_put_scheduled_update_group_action(
        &self,
        input: BatchPutScheduledUpdateGroupActionType,
    ) -> Result<
        BatchPutScheduledUpdateGroupActionAnswer,
        RusotoError<BatchPutScheduledUpdateGroupActionError>,
    >;

    /// <p>Completes the lifecycle action for the specified token or instance with the specified result.</p> <p>This step is a part of the procedure for adding a lifecycle hook to an Auto Scaling group:</p> <ol> <li> <p>(Optional) Create a Lambda function and a rule that allows CloudWatch Events to invoke your Lambda function when Amazon EC2 Auto Scaling launches or terminates instances.</p> </li> <li> <p>(Optional) Create a notification target and an IAM role. The target can be either an Amazon SQS queue or an Amazon SNS topic. The role allows Amazon EC2 Auto Scaling to publish lifecycle notifications to the target.</p> </li> <li> <p>Create the lifecycle hook. Specify whether the hook is used when the instances launch or terminate.</p> </li> <li> <p>If you need more time, record the lifecycle action heartbeat to keep the instance in a pending state.</p> </li> <li> <p> <b>If you finish before the timeout period ends, complete the lifecycle action.</b> </p> </li> </ol> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/lifecycle-hooks.html">Amazon EC2 Auto Scaling Lifecycle Hooks</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    async fn complete_lifecycle_action(
        &self,
        input: CompleteLifecycleActionType,
    ) -> Result<CompleteLifecycleActionAnswer, RusotoError<CompleteLifecycleActionError>>;

    /// <p>Creates an Auto Scaling group with the specified name and attributes.</p> <p>If you exceed your maximum limit of Auto Scaling groups, the call fails. For information about viewing this limit, see <a>DescribeAccountLimits</a>. For information about updating this limit, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/as-account-limits.html">Amazon EC2 Auto Scaling Limits</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    async fn create_auto_scaling_group(
        &self,
        input: CreateAutoScalingGroupType,
    ) -> Result<(), RusotoError<CreateAutoScalingGroupError>>;

    /// <p>Creates a launch configuration.</p> <p>If you exceed your maximum limit of launch configurations, the call fails. For information about viewing this limit, see <a>DescribeAccountLimits</a>. For information about updating this limit, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/as-account-limits.html">Amazon EC2 Auto Scaling Limits</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/LaunchConfiguration.html">Launch Configurations</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    async fn create_launch_configuration(
        &self,
        input: CreateLaunchConfigurationType,
    ) -> Result<(), RusotoError<CreateLaunchConfigurationError>>;

    /// <p>Creates or updates tags for the specified Auto Scaling group.</p> <p>When you specify a tag with a key that already exists, the operation overwrites the previous tag definition, and you do not get an error message.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/autoscaling-tagging.html">Tagging Auto Scaling Groups and Instances</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    async fn create_or_update_tags(
        &self,
        input: CreateOrUpdateTagsType,
    ) -> Result<(), RusotoError<CreateOrUpdateTagsError>>;

    /// <p>Deletes the specified Auto Scaling group.</p> <p>If the group has instances or scaling activities in progress, you must specify the option to force the deletion in order for it to succeed.</p> <p>If the group has policies, deleting the group deletes the policies, the underlying alarm actions, and any alarm that no longer has an associated action.</p> <p>To remove instances from the Auto Scaling group before deleting it, call <a>DetachInstances</a> with the list of instances and the option to decrement the desired capacity. This ensures that Amazon EC2 Auto Scaling does not launch replacement instances.</p> <p>To terminate all instances before deleting the Auto Scaling group, call <a>UpdateAutoScalingGroup</a> and set the minimum size and desired capacity of the Auto Scaling group to zero.</p>
    async fn delete_auto_scaling_group(
        &self,
        input: DeleteAutoScalingGroupType,
    ) -> Result<(), RusotoError<DeleteAutoScalingGroupError>>;

    /// <p>Deletes the specified launch configuration.</p> <p>The launch configuration must not be attached to an Auto Scaling group. When this call completes, the launch configuration is no longer available for use.</p>
    async fn delete_launch_configuration(
        &self,
        input: LaunchConfigurationNameType,
    ) -> Result<(), RusotoError<DeleteLaunchConfigurationError>>;

    /// <p>Deletes the specified lifecycle hook.</p> <p>If there are any outstanding lifecycle actions, they are completed first (<code>ABANDON</code> for launching instances, <code>CONTINUE</code> for terminating instances).</p>
    async fn delete_lifecycle_hook(
        &self,
        input: DeleteLifecycleHookType,
    ) -> Result<DeleteLifecycleHookAnswer, RusotoError<DeleteLifecycleHookError>>;

    /// <p>Deletes the specified notification.</p>
    async fn delete_notification_configuration(
        &self,
        input: DeleteNotificationConfigurationType,
    ) -> Result<(), RusotoError<DeleteNotificationConfigurationError>>;

    /// <p>Deletes the specified scaling policy.</p> <p>Deleting either a step scaling policy or a simple scaling policy deletes the underlying alarm action, but does not delete the alarm, even if it no longer has an associated action.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/deleting-scaling-policy.html">Deleting a Scaling Policy</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    async fn delete_policy(
        &self,
        input: DeletePolicyType,
    ) -> Result<(), RusotoError<DeletePolicyError>>;

    /// <p>Deletes the specified scheduled action.</p>
    async fn delete_scheduled_action(
        &self,
        input: DeleteScheduledActionType,
    ) -> Result<(), RusotoError<DeleteScheduledActionError>>;

    /// <p>Deletes the specified tags.</p>
    async fn delete_tags(&self, input: DeleteTagsType) -> Result<(), RusotoError<DeleteTagsError>>;

    /// <p>Describes the current Amazon EC2 Auto Scaling resource limits for your AWS account.</p> <p>For information about requesting an increase in these limits, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/as-account-limits.html">Amazon EC2 Auto Scaling Limits</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    async fn describe_account_limits(
        &self,
    ) -> Result<DescribeAccountLimitsAnswer, RusotoError<DescribeAccountLimitsError>>;

    /// <p>Describes the policy adjustment types for use with <a>PutScalingPolicy</a>.</p>
    async fn describe_adjustment_types(
        &self,
    ) -> Result<DescribeAdjustmentTypesAnswer, RusotoError<DescribeAdjustmentTypesError>>;

    /// <p>Describes one or more Auto Scaling groups.</p>
    async fn describe_auto_scaling_groups(
        &self,
        input: AutoScalingGroupNamesType,
    ) -> Result<AutoScalingGroupsType, RusotoError<DescribeAutoScalingGroupsError>>;

    /// <p>Describes one or more Auto Scaling instances.</p>
    async fn describe_auto_scaling_instances(
        &self,
        input: DescribeAutoScalingInstancesType,
    ) -> Result<AutoScalingInstancesType, RusotoError<DescribeAutoScalingInstancesError>>;

    /// <p>Describes the notification types that are supported by Amazon EC2 Auto Scaling.</p>
    async fn describe_auto_scaling_notification_types(
        &self,
    ) -> Result<
        DescribeAutoScalingNotificationTypesAnswer,
        RusotoError<DescribeAutoScalingNotificationTypesError>,
    >;

    /// <p>Describes one or more launch configurations.</p>
    async fn describe_launch_configurations(
        &self,
        input: LaunchConfigurationNamesType,
    ) -> Result<LaunchConfigurationsType, RusotoError<DescribeLaunchConfigurationsError>>;

    /// <p><p>Describes the available types of lifecycle hooks.</p> <p>The following hook types are supported:</p> <ul> <li> <p>autoscaling:EC2<em>INSTANCE</em>LAUNCHING</p> </li> <li> <p>autoscaling:EC2<em>INSTANCE</em>TERMINATING</p> </li> </ul></p>
    async fn describe_lifecycle_hook_types(
        &self,
    ) -> Result<DescribeLifecycleHookTypesAnswer, RusotoError<DescribeLifecycleHookTypesError>>;

    /// <p>Describes the lifecycle hooks for the specified Auto Scaling group.</p>
    async fn describe_lifecycle_hooks(
        &self,
        input: DescribeLifecycleHooksType,
    ) -> Result<DescribeLifecycleHooksAnswer, RusotoError<DescribeLifecycleHooksError>>;

    /// <p>Describes the target groups for the specified Auto Scaling group.</p>
    async fn describe_load_balancer_target_groups(
        &self,
        input: DescribeLoadBalancerTargetGroupsRequest,
    ) -> Result<
        DescribeLoadBalancerTargetGroupsResponse,
        RusotoError<DescribeLoadBalancerTargetGroupsError>,
    >;

    /// <p>Describes the load balancers for the specified Auto Scaling group.</p> <p>This operation describes only Classic Load Balancers. If you have Application Load Balancers or Network Load Balancers, use <a>DescribeLoadBalancerTargetGroups</a> instead.</p>
    async fn describe_load_balancers(
        &self,
        input: DescribeLoadBalancersRequest,
    ) -> Result<DescribeLoadBalancersResponse, RusotoError<DescribeLoadBalancersError>>;

    /// <p>Describes the available CloudWatch metrics for Amazon EC2 Auto Scaling.</p> <p>The <code>GroupStandbyInstances</code> metric is not returned by default. You must explicitly request this metric when calling <a>EnableMetricsCollection</a>.</p>
    async fn describe_metric_collection_types(
        &self,
    ) -> Result<DescribeMetricCollectionTypesAnswer, RusotoError<DescribeMetricCollectionTypesError>>;

    /// <p>Describes the notification actions associated with the specified Auto Scaling group.</p>
    async fn describe_notification_configurations(
        &self,
        input: DescribeNotificationConfigurationsType,
    ) -> Result<
        DescribeNotificationConfigurationsAnswer,
        RusotoError<DescribeNotificationConfigurationsError>,
    >;

    /// <p>Describes the policies for the specified Auto Scaling group.</p>
    async fn describe_policies(
        &self,
        input: DescribePoliciesType,
    ) -> Result<PoliciesType, RusotoError<DescribePoliciesError>>;

    /// <p>Describes one or more scaling activities for the specified Auto Scaling group.</p>
    async fn describe_scaling_activities(
        &self,
        input: DescribeScalingActivitiesType,
    ) -> Result<ActivitiesType, RusotoError<DescribeScalingActivitiesError>>;

    /// <p>Describes the scaling process types for use with <a>ResumeProcesses</a> and <a>SuspendProcesses</a>.</p>
    async fn describe_scaling_process_types(
        &self,
    ) -> Result<ProcessesType, RusotoError<DescribeScalingProcessTypesError>>;

    /// <p>Describes the actions scheduled for your Auto Scaling group that haven't run or that have not reached their end time. To describe the actions that have already run, use <a>DescribeScalingActivities</a>.</p>
    async fn describe_scheduled_actions(
        &self,
        input: DescribeScheduledActionsType,
    ) -> Result<ScheduledActionsType, RusotoError<DescribeScheduledActionsError>>;

    /// <p>Describes the specified tags.</p> <p>You can use filters to limit the results. For example, you can query for the tags for a specific Auto Scaling group. You can specify multiple values for a filter. A tag must match at least one of the specified values for it to be included in the results.</p> <p>You can also specify multiple filters. The result includes information for a particular tag only if it matches all the filters. If there's no match, no special message is returned.</p>
    async fn describe_tags(
        &self,
        input: DescribeTagsType,
    ) -> Result<TagsType, RusotoError<DescribeTagsError>>;

    /// <p>Describes the termination policies supported by Amazon EC2 Auto Scaling.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/as-instance-termination.html">Controlling Which Auto Scaling Instances Terminate During Scale In</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    async fn describe_termination_policy_types(
        &self,
    ) -> Result<
        DescribeTerminationPolicyTypesAnswer,
        RusotoError<DescribeTerminationPolicyTypesError>,
    >;

    /// <p>Removes one or more instances from the specified Auto Scaling group.</p> <p>After the instances are detached, you can manage them independent of the Auto Scaling group.</p> <p>If you do not specify the option to decrement the desired capacity, Amazon EC2 Auto Scaling launches instances to replace the ones that are detached.</p> <p>If there is a Classic Load Balancer attached to the Auto Scaling group, the instances are deregistered from the load balancer. If there are target groups attached to the Auto Scaling group, the instances are deregistered from the target groups.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/detach-instance-asg.html">Detach EC2 Instances from Your Auto Scaling Group</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    async fn detach_instances(
        &self,
        input: DetachInstancesQuery,
    ) -> Result<DetachInstancesAnswer, RusotoError<DetachInstancesError>>;

    /// <p>Detaches one or more target groups from the specified Auto Scaling group.</p>
    async fn detach_load_balancer_target_groups(
        &self,
        input: DetachLoadBalancerTargetGroupsType,
    ) -> Result<
        DetachLoadBalancerTargetGroupsResultType,
        RusotoError<DetachLoadBalancerTargetGroupsError>,
    >;

    /// <p>Detaches one or more Classic Load Balancers from the specified Auto Scaling group.</p> <p>This operation detaches only Classic Load Balancers. If you have Application Load Balancers or Network Load Balancers, use <a>DetachLoadBalancerTargetGroups</a> instead.</p> <p>When you detach a load balancer, it enters the <code>Removing</code> state while deregistering the instances in the group. When all instances are deregistered, then you can no longer describe the load balancer using <a>DescribeLoadBalancers</a>. The instances remain running.</p>
    async fn detach_load_balancers(
        &self,
        input: DetachLoadBalancersType,
    ) -> Result<DetachLoadBalancersResultType, RusotoError<DetachLoadBalancersError>>;

    /// <p>Disables group metrics for the specified Auto Scaling group.</p>
    async fn disable_metrics_collection(
        &self,
        input: DisableMetricsCollectionQuery,
    ) -> Result<(), RusotoError<DisableMetricsCollectionError>>;

    /// <p>Enables group metrics for the specified Auto Scaling group. For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/as-instance-monitoring.html">Monitoring Your Auto Scaling Groups and Instances</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    async fn enable_metrics_collection(
        &self,
        input: EnableMetricsCollectionQuery,
    ) -> Result<(), RusotoError<EnableMetricsCollectionError>>;

    /// <p>Moves the specified instances into the standby state.</p> <p>If you choose to decrement the desired capacity of the Auto Scaling group, the instances can enter standby as long as the desired capacity of the Auto Scaling group after the instances are placed into standby is equal to or greater than the minimum capacity of the group.</p> <p>If you choose not to decrement the desired capacity of the Auto Scaling group, the Auto Scaling group launches new instances to replace the instances on standby.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/as-enter-exit-standby.html">Temporarily Removing Instances from Your Auto Scaling Group</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    async fn enter_standby(
        &self,
        input: EnterStandbyQuery,
    ) -> Result<EnterStandbyAnswer, RusotoError<EnterStandbyError>>;

    /// <p>Executes the specified policy.</p>
    async fn execute_policy(
        &self,
        input: ExecutePolicyType,
    ) -> Result<(), RusotoError<ExecutePolicyError>>;

    /// <p>Moves the specified instances out of the standby state.</p> <p>After you put the instances back in service, the desired capacity is incremented.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/as-enter-exit-standby.html">Temporarily Removing Instances from Your Auto Scaling Group</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    async fn exit_standby(
        &self,
        input: ExitStandbyQuery,
    ) -> Result<ExitStandbyAnswer, RusotoError<ExitStandbyError>>;

    /// <p>Creates or updates a lifecycle hook for the specified Auto Scaling group.</p> <p>A lifecycle hook tells Amazon EC2 Auto Scaling to perform an action on an instance when the instance launches (before it is put into service) or as the instance terminates (before it is fully terminated).</p> <p>This step is a part of the procedure for adding a lifecycle hook to an Auto Scaling group:</p> <ol> <li> <p>(Optional) Create a Lambda function and a rule that allows CloudWatch Events to invoke your Lambda function when Amazon EC2 Auto Scaling launches or terminates instances.</p> </li> <li> <p>(Optional) Create a notification target and an IAM role. The target can be either an Amazon SQS queue or an Amazon SNS topic. The role allows Amazon EC2 Auto Scaling to publish lifecycle notifications to the target.</p> </li> <li> <p> <b>Create the lifecycle hook. Specify whether the hook is used when the instances launch or terminate.</b> </p> </li> <li> <p>If you need more time, record the lifecycle action heartbeat to keep the instance in a pending state using <a>RecordLifecycleActionHeartbeat</a>.</p> </li> <li> <p>If you finish before the timeout period ends, complete the lifecycle action using <a>CompleteLifecycleAction</a>.</p> </li> </ol> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/lifecycle-hooks.html">Amazon EC2 Auto Scaling Lifecycle Hooks</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p> <p>If you exceed your maximum limit of lifecycle hooks, which by default is 50 per Auto Scaling group, the call fails.</p> <p>You can view the lifecycle hooks for an Auto Scaling group using <a>DescribeLifecycleHooks</a>. If you are no longer using a lifecycle hook, you can delete it using <a>DeleteLifecycleHook</a>.</p>
    async fn put_lifecycle_hook(
        &self,
        input: PutLifecycleHookType,
    ) -> Result<PutLifecycleHookAnswer, RusotoError<PutLifecycleHookError>>;

    /// <p>Configures an Auto Scaling group to send notifications when specified events take place. Subscribers to the specified topic can have messages delivered to an endpoint such as a web server or an email address.</p> <p>This configuration overwrites any existing configuration.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/ASGettingNotifications.html">Getting Amazon SNS Notifications When Your Auto Scaling Group Scales</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    async fn put_notification_configuration(
        &self,
        input: PutNotificationConfigurationType,
    ) -> Result<(), RusotoError<PutNotificationConfigurationError>>;

    /// <p>Creates or updates a scaling policy for an Auto Scaling group. To update an existing scaling policy, use the existing policy name and set the parameters to change. Any existing parameter not changed in an update to an existing policy is not changed in this update request.</p> <p>For more information about using scaling policies to scale your Auto Scaling group automatically, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/as-scale-based-on-demand.html">Dynamic Scaling</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    async fn put_scaling_policy(
        &self,
        input: PutScalingPolicyType,
    ) -> Result<PolicyARNType, RusotoError<PutScalingPolicyError>>;

    /// <p>Creates or updates a scheduled scaling action for an Auto Scaling group. If you leave a parameter unspecified when updating a scheduled scaling action, the corresponding value remains unchanged.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/schedule_time.html">Scheduled Scaling</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    async fn put_scheduled_update_group_action(
        &self,
        input: PutScheduledUpdateGroupActionType,
    ) -> Result<(), RusotoError<PutScheduledUpdateGroupActionError>>;

    /// <p>Records a heartbeat for the lifecycle action associated with the specified token or instance. This extends the timeout by the length of time defined using <a>PutLifecycleHook</a>.</p> <p>This step is a part of the procedure for adding a lifecycle hook to an Auto Scaling group:</p> <ol> <li> <p>(Optional) Create a Lambda function and a rule that allows CloudWatch Events to invoke your Lambda function when Amazon EC2 Auto Scaling launches or terminates instances.</p> </li> <li> <p>(Optional) Create a notification target and an IAM role. The target can be either an Amazon SQS queue or an Amazon SNS topic. The role allows Amazon EC2 Auto Scaling to publish lifecycle notifications to the target.</p> </li> <li> <p>Create the lifecycle hook. Specify whether the hook is used when the instances launch or terminate.</p> </li> <li> <p> <b>If you need more time, record the lifecycle action heartbeat to keep the instance in a pending state.</b> </p> </li> <li> <p>If you finish before the timeout period ends, complete the lifecycle action.</p> </li> </ol> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/AutoScalingGroupLifecycle.html">Auto Scaling Lifecycle</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    async fn record_lifecycle_action_heartbeat(
        &self,
        input: RecordLifecycleActionHeartbeatType,
    ) -> Result<
        RecordLifecycleActionHeartbeatAnswer,
        RusotoError<RecordLifecycleActionHeartbeatError>,
    >;

    /// <p>Resumes the specified suspended automatic scaling processes, or all suspended process, for the specified Auto Scaling group.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/as-suspend-resume-processes.html">Suspending and Resuming Scaling Processes</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    async fn resume_processes(
        &self,
        input: ScalingProcessQuery,
    ) -> Result<(), RusotoError<ResumeProcessesError>>;

    /// <p>Sets the size of the specified Auto Scaling group.</p> <p>For more information about desired capacity, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/what-is-amazon-ec2-auto-scaling.html">What Is Amazon EC2 Auto Scaling?</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    async fn set_desired_capacity(
        &self,
        input: SetDesiredCapacityType,
    ) -> Result<(), RusotoError<SetDesiredCapacityError>>;

    /// <p>Sets the health status of the specified instance.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/healthcheck.html">Health Checks for Auto Scaling Instances</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    async fn set_instance_health(
        &self,
        input: SetInstanceHealthQuery,
    ) -> Result<(), RusotoError<SetInstanceHealthError>>;

    /// <p>Updates the instance protection settings of the specified instances.</p> <p>For more information about preventing instances that are part of an Auto Scaling group from terminating on scale in, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/as-instance-termination.html#instance-protection">Instance Protection</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    async fn set_instance_protection(
        &self,
        input: SetInstanceProtectionQuery,
    ) -> Result<SetInstanceProtectionAnswer, RusotoError<SetInstanceProtectionError>>;

    /// <p>Suspends the specified automatic scaling processes, or all processes, for the specified Auto Scaling group.</p> <p>If you suspend either the <code>Launch</code> or <code>Terminate</code> process types, it can prevent other process types from functioning properly.</p> <p>To resume processes that have been suspended, use <a>ResumeProcesses</a>.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/as-suspend-resume-processes.html">Suspending and Resuming Scaling Processes</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    async fn suspend_processes(
        &self,
        input: ScalingProcessQuery,
    ) -> Result<(), RusotoError<SuspendProcessesError>>;

    /// <p>Terminates the specified instance and optionally adjusts the desired group size.</p> <p>This call simply makes a termination request. The instance is not terminated immediately.</p>
    async fn terminate_instance_in_auto_scaling_group(
        &self,
        input: TerminateInstanceInAutoScalingGroupType,
    ) -> Result<ActivityType, RusotoError<TerminateInstanceInAutoScalingGroupError>>;

    /// <p>Updates the configuration for the specified Auto Scaling group.</p> <p>To update an Auto Scaling group, specify the name of the group and the parameter that you want to change. Any parameters that you don't specify are not changed by this update request. The new settings take effect on any scaling activities after this call returns. </p> <p>If you associate a new launch configuration or template with an Auto Scaling group, all new instances will get the updated configuration. Existing instances continue to run with the configuration that they were originally launched with. When you update a group to specify a mixed instances policy instead of a launch configuration or template, existing instances may be replaced to match the new purchasing options that you specified in the policy. For example, if the group currently has 100% On-Demand capacity and the policy specifies 50% Spot capacity, this means that half of your instances will be gradually terminated and relaunched as Spot Instances. When replacing instances, Amazon EC2 Auto Scaling launches new instances before terminating the old ones, so that updating your group does not compromise the performance or availability of your application.</p> <p>Note the following about changing <code>DesiredCapacity</code>, <code>MaxSize</code>, or <code>MinSize</code>:</p> <ul> <li> <p>If a scale-in event occurs as a result of a new <code>DesiredCapacity</code> value that is lower than the current size of the group, the Auto Scaling group uses its termination policy to determine which instances to terminate.</p> </li> <li> <p>If you specify a new value for <code>MinSize</code> without specifying a value for <code>DesiredCapacity</code>, and the new <code>MinSize</code> is larger than the current size of the group, this sets the group's <code>DesiredCapacity</code> to the new <code>MinSize</code> value.</p> </li> <li> <p>If you specify a new value for <code>MaxSize</code> without specifying a value for <code>DesiredCapacity</code>, and the new <code>MaxSize</code> is smaller than the current size of the group, this sets the group's <code>DesiredCapacity</code> to the new <code>MaxSize</code> value.</p> </li> </ul> <p>To see which parameters have been set, use <a>DescribeAutoScalingGroups</a>. You can also view the scaling policies for an Auto Scaling group using <a>DescribePolicies</a>. If the group has scaling policies, you can update them using <a>PutScalingPolicy</a>.</p>
    async fn update_auto_scaling_group(
        &self,
        input: UpdateAutoScalingGroupType,
    ) -> Result<(), RusotoError<UpdateAutoScalingGroupError>>;
}
/// A client for the Auto Scaling API.
#[derive(Clone)]
pub struct AutoscalingClient {
    client: Client,
    region: region::Region,
}

impl AutoscalingClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> AutoscalingClient {
        AutoscalingClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> AutoscalingClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        AutoscalingClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> AutoscalingClient {
        AutoscalingClient { client, region }
    }
}

#[async_trait]
impl Autoscaling for AutoscalingClient {
    /// <p>Attaches one or more EC2 instances to the specified Auto Scaling group.</p> <p>When you attach instances, Amazon EC2 Auto Scaling increases the desired capacity of the group by the number of instances being attached. If the number of instances being attached plus the desired capacity of the group exceeds the maximum size of the group, the operation fails.</p> <p>If there is a Classic Load Balancer attached to your Auto Scaling group, the instances are also registered with the load balancer. If there are target groups attached to your Auto Scaling group, the instances are also registered with the target groups.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/attach-instance-asg.html">Attach EC2 Instances to Your Auto Scaling Group</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    async fn attach_instances(
        &self,
        input: AttachInstancesQuery,
    ) -> Result<(), RusotoError<AttachInstancesError>> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "AttachInstances");
        params.put("Version", "2011-01-01");
        AttachInstancesQuerySerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(AttachInstancesError::from_response(response));
        }

        Ok(std::mem::drop(response))
    }

    /// <p>Attaches one or more target groups to the specified Auto Scaling group.</p> <p>To describe the target groups for an Auto Scaling group, use <a>DescribeLoadBalancerTargetGroups</a>. To detach the target group from the Auto Scaling group, use <a>DetachLoadBalancerTargetGroups</a>.</p> <p>With Application Load Balancers and Network Load Balancers, instances are registered as targets with a target group. With Classic Load Balancers, instances are registered with the load balancer. For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/attach-load-balancer-asg.html">Attaching a Load Balancer to Your Auto Scaling Group</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    async fn attach_load_balancer_target_groups(
        &self,
        input: AttachLoadBalancerTargetGroupsType,
    ) -> Result<
        AttachLoadBalancerTargetGroupsResultType,
        RusotoError<AttachLoadBalancerTargetGroupsError>,
    > {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "AttachLoadBalancerTargetGroups");
        params.put("Version", "2011-01-01");
        AttachLoadBalancerTargetGroupsTypeSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(AttachLoadBalancerTargetGroupsError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = AttachLoadBalancerTargetGroupsResultType::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = AttachLoadBalancerTargetGroupsResultTypeDeserializer::deserialize(
                "AttachLoadBalancerTargetGroupsResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Attaches one or more Classic Load Balancers to the specified Auto Scaling group.</p> <p>To attach an Application Load Balancer or a Network Load Balancer instead, see <a>AttachLoadBalancerTargetGroups</a>.</p> <p>To describe the load balancers for an Auto Scaling group, use <a>DescribeLoadBalancers</a>. To detach the load balancer from the Auto Scaling group, use <a>DetachLoadBalancers</a>.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/attach-load-balancer-asg.html">Attaching a Load Balancer to Your Auto Scaling Group</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    async fn attach_load_balancers(
        &self,
        input: AttachLoadBalancersType,
    ) -> Result<AttachLoadBalancersResultType, RusotoError<AttachLoadBalancersError>> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "AttachLoadBalancers");
        params.put("Version", "2011-01-01");
        AttachLoadBalancersTypeSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(AttachLoadBalancersError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = AttachLoadBalancersResultType::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = AttachLoadBalancersResultTypeDeserializer::deserialize(
                "AttachLoadBalancersResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Deletes one or more scheduled actions for the specified Auto Scaling group.</p>
    async fn batch_delete_scheduled_action(
        &self,
        input: BatchDeleteScheduledActionType,
    ) -> Result<BatchDeleteScheduledActionAnswer, RusotoError<BatchDeleteScheduledActionError>>
    {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "BatchDeleteScheduledAction");
        params.put("Version", "2011-01-01");
        BatchDeleteScheduledActionTypeSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(BatchDeleteScheduledActionError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = BatchDeleteScheduledActionAnswer::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = BatchDeleteScheduledActionAnswerDeserializer::deserialize(
                "BatchDeleteScheduledActionResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Creates or updates one or more scheduled scaling actions for an Auto Scaling group. If you leave a parameter unspecified when updating a scheduled scaling action, the corresponding value remains unchanged.</p>
    async fn batch_put_scheduled_update_group_action(
        &self,
        input: BatchPutScheduledUpdateGroupActionType,
    ) -> Result<
        BatchPutScheduledUpdateGroupActionAnswer,
        RusotoError<BatchPutScheduledUpdateGroupActionError>,
    > {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "BatchPutScheduledUpdateGroupAction");
        params.put("Version", "2011-01-01");
        BatchPutScheduledUpdateGroupActionTypeSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(BatchPutScheduledUpdateGroupActionError::from_response(
                response,
            ));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = BatchPutScheduledUpdateGroupActionAnswer::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = BatchPutScheduledUpdateGroupActionAnswerDeserializer::deserialize(
                "BatchPutScheduledUpdateGroupActionResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Completes the lifecycle action for the specified token or instance with the specified result.</p> <p>This step is a part of the procedure for adding a lifecycle hook to an Auto Scaling group:</p> <ol> <li> <p>(Optional) Create a Lambda function and a rule that allows CloudWatch Events to invoke your Lambda function when Amazon EC2 Auto Scaling launches or terminates instances.</p> </li> <li> <p>(Optional) Create a notification target and an IAM role. The target can be either an Amazon SQS queue or an Amazon SNS topic. The role allows Amazon EC2 Auto Scaling to publish lifecycle notifications to the target.</p> </li> <li> <p>Create the lifecycle hook. Specify whether the hook is used when the instances launch or terminate.</p> </li> <li> <p>If you need more time, record the lifecycle action heartbeat to keep the instance in a pending state.</p> </li> <li> <p> <b>If you finish before the timeout period ends, complete the lifecycle action.</b> </p> </li> </ol> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/lifecycle-hooks.html">Amazon EC2 Auto Scaling Lifecycle Hooks</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    async fn complete_lifecycle_action(
        &self,
        input: CompleteLifecycleActionType,
    ) -> Result<CompleteLifecycleActionAnswer, RusotoError<CompleteLifecycleActionError>> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CompleteLifecycleAction");
        params.put("Version", "2011-01-01");
        CompleteLifecycleActionTypeSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(CompleteLifecycleActionError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = CompleteLifecycleActionAnswer::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = CompleteLifecycleActionAnswerDeserializer::deserialize(
                "CompleteLifecycleActionResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Creates an Auto Scaling group with the specified name and attributes.</p> <p>If you exceed your maximum limit of Auto Scaling groups, the call fails. For information about viewing this limit, see <a>DescribeAccountLimits</a>. For information about updating this limit, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/as-account-limits.html">Amazon EC2 Auto Scaling Limits</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    async fn create_auto_scaling_group(
        &self,
        input: CreateAutoScalingGroupType,
    ) -> Result<(), RusotoError<CreateAutoScalingGroupError>> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CreateAutoScalingGroup");
        params.put("Version", "2011-01-01");
        CreateAutoScalingGroupTypeSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(CreateAutoScalingGroupError::from_response(response));
        }

        Ok(std::mem::drop(response))
    }

    /// <p>Creates a launch configuration.</p> <p>If you exceed your maximum limit of launch configurations, the call fails. For information about viewing this limit, see <a>DescribeAccountLimits</a>. For information about updating this limit, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/as-account-limits.html">Amazon EC2 Auto Scaling Limits</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/LaunchConfiguration.html">Launch Configurations</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    async fn create_launch_configuration(
        &self,
        input: CreateLaunchConfigurationType,
    ) -> Result<(), RusotoError<CreateLaunchConfigurationError>> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CreateLaunchConfiguration");
        params.put("Version", "2011-01-01");
        CreateLaunchConfigurationTypeSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(CreateLaunchConfigurationError::from_response(response));
        }

        Ok(std::mem::drop(response))
    }

    /// <p>Creates or updates tags for the specified Auto Scaling group.</p> <p>When you specify a tag with a key that already exists, the operation overwrites the previous tag definition, and you do not get an error message.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/autoscaling-tagging.html">Tagging Auto Scaling Groups and Instances</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    async fn create_or_update_tags(
        &self,
        input: CreateOrUpdateTagsType,
    ) -> Result<(), RusotoError<CreateOrUpdateTagsError>> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CreateOrUpdateTags");
        params.put("Version", "2011-01-01");
        CreateOrUpdateTagsTypeSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(CreateOrUpdateTagsError::from_response(response));
        }

        Ok(std::mem::drop(response))
    }

    /// <p>Deletes the specified Auto Scaling group.</p> <p>If the group has instances or scaling activities in progress, you must specify the option to force the deletion in order for it to succeed.</p> <p>If the group has policies, deleting the group deletes the policies, the underlying alarm actions, and any alarm that no longer has an associated action.</p> <p>To remove instances from the Auto Scaling group before deleting it, call <a>DetachInstances</a> with the list of instances and the option to decrement the desired capacity. This ensures that Amazon EC2 Auto Scaling does not launch replacement instances.</p> <p>To terminate all instances before deleting the Auto Scaling group, call <a>UpdateAutoScalingGroup</a> and set the minimum size and desired capacity of the Auto Scaling group to zero.</p>
    async fn delete_auto_scaling_group(
        &self,
        input: DeleteAutoScalingGroupType,
    ) -> Result<(), RusotoError<DeleteAutoScalingGroupError>> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteAutoScalingGroup");
        params.put("Version", "2011-01-01");
        DeleteAutoScalingGroupTypeSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DeleteAutoScalingGroupError::from_response(response));
        }

        Ok(std::mem::drop(response))
    }

    /// <p>Deletes the specified launch configuration.</p> <p>The launch configuration must not be attached to an Auto Scaling group. When this call completes, the launch configuration is no longer available for use.</p>
    async fn delete_launch_configuration(
        &self,
        input: LaunchConfigurationNameType,
    ) -> Result<(), RusotoError<DeleteLaunchConfigurationError>> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteLaunchConfiguration");
        params.put("Version", "2011-01-01");
        LaunchConfigurationNameTypeSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DeleteLaunchConfigurationError::from_response(response));
        }

        Ok(std::mem::drop(response))
    }

    /// <p>Deletes the specified lifecycle hook.</p> <p>If there are any outstanding lifecycle actions, they are completed first (<code>ABANDON</code> for launching instances, <code>CONTINUE</code> for terminating instances).</p>
    async fn delete_lifecycle_hook(
        &self,
        input: DeleteLifecycleHookType,
    ) -> Result<DeleteLifecycleHookAnswer, RusotoError<DeleteLifecycleHookError>> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteLifecycleHook");
        params.put("Version", "2011-01-01");
        DeleteLifecycleHookTypeSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DeleteLifecycleHookError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = DeleteLifecycleHookAnswer::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = DeleteLifecycleHookAnswerDeserializer::deserialize(
                "DeleteLifecycleHookResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Deletes the specified notification.</p>
    async fn delete_notification_configuration(
        &self,
        input: DeleteNotificationConfigurationType,
    ) -> Result<(), RusotoError<DeleteNotificationConfigurationError>> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteNotificationConfiguration");
        params.put("Version", "2011-01-01");
        DeleteNotificationConfigurationTypeSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DeleteNotificationConfigurationError::from_response(
                response,
            ));
        }

        Ok(std::mem::drop(response))
    }

    /// <p>Deletes the specified scaling policy.</p> <p>Deleting either a step scaling policy or a simple scaling policy deletes the underlying alarm action, but does not delete the alarm, even if it no longer has an associated action.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/deleting-scaling-policy.html">Deleting a Scaling Policy</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    async fn delete_policy(
        &self,
        input: DeletePolicyType,
    ) -> Result<(), RusotoError<DeletePolicyError>> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeletePolicy");
        params.put("Version", "2011-01-01");
        DeletePolicyTypeSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DeletePolicyError::from_response(response));
        }

        Ok(std::mem::drop(response))
    }

    /// <p>Deletes the specified scheduled action.</p>
    async fn delete_scheduled_action(
        &self,
        input: DeleteScheduledActionType,
    ) -> Result<(), RusotoError<DeleteScheduledActionError>> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteScheduledAction");
        params.put("Version", "2011-01-01");
        DeleteScheduledActionTypeSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DeleteScheduledActionError::from_response(response));
        }

        Ok(std::mem::drop(response))
    }

    /// <p>Deletes the specified tags.</p>
    async fn delete_tags(&self, input: DeleteTagsType) -> Result<(), RusotoError<DeleteTagsError>> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteTags");
        params.put("Version", "2011-01-01");
        DeleteTagsTypeSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DeleteTagsError::from_response(response));
        }

        Ok(std::mem::drop(response))
    }

    /// <p>Describes the current Amazon EC2 Auto Scaling resource limits for your AWS account.</p> <p>For information about requesting an increase in these limits, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/as-account-limits.html">Amazon EC2 Auto Scaling Limits</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    async fn describe_account_limits(
        &self,
    ) -> Result<DescribeAccountLimitsAnswer, RusotoError<DescribeAccountLimitsError>> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeAccountLimits");
        params.put("Version", "2011-01-01");

        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DescribeAccountLimitsError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = DescribeAccountLimitsAnswer::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = DescribeAccountLimitsAnswerDeserializer::deserialize(
                "DescribeAccountLimitsResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Describes the policy adjustment types for use with <a>PutScalingPolicy</a>.</p>
    async fn describe_adjustment_types(
        &self,
    ) -> Result<DescribeAdjustmentTypesAnswer, RusotoError<DescribeAdjustmentTypesError>> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeAdjustmentTypes");
        params.put("Version", "2011-01-01");

        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DescribeAdjustmentTypesError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = DescribeAdjustmentTypesAnswer::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = DescribeAdjustmentTypesAnswerDeserializer::deserialize(
                "DescribeAdjustmentTypesResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Describes one or more Auto Scaling groups.</p>
    async fn describe_auto_scaling_groups(
        &self,
        input: AutoScalingGroupNamesType,
    ) -> Result<AutoScalingGroupsType, RusotoError<DescribeAutoScalingGroupsError>> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeAutoScalingGroups");
        params.put("Version", "2011-01-01");
        AutoScalingGroupNamesTypeSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DescribeAutoScalingGroupsError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = AutoScalingGroupsType::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = AutoScalingGroupsTypeDeserializer::deserialize(
                "DescribeAutoScalingGroupsResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Describes one or more Auto Scaling instances.</p>
    async fn describe_auto_scaling_instances(
        &self,
        input: DescribeAutoScalingInstancesType,
    ) -> Result<AutoScalingInstancesType, RusotoError<DescribeAutoScalingInstancesError>> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeAutoScalingInstances");
        params.put("Version", "2011-01-01");
        DescribeAutoScalingInstancesTypeSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DescribeAutoScalingInstancesError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = AutoScalingInstancesType::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = AutoScalingInstancesTypeDeserializer::deserialize(
                "DescribeAutoScalingInstancesResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Describes the notification types that are supported by Amazon EC2 Auto Scaling.</p>
    async fn describe_auto_scaling_notification_types(
        &self,
    ) -> Result<
        DescribeAutoScalingNotificationTypesAnswer,
        RusotoError<DescribeAutoScalingNotificationTypesError>,
    > {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeAutoScalingNotificationTypes");
        params.put("Version", "2011-01-01");

        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DescribeAutoScalingNotificationTypesError::from_response(
                response,
            ));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = DescribeAutoScalingNotificationTypesAnswer::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = DescribeAutoScalingNotificationTypesAnswerDeserializer::deserialize(
                "DescribeAutoScalingNotificationTypesResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Describes one or more launch configurations.</p>
    async fn describe_launch_configurations(
        &self,
        input: LaunchConfigurationNamesType,
    ) -> Result<LaunchConfigurationsType, RusotoError<DescribeLaunchConfigurationsError>> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeLaunchConfigurations");
        params.put("Version", "2011-01-01");
        LaunchConfigurationNamesTypeSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DescribeLaunchConfigurationsError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = LaunchConfigurationsType::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = LaunchConfigurationsTypeDeserializer::deserialize(
                "DescribeLaunchConfigurationsResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p><p>Describes the available types of lifecycle hooks.</p> <p>The following hook types are supported:</p> <ul> <li> <p>autoscaling:EC2<em>INSTANCE</em>LAUNCHING</p> </li> <li> <p>autoscaling:EC2<em>INSTANCE</em>TERMINATING</p> </li> </ul></p>
    async fn describe_lifecycle_hook_types(
        &self,
    ) -> Result<DescribeLifecycleHookTypesAnswer, RusotoError<DescribeLifecycleHookTypesError>>
    {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeLifecycleHookTypes");
        params.put("Version", "2011-01-01");

        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DescribeLifecycleHookTypesError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = DescribeLifecycleHookTypesAnswer::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = DescribeLifecycleHookTypesAnswerDeserializer::deserialize(
                "DescribeLifecycleHookTypesResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Describes the lifecycle hooks for the specified Auto Scaling group.</p>
    async fn describe_lifecycle_hooks(
        &self,
        input: DescribeLifecycleHooksType,
    ) -> Result<DescribeLifecycleHooksAnswer, RusotoError<DescribeLifecycleHooksError>> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeLifecycleHooks");
        params.put("Version", "2011-01-01");
        DescribeLifecycleHooksTypeSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DescribeLifecycleHooksError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = DescribeLifecycleHooksAnswer::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = DescribeLifecycleHooksAnswerDeserializer::deserialize(
                "DescribeLifecycleHooksResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Describes the target groups for the specified Auto Scaling group.</p>
    async fn describe_load_balancer_target_groups(
        &self,
        input: DescribeLoadBalancerTargetGroupsRequest,
    ) -> Result<
        DescribeLoadBalancerTargetGroupsResponse,
        RusotoError<DescribeLoadBalancerTargetGroupsError>,
    > {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeLoadBalancerTargetGroups");
        params.put("Version", "2011-01-01");
        DescribeLoadBalancerTargetGroupsRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DescribeLoadBalancerTargetGroupsError::from_response(
                response,
            ));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = DescribeLoadBalancerTargetGroupsResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = DescribeLoadBalancerTargetGroupsResponseDeserializer::deserialize(
                "DescribeLoadBalancerTargetGroupsResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Describes the load balancers for the specified Auto Scaling group.</p> <p>This operation describes only Classic Load Balancers. If you have Application Load Balancers or Network Load Balancers, use <a>DescribeLoadBalancerTargetGroups</a> instead.</p>
    async fn describe_load_balancers(
        &self,
        input: DescribeLoadBalancersRequest,
    ) -> Result<DescribeLoadBalancersResponse, RusotoError<DescribeLoadBalancersError>> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeLoadBalancers");
        params.put("Version", "2011-01-01");
        DescribeLoadBalancersRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DescribeLoadBalancersError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = DescribeLoadBalancersResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = DescribeLoadBalancersResponseDeserializer::deserialize(
                "DescribeLoadBalancersResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Describes the available CloudWatch metrics for Amazon EC2 Auto Scaling.</p> <p>The <code>GroupStandbyInstances</code> metric is not returned by default. You must explicitly request this metric when calling <a>EnableMetricsCollection</a>.</p>
    async fn describe_metric_collection_types(
        &self,
    ) -> Result<DescribeMetricCollectionTypesAnswer, RusotoError<DescribeMetricCollectionTypesError>>
    {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeMetricCollectionTypes");
        params.put("Version", "2011-01-01");

        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DescribeMetricCollectionTypesError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = DescribeMetricCollectionTypesAnswer::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = DescribeMetricCollectionTypesAnswerDeserializer::deserialize(
                "DescribeMetricCollectionTypesResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Describes the notification actions associated with the specified Auto Scaling group.</p>
    async fn describe_notification_configurations(
        &self,
        input: DescribeNotificationConfigurationsType,
    ) -> Result<
        DescribeNotificationConfigurationsAnswer,
        RusotoError<DescribeNotificationConfigurationsError>,
    > {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeNotificationConfigurations");
        params.put("Version", "2011-01-01");
        DescribeNotificationConfigurationsTypeSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DescribeNotificationConfigurationsError::from_response(
                response,
            ));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = DescribeNotificationConfigurationsAnswer::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = DescribeNotificationConfigurationsAnswerDeserializer::deserialize(
                "DescribeNotificationConfigurationsResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Describes the policies for the specified Auto Scaling group.</p>
    async fn describe_policies(
        &self,
        input: DescribePoliciesType,
    ) -> Result<PoliciesType, RusotoError<DescribePoliciesError>> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribePolicies");
        params.put("Version", "2011-01-01");
        DescribePoliciesTypeSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DescribePoliciesError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = PoliciesType::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = PoliciesTypeDeserializer::deserialize("DescribePoliciesResult", &mut stack)?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Describes one or more scaling activities for the specified Auto Scaling group.</p>
    async fn describe_scaling_activities(
        &self,
        input: DescribeScalingActivitiesType,
    ) -> Result<ActivitiesType, RusotoError<DescribeScalingActivitiesError>> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeScalingActivities");
        params.put("Version", "2011-01-01");
        DescribeScalingActivitiesTypeSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DescribeScalingActivitiesError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = ActivitiesType::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = ActivitiesTypeDeserializer::deserialize(
                "DescribeScalingActivitiesResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Describes the scaling process types for use with <a>ResumeProcesses</a> and <a>SuspendProcesses</a>.</p>
    async fn describe_scaling_process_types(
        &self,
    ) -> Result<ProcessesType, RusotoError<DescribeScalingProcessTypesError>> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeScalingProcessTypes");
        params.put("Version", "2011-01-01");

        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DescribeScalingProcessTypesError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = ProcessesType::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = ProcessesTypeDeserializer::deserialize(
                "DescribeScalingProcessTypesResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Describes the actions scheduled for your Auto Scaling group that haven't run or that have not reached their end time. To describe the actions that have already run, use <a>DescribeScalingActivities</a>.</p>
    async fn describe_scheduled_actions(
        &self,
        input: DescribeScheduledActionsType,
    ) -> Result<ScheduledActionsType, RusotoError<DescribeScheduledActionsError>> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeScheduledActions");
        params.put("Version", "2011-01-01");
        DescribeScheduledActionsTypeSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DescribeScheduledActionsError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = ScheduledActionsType::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = ScheduledActionsTypeDeserializer::deserialize(
                "DescribeScheduledActionsResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Describes the specified tags.</p> <p>You can use filters to limit the results. For example, you can query for the tags for a specific Auto Scaling group. You can specify multiple values for a filter. A tag must match at least one of the specified values for it to be included in the results.</p> <p>You can also specify multiple filters. The result includes information for a particular tag only if it matches all the filters. If there's no match, no special message is returned.</p>
    async fn describe_tags(
        &self,
        input: DescribeTagsType,
    ) -> Result<TagsType, RusotoError<DescribeTagsError>> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeTags");
        params.put("Version", "2011-01-01");
        DescribeTagsTypeSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DescribeTagsError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = TagsType::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = TagsTypeDeserializer::deserialize("DescribeTagsResult", &mut stack)?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Describes the termination policies supported by Amazon EC2 Auto Scaling.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/as-instance-termination.html">Controlling Which Auto Scaling Instances Terminate During Scale In</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    async fn describe_termination_policy_types(
        &self,
    ) -> Result<
        DescribeTerminationPolicyTypesAnswer,
        RusotoError<DescribeTerminationPolicyTypesError>,
    > {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeTerminationPolicyTypes");
        params.put("Version", "2011-01-01");

        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DescribeTerminationPolicyTypesError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = DescribeTerminationPolicyTypesAnswer::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = DescribeTerminationPolicyTypesAnswerDeserializer::deserialize(
                "DescribeTerminationPolicyTypesResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Removes one or more instances from the specified Auto Scaling group.</p> <p>After the instances are detached, you can manage them independent of the Auto Scaling group.</p> <p>If you do not specify the option to decrement the desired capacity, Amazon EC2 Auto Scaling launches instances to replace the ones that are detached.</p> <p>If there is a Classic Load Balancer attached to the Auto Scaling group, the instances are deregistered from the load balancer. If there are target groups attached to the Auto Scaling group, the instances are deregistered from the target groups.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/detach-instance-asg.html">Detach EC2 Instances from Your Auto Scaling Group</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    async fn detach_instances(
        &self,
        input: DetachInstancesQuery,
    ) -> Result<DetachInstancesAnswer, RusotoError<DetachInstancesError>> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DetachInstances");
        params.put("Version", "2011-01-01");
        DetachInstancesQuerySerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DetachInstancesError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = DetachInstancesAnswer::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = DetachInstancesAnswerDeserializer::deserialize(
                "DetachInstancesResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Detaches one or more target groups from the specified Auto Scaling group.</p>
    async fn detach_load_balancer_target_groups(
        &self,
        input: DetachLoadBalancerTargetGroupsType,
    ) -> Result<
        DetachLoadBalancerTargetGroupsResultType,
        RusotoError<DetachLoadBalancerTargetGroupsError>,
    > {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DetachLoadBalancerTargetGroups");
        params.put("Version", "2011-01-01");
        DetachLoadBalancerTargetGroupsTypeSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DetachLoadBalancerTargetGroupsError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = DetachLoadBalancerTargetGroupsResultType::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = DetachLoadBalancerTargetGroupsResultTypeDeserializer::deserialize(
                "DetachLoadBalancerTargetGroupsResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Detaches one or more Classic Load Balancers from the specified Auto Scaling group.</p> <p>This operation detaches only Classic Load Balancers. If you have Application Load Balancers or Network Load Balancers, use <a>DetachLoadBalancerTargetGroups</a> instead.</p> <p>When you detach a load balancer, it enters the <code>Removing</code> state while deregistering the instances in the group. When all instances are deregistered, then you can no longer describe the load balancer using <a>DescribeLoadBalancers</a>. The instances remain running.</p>
    async fn detach_load_balancers(
        &self,
        input: DetachLoadBalancersType,
    ) -> Result<DetachLoadBalancersResultType, RusotoError<DetachLoadBalancersError>> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DetachLoadBalancers");
        params.put("Version", "2011-01-01");
        DetachLoadBalancersTypeSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DetachLoadBalancersError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = DetachLoadBalancersResultType::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = DetachLoadBalancersResultTypeDeserializer::deserialize(
                "DetachLoadBalancersResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Disables group metrics for the specified Auto Scaling group.</p>
    async fn disable_metrics_collection(
        &self,
        input: DisableMetricsCollectionQuery,
    ) -> Result<(), RusotoError<DisableMetricsCollectionError>> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DisableMetricsCollection");
        params.put("Version", "2011-01-01");
        DisableMetricsCollectionQuerySerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DisableMetricsCollectionError::from_response(response));
        }

        Ok(std::mem::drop(response))
    }

    /// <p>Enables group metrics for the specified Auto Scaling group. For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/as-instance-monitoring.html">Monitoring Your Auto Scaling Groups and Instances</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    async fn enable_metrics_collection(
        &self,
        input: EnableMetricsCollectionQuery,
    ) -> Result<(), RusotoError<EnableMetricsCollectionError>> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "EnableMetricsCollection");
        params.put("Version", "2011-01-01");
        EnableMetricsCollectionQuerySerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(EnableMetricsCollectionError::from_response(response));
        }

        Ok(std::mem::drop(response))
    }

    /// <p>Moves the specified instances into the standby state.</p> <p>If you choose to decrement the desired capacity of the Auto Scaling group, the instances can enter standby as long as the desired capacity of the Auto Scaling group after the instances are placed into standby is equal to or greater than the minimum capacity of the group.</p> <p>If you choose not to decrement the desired capacity of the Auto Scaling group, the Auto Scaling group launches new instances to replace the instances on standby.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/as-enter-exit-standby.html">Temporarily Removing Instances from Your Auto Scaling Group</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    async fn enter_standby(
        &self,
        input: EnterStandbyQuery,
    ) -> Result<EnterStandbyAnswer, RusotoError<EnterStandbyError>> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "EnterStandby");
        params.put("Version", "2011-01-01");
        EnterStandbyQuerySerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(EnterStandbyError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = EnterStandbyAnswer::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = EnterStandbyAnswerDeserializer::deserialize("EnterStandbyResult", &mut stack)?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Executes the specified policy.</p>
    async fn execute_policy(
        &self,
        input: ExecutePolicyType,
    ) -> Result<(), RusotoError<ExecutePolicyError>> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ExecutePolicy");
        params.put("Version", "2011-01-01");
        ExecutePolicyTypeSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(ExecutePolicyError::from_response(response));
        }

        Ok(std::mem::drop(response))
    }

    /// <p>Moves the specified instances out of the standby state.</p> <p>After you put the instances back in service, the desired capacity is incremented.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/as-enter-exit-standby.html">Temporarily Removing Instances from Your Auto Scaling Group</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    async fn exit_standby(
        &self,
        input: ExitStandbyQuery,
    ) -> Result<ExitStandbyAnswer, RusotoError<ExitStandbyError>> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ExitStandby");
        params.put("Version", "2011-01-01");
        ExitStandbyQuerySerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(ExitStandbyError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = ExitStandbyAnswer::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = ExitStandbyAnswerDeserializer::deserialize("ExitStandbyResult", &mut stack)?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Creates or updates a lifecycle hook for the specified Auto Scaling group.</p> <p>A lifecycle hook tells Amazon EC2 Auto Scaling to perform an action on an instance when the instance launches (before it is put into service) or as the instance terminates (before it is fully terminated).</p> <p>This step is a part of the procedure for adding a lifecycle hook to an Auto Scaling group:</p> <ol> <li> <p>(Optional) Create a Lambda function and a rule that allows CloudWatch Events to invoke your Lambda function when Amazon EC2 Auto Scaling launches or terminates instances.</p> </li> <li> <p>(Optional) Create a notification target and an IAM role. The target can be either an Amazon SQS queue or an Amazon SNS topic. The role allows Amazon EC2 Auto Scaling to publish lifecycle notifications to the target.</p> </li> <li> <p> <b>Create the lifecycle hook. Specify whether the hook is used when the instances launch or terminate.</b> </p> </li> <li> <p>If you need more time, record the lifecycle action heartbeat to keep the instance in a pending state using <a>RecordLifecycleActionHeartbeat</a>.</p> </li> <li> <p>If you finish before the timeout period ends, complete the lifecycle action using <a>CompleteLifecycleAction</a>.</p> </li> </ol> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/lifecycle-hooks.html">Amazon EC2 Auto Scaling Lifecycle Hooks</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p> <p>If you exceed your maximum limit of lifecycle hooks, which by default is 50 per Auto Scaling group, the call fails.</p> <p>You can view the lifecycle hooks for an Auto Scaling group using <a>DescribeLifecycleHooks</a>. If you are no longer using a lifecycle hook, you can delete it using <a>DeleteLifecycleHook</a>.</p>
    async fn put_lifecycle_hook(
        &self,
        input: PutLifecycleHookType,
    ) -> Result<PutLifecycleHookAnswer, RusotoError<PutLifecycleHookError>> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "PutLifecycleHook");
        params.put("Version", "2011-01-01");
        PutLifecycleHookTypeSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(PutLifecycleHookError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = PutLifecycleHookAnswer::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = PutLifecycleHookAnswerDeserializer::deserialize(
                "PutLifecycleHookResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Configures an Auto Scaling group to send notifications when specified events take place. Subscribers to the specified topic can have messages delivered to an endpoint such as a web server or an email address.</p> <p>This configuration overwrites any existing configuration.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/ASGettingNotifications.html">Getting Amazon SNS Notifications When Your Auto Scaling Group Scales</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    async fn put_notification_configuration(
        &self,
        input: PutNotificationConfigurationType,
    ) -> Result<(), RusotoError<PutNotificationConfigurationError>> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "PutNotificationConfiguration");
        params.put("Version", "2011-01-01");
        PutNotificationConfigurationTypeSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(PutNotificationConfigurationError::from_response(response));
        }

        Ok(std::mem::drop(response))
    }

    /// <p>Creates or updates a scaling policy for an Auto Scaling group. To update an existing scaling policy, use the existing policy name and set the parameters to change. Any existing parameter not changed in an update to an existing policy is not changed in this update request.</p> <p>For more information about using scaling policies to scale your Auto Scaling group automatically, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/as-scale-based-on-demand.html">Dynamic Scaling</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    async fn put_scaling_policy(
        &self,
        input: PutScalingPolicyType,
    ) -> Result<PolicyARNType, RusotoError<PutScalingPolicyError>> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "PutScalingPolicy");
        params.put("Version", "2011-01-01");
        PutScalingPolicyTypeSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(PutScalingPolicyError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = PolicyARNType::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = PolicyARNTypeDeserializer::deserialize("PutScalingPolicyResult", &mut stack)?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Creates or updates a scheduled scaling action for an Auto Scaling group. If you leave a parameter unspecified when updating a scheduled scaling action, the corresponding value remains unchanged.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/schedule_time.html">Scheduled Scaling</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    async fn put_scheduled_update_group_action(
        &self,
        input: PutScheduledUpdateGroupActionType,
    ) -> Result<(), RusotoError<PutScheduledUpdateGroupActionError>> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "PutScheduledUpdateGroupAction");
        params.put("Version", "2011-01-01");
        PutScheduledUpdateGroupActionTypeSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(PutScheduledUpdateGroupActionError::from_response(response));
        }

        Ok(std::mem::drop(response))
    }

    /// <p>Records a heartbeat for the lifecycle action associated with the specified token or instance. This extends the timeout by the length of time defined using <a>PutLifecycleHook</a>.</p> <p>This step is a part of the procedure for adding a lifecycle hook to an Auto Scaling group:</p> <ol> <li> <p>(Optional) Create a Lambda function and a rule that allows CloudWatch Events to invoke your Lambda function when Amazon EC2 Auto Scaling launches or terminates instances.</p> </li> <li> <p>(Optional) Create a notification target and an IAM role. The target can be either an Amazon SQS queue or an Amazon SNS topic. The role allows Amazon EC2 Auto Scaling to publish lifecycle notifications to the target.</p> </li> <li> <p>Create the lifecycle hook. Specify whether the hook is used when the instances launch or terminate.</p> </li> <li> <p> <b>If you need more time, record the lifecycle action heartbeat to keep the instance in a pending state.</b> </p> </li> <li> <p>If you finish before the timeout period ends, complete the lifecycle action.</p> </li> </ol> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/AutoScalingGroupLifecycle.html">Auto Scaling Lifecycle</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    async fn record_lifecycle_action_heartbeat(
        &self,
        input: RecordLifecycleActionHeartbeatType,
    ) -> Result<
        RecordLifecycleActionHeartbeatAnswer,
        RusotoError<RecordLifecycleActionHeartbeatError>,
    > {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "RecordLifecycleActionHeartbeat");
        params.put("Version", "2011-01-01");
        RecordLifecycleActionHeartbeatTypeSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(RecordLifecycleActionHeartbeatError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = RecordLifecycleActionHeartbeatAnswer::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = RecordLifecycleActionHeartbeatAnswerDeserializer::deserialize(
                "RecordLifecycleActionHeartbeatResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Resumes the specified suspended automatic scaling processes, or all suspended process, for the specified Auto Scaling group.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/as-suspend-resume-processes.html">Suspending and Resuming Scaling Processes</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    async fn resume_processes(
        &self,
        input: ScalingProcessQuery,
    ) -> Result<(), RusotoError<ResumeProcessesError>> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ResumeProcesses");
        params.put("Version", "2011-01-01");
        ScalingProcessQuerySerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(ResumeProcessesError::from_response(response));
        }

        Ok(std::mem::drop(response))
    }

    /// <p>Sets the size of the specified Auto Scaling group.</p> <p>For more information about desired capacity, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/what-is-amazon-ec2-auto-scaling.html">What Is Amazon EC2 Auto Scaling?</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    async fn set_desired_capacity(
        &self,
        input: SetDesiredCapacityType,
    ) -> Result<(), RusotoError<SetDesiredCapacityError>> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "SetDesiredCapacity");
        params.put("Version", "2011-01-01");
        SetDesiredCapacityTypeSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(SetDesiredCapacityError::from_response(response));
        }

        Ok(std::mem::drop(response))
    }

    /// <p>Sets the health status of the specified instance.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/healthcheck.html">Health Checks for Auto Scaling Instances</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    async fn set_instance_health(
        &self,
        input: SetInstanceHealthQuery,
    ) -> Result<(), RusotoError<SetInstanceHealthError>> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "SetInstanceHealth");
        params.put("Version", "2011-01-01");
        SetInstanceHealthQuerySerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(SetInstanceHealthError::from_response(response));
        }

        Ok(std::mem::drop(response))
    }

    /// <p>Updates the instance protection settings of the specified instances.</p> <p>For more information about preventing instances that are part of an Auto Scaling group from terminating on scale in, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/as-instance-termination.html#instance-protection">Instance Protection</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    async fn set_instance_protection(
        &self,
        input: SetInstanceProtectionQuery,
    ) -> Result<SetInstanceProtectionAnswer, RusotoError<SetInstanceProtectionError>> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "SetInstanceProtection");
        params.put("Version", "2011-01-01");
        SetInstanceProtectionQuerySerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(SetInstanceProtectionError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = SetInstanceProtectionAnswer::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = SetInstanceProtectionAnswerDeserializer::deserialize(
                "SetInstanceProtectionResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Suspends the specified automatic scaling processes, or all processes, for the specified Auto Scaling group.</p> <p>If you suspend either the <code>Launch</code> or <code>Terminate</code> process types, it can prevent other process types from functioning properly.</p> <p>To resume processes that have been suspended, use <a>ResumeProcesses</a>.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/as-suspend-resume-processes.html">Suspending and Resuming Scaling Processes</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    async fn suspend_processes(
        &self,
        input: ScalingProcessQuery,
    ) -> Result<(), RusotoError<SuspendProcessesError>> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "SuspendProcesses");
        params.put("Version", "2011-01-01");
        ScalingProcessQuerySerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(SuspendProcessesError::from_response(response));
        }

        Ok(std::mem::drop(response))
    }

    /// <p>Terminates the specified instance and optionally adjusts the desired group size.</p> <p>This call simply makes a termination request. The instance is not terminated immediately.</p>
    async fn terminate_instance_in_auto_scaling_group(
        &self,
        input: TerminateInstanceInAutoScalingGroupType,
    ) -> Result<ActivityType, RusotoError<TerminateInstanceInAutoScalingGroupError>> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "TerminateInstanceInAutoScalingGroup");
        params.put("Version", "2011-01-01");
        TerminateInstanceInAutoScalingGroupTypeSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(TerminateInstanceInAutoScalingGroupError::from_response(
                response,
            ));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = ActivityType::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = ActivityTypeDeserializer::deserialize(
                "TerminateInstanceInAutoScalingGroupResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Updates the configuration for the specified Auto Scaling group.</p> <p>To update an Auto Scaling group, specify the name of the group and the parameter that you want to change. Any parameters that you don't specify are not changed by this update request. The new settings take effect on any scaling activities after this call returns. </p> <p>If you associate a new launch configuration or template with an Auto Scaling group, all new instances will get the updated configuration. Existing instances continue to run with the configuration that they were originally launched with. When you update a group to specify a mixed instances policy instead of a launch configuration or template, existing instances may be replaced to match the new purchasing options that you specified in the policy. For example, if the group currently has 100% On-Demand capacity and the policy specifies 50% Spot capacity, this means that half of your instances will be gradually terminated and relaunched as Spot Instances. When replacing instances, Amazon EC2 Auto Scaling launches new instances before terminating the old ones, so that updating your group does not compromise the performance or availability of your application.</p> <p>Note the following about changing <code>DesiredCapacity</code>, <code>MaxSize</code>, or <code>MinSize</code>:</p> <ul> <li> <p>If a scale-in event occurs as a result of a new <code>DesiredCapacity</code> value that is lower than the current size of the group, the Auto Scaling group uses its termination policy to determine which instances to terminate.</p> </li> <li> <p>If you specify a new value for <code>MinSize</code> without specifying a value for <code>DesiredCapacity</code>, and the new <code>MinSize</code> is larger than the current size of the group, this sets the group's <code>DesiredCapacity</code> to the new <code>MinSize</code> value.</p> </li> <li> <p>If you specify a new value for <code>MaxSize</code> without specifying a value for <code>DesiredCapacity</code>, and the new <code>MaxSize</code> is smaller than the current size of the group, this sets the group's <code>DesiredCapacity</code> to the new <code>MaxSize</code> value.</p> </li> </ul> <p>To see which parameters have been set, use <a>DescribeAutoScalingGroups</a>. You can also view the scaling policies for an Auto Scaling group using <a>DescribePolicies</a>. If the group has scaling policies, you can update them using <a>PutScalingPolicy</a>.</p>
    async fn update_auto_scaling_group(
        &self,
        input: UpdateAutoScalingGroupType,
    ) -> Result<(), RusotoError<UpdateAutoScalingGroupError>> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "UpdateAutoScalingGroup");
        params.put("Version", "2011-01-01");
        UpdateAutoScalingGroupTypeSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(UpdateAutoScalingGroupError::from_response(response));
        }

        Ok(std::mem::drop(response))
    }
}

#[cfg(test)]
mod protocol_tests {

    extern crate rusoto_mock;

    use self::rusoto_mock::*;
    use super::*;
    use rusoto_core::Region as rusoto_region;

    #[tokio::test]
    async fn test_parse_error_autoscaling_delete_policy() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/error",
            "autoscaling-delete-policy.xml",
        );
        let mock = MockRequestDispatcher::with_status(400).with_body(&mock_response);
        let client =
            AutoscalingClient::new_with(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = DeletePolicyType::default();
        let result = client.delete_policy(request).await;
        assert!(!result.is_ok(), "parse error: {:?}", result);
    }

    #[tokio::test]
    async fn test_parse_valid_autoscaling_describe_adjustment_types() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "autoscaling-describe-adjustment-types.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client =
            AutoscalingClient::new_with(mock, MockCredentialsProvider, rusoto_region::UsEast1);

        let result = client.describe_adjustment_types().await;
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[tokio::test]
    async fn test_parse_valid_autoscaling_describe_auto_scaling_groups() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "autoscaling-describe-auto-scaling-groups.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client =
            AutoscalingClient::new_with(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = AutoScalingGroupNamesType::default();
        let result = client.describe_auto_scaling_groups(request).await;
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[tokio::test]
    async fn test_parse_valid_autoscaling_describe_auto_scaling_instances() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "autoscaling-describe-auto-scaling-instances.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client =
            AutoscalingClient::new_with(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = DescribeAutoScalingInstancesType::default();
        let result = client.describe_auto_scaling_instances(request).await;
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[tokio::test]
    async fn test_parse_valid_autoscaling_describe_auto_scaling_notification_types() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "autoscaling-describe-auto-scaling-notification-types.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client =
            AutoscalingClient::new_with(mock, MockCredentialsProvider, rusoto_region::UsEast1);

        let result = client.describe_auto_scaling_notification_types().await;
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[tokio::test]
    async fn test_parse_valid_autoscaling_describe_launch_configurations() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "autoscaling-describe-launch-configurations.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client =
            AutoscalingClient::new_with(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = LaunchConfigurationNamesType::default();
        let result = client.describe_launch_configurations(request).await;
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[tokio::test]
    async fn test_parse_valid_autoscaling_describe_metric_collection_types() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "autoscaling-describe-metric-collection-types.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client =
            AutoscalingClient::new_with(mock, MockCredentialsProvider, rusoto_region::UsEast1);

        let result = client.describe_metric_collection_types().await;
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[tokio::test]
    async fn test_parse_valid_autoscaling_describe_notification_configurations() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "autoscaling-describe-notification-configurations.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client =
            AutoscalingClient::new_with(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = DescribeNotificationConfigurationsType::default();
        let result = client.describe_notification_configurations(request).await;
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[tokio::test]
    async fn test_parse_valid_autoscaling_describe_policies() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "autoscaling-describe-policies.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client =
            AutoscalingClient::new_with(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = DescribePoliciesType::default();
        let result = client.describe_policies(request).await;
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[tokio::test]
    async fn test_parse_valid_autoscaling_describe_scaling_activities() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "autoscaling-describe-scaling-activities.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client =
            AutoscalingClient::new_with(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = DescribeScalingActivitiesType::default();
        let result = client.describe_scaling_activities(request).await;
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[tokio::test]
    async fn test_parse_valid_autoscaling_describe_scaling_process_types() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "autoscaling-describe-scaling-process-types.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client =
            AutoscalingClient::new_with(mock, MockCredentialsProvider, rusoto_region::UsEast1);

        let result = client.describe_scaling_process_types().await;
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[tokio::test]
    async fn test_parse_valid_autoscaling_describe_scheduled_actions() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "autoscaling-describe-scheduled-actions.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client =
            AutoscalingClient::new_with(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = DescribeScheduledActionsType::default();
        let result = client.describe_scheduled_actions(request).await;
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[tokio::test]
    async fn test_parse_valid_autoscaling_describe_tags() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "autoscaling-describe-tags.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client =
            AutoscalingClient::new_with(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = DescribeTagsType::default();
        let result = client.describe_tags(request).await;
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[tokio::test]
    async fn test_parse_valid_autoscaling_describe_termination_policy_types() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "autoscaling-describe-termination-policy-types.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client =
            AutoscalingClient::new_with(mock, MockCredentialsProvider, rusoto_region::UsEast1);

        let result = client.describe_termination_policy_types().await;
        assert!(result.is_ok(), "parse error: {:?}", result);
    }
}
