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

impl AutoscalingClient {
    fn new_params(&self, operation_name: &str) -> Params {
        let mut params = Params::new();

        params.put("Action", operation_name);
        params.put("Version", "2011-01-01");

        params
    }

    async fn sign_and_dispatch<E>(
        &self,
        request: SignedRequest,
        from_response: fn(BufferedHttpResponse) -> RusotoError<E>,
    ) -> Result<HttpResponse, RusotoError<E>> {
        let mut response = self.client.sign_and_dispatch(request).await?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(from_response(response));
        }

        Ok(response)
    }
}

#[allow(dead_code)]
struct ActivitiesDeserializer;
impl ActivitiesDeserializer {
    #[allow(dead_code, unused_variables)]
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
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ActivitiesType {
    /// <p>The scaling activities. Activities are sorted by start time. Activities still in progress are described first.</p>
    pub activities: Vec<Activity>,
    /// <p>A string that indicates that the response contains more items than can be returned in a single response. To receive additional items, specify this string for the <code>NextToken</code> value when requesting the next set of items. This value is null when there are no more items to return.</p>
    pub next_token: Option<String>,
}

#[allow(dead_code)]
struct ActivitiesTypeDeserializer;
impl ActivitiesTypeDeserializer {
    #[allow(dead_code, unused_variables)]
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
#[derive(Clone, Debug, Default, PartialEq)]
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

#[allow(dead_code)]
struct ActivityDeserializer;
impl ActivityDeserializer {
    #[allow(dead_code, unused_variables)]
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

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ActivityType {
    /// <p>A scaling activity.</p>
    pub activity: Option<Activity>,
}

#[allow(dead_code)]
struct ActivityTypeDeserializer;
impl ActivityTypeDeserializer {
    #[allow(dead_code, unused_variables)]
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
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct AdjustmentType {
    /// <p>The policy adjustment type. The valid values are <code>ChangeInCapacity</code>, <code>ExactCapacity</code>, and <code>PercentChangeInCapacity</code>.</p>
    pub adjustment_type: Option<String>,
}

#[allow(dead_code)]
struct AdjustmentTypeDeserializer;
impl AdjustmentTypeDeserializer {
    #[allow(dead_code, unused_variables)]
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
#[allow(dead_code)]
struct AdjustmentTypesDeserializer;
impl AdjustmentTypesDeserializer {
    #[allow(dead_code, unused_variables)]
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
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct Alarm {
    /// <p>The Amazon Resource Name (ARN) of the alarm.</p>
    pub alarm_arn: Option<String>,
    /// <p>The name of the alarm.</p>
    pub alarm_name: Option<String>,
}

#[allow(dead_code)]
struct AlarmDeserializer;
impl AlarmDeserializer {
    #[allow(dead_code, unused_variables)]
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
#[allow(dead_code)]
struct AlarmsDeserializer;
impl AlarmsDeserializer {
    #[allow(dead_code, unused_variables)]
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
#[allow(dead_code)]
struct AsciiStringMaxLen255Deserializer;
impl AsciiStringMaxLen255Deserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}
#[allow(dead_code)]
struct AssociatePublicIpAddressDeserializer;
impl AssociatePublicIpAddressDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<bool, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, |s| {
            Ok(bool::from_str(&s.to_lowercase()).unwrap())
        })
    }
}
#[derive(Clone, Debug, Default, PartialEq)]
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

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct AttachLoadBalancerTargetGroupsResultType {}

#[allow(dead_code)]
struct AttachLoadBalancerTargetGroupsResultTypeDeserializer;
impl AttachLoadBalancerTargetGroupsResultTypeDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AttachLoadBalancerTargetGroupsResultType, XmlParseError> {
        xml_util::start_element(tag_name, stack)?;

        let obj = AttachLoadBalancerTargetGroupsResultType::default();

        xml_util::end_element(tag_name, stack)?;

        Ok(obj)
    }
}
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AttachLoadBalancerTargetGroupsType {
    /// <p>The name of the Auto Scaling group.</p>
    pub auto_scaling_group_name: String,
    /// <p>The Amazon Resource Names (ARN) of the target groups. You can specify up to 10 target groups. To get the ARN of a target group, use the Elastic Load Balancing <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/APIReference/API_DescribeTargetGroups.html">DescribeTargetGroups</a> API operation.</p>
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

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct AttachLoadBalancersResultType {}

#[allow(dead_code)]
struct AttachLoadBalancersResultTypeDeserializer;
impl AttachLoadBalancersResultTypeDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AttachLoadBalancersResultType, XmlParseError> {
        xml_util::start_element(tag_name, stack)?;

        let obj = AttachLoadBalancersResultType::default();

        xml_util::end_element(tag_name, stack)?;

        Ok(obj)
    }
}
#[derive(Clone, Debug, Default, PartialEq)]
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
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct AutoScalingGroup {
    /// <p>The Amazon Resource Name (ARN) of the Auto Scaling group.</p>
    pub auto_scaling_group_arn: Option<String>,
    /// <p>The name of the Auto Scaling group.</p>
    pub auto_scaling_group_name: String,
    /// <p>One or more Availability Zones for the group.</p>
    pub availability_zones: Vec<String>,
    /// <p>Indicates whether Capacity Rebalancing is enabled.</p>
    pub capacity_rebalance: Option<bool>,
    /// <p>The date and time the group was created.</p>
    pub created_time: String,
    /// <p>The duration of the default cooldown period, in seconds.</p>
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
    /// <p>The maximum amount of time, in seconds, that an instance can be in service.</p> <p>Valid Range: Minimum value of 0.</p>
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
    /// <p>The current state of the group when the <a>DeleteAutoScalingGroup</a> operation is in progress.</p>
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

#[allow(dead_code)]
struct AutoScalingGroupDeserializer;
impl AutoScalingGroupDeserializer {
    #[allow(dead_code, unused_variables)]
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
                "CapacityRebalance" => {
                    obj.capacity_rebalance =
                        Some(CapacityRebalanceEnabledDeserializer::deserialize(
                            "CapacityRebalance",
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
#[allow(dead_code)]
struct AutoScalingGroupDesiredCapacityDeserializer;
impl AutoScalingGroupDesiredCapacityDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, |s| Ok(i64::from_str(&s).unwrap()))
    }
}
#[allow(dead_code)]
struct AutoScalingGroupMaxSizeDeserializer;
impl AutoScalingGroupMaxSizeDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, |s| Ok(i64::from_str(&s).unwrap()))
    }
}
#[allow(dead_code)]
struct AutoScalingGroupMinSizeDeserializer;
impl AutoScalingGroupMinSizeDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, |s| Ok(i64::from_str(&s).unwrap()))
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

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AutoScalingGroupNamesType {
    /// <p>The names of the Auto Scaling groups. By default, you can only specify up to 50 names. You can optionally increase this limit using the <code>MaxRecords</code> parameter.</p> <p>If you omit this parameter, all Auto Scaling groups are described.</p>
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

#[allow(dead_code)]
struct AutoScalingGroupsDeserializer;
impl AutoScalingGroupsDeserializer {
    #[allow(dead_code, unused_variables)]
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
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct AutoScalingGroupsType {
    /// <p>The groups.</p>
    pub auto_scaling_groups: Vec<AutoScalingGroup>,
    /// <p>A string that indicates that the response contains more items than can be returned in a single response. To receive additional items, specify this string for the <code>NextToken</code> value when requesting the next set of items. This value is null when there are no more items to return.</p>
    pub next_token: Option<String>,
}

#[allow(dead_code)]
struct AutoScalingGroupsTypeDeserializer;
impl AutoScalingGroupsTypeDeserializer {
    #[allow(dead_code, unused_variables)]
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
#[derive(Clone, Debug, Default, PartialEq)]
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

#[allow(dead_code)]
struct AutoScalingInstanceDetailsDeserializer;
impl AutoScalingInstanceDetailsDeserializer {
    #[allow(dead_code, unused_variables)]
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
#[allow(dead_code)]
struct AutoScalingInstancesDeserializer;
impl AutoScalingInstancesDeserializer {
    #[allow(dead_code, unused_variables)]
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
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct AutoScalingInstancesType {
    /// <p>The instances.</p>
    pub auto_scaling_instances: Option<Vec<AutoScalingInstanceDetails>>,
    /// <p>A string that indicates that the response contains more items than can be returned in a single response. To receive additional items, specify this string for the <code>NextToken</code> value when requesting the next set of items. This value is null when there are no more items to return.</p>
    pub next_token: Option<String>,
}

#[allow(dead_code)]
struct AutoScalingInstancesTypeDeserializer;
impl AutoScalingInstancesTypeDeserializer {
    #[allow(dead_code, unused_variables)]
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
#[allow(dead_code)]
struct AutoScalingNotificationTypesDeserializer;
impl AutoScalingNotificationTypesDeserializer {
    #[allow(dead_code, unused_variables)]
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

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct BatchDeleteScheduledActionAnswer {
    /// <p>The names of the scheduled actions that could not be deleted, including an error message.</p>
    pub failed_scheduled_actions: Option<Vec<FailedScheduledUpdateGroupActionRequest>>,
}

#[allow(dead_code)]
struct BatchDeleteScheduledActionAnswerDeserializer;
impl BatchDeleteScheduledActionAnswerDeserializer {
    #[allow(dead_code, unused_variables)]
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
#[derive(Clone, Debug, Default, PartialEq)]
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

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct BatchPutScheduledUpdateGroupActionAnswer {
    /// <p>The names of the scheduled actions that could not be created or updated, including an error message.</p>
    pub failed_scheduled_update_group_actions: Option<Vec<FailedScheduledUpdateGroupActionRequest>>,
}

#[allow(dead_code)]
struct BatchPutScheduledUpdateGroupActionAnswerDeserializer;
impl BatchPutScheduledUpdateGroupActionAnswerDeserializer {
    #[allow(dead_code, unused_variables)]
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
#[derive(Clone, Debug, Default, PartialEq)]
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

#[allow(dead_code)]
struct BlockDeviceEbsDeleteOnTerminationDeserializer;
impl BlockDeviceEbsDeleteOnTerminationDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<bool, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, |s| {
            Ok(bool::from_str(&s.to_lowercase()).unwrap())
        })
    }
}
#[allow(dead_code)]
struct BlockDeviceEbsEncryptedDeserializer;
impl BlockDeviceEbsEncryptedDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<bool, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, |s| {
            Ok(bool::from_str(&s.to_lowercase()).unwrap())
        })
    }
}
#[allow(dead_code)]
struct BlockDeviceEbsIopsDeserializer;
impl BlockDeviceEbsIopsDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, |s| Ok(i64::from_str(&s).unwrap()))
    }
}
#[allow(dead_code)]
struct BlockDeviceEbsVolumeSizeDeserializer;
impl BlockDeviceEbsVolumeSizeDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, |s| Ok(i64::from_str(&s).unwrap()))
    }
}
#[allow(dead_code)]
struct BlockDeviceEbsVolumeTypeDeserializer;
impl BlockDeviceEbsVolumeTypeDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}
/// <p>Describes a block device mapping.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BlockDeviceMapping {
    /// <p>The device name exposed to the EC2 instance (for example, <code>/dev/sdh</code> or <code>xvdh</code>). For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/device_naming.html">Device Naming on Linux Instances</a> in the <i>Amazon EC2 User Guide for Linux Instances</i>.</p>
    pub device_name: String,
    /// <p>Parameters used to automatically set up EBS volumes when an instance is launched.</p> <p>You can specify either <code>VirtualName</code> or <code>Ebs</code>, but not both.</p>
    pub ebs: Option<Ebs>,
    /// <p>Setting this value to <code>true</code> suppresses the specified device included in the block device mapping of the AMI.</p> <p>If <code>NoDevice</code> is <code>true</code> for the root device, instances might fail the EC2 health check. In that case, Amazon EC2 Auto Scaling launches replacement instances.</p> <p>If you specify <code>NoDevice</code>, you cannot specify <code>Ebs</code>.</p>
    pub no_device: Option<bool>,
    /// <p>The name of the virtual device (for example, <code>ephemeral0</code>).</p> <p>You can specify either <code>VirtualName</code> or <code>Ebs</code>, but not both.</p>
    pub virtual_name: Option<String>,
}

#[allow(dead_code)]
struct BlockDeviceMappingDeserializer;
impl BlockDeviceMappingDeserializer {
    #[allow(dead_code, unused_variables)]
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

#[allow(dead_code)]
struct BlockDeviceMappingsDeserializer;
impl BlockDeviceMappingsDeserializer {
    #[allow(dead_code, unused_variables)]
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

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CancelInstanceRefreshAnswer {
    /// <p>The instance refresh ID.</p>
    pub instance_refresh_id: Option<String>,
}

#[allow(dead_code)]
struct CancelInstanceRefreshAnswerDeserializer;
impl CancelInstanceRefreshAnswerDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CancelInstanceRefreshAnswer, XmlParseError> {
        deserialize_elements::<_, CancelInstanceRefreshAnswer, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "InstanceRefreshId" => {
                        obj.instance_refresh_id =
                            Some(XmlStringMaxLen255Deserializer::deserialize(
                                "InstanceRefreshId",
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
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CancelInstanceRefreshType {
    /// <p>The name of the Auto Scaling group.</p>
    pub auto_scaling_group_name: String,
}

/// Serialize `CancelInstanceRefreshType` contents to a `SignedRequest`.
struct CancelInstanceRefreshTypeSerializer;
impl CancelInstanceRefreshTypeSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CancelInstanceRefreshType) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "AutoScalingGroupName"),
            &obj.auto_scaling_group_name,
        );
    }
}

#[allow(dead_code)]
struct CapacityRebalanceEnabledDeserializer;
impl CapacityRebalanceEnabledDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<bool, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, |s| {
            Ok(bool::from_str(&s.to_lowercase()).unwrap())
        })
    }
}
#[allow(dead_code)]
struct ClassicLinkVPCSecurityGroupsDeserializer;
impl ClassicLinkVPCSecurityGroupsDeserializer {
    #[allow(dead_code, unused_variables)]
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

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CompleteLifecycleActionAnswer {}

#[allow(dead_code)]
struct CompleteLifecycleActionAnswerDeserializer;
impl CompleteLifecycleActionAnswerDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CompleteLifecycleActionAnswer, XmlParseError> {
        xml_util::start_element(tag_name, stack)?;

        let obj = CompleteLifecycleActionAnswer::default();

        xml_util::end_element(tag_name, stack)?;

        Ok(obj)
    }
}
#[derive(Clone, Debug, Default, PartialEq)]
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

#[allow(dead_code)]
struct CooldownDeserializer;
impl CooldownDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, |s| Ok(i64::from_str(&s).unwrap()))
    }
}
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateAutoScalingGroupType {
    /// <p>The name of the Auto Scaling group. This name must be unique per Region per account.</p>
    pub auto_scaling_group_name: String,
    /// <p>A list of Availability Zones where instances in the Auto Scaling group can be created. This parameter is optional if you specify one or more subnets for <code>VPCZoneIdentifier</code>.</p> <p>Conditional: If your account supports EC2-Classic and VPC, this parameter is required to launch instances into EC2-Classic.</p>
    pub availability_zones: Option<Vec<String>>,
    /// <p>Indicates whether Capacity Rebalancing is enabled. Otherwise, Capacity Rebalancing is disabled. When you turn on Capacity Rebalancing, Amazon EC2 Auto Scaling attempts to launch a Spot Instance whenever Amazon EC2 notifies that a Spot Instance is at an elevated risk of interruption. After launching a new instance, it then terminates an old instance. For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/capacity-rebalance.html">Amazon EC2 Auto Scaling Capacity Rebalancing</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    pub capacity_rebalance: Option<bool>,
    /// <p>The amount of time, in seconds, after a scaling activity completes before another scaling activity can start. The default value is <code>300</code>. This setting applies when using simple scaling policies, but not when using other scaling policies or scheduled scaling. For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/Cooldown.html">Scaling cooldowns for Amazon EC2 Auto Scaling</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    pub default_cooldown: Option<i64>,
    /// <p>The desired capacity is the initial capacity of the Auto Scaling group at the time of its creation and the capacity it attempts to maintain. It can scale beyond this capacity if you configure auto scaling. This number must be greater than or equal to the minimum size of the group and less than or equal to the maximum size of the group. If you do not specify a desired capacity, the default is the minimum size of the group.</p>
    pub desired_capacity: Option<i64>,
    /// <p>The amount of time, in seconds, that Amazon EC2 Auto Scaling waits before checking the health status of an EC2 instance that has come into service. During this time, any health check failures for the instance are ignored. The default value is <code>0</code>. For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/healthcheck.html#health-check-grace-period">Health check grace period</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p> <p>Conditional: Required if you are adding an <code>ELB</code> health check.</p>
    pub health_check_grace_period: Option<i64>,
    /// <p>The service to use for the health checks. The valid values are <code>EC2</code> (default) and <code>ELB</code>. If you configure an Auto Scaling group to use load balancer (ELB) health checks, it considers the instance unhealthy if it fails either the EC2 status checks or the load balancer health checks. For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/healthcheck.html">Health checks for Auto Scaling instances</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    pub health_check_type: Option<String>,
    /// <p>The ID of the instance used to base the launch configuration on. If specified, Amazon EC2 Auto Scaling uses the configuration values from the specified instance to create a new launch configuration. To get the instance ID, use the Amazon EC2 <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_DescribeInstances.html">DescribeInstances</a> API operation. For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/create-asg-from-instance.html">Creating an Auto Scaling group using an EC2 instance</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    pub instance_id: Option<String>,
    /// <p>The name of the launch configuration to use to launch instances. </p> <p>Conditional: You must specify either a launch template (<code>LaunchTemplate</code> or <code>MixedInstancesPolicy</code>) or a launch configuration (<code>LaunchConfigurationName</code> or <code>InstanceId</code>).</p>
    pub launch_configuration_name: Option<String>,
    /// <p><p>Parameters used to specify the <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-launchtemplate.html">launch template</a> and version to use to launch instances. </p> <p>Conditional: You must specify either a launch template (<code>LaunchTemplate</code> or <code>MixedInstancesPolicy</code>) or a launch configuration (<code>LaunchConfigurationName</code> or <code>InstanceId</code>).</p> <note> <p>The launch template that is specified must be configured for use with an Auto Scaling group. For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/create-launch-template.html">Creating a launch template for an Auto Scaling group</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p> </note></p>
    pub launch_template: Option<LaunchTemplateSpecification>,
    /// <p>One or more lifecycle hooks for the group, which specify actions to perform when Amazon EC2 Auto Scaling launches or terminates instances.</p>
    pub lifecycle_hook_specification_list: Option<Vec<LifecycleHookSpecification>>,
    /// <p>A list of Classic Load Balancers associated with this Auto Scaling group. For Application Load Balancers, Network Load Balancers, and Gateway Load Balancers, specify the <code>TargetGroupARNs</code> property instead.</p>
    pub load_balancer_names: Option<Vec<String>>,
    /// <p>The maximum amount of time, in seconds, that an instance can be in service. The default is null. If specified, the value must be either 0 or a number equal to or greater than 86,400 seconds (1 day). For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/asg-max-instance-lifetime.html">Replacing Auto Scaling instances based on maximum instance lifetime</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    pub max_instance_lifetime: Option<i64>,
    /// <p><p>The maximum size of the group.</p> <note> <p>With a mixed instances policy that uses instance weighting, Amazon EC2 Auto Scaling may need to go above <code>MaxSize</code> to meet your capacity requirements. In this event, Amazon EC2 Auto Scaling will never go above <code>MaxSize</code> by more than your largest instance weight (weights that define how many units each instance contributes to the desired capacity of the group).</p> </note></p>
    pub max_size: i64,
    /// <p>The minimum size of the group.</p>
    pub min_size: i64,
    /// <p>An embedded object that specifies a mixed instances policy. The required parameters must be specified. If optional parameters are unspecified, their default values are used.</p> <p>The policy includes parameters that not only define the distribution of On-Demand Instances and Spot Instances, the maximum price to pay for Spot Instances, and how the Auto Scaling group allocates instance types to fulfill On-Demand and Spot capacities, but also the parameters that specify the instance configuration information—the launch template and instance types. The policy can also include a weight for each instance type and different launch templates for individual instance types. For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/asg-purchase-options.html">Auto Scaling groups with multiple instance types and purchase options</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    pub mixed_instances_policy: Option<MixedInstancesPolicy>,
    /// <p>Indicates whether newly launched instances are protected from termination by Amazon EC2 Auto Scaling when scaling in. For more information about preventing instances from terminating on scale in, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/as-instance-termination.html#instance-protection">Instance scale-in protection</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    pub new_instances_protected_from_scale_in: Option<bool>,
    /// <p>The name of an existing placement group into which to launch your instances, if any. A placement group is a logical grouping of instances within a single Availability Zone. You cannot specify multiple Availability Zones and a placement group. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/placement-groups.html">Placement Groups</a> in the <i>Amazon EC2 User Guide for Linux Instances</i>.</p>
    pub placement_group: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the service-linked role that the Auto Scaling group uses to call other AWS services on your behalf. By default, Amazon EC2 Auto Scaling uses a service-linked role named AWSServiceRoleForAutoScaling, which it creates if it does not exist. For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/autoscaling-service-linked-role.html">Service-linked roles</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    pub service_linked_role_arn: Option<String>,
    /// <p>One or more tags. You can tag your Auto Scaling group and propagate the tags to the Amazon EC2 instances it launches. Tags are not propagated to Amazon EBS volumes. To add tags to Amazon EBS volumes, specify the tags in a launch template but use caution. If the launch template specifies an instance tag with a key that is also specified for the Auto Scaling group, Amazon EC2 Auto Scaling overrides the value of that instance tag with the value specified by the Auto Scaling group. For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/autoscaling-tagging.html">Tagging Auto Scaling groups and instances</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    pub tags: Option<Vec<Tag>>,
    /// <p>The Amazon Resource Names (ARN) of the target groups to associate with the Auto Scaling group. Instances are registered as targets in a target group, and traffic is routed to the target group. For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/autoscaling-load-balancer.html">Elastic Load Balancing and Amazon EC2 Auto Scaling</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    pub target_group_ar_ns: Option<Vec<String>>,
    /// <p>A policy or a list of policies that are used to select the instance to terminate. These policies are executed in the order that you list them. For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/as-instance-termination.html">Controlling which Auto Scaling instances terminate during scale in</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    pub termination_policies: Option<Vec<String>>,
    /// <p>A comma-separated list of subnet IDs for a virtual private cloud (VPC) where instances in the Auto Scaling group can be created. If you specify <code>VPCZoneIdentifier</code> with <code>AvailabilityZones</code>, the subnets that you specify for this parameter must reside in those Availability Zones.</p> <p>Conditional: If your account supports EC2-Classic and VPC, this parameter is required to launch instances into a VPC.</p>
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
        if let Some(ref field_value) = obj.capacity_rebalance {
            params.put(&format!("{}{}", prefix, "CapacityRebalance"), &field_value);
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

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateLaunchConfigurationType {
    /// <p><p>For Auto Scaling groups that are running in a virtual private cloud (VPC), specifies whether to assign a public IP address to the group&#39;s instances. If you specify <code>true</code>, each instance in the Auto Scaling group receives a unique public IP address. For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/asg-in-vpc.html">Launching Auto Scaling instances in a VPC</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p> <p>If you specify this parameter, you must specify at least one subnet for <code>VPCZoneIdentifier</code> when you create your group.</p> <note> <p>If the instance is launched into a default subnet, the default is to assign a public IP address, unless you disabled the option to assign a public IP address on the subnet. If the instance is launched into a nondefault subnet, the default is not to assign a public IP address, unless you enabled the option to assign a public IP address on the subnet.</p> </note></p>
    pub associate_public_ip_address: Option<bool>,
    /// <p>A block device mapping, which specifies the block devices for the instance. You can specify virtual devices and EBS volumes. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/block-device-mapping-concepts.html">Block Device Mapping</a> in the <i>Amazon EC2 User Guide for Linux Instances</i>.</p>
    pub block_device_mappings: Option<Vec<BlockDeviceMapping>>,
    /// <p>The ID of a ClassicLink-enabled VPC to link your EC2-Classic instances to. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/vpc-classiclink.html">ClassicLink</a> in the <i>Amazon EC2 User Guide for Linux Instances</i> and <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/asg-in-vpc.html#as-ClassicLink">Linking EC2-Classic instances to a VPC</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p> <p>This parameter can only be used if you are launching EC2-Classic instances.</p>
    pub classic_link_vpc_id: Option<String>,
    /// <p>The IDs of one or more security groups for the specified ClassicLink-enabled VPC. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/vpc-classiclink.html">ClassicLink</a> in the <i>Amazon EC2 User Guide for Linux Instances</i> and <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/asg-in-vpc.html#as-ClassicLink">Linking EC2-Classic instances to a VPC</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p> <p>If you specify the <code>ClassicLinkVPCId</code> parameter, you must specify this parameter.</p>
    pub classic_link_vpc_security_groups: Option<Vec<String>>,
    /// <p>Specifies whether the launch configuration is optimized for EBS I/O (<code>true</code>) or not (<code>false</code>). The optimization provides dedicated throughput to Amazon EBS and an optimized configuration stack to provide optimal I/O performance. This optimization is not available with all instance types. Additional fees are incurred when you enable EBS optimization for an instance type that is not EBS-optimized by default. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/EBSOptimized.html">Amazon EBS-Optimized Instances</a> in the <i>Amazon EC2 User Guide for Linux Instances</i>.</p> <p>The default value is <code>false</code>.</p>
    pub ebs_optimized: Option<bool>,
    /// <p>The name or the Amazon Resource Name (ARN) of the instance profile associated with the IAM role for the instance. The instance profile contains the IAM role.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/us-iam-role.html">IAM role for applications that run on Amazon EC2 instances</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    pub iam_instance_profile: Option<String>,
    /// <p>The ID of the Amazon Machine Image (AMI) that was assigned during registration. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/finding-an-ami.html">Finding an AMI</a> in the <i>Amazon EC2 User Guide for Linux Instances</i>.</p> <p>If you do not specify <code>InstanceId</code>, you must specify <code>ImageId</code>.</p>
    pub image_id: Option<String>,
    /// <p>The ID of the instance to use to create the launch configuration. The new launch configuration derives attributes from the instance, except for the block device mapping.</p> <p>To create a launch configuration with a block device mapping or override any other instance attributes, specify them as part of the same request.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/create-lc-with-instanceID.html">Creating a launch configuration using an EC2 instance</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p> <p>If you do not specify <code>InstanceId</code>, you must specify both <code>ImageId</code> and <code>InstanceType</code>.</p>
    pub instance_id: Option<String>,
    /// <p><p>Controls whether instances in this group are launched with detailed (<code>true</code>) or basic (<code>false</code>) monitoring.</p> <p>The default value is <code>true</code> (enabled).</p> <important> <p>When detailed monitoring is enabled, Amazon CloudWatch generates metrics every minute and your account is charged a fee. When you disable detailed monitoring, CloudWatch generates metrics every 5 minutes. For more information, see <a href="https://docs.aws.amazon.com/autoscaling/latest/userguide/enable-as-instance-metrics.html">Configure Monitoring for Auto Scaling Instances</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p> </important></p>
    pub instance_monitoring: Option<InstanceMonitoring>,
    /// <p>Specifies the instance type of the EC2 instance.</p> <p>For information about available instance types, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/instance-types.html#AvailableInstanceTypes">Available Instance Types</a> in the <i>Amazon EC2 User Guide for Linux Instances.</i> </p> <p>If you do not specify <code>InstanceId</code>, you must specify <code>InstanceType</code>.</p>
    pub instance_type: Option<String>,
    /// <p>The ID of the kernel associated with the AMI.</p>
    pub kernel_id: Option<String>,
    /// <p>The name of the key pair. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-key-pairs.html">Amazon EC2 Key Pairs</a> in the <i>Amazon EC2 User Guide for Linux Instances</i>.</p>
    pub key_name: Option<String>,
    /// <p>The name of the launch configuration. This name must be unique per Region per account.</p>
    pub launch_configuration_name: String,
    /// <p>The metadata options for the instances. For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/create-launch-config.html#launch-configurations-imds">Configuring the Instance Metadata Options</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    pub metadata_options: Option<InstanceMetadataOptions>,
    /// <p>The tenancy of the instance. An instance with <code>dedicated</code> tenancy runs on isolated, single-tenant hardware and can only be launched into a VPC.</p> <p>To launch dedicated instances into a shared tenancy VPC (a VPC with the instance placement tenancy attribute set to <code>default</code>), you must set the value of this parameter to <code>dedicated</code>.</p> <p>If you specify <code>PlacementTenancy</code>, you must specify at least one subnet for <code>VPCZoneIdentifier</code> when you create your group.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/auto-scaling-dedicated-instances.html">Configuring instance tenancy with Amazon EC2 Auto Scaling</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p> <p>Valid Values: <code>default</code> | <code>dedicated</code> </p>
    pub placement_tenancy: Option<String>,
    /// <p>The ID of the RAM disk to select.</p>
    pub ramdisk_id: Option<String>,
    /// <p>A list that contains the security groups to assign to the instances in the Auto Scaling group.</p> <p>[EC2-VPC] Specify the security group IDs. For more information, see <a href="https://docs.aws.amazon.com/AmazonVPC/latest/UserGuide/VPC_SecurityGroups.html">Security Groups for Your VPC</a> in the <i>Amazon Virtual Private Cloud User Guide</i>.</p> <p>[EC2-Classic] Specify either the security group names or the security group IDs. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/using-network-security.html">Amazon EC2 Security Groups</a> in the <i>Amazon EC2 User Guide for Linux Instances</i>.</p>
    pub security_groups: Option<Vec<String>>,
    /// <p><p>The maximum hourly price to be paid for any Spot Instance launched to fulfill the request. Spot Instances are launched when the price you specify exceeds the current Spot price. For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/asg-launch-spot-instances.html">Requesting Spot Instances</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p> <note> <p>When you change your maximum price by creating a new launch configuration, running instances will continue to run as long as the maximum price for those running instances is higher than the current Spot price.</p> </note></p>
    pub spot_price: Option<String>,
    /// <p>The Base64-encoded user data to make available to the launched EC2 instances. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-instance-metadata.html">Instance metadata and user data</a> in the <i>Amazon EC2 User Guide for Linux Instances</i>.</p>
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
        if let Some(ref field_value) = obj.metadata_options {
            InstanceMetadataOptionsSerializer::serialize(
                params,
                &format!("{}{}", prefix, "MetadataOptions"),
                field_value,
            );
        }
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

#[derive(Clone, Debug, Default, PartialEq)]
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
#[derive(Clone, Debug, Default, PartialEq)]
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

#[allow(dead_code)]
struct CustomizedMetricSpecificationDeserializer;
impl CustomizedMetricSpecificationDeserializer {
    #[allow(dead_code, unused_variables)]
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

#[derive(Clone, Debug, Default, PartialEq)]
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

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DeleteLifecycleHookAnswer {}

#[allow(dead_code)]
struct DeleteLifecycleHookAnswerDeserializer;
impl DeleteLifecycleHookAnswerDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteLifecycleHookAnswer, XmlParseError> {
        xml_util::start_element(tag_name, stack)?;

        let obj = DeleteLifecycleHookAnswer::default();

        xml_util::end_element(tag_name, stack)?;

        Ok(obj)
    }
}
#[derive(Clone, Debug, Default, PartialEq)]
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

#[derive(Clone, Debug, Default, PartialEq)]
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

#[derive(Clone, Debug, Default, PartialEq)]
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

#[derive(Clone, Debug, Default, PartialEq)]
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

#[derive(Clone, Debug, Default, PartialEq)]
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

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DescribeAccountLimitsAnswer {
    /// <p>The maximum number of groups allowed for your AWS account. The default is 200 groups per AWS Region.</p>
    pub max_number_of_auto_scaling_groups: Option<i64>,
    /// <p>The maximum number of launch configurations allowed for your AWS account. The default is 200 launch configurations per AWS Region.</p>
    pub max_number_of_launch_configurations: Option<i64>,
    /// <p>The current number of groups for your AWS account.</p>
    pub number_of_auto_scaling_groups: Option<i64>,
    /// <p>The current number of launch configurations for your AWS account.</p>
    pub number_of_launch_configurations: Option<i64>,
}

#[allow(dead_code)]
struct DescribeAccountLimitsAnswerDeserializer;
impl DescribeAccountLimitsAnswerDeserializer {
    #[allow(dead_code, unused_variables)]
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
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DescribeAdjustmentTypesAnswer {
    /// <p>The policy adjustment types.</p>
    pub adjustment_types: Option<Vec<AdjustmentType>>,
}

#[allow(dead_code)]
struct DescribeAdjustmentTypesAnswerDeserializer;
impl DescribeAdjustmentTypesAnswerDeserializer {
    #[allow(dead_code, unused_variables)]
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
#[derive(Clone, Debug, Default, PartialEq)]
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

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DescribeAutoScalingNotificationTypesAnswer {
    /// <p>The notification types.</p>
    pub auto_scaling_notification_types: Option<Vec<String>>,
}

#[allow(dead_code)]
struct DescribeAutoScalingNotificationTypesAnswerDeserializer;
impl DescribeAutoScalingNotificationTypesAnswerDeserializer {
    #[allow(dead_code, unused_variables)]
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
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DescribeInstanceRefreshesAnswer {
    /// <p>The instance refreshes for the specified group.</p>
    pub instance_refreshes: Option<Vec<InstanceRefresh>>,
    /// <p>A string that indicates that the response contains more items than can be returned in a single response. To receive additional items, specify this string for the <code>NextToken</code> value when requesting the next set of items. This value is null when there are no more items to return.</p>
    pub next_token: Option<String>,
}

#[allow(dead_code)]
struct DescribeInstanceRefreshesAnswerDeserializer;
impl DescribeInstanceRefreshesAnswerDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeInstanceRefreshesAnswer, XmlParseError> {
        deserialize_elements::<_, DescribeInstanceRefreshesAnswer, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "InstanceRefreshes" => {
                        obj.instance_refreshes.get_or_insert(vec![]).extend(
                            InstanceRefreshesDeserializer::deserialize("InstanceRefreshes", stack)?,
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
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeInstanceRefreshesType {
    /// <p>The name of the Auto Scaling group.</p>
    pub auto_scaling_group_name: String,
    /// <p>One or more instance refresh IDs.</p>
    pub instance_refresh_ids: Option<Vec<String>>,
    /// <p>The maximum number of items to return with this call. The default value is <code>50</code> and the maximum value is <code>100</code>.</p>
    pub max_records: Option<i64>,
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    pub next_token: Option<String>,
}

/// Serialize `DescribeInstanceRefreshesType` contents to a `SignedRequest`.
struct DescribeInstanceRefreshesTypeSerializer;
impl DescribeInstanceRefreshesTypeSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeInstanceRefreshesType) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "AutoScalingGroupName"),
            &obj.auto_scaling_group_name,
        );
        if let Some(ref field_value) = obj.instance_refresh_ids {
            InstanceRefreshIdsSerializer::serialize(
                params,
                &format!("{}{}", prefix, "InstanceRefreshIds"),
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

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DescribeLifecycleHookTypesAnswer {
    /// <p>The lifecycle hook types.</p>
    pub lifecycle_hook_types: Option<Vec<String>>,
}

#[allow(dead_code)]
struct DescribeLifecycleHookTypesAnswerDeserializer;
impl DescribeLifecycleHookTypesAnswerDeserializer {
    #[allow(dead_code, unused_variables)]
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
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DescribeLifecycleHooksAnswer {
    /// <p>The lifecycle hooks for the specified group.</p>
    pub lifecycle_hooks: Option<Vec<LifecycleHook>>,
}

#[allow(dead_code)]
struct DescribeLifecycleHooksAnswerDeserializer;
impl DescribeLifecycleHooksAnswerDeserializer {
    #[allow(dead_code, unused_variables)]
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
#[derive(Clone, Debug, Default, PartialEq)]
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

#[derive(Clone, Debug, Default, PartialEq)]
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

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DescribeLoadBalancerTargetGroupsResponse {
    /// <p>Information about the target groups.</p>
    pub load_balancer_target_groups: Option<Vec<LoadBalancerTargetGroupState>>,
    /// <p>A string that indicates that the response contains more items than can be returned in a single response. To receive additional items, specify this string for the <code>NextToken</code> value when requesting the next set of items. This value is null when there are no more items to return.</p>
    pub next_token: Option<String>,
}

#[allow(dead_code)]
struct DescribeLoadBalancerTargetGroupsResponseDeserializer;
impl DescribeLoadBalancerTargetGroupsResponseDeserializer {
    #[allow(dead_code, unused_variables)]
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
#[derive(Clone, Debug, Default, PartialEq)]
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

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DescribeLoadBalancersResponse {
    /// <p>The load balancers.</p>
    pub load_balancers: Option<Vec<LoadBalancerState>>,
    /// <p>A string that indicates that the response contains more items than can be returned in a single response. To receive additional items, specify this string for the <code>NextToken</code> value when requesting the next set of items. This value is null when there are no more items to return.</p>
    pub next_token: Option<String>,
}

#[allow(dead_code)]
struct DescribeLoadBalancersResponseDeserializer;
impl DescribeLoadBalancersResponseDeserializer {
    #[allow(dead_code, unused_variables)]
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
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DescribeMetricCollectionTypesAnswer {
    /// <p>The granularities for the metrics.</p>
    pub granularities: Option<Vec<MetricGranularityType>>,
    /// <p>One or more metrics.</p>
    pub metrics: Option<Vec<MetricCollectionType>>,
}

#[allow(dead_code)]
struct DescribeMetricCollectionTypesAnswerDeserializer;
impl DescribeMetricCollectionTypesAnswerDeserializer {
    #[allow(dead_code, unused_variables)]
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
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DescribeNotificationConfigurationsAnswer {
    /// <p>A string that indicates that the response contains more items than can be returned in a single response. To receive additional items, specify this string for the <code>NextToken</code> value when requesting the next set of items. This value is null when there are no more items to return.</p>
    pub next_token: Option<String>,
    /// <p>The notification configurations.</p>
    pub notification_configurations: Vec<NotificationConfiguration>,
}

#[allow(dead_code)]
struct DescribeNotificationConfigurationsAnswerDeserializer;
impl DescribeNotificationConfigurationsAnswerDeserializer {
    #[allow(dead_code, unused_variables)]
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
#[derive(Clone, Debug, Default, PartialEq)]
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

#[derive(Clone, Debug, Default, PartialEq)]
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

#[derive(Clone, Debug, Default, PartialEq)]
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

#[derive(Clone, Debug, Default, PartialEq)]
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

#[derive(Clone, Debug, Default, PartialEq)]
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

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DescribeTerminationPolicyTypesAnswer {
    /// <p>The termination policies supported by Amazon EC2 Auto Scaling: <code>OldestInstance</code>, <code>OldestLaunchConfiguration</code>, <code>NewestInstance</code>, <code>ClosestToNextInstanceHour</code>, <code>Default</code>, <code>OldestLaunchTemplate</code>, and <code>AllocationStrategy</code>.</p>
    pub termination_policy_types: Option<Vec<String>>,
}

#[allow(dead_code)]
struct DescribeTerminationPolicyTypesAnswerDeserializer;
impl DescribeTerminationPolicyTypesAnswerDeserializer {
    #[allow(dead_code, unused_variables)]
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
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DetachInstancesAnswer {
    /// <p>The activities related to detaching the instances from the Auto Scaling group.</p>
    pub activities: Option<Vec<Activity>>,
}

#[allow(dead_code)]
struct DetachInstancesAnswerDeserializer;
impl DetachInstancesAnswerDeserializer {
    #[allow(dead_code, unused_variables)]
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
#[derive(Clone, Debug, Default, PartialEq)]
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

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DetachLoadBalancerTargetGroupsResultType {}

#[allow(dead_code)]
struct DetachLoadBalancerTargetGroupsResultTypeDeserializer;
impl DetachLoadBalancerTargetGroupsResultTypeDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DetachLoadBalancerTargetGroupsResultType, XmlParseError> {
        xml_util::start_element(tag_name, stack)?;

        let obj = DetachLoadBalancerTargetGroupsResultType::default();

        xml_util::end_element(tag_name, stack)?;

        Ok(obj)
    }
}
#[derive(Clone, Debug, Default, PartialEq)]
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

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DetachLoadBalancersResultType {}

#[allow(dead_code)]
struct DetachLoadBalancersResultTypeDeserializer;
impl DetachLoadBalancersResultTypeDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DetachLoadBalancersResultType, XmlParseError> {
        xml_util::start_element(tag_name, stack)?;

        let obj = DetachLoadBalancersResultType::default();

        xml_util::end_element(tag_name, stack)?;

        Ok(obj)
    }
}
#[derive(Clone, Debug, Default, PartialEq)]
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

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DisableMetricsCollectionQuery {
    /// <p>The name of the Auto Scaling group.</p>
    pub auto_scaling_group_name: String,
    /// <p>Specifies one or more of the following metrics:</p> <ul> <li> <p> <code>GroupMinSize</code> </p> </li> <li> <p> <code>GroupMaxSize</code> </p> </li> <li> <p> <code>GroupDesiredCapacity</code> </p> </li> <li> <p> <code>GroupInServiceInstances</code> </p> </li> <li> <p> <code>GroupPendingInstances</code> </p> </li> <li> <p> <code>GroupStandbyInstances</code> </p> </li> <li> <p> <code>GroupTerminatingInstances</code> </p> </li> <li> <p> <code>GroupTotalInstances</code> </p> </li> <li> <p> <code>GroupInServiceCapacity</code> </p> </li> <li> <p> <code>GroupPendingCapacity</code> </p> </li> <li> <p> <code>GroupStandbyCapacity</code> </p> </li> <li> <p> <code>GroupTerminatingCapacity</code> </p> </li> <li> <p> <code>GroupTotalCapacity</code> </p> </li> </ul> <p>If you omit this parameter, all metrics are disabled. </p>
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

#[allow(dead_code)]
struct DisableScaleInDeserializer;
impl DisableScaleInDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<bool, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, |s| {
            Ok(bool::from_str(&s.to_lowercase()).unwrap())
        })
    }
}
/// <p>Describes information used to set up an Amazon EBS volume specified in a block device mapping.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Ebs {
    /// <p>Indicates whether the volume is deleted on instance termination. For Amazon EC2 Auto Scaling, the default value is <code>true</code>.</p>
    pub delete_on_termination: Option<bool>,
    /// <p>Specifies whether the volume should be encrypted. Encrypted EBS volumes can only be attached to instances that support Amazon EBS encryption. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/EBSEncryption.html#EBSEncryption_supported_instances">Supported Instance Types</a>. If your AMI uses encrypted volumes, you can also only launch it on supported instance types.</p> <note> <p>If you are creating a volume from a snapshot, you cannot specify an encryption value. Volumes that are created from encrypted snapshots are automatically encrypted, and volumes that are created from unencrypted snapshots are automatically unencrypted. By default, encrypted snapshots use the AWS managed CMK that is used for EBS encryption, but you can specify a custom CMK when you create the snapshot. The ability to encrypt a snapshot during copying also allows you to apply a new CMK to an already-encrypted snapshot. Volumes restored from the resulting copy are only accessible using the new CMK.</p> <p>Enabling <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/EBSEncryption.html#encryption-by-default">encryption by default</a> results in all EBS volumes being encrypted with the AWS managed CMK or a customer managed CMK, whether or not the snapshot was encrypted.</p> </note> <p>For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/AMIEncryption.html">Using Encryption with EBS-Backed AMIs</a> in the <i>Amazon EC2 User Guide for Linux Instances</i> and <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/key-policy-requirements-EBS-encryption.html">Required CMK key policy for use with encrypted volumes</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    pub encrypted: Option<bool>,
    /// <p>The number of I/O operations per second (IOPS) to provision for the volume. The maximum ratio of IOPS to volume size (in GiB) is 50:1. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/EBSVolumeTypes.html">Amazon EBS Volume Types</a> in the <i>Amazon EC2 User Guide for Linux Instances</i>.</p> <p>Required when the volume type is <code>io1</code>. (Not used with <code>standard</code>, <code>gp2</code>, <code>st1</code>, or <code>sc1</code> volumes.) </p>
    pub iops: Option<i64>,
    /// <p>The snapshot ID of the volume to use.</p> <p>You must specify either a <code>VolumeSize</code> or a <code>SnapshotId</code>.</p>
    pub snapshot_id: Option<String>,
    /// <p>The volume size, in Gibibytes (GiB).</p> <p>This can be a number from 1-1,024 for <code>standard</code>, 4-16,384 for <code>io1</code>, 1-16,384 for <code>gp2</code>, and 500-16,384 for <code>st1</code> and <code>sc1</code>. If you specify a snapshot, the volume size must be equal to or larger than the snapshot size.</p> <p>Default: If you create a volume from a snapshot and you don't specify a volume size, the default is the snapshot size.</p> <p>You must specify either a <code>VolumeSize</code> or a <code>SnapshotId</code>. If you specify both <code>SnapshotId</code> and <code>VolumeSize</code>, the volume size must be equal or greater than the size of the snapshot.</p>
    pub volume_size: Option<i64>,
    /// <p>The volume type, which can be <code>standard</code> for Magnetic, <code>io1</code> for Provisioned IOPS SSD, <code>gp2</code> for General Purpose SSD, <code>st1</code> for Throughput Optimized HDD, or <code>sc1</code> for Cold HDD. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/EBSVolumeTypes.html">Amazon EBS Volume Types</a> in the <i>Amazon EC2 User Guide for Linux Instances</i>.</p> <p>Valid Values: <code>standard</code> | <code>io1</code> | <code>gp2</code> | <code>st1</code> | <code>sc1</code> </p>
    pub volume_type: Option<String>,
}

#[allow(dead_code)]
struct EbsDeserializer;
impl EbsDeserializer {
    #[allow(dead_code, unused_variables)]
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

#[allow(dead_code)]
struct EbsOptimizedDeserializer;
impl EbsOptimizedDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<bool, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, |s| {
            Ok(bool::from_str(&s.to_lowercase()).unwrap())
        })
    }
}
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct EnableMetricsCollectionQuery {
    /// <p>The name of the Auto Scaling group.</p>
    pub auto_scaling_group_name: String,
    /// <p>The granularity to associate with the metrics to collect. The only valid value is <code>1Minute</code>.</p>
    pub granularity: String,
    /// <p>Specifies which group-level metrics to start collecting. You can specify one or more of the following metrics:</p> <ul> <li> <p> <code>GroupMinSize</code> </p> </li> <li> <p> <code>GroupMaxSize</code> </p> </li> <li> <p> <code>GroupDesiredCapacity</code> </p> </li> <li> <p> <code>GroupInServiceInstances</code> </p> </li> <li> <p> <code>GroupPendingInstances</code> </p> </li> <li> <p> <code>GroupStandbyInstances</code> </p> </li> <li> <p> <code>GroupTerminatingInstances</code> </p> </li> <li> <p> <code>GroupTotalInstances</code> </p> </li> </ul> <p>The instance weighting feature supports the following additional metrics: </p> <ul> <li> <p> <code>GroupInServiceCapacity</code> </p> </li> <li> <p> <code>GroupPendingCapacity</code> </p> </li> <li> <p> <code>GroupStandbyCapacity</code> </p> </li> <li> <p> <code>GroupTerminatingCapacity</code> </p> </li> <li> <p> <code>GroupTotalCapacity</code> </p> </li> </ul> <p>If you omit this parameter, all metrics are enabled. </p>
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
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct EnabledMetric {
    /// <p>The granularity of the metric. The only valid value is <code>1Minute</code>.</p>
    pub granularity: Option<String>,
    /// <p><p>One of the following metrics:</p> <ul> <li> <p> <code>GroupMinSize</code> </p> </li> <li> <p> <code>GroupMaxSize</code> </p> </li> <li> <p> <code>GroupDesiredCapacity</code> </p> </li> <li> <p> <code>GroupInServiceInstances</code> </p> </li> <li> <p> <code>GroupPendingInstances</code> </p> </li> <li> <p> <code>GroupStandbyInstances</code> </p> </li> <li> <p> <code>GroupTerminatingInstances</code> </p> </li> <li> <p> <code>GroupTotalInstances</code> </p> </li> <li> <p> <code>GroupInServiceCapacity</code> </p> </li> <li> <p> <code>GroupPendingCapacity</code> </p> </li> <li> <p> <code>GroupStandbyCapacity</code> </p> </li> <li> <p> <code>GroupTerminatingCapacity</code> </p> </li> <li> <p> <code>GroupTotalCapacity</code> </p> </li> </ul></p>
    pub metric: Option<String>,
}

#[allow(dead_code)]
struct EnabledMetricDeserializer;
impl EnabledMetricDeserializer {
    #[allow(dead_code, unused_variables)]
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
#[allow(dead_code)]
struct EnabledMetricsDeserializer;
impl EnabledMetricsDeserializer {
    #[allow(dead_code, unused_variables)]
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
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct EnterStandbyAnswer {
    /// <p>The activities related to moving instances into <code>Standby</code> mode.</p>
    pub activities: Option<Vec<Activity>>,
}

#[allow(dead_code)]
struct EnterStandbyAnswerDeserializer;
impl EnterStandbyAnswerDeserializer {
    #[allow(dead_code, unused_variables)]
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
#[derive(Clone, Debug, Default, PartialEq)]
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

#[allow(dead_code)]
struct EstimatedInstanceWarmupDeserializer;
impl EstimatedInstanceWarmupDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, |s| Ok(i64::from_str(&s).unwrap()))
    }
}
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ExecutePolicyType {
    /// <p>The name of the Auto Scaling group.</p>
    pub auto_scaling_group_name: Option<String>,
    /// <p>The breach threshold for the alarm.</p> <p>Required if the policy type is <code>StepScaling</code> and not supported otherwise.</p>
    pub breach_threshold: Option<f64>,
    /// <p>Indicates whether Amazon EC2 Auto Scaling waits for the cooldown period to complete before executing the policy.</p> <p>Valid only if the policy type is <code>SimpleScaling</code>. For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/Cooldown.html">Scaling cooldowns for Amazon EC2 Auto Scaling</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    pub honor_cooldown: Option<bool>,
    /// <p>The metric value to compare to <code>BreachThreshold</code>. This enables you to execute a policy of type <code>StepScaling</code> and determine which step adjustment to use. For example, if the breach threshold is 50 and you want to use a step adjustment with a lower bound of 0 and an upper bound of 10, you can set the metric value to 59.</p> <p>If you specify a metric value that doesn't correspond to a step adjustment for the policy, the call returns an error.</p> <p>Required if the policy type is <code>StepScaling</code> and not supported otherwise.</p>
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

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ExitStandbyAnswer {
    /// <p>The activities related to moving instances out of <code>Standby</code> mode.</p>
    pub activities: Option<Vec<Activity>>,
}

#[allow(dead_code)]
struct ExitStandbyAnswerDeserializer;
impl ExitStandbyAnswerDeserializer {
    #[allow(dead_code, unused_variables)]
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
#[derive(Clone, Debug, Default, PartialEq)]
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
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct FailedScheduledUpdateGroupActionRequest {
    /// <p>The error code.</p>
    pub error_code: Option<String>,
    /// <p>The error message accompanying the error code.</p>
    pub error_message: Option<String>,
    /// <p>The name of the scheduled action.</p>
    pub scheduled_action_name: String,
}

#[allow(dead_code)]
struct FailedScheduledUpdateGroupActionRequestDeserializer;
impl FailedScheduledUpdateGroupActionRequestDeserializer {
    #[allow(dead_code, unused_variables)]
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
#[allow(dead_code)]
struct FailedScheduledUpdateGroupActionRequestsDeserializer;
impl FailedScheduledUpdateGroupActionRequestsDeserializer {
    #[allow(dead_code, unused_variables)]
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
/// <p>Describes a filter that is used to return a more specific list of results when describing tags.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/autoscaling-tagging.html">Tagging Auto Scaling groups and instances</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Filter {
    /// <p>The name of the filter. The valid values are: <code>auto-scaling-group</code>, <code>key</code>, <code>value</code>, and <code>propagate-at-launch</code>.</p>
    pub name: Option<String>,
    /// <p>One or more filter values. Filter values are case-sensitive.</p>
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

#[allow(dead_code)]
struct GlobalTimeoutDeserializer;
impl GlobalTimeoutDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, |s| Ok(i64::from_str(&s).unwrap()))
    }
}
#[allow(dead_code)]
struct HealthCheckGracePeriodDeserializer;
impl HealthCheckGracePeriodDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, |s| Ok(i64::from_str(&s).unwrap()))
    }
}
#[allow(dead_code)]
struct HeartbeatTimeoutDeserializer;
impl HeartbeatTimeoutDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, |s| Ok(i64::from_str(&s).unwrap()))
    }
}
/// <p>Describes an EC2 instance.</p>
#[derive(Clone, Debug, Default, PartialEq)]
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

#[allow(dead_code)]
struct InstanceMetadataEndpointStateDeserializer;
impl InstanceMetadataEndpointStateDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}
#[allow(dead_code)]
struct InstanceMetadataHttpPutResponseHopLimitDeserializer;
impl InstanceMetadataHttpPutResponseHopLimitDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, |s| Ok(i64::from_str(&s).unwrap()))
    }
}
#[allow(dead_code)]
struct InstanceMetadataHttpTokensStateDeserializer;
impl InstanceMetadataHttpTokensStateDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}
/// <p>The metadata options for the instances. For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/create-launch-config.html#launch-configurations-imds">Configuring the Instance Metadata Options</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct InstanceMetadataOptions {
    /// <p><p>This parameter enables or disables the HTTP metadata endpoint on your instances. If the parameter is not specified, the default state is <code>enabled</code>.</p> <note> <p>If you specify a value of <code>disabled</code>, you will not be able to access your instance metadata. </p> </note></p>
    pub http_endpoint: Option<String>,
    /// <p>The desired HTTP PUT response hop limit for instance metadata requests. The larger the number, the further instance metadata requests can travel.</p> <p>Default: 1</p> <p>Possible values: Integers from 1 to 64</p>
    pub http_put_response_hop_limit: Option<i64>,
    /// <p>The state of token usage for your instance metadata requests. If the parameter is not specified in the request, the default state is <code>optional</code>.</p> <p>If the state is <code>optional</code>, you can choose to retrieve instance metadata with or without a signed token header on your request. If you retrieve the IAM role credentials without a token, the version 1.0 role credentials are returned. If you retrieve the IAM role credentials using a valid signed token, the version 2.0 role credentials are returned.</p> <p>If the state is <code>required</code>, you must send a signed token header with any instance metadata retrieval requests. In this state, retrieving the IAM role credentials always returns the version 2.0 credentials; the version 1.0 credentials are not available.</p>
    pub http_tokens: Option<String>,
}

#[allow(dead_code)]
struct InstanceMetadataOptionsDeserializer;
impl InstanceMetadataOptionsDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<InstanceMetadataOptions, XmlParseError> {
        deserialize_elements::<_, InstanceMetadataOptions, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "HttpEndpoint" => {
                        obj.http_endpoint =
                            Some(InstanceMetadataEndpointStateDeserializer::deserialize(
                                "HttpEndpoint",
                                stack,
                            )?);
                    }
                    "HttpPutResponseHopLimit" => {
                        obj.http_put_response_hop_limit = Some(
                            InstanceMetadataHttpPutResponseHopLimitDeserializer::deserialize(
                                "HttpPutResponseHopLimit",
                                stack,
                            )?,
                        );
                    }
                    "HttpTokens" => {
                        obj.http_tokens =
                            Some(InstanceMetadataHttpTokensStateDeserializer::deserialize(
                                "HttpTokens",
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

/// Serialize `InstanceMetadataOptions` contents to a `SignedRequest`.
struct InstanceMetadataOptionsSerializer;
impl InstanceMetadataOptionsSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &InstanceMetadataOptions) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.http_endpoint {
            params.put(&format!("{}{}", prefix, "HttpEndpoint"), &field_value);
        }
        if let Some(ref field_value) = obj.http_put_response_hop_limit {
            params.put(
                &format!("{}{}", prefix, "HttpPutResponseHopLimit"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.http_tokens {
            params.put(&format!("{}{}", prefix, "HttpTokens"), &field_value);
        }
    }
}

/// <p>Describes whether detailed monitoring is enabled for the Auto Scaling instances.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct InstanceMonitoring {
    /// <p>If <code>true</code>, detailed monitoring is enabled. Otherwise, basic monitoring is enabled.</p>
    pub enabled: Option<bool>,
}

#[allow(dead_code)]
struct InstanceMonitoringDeserializer;
impl InstanceMonitoringDeserializer {
    #[allow(dead_code, unused_variables)]
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

#[allow(dead_code)]
struct InstanceProtectedDeserializer;
impl InstanceProtectedDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<bool, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, |s| {
            Ok(bool::from_str(&s.to_lowercase()).unwrap())
        })
    }
}
/// <p>Describes an instance refresh for an Auto Scaling group. </p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct InstanceRefresh {
    /// <p>The name of the Auto Scaling group.</p>
    pub auto_scaling_group_name: Option<String>,
    /// <p>The date and time at which the instance refresh ended.</p>
    pub end_time: Option<String>,
    /// <p>The instance refresh ID.</p>
    pub instance_refresh_id: Option<String>,
    /// <p>The number of instances remaining to update before the instance refresh is complete.</p>
    pub instances_to_update: Option<i64>,
    /// <p>The percentage of the instance refresh that is complete. For each instance replacement, Amazon EC2 Auto Scaling tracks the instance's health status and warm-up time. When the instance's health status changes to healthy and the specified warm-up time passes, the instance is considered updated and added to the percentage complete.</p>
    pub percentage_complete: Option<i64>,
    /// <p>The date and time at which the instance refresh began.</p>
    pub start_time: Option<String>,
    /// <p><p>The current status for the instance refresh operation:</p> <ul> <li> <p> <code>Pending</code> - The request was created, but the operation has not started.</p> </li> <li> <p> <code>InProgress</code> - The operation is in progress.</p> </li> <li> <p> <code>Successful</code> - The operation completed successfully.</p> </li> <li> <p> <code>Failed</code> - The operation failed to complete. You can troubleshoot using the status reason and the scaling activities. </p> </li> <li> <p> <code>Cancelling</code> - An ongoing operation is being cancelled. Cancellation does not roll back any replacements that have already been completed, but it prevents new replacements from being started. </p> </li> <li> <p> <code>Cancelled</code> - The operation is cancelled. </p> </li> </ul></p>
    pub status: Option<String>,
    /// <p>Provides more details about the current status of the instance refresh. </p>
    pub status_reason: Option<String>,
}

#[allow(dead_code)]
struct InstanceRefreshDeserializer;
impl InstanceRefreshDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<InstanceRefresh, XmlParseError> {
        deserialize_elements::<_, InstanceRefresh, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "AutoScalingGroupName" => {
                    obj.auto_scaling_group_name = Some(
                        XmlStringMaxLen255Deserializer::deserialize("AutoScalingGroupName", stack)?,
                    );
                }
                "EndTime" => {
                    obj.end_time = Some(TimestampTypeDeserializer::deserialize("EndTime", stack)?);
                }
                "InstanceRefreshId" => {
                    obj.instance_refresh_id = Some(XmlStringMaxLen255Deserializer::deserialize(
                        "InstanceRefreshId",
                        stack,
                    )?);
                }
                "InstancesToUpdate" => {
                    obj.instances_to_update = Some(InstancesToUpdateDeserializer::deserialize(
                        "InstancesToUpdate",
                        stack,
                    )?);
                }
                "PercentageComplete" => {
                    obj.percentage_complete = Some(IntPercentDeserializer::deserialize(
                        "PercentageComplete",
                        stack,
                    )?);
                }
                "StartTime" => {
                    obj.start_time =
                        Some(TimestampTypeDeserializer::deserialize("StartTime", stack)?);
                }
                "Status" => {
                    obj.status = Some(InstanceRefreshStatusDeserializer::deserialize(
                        "Status", stack,
                    )?);
                }
                "StatusReason" => {
                    obj.status_reason = Some(XmlStringMaxLen1023Deserializer::deserialize(
                        "StatusReason",
                        stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

/// Serialize `InstanceRefreshIds` contents to a `SignedRequest`.
struct InstanceRefreshIdsSerializer;
impl InstanceRefreshIdsSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

#[allow(dead_code)]
struct InstanceRefreshStatusDeserializer;
impl InstanceRefreshStatusDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}
#[allow(dead_code)]
struct InstanceRefreshesDeserializer;
impl InstanceRefreshesDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<InstanceRefresh>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(InstanceRefreshDeserializer::deserialize("member", stack)?);
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
/// <p>Describes an instances distribution for an Auto Scaling group with a <a>MixedInstancesPolicy</a>.</p> <p>The instances distribution specifies the distribution of On-Demand Instances and Spot Instances, the maximum price to pay for Spot Instances, and how the Auto Scaling group allocates instance types to fulfill On-Demand and Spot capacities.</p> <p>When you update <code>SpotAllocationStrategy</code>, <code>SpotInstancePools</code>, or <code>SpotMaxPrice</code>, this update action does not deploy any changes across the running Amazon EC2 instances in the group. Your existing Spot Instances continue to run as long as the maximum price for those instances is higher than the current Spot price. When scale out occurs, Amazon EC2 Auto Scaling launches instances based on the new settings. When scale in occurs, Amazon EC2 Auto Scaling terminates instances according to the group's termination policies.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct InstancesDistribution {
    /// <p>Indicates how to allocate instance types to fulfill On-Demand capacity. The only valid value is <code>prioritized</code>, which is also the default value. This strategy uses the order of instance types in the overrides to define the launch priority of each instance type. The first instance type in the array is prioritized higher than the last. If all your On-Demand capacity cannot be fulfilled using your highest priority instance, then the Auto Scaling groups launches the remaining capacity using the second priority instance type, and so on.</p>
    pub on_demand_allocation_strategy: Option<String>,
    /// <p>The minimum amount of the Auto Scaling group's capacity that must be fulfilled by On-Demand Instances. This base portion is provisioned first as your group scales. Defaults to 0 if not specified. If you specify weights for the instance types in the overrides, set the value of <code>OnDemandBaseCapacity</code> in terms of the number of capacity units, and not the number of instances.</p>
    pub on_demand_base_capacity: Option<i64>,
    /// <p>Controls the percentages of On-Demand Instances and Spot Instances for your additional capacity beyond <code>OnDemandBaseCapacity</code>. Expressed as a number (for example, 20 specifies 20% On-Demand Instances, 80% Spot Instances). Defaults to 100 if not specified. If set to 100, only On-Demand Instances are provisioned.</p>
    pub on_demand_percentage_above_base_capacity: Option<i64>,
    /// <p>Indicates how to allocate instances across Spot Instance pools. If the allocation strategy is <code>capacity-optimized</code> (recommended), the Auto Scaling group launches instances using Spot pools that are optimally chosen based on the available Spot capacity. If the allocation strategy is <code>lowest-price</code>, the Auto Scaling group launches instances using the Spot pools with the lowest price, and evenly allocates your instances across the number of Spot pools that you specify. Defaults to <code>lowest-price</code> if not specified.</p>
    pub spot_allocation_strategy: Option<String>,
    /// <p>The number of Spot Instance pools across which to allocate your Spot Instances. The Spot pools are determined from the different instance types in the overrides. Valid only when the Spot allocation strategy is <code>lowest-price</code>. Value must be in the range of 1 to 20. Defaults to 2 if not specified.</p>
    pub spot_instance_pools: Option<i64>,
    /// <p>The maximum price per unit hour that you are willing to pay for a Spot Instance. If you leave the value at its default (empty), Amazon EC2 Auto Scaling uses the On-Demand price as the maximum Spot price. To remove a value that you previously set, include the property but specify an empty string ("") for the value.</p>
    pub spot_max_price: Option<String>,
}

#[allow(dead_code)]
struct InstancesDistributionDeserializer;
impl InstancesDistributionDeserializer {
    #[allow(dead_code, unused_variables)]
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

#[allow(dead_code)]
struct InstancesToUpdateDeserializer;
impl InstancesToUpdateDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, |s| Ok(i64::from_str(&s).unwrap()))
    }
}
#[allow(dead_code)]
struct IntPercentDeserializer;
impl IntPercentDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, |s| Ok(i64::from_str(&s).unwrap()))
    }
}
/// <p>Describes a launch configuration.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct LaunchConfiguration {
    /// <p>For Auto Scaling groups that are running in a VPC, specifies whether to assign a public IP address to the group's instances. For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/asg-in-vpc.html">Launching Auto Scaling instances in a VPC</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    pub associate_public_ip_address: Option<bool>,
    /// <p>A block device mapping, which specifies the block devices for the instance. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/block-device-mapping-concepts.html">Block Device Mapping</a> in the <i>Amazon EC2 User Guide for Linux Instances</i>.</p>
    pub block_device_mappings: Option<Vec<BlockDeviceMapping>>,
    /// <p>The ID of a ClassicLink-enabled VPC to link your EC2-Classic instances to. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/vpc-classiclink.html">ClassicLink</a> in the <i>Amazon EC2 User Guide for Linux Instances</i> and <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/asg-in-vpc.html#as-ClassicLink">Linking EC2-Classic instances to a VPC</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    pub classic_link_vpc_id: Option<String>,
    /// <p>The IDs of one or more security groups for the VPC specified in <code>ClassicLinkVPCId</code>.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/vpc-classiclink.html">ClassicLink</a> in the <i>Amazon EC2 User Guide for Linux Instances</i> and <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/asg-in-vpc.html#as-ClassicLink">Linking EC2-Classic instances to a VPC</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    pub classic_link_vpc_security_groups: Option<Vec<String>>,
    /// <p>The creation date and time for the launch configuration.</p>
    pub created_time: String,
    /// <p>Specifies whether the launch configuration is optimized for EBS I/O (<code>true</code>) or not (<code>false</code>). For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/EBSOptimized.html">Amazon EBS-Optimized Instances</a> in the <i>Amazon EC2 User Guide for Linux Instances</i>.</p>
    pub ebs_optimized: Option<bool>,
    /// <p>The name or the Amazon Resource Name (ARN) of the instance profile associated with the IAM role for the instance. The instance profile contains the IAM role. For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/us-iam-role.html">IAM role for applications that run on Amazon EC2 instances</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    pub iam_instance_profile: Option<String>,
    /// <p>The ID of the Amazon Machine Image (AMI) to use to launch your EC2 instances. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/finding-an-ami.html">Finding an AMI</a> in the <i>Amazon EC2 User Guide for Linux Instances</i>.</p>
    pub image_id: String,
    /// <p>Controls whether instances in this group are launched with detailed (<code>true</code>) or basic (<code>false</code>) monitoring.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/latest/userguide/enable-as-instance-metrics.html">Configure Monitoring for Auto Scaling Instances</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
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
    /// <p>The metadata options for the instances. For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/create-launch-config.html#launch-configurations-imds">Configuring the Instance Metadata Options</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    pub metadata_options: Option<InstanceMetadataOptions>,
    /// <p>The tenancy of the instance, either <code>default</code> or <code>dedicated</code>. An instance with <code>dedicated</code> tenancy runs on isolated, single-tenant hardware and can only be launched into a VPC.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/auto-scaling-dedicated-instances.html">Configuring instance tenancy with Amazon EC2 Auto Scaling</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    pub placement_tenancy: Option<String>,
    /// <p>The ID of the RAM disk associated with the AMI.</p>
    pub ramdisk_id: Option<String>,
    /// <p>A list that contains the security groups to assign to the instances in the Auto Scaling group. For more information, see <a href="https://docs.aws.amazon.com/AmazonVPC/latest/UserGuide/VPC_SecurityGroups.html">Security Groups for Your VPC</a> in the <i>Amazon Virtual Private Cloud User Guide</i>.</p>
    pub security_groups: Option<Vec<String>>,
    /// <p>The maximum hourly price to be paid for any Spot Instance launched to fulfill the request. Spot Instances are launched when the price you specify exceeds the current Spot price. For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/asg-launch-spot-instances.html">Requesting Spot Instances</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    pub spot_price: Option<String>,
    /// <p>The Base64-encoded user data to make available to the launched EC2 instances. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-instance-metadata.html">Instance metadata and user data</a> in the <i>Amazon EC2 User Guide for Linux Instances</i>.</p>
    pub user_data: Option<String>,
}

#[allow(dead_code)]
struct LaunchConfigurationDeserializer;
impl LaunchConfigurationDeserializer {
    #[allow(dead_code, unused_variables)]
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
                "MetadataOptions" => {
                    obj.metadata_options = Some(InstanceMetadataOptionsDeserializer::deserialize(
                        "MetadataOptions",
                        stack,
                    )?);
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
#[derive(Clone, Debug, Default, PartialEq)]
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

#[derive(Clone, Debug, Default, PartialEq)]
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

#[allow(dead_code)]
struct LaunchConfigurationsDeserializer;
impl LaunchConfigurationsDeserializer {
    #[allow(dead_code, unused_variables)]
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
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct LaunchConfigurationsType {
    /// <p>The launch configurations.</p>
    pub launch_configurations: Vec<LaunchConfiguration>,
    /// <p>A string that indicates that the response contains more items than can be returned in a single response. To receive additional items, specify this string for the <code>NextToken</code> value when requesting the next set of items. This value is null when there are no more items to return.</p>
    pub next_token: Option<String>,
}

#[allow(dead_code)]
struct LaunchConfigurationsTypeDeserializer;
impl LaunchConfigurationsTypeDeserializer {
    #[allow(dead_code, unused_variables)]
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
/// <p>Describes a launch template and overrides. </p> <p>You specify these parameters as part of a mixed instances policy. </p> <p>When you update the launch template or overrides, existing Amazon EC2 instances continue to run. When scale out occurs, Amazon EC2 Auto Scaling launches instances to match the new settings. When scale in occurs, Amazon EC2 Auto Scaling terminates instances according to the group's termination policies.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct LaunchTemplate {
    /// <p>The launch template to use.</p>
    pub launch_template_specification: Option<LaunchTemplateSpecification>,
    /// <p>Any parameters that you specify override the same parameters in the launch template. If not provided, Amazon EC2 Auto Scaling uses the instance type specified in the launch template when it launches an instance. </p>
    pub overrides: Option<Vec<LaunchTemplateOverrides>>,
}

#[allow(dead_code)]
struct LaunchTemplateDeserializer;
impl LaunchTemplateDeserializer {
    #[allow(dead_code, unused_variables)]
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

#[allow(dead_code)]
struct LaunchTemplateNameDeserializer;
impl LaunchTemplateNameDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}
/// <p>Describes an override for a launch template. The maximum number of instance types that can be associated with an Auto Scaling group is 20. For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/asg-override-options.html">Configuring overrides</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>. </p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct LaunchTemplateOverrides {
    /// <p>The instance type, such as <code>m3.xlarge</code>. You must use an instance type that is supported in your requested Region and Availability Zones. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/instance-types.html">Instance types</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
    pub instance_type: Option<String>,
    /// <p>Provides the launch template to be used when launching the instance type. For example, some instance types might require a launch template with a different AMI. If not provided, Amazon EC2 Auto Scaling uses the launch template that's defined for your mixed instances policy. For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/asg-launch-template-overrides.html">Specifying a different launch template for an instance type</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>. </p>
    pub launch_template_specification: Option<LaunchTemplateSpecification>,
    /// <p>The number of capacity units provided by the specified instance type in terms of virtual CPUs, memory, storage, throughput, or other relative performance characteristic. When a Spot or On-Demand Instance is provisioned, the capacity units count toward the desired capacity. Amazon EC2 Auto Scaling provisions instances until the desired capacity is totally fulfilled, even if this results in an overage. For example, if there are 2 units remaining to fulfill capacity, and Amazon EC2 Auto Scaling can only provision an instance with a <code>WeightedCapacity</code> of 5 units, the instance is provisioned, and the desired capacity is exceeded by 3 units. For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/asg-instance-weighting.html">Instance weighting for Amazon EC2 Auto Scaling</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>. Value must be in the range of 1 to 999.</p>
    pub weighted_capacity: Option<String>,
}

#[allow(dead_code)]
struct LaunchTemplateOverridesDeserializer;
impl LaunchTemplateOverridesDeserializer {
    #[allow(dead_code, unused_variables)]
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
                    "LaunchTemplateSpecification" => {
                        obj.launch_template_specification =
                            Some(LaunchTemplateSpecificationDeserializer::deserialize(
                                "LaunchTemplateSpecification",
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
        if let Some(ref field_value) = obj.launch_template_specification {
            LaunchTemplateSpecificationSerializer::serialize(
                params,
                &format!("{}{}", prefix, "LaunchTemplateSpecification"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.weighted_capacity {
            params.put(&format!("{}{}", prefix, "WeightedCapacity"), &field_value);
        }
    }
}

/// <p>Describes the Amazon EC2 launch template and the launch template version that can be used by an Auto Scaling group to configure Amazon EC2 instances.</p> <p>The launch template that is specified must be configured for use with an Auto Scaling group. For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/create-launch-template.html">Creating a launch template for an Auto Scaling group</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct LaunchTemplateSpecification {
    /// <p>The ID of the launch template. To get the template ID, use the Amazon EC2 <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_DescribeLaunchTemplates.html">DescribeLaunchTemplates</a> API operation. New launch templates can be created using the Amazon EC2 <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_CreateLaunchTemplate.html">CreateLaunchTemplate</a> API. </p> <p>Conditional: You must specify either a <code>LaunchTemplateId</code> or a <code>LaunchTemplateName</code>.</p>
    pub launch_template_id: Option<String>,
    /// <p>The name of the launch template. To get the template name, use the Amazon EC2 <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_DescribeLaunchTemplates.html">DescribeLaunchTemplates</a> API operation. New launch templates can be created using the Amazon EC2 <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_CreateLaunchTemplate.html">CreateLaunchTemplate</a> API. </p> <p>Conditional: You must specify either a <code>LaunchTemplateId</code> or a <code>LaunchTemplateName</code>.</p>
    pub launch_template_name: Option<String>,
    /// <p>The version number, <code>$Latest</code>, or <code>$Default</code>. To get the version number, use the Amazon EC2 <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_DescribeLaunchTemplateVersions.html">DescribeLaunchTemplateVersions</a> API operation. New launch template versions can be created using the Amazon EC2 <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_CreateLaunchTemplateVersion.html">CreateLaunchTemplateVersion</a> API. If the value is <code>$Latest</code>, Amazon EC2 Auto Scaling selects the latest version of the launch template when launching instances. If the value is <code>$Default</code>, Amazon EC2 Auto Scaling selects the default version of the launch template when launching instances. The default value is <code>$Default</code>.</p>
    pub version: Option<String>,
}

#[allow(dead_code)]
struct LaunchTemplateSpecificationDeserializer;
impl LaunchTemplateSpecificationDeserializer {
    #[allow(dead_code, unused_variables)]
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

#[allow(dead_code)]
struct LifecycleActionResultDeserializer;
impl LifecycleActionResultDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}
/// <p>Describes a lifecycle hook, which tells Amazon EC2 Auto Scaling that you want to perform an action whenever it launches instances or terminates instances.</p>
#[derive(Clone, Debug, Default, PartialEq)]
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

#[allow(dead_code)]
struct LifecycleHookDeserializer;
impl LifecycleHookDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<LifecycleHook, XmlParseError> {
        deserialize_elements::<_, LifecycleHook, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "AutoScalingGroupName" => {
                    obj.auto_scaling_group_name = Some(
                        XmlStringMaxLen255Deserializer::deserialize("AutoScalingGroupName", stack)?,
                    );
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
                    obj.notification_target_arn =
                        Some(NotificationTargetResourceNameDeserializer::deserialize(
                            "NotificationTargetARN",
                            stack,
                        )?);
                }
                "RoleARN" => {
                    obj.role_arn = Some(XmlStringMaxLen255Deserializer::deserialize(
                        "RoleARN", stack,
                    )?);
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

/// <p>Describes information used to specify a lifecycle hook for an Auto Scaling group.</p> <p>A lifecycle hook tells Amazon EC2 Auto Scaling to perform an action on an instance when the instance launches (before it is put into service) or as the instance terminates (before it is fully terminated).</p> <p>This step is a part of the procedure for creating a lifecycle hook for an Auto Scaling group:</p> <ol> <li> <p>(Optional) Create a Lambda function and a rule that allows CloudWatch Events to invoke your Lambda function when Amazon EC2 Auto Scaling launches or terminates instances.</p> </li> <li> <p>(Optional) Create a notification target and an IAM role. The target can be either an Amazon SQS queue or an Amazon SNS topic. The role allows Amazon EC2 Auto Scaling to publish lifecycle notifications to the target.</p> </li> <li> <p> <b>Create the lifecycle hook. Specify whether the hook is used when the instances launch or terminate.</b> </p> </li> <li> <p>If you need more time, record the lifecycle action heartbeat to keep the instance in a pending state.</p> </li> <li> <p>If you finish before the timeout period ends, complete the lifecycle action.</p> </li> </ol> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/lifecycle-hooks.html">Amazon EC2 Auto Scaling lifecycle hooks</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
#[derive(Clone, Debug, Default, PartialEq)]
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

#[allow(dead_code)]
struct LifecycleHooksDeserializer;
impl LifecycleHooksDeserializer {
    #[allow(dead_code, unused_variables)]
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
#[allow(dead_code)]
struct LifecycleStateDeserializer;
impl LifecycleStateDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}
#[allow(dead_code)]
struct LifecycleTransitionDeserializer;
impl LifecycleTransitionDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}
#[allow(dead_code)]
struct LoadBalancerNamesDeserializer;
impl LoadBalancerNamesDeserializer {
    #[allow(dead_code, unused_variables)]
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
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct LoadBalancerState {
    /// <p>The name of the load balancer.</p>
    pub load_balancer_name: Option<String>,
    /// <p><p>One of the following load balancer states:</p> <ul> <li> <p> <code>Adding</code> - The instances in the group are being registered with the load balancer.</p> </li> <li> <p> <code>Added</code> - All instances in the group are registered with the load balancer.</p> </li> <li> <p> <code>InService</code> - At least one instance in the group passed an ELB health check.</p> </li> <li> <p> <code>Removing</code> - The instances in the group are being deregistered from the load balancer. If connection draining is enabled, Elastic Load Balancing waits for in-flight requests to complete before deregistering the instances.</p> </li> <li> <p> <code>Removed</code> - All instances in the group are deregistered from the load balancer.</p> </li> </ul></p>
    pub state: Option<String>,
}

#[allow(dead_code)]
struct LoadBalancerStateDeserializer;
impl LoadBalancerStateDeserializer {
    #[allow(dead_code, unused_variables)]
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
#[allow(dead_code)]
struct LoadBalancerStatesDeserializer;
impl LoadBalancerStatesDeserializer {
    #[allow(dead_code, unused_variables)]
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
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct LoadBalancerTargetGroupState {
    /// <p>The Amazon Resource Name (ARN) of the target group.</p>
    pub load_balancer_target_group_arn: Option<String>,
    /// <p><p>The state of the target group.</p> <ul> <li> <p> <code>Adding</code> - The Auto Scaling instances are being registered with the target group.</p> </li> <li> <p> <code>Added</code> - All Auto Scaling instances are registered with the target group.</p> </li> <li> <p> <code>InService</code> - At least one Auto Scaling instance passed an ELB health check.</p> </li> <li> <p> <code>Removing</code> - The Auto Scaling instances are being deregistered from the target group. If connection draining is enabled, Elastic Load Balancing waits for in-flight requests to complete before deregistering the instances.</p> </li> <li> <p> <code>Removed</code> - All Auto Scaling instances are deregistered from the target group.</p> </li> </ul></p>
    pub state: Option<String>,
}

#[allow(dead_code)]
struct LoadBalancerTargetGroupStateDeserializer;
impl LoadBalancerTargetGroupStateDeserializer {
    #[allow(dead_code, unused_variables)]
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
#[allow(dead_code)]
struct LoadBalancerTargetGroupStatesDeserializer;
impl LoadBalancerTargetGroupStatesDeserializer {
    #[allow(dead_code, unused_variables)]
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
#[allow(dead_code)]
struct MaxInstanceLifetimeDeserializer;
impl MaxInstanceLifetimeDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, |s| Ok(i64::from_str(&s).unwrap()))
    }
}
#[allow(dead_code)]
struct MaxNumberOfAutoScalingGroupsDeserializer;
impl MaxNumberOfAutoScalingGroupsDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, |s| Ok(i64::from_str(&s).unwrap()))
    }
}
#[allow(dead_code)]
struct MaxNumberOfLaunchConfigurationsDeserializer;
impl MaxNumberOfLaunchConfigurationsDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, |s| Ok(i64::from_str(&s).unwrap()))
    }
}
/// <p>Describes a metric.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct MetricCollectionType {
    /// <p><p>One of the following metrics:</p> <ul> <li> <p> <code>GroupMinSize</code> </p> </li> <li> <p> <code>GroupMaxSize</code> </p> </li> <li> <p> <code>GroupDesiredCapacity</code> </p> </li> <li> <p> <code>GroupInServiceInstances</code> </p> </li> <li> <p> <code>GroupPendingInstances</code> </p> </li> <li> <p> <code>GroupStandbyInstances</code> </p> </li> <li> <p> <code>GroupTerminatingInstances</code> </p> </li> <li> <p> <code>GroupTotalInstances</code> </p> </li> <li> <p> <code>GroupInServiceCapacity</code> </p> </li> <li> <p> <code>GroupPendingCapacity</code> </p> </li> <li> <p> <code>GroupStandbyCapacity</code> </p> </li> <li> <p> <code>GroupTerminatingCapacity</code> </p> </li> <li> <p> <code>GroupTotalCapacity</code> </p> </li> </ul></p>
    pub metric: Option<String>,
}

#[allow(dead_code)]
struct MetricCollectionTypeDeserializer;
impl MetricCollectionTypeDeserializer {
    #[allow(dead_code, unused_variables)]
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
#[allow(dead_code)]
struct MetricCollectionTypesDeserializer;
impl MetricCollectionTypesDeserializer {
    #[allow(dead_code, unused_variables)]
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
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct MetricDimension {
    /// <p>The name of the dimension.</p>
    pub name: String,
    /// <p>The value of the dimension.</p>
    pub value: String,
}

#[allow(dead_code)]
struct MetricDimensionDeserializer;
impl MetricDimensionDeserializer {
    #[allow(dead_code, unused_variables)]
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

#[allow(dead_code)]
struct MetricDimensionNameDeserializer;
impl MetricDimensionNameDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}
#[allow(dead_code)]
struct MetricDimensionValueDeserializer;
impl MetricDimensionValueDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}
#[allow(dead_code)]
struct MetricDimensionsDeserializer;
impl MetricDimensionsDeserializer {
    #[allow(dead_code, unused_variables)]
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
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct MetricGranularityType {
    /// <p>The granularity. The only valid value is <code>1Minute</code>.</p>
    pub granularity: Option<String>,
}

#[allow(dead_code)]
struct MetricGranularityTypeDeserializer;
impl MetricGranularityTypeDeserializer {
    #[allow(dead_code, unused_variables)]
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
#[allow(dead_code)]
struct MetricGranularityTypesDeserializer;
impl MetricGranularityTypesDeserializer {
    #[allow(dead_code, unused_variables)]
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
#[allow(dead_code)]
struct MetricNameDeserializer;
impl MetricNameDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}
#[allow(dead_code)]
struct MetricNamespaceDeserializer;
impl MetricNamespaceDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}
#[allow(dead_code)]
struct MetricScaleDeserializer;
impl MetricScaleDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<f64, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, |s| Ok(f64::from_str(&s).unwrap()))
    }
}
#[allow(dead_code)]
struct MetricStatisticDeserializer;
impl MetricStatisticDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}
#[allow(dead_code)]
struct MetricTypeDeserializer;
impl MetricTypeDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}
#[allow(dead_code)]
struct MetricUnitDeserializer;
impl MetricUnitDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
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

#[allow(dead_code)]
struct MinAdjustmentMagnitudeDeserializer;
impl MinAdjustmentMagnitudeDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, |s| Ok(i64::from_str(&s).unwrap()))
    }
}
#[allow(dead_code)]
struct MinAdjustmentStepDeserializer;
impl MinAdjustmentStepDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, |s| Ok(i64::from_str(&s).unwrap()))
    }
}
#[allow(dead_code)]
struct MixedInstanceSpotPriceDeserializer;
impl MixedInstanceSpotPriceDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}
/// <p>Describes a mixed instances policy for an Auto Scaling group. With mixed instances, your Auto Scaling group can provision a combination of On-Demand Instances and Spot Instances across multiple instance types. For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/asg-purchase-options.html">Auto Scaling groups with multiple instance types and purchase options</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p> <p>You can create a mixed instances policy for a new Auto Scaling group, or you can create it for an existing group by updating the group to specify <code>MixedInstancesPolicy</code> as the top-level parameter instead of a launch configuration or launch template.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct MixedInstancesPolicy {
    /// <p>Specifies the instances distribution. If not provided, the value for each parameter in <code>InstancesDistribution</code> uses a default value.</p>
    pub instances_distribution: Option<InstancesDistribution>,
    /// <p>Specifies the launch template to use and optionally the instance types (overrides) that are used to provision EC2 instances to fulfill On-Demand and Spot capacities. Required when creating a mixed instances policy.</p>
    pub launch_template: Option<LaunchTemplate>,
}

#[allow(dead_code)]
struct MixedInstancesPolicyDeserializer;
impl MixedInstancesPolicyDeserializer {
    #[allow(dead_code, unused_variables)]
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

#[allow(dead_code)]
struct MonitoringEnabledDeserializer;
impl MonitoringEnabledDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<bool, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, |s| {
            Ok(bool::from_str(&s.to_lowercase()).unwrap())
        })
    }
}
#[allow(dead_code)]
struct NoDeviceDeserializer;
impl NoDeviceDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<bool, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, |s| {
            Ok(bool::from_str(&s.to_lowercase()).unwrap())
        })
    }
}
/// <p>Describes a notification.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct NotificationConfiguration {
    /// <p>The name of the Auto Scaling group.</p>
    pub auto_scaling_group_name: Option<String>,
    /// <p><p>One of the following event notification types:</p> <ul> <li> <p> <code>autoscaling:EC2<em>INSTANCE</em>LAUNCH</code> </p> </li> <li> <p> <code>autoscaling:EC2<em>INSTANCE</em>LAUNCH<em>ERROR</code> </p> </li> <li> <p> <code>autoscaling:EC2</em>INSTANCE<em>TERMINATE</code> </p> </li> <li> <p> <code>autoscaling:EC2</em>INSTANCE<em>TERMINATE</em>ERROR</code> </p> </li> <li> <p> <code>autoscaling:TEST_NOTIFICATION</code> </p> </li> </ul></p>
    pub notification_type: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the Amazon Simple Notification Service (Amazon SNS) topic.</p>
    pub topic_arn: Option<String>,
}

#[allow(dead_code)]
struct NotificationConfigurationDeserializer;
impl NotificationConfigurationDeserializer {
    #[allow(dead_code, unused_variables)]
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
                        obj.auto_scaling_group_name =
                            Some(XmlStringMaxLen255Deserializer::deserialize(
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
                        obj.topic_arn = Some(XmlStringMaxLen255Deserializer::deserialize(
                            "TopicARN", stack,
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
struct NotificationConfigurationsDeserializer;
impl NotificationConfigurationsDeserializer {
    #[allow(dead_code, unused_variables)]
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
#[allow(dead_code)]
struct NotificationTargetResourceNameDeserializer;
impl NotificationTargetResourceNameDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}
#[allow(dead_code)]
struct NumberOfAutoScalingGroupsDeserializer;
impl NumberOfAutoScalingGroupsDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, |s| Ok(i64::from_str(&s).unwrap()))
    }
}
#[allow(dead_code)]
struct NumberOfLaunchConfigurationsDeserializer;
impl NumberOfLaunchConfigurationsDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, |s| Ok(i64::from_str(&s).unwrap()))
    }
}
#[allow(dead_code)]
struct OnDemandBaseCapacityDeserializer;
impl OnDemandBaseCapacityDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, |s| Ok(i64::from_str(&s).unwrap()))
    }
}
#[allow(dead_code)]
struct OnDemandPercentageAboveBaseCapacityDeserializer;
impl OnDemandPercentageAboveBaseCapacityDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, |s| Ok(i64::from_str(&s).unwrap()))
    }
}
#[allow(dead_code)]
struct OverridesDeserializer;
impl OverridesDeserializer {
    #[allow(dead_code, unused_variables)]
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

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct PoliciesType {
    /// <p>A string that indicates that the response contains more items than can be returned in a single response. To receive additional items, specify this string for the <code>NextToken</code> value when requesting the next set of items. This value is null when there are no more items to return.</p>
    pub next_token: Option<String>,
    /// <p>The scaling policies.</p>
    pub scaling_policies: Option<Vec<ScalingPolicy>>,
}

#[allow(dead_code)]
struct PoliciesTypeDeserializer;
impl PoliciesTypeDeserializer {
    #[allow(dead_code, unused_variables)]
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
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct PolicyARNType {
    /// <p>The CloudWatch alarms created for the target tracking scaling policy.</p>
    pub alarms: Option<Vec<Alarm>>,
    /// <p>The Amazon Resource Name (ARN) of the policy.</p>
    pub policy_arn: Option<String>,
}

#[allow(dead_code)]
struct PolicyARNTypeDeserializer;
impl PolicyARNTypeDeserializer {
    #[allow(dead_code, unused_variables)]
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
#[allow(dead_code)]
struct PolicyIncrementDeserializer;
impl PolicyIncrementDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, |s| Ok(i64::from_str(&s).unwrap()))
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
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PredefinedMetricSpecification {
    /// <p><p>The metric type. The following predefined metrics are available:</p> <ul> <li> <p> <code>ASGAverageCPUUtilization</code> - Average CPU utilization of the Auto Scaling group.</p> </li> <li> <p> <code>ASGAverageNetworkIn</code> - Average number of bytes received on all network interfaces by the Auto Scaling group.</p> </li> <li> <p> <code>ASGAverageNetworkOut</code> - Average number of bytes sent out on all network interfaces by the Auto Scaling group.</p> </li> <li> <p> <code>ALBRequestCountPerTarget</code> - Number of requests completed per target in an Application Load Balancer target group.</p> </li> </ul></p>
    pub predefined_metric_type: String,
    /// <p>Identifies the resource associated with the metric type. You can't specify a resource label unless the metric type is <code>ALBRequestCountPerTarget</code> and there is a target group attached to the Auto Scaling group.</p> <p>You create the resource label by appending the final portion of the load balancer ARN and the final portion of the target group ARN into a single value, separated by a forward slash (/). The format is app/&lt;load-balancer-name&gt;/&lt;load-balancer-id&gt;/targetgroup/&lt;target-group-name&gt;/&lt;target-group-id&gt;, where:</p> <ul> <li> <p>app/&lt;load-balancer-name&gt;/&lt;load-balancer-id&gt; is the final portion of the load balancer ARN</p> </li> <li> <p>targetgroup/&lt;target-group-name&gt;/&lt;target-group-id&gt; is the final portion of the target group ARN.</p> </li> </ul> <p>This is an example: app/EC2Co-EcsEl-1TKLTMITMM0EO/f37c06a68c1748aa/targetgroup/EC2Co-Defau-LDNM7Q3ZH1ZN/6d4ea56ca2d6a18d.</p> <p>To find the ARN for an Application Load Balancer, use the <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/APIReference/API_DescribeLoadBalancers.html">DescribeLoadBalancers</a> API operation. To find the ARN for the target group, use the <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/APIReference/API_DescribeTargetGroups.html">DescribeTargetGroups</a> API operation.</p>
    pub resource_label: Option<String>,
}

#[allow(dead_code)]
struct PredefinedMetricSpecificationDeserializer;
impl PredefinedMetricSpecificationDeserializer {
    #[allow(dead_code, unused_variables)]
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

/// <p>Describes a process type.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/as-suspend-resume-processes.html#process-types">Scaling processes</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ProcessType {
    /// <p><p>One of the following processes:</p> <ul> <li> <p> <code>Launch</code> </p> </li> <li> <p> <code>Terminate</code> </p> </li> <li> <p> <code>AddToLoadBalancer</code> </p> </li> <li> <p> <code>AlarmNotification</code> </p> </li> <li> <p> <code>AZRebalance</code> </p> </li> <li> <p> <code>HealthCheck</code> </p> </li> <li> <p> <code>InstanceRefresh</code> </p> </li> <li> <p> <code>ReplaceUnhealthy</code> </p> </li> <li> <p> <code>ScheduledActions</code> </p> </li> </ul></p>
    pub process_name: String,
}

#[allow(dead_code)]
struct ProcessTypeDeserializer;
impl ProcessTypeDeserializer {
    #[allow(dead_code, unused_variables)]
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
#[allow(dead_code)]
struct ProcessesDeserializer;
impl ProcessesDeserializer {
    #[allow(dead_code, unused_variables)]
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
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ProcessesType {
    /// <p>The names of the process types.</p>
    pub processes: Option<Vec<ProcessType>>,
}

#[allow(dead_code)]
struct ProcessesTypeDeserializer;
impl ProcessesTypeDeserializer {
    #[allow(dead_code, unused_variables)]
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
#[allow(dead_code)]
struct ProgressDeserializer;
impl ProgressDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, |s| Ok(i64::from_str(&s).unwrap()))
    }
}
#[allow(dead_code)]
struct PropagateAtLaunchDeserializer;
impl PropagateAtLaunchDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<bool, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, |s| {
            Ok(bool::from_str(&s.to_lowercase()).unwrap())
        })
    }
}
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct PutLifecycleHookAnswer {}

#[allow(dead_code)]
struct PutLifecycleHookAnswerDeserializer;
impl PutLifecycleHookAnswerDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PutLifecycleHookAnswer, XmlParseError> {
        xml_util::start_element(tag_name, stack)?;

        let obj = PutLifecycleHookAnswer::default();

        xml_util::end_element(tag_name, stack)?;

        Ok(obj)
    }
}
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutLifecycleHookType {
    /// <p>The name of the Auto Scaling group.</p>
    pub auto_scaling_group_name: String,
    /// <p>Defines the action the Auto Scaling group should take when the lifecycle hook timeout elapses or if an unexpected failure occurs. This parameter can be either <code>CONTINUE</code> or <code>ABANDON</code>. The default value is <code>ABANDON</code>.</p>
    pub default_result: Option<String>,
    /// <p>The maximum time, in seconds, that can elapse before the lifecycle hook times out. The range is from <code>30</code> to <code>7200</code> seconds. The default value is <code>3600</code> seconds (1 hour).</p> <p>If the lifecycle hook times out, Amazon EC2 Auto Scaling performs the action that you specified in the <code>DefaultResult</code> parameter. You can prevent the lifecycle hook from timing out by calling the <a>RecordLifecycleActionHeartbeat</a> API.</p>
    pub heartbeat_timeout: Option<i64>,
    /// <p>The name of the lifecycle hook.</p>
    pub lifecycle_hook_name: String,
    /// <p>The instance state to which you want to attach the lifecycle hook. The valid values are:</p> <ul> <li> <p>autoscaling:EC2_INSTANCE_LAUNCHING</p> </li> <li> <p>autoscaling:EC2_INSTANCE_TERMINATING</p> </li> </ul> <p>Required for new lifecycle hooks, but optional when updating existing hooks.</p>
    pub lifecycle_transition: Option<String>,
    /// <p>Additional information that you want to include any time Amazon EC2 Auto Scaling sends a message to the notification target.</p>
    pub notification_metadata: Option<String>,
    /// <p>The ARN of the notification target that Amazon EC2 Auto Scaling uses to notify you when an instance is in the transition state for the lifecycle hook. This target can be either an SQS queue or an SNS topic.</p> <p>If you specify an empty string, this overrides the current ARN.</p> <p>This operation uses the JSON format when sending notifications to an Amazon SQS queue, and an email key-value pair format when sending notifications to an Amazon SNS topic.</p> <p>When you specify a notification target, Amazon EC2 Auto Scaling sends it a test message. Test messages contain the following additional key-value pair: <code>"Event": "autoscaling:TEST_NOTIFICATION"</code>.</p>
    pub notification_target_arn: Option<String>,
    /// <p>The ARN of the IAM role that allows the Auto Scaling group to publish to the specified notification target, for example, an Amazon SNS topic or an Amazon SQS queue.</p> <p>Required for new lifecycle hooks, but optional when updating existing hooks.</p>
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

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutNotificationConfigurationType {
    /// <p>The name of the Auto Scaling group.</p>
    pub auto_scaling_group_name: String,
    /// <p>The type of event that causes the notification to be sent. To query the notification types supported by Amazon EC2 Auto Scaling, call the <a>DescribeAutoScalingNotificationTypes</a> API.</p>
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

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutScalingPolicyType {
    /// <p>Specifies how the scaling adjustment is interpreted (for example, an absolute number or a percentage). The valid values are <code>ChangeInCapacity</code>, <code>ExactCapacity</code>, and <code>PercentChangeInCapacity</code>.</p> <p>Required if the policy type is <code>StepScaling</code> or <code>SimpleScaling</code>. For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/as-scaling-simple-step.html#as-scaling-adjustment">Scaling adjustment types</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    pub adjustment_type: Option<String>,
    /// <p>The name of the Auto Scaling group.</p>
    pub auto_scaling_group_name: String,
    /// <p>The duration of the policy's cooldown period, in seconds. When a cooldown period is specified here, it overrides the default cooldown period defined for the Auto Scaling group.</p> <p>Valid only if the policy type is <code>SimpleScaling</code>. For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/Cooldown.html">Scaling cooldowns for Amazon EC2 Auto Scaling</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    pub cooldown: Option<i64>,
    /// <p>Indicates whether the scaling policy is enabled or disabled. The default is enabled. For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/as-enable-disable-scaling-policy.html">Disabling a scaling policy for an Auto Scaling group</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    pub enabled: Option<bool>,
    /// <p>The estimated time, in seconds, until a newly launched instance can contribute to the CloudWatch metrics. If not provided, the default is to use the value from the default cooldown period for the Auto Scaling group.</p> <p>Valid only if the policy type is <code>TargetTrackingScaling</code> or <code>StepScaling</code>.</p>
    pub estimated_instance_warmup: Option<i64>,
    /// <p>The aggregation type for the CloudWatch metrics. The valid values are <code>Minimum</code>, <code>Maximum</code>, and <code>Average</code>. If the aggregation type is null, the value is treated as <code>Average</code>.</p> <p>Valid only if the policy type is <code>StepScaling</code>.</p>
    pub metric_aggregation_type: Option<String>,
    /// <p><p>The minimum value to scale by when the adjustment type is <code>PercentChangeInCapacity</code>. For example, suppose that you create a step scaling policy to scale out an Auto Scaling group by 25 percent and you specify a <code>MinAdjustmentMagnitude</code> of 2. If the group has 4 instances and the scaling policy is performed, 25 percent of 4 is 1. However, because you specified a <code>MinAdjustmentMagnitude</code> of 2, Amazon EC2 Auto Scaling scales out the group by 2 instances.</p> <p>Valid only if the policy type is <code>StepScaling</code> or <code>SimpleScaling</code>. For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/as-scaling-simple-step.html#as-scaling-adjustment">Scaling adjustment types</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p> <note> <p>Some Auto Scaling groups use instance weights. In this case, set the <code>MinAdjustmentMagnitude</code> to a value that is at least as large as your largest instance weight.</p> </note></p>
    pub min_adjustment_magnitude: Option<i64>,
    /// <p>Available for backward compatibility. Use <code>MinAdjustmentMagnitude</code> instead.</p>
    pub min_adjustment_step: Option<i64>,
    /// <p>The name of the policy.</p>
    pub policy_name: String,
    /// <p><p>One of the following policy types: </p> <ul> <li> <p> <code>TargetTrackingScaling</code> </p> </li> <li> <p> <code>StepScaling</code> </p> </li> <li> <p> <code>SimpleScaling</code> (default)</p> </li> </ul></p>
    pub policy_type: Option<String>,
    /// <p>The amount by which to scale, based on the specified adjustment type. A positive value adds to the current capacity while a negative number removes from the current capacity. For exact capacity, you must specify a positive value.</p> <p>Required if the policy type is <code>SimpleScaling</code>. (Not used with any other policy type.) </p>
    pub scaling_adjustment: Option<i64>,
    /// <p>A set of adjustments that enable you to scale based on the size of the alarm breach.</p> <p>Required if the policy type is <code>StepScaling</code>. (Not used with any other policy type.) </p>
    pub step_adjustments: Option<Vec<StepAdjustment>>,
    /// <p>A target tracking scaling policy. Includes support for predefined or customized metrics.</p> <p>The following predefined metrics are available:</p> <ul> <li> <p> <code>ASGAverageCPUUtilization</code> </p> </li> <li> <p> <code>ASGAverageNetworkIn</code> </p> </li> <li> <p> <code>ASGAverageNetworkOut</code> </p> </li> <li> <p> <code>ALBRequestCountPerTarget</code> </p> </li> </ul> <p>If you specify <code>ALBRequestCountPerTarget</code> for the metric, you must specify the <code>ResourceLabel</code> parameter with the <code>PredefinedMetricSpecification</code>.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/APIReference/API_TargetTrackingConfiguration.html">TargetTrackingConfiguration</a> in the <i>Amazon EC2 Auto Scaling API Reference</i>.</p> <p>Required if the policy type is <code>TargetTrackingScaling</code>.</p>
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
        if let Some(ref field_value) = obj.enabled {
            params.put(&format!("{}{}", prefix, "Enabled"), &field_value);
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

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutScheduledUpdateGroupActionType {
    /// <p>The name of the Auto Scaling group.</p>
    pub auto_scaling_group_name: String,
    /// <p>The desired capacity is the initial capacity of the Auto Scaling group after the scheduled action runs and the capacity it attempts to maintain. It can scale beyond this capacity if you add more scaling conditions. </p>
    pub desired_capacity: Option<i64>,
    /// <p>The date and time for the recurring schedule to end. Amazon EC2 Auto Scaling does not perform the action after this time.</p>
    pub end_time: Option<String>,
    /// <p>The maximum size of the Auto Scaling group.</p>
    pub max_size: Option<i64>,
    /// <p>The minimum size of the Auto Scaling group.</p>
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

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct RecordLifecycleActionHeartbeatAnswer {}

#[allow(dead_code)]
struct RecordLifecycleActionHeartbeatAnswerDeserializer;
impl RecordLifecycleActionHeartbeatAnswerDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<RecordLifecycleActionHeartbeatAnswer, XmlParseError> {
        xml_util::start_element(tag_name, stack)?;

        let obj = RecordLifecycleActionHeartbeatAnswer::default();

        xml_util::end_element(tag_name, stack)?;

        Ok(obj)
    }
}
#[derive(Clone, Debug, Default, PartialEq)]
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

/// <p>Describes information used to start an instance refresh. </p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RefreshPreferences {
    /// <p>The number of seconds until a newly launched instance is configured and ready to use. During this time, Amazon EC2 Auto Scaling does not immediately move on to the next replacement. The default is to use the value for the health check grace period defined for the group.</p>
    pub instance_warmup: Option<i64>,
    /// <p>The amount of capacity in the Auto Scaling group that must remain healthy during an instance refresh to allow the operation to continue, as a percentage of the desired capacity of the Auto Scaling group (rounded up to the nearest integer). The default is <code>90</code>. </p>
    pub min_healthy_percentage: Option<i64>,
}

/// Serialize `RefreshPreferences` contents to a `SignedRequest`.
struct RefreshPreferencesSerializer;
impl RefreshPreferencesSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &RefreshPreferences) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.instance_warmup {
            params.put(&format!("{}{}", prefix, "InstanceWarmup"), &field_value);
        }
        if let Some(ref field_value) = obj.min_healthy_percentage {
            params.put(
                &format!("{}{}", prefix, "MinHealthyPercentage"),
                &field_value,
            );
        }
    }
}

#[allow(dead_code)]
struct ResourceNameDeserializer;
impl ResourceNameDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}
#[allow(dead_code)]
struct ScalingActivityStatusCodeDeserializer;
impl ScalingActivityStatusCodeDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}
#[allow(dead_code)]
struct ScalingPoliciesDeserializer;
impl ScalingPoliciesDeserializer {
    #[allow(dead_code, unused_variables)]
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
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ScalingPolicy {
    /// <p>Specifies how the scaling adjustment is interpreted (for example, an absolute number or a percentage). The valid values are <code>ChangeInCapacity</code>, <code>ExactCapacity</code>, and <code>PercentChangeInCapacity</code>.</p>
    pub adjustment_type: Option<String>,
    /// <p>The CloudWatch alarms related to the policy.</p>
    pub alarms: Option<Vec<Alarm>>,
    /// <p>The name of the Auto Scaling group.</p>
    pub auto_scaling_group_name: Option<String>,
    /// <p>The duration of the policy's cooldown period, in seconds.</p>
    pub cooldown: Option<i64>,
    /// <p>Indicates whether the policy is enabled (<code>true</code>) or disabled (<code>false</code>).</p>
    pub enabled: Option<bool>,
    /// <p>The estimated time, in seconds, until a newly launched instance can contribute to the CloudWatch metrics.</p>
    pub estimated_instance_warmup: Option<i64>,
    /// <p>The aggregation type for the CloudWatch metrics. The valid values are <code>Minimum</code>, <code>Maximum</code>, and <code>Average</code>.</p>
    pub metric_aggregation_type: Option<String>,
    /// <p>The minimum value to scale by when the adjustment type is <code>PercentChangeInCapacity</code>. </p>
    pub min_adjustment_magnitude: Option<i64>,
    /// <p>Available for backward compatibility. Use <code>MinAdjustmentMagnitude</code> instead.</p>
    pub min_adjustment_step: Option<i64>,
    /// <p>The Amazon Resource Name (ARN) of the policy.</p>
    pub policy_arn: Option<String>,
    /// <p>The name of the scaling policy.</p>
    pub policy_name: Option<String>,
    /// <p>One of the following policy types: </p> <ul> <li> <p> <code>TargetTrackingScaling</code> </p> </li> <li> <p> <code>StepScaling</code> </p> </li> <li> <p> <code>SimpleScaling</code> (default)</p> </li> </ul> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/as-scaling-target-tracking.html">Target tracking scaling policies</a> and <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/as-scaling-simple-step.html">Step and simple scaling policies</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    pub policy_type: Option<String>,
    /// <p>The amount by which to scale, based on the specified adjustment type. A positive value adds to the current capacity while a negative number removes from the current capacity.</p>
    pub scaling_adjustment: Option<i64>,
    /// <p>A set of adjustments that enable you to scale based on the size of the alarm breach.</p>
    pub step_adjustments: Option<Vec<StepAdjustment>>,
    /// <p>A target tracking scaling policy.</p>
    pub target_tracking_configuration: Option<TargetTrackingConfiguration>,
}

#[allow(dead_code)]
struct ScalingPolicyDeserializer;
impl ScalingPolicyDeserializer {
    #[allow(dead_code, unused_variables)]
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
                "Enabled" => {
                    obj.enabled = Some(ScalingPolicyEnabledDeserializer::deserialize(
                        "Enabled", stack,
                    )?);
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
#[allow(dead_code)]
struct ScalingPolicyEnabledDeserializer;
impl ScalingPolicyEnabledDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<bool, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, |s| {
            Ok(bool::from_str(&s.to_lowercase()).unwrap())
        })
    }
}
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ScalingProcessQuery {
    /// <p>The name of the Auto Scaling group.</p>
    pub auto_scaling_group_name: String,
    /// <p>One or more of the following processes:</p> <ul> <li> <p> <code>Launch</code> </p> </li> <li> <p> <code>Terminate</code> </p> </li> <li> <p> <code>AddToLoadBalancer</code> </p> </li> <li> <p> <code>AlarmNotification</code> </p> </li> <li> <p> <code>AZRebalance</code> </p> </li> <li> <p> <code>HealthCheck</code> </p> </li> <li> <p> <code>InstanceRefresh</code> </p> </li> <li> <p> <code>ReplaceUnhealthy</code> </p> </li> <li> <p> <code>ScheduledActions</code> </p> </li> </ul> <p>If you omit this parameter, all processes are specified.</p>
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

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ScheduledActionsType {
    /// <p>A string that indicates that the response contains more items than can be returned in a single response. To receive additional items, specify this string for the <code>NextToken</code> value when requesting the next set of items. This value is null when there are no more items to return.</p>
    pub next_token: Option<String>,
    /// <p>The scheduled actions.</p>
    pub scheduled_update_group_actions: Option<Vec<ScheduledUpdateGroupAction>>,
}

#[allow(dead_code)]
struct ScheduledActionsTypeDeserializer;
impl ScheduledActionsTypeDeserializer {
    #[allow(dead_code, unused_variables)]
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
/// <p>Describes a scheduled scaling action.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ScheduledUpdateGroupAction {
    /// <p>The name of the Auto Scaling group.</p>
    pub auto_scaling_group_name: Option<String>,
    /// <p>The desired capacity is the initial capacity of the Auto Scaling group after the scheduled action runs and the capacity it attempts to maintain.</p>
    pub desired_capacity: Option<i64>,
    /// <p>The date and time in UTC for the recurring schedule to end. For example, <code>"2019-06-01T00:00:00Z"</code>. </p>
    pub end_time: Option<String>,
    /// <p>The maximum size of the Auto Scaling group.</p>
    pub max_size: Option<i64>,
    /// <p>The minimum size of the Auto Scaling group.</p>
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

#[allow(dead_code)]
struct ScheduledUpdateGroupActionDeserializer;
impl ScheduledUpdateGroupActionDeserializer {
    #[allow(dead_code, unused_variables)]
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
/// <p>Describes information used for one or more scheduled scaling action updates in a <a>BatchPutScheduledUpdateGroupAction</a> operation.</p> <p>When updating a scheduled scaling action, all optional parameters are left unchanged if not specified.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ScheduledUpdateGroupActionRequest {
    /// <p>The desired capacity is the initial capacity of the Auto Scaling group after the scheduled action runs and the capacity it attempts to maintain.</p>
    pub desired_capacity: Option<i64>,
    /// <p>The date and time for the recurring schedule to end. Amazon EC2 Auto Scaling does not perform the action after this time.</p>
    pub end_time: Option<String>,
    /// <p>The maximum size of the Auto Scaling group.</p>
    pub max_size: Option<i64>,
    /// <p>The minimum size of the Auto Scaling group.</p>
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

#[allow(dead_code)]
struct ScheduledUpdateGroupActionsDeserializer;
impl ScheduledUpdateGroupActionsDeserializer {
    #[allow(dead_code, unused_variables)]
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

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SetDesiredCapacityType {
    /// <p>The name of the Auto Scaling group.</p>
    pub auto_scaling_group_name: String,
    /// <p>The desired capacity is the initial capacity of the Auto Scaling group after this operation completes and the capacity it attempts to maintain.</p>
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

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SetInstanceHealthQuery {
    /// <p>The health status of the instance. Set to <code>Healthy</code> to have the instance remain in service. Set to <code>Unhealthy</code> to have the instance be out of service. Amazon EC2 Auto Scaling terminates and replaces the unhealthy instance.</p>
    pub health_status: String,
    /// <p>The ID of the instance.</p>
    pub instance_id: String,
    /// <p>If the Auto Scaling group of the specified instance has a <code>HealthCheckGracePeriod</code> specified for the group, by default, this call respects the grace period. Set this to <code>False</code>, to have the call not respect the grace period associated with the group.</p> <p>For more information about the health check grace period, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/APIReference/API_CreateAutoScalingGroup.html">CreateAutoScalingGroup</a> in the <i>Amazon EC2 Auto Scaling API Reference</i>.</p>
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

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct SetInstanceProtectionAnswer {}

#[allow(dead_code)]
struct SetInstanceProtectionAnswerDeserializer;
impl SetInstanceProtectionAnswerDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<SetInstanceProtectionAnswer, XmlParseError> {
        xml_util::start_element(tag_name, stack)?;

        let obj = SetInstanceProtectionAnswer::default();

        xml_util::end_element(tag_name, stack)?;

        Ok(obj)
    }
}
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SetInstanceProtectionQuery {
    /// <p>The name of the Auto Scaling group.</p>
    pub auto_scaling_group_name: String,
    /// <p>One or more instance IDs. You can specify up to 50 instances.</p>
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

#[allow(dead_code)]
struct SpotInstancePoolsDeserializer;
impl SpotInstancePoolsDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, |s| Ok(i64::from_str(&s).unwrap()))
    }
}
#[allow(dead_code)]
struct SpotPriceDeserializer;
impl SpotPriceDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct StartInstanceRefreshAnswer {
    /// <p>A unique ID for tracking the progress of the request.</p>
    pub instance_refresh_id: Option<String>,
}

#[allow(dead_code)]
struct StartInstanceRefreshAnswerDeserializer;
impl StartInstanceRefreshAnswerDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<StartInstanceRefreshAnswer, XmlParseError> {
        deserialize_elements::<_, StartInstanceRefreshAnswer, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "InstanceRefreshId" => {
                        obj.instance_refresh_id =
                            Some(XmlStringMaxLen255Deserializer::deserialize(
                                "InstanceRefreshId",
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
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartInstanceRefreshType {
    /// <p>The name of the Auto Scaling group.</p>
    pub auto_scaling_group_name: String,
    /// <p>Set of preferences associated with the instance refresh request.</p> <p>If not provided, the default values are used. For <code>MinHealthyPercentage</code>, the default value is <code>90</code>. For <code>InstanceWarmup</code>, the default is to use the value specified for the health check grace period for the Auto Scaling group.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/APIReference/API_RefreshPreferences.html">RefreshPreferences</a> in the <i>Amazon EC2 Auto Scaling API Reference</i>.</p>
    pub preferences: Option<RefreshPreferences>,
    /// <p>The strategy to use for the instance refresh. The only valid value is <code>Rolling</code>.</p> <p>A rolling update is an update that is applied to all instances in an Auto Scaling group until all instances have been updated. A rolling update can fail due to failed health checks or if instances are on standby or are protected from scale in. If the rolling update process fails, any instances that were already replaced are not rolled back to their previous configuration. </p>
    pub strategy: Option<String>,
}

/// Serialize `StartInstanceRefreshType` contents to a `SignedRequest`.
struct StartInstanceRefreshTypeSerializer;
impl StartInstanceRefreshTypeSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &StartInstanceRefreshType) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "AutoScalingGroupName"),
            &obj.auto_scaling_group_name,
        );
        if let Some(ref field_value) = obj.preferences {
            RefreshPreferencesSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Preferences"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.strategy {
            params.put(&format!("{}{}", prefix, "Strategy"), &field_value);
        }
    }
}

/// <p>Describes information used to create a step adjustment for a step scaling policy.</p> <p>For the following examples, suppose that you have an alarm with a breach threshold of 50:</p> <ul> <li> <p>To trigger the adjustment when the metric is greater than or equal to 50 and less than 60, specify a lower bound of 0 and an upper bound of 10.</p> </li> <li> <p>To trigger the adjustment when the metric is greater than 40 and less than or equal to 50, specify a lower bound of -10 and an upper bound of 0.</p> </li> </ul> <p>There are a few rules for the step adjustments for your step policy:</p> <ul> <li> <p>The ranges of your step adjustments can't overlap or have a gap.</p> </li> <li> <p>At most, one step adjustment can have a null lower bound. If one step adjustment has a negative lower bound, then there must be a step adjustment with a null lower bound.</p> </li> <li> <p>At most, one step adjustment can have a null upper bound. If one step adjustment has a positive upper bound, then there must be a step adjustment with a null upper bound.</p> </li> <li> <p>The upper and lower bound can't be null in the same step adjustment.</p> </li> </ul> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/as-scaling-simple-step.html#as-scaling-steps">Step adjustments</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
#[derive(Clone, Debug, Default, PartialEq)]
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

#[allow(dead_code)]
struct StepAdjustmentDeserializer;
impl StepAdjustmentDeserializer {
    #[allow(dead_code, unused_variables)]
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

#[allow(dead_code)]
struct StepAdjustmentsDeserializer;
impl StepAdjustmentsDeserializer {
    #[allow(dead_code, unused_variables)]
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

/// <p>Describes an auto scaling process that has been suspended.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/as-suspend-resume-processes.html#process-types">Scaling processes</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct SuspendedProcess {
    /// <p>The name of the suspended process.</p>
    pub process_name: Option<String>,
    /// <p>The reason that the process was suspended.</p>
    pub suspension_reason: Option<String>,
}

#[allow(dead_code)]
struct SuspendedProcessDeserializer;
impl SuspendedProcessDeserializer {
    #[allow(dead_code, unused_variables)]
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
#[allow(dead_code)]
struct SuspendedProcessesDeserializer;
impl SuspendedProcessesDeserializer {
    #[allow(dead_code, unused_variables)]
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
#[derive(Clone, Debug, Default, PartialEq)]
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
#[derive(Clone, Debug, Default, PartialEq)]
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
#[allow(dead_code)]
struct TagDescriptionListDeserializer;
impl TagDescriptionListDeserializer {
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
#[allow(dead_code)]
struct TagValueDeserializer;
impl TagValueDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
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

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct TagsType {
    /// <p>A string that indicates that the response contains more items than can be returned in a single response. To receive additional items, specify this string for the <code>NextToken</code> value when requesting the next set of items. This value is null when there are no more items to return.</p>
    pub next_token: Option<String>,
    /// <p>One or more tags.</p>
    pub tags: Option<Vec<TagDescription>>,
}

#[allow(dead_code)]
struct TagsTypeDeserializer;
impl TagsTypeDeserializer {
    #[allow(dead_code, unused_variables)]
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
#[allow(dead_code)]
struct TargetGroupARNsDeserializer;
impl TargetGroupARNsDeserializer {
    #[allow(dead_code, unused_variables)]
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
#[derive(Clone, Debug, Default, PartialEq)]
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

#[allow(dead_code)]
struct TargetTrackingConfigurationDeserializer;
impl TargetTrackingConfigurationDeserializer {
    #[allow(dead_code, unused_variables)]
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

#[derive(Clone, Debug, Default, PartialEq)]
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

#[allow(dead_code)]
struct TerminationPoliciesDeserializer;
impl TerminationPoliciesDeserializer {
    #[allow(dead_code, unused_variables)]
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

#[allow(dead_code)]
struct TimestampTypeDeserializer;
impl TimestampTypeDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateAutoScalingGroupType {
    /// <p>The name of the Auto Scaling group.</p>
    pub auto_scaling_group_name: String,
    /// <p>One or more Availability Zones for the group.</p>
    pub availability_zones: Option<Vec<String>>,
    /// <p>Enables or disables Capacity Rebalancing. For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/capacity-rebalance.html">Amazon EC2 Auto Scaling Capacity Rebalancing</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    pub capacity_rebalance: Option<bool>,
    /// <p>The amount of time, in seconds, after a scaling activity completes before another scaling activity can start. The default value is <code>300</code>. This setting applies when using simple scaling policies, but not when using other scaling policies or scheduled scaling. For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/Cooldown.html">Scaling cooldowns for Amazon EC2 Auto Scaling</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    pub default_cooldown: Option<i64>,
    /// <p>The desired capacity is the initial capacity of the Auto Scaling group after this operation completes and the capacity it attempts to maintain. This number must be greater than or equal to the minimum size of the group and less than or equal to the maximum size of the group.</p>
    pub desired_capacity: Option<i64>,
    /// <p>The amount of time, in seconds, that Amazon EC2 Auto Scaling waits before checking the health status of an EC2 instance that has come into service. The default value is <code>0</code>. For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/healthcheck.html#health-check-grace-period">Health check grace period</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p> <p>Conditional: Required if you are adding an <code>ELB</code> health check.</p>
    pub health_check_grace_period: Option<i64>,
    /// <p>The service to use for the health checks. The valid values are <code>EC2</code> and <code>ELB</code>. If you configure an Auto Scaling group to use ELB health checks, it considers the instance unhealthy if it fails either the EC2 status checks or the load balancer health checks.</p>
    pub health_check_type: Option<String>,
    /// <p>The name of the launch configuration. If you specify <code>LaunchConfigurationName</code> in your update request, you can't specify <code>LaunchTemplate</code> or <code>MixedInstancesPolicy</code>.</p>
    pub launch_configuration_name: Option<String>,
    /// <p>The launch template and version to use to specify the updates. If you specify <code>LaunchTemplate</code> in your update request, you can't specify <code>LaunchConfigurationName</code> or <code>MixedInstancesPolicy</code>.</p>
    pub launch_template: Option<LaunchTemplateSpecification>,
    /// <p>The maximum amount of time, in seconds, that an instance can be in service. The default is null. If specified, the value must be either 0 or a number equal to or greater than 86,400 seconds (1 day). To clear a previously set value, specify a new value of 0. For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/asg-max-instance-lifetime.html">Replacing Auto Scaling instances based on maximum instance lifetime</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    pub max_instance_lifetime: Option<i64>,
    /// <p><p>The maximum size of the Auto Scaling group.</p> <note> <p>With a mixed instances policy that uses instance weighting, Amazon EC2 Auto Scaling may need to go above <code>MaxSize</code> to meet your capacity requirements. In this event, Amazon EC2 Auto Scaling will never go above <code>MaxSize</code> by more than your largest instance weight (weights that define how many units each instance contributes to the desired capacity of the group).</p> </note></p>
    pub max_size: Option<i64>,
    /// <p>The minimum size of the Auto Scaling group.</p>
    pub min_size: Option<i64>,
    /// <p>An embedded object that specifies a mixed instances policy. When you make changes to an existing policy, all optional parameters are left unchanged if not specified. For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/asg-purchase-options.html">Auto Scaling groups with multiple instance types and purchase options</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    pub mixed_instances_policy: Option<MixedInstancesPolicy>,
    /// <p>Indicates whether newly launched instances are protected from termination by Amazon EC2 Auto Scaling when scaling in. For more information about preventing instances from terminating on scale in, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/as-instance-termination.html#instance-protection">Instance scale-in protection</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    pub new_instances_protected_from_scale_in: Option<bool>,
    /// <p>The name of an existing placement group into which to launch your instances, if any. A placement group is a logical grouping of instances within a single Availability Zone. You cannot specify multiple Availability Zones and a placement group. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/placement-groups.html">Placement Groups</a> in the <i>Amazon EC2 User Guide for Linux Instances</i>.</p>
    pub placement_group: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the service-linked role that the Auto Scaling group uses to call other AWS services on your behalf. For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/autoscaling-service-linked-role.html">Service-linked roles</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    pub service_linked_role_arn: Option<String>,
    /// <p>A policy or a list of policies that are used to select the instances to terminate. The policies are executed in the order that you list them. For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/as-instance-termination.html">Controlling which Auto Scaling instances terminate during scale in</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    pub termination_policies: Option<Vec<String>>,
    /// <p>A comma-separated list of subnet IDs for a virtual private cloud (VPC). If you specify <code>VPCZoneIdentifier</code> with <code>AvailabilityZones</code>, the subnets that you specify for this parameter must reside in those Availability Zones.</p>
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
        if let Some(ref field_value) = obj.capacity_rebalance {
            params.put(&format!("{}{}", prefix, "CapacityRebalance"), &field_value);
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

#[allow(dead_code)]
struct XmlStringDeserializer;
impl XmlStringDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}
#[allow(dead_code)]
struct XmlStringMaxLen1023Deserializer;
impl XmlStringMaxLen1023Deserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}
#[allow(dead_code)]
struct XmlStringMaxLen1600Deserializer;
impl XmlStringMaxLen1600Deserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}
#[allow(dead_code)]
struct XmlStringMaxLen19Deserializer;
impl XmlStringMaxLen19Deserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}
#[allow(dead_code)]
struct XmlStringMaxLen2047Deserializer;
impl XmlStringMaxLen2047Deserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}
#[allow(dead_code)]
struct XmlStringMaxLen255Deserializer;
impl XmlStringMaxLen255Deserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}
#[allow(dead_code)]
struct XmlStringMaxLen32Deserializer;
impl XmlStringMaxLen32Deserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}
#[allow(dead_code)]
struct XmlStringMaxLen511Deserializer;
impl XmlStringMaxLen511Deserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}
#[allow(dead_code)]
struct XmlStringMaxLen64Deserializer;
impl XmlStringMaxLen64Deserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}
#[allow(dead_code)]
struct XmlStringUserDataDeserializer;
impl XmlStringUserDataDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
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
        xml_util::start_element("ErrorResponse", stack)?;
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
        xml_util::start_element("ErrorResponse", stack)?;
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
        xml_util::start_element("ErrorResponse", stack)?;
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
        xml_util::start_element("ErrorResponse", stack)?;
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
    /// <p>You have already reached a limit for your Amazon EC2 Auto Scaling resources (for example, Auto Scaling groups, launch configurations, or lifecycle hooks). For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/APIReference/API_DescribeAccountLimits.html">DescribeAccountLimits</a> in the <i>Amazon EC2 Auto Scaling API Reference</i>.</p>
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
        xml_util::start_element("ErrorResponse", stack)?;
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
/// Errors returned by CancelInstanceRefresh
#[derive(Debug, PartialEq)]
pub enum CancelInstanceRefreshError {
    /// <p>The request failed because an active instance refresh for the specified Auto Scaling group was not found. </p>
    ActiveInstanceRefreshNotFoundFault(String),
    /// <p>You have already reached a limit for your Amazon EC2 Auto Scaling resources (for example, Auto Scaling groups, launch configurations, or lifecycle hooks). For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/APIReference/API_DescribeAccountLimits.html">DescribeAccountLimits</a> in the <i>Amazon EC2 Auto Scaling API Reference</i>.</p>
    LimitExceededFault(String),
    /// <p>You already have a pending update to an Amazon EC2 Auto Scaling resource (for example, an Auto Scaling group, instance, or load balancer).</p>
    ResourceContentionFault(String),
}

impl CancelInstanceRefreshError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CancelInstanceRefreshError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "ActiveInstanceRefreshNotFound" => {
                        return RusotoError::Service(
                            CancelInstanceRefreshError::ActiveInstanceRefreshNotFoundFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "LimitExceeded" => {
                        return RusotoError::Service(
                            CancelInstanceRefreshError::LimitExceededFault(parsed_error.message),
                        )
                    }
                    "ResourceContention" => {
                        return RusotoError::Service(
                            CancelInstanceRefreshError::ResourceContentionFault(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for CancelInstanceRefreshError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CancelInstanceRefreshError::ActiveInstanceRefreshNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
            CancelInstanceRefreshError::LimitExceededFault(ref cause) => write!(f, "{}", cause),
            CancelInstanceRefreshError::ResourceContentionFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for CancelInstanceRefreshError {}
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
        xml_util::start_element("ErrorResponse", stack)?;
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
    /// <p>You have already reached a limit for your Amazon EC2 Auto Scaling resources (for example, Auto Scaling groups, launch configurations, or lifecycle hooks). For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/APIReference/API_DescribeAccountLimits.html">DescribeAccountLimits</a> in the <i>Amazon EC2 Auto Scaling API Reference</i>.</p>
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
        xml_util::start_element("ErrorResponse", stack)?;
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
    /// <p>You have already reached a limit for your Amazon EC2 Auto Scaling resources (for example, Auto Scaling groups, launch configurations, or lifecycle hooks). For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/APIReference/API_DescribeAccountLimits.html">DescribeAccountLimits</a> in the <i>Amazon EC2 Auto Scaling API Reference</i>.</p>
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
        xml_util::start_element("ErrorResponse", stack)?;
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
    /// <p>You have already reached a limit for your Amazon EC2 Auto Scaling resources (for example, Auto Scaling groups, launch configurations, or lifecycle hooks). For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/APIReference/API_DescribeAccountLimits.html">DescribeAccountLimits</a> in the <i>Amazon EC2 Auto Scaling API Reference</i>.</p>
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
        xml_util::start_element("ErrorResponse", stack)?;
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
        xml_util::start_element("ErrorResponse", stack)?;
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
        xml_util::start_element("ErrorResponse", stack)?;
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
        xml_util::start_element("ErrorResponse", stack)?;
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
        xml_util::start_element("ErrorResponse", stack)?;
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
        xml_util::start_element("ErrorResponse", stack)?;
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
        xml_util::start_element("ErrorResponse", stack)?;
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
        xml_util::start_element("ErrorResponse", stack)?;
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
        xml_util::start_element("ErrorResponse", stack)?;
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
        xml_util::start_element("ErrorResponse", stack)?;
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
        xml_util::start_element("ErrorResponse", stack)?;
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
        xml_util::start_element("ErrorResponse", stack)?;
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
        xml_util::start_element("ErrorResponse", stack)?;
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
/// Errors returned by DescribeInstanceRefreshes
#[derive(Debug, PartialEq)]
pub enum DescribeInstanceRefreshesError {
    /// <p>The <code>NextToken</code> value is not valid.</p>
    InvalidNextToken(String),
    /// <p>You already have a pending update to an Amazon EC2 Auto Scaling resource (for example, an Auto Scaling group, instance, or load balancer).</p>
    ResourceContentionFault(String),
}

impl DescribeInstanceRefreshesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeInstanceRefreshesError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidNextToken" => {
                        return RusotoError::Service(
                            DescribeInstanceRefreshesError::InvalidNextToken(parsed_error.message),
                        )
                    }
                    "ResourceContention" => {
                        return RusotoError::Service(
                            DescribeInstanceRefreshesError::ResourceContentionFault(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DescribeInstanceRefreshesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeInstanceRefreshesError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            DescribeInstanceRefreshesError::ResourceContentionFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeInstanceRefreshesError {}
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
        xml_util::start_element("ErrorResponse", stack)?;
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
        xml_util::start_element("ErrorResponse", stack)?;
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
        xml_util::start_element("ErrorResponse", stack)?;
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
        xml_util::start_element("ErrorResponse", stack)?;
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
        xml_util::start_element("ErrorResponse", stack)?;
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
        xml_util::start_element("ErrorResponse", stack)?;
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
        xml_util::start_element("ErrorResponse", stack)?;
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
        xml_util::start_element("ErrorResponse", stack)?;
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
        xml_util::start_element("ErrorResponse", stack)?;
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
        xml_util::start_element("ErrorResponse", stack)?;
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
        xml_util::start_element("ErrorResponse", stack)?;
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
        xml_util::start_element("ErrorResponse", stack)?;
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
        xml_util::start_element("ErrorResponse", stack)?;
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
        xml_util::start_element("ErrorResponse", stack)?;
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
        xml_util::start_element("ErrorResponse", stack)?;
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
        xml_util::start_element("ErrorResponse", stack)?;
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
        xml_util::start_element("ErrorResponse", stack)?;
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
        xml_util::start_element("ErrorResponse", stack)?;
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
        xml_util::start_element("ErrorResponse", stack)?;
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
        xml_util::start_element("ErrorResponse", stack)?;
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
        xml_util::start_element("ErrorResponse", stack)?;
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
    /// <p>You have already reached a limit for your Amazon EC2 Auto Scaling resources (for example, Auto Scaling groups, launch configurations, or lifecycle hooks). For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/APIReference/API_DescribeAccountLimits.html">DescribeAccountLimits</a> in the <i>Amazon EC2 Auto Scaling API Reference</i>.</p>
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
        xml_util::start_element("ErrorResponse", stack)?;
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
    /// <p>You have already reached a limit for your Amazon EC2 Auto Scaling resources (for example, Auto Scaling groups, launch configurations, or lifecycle hooks). For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/APIReference/API_DescribeAccountLimits.html">DescribeAccountLimits</a> in the <i>Amazon EC2 Auto Scaling API Reference</i>.</p>
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
        xml_util::start_element("ErrorResponse", stack)?;
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
    /// <p>You have already reached a limit for your Amazon EC2 Auto Scaling resources (for example, Auto Scaling groups, launch configurations, or lifecycle hooks). For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/APIReference/API_DescribeAccountLimits.html">DescribeAccountLimits</a> in the <i>Amazon EC2 Auto Scaling API Reference</i>.</p>
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
        xml_util::start_element("ErrorResponse", stack)?;
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
    /// <p>You have already reached a limit for your Amazon EC2 Auto Scaling resources (for example, Auto Scaling groups, launch configurations, or lifecycle hooks). For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/APIReference/API_DescribeAccountLimits.html">DescribeAccountLimits</a> in the <i>Amazon EC2 Auto Scaling API Reference</i>.</p>
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
        xml_util::start_element("ErrorResponse", stack)?;
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
        xml_util::start_element("ErrorResponse", stack)?;
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
        xml_util::start_element("ErrorResponse", stack)?;
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
        xml_util::start_element("ErrorResponse", stack)?;
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
        xml_util::start_element("ErrorResponse", stack)?;
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
    /// <p>You have already reached a limit for your Amazon EC2 Auto Scaling resources (for example, Auto Scaling groups, launch configurations, or lifecycle hooks). For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/APIReference/API_DescribeAccountLimits.html">DescribeAccountLimits</a> in the <i>Amazon EC2 Auto Scaling API Reference</i>.</p>
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
        xml_util::start_element("ErrorResponse", stack)?;
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
/// Errors returned by StartInstanceRefresh
#[derive(Debug, PartialEq)]
pub enum StartInstanceRefreshError {
    /// <p>The request failed because an active instance refresh operation already exists for the specified Auto Scaling group.</p>
    InstanceRefreshInProgressFault(String),
    /// <p>You have already reached a limit for your Amazon EC2 Auto Scaling resources (for example, Auto Scaling groups, launch configurations, or lifecycle hooks). For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/APIReference/API_DescribeAccountLimits.html">DescribeAccountLimits</a> in the <i>Amazon EC2 Auto Scaling API Reference</i>.</p>
    LimitExceededFault(String),
    /// <p>You already have a pending update to an Amazon EC2 Auto Scaling resource (for example, an Auto Scaling group, instance, or load balancer).</p>
    ResourceContentionFault(String),
}

impl StartInstanceRefreshError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartInstanceRefreshError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InstanceRefreshInProgress" => {
                        return RusotoError::Service(
                            StartInstanceRefreshError::InstanceRefreshInProgressFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "LimitExceeded" => {
                        return RusotoError::Service(StartInstanceRefreshError::LimitExceededFault(
                            parsed_error.message,
                        ))
                    }
                    "ResourceContention" => {
                        return RusotoError::Service(
                            StartInstanceRefreshError::ResourceContentionFault(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for StartInstanceRefreshError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartInstanceRefreshError::InstanceRefreshInProgressFault(ref cause) => {
                write!(f, "{}", cause)
            }
            StartInstanceRefreshError::LimitExceededFault(ref cause) => write!(f, "{}", cause),
            StartInstanceRefreshError::ResourceContentionFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StartInstanceRefreshError {}
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
        xml_util::start_element("ErrorResponse", stack)?;
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
        xml_util::start_element("ErrorResponse", stack)?;
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
        xml_util::start_element("ErrorResponse", stack)?;
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
    /// <p>Attaches one or more EC2 instances to the specified Auto Scaling group.</p> <p>When you attach instances, Amazon EC2 Auto Scaling increases the desired capacity of the group by the number of instances being attached. If the number of instances being attached plus the desired capacity of the group exceeds the maximum size of the group, the operation fails.</p> <p>If there is a Classic Load Balancer attached to your Auto Scaling group, the instances are also registered with the load balancer. If there are target groups attached to your Auto Scaling group, the instances are also registered with the target groups.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/attach-instance-asg.html">Attach EC2 instances to your Auto Scaling group</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    async fn attach_instances(
        &self,
        input: AttachInstancesQuery,
    ) -> Result<(), RusotoError<AttachInstancesError>>;

    /// <p>Attaches one or more target groups to the specified Auto Scaling group.</p> <p>This operation is used with the following load balancer types: </p> <ul> <li> <p> Application Load Balancer - Operates at the application layer (layer 7) and supports HTTP and HTTPS. </p> </li> <li> <p> Network Load Balancer - Operates at the transport layer (layer 4) and supports TCP, TLS, and UDP. </p> </li> <li> <p> Gateway Load Balancer - Operates at the network layer (layer 3).</p> </li> </ul> <p>To describe the target groups for an Auto Scaling group, call the <a>DescribeLoadBalancerTargetGroups</a> API. To detach the target group from the Auto Scaling group, call the <a>DetachLoadBalancerTargetGroups</a> API.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/autoscaling-load-balancer.html">Elastic Load Balancing and Amazon EC2 Auto Scaling</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>. </p>
    async fn attach_load_balancer_target_groups(
        &self,
        input: AttachLoadBalancerTargetGroupsType,
    ) -> Result<
        AttachLoadBalancerTargetGroupsResultType,
        RusotoError<AttachLoadBalancerTargetGroupsError>,
    >;

    /// <p><note> <p>To attach an Application Load Balancer, Network Load Balancer, or Gateway Load Balancer, use the <a>AttachLoadBalancerTargetGroups</a> API operation instead.</p> </note> <p>Attaches one or more Classic Load Balancers to the specified Auto Scaling group. Amazon EC2 Auto Scaling registers the running instances with these Classic Load Balancers.</p> <p>To describe the load balancers for an Auto Scaling group, call the <a>DescribeLoadBalancers</a> API. To detach the load balancer from the Auto Scaling group, call the <a>DetachLoadBalancers</a> API.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/autoscaling-load-balancer.html">Elastic Load Balancing and Amazon EC2 Auto Scaling</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>. </p></p>
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

    /// <p>Cancels an instance refresh operation in progress. Cancellation does not roll back any replacements that have already been completed, but it prevents new replacements from being started. </p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/asg-instance-refresh.html">Replacing Auto Scaling Instances Based on an Instance Refresh</a>.</p>
    async fn cancel_instance_refresh(
        &self,
        input: CancelInstanceRefreshType,
    ) -> Result<CancelInstanceRefreshAnswer, RusotoError<CancelInstanceRefreshError>>;

    /// <p>Completes the lifecycle action for the specified token or instance with the specified result.</p> <p>This step is a part of the procedure for adding a lifecycle hook to an Auto Scaling group:</p> <ol> <li> <p>(Optional) Create a Lambda function and a rule that allows CloudWatch Events to invoke your Lambda function when Amazon EC2 Auto Scaling launches or terminates instances.</p> </li> <li> <p>(Optional) Create a notification target and an IAM role. The target can be either an Amazon SQS queue or an Amazon SNS topic. The role allows Amazon EC2 Auto Scaling to publish lifecycle notifications to the target.</p> </li> <li> <p>Create the lifecycle hook. Specify whether the hook is used when the instances launch or terminate.</p> </li> <li> <p>If you need more time, record the lifecycle action heartbeat to keep the instance in a pending state.</p> </li> <li> <p> <b>If you finish before the timeout period ends, complete the lifecycle action.</b> </p> </li> </ol> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/lifecycle-hooks.html">Amazon EC2 Auto Scaling lifecycle hooks</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    async fn complete_lifecycle_action(
        &self,
        input: CompleteLifecycleActionType,
    ) -> Result<CompleteLifecycleActionAnswer, RusotoError<CompleteLifecycleActionError>>;

    /// <p>Creates an Auto Scaling group with the specified name and attributes. </p> <p>If you exceed your maximum limit of Auto Scaling groups, the call fails. To query this limit, call the <a>DescribeAccountLimits</a> API. For information about updating this limit, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/as-account-limits.html">Amazon EC2 Auto Scaling service quotas</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p> <p>For introductory exercises for creating an Auto Scaling group, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/GettingStartedTutorial.html">Getting started with Amazon EC2 Auto Scaling</a> and <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/as-register-lbs-with-asg.html">Tutorial: Set up a scaled and load-balanced application</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>. For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/AutoScalingGroup.html">Auto Scaling groups</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p> <p>Every Auto Scaling group has three size parameters (<code>DesiredCapacity</code>, <code>MaxSize</code>, and <code>MinSize</code>). Usually, you set these sizes based on a specific number of instances. However, if you configure a mixed instances policy that defines weights for the instance types, you must specify these sizes with the same units that you use for weighting instances.</p>
    async fn create_auto_scaling_group(
        &self,
        input: CreateAutoScalingGroupType,
    ) -> Result<(), RusotoError<CreateAutoScalingGroupError>>;

    /// <p>Creates a launch configuration.</p> <p>If you exceed your maximum limit of launch configurations, the call fails. To query this limit, call the <a>DescribeAccountLimits</a> API. For information about updating this limit, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/as-account-limits.html">Amazon EC2 Auto Scaling service quotas</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/LaunchConfiguration.html">Launch configurations</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    async fn create_launch_configuration(
        &self,
        input: CreateLaunchConfigurationType,
    ) -> Result<(), RusotoError<CreateLaunchConfigurationError>>;

    /// <p>Creates or updates tags for the specified Auto Scaling group.</p> <p>When you specify a tag with a key that already exists, the operation overwrites the previous tag definition, and you do not get an error message.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/autoscaling-tagging.html">Tagging Auto Scaling groups and instances</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    async fn create_or_update_tags(
        &self,
        input: CreateOrUpdateTagsType,
    ) -> Result<(), RusotoError<CreateOrUpdateTagsError>>;

    /// <p>Deletes the specified Auto Scaling group.</p> <p>If the group has instances or scaling activities in progress, you must specify the option to force the deletion in order for it to succeed.</p> <p>If the group has policies, deleting the group deletes the policies, the underlying alarm actions, and any alarm that no longer has an associated action.</p> <p>To remove instances from the Auto Scaling group before deleting it, call the <a>DetachInstances</a> API with the list of instances and the option to decrement the desired capacity. This ensures that Amazon EC2 Auto Scaling does not launch replacement instances.</p> <p>To terminate all instances before deleting the Auto Scaling group, call the <a>UpdateAutoScalingGroup</a> API and set the minimum size and desired capacity of the Auto Scaling group to zero.</p>
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

    /// <p>Deletes the specified scaling policy.</p> <p>Deleting either a step scaling policy or a simple scaling policy deletes the underlying alarm action, but does not delete the alarm, even if it no longer has an associated action.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/deleting-scaling-policy.html">Deleting a scaling policy</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
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

    /// <p>Describes the current Amazon EC2 Auto Scaling resource quotas for your AWS account.</p> <p>For information about requesting an increase, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/as-account-limits.html">Amazon EC2 Auto Scaling service quotas</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    async fn describe_account_limits(
        &self,
    ) -> Result<DescribeAccountLimitsAnswer, RusotoError<DescribeAccountLimitsError>>;

    /// <p><p>Describes the available adjustment types for Amazon EC2 Auto Scaling scaling policies. These settings apply to step scaling policies and simple scaling policies; they do not apply to target tracking scaling policies.</p> <p>The following adjustment types are supported:</p> <ul> <li> <p>ChangeInCapacity</p> </li> <li> <p>ExactCapacity</p> </li> <li> <p>PercentChangeInCapacity</p> </li> </ul></p>
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

    /// <p>Describes one or more instance refreshes.</p> <p>You can determine the status of a request by looking at the <code>Status</code> parameter. The following are the possible statuses: </p> <ul> <li> <p> <code>Pending</code> - The request was created, but the operation has not started.</p> </li> <li> <p> <code>InProgress</code> - The operation is in progress.</p> </li> <li> <p> <code>Successful</code> - The operation completed successfully.</p> </li> <li> <p> <code>Failed</code> - The operation failed to complete. You can troubleshoot using the status reason and the scaling activities. </p> </li> <li> <p> <code>Cancelling</code> - An ongoing operation is being cancelled. Cancellation does not roll back any replacements that have already been completed, but it prevents new replacements from being started. </p> </li> <li> <p> <code>Cancelled</code> - The operation is cancelled. </p> </li> </ul> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/asg-instance-refresh.html">Replacing Auto Scaling Instances Based on an Instance Refresh</a>.</p>
    async fn describe_instance_refreshes(
        &self,
        input: DescribeInstanceRefreshesType,
    ) -> Result<DescribeInstanceRefreshesAnswer, RusotoError<DescribeInstanceRefreshesError>>;

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

    /// <p>Describes the load balancers for the specified Auto Scaling group.</p> <p>This operation describes only Classic Load Balancers. If you have Application Load Balancers, Network Load Balancers, or Gateway Load Balancers, use the <a>DescribeLoadBalancerTargetGroups</a> API instead.</p>
    async fn describe_load_balancers(
        &self,
        input: DescribeLoadBalancersRequest,
    ) -> Result<DescribeLoadBalancersResponse, RusotoError<DescribeLoadBalancersError>>;

    /// <p>Describes the available CloudWatch metrics for Amazon EC2 Auto Scaling.</p> <p>The <code>GroupStandbyInstances</code> metric is not returned by default. You must explicitly request this metric when calling the <a>EnableMetricsCollection</a> API.</p>
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

    /// <p>Describes the scaling process types for use with the <a>ResumeProcesses</a> and <a>SuspendProcesses</a> APIs.</p>
    async fn describe_scaling_process_types(
        &self,
    ) -> Result<ProcessesType, RusotoError<DescribeScalingProcessTypesError>>;

    /// <p>Describes the actions scheduled for your Auto Scaling group that haven't run or that have not reached their end time. To describe the actions that have already run, call the <a>DescribeScalingActivities</a> API.</p>
    async fn describe_scheduled_actions(
        &self,
        input: DescribeScheduledActionsType,
    ) -> Result<ScheduledActionsType, RusotoError<DescribeScheduledActionsError>>;

    /// <p>Describes the specified tags.</p> <p>You can use filters to limit the results. For example, you can query for the tags for a specific Auto Scaling group. You can specify multiple values for a filter. A tag must match at least one of the specified values for it to be included in the results.</p> <p>You can also specify multiple filters. The result includes information for a particular tag only if it matches all the filters. If there's no match, no special message is returned.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/autoscaling-tagging.html">Tagging Auto Scaling groups and instances</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    async fn describe_tags(
        &self,
        input: DescribeTagsType,
    ) -> Result<TagsType, RusotoError<DescribeTagsError>>;

    /// <p>Describes the termination policies supported by Amazon EC2 Auto Scaling.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/as-instance-termination.html">Controlling which Auto Scaling instances terminate during scale in</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    async fn describe_termination_policy_types(
        &self,
    ) -> Result<
        DescribeTerminationPolicyTypesAnswer,
        RusotoError<DescribeTerminationPolicyTypesError>,
    >;

    /// <p>Removes one or more instances from the specified Auto Scaling group.</p> <p>After the instances are detached, you can manage them independent of the Auto Scaling group.</p> <p>If you do not specify the option to decrement the desired capacity, Amazon EC2 Auto Scaling launches instances to replace the ones that are detached.</p> <p>If there is a Classic Load Balancer attached to the Auto Scaling group, the instances are deregistered from the load balancer. If there are target groups attached to the Auto Scaling group, the instances are deregistered from the target groups.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/detach-instance-asg.html">Detach EC2 instances from your Auto Scaling group</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
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

    /// <p>Detaches one or more Classic Load Balancers from the specified Auto Scaling group.</p> <p>This operation detaches only Classic Load Balancers. If you have Application Load Balancers, Network Load Balancers, or Gateway Load Balancers, use the <a>DetachLoadBalancerTargetGroups</a> API instead.</p> <p>When you detach a load balancer, it enters the <code>Removing</code> state while deregistering the instances in the group. When all instances are deregistered, then you can no longer describe the load balancer using the <a>DescribeLoadBalancers</a> API call. The instances remain running.</p>
    async fn detach_load_balancers(
        &self,
        input: DetachLoadBalancersType,
    ) -> Result<DetachLoadBalancersResultType, RusotoError<DetachLoadBalancersError>>;

    /// <p>Disables group metrics for the specified Auto Scaling group.</p>
    async fn disable_metrics_collection(
        &self,
        input: DisableMetricsCollectionQuery,
    ) -> Result<(), RusotoError<DisableMetricsCollectionError>>;

    /// <p>Enables group metrics for the specified Auto Scaling group. For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/as-instance-monitoring.html">Monitoring CloudWatch metrics for your Auto Scaling groups and instances</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    async fn enable_metrics_collection(
        &self,
        input: EnableMetricsCollectionQuery,
    ) -> Result<(), RusotoError<EnableMetricsCollectionError>>;

    /// <p>Moves the specified instances into the standby state.</p> <p>If you choose to decrement the desired capacity of the Auto Scaling group, the instances can enter standby as long as the desired capacity of the Auto Scaling group after the instances are placed into standby is equal to or greater than the minimum capacity of the group.</p> <p>If you choose not to decrement the desired capacity of the Auto Scaling group, the Auto Scaling group launches new instances to replace the instances on standby.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/as-enter-exit-standby.html">Temporarily removing instances from your Auto Scaling group</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    async fn enter_standby(
        &self,
        input: EnterStandbyQuery,
    ) -> Result<EnterStandbyAnswer, RusotoError<EnterStandbyError>>;

    /// <p>Executes the specified policy. This can be useful for testing the design of your scaling policy.</p>
    async fn execute_policy(
        &self,
        input: ExecutePolicyType,
    ) -> Result<(), RusotoError<ExecutePolicyError>>;

    /// <p>Moves the specified instances out of the standby state.</p> <p>After you put the instances back in service, the desired capacity is incremented.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/as-enter-exit-standby.html">Temporarily removing instances from your Auto Scaling group</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    async fn exit_standby(
        &self,
        input: ExitStandbyQuery,
    ) -> Result<ExitStandbyAnswer, RusotoError<ExitStandbyError>>;

    /// <p>Creates or updates a lifecycle hook for the specified Auto Scaling group.</p> <p>A lifecycle hook tells Amazon EC2 Auto Scaling to perform an action on an instance when the instance launches (before it is put into service) or as the instance terminates (before it is fully terminated).</p> <p>This step is a part of the procedure for adding a lifecycle hook to an Auto Scaling group:</p> <ol> <li> <p>(Optional) Create a Lambda function and a rule that allows CloudWatch Events to invoke your Lambda function when Amazon EC2 Auto Scaling launches or terminates instances.</p> </li> <li> <p>(Optional) Create a notification target and an IAM role. The target can be either an Amazon SQS queue or an Amazon SNS topic. The role allows Amazon EC2 Auto Scaling to publish lifecycle notifications to the target.</p> </li> <li> <p> <b>Create the lifecycle hook. Specify whether the hook is used when the instances launch or terminate.</b> </p> </li> <li> <p>If you need more time, record the lifecycle action heartbeat to keep the instance in a pending state using the <a>RecordLifecycleActionHeartbeat</a> API call.</p> </li> <li> <p>If you finish before the timeout period ends, complete the lifecycle action using the <a>CompleteLifecycleAction</a> API call.</p> </li> </ol> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/lifecycle-hooks.html">Amazon EC2 Auto Scaling lifecycle hooks</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p> <p>If you exceed your maximum limit of lifecycle hooks, which by default is 50 per Auto Scaling group, the call fails.</p> <p>You can view the lifecycle hooks for an Auto Scaling group using the <a>DescribeLifecycleHooks</a> API call. If you are no longer using a lifecycle hook, you can delete it by calling the <a>DeleteLifecycleHook</a> API.</p>
    async fn put_lifecycle_hook(
        &self,
        input: PutLifecycleHookType,
    ) -> Result<PutLifecycleHookAnswer, RusotoError<PutLifecycleHookError>>;

    /// <p>Configures an Auto Scaling group to send notifications when specified events take place. Subscribers to the specified topic can have messages delivered to an endpoint such as a web server or an email address.</p> <p>This configuration overwrites any existing configuration.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/ASGettingNotifications.html">Getting Amazon SNS notifications when your Auto Scaling group scales</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p> <p>If you exceed your maximum limit of SNS topics, which is 10 per Auto Scaling group, the call fails.</p>
    async fn put_notification_configuration(
        &self,
        input: PutNotificationConfigurationType,
    ) -> Result<(), RusotoError<PutNotificationConfigurationError>>;

    /// <p>Creates or updates a scaling policy for an Auto Scaling group.</p> <p>For more information about using scaling policies to scale your Auto Scaling group, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/as-scaling-target-tracking.html">Target tracking scaling policies</a> and <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/as-scaling-simple-step.html">Step and simple scaling policies</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    async fn put_scaling_policy(
        &self,
        input: PutScalingPolicyType,
    ) -> Result<PolicyARNType, RusotoError<PutScalingPolicyError>>;

    /// <p>Creates or updates a scheduled scaling action for an Auto Scaling group. If you leave a parameter unspecified when updating a scheduled scaling action, the corresponding value remains unchanged.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/schedule_time.html">Scheduled scaling</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    async fn put_scheduled_update_group_action(
        &self,
        input: PutScheduledUpdateGroupActionType,
    ) -> Result<(), RusotoError<PutScheduledUpdateGroupActionError>>;

    /// <p>Records a heartbeat for the lifecycle action associated with the specified token or instance. This extends the timeout by the length of time defined using the <a>PutLifecycleHook</a> API call.</p> <p>This step is a part of the procedure for adding a lifecycle hook to an Auto Scaling group:</p> <ol> <li> <p>(Optional) Create a Lambda function and a rule that allows CloudWatch Events to invoke your Lambda function when Amazon EC2 Auto Scaling launches or terminates instances.</p> </li> <li> <p>(Optional) Create a notification target and an IAM role. The target can be either an Amazon SQS queue or an Amazon SNS topic. The role allows Amazon EC2 Auto Scaling to publish lifecycle notifications to the target.</p> </li> <li> <p>Create the lifecycle hook. Specify whether the hook is used when the instances launch or terminate.</p> </li> <li> <p> <b>If you need more time, record the lifecycle action heartbeat to keep the instance in a pending state.</b> </p> </li> <li> <p>If you finish before the timeout period ends, complete the lifecycle action.</p> </li> </ol> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/AutoScalingGroupLifecycle.html">Auto Scaling lifecycle</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    async fn record_lifecycle_action_heartbeat(
        &self,
        input: RecordLifecycleActionHeartbeatType,
    ) -> Result<
        RecordLifecycleActionHeartbeatAnswer,
        RusotoError<RecordLifecycleActionHeartbeatError>,
    >;

    /// <p>Resumes the specified suspended auto scaling processes, or all suspended process, for the specified Auto Scaling group.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/as-suspend-resume-processes.html">Suspending and resuming scaling processes</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    async fn resume_processes(
        &self,
        input: ScalingProcessQuery,
    ) -> Result<(), RusotoError<ResumeProcessesError>>;

    /// <p>Sets the size of the specified Auto Scaling group.</p> <p>If a scale-in activity occurs as a result of a new <code>DesiredCapacity</code> value that is lower than the current size of the group, the Auto Scaling group uses its termination policy to determine which instances to terminate. </p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/as-manual-scaling.html">Manual scaling</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    async fn set_desired_capacity(
        &self,
        input: SetDesiredCapacityType,
    ) -> Result<(), RusotoError<SetDesiredCapacityError>>;

    /// <p>Sets the health status of the specified instance.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/healthcheck.html">Health checks for Auto Scaling instances</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    async fn set_instance_health(
        &self,
        input: SetInstanceHealthQuery,
    ) -> Result<(), RusotoError<SetInstanceHealthError>>;

    /// <p>Updates the instance protection settings of the specified instances.</p> <p>For more information about preventing instances that are part of an Auto Scaling group from terminating on scale in, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/as-instance-termination.html#instance-protection">Instance scale-in protection</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p> <p>If you exceed your maximum limit of instance IDs, which is 50 per Auto Scaling group, the call fails.</p>
    async fn set_instance_protection(
        &self,
        input: SetInstanceProtectionQuery,
    ) -> Result<SetInstanceProtectionAnswer, RusotoError<SetInstanceProtectionError>>;

    /// <p>Starts a new instance refresh operation, which triggers a rolling replacement of all previously launched instances in the Auto Scaling group with a new group of instances.</p> <p>If successful, this call creates a new instance refresh request with a unique ID that you can use to track its progress. To query its status, call the <a>DescribeInstanceRefreshes</a> API. To describe the instance refreshes that have already run, call the <a>DescribeInstanceRefreshes</a> API. To cancel an instance refresh operation in progress, use the <a>CancelInstanceRefresh</a> API. </p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/asg-instance-refresh.html">Replacing Auto Scaling Instances Based on an Instance Refresh</a>.</p>
    async fn start_instance_refresh(
        &self,
        input: StartInstanceRefreshType,
    ) -> Result<StartInstanceRefreshAnswer, RusotoError<StartInstanceRefreshError>>;

    /// <p>Suspends the specified auto scaling processes, or all processes, for the specified Auto Scaling group.</p> <p>If you suspend either the <code>Launch</code> or <code>Terminate</code> process types, it can prevent other process types from functioning properly. For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/as-suspend-resume-processes.html">Suspending and resuming scaling processes</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p> <p>To resume processes that have been suspended, call the <a>ResumeProcesses</a> API.</p>
    async fn suspend_processes(
        &self,
        input: ScalingProcessQuery,
    ) -> Result<(), RusotoError<SuspendProcessesError>>;

    /// <p>Terminates the specified instance and optionally adjusts the desired group size. </p> <p>This call simply makes a termination request. The instance is not terminated immediately. When an instance is terminated, the instance status changes to <code>terminated</code>. You can't connect to or start an instance after you've terminated it.</p> <p>If you do not specify the option to decrement the desired capacity, Amazon EC2 Auto Scaling launches instances to replace the ones that are terminated. </p> <p>By default, Amazon EC2 Auto Scaling balances instances across all Availability Zones. If you decrement the desired capacity, your Auto Scaling group can become unbalanced between Availability Zones. Amazon EC2 Auto Scaling tries to rebalance the group, and rebalancing might terminate instances in other zones. For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/auto-scaling-benefits.html#AutoScalingBehavior.InstanceUsage">Rebalancing activities</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    async fn terminate_instance_in_auto_scaling_group(
        &self,
        input: TerminateInstanceInAutoScalingGroupType,
    ) -> Result<ActivityType, RusotoError<TerminateInstanceInAutoScalingGroupError>>;

    /// <p>Updates the configuration for the specified Auto Scaling group.</p> <p>To update an Auto Scaling group, specify the name of the group and the parameter that you want to change. Any parameters that you don't specify are not changed by this update request. The new settings take effect on any scaling activities after this call returns. </p> <p>If you associate a new launch configuration or template with an Auto Scaling group, all new instances will get the updated configuration. Existing instances continue to run with the configuration that they were originally launched with. When you update a group to specify a mixed instances policy instead of a launch configuration or template, existing instances may be replaced to match the new purchasing options that you specified in the policy. For example, if the group currently has 100% On-Demand capacity and the policy specifies 50% Spot capacity, this means that half of your instances will be gradually terminated and relaunched as Spot Instances. When replacing instances, Amazon EC2 Auto Scaling launches new instances before terminating the old ones, so that updating your group does not compromise the performance or availability of your application.</p> <p>Note the following about changing <code>DesiredCapacity</code>, <code>MaxSize</code>, or <code>MinSize</code>:</p> <ul> <li> <p>If a scale-in activity occurs as a result of a new <code>DesiredCapacity</code> value that is lower than the current size of the group, the Auto Scaling group uses its termination policy to determine which instances to terminate.</p> </li> <li> <p>If you specify a new value for <code>MinSize</code> without specifying a value for <code>DesiredCapacity</code>, and the new <code>MinSize</code> is larger than the current size of the group, this sets the group's <code>DesiredCapacity</code> to the new <code>MinSize</code> value.</p> </li> <li> <p>If you specify a new value for <code>MaxSize</code> without specifying a value for <code>DesiredCapacity</code>, and the new <code>MaxSize</code> is smaller than the current size of the group, this sets the group's <code>DesiredCapacity</code> to the new <code>MaxSize</code> value.</p> </li> </ul> <p>To see which parameters have been set, call the <a>DescribeAutoScalingGroups</a> API. To view the scaling policies for an Auto Scaling group, call the <a>DescribePolicies</a> API. If the group has scaling policies, you can update them by calling the <a>PutScalingPolicy</a> API.</p>
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
    /// <p>Attaches one or more EC2 instances to the specified Auto Scaling group.</p> <p>When you attach instances, Amazon EC2 Auto Scaling increases the desired capacity of the group by the number of instances being attached. If the number of instances being attached plus the desired capacity of the group exceeds the maximum size of the group, the operation fails.</p> <p>If there is a Classic Load Balancer attached to your Auto Scaling group, the instances are also registered with the load balancer. If there are target groups attached to your Auto Scaling group, the instances are also registered with the target groups.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/attach-instance-asg.html">Attach EC2 instances to your Auto Scaling group</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    async fn attach_instances(
        &self,
        input: AttachInstancesQuery,
    ) -> Result<(), RusotoError<AttachInstancesError>> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let params = self.new_params("AttachInstances");
        let mut params = params;
        AttachInstancesQuerySerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, AttachInstancesError::from_response)
            .await?;

        std::mem::drop(response);
        Ok(())
    }

    /// <p>Attaches one or more target groups to the specified Auto Scaling group.</p> <p>This operation is used with the following load balancer types: </p> <ul> <li> <p> Application Load Balancer - Operates at the application layer (layer 7) and supports HTTP and HTTPS. </p> </li> <li> <p> Network Load Balancer - Operates at the transport layer (layer 4) and supports TCP, TLS, and UDP. </p> </li> <li> <p> Gateway Load Balancer - Operates at the network layer (layer 3).</p> </li> </ul> <p>To describe the target groups for an Auto Scaling group, call the <a>DescribeLoadBalancerTargetGroups</a> API. To detach the target group from the Auto Scaling group, call the <a>DetachLoadBalancerTargetGroups</a> API.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/autoscaling-load-balancer.html">Elastic Load Balancing and Amazon EC2 Auto Scaling</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>. </p>
    async fn attach_load_balancer_target_groups(
        &self,
        input: AttachLoadBalancerTargetGroupsType,
    ) -> Result<
        AttachLoadBalancerTargetGroupsResultType,
        RusotoError<AttachLoadBalancerTargetGroupsError>,
    > {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let params = self.new_params("AttachLoadBalancerTargetGroups");
        let mut params = params;
        AttachLoadBalancerTargetGroupsTypeSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, AttachLoadBalancerTargetGroupsError::from_response)
            .await?;

        let result = AttachLoadBalancerTargetGroupsResultType::default();

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p><note> <p>To attach an Application Load Balancer, Network Load Balancer, or Gateway Load Balancer, use the <a>AttachLoadBalancerTargetGroups</a> API operation instead.</p> </note> <p>Attaches one or more Classic Load Balancers to the specified Auto Scaling group. Amazon EC2 Auto Scaling registers the running instances with these Classic Load Balancers.</p> <p>To describe the load balancers for an Auto Scaling group, call the <a>DescribeLoadBalancers</a> API. To detach the load balancer from the Auto Scaling group, call the <a>DetachLoadBalancers</a> API.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/autoscaling-load-balancer.html">Elastic Load Balancing and Amazon EC2 Auto Scaling</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>. </p></p>
    async fn attach_load_balancers(
        &self,
        input: AttachLoadBalancersType,
    ) -> Result<AttachLoadBalancersResultType, RusotoError<AttachLoadBalancersError>> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let params = self.new_params("AttachLoadBalancers");
        let mut params = params;
        AttachLoadBalancersTypeSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, AttachLoadBalancersError::from_response)
            .await?;

        let result = AttachLoadBalancersResultType::default();

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p>Deletes one or more scheduled actions for the specified Auto Scaling group.</p>
    async fn batch_delete_scheduled_action(
        &self,
        input: BatchDeleteScheduledActionType,
    ) -> Result<BatchDeleteScheduledActionAnswer, RusotoError<BatchDeleteScheduledActionError>>
    {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let params = self.new_params("BatchDeleteScheduledAction");
        let mut params = params;
        BatchDeleteScheduledActionTypeSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, BatchDeleteScheduledActionError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = BatchDeleteScheduledActionAnswerDeserializer::deserialize(
                "BatchDeleteScheduledActionResult",
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

    /// <p>Creates or updates one or more scheduled scaling actions for an Auto Scaling group. If you leave a parameter unspecified when updating a scheduled scaling action, the corresponding value remains unchanged.</p>
    async fn batch_put_scheduled_update_group_action(
        &self,
        input: BatchPutScheduledUpdateGroupActionType,
    ) -> Result<
        BatchPutScheduledUpdateGroupActionAnswer,
        RusotoError<BatchPutScheduledUpdateGroupActionError>,
    > {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let params = self.new_params("BatchPutScheduledUpdateGroupAction");
        let mut params = params;
        BatchPutScheduledUpdateGroupActionTypeSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(
                request,
                BatchPutScheduledUpdateGroupActionError::from_response,
            )
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = BatchPutScheduledUpdateGroupActionAnswerDeserializer::deserialize(
                "BatchPutScheduledUpdateGroupActionResult",
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

    /// <p>Cancels an instance refresh operation in progress. Cancellation does not roll back any replacements that have already been completed, but it prevents new replacements from being started. </p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/asg-instance-refresh.html">Replacing Auto Scaling Instances Based on an Instance Refresh</a>.</p>
    async fn cancel_instance_refresh(
        &self,
        input: CancelInstanceRefreshType,
    ) -> Result<CancelInstanceRefreshAnswer, RusotoError<CancelInstanceRefreshError>> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let params = self.new_params("CancelInstanceRefresh");
        let mut params = params;
        CancelInstanceRefreshTypeSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, CancelInstanceRefreshError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = CancelInstanceRefreshAnswerDeserializer::deserialize(
                "CancelInstanceRefreshResult",
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

    /// <p>Completes the lifecycle action for the specified token or instance with the specified result.</p> <p>This step is a part of the procedure for adding a lifecycle hook to an Auto Scaling group:</p> <ol> <li> <p>(Optional) Create a Lambda function and a rule that allows CloudWatch Events to invoke your Lambda function when Amazon EC2 Auto Scaling launches or terminates instances.</p> </li> <li> <p>(Optional) Create a notification target and an IAM role. The target can be either an Amazon SQS queue or an Amazon SNS topic. The role allows Amazon EC2 Auto Scaling to publish lifecycle notifications to the target.</p> </li> <li> <p>Create the lifecycle hook. Specify whether the hook is used when the instances launch or terminate.</p> </li> <li> <p>If you need more time, record the lifecycle action heartbeat to keep the instance in a pending state.</p> </li> <li> <p> <b>If you finish before the timeout period ends, complete the lifecycle action.</b> </p> </li> </ol> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/lifecycle-hooks.html">Amazon EC2 Auto Scaling lifecycle hooks</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    async fn complete_lifecycle_action(
        &self,
        input: CompleteLifecycleActionType,
    ) -> Result<CompleteLifecycleActionAnswer, RusotoError<CompleteLifecycleActionError>> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let params = self.new_params("CompleteLifecycleAction");
        let mut params = params;
        CompleteLifecycleActionTypeSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, CompleteLifecycleActionError::from_response)
            .await?;

        let result = CompleteLifecycleActionAnswer::default();

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p>Creates an Auto Scaling group with the specified name and attributes. </p> <p>If you exceed your maximum limit of Auto Scaling groups, the call fails. To query this limit, call the <a>DescribeAccountLimits</a> API. For information about updating this limit, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/as-account-limits.html">Amazon EC2 Auto Scaling service quotas</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p> <p>For introductory exercises for creating an Auto Scaling group, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/GettingStartedTutorial.html">Getting started with Amazon EC2 Auto Scaling</a> and <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/as-register-lbs-with-asg.html">Tutorial: Set up a scaled and load-balanced application</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>. For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/AutoScalingGroup.html">Auto Scaling groups</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p> <p>Every Auto Scaling group has three size parameters (<code>DesiredCapacity</code>, <code>MaxSize</code>, and <code>MinSize</code>). Usually, you set these sizes based on a specific number of instances. However, if you configure a mixed instances policy that defines weights for the instance types, you must specify these sizes with the same units that you use for weighting instances.</p>
    async fn create_auto_scaling_group(
        &self,
        input: CreateAutoScalingGroupType,
    ) -> Result<(), RusotoError<CreateAutoScalingGroupError>> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let params = self.new_params("CreateAutoScalingGroup");
        let mut params = params;
        CreateAutoScalingGroupTypeSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, CreateAutoScalingGroupError::from_response)
            .await?;

        std::mem::drop(response);
        Ok(())
    }

    /// <p>Creates a launch configuration.</p> <p>If you exceed your maximum limit of launch configurations, the call fails. To query this limit, call the <a>DescribeAccountLimits</a> API. For information about updating this limit, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/as-account-limits.html">Amazon EC2 Auto Scaling service quotas</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/LaunchConfiguration.html">Launch configurations</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    async fn create_launch_configuration(
        &self,
        input: CreateLaunchConfigurationType,
    ) -> Result<(), RusotoError<CreateLaunchConfigurationError>> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let params = self.new_params("CreateLaunchConfiguration");
        let mut params = params;
        CreateLaunchConfigurationTypeSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, CreateLaunchConfigurationError::from_response)
            .await?;

        std::mem::drop(response);
        Ok(())
    }

    /// <p>Creates or updates tags for the specified Auto Scaling group.</p> <p>When you specify a tag with a key that already exists, the operation overwrites the previous tag definition, and you do not get an error message.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/autoscaling-tagging.html">Tagging Auto Scaling groups and instances</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    async fn create_or_update_tags(
        &self,
        input: CreateOrUpdateTagsType,
    ) -> Result<(), RusotoError<CreateOrUpdateTagsError>> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let params = self.new_params("CreateOrUpdateTags");
        let mut params = params;
        CreateOrUpdateTagsTypeSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, CreateOrUpdateTagsError::from_response)
            .await?;

        std::mem::drop(response);
        Ok(())
    }

    /// <p>Deletes the specified Auto Scaling group.</p> <p>If the group has instances or scaling activities in progress, you must specify the option to force the deletion in order for it to succeed.</p> <p>If the group has policies, deleting the group deletes the policies, the underlying alarm actions, and any alarm that no longer has an associated action.</p> <p>To remove instances from the Auto Scaling group before deleting it, call the <a>DetachInstances</a> API with the list of instances and the option to decrement the desired capacity. This ensures that Amazon EC2 Auto Scaling does not launch replacement instances.</p> <p>To terminate all instances before deleting the Auto Scaling group, call the <a>UpdateAutoScalingGroup</a> API and set the minimum size and desired capacity of the Auto Scaling group to zero.</p>
    async fn delete_auto_scaling_group(
        &self,
        input: DeleteAutoScalingGroupType,
    ) -> Result<(), RusotoError<DeleteAutoScalingGroupError>> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let params = self.new_params("DeleteAutoScalingGroup");
        let mut params = params;
        DeleteAutoScalingGroupTypeSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, DeleteAutoScalingGroupError::from_response)
            .await?;

        std::mem::drop(response);
        Ok(())
    }

    /// <p>Deletes the specified launch configuration.</p> <p>The launch configuration must not be attached to an Auto Scaling group. When this call completes, the launch configuration is no longer available for use.</p>
    async fn delete_launch_configuration(
        &self,
        input: LaunchConfigurationNameType,
    ) -> Result<(), RusotoError<DeleteLaunchConfigurationError>> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let params = self.new_params("DeleteLaunchConfiguration");
        let mut params = params;
        LaunchConfigurationNameTypeSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, DeleteLaunchConfigurationError::from_response)
            .await?;

        std::mem::drop(response);
        Ok(())
    }

    /// <p>Deletes the specified lifecycle hook.</p> <p>If there are any outstanding lifecycle actions, they are completed first (<code>ABANDON</code> for launching instances, <code>CONTINUE</code> for terminating instances).</p>
    async fn delete_lifecycle_hook(
        &self,
        input: DeleteLifecycleHookType,
    ) -> Result<DeleteLifecycleHookAnswer, RusotoError<DeleteLifecycleHookError>> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let params = self.new_params("DeleteLifecycleHook");
        let mut params = params;
        DeleteLifecycleHookTypeSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, DeleteLifecycleHookError::from_response)
            .await?;

        let result = DeleteLifecycleHookAnswer::default();

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p>Deletes the specified notification.</p>
    async fn delete_notification_configuration(
        &self,
        input: DeleteNotificationConfigurationType,
    ) -> Result<(), RusotoError<DeleteNotificationConfigurationError>> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let params = self.new_params("DeleteNotificationConfiguration");
        let mut params = params;
        DeleteNotificationConfigurationTypeSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, DeleteNotificationConfigurationError::from_response)
            .await?;

        std::mem::drop(response);
        Ok(())
    }

    /// <p>Deletes the specified scaling policy.</p> <p>Deleting either a step scaling policy or a simple scaling policy deletes the underlying alarm action, but does not delete the alarm, even if it no longer has an associated action.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/deleting-scaling-policy.html">Deleting a scaling policy</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    async fn delete_policy(
        &self,
        input: DeletePolicyType,
    ) -> Result<(), RusotoError<DeletePolicyError>> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let params = self.new_params("DeletePolicy");
        let mut params = params;
        DeletePolicyTypeSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, DeletePolicyError::from_response)
            .await?;

        std::mem::drop(response);
        Ok(())
    }

    /// <p>Deletes the specified scheduled action.</p>
    async fn delete_scheduled_action(
        &self,
        input: DeleteScheduledActionType,
    ) -> Result<(), RusotoError<DeleteScheduledActionError>> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let params = self.new_params("DeleteScheduledAction");
        let mut params = params;
        DeleteScheduledActionTypeSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, DeleteScheduledActionError::from_response)
            .await?;

        std::mem::drop(response);
        Ok(())
    }

    /// <p>Deletes the specified tags.</p>
    async fn delete_tags(&self, input: DeleteTagsType) -> Result<(), RusotoError<DeleteTagsError>> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let params = self.new_params("DeleteTags");
        let mut params = params;
        DeleteTagsTypeSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, DeleteTagsError::from_response)
            .await?;

        std::mem::drop(response);
        Ok(())
    }

    /// <p>Describes the current Amazon EC2 Auto Scaling resource quotas for your AWS account.</p> <p>For information about requesting an increase, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/as-account-limits.html">Amazon EC2 Auto Scaling service quotas</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    async fn describe_account_limits(
        &self,
    ) -> Result<DescribeAccountLimitsAnswer, RusotoError<DescribeAccountLimitsError>> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let params = self.new_params("DescribeAccountLimits");

        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, DescribeAccountLimitsError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = DescribeAccountLimitsAnswerDeserializer::deserialize(
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

    /// <p><p>Describes the available adjustment types for Amazon EC2 Auto Scaling scaling policies. These settings apply to step scaling policies and simple scaling policies; they do not apply to target tracking scaling policies.</p> <p>The following adjustment types are supported:</p> <ul> <li> <p>ChangeInCapacity</p> </li> <li> <p>ExactCapacity</p> </li> <li> <p>PercentChangeInCapacity</p> </li> </ul></p>
    async fn describe_adjustment_types(
        &self,
    ) -> Result<DescribeAdjustmentTypesAnswer, RusotoError<DescribeAdjustmentTypesError>> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let params = self.new_params("DescribeAdjustmentTypes");

        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, DescribeAdjustmentTypesError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = DescribeAdjustmentTypesAnswerDeserializer::deserialize(
                "DescribeAdjustmentTypesResult",
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

    /// <p>Describes one or more Auto Scaling groups.</p>
    async fn describe_auto_scaling_groups(
        &self,
        input: AutoScalingGroupNamesType,
    ) -> Result<AutoScalingGroupsType, RusotoError<DescribeAutoScalingGroupsError>> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let params = self.new_params("DescribeAutoScalingGroups");
        let mut params = params;
        AutoScalingGroupNamesTypeSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, DescribeAutoScalingGroupsError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = AutoScalingGroupsTypeDeserializer::deserialize(
                "DescribeAutoScalingGroupsResult",
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

    /// <p>Describes one or more Auto Scaling instances.</p>
    async fn describe_auto_scaling_instances(
        &self,
        input: DescribeAutoScalingInstancesType,
    ) -> Result<AutoScalingInstancesType, RusotoError<DescribeAutoScalingInstancesError>> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let params = self.new_params("DescribeAutoScalingInstances");
        let mut params = params;
        DescribeAutoScalingInstancesTypeSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, DescribeAutoScalingInstancesError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = AutoScalingInstancesTypeDeserializer::deserialize(
                "DescribeAutoScalingInstancesResult",
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

    /// <p>Describes the notification types that are supported by Amazon EC2 Auto Scaling.</p>
    async fn describe_auto_scaling_notification_types(
        &self,
    ) -> Result<
        DescribeAutoScalingNotificationTypesAnswer,
        RusotoError<DescribeAutoScalingNotificationTypesError>,
    > {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let params = self.new_params("DescribeAutoScalingNotificationTypes");

        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(
                request,
                DescribeAutoScalingNotificationTypesError::from_response,
            )
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = DescribeAutoScalingNotificationTypesAnswerDeserializer::deserialize(
                "DescribeAutoScalingNotificationTypesResult",
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

    /// <p>Describes one or more instance refreshes.</p> <p>You can determine the status of a request by looking at the <code>Status</code> parameter. The following are the possible statuses: </p> <ul> <li> <p> <code>Pending</code> - The request was created, but the operation has not started.</p> </li> <li> <p> <code>InProgress</code> - The operation is in progress.</p> </li> <li> <p> <code>Successful</code> - The operation completed successfully.</p> </li> <li> <p> <code>Failed</code> - The operation failed to complete. You can troubleshoot using the status reason and the scaling activities. </p> </li> <li> <p> <code>Cancelling</code> - An ongoing operation is being cancelled. Cancellation does not roll back any replacements that have already been completed, but it prevents new replacements from being started. </p> </li> <li> <p> <code>Cancelled</code> - The operation is cancelled. </p> </li> </ul> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/asg-instance-refresh.html">Replacing Auto Scaling Instances Based on an Instance Refresh</a>.</p>
    async fn describe_instance_refreshes(
        &self,
        input: DescribeInstanceRefreshesType,
    ) -> Result<DescribeInstanceRefreshesAnswer, RusotoError<DescribeInstanceRefreshesError>> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let params = self.new_params("DescribeInstanceRefreshes");
        let mut params = params;
        DescribeInstanceRefreshesTypeSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, DescribeInstanceRefreshesError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = DescribeInstanceRefreshesAnswerDeserializer::deserialize(
                "DescribeInstanceRefreshesResult",
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

    /// <p>Describes one or more launch configurations.</p>
    async fn describe_launch_configurations(
        &self,
        input: LaunchConfigurationNamesType,
    ) -> Result<LaunchConfigurationsType, RusotoError<DescribeLaunchConfigurationsError>> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let params = self.new_params("DescribeLaunchConfigurations");
        let mut params = params;
        LaunchConfigurationNamesTypeSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, DescribeLaunchConfigurationsError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = LaunchConfigurationsTypeDeserializer::deserialize(
                "DescribeLaunchConfigurationsResult",
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

    /// <p><p>Describes the available types of lifecycle hooks.</p> <p>The following hook types are supported:</p> <ul> <li> <p>autoscaling:EC2<em>INSTANCE</em>LAUNCHING</p> </li> <li> <p>autoscaling:EC2<em>INSTANCE</em>TERMINATING</p> </li> </ul></p>
    async fn describe_lifecycle_hook_types(
        &self,
    ) -> Result<DescribeLifecycleHookTypesAnswer, RusotoError<DescribeLifecycleHookTypesError>>
    {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let params = self.new_params("DescribeLifecycleHookTypes");

        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, DescribeLifecycleHookTypesError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = DescribeLifecycleHookTypesAnswerDeserializer::deserialize(
                "DescribeLifecycleHookTypesResult",
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

    /// <p>Describes the lifecycle hooks for the specified Auto Scaling group.</p>
    async fn describe_lifecycle_hooks(
        &self,
        input: DescribeLifecycleHooksType,
    ) -> Result<DescribeLifecycleHooksAnswer, RusotoError<DescribeLifecycleHooksError>> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let params = self.new_params("DescribeLifecycleHooks");
        let mut params = params;
        DescribeLifecycleHooksTypeSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, DescribeLifecycleHooksError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = DescribeLifecycleHooksAnswerDeserializer::deserialize(
                "DescribeLifecycleHooksResult",
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

    /// <p>Describes the target groups for the specified Auto Scaling group.</p>
    async fn describe_load_balancer_target_groups(
        &self,
        input: DescribeLoadBalancerTargetGroupsRequest,
    ) -> Result<
        DescribeLoadBalancerTargetGroupsResponse,
        RusotoError<DescribeLoadBalancerTargetGroupsError>,
    > {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let params = self.new_params("DescribeLoadBalancerTargetGroups");
        let mut params = params;
        DescribeLoadBalancerTargetGroupsRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(
                request,
                DescribeLoadBalancerTargetGroupsError::from_response,
            )
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = DescribeLoadBalancerTargetGroupsResponseDeserializer::deserialize(
                "DescribeLoadBalancerTargetGroupsResult",
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

    /// <p>Describes the load balancers for the specified Auto Scaling group.</p> <p>This operation describes only Classic Load Balancers. If you have Application Load Balancers, Network Load Balancers, or Gateway Load Balancers, use the <a>DescribeLoadBalancerTargetGroups</a> API instead.</p>
    async fn describe_load_balancers(
        &self,
        input: DescribeLoadBalancersRequest,
    ) -> Result<DescribeLoadBalancersResponse, RusotoError<DescribeLoadBalancersError>> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let params = self.new_params("DescribeLoadBalancers");
        let mut params = params;
        DescribeLoadBalancersRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, DescribeLoadBalancersError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = DescribeLoadBalancersResponseDeserializer::deserialize(
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

    /// <p>Describes the available CloudWatch metrics for Amazon EC2 Auto Scaling.</p> <p>The <code>GroupStandbyInstances</code> metric is not returned by default. You must explicitly request this metric when calling the <a>EnableMetricsCollection</a> API.</p>
    async fn describe_metric_collection_types(
        &self,
    ) -> Result<DescribeMetricCollectionTypesAnswer, RusotoError<DescribeMetricCollectionTypesError>>
    {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let params = self.new_params("DescribeMetricCollectionTypes");

        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, DescribeMetricCollectionTypesError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = DescribeMetricCollectionTypesAnswerDeserializer::deserialize(
                "DescribeMetricCollectionTypesResult",
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

    /// <p>Describes the notification actions associated with the specified Auto Scaling group.</p>
    async fn describe_notification_configurations(
        &self,
        input: DescribeNotificationConfigurationsType,
    ) -> Result<
        DescribeNotificationConfigurationsAnswer,
        RusotoError<DescribeNotificationConfigurationsError>,
    > {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let params = self.new_params("DescribeNotificationConfigurations");
        let mut params = params;
        DescribeNotificationConfigurationsTypeSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(
                request,
                DescribeNotificationConfigurationsError::from_response,
            )
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = DescribeNotificationConfigurationsAnswerDeserializer::deserialize(
                "DescribeNotificationConfigurationsResult",
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

    /// <p>Describes the policies for the specified Auto Scaling group.</p>
    async fn describe_policies(
        &self,
        input: DescribePoliciesType,
    ) -> Result<PoliciesType, RusotoError<DescribePoliciesError>> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let params = self.new_params("DescribePolicies");
        let mut params = params;
        DescribePoliciesTypeSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, DescribePoliciesError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = PoliciesTypeDeserializer::deserialize("DescribePoliciesResult", stack)?;
            skip_tree(stack);
            xml_util::end_element(actual_tag_name, stack)?;
            Ok(result)
        })
        .await?;

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p>Describes one or more scaling activities for the specified Auto Scaling group.</p>
    async fn describe_scaling_activities(
        &self,
        input: DescribeScalingActivitiesType,
    ) -> Result<ActivitiesType, RusotoError<DescribeScalingActivitiesError>> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let params = self.new_params("DescribeScalingActivities");
        let mut params = params;
        DescribeScalingActivitiesTypeSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, DescribeScalingActivitiesError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result =
                ActivitiesTypeDeserializer::deserialize("DescribeScalingActivitiesResult", stack)?;
            skip_tree(stack);
            xml_util::end_element(actual_tag_name, stack)?;
            Ok(result)
        })
        .await?;

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p>Describes the scaling process types for use with the <a>ResumeProcesses</a> and <a>SuspendProcesses</a> APIs.</p>
    async fn describe_scaling_process_types(
        &self,
    ) -> Result<ProcessesType, RusotoError<DescribeScalingProcessTypesError>> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let params = self.new_params("DescribeScalingProcessTypes");

        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, DescribeScalingProcessTypesError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result =
                ProcessesTypeDeserializer::deserialize("DescribeScalingProcessTypesResult", stack)?;
            skip_tree(stack);
            xml_util::end_element(actual_tag_name, stack)?;
            Ok(result)
        })
        .await?;

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p>Describes the actions scheduled for your Auto Scaling group that haven't run or that have not reached their end time. To describe the actions that have already run, call the <a>DescribeScalingActivities</a> API.</p>
    async fn describe_scheduled_actions(
        &self,
        input: DescribeScheduledActionsType,
    ) -> Result<ScheduledActionsType, RusotoError<DescribeScheduledActionsError>> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let params = self.new_params("DescribeScheduledActions");
        let mut params = params;
        DescribeScheduledActionsTypeSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, DescribeScheduledActionsError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = ScheduledActionsTypeDeserializer::deserialize(
                "DescribeScheduledActionsResult",
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

    /// <p>Describes the specified tags.</p> <p>You can use filters to limit the results. For example, you can query for the tags for a specific Auto Scaling group. You can specify multiple values for a filter. A tag must match at least one of the specified values for it to be included in the results.</p> <p>You can also specify multiple filters. The result includes information for a particular tag only if it matches all the filters. If there's no match, no special message is returned.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/autoscaling-tagging.html">Tagging Auto Scaling groups and instances</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    async fn describe_tags(
        &self,
        input: DescribeTagsType,
    ) -> Result<TagsType, RusotoError<DescribeTagsError>> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let params = self.new_params("DescribeTags");
        let mut params = params;
        DescribeTagsTypeSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, DescribeTagsError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = TagsTypeDeserializer::deserialize("DescribeTagsResult", stack)?;
            skip_tree(stack);
            xml_util::end_element(actual_tag_name, stack)?;
            Ok(result)
        })
        .await?;

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p>Describes the termination policies supported by Amazon EC2 Auto Scaling.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/as-instance-termination.html">Controlling which Auto Scaling instances terminate during scale in</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    async fn describe_termination_policy_types(
        &self,
    ) -> Result<
        DescribeTerminationPolicyTypesAnswer,
        RusotoError<DescribeTerminationPolicyTypesError>,
    > {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let params = self.new_params("DescribeTerminationPolicyTypes");

        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, DescribeTerminationPolicyTypesError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = DescribeTerminationPolicyTypesAnswerDeserializer::deserialize(
                "DescribeTerminationPolicyTypesResult",
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

    /// <p>Removes one or more instances from the specified Auto Scaling group.</p> <p>After the instances are detached, you can manage them independent of the Auto Scaling group.</p> <p>If you do not specify the option to decrement the desired capacity, Amazon EC2 Auto Scaling launches instances to replace the ones that are detached.</p> <p>If there is a Classic Load Balancer attached to the Auto Scaling group, the instances are deregistered from the load balancer. If there are target groups attached to the Auto Scaling group, the instances are deregistered from the target groups.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/detach-instance-asg.html">Detach EC2 instances from your Auto Scaling group</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    async fn detach_instances(
        &self,
        input: DetachInstancesQuery,
    ) -> Result<DetachInstancesAnswer, RusotoError<DetachInstancesError>> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let params = self.new_params("DetachInstances");
        let mut params = params;
        DetachInstancesQuerySerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, DetachInstancesError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result =
                DetachInstancesAnswerDeserializer::deserialize("DetachInstancesResult", stack)?;
            skip_tree(stack);
            xml_util::end_element(actual_tag_name, stack)?;
            Ok(result)
        })
        .await?;

        drop(response); // parse non-payload
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
        let params = self.new_params("DetachLoadBalancerTargetGroups");
        let mut params = params;
        DetachLoadBalancerTargetGroupsTypeSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, DetachLoadBalancerTargetGroupsError::from_response)
            .await?;

        let result = DetachLoadBalancerTargetGroupsResultType::default();

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p>Detaches one or more Classic Load Balancers from the specified Auto Scaling group.</p> <p>This operation detaches only Classic Load Balancers. If you have Application Load Balancers, Network Load Balancers, or Gateway Load Balancers, use the <a>DetachLoadBalancerTargetGroups</a> API instead.</p> <p>When you detach a load balancer, it enters the <code>Removing</code> state while deregistering the instances in the group. When all instances are deregistered, then you can no longer describe the load balancer using the <a>DescribeLoadBalancers</a> API call. The instances remain running.</p>
    async fn detach_load_balancers(
        &self,
        input: DetachLoadBalancersType,
    ) -> Result<DetachLoadBalancersResultType, RusotoError<DetachLoadBalancersError>> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let params = self.new_params("DetachLoadBalancers");
        let mut params = params;
        DetachLoadBalancersTypeSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, DetachLoadBalancersError::from_response)
            .await?;

        let result = DetachLoadBalancersResultType::default();

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p>Disables group metrics for the specified Auto Scaling group.</p>
    async fn disable_metrics_collection(
        &self,
        input: DisableMetricsCollectionQuery,
    ) -> Result<(), RusotoError<DisableMetricsCollectionError>> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let params = self.new_params("DisableMetricsCollection");
        let mut params = params;
        DisableMetricsCollectionQuerySerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, DisableMetricsCollectionError::from_response)
            .await?;

        std::mem::drop(response);
        Ok(())
    }

    /// <p>Enables group metrics for the specified Auto Scaling group. For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/as-instance-monitoring.html">Monitoring CloudWatch metrics for your Auto Scaling groups and instances</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    async fn enable_metrics_collection(
        &self,
        input: EnableMetricsCollectionQuery,
    ) -> Result<(), RusotoError<EnableMetricsCollectionError>> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let params = self.new_params("EnableMetricsCollection");
        let mut params = params;
        EnableMetricsCollectionQuerySerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, EnableMetricsCollectionError::from_response)
            .await?;

        std::mem::drop(response);
        Ok(())
    }

    /// <p>Moves the specified instances into the standby state.</p> <p>If you choose to decrement the desired capacity of the Auto Scaling group, the instances can enter standby as long as the desired capacity of the Auto Scaling group after the instances are placed into standby is equal to or greater than the minimum capacity of the group.</p> <p>If you choose not to decrement the desired capacity of the Auto Scaling group, the Auto Scaling group launches new instances to replace the instances on standby.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/as-enter-exit-standby.html">Temporarily removing instances from your Auto Scaling group</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    async fn enter_standby(
        &self,
        input: EnterStandbyQuery,
    ) -> Result<EnterStandbyAnswer, RusotoError<EnterStandbyError>> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let params = self.new_params("EnterStandby");
        let mut params = params;
        EnterStandbyQuerySerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, EnterStandbyError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = EnterStandbyAnswerDeserializer::deserialize("EnterStandbyResult", stack)?;
            skip_tree(stack);
            xml_util::end_element(actual_tag_name, stack)?;
            Ok(result)
        })
        .await?;

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p>Executes the specified policy. This can be useful for testing the design of your scaling policy.</p>
    async fn execute_policy(
        &self,
        input: ExecutePolicyType,
    ) -> Result<(), RusotoError<ExecutePolicyError>> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let params = self.new_params("ExecutePolicy");
        let mut params = params;
        ExecutePolicyTypeSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, ExecutePolicyError::from_response)
            .await?;

        std::mem::drop(response);
        Ok(())
    }

    /// <p>Moves the specified instances out of the standby state.</p> <p>After you put the instances back in service, the desired capacity is incremented.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/as-enter-exit-standby.html">Temporarily removing instances from your Auto Scaling group</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    async fn exit_standby(
        &self,
        input: ExitStandbyQuery,
    ) -> Result<ExitStandbyAnswer, RusotoError<ExitStandbyError>> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let params = self.new_params("ExitStandby");
        let mut params = params;
        ExitStandbyQuerySerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, ExitStandbyError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = ExitStandbyAnswerDeserializer::deserialize("ExitStandbyResult", stack)?;
            skip_tree(stack);
            xml_util::end_element(actual_tag_name, stack)?;
            Ok(result)
        })
        .await?;

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p>Creates or updates a lifecycle hook for the specified Auto Scaling group.</p> <p>A lifecycle hook tells Amazon EC2 Auto Scaling to perform an action on an instance when the instance launches (before it is put into service) or as the instance terminates (before it is fully terminated).</p> <p>This step is a part of the procedure for adding a lifecycle hook to an Auto Scaling group:</p> <ol> <li> <p>(Optional) Create a Lambda function and a rule that allows CloudWatch Events to invoke your Lambda function when Amazon EC2 Auto Scaling launches or terminates instances.</p> </li> <li> <p>(Optional) Create a notification target and an IAM role. The target can be either an Amazon SQS queue or an Amazon SNS topic. The role allows Amazon EC2 Auto Scaling to publish lifecycle notifications to the target.</p> </li> <li> <p> <b>Create the lifecycle hook. Specify whether the hook is used when the instances launch or terminate.</b> </p> </li> <li> <p>If you need more time, record the lifecycle action heartbeat to keep the instance in a pending state using the <a>RecordLifecycleActionHeartbeat</a> API call.</p> </li> <li> <p>If you finish before the timeout period ends, complete the lifecycle action using the <a>CompleteLifecycleAction</a> API call.</p> </li> </ol> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/lifecycle-hooks.html">Amazon EC2 Auto Scaling lifecycle hooks</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p> <p>If you exceed your maximum limit of lifecycle hooks, which by default is 50 per Auto Scaling group, the call fails.</p> <p>You can view the lifecycle hooks for an Auto Scaling group using the <a>DescribeLifecycleHooks</a> API call. If you are no longer using a lifecycle hook, you can delete it by calling the <a>DeleteLifecycleHook</a> API.</p>
    async fn put_lifecycle_hook(
        &self,
        input: PutLifecycleHookType,
    ) -> Result<PutLifecycleHookAnswer, RusotoError<PutLifecycleHookError>> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let params = self.new_params("PutLifecycleHook");
        let mut params = params;
        PutLifecycleHookTypeSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, PutLifecycleHookError::from_response)
            .await?;

        let result = PutLifecycleHookAnswer::default();

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p>Configures an Auto Scaling group to send notifications when specified events take place. Subscribers to the specified topic can have messages delivered to an endpoint such as a web server or an email address.</p> <p>This configuration overwrites any existing configuration.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/ASGettingNotifications.html">Getting Amazon SNS notifications when your Auto Scaling group scales</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p> <p>If you exceed your maximum limit of SNS topics, which is 10 per Auto Scaling group, the call fails.</p>
    async fn put_notification_configuration(
        &self,
        input: PutNotificationConfigurationType,
    ) -> Result<(), RusotoError<PutNotificationConfigurationError>> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let params = self.new_params("PutNotificationConfiguration");
        let mut params = params;
        PutNotificationConfigurationTypeSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, PutNotificationConfigurationError::from_response)
            .await?;

        std::mem::drop(response);
        Ok(())
    }

    /// <p>Creates or updates a scaling policy for an Auto Scaling group.</p> <p>For more information about using scaling policies to scale your Auto Scaling group, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/as-scaling-target-tracking.html">Target tracking scaling policies</a> and <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/as-scaling-simple-step.html">Step and simple scaling policies</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    async fn put_scaling_policy(
        &self,
        input: PutScalingPolicyType,
    ) -> Result<PolicyARNType, RusotoError<PutScalingPolicyError>> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let params = self.new_params("PutScalingPolicy");
        let mut params = params;
        PutScalingPolicyTypeSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, PutScalingPolicyError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = PolicyARNTypeDeserializer::deserialize("PutScalingPolicyResult", stack)?;
            skip_tree(stack);
            xml_util::end_element(actual_tag_name, stack)?;
            Ok(result)
        })
        .await?;

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p>Creates or updates a scheduled scaling action for an Auto Scaling group. If you leave a parameter unspecified when updating a scheduled scaling action, the corresponding value remains unchanged.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/schedule_time.html">Scheduled scaling</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    async fn put_scheduled_update_group_action(
        &self,
        input: PutScheduledUpdateGroupActionType,
    ) -> Result<(), RusotoError<PutScheduledUpdateGroupActionError>> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let params = self.new_params("PutScheduledUpdateGroupAction");
        let mut params = params;
        PutScheduledUpdateGroupActionTypeSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, PutScheduledUpdateGroupActionError::from_response)
            .await?;

        std::mem::drop(response);
        Ok(())
    }

    /// <p>Records a heartbeat for the lifecycle action associated with the specified token or instance. This extends the timeout by the length of time defined using the <a>PutLifecycleHook</a> API call.</p> <p>This step is a part of the procedure for adding a lifecycle hook to an Auto Scaling group:</p> <ol> <li> <p>(Optional) Create a Lambda function and a rule that allows CloudWatch Events to invoke your Lambda function when Amazon EC2 Auto Scaling launches or terminates instances.</p> </li> <li> <p>(Optional) Create a notification target and an IAM role. The target can be either an Amazon SQS queue or an Amazon SNS topic. The role allows Amazon EC2 Auto Scaling to publish lifecycle notifications to the target.</p> </li> <li> <p>Create the lifecycle hook. Specify whether the hook is used when the instances launch or terminate.</p> </li> <li> <p> <b>If you need more time, record the lifecycle action heartbeat to keep the instance in a pending state.</b> </p> </li> <li> <p>If you finish before the timeout period ends, complete the lifecycle action.</p> </li> </ol> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/AutoScalingGroupLifecycle.html">Auto Scaling lifecycle</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    async fn record_lifecycle_action_heartbeat(
        &self,
        input: RecordLifecycleActionHeartbeatType,
    ) -> Result<
        RecordLifecycleActionHeartbeatAnswer,
        RusotoError<RecordLifecycleActionHeartbeatError>,
    > {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let params = self.new_params("RecordLifecycleActionHeartbeat");
        let mut params = params;
        RecordLifecycleActionHeartbeatTypeSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, RecordLifecycleActionHeartbeatError::from_response)
            .await?;

        let result = RecordLifecycleActionHeartbeatAnswer::default();

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p>Resumes the specified suspended auto scaling processes, or all suspended process, for the specified Auto Scaling group.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/as-suspend-resume-processes.html">Suspending and resuming scaling processes</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    async fn resume_processes(
        &self,
        input: ScalingProcessQuery,
    ) -> Result<(), RusotoError<ResumeProcessesError>> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let params = self.new_params("ResumeProcesses");
        let mut params = params;
        ScalingProcessQuerySerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, ResumeProcessesError::from_response)
            .await?;

        std::mem::drop(response);
        Ok(())
    }

    /// <p>Sets the size of the specified Auto Scaling group.</p> <p>If a scale-in activity occurs as a result of a new <code>DesiredCapacity</code> value that is lower than the current size of the group, the Auto Scaling group uses its termination policy to determine which instances to terminate. </p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/as-manual-scaling.html">Manual scaling</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    async fn set_desired_capacity(
        &self,
        input: SetDesiredCapacityType,
    ) -> Result<(), RusotoError<SetDesiredCapacityError>> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let params = self.new_params("SetDesiredCapacity");
        let mut params = params;
        SetDesiredCapacityTypeSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, SetDesiredCapacityError::from_response)
            .await?;

        std::mem::drop(response);
        Ok(())
    }

    /// <p>Sets the health status of the specified instance.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/healthcheck.html">Health checks for Auto Scaling instances</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    async fn set_instance_health(
        &self,
        input: SetInstanceHealthQuery,
    ) -> Result<(), RusotoError<SetInstanceHealthError>> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let params = self.new_params("SetInstanceHealth");
        let mut params = params;
        SetInstanceHealthQuerySerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, SetInstanceHealthError::from_response)
            .await?;

        std::mem::drop(response);
        Ok(())
    }

    /// <p>Updates the instance protection settings of the specified instances.</p> <p>For more information about preventing instances that are part of an Auto Scaling group from terminating on scale in, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/as-instance-termination.html#instance-protection">Instance scale-in protection</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p> <p>If you exceed your maximum limit of instance IDs, which is 50 per Auto Scaling group, the call fails.</p>
    async fn set_instance_protection(
        &self,
        input: SetInstanceProtectionQuery,
    ) -> Result<SetInstanceProtectionAnswer, RusotoError<SetInstanceProtectionError>> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let params = self.new_params("SetInstanceProtection");
        let mut params = params;
        SetInstanceProtectionQuerySerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, SetInstanceProtectionError::from_response)
            .await?;

        let result = SetInstanceProtectionAnswer::default();

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p>Starts a new instance refresh operation, which triggers a rolling replacement of all previously launched instances in the Auto Scaling group with a new group of instances.</p> <p>If successful, this call creates a new instance refresh request with a unique ID that you can use to track its progress. To query its status, call the <a>DescribeInstanceRefreshes</a> API. To describe the instance refreshes that have already run, call the <a>DescribeInstanceRefreshes</a> API. To cancel an instance refresh operation in progress, use the <a>CancelInstanceRefresh</a> API. </p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/asg-instance-refresh.html">Replacing Auto Scaling Instances Based on an Instance Refresh</a>.</p>
    async fn start_instance_refresh(
        &self,
        input: StartInstanceRefreshType,
    ) -> Result<StartInstanceRefreshAnswer, RusotoError<StartInstanceRefreshError>> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let params = self.new_params("StartInstanceRefresh");
        let mut params = params;
        StartInstanceRefreshTypeSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, StartInstanceRefreshError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = StartInstanceRefreshAnswerDeserializer::deserialize(
                "StartInstanceRefreshResult",
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

    /// <p>Suspends the specified auto scaling processes, or all processes, for the specified Auto Scaling group.</p> <p>If you suspend either the <code>Launch</code> or <code>Terminate</code> process types, it can prevent other process types from functioning properly. For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/as-suspend-resume-processes.html">Suspending and resuming scaling processes</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p> <p>To resume processes that have been suspended, call the <a>ResumeProcesses</a> API.</p>
    async fn suspend_processes(
        &self,
        input: ScalingProcessQuery,
    ) -> Result<(), RusotoError<SuspendProcessesError>> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let params = self.new_params("SuspendProcesses");
        let mut params = params;
        ScalingProcessQuerySerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, SuspendProcessesError::from_response)
            .await?;

        std::mem::drop(response);
        Ok(())
    }

    /// <p>Terminates the specified instance and optionally adjusts the desired group size. </p> <p>This call simply makes a termination request. The instance is not terminated immediately. When an instance is terminated, the instance status changes to <code>terminated</code>. You can't connect to or start an instance after you've terminated it.</p> <p>If you do not specify the option to decrement the desired capacity, Amazon EC2 Auto Scaling launches instances to replace the ones that are terminated. </p> <p>By default, Amazon EC2 Auto Scaling balances instances across all Availability Zones. If you decrement the desired capacity, your Auto Scaling group can become unbalanced between Availability Zones. Amazon EC2 Auto Scaling tries to rebalance the group, and rebalancing might terminate instances in other zones. For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/auto-scaling-benefits.html#AutoScalingBehavior.InstanceUsage">Rebalancing activities</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    async fn terminate_instance_in_auto_scaling_group(
        &self,
        input: TerminateInstanceInAutoScalingGroupType,
    ) -> Result<ActivityType, RusotoError<TerminateInstanceInAutoScalingGroupError>> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let params = self.new_params("TerminateInstanceInAutoScalingGroup");
        let mut params = params;
        TerminateInstanceInAutoScalingGroupTypeSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(
                request,
                TerminateInstanceInAutoScalingGroupError::from_response,
            )
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = ActivityTypeDeserializer::deserialize(
                "TerminateInstanceInAutoScalingGroupResult",
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

    /// <p>Updates the configuration for the specified Auto Scaling group.</p> <p>To update an Auto Scaling group, specify the name of the group and the parameter that you want to change. Any parameters that you don't specify are not changed by this update request. The new settings take effect on any scaling activities after this call returns. </p> <p>If you associate a new launch configuration or template with an Auto Scaling group, all new instances will get the updated configuration. Existing instances continue to run with the configuration that they were originally launched with. When you update a group to specify a mixed instances policy instead of a launch configuration or template, existing instances may be replaced to match the new purchasing options that you specified in the policy. For example, if the group currently has 100% On-Demand capacity and the policy specifies 50% Spot capacity, this means that half of your instances will be gradually terminated and relaunched as Spot Instances. When replacing instances, Amazon EC2 Auto Scaling launches new instances before terminating the old ones, so that updating your group does not compromise the performance or availability of your application.</p> <p>Note the following about changing <code>DesiredCapacity</code>, <code>MaxSize</code>, or <code>MinSize</code>:</p> <ul> <li> <p>If a scale-in activity occurs as a result of a new <code>DesiredCapacity</code> value that is lower than the current size of the group, the Auto Scaling group uses its termination policy to determine which instances to terminate.</p> </li> <li> <p>If you specify a new value for <code>MinSize</code> without specifying a value for <code>DesiredCapacity</code>, and the new <code>MinSize</code> is larger than the current size of the group, this sets the group's <code>DesiredCapacity</code> to the new <code>MinSize</code> value.</p> </li> <li> <p>If you specify a new value for <code>MaxSize</code> without specifying a value for <code>DesiredCapacity</code>, and the new <code>MaxSize</code> is smaller than the current size of the group, this sets the group's <code>DesiredCapacity</code> to the new <code>MaxSize</code> value.</p> </li> </ul> <p>To see which parameters have been set, call the <a>DescribeAutoScalingGroups</a> API. To view the scaling policies for an Auto Scaling group, call the <a>DescribePolicies</a> API. If the group has scaling policies, you can update them by calling the <a>PutScalingPolicy</a> API.</p>
    async fn update_auto_scaling_group(
        &self,
        input: UpdateAutoScalingGroupType,
    ) -> Result<(), RusotoError<UpdateAutoScalingGroupError>> {
        let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
        let params = self.new_params("UpdateAutoScalingGroup");
        let mut params = params;
        UpdateAutoScalingGroupTypeSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, UpdateAutoScalingGroupError::from_response)
            .await?;

        std::mem::drop(response);
        Ok(())
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
