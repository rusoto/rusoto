use std::collections::HashMap;
use std::str;
#[derive(Debug, Default)]
pub struct ExecutePolicyType {
	/// The metric value to compare to `BreachThreshold`. This enables you to execute
	/// a policy of type `StepScaling` and determine which step adjustment to use. For
	/// example, if the breach threshold is 50 and you want to use a step adjustment
	/// with a lower bound of 0 and an upper bound of 10, you can set the metric value
	/// to 59.
	/// If you specify a metric value that doesn't correspond to a step adjustment for
	/// the policy, the call returns an error.
	/// This parameter is required if the policy type is `StepScaling` and not
	/// supported otherwise.
	pub metric_value: Option<MetricScale>,
	/// The name or ARN of the policy.
	pub policy_name: ResourceName,
	/// The name or Amazon Resource Name (ARN) of the Auto Scaling group.
	pub auto_scaling_group_name: Option<ResourceName>,
	/// If this parameter is true, Auto Scaling waits for the cooldown period to
	/// complete before executing the policy. Otherwise, Auto Scaling executes the
	/// policy without waiting for the cooldown period to complete.
	/// This parameter is not supported if the policy type is `StepScaling`.
	/// For more information, see [Understanding Auto Scaling Cooldowns](http://docs.a
	/// ws.amazon.com/AutoScaling/latest/DeveloperGuide/Cooldown.html) in the _Auto
	/// Scaling Developer Guide_.
	pub honor_cooldown: Option<HonorCooldown>,
	/// The breach threshold for the alarm.
	/// This parameter is required if the policy type is `StepScaling` and not
	/// supported otherwise.
	pub breach_threshold: Option<MetricScale>,
}

/// Parse ExecutePolicyType from XML
struct ExecutePolicyTypeParser;
impl ExecutePolicyTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ExecutePolicyType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = ExecutePolicyType::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "MetricValue" {
				obj.metric_value = Some(try!(MetricScaleParser::parse_xml("MetricValue", stack)));
				continue;
			}
			if current_name == "PolicyName" {
				obj.policy_name = try!(ResourceNameParser::parse_xml("PolicyName", stack));
				continue;
			}
			if current_name == "AutoScalingGroupName" {
				obj.auto_scaling_group_name = Some(try!(ResourceNameParser::parse_xml("AutoScalingGroupName", stack)));
				continue;
			}
			if current_name == "HonorCooldown" {
				obj.honor_cooldown = Some(try!(HonorCooldownParser::parse_xml("HonorCooldown", stack)));
				continue;
			}
			if current_name == "BreachThreshold" {
				obj.breach_threshold = Some(try!(MetricScaleParser::parse_xml("BreachThreshold", stack)));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ExecutePolicyType contents to a SignedRequest
struct ExecutePolicyTypeWriter;
impl ExecutePolicyTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ExecutePolicyType) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		if let Some(ref obj) = obj.metric_value {
			MetricScaleWriter::write_params(params, &(prefix.to_string() + "MetricValue"), obj);
		}
		ResourceNameWriter::write_params(params, &(prefix.to_string() + "PolicyName"), &obj.policy_name);
		if let Some(ref obj) = obj.auto_scaling_group_name {
			ResourceNameWriter::write_params(params, &(prefix.to_string() + "AutoScalingGroupName"), obj);
		}
		if let Some(ref obj) = obj.honor_cooldown {
			HonorCooldownWriter::write_params(params, &(prefix.to_string() + "HonorCooldown"), obj);
		}
		if let Some(ref obj) = obj.breach_threshold {
			MetricScaleWriter::write_params(params, &(prefix.to_string() + "BreachThreshold"), obj);
		}
	}
}
#[derive(Debug, Default)]
pub struct DetachInstancesQuery {
	/// If `True`, the Auto Scaling group decrements the desired capacity value by the
	/// number of instances detached.
	pub should_decrement_desired_capacity: ShouldDecrementDesiredCapacity,
	/// The name of the group.
	pub auto_scaling_group_name: ResourceName,
	/// One or more instance IDs.
	pub instance_ids: Option<InstanceIds>,
}

/// Parse DetachInstancesQuery from XML
struct DetachInstancesQueryParser;
impl DetachInstancesQueryParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<DetachInstancesQuery, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = DetachInstancesQuery::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "ShouldDecrementDesiredCapacity" {
				obj.should_decrement_desired_capacity = try!(ShouldDecrementDesiredCapacityParser::parse_xml("ShouldDecrementDesiredCapacity", stack));
				continue;
			}
			if current_name == "AutoScalingGroupName" {
				obj.auto_scaling_group_name = try!(ResourceNameParser::parse_xml("AutoScalingGroupName", stack));
				continue;
			}
			if current_name == "XmlStringMaxLen16" {
				obj.instance_ids = Some(try!(InstanceIdsParser::parse_xml("XmlStringMaxLen16", stack)));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write DetachInstancesQuery contents to a SignedRequest
struct DetachInstancesQueryWriter;
impl DetachInstancesQueryWriter {
	fn write_params(params: &mut Params, name: &str, obj: &DetachInstancesQuery) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		ShouldDecrementDesiredCapacityWriter::write_params(params, &(prefix.to_string() + "ShouldDecrementDesiredCapacity"), &obj.should_decrement_desired_capacity);
		ResourceNameWriter::write_params(params, &(prefix.to_string() + "AutoScalingGroupName"), &obj.auto_scaling_group_name);
		if let Some(ref obj) = obj.instance_ids {
			InstanceIdsWriter::write_params(params, &(prefix.to_string() + "XmlStringMaxLen16"), obj);
		}
	}
}
pub type NotificationConfigurations = Vec<NotificationConfiguration>;
/// Parse NotificationConfigurations from XML
struct NotificationConfigurationsParser;
impl NotificationConfigurationsParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<NotificationConfigurations, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "NotificationConfiguration" {
			obj.push(try!(NotificationConfigurationParser::parse_xml("NotificationConfiguration", stack)));
		}
		Ok(obj)
	}
}
/// Write NotificationConfigurations contents to a SignedRequest
struct NotificationConfigurationsWriter;
impl NotificationConfigurationsWriter {
	fn write_params(params: &mut Params, name: &str, obj: &NotificationConfigurations) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			NotificationConfigurationWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
pub type MinAdjustmentMagnitude = i32;
/// Parse MinAdjustmentMagnitude from XML
struct MinAdjustmentMagnitudeParser;
impl MinAdjustmentMagnitudeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<MinAdjustmentMagnitude, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = i32::from_str(try!(characters(stack)).as_ref()).unwrap();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write MinAdjustmentMagnitude contents to a SignedRequest
struct MinAdjustmentMagnitudeWriter;
impl MinAdjustmentMagnitudeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &MinAdjustmentMagnitude) {
		params.put(name, &obj.to_string());
	}
}
pub type XmlStringMaxLen16 = String;
/// Parse XmlStringMaxLen16 from XML
struct XmlStringMaxLen16Parser;
impl XmlStringMaxLen16Parser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<XmlStringMaxLen16, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write XmlStringMaxLen16 contents to a SignedRequest
struct XmlStringMaxLen16Writer;
impl XmlStringMaxLen16Writer {
	fn write_params(params: &mut Params, name: &str, obj: &XmlStringMaxLen16) {
		params.put(name, obj);
	}
}
#[derive(Debug, Default)]
pub struct TerminateInstanceInAutoScalingGroupType {
	/// The ID of the EC2 instance.
	pub instance_id: XmlStringMaxLen16,
	/// If `true`, terminating this instance also decrements the size of the Auto
	/// Scaling group.
	pub should_decrement_desired_capacity: ShouldDecrementDesiredCapacity,
}

/// Parse TerminateInstanceInAutoScalingGroupType from XML
struct TerminateInstanceInAutoScalingGroupTypeParser;
impl TerminateInstanceInAutoScalingGroupTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<TerminateInstanceInAutoScalingGroupType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = TerminateInstanceInAutoScalingGroupType::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "InstanceId" {
				obj.instance_id = try!(XmlStringMaxLen16Parser::parse_xml("InstanceId", stack));
				continue;
			}
			if current_name == "ShouldDecrementDesiredCapacity" {
				obj.should_decrement_desired_capacity = try!(ShouldDecrementDesiredCapacityParser::parse_xml("ShouldDecrementDesiredCapacity", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write TerminateInstanceInAutoScalingGroupType contents to a SignedRequest
struct TerminateInstanceInAutoScalingGroupTypeWriter;
impl TerminateInstanceInAutoScalingGroupTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &TerminateInstanceInAutoScalingGroupType) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		XmlStringMaxLen16Writer::write_params(params, &(prefix.to_string() + "InstanceId"), &obj.instance_id);
		ShouldDecrementDesiredCapacityWriter::write_params(params, &(prefix.to_string() + "ShouldDecrementDesiredCapacity"), &obj.should_decrement_desired_capacity);
	}
}
#[derive(Debug, Default)]
pub struct ProcessesType {
	/// The names of the process types.
	pub processes: Processes,
}

/// Parse ProcessesType from XML
struct ProcessesTypeParser;
impl ProcessesTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ProcessesType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = ProcessesType::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "ProcessType" {
				obj.processes = try!(ProcessesParser::parse_xml("ProcessType", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ProcessesType contents to a SignedRequest
struct ProcessesTypeWriter;
impl ProcessesTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ProcessesType) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		ProcessesWriter::write_params(params, &(prefix.to_string() + "ProcessType"), &obj.processes);
	}
}
/// The Auto Scaling group can't be deleted because there are scaling activities
/// in progress.
#[derive(Debug, Default)]
pub struct ScalingActivityInProgressFault {
	pub message: XmlStringMaxLen255,
}

/// Parse ScalingActivityInProgressFault from XML
struct ScalingActivityInProgressFaultParser;
impl ScalingActivityInProgressFaultParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ScalingActivityInProgressFault, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = ScalingActivityInProgressFault::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "message" {
				obj.message = try!(XmlStringMaxLen255Parser::parse_xml("message", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ScalingActivityInProgressFault contents to a SignedRequest
struct ScalingActivityInProgressFaultWriter;
impl ScalingActivityInProgressFaultWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ScalingActivityInProgressFault) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		XmlStringMaxLen255Writer::write_params(params, &(prefix.to_string() + "message"), &obj.message);
	}
}
pub type AutoScalingNotificationTypes = Vec<XmlStringMaxLen255>;
/// Parse AutoScalingNotificationTypes from XML
struct AutoScalingNotificationTypesParser;
impl AutoScalingNotificationTypesParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<AutoScalingNotificationTypes, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "XmlStringMaxLen255" {
			obj.push(try!(XmlStringMaxLen255Parser::parse_xml("XmlStringMaxLen255", stack)));
		}
		Ok(obj)
	}
}
/// Write AutoScalingNotificationTypes contents to a SignedRequest
struct AutoScalingNotificationTypesWriter;
impl AutoScalingNotificationTypesWriter {
	fn write_params(params: &mut Params, name: &str, obj: &AutoScalingNotificationTypes) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			XmlStringMaxLen255Writer::write_params(params, key, &element);
			index += 1;
		}
	}
}
pub type Progress = i32;
/// Parse Progress from XML
struct ProgressParser;
impl ProgressParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<Progress, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = i32::from_str(try!(characters(stack)).as_ref()).unwrap();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write Progress contents to a SignedRequest
struct ProgressWriter;
impl ProgressWriter {
	fn write_params(params: &mut Params, name: &str, obj: &Progress) {
		params.put(name, &obj.to_string());
	}
}
pub type BlockDeviceEbsVolumeSize = i32;
/// Parse BlockDeviceEbsVolumeSize from XML
struct BlockDeviceEbsVolumeSizeParser;
impl BlockDeviceEbsVolumeSizeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<BlockDeviceEbsVolumeSize, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = i32::from_str(try!(characters(stack)).as_ref()).unwrap();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write BlockDeviceEbsVolumeSize contents to a SignedRequest
struct BlockDeviceEbsVolumeSizeWriter;
impl BlockDeviceEbsVolumeSizeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &BlockDeviceEbsVolumeSize) {
		params.put(name, &obj.to_string());
	}
}
#[derive(Debug, Default)]
pub struct AttachInstancesQuery {
	/// The name of the group.
	pub auto_scaling_group_name: ResourceName,
	/// One or more EC2 instance IDs.
	pub instance_ids: Option<InstanceIds>,
}

/// Parse AttachInstancesQuery from XML
struct AttachInstancesQueryParser;
impl AttachInstancesQueryParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<AttachInstancesQuery, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = AttachInstancesQuery::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "AutoScalingGroupName" {
				obj.auto_scaling_group_name = try!(ResourceNameParser::parse_xml("AutoScalingGroupName", stack));
				continue;
			}
			if current_name == "XmlStringMaxLen16" {
				obj.instance_ids = Some(try!(InstanceIdsParser::parse_xml("XmlStringMaxLen16", stack)));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write AttachInstancesQuery contents to a SignedRequest
struct AttachInstancesQueryWriter;
impl AttachInstancesQueryWriter {
	fn write_params(params: &mut Params, name: &str, obj: &AttachInstancesQuery) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		ResourceNameWriter::write_params(params, &(prefix.to_string() + "AutoScalingGroupName"), &obj.auto_scaling_group_name);
		if let Some(ref obj) = obj.instance_ids {
			InstanceIdsWriter::write_params(params, &(prefix.to_string() + "XmlStringMaxLen16"), obj);
		}
	}
}
pub type ShouldRespectGracePeriod = bool;
/// Parse ShouldRespectGracePeriod from XML
struct ShouldRespectGracePeriodParser;
impl ShouldRespectGracePeriodParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ShouldRespectGracePeriod, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = bool::from_str(try!(characters(stack)).as_ref()).unwrap();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ShouldRespectGracePeriod contents to a SignedRequest
struct ShouldRespectGracePeriodWriter;
impl ShouldRespectGracePeriodWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ShouldRespectGracePeriod) {
		params.put(name, &obj.to_string());
	}
}
pub type TagKey = String;
/// Parse TagKey from XML
struct TagKeyParser;
impl TagKeyParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<TagKey, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write TagKey contents to a SignedRequest
struct TagKeyWriter;
impl TagKeyWriter {
	fn write_params(params: &mut Params, name: &str, obj: &TagKey) {
		params.put(name, obj);
	}
}
pub type PropagateAtLaunch = bool;
/// Parse PropagateAtLaunch from XML
struct PropagateAtLaunchParser;
impl PropagateAtLaunchParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<PropagateAtLaunch, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = bool::from_str(try!(characters(stack)).as_ref()).unwrap();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write PropagateAtLaunch contents to a SignedRequest
struct PropagateAtLaunchWriter;
impl PropagateAtLaunchWriter {
	fn write_params(params: &mut Params, name: &str, obj: &PropagateAtLaunch) {
		params.put(name, &obj.to_string());
	}
}
#[derive(Debug, Default)]
pub struct ActivityType {
	/// A scaling activity.
	pub activity: Activity,
}

/// Parse ActivityType from XML
struct ActivityTypeParser;
impl ActivityTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ActivityType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = ActivityType::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "Activity" {
				obj.activity = try!(ActivityParser::parse_xml("Activity", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ActivityType contents to a SignedRequest
struct ActivityTypeWriter;
impl ActivityTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ActivityType) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		ActivityWriter::write_params(params, &(prefix.to_string() + "Activity"), &obj.activity);
	}
}
#[derive(Debug, Default)]
pub struct DescribeScalingActivitiesType {
	/// The activity IDs of the desired scaling activities. If this list is omitted,
	/// all activities are described. If the `AutoScalingGroupName` parameter is
	/// provided, the results are limited to that group. The list of requested
	/// activities cannot contain more than 50 items. If unknown activities are
	/// requested, they are ignored with no error.
	pub activity_ids: ActivityIds,
	/// The maximum number of items to return with this call.
	pub max_records: MaxRecords,
	/// The token for the next set of items to return. (You received this token from a
	/// previous call.)
	pub next_token: XmlString,
	/// The name of the group.
	pub auto_scaling_group_name: ResourceName,
}

/// Parse DescribeScalingActivitiesType from XML
struct DescribeScalingActivitiesTypeParser;
impl DescribeScalingActivitiesTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<DescribeScalingActivitiesType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = DescribeScalingActivitiesType::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "XmlString" {
				obj.activity_ids = try!(ActivityIdsParser::parse_xml("XmlString", stack));
				continue;
			}
			if current_name == "MaxRecords" {
				obj.max_records = try!(MaxRecordsParser::parse_xml("MaxRecords", stack));
				continue;
			}
			if current_name == "NextToken" {
				obj.next_token = try!(XmlStringParser::parse_xml("NextToken", stack));
				continue;
			}
			if current_name == "AutoScalingGroupName" {
				obj.auto_scaling_group_name = try!(ResourceNameParser::parse_xml("AutoScalingGroupName", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write DescribeScalingActivitiesType contents to a SignedRequest
struct DescribeScalingActivitiesTypeWriter;
impl DescribeScalingActivitiesTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &DescribeScalingActivitiesType) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		ActivityIdsWriter::write_params(params, &(prefix.to_string() + "XmlString"), &obj.activity_ids);
		MaxRecordsWriter::write_params(params, &(prefix.to_string() + "MaxRecords"), &obj.max_records);
		XmlStringWriter::write_params(params, &(prefix.to_string() + "NextToken"), &obj.next_token);
		ResourceNameWriter::write_params(params, &(prefix.to_string() + "AutoScalingGroupName"), &obj.auto_scaling_group_name);
	}
}
#[derive(Debug, Default)]
pub struct PoliciesType {
	/// The scaling policies.
	pub scaling_policies: ScalingPolicies,
	/// The token to use when requesting the next set of items. If there are no
	/// additional items to return, the string is empty.
	pub next_token: XmlString,
}

/// Parse PoliciesType from XML
struct PoliciesTypeParser;
impl PoliciesTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<PoliciesType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = PoliciesType::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "ScalingPolicy" {
				obj.scaling_policies = try!(ScalingPoliciesParser::parse_xml("ScalingPolicy", stack));
				continue;
			}
			if current_name == "NextToken" {
				obj.next_token = try!(XmlStringParser::parse_xml("NextToken", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write PoliciesType contents to a SignedRequest
struct PoliciesTypeWriter;
impl PoliciesTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &PoliciesType) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		ScalingPoliciesWriter::write_params(params, &(prefix.to_string() + "ScalingPolicy"), &obj.scaling_policies);
		XmlStringWriter::write_params(params, &(prefix.to_string() + "NextToken"), &obj.next_token);
	}
}
#[derive(Debug, Default)]
pub struct AttachLoadBalancersResultType;

/// Parse AttachLoadBalancersResultType from XML
struct AttachLoadBalancersResultTypeParser;
impl AttachLoadBalancersResultTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<AttachLoadBalancersResultType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = AttachLoadBalancersResultType::default();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write AttachLoadBalancersResultType contents to a SignedRequest
struct AttachLoadBalancersResultTypeWriter;
impl AttachLoadBalancersResultTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &AttachLoadBalancersResultType) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
	}
}
#[derive(Debug, Default)]
pub struct ScheduledActionsType {
	/// The token to use when requesting the next set of items. If there are no
	/// additional items to return, the string is empty.
	pub next_token: XmlString,
	/// The scheduled actions.
	pub scheduled_update_group_actions: ScheduledUpdateGroupActions,
}

/// Parse ScheduledActionsType from XML
struct ScheduledActionsTypeParser;
impl ScheduledActionsTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ScheduledActionsType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = ScheduledActionsType::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "NextToken" {
				obj.next_token = try!(XmlStringParser::parse_xml("NextToken", stack));
				continue;
			}
			if current_name == "ScheduledUpdateGroupAction" {
				obj.scheduled_update_group_actions = try!(ScheduledUpdateGroupActionsParser::parse_xml("ScheduledUpdateGroupAction", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ScheduledActionsType contents to a SignedRequest
struct ScheduledActionsTypeWriter;
impl ScheduledActionsTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ScheduledActionsType) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		XmlStringWriter::write_params(params, &(prefix.to_string() + "NextToken"), &obj.next_token);
		ScheduledUpdateGroupActionsWriter::write_params(params, &(prefix.to_string() + "ScheduledUpdateGroupAction"), &obj.scheduled_update_group_actions);
	}
}
pub type SecurityGroups = Vec<XmlString>;
/// Parse SecurityGroups from XML
struct SecurityGroupsParser;
impl SecurityGroupsParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<SecurityGroups, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "XmlString" {
			obj.push(try!(XmlStringParser::parse_xml("XmlString", stack)));
		}
		Ok(obj)
	}
}
/// Write SecurityGroups contents to a SignedRequest
struct SecurityGroupsWriter;
impl SecurityGroupsWriter {
	fn write_params(params: &mut Params, name: &str, obj: &SecurityGroups) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			XmlStringWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
pub type HealthCheckGracePeriod = i32;
/// Parse HealthCheckGracePeriod from XML
struct HealthCheckGracePeriodParser;
impl HealthCheckGracePeriodParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<HealthCheckGracePeriod, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = i32::from_str(try!(characters(stack)).as_ref()).unwrap();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write HealthCheckGracePeriod contents to a SignedRequest
struct HealthCheckGracePeriodWriter;
impl HealthCheckGracePeriodWriter {
	fn write_params(params: &mut Params, name: &str, obj: &HealthCheckGracePeriod) {
		params.put(name, &obj.to_string());
	}
}
pub type AutoScalingGroupDesiredCapacity = i32;
/// Parse AutoScalingGroupDesiredCapacity from XML
struct AutoScalingGroupDesiredCapacityParser;
impl AutoScalingGroupDesiredCapacityParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<AutoScalingGroupDesiredCapacity, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = i32::from_str(try!(characters(stack)).as_ref()).unwrap();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write AutoScalingGroupDesiredCapacity contents to a SignedRequest
struct AutoScalingGroupDesiredCapacityWriter;
impl AutoScalingGroupDesiredCapacityWriter {
	fn write_params(params: &mut Params, name: &str, obj: &AutoScalingGroupDesiredCapacity) {
		params.put(name, &obj.to_string());
	}
}
/// The Auto Scaling group or launch configuration can't be deleted because it is
/// in use.
#[derive(Debug, Default)]
pub struct ResourceInUseFault {
	pub message: XmlStringMaxLen255,
}

/// Parse ResourceInUseFault from XML
struct ResourceInUseFaultParser;
impl ResourceInUseFaultParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ResourceInUseFault, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = ResourceInUseFault::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "message" {
				obj.message = try!(XmlStringMaxLen255Parser::parse_xml("message", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ResourceInUseFault contents to a SignedRequest
struct ResourceInUseFaultWriter;
impl ResourceInUseFaultWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ResourceInUseFault) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		XmlStringMaxLen255Writer::write_params(params, &(prefix.to_string() + "message"), &obj.message);
	}
}
pub type LifecycleHookNames = Vec<AsciiStringMaxLen255>;
/// Parse LifecycleHookNames from XML
struct LifecycleHookNamesParser;
impl LifecycleHookNamesParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<LifecycleHookNames, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "AsciiStringMaxLen255" {
			obj.push(try!(AsciiStringMaxLen255Parser::parse_xml("AsciiStringMaxLen255", stack)));
		}
		Ok(obj)
	}
}
/// Write LifecycleHookNames contents to a SignedRequest
struct LifecycleHookNamesWriter;
impl LifecycleHookNamesWriter {
	fn write_params(params: &mut Params, name: &str, obj: &LifecycleHookNames) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			AsciiStringMaxLen255Writer::write_params(params, key, &element);
			index += 1;
		}
	}
}
#[derive(Debug, Default)]
pub struct LaunchConfigurationNameType {
	/// The name of the launch configuration.
	pub launch_configuration_name: ResourceName,
}

/// Parse LaunchConfigurationNameType from XML
struct LaunchConfigurationNameTypeParser;
impl LaunchConfigurationNameTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<LaunchConfigurationNameType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = LaunchConfigurationNameType::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "LaunchConfigurationName" {
				obj.launch_configuration_name = try!(ResourceNameParser::parse_xml("LaunchConfigurationName", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write LaunchConfigurationNameType contents to a SignedRequest
struct LaunchConfigurationNameTypeWriter;
impl LaunchConfigurationNameTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &LaunchConfigurationNameType) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		ResourceNameWriter::write_params(params, &(prefix.to_string() + "LaunchConfigurationName"), &obj.launch_configuration_name);
	}
}
pub type MaxNumberOfLaunchConfigurations = i32;
/// Parse MaxNumberOfLaunchConfigurations from XML
struct MaxNumberOfLaunchConfigurationsParser;
impl MaxNumberOfLaunchConfigurationsParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<MaxNumberOfLaunchConfigurations, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = i32::from_str(try!(characters(stack)).as_ref()).unwrap();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write MaxNumberOfLaunchConfigurations contents to a SignedRequest
struct MaxNumberOfLaunchConfigurationsWriter;
impl MaxNumberOfLaunchConfigurationsWriter {
	fn write_params(params: &mut Params, name: &str, obj: &MaxNumberOfLaunchConfigurations) {
		params.put(name, &obj.to_string());
	}
}
/// Describes an adjustment based on the difference between the value of the
/// aggregated CloudWatch metric and the breach threshold that you've defined for
/// the alarm.
/// For the following examples, suppose that you have an alarm with a breach
/// threshold of 50:
///   * If you want the adjustment to be triggered when the metric is greater than or equal to 50 and less than 60, specify a lower bound of 0 and an upper bound of 10.
///   * If you want the adjustment to be triggered when the metric is greater than 40 and less than or equal to 50, specify a lower bound of -10 and an upper bound of 0.
/// There are a few rules for the step adjustments for your step policy:
///   * The ranges of your step adjustments can't overlap or have a gap.
///   * At most one step adjustment can have a null lower bound. If one step adjustment has a negative lower bound, then there must be a step adjustment with a null lower bound.
///   * At most one step adjustment can have a null upper bound. If one step adjustment has a positive upper bound, then there must be a step adjustment with a null upper bound.
///   * The upper and lower bound can't be null in the same step adjustment.
#[derive(Debug, Default)]
pub struct StepAdjustment {
	/// The amount by which to scale, based on the specified adjustment type. A
	/// positive value adds to the current capacity while a negative number removes
	/// from the current capacity.
	pub scaling_adjustment: PolicyIncrement,
	/// The lower bound for the difference between the alarm threshold and the
	/// CloudWatch metric. If the metric value is above the breach threshold, the
	/// lower bound is inclusive (the metric must be greater than or equal to the
	/// threshold plus the lower bound). Otherwise, it is exclusive (the metric must
	/// be greater than the threshold plus the lower bound). A null value indicates
	/// negative infinity.
	pub metric_interval_lower_bound: Option<MetricScale>,
	/// The upper bound for the difference between the alarm threshold and the
	/// CloudWatch metric. If the metric value is above the breach threshold, the
	/// upper bound is exclusive (the metric must be less than the threshold plus the
	/// upper bound). Otherwise, it is inclusive (the metric must be less than or
	/// equal to the threshold plus the upper bound). A null value indicates positive
	/// infinity.
	/// The upper bound must be greater than the lower bound.
	pub metric_interval_upper_bound: Option<MetricScale>,
}

/// Parse StepAdjustment from XML
struct StepAdjustmentParser;
impl StepAdjustmentParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<StepAdjustment, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = StepAdjustment::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "ScalingAdjustment" {
				obj.scaling_adjustment = try!(PolicyIncrementParser::parse_xml("ScalingAdjustment", stack));
				continue;
			}
			if current_name == "MetricIntervalLowerBound" {
				obj.metric_interval_lower_bound = Some(try!(MetricScaleParser::parse_xml("MetricIntervalLowerBound", stack)));
				continue;
			}
			if current_name == "MetricIntervalUpperBound" {
				obj.metric_interval_upper_bound = Some(try!(MetricScaleParser::parse_xml("MetricIntervalUpperBound", stack)));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write StepAdjustment contents to a SignedRequest
struct StepAdjustmentWriter;
impl StepAdjustmentWriter {
	fn write_params(params: &mut Params, name: &str, obj: &StepAdjustment) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		PolicyIncrementWriter::write_params(params, &(prefix.to_string() + "ScalingAdjustment"), &obj.scaling_adjustment);
		if let Some(ref obj) = obj.metric_interval_lower_bound {
			MetricScaleWriter::write_params(params, &(prefix.to_string() + "MetricIntervalLowerBound"), obj);
		}
		if let Some(ref obj) = obj.metric_interval_upper_bound {
			MetricScaleWriter::write_params(params, &(prefix.to_string() + "MetricIntervalUpperBound"), obj);
		}
	}
}
pub type ScalingPolicies = Vec<ScalingPolicy>;
/// Parse ScalingPolicies from XML
struct ScalingPoliciesParser;
impl ScalingPoliciesParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ScalingPolicies, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "ScalingPolicy" {
			obj.push(try!(ScalingPolicyParser::parse_xml("ScalingPolicy", stack)));
		}
		Ok(obj)
	}
}
/// Write ScalingPolicies contents to a SignedRequest
struct ScalingPoliciesWriter;
impl ScalingPoliciesWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ScalingPolicies) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			ScalingPolicyWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
pub type BlockDeviceMappings = Vec<BlockDeviceMapping>;
/// Parse BlockDeviceMappings from XML
struct BlockDeviceMappingsParser;
impl BlockDeviceMappingsParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<BlockDeviceMappings, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "BlockDeviceMapping" {
			obj.push(try!(BlockDeviceMappingParser::parse_xml("BlockDeviceMapping", stack)));
		}
		Ok(obj)
	}
}
/// Write BlockDeviceMappings contents to a SignedRequest
struct BlockDeviceMappingsWriter;
impl BlockDeviceMappingsWriter {
	fn write_params(params: &mut Params, name: &str, obj: &BlockDeviceMappings) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			BlockDeviceMappingWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
#[derive(Debug, Default)]
pub struct DeleteLifecycleHookAnswer;

/// Parse DeleteLifecycleHookAnswer from XML
struct DeleteLifecycleHookAnswerParser;
impl DeleteLifecycleHookAnswerParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<DeleteLifecycleHookAnswer, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = DeleteLifecycleHookAnswer::default();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write DeleteLifecycleHookAnswer contents to a SignedRequest
struct DeleteLifecycleHookAnswerWriter;
impl DeleteLifecycleHookAnswerWriter {
	fn write_params(params: &mut Params, name: &str, obj: &DeleteLifecycleHookAnswer) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
	}
}
pub type ForceDelete = bool;
/// Parse ForceDelete from XML
struct ForceDeleteParser;
impl ForceDeleteParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ForceDelete, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = bool::from_str(try!(characters(stack)).as_ref()).unwrap();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ForceDelete contents to a SignedRequest
struct ForceDeleteWriter;
impl ForceDeleteWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ForceDelete) {
		params.put(name, &obj.to_string());
	}
}
/// Describes a launch configuration.
#[derive(Debug, Default)]
pub struct LaunchConfiguration {
	/// The user data available to the instances.
	pub user_data: Option<XmlStringUserData>,
	/// The name or Amazon Resource Name (ARN) of the instance profile associated with
	/// the IAM role for the instance.
	pub iam_instance_profile: Option<XmlStringMaxLen1600>,
	/// The ID of a ClassicLink-enabled VPC to link your EC2-Classic instances to.
	/// This parameter can only be used if you are launching EC2-Classic instances.
	/// For more information, see
	/// [ClassicLink](http://docs.aws.amazon.com/AWSEC2/latest/UserGuide/vpc-
	/// classiclink.html) in the _Amazon Elastic Compute Cloud User Guide_.
	pub classic_link_vpc_id: Option<XmlStringMaxLen255>,
	/// Controls whether the instance is optimized for EBS I/O (`true`) or not
	/// (`false`).
	pub ebs_optimized: Option<EbsOptimized>,
	/// The tenancy of the instance, either `default` or `dedicated`. An instance with
	/// `dedicated` tenancy runs in an isolated, single-tenant hardware and can only
	/// be launched into a VPC.
	pub placement_tenancy: Option<XmlStringMaxLen64>,
	/// The Amazon Resource Name (ARN) of the launch configuration.
	pub launch_configuration_arn: Option<ResourceName>,
	/// Controls whether instances in this group are launched with detailed
	/// monitoring.
	pub instance_monitoring: Option<InstanceMonitoring>,
	/// The ID of the Amazon Machine Image (AMI).
	pub image_id: XmlStringMaxLen255,
	/// The creation date and time for the launch configuration.
	pub created_time: TimestampType,
	/// A block device mapping, which specifies the block devices for the instance.
	pub block_device_mappings: Option<BlockDeviceMappings>,
	/// The name of the key pair.
	pub key_name: Option<XmlStringMaxLen255>,
	/// The security groups to associate with the instances.
	pub security_groups: Option<SecurityGroups>,
	/// Specifies whether the instances are associated with a public IP address
	/// (`true`) or not (`false`).
	pub associate_public_ip_address: Option<AssociatePublicIpAddress>,
	/// The name of the launch configuration.
	pub launch_configuration_name: XmlStringMaxLen255,
	/// The ID of the kernel associated with the AMI.
	pub kernel_id: Option<XmlStringMaxLen255>,
	/// The ID of the RAM disk associated with the AMI.
	pub ramdisk_id: Option<XmlStringMaxLen255>,
	/// The IDs of one or more security groups for the VPC specified in
	/// `ClassicLinkVPCId`. This parameter is required if `ClassicLinkVPCId` is
	/// specified, and cannot be used otherwise. For more information, see
	/// [ClassicLink](http://docs.aws.amazon.com/AWSEC2/latest/UserGuide/vpc-
	/// classiclink.html) in the _Amazon Elastic Compute Cloud User Guide_.
	pub classic_link_vpc_security_groups: Option<ClassicLinkVPCSecurityGroups>,
	/// The instance type for the instances.
	pub instance_type: XmlStringMaxLen255,
	/// The price to bid when launching Spot Instances.
	pub spot_price: Option<SpotPrice>,
}

/// Parse LaunchConfiguration from XML
struct LaunchConfigurationParser;
impl LaunchConfigurationParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<LaunchConfiguration, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = LaunchConfiguration::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "UserData" {
				obj.user_data = Some(try!(XmlStringUserDataParser::parse_xml("UserData", stack)));
				continue;
			}
			if current_name == "IamInstanceProfile" {
				obj.iam_instance_profile = Some(try!(XmlStringMaxLen1600Parser::parse_xml("IamInstanceProfile", stack)));
				continue;
			}
			if current_name == "ClassicLinkVPCId" {
				obj.classic_link_vpc_id = Some(try!(XmlStringMaxLen255Parser::parse_xml("ClassicLinkVPCId", stack)));
				continue;
			}
			if current_name == "EbsOptimized" {
				obj.ebs_optimized = Some(try!(EbsOptimizedParser::parse_xml("EbsOptimized", stack)));
				continue;
			}
			if current_name == "PlacementTenancy" {
				obj.placement_tenancy = Some(try!(XmlStringMaxLen64Parser::parse_xml("PlacementTenancy", stack)));
				continue;
			}
			if current_name == "LaunchConfigurationARN" {
				obj.launch_configuration_arn = Some(try!(ResourceNameParser::parse_xml("LaunchConfigurationARN", stack)));
				continue;
			}
			if current_name == "InstanceMonitoring" {
				obj.instance_monitoring = Some(try!(InstanceMonitoringParser::parse_xml("InstanceMonitoring", stack)));
				continue;
			}
			if current_name == "ImageId" {
				obj.image_id = try!(XmlStringMaxLen255Parser::parse_xml("ImageId", stack));
				continue;
			}
			if current_name == "CreatedTime" {
				obj.created_time = try!(TimestampTypeParser::parse_xml("CreatedTime", stack));
				continue;
			}
			if current_name == "BlockDeviceMapping" {
				obj.block_device_mappings = Some(try!(BlockDeviceMappingsParser::parse_xml("BlockDeviceMapping", stack)));
				continue;
			}
			if current_name == "KeyName" {
				obj.key_name = Some(try!(XmlStringMaxLen255Parser::parse_xml("KeyName", stack)));
				continue;
			}
			if current_name == "XmlString" {
				obj.security_groups = Some(try!(SecurityGroupsParser::parse_xml("XmlString", stack)));
				continue;
			}
			if current_name == "AssociatePublicIpAddress" {
				obj.associate_public_ip_address = Some(try!(AssociatePublicIpAddressParser::parse_xml("AssociatePublicIpAddress", stack)));
				continue;
			}
			if current_name == "LaunchConfigurationName" {
				obj.launch_configuration_name = try!(XmlStringMaxLen255Parser::parse_xml("LaunchConfigurationName", stack));
				continue;
			}
			if current_name == "KernelId" {
				obj.kernel_id = Some(try!(XmlStringMaxLen255Parser::parse_xml("KernelId", stack)));
				continue;
			}
			if current_name == "RamdiskId" {
				obj.ramdisk_id = Some(try!(XmlStringMaxLen255Parser::parse_xml("RamdiskId", stack)));
				continue;
			}
			if current_name == "XmlStringMaxLen255" {
				obj.classic_link_vpc_security_groups = Some(try!(ClassicLinkVPCSecurityGroupsParser::parse_xml("XmlStringMaxLen255", stack)));
				continue;
			}
			if current_name == "InstanceType" {
				obj.instance_type = try!(XmlStringMaxLen255Parser::parse_xml("InstanceType", stack));
				continue;
			}
			if current_name == "SpotPrice" {
				obj.spot_price = Some(try!(SpotPriceParser::parse_xml("SpotPrice", stack)));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write LaunchConfiguration contents to a SignedRequest
struct LaunchConfigurationWriter;
impl LaunchConfigurationWriter {
	fn write_params(params: &mut Params, name: &str, obj: &LaunchConfiguration) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		if let Some(ref obj) = obj.user_data {
			XmlStringUserDataWriter::write_params(params, &(prefix.to_string() + "UserData"), obj);
		}
		if let Some(ref obj) = obj.iam_instance_profile {
			XmlStringMaxLen1600Writer::write_params(params, &(prefix.to_string() + "IamInstanceProfile"), obj);
		}
		if let Some(ref obj) = obj.classic_link_vpc_id {
			XmlStringMaxLen255Writer::write_params(params, &(prefix.to_string() + "ClassicLinkVPCId"), obj);
		}
		if let Some(ref obj) = obj.ebs_optimized {
			EbsOptimizedWriter::write_params(params, &(prefix.to_string() + "EbsOptimized"), obj);
		}
		if let Some(ref obj) = obj.placement_tenancy {
			XmlStringMaxLen64Writer::write_params(params, &(prefix.to_string() + "PlacementTenancy"), obj);
		}
		if let Some(ref obj) = obj.launch_configuration_arn {
			ResourceNameWriter::write_params(params, &(prefix.to_string() + "LaunchConfigurationARN"), obj);
		}
		if let Some(ref obj) = obj.instance_monitoring {
			InstanceMonitoringWriter::write_params(params, &(prefix.to_string() + "InstanceMonitoring"), obj);
		}
		XmlStringMaxLen255Writer::write_params(params, &(prefix.to_string() + "ImageId"), &obj.image_id);
		TimestampTypeWriter::write_params(params, &(prefix.to_string() + "CreatedTime"), &obj.created_time);
		if let Some(ref obj) = obj.block_device_mappings {
			BlockDeviceMappingsWriter::write_params(params, &(prefix.to_string() + "BlockDeviceMapping"), obj);
		}
		if let Some(ref obj) = obj.key_name {
			XmlStringMaxLen255Writer::write_params(params, &(prefix.to_string() + "KeyName"), obj);
		}
		if let Some(ref obj) = obj.security_groups {
			SecurityGroupsWriter::write_params(params, &(prefix.to_string() + "XmlString"), obj);
		}
		if let Some(ref obj) = obj.associate_public_ip_address {
			AssociatePublicIpAddressWriter::write_params(params, &(prefix.to_string() + "AssociatePublicIpAddress"), obj);
		}
		XmlStringMaxLen255Writer::write_params(params, &(prefix.to_string() + "LaunchConfigurationName"), &obj.launch_configuration_name);
		if let Some(ref obj) = obj.kernel_id {
			XmlStringMaxLen255Writer::write_params(params, &(prefix.to_string() + "KernelId"), obj);
		}
		if let Some(ref obj) = obj.ramdisk_id {
			XmlStringMaxLen255Writer::write_params(params, &(prefix.to_string() + "RamdiskId"), obj);
		}
		if let Some(ref obj) = obj.classic_link_vpc_security_groups {
			ClassicLinkVPCSecurityGroupsWriter::write_params(params, &(prefix.to_string() + "XmlStringMaxLen255"), obj);
		}
		XmlStringMaxLen255Writer::write_params(params, &(prefix.to_string() + "InstanceType"), &obj.instance_type);
		if let Some(ref obj) = obj.spot_price {
			SpotPriceWriter::write_params(params, &(prefix.to_string() + "SpotPrice"), obj);
		}
	}
}
pub type AssociatePublicIpAddress = bool;
/// Parse AssociatePublicIpAddress from XML
struct AssociatePublicIpAddressParser;
impl AssociatePublicIpAddressParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<AssociatePublicIpAddress, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = bool::from_str(try!(characters(stack)).as_ref()).unwrap();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write AssociatePublicIpAddress contents to a SignedRequest
struct AssociatePublicIpAddressWriter;
impl AssociatePublicIpAddressWriter {
	fn write_params(params: &mut Params, name: &str, obj: &AssociatePublicIpAddress) {
		params.put(name, &obj.to_string());
	}
}
#[derive(Debug, Default)]
pub struct ScalingProcessQuery {
	/// The name or Amazon Resource Name (ARN) of the Auto Scaling group.
	pub auto_scaling_group_name: ResourceName,
	/// One or more of the following processes:
	///   * `Launch`
	///   * `Terminate`
	///   * `HealthCheck`
	///   * `ReplaceUnhealthy`
	///   * `AZRebalance`
	///   * `AlarmNotification`
	///   * `ScheduledActions`
	///   * `AddToLoadBalancer`
	pub scaling_processes: Option<ProcessNames>,
}

/// Parse ScalingProcessQuery from XML
struct ScalingProcessQueryParser;
impl ScalingProcessQueryParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ScalingProcessQuery, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = ScalingProcessQuery::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "AutoScalingGroupName" {
				obj.auto_scaling_group_name = try!(ResourceNameParser::parse_xml("AutoScalingGroupName", stack));
				continue;
			}
			if current_name == "XmlStringMaxLen255" {
				obj.scaling_processes = Some(try!(ProcessNamesParser::parse_xml("XmlStringMaxLen255", stack)));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ScalingProcessQuery contents to a SignedRequest
struct ScalingProcessQueryWriter;
impl ScalingProcessQueryWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ScalingProcessQuery) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		ResourceNameWriter::write_params(params, &(prefix.to_string() + "AutoScalingGroupName"), &obj.auto_scaling_group_name);
		if let Some(ref obj) = obj.scaling_processes {
			ProcessNamesWriter::write_params(params, &(prefix.to_string() + "XmlStringMaxLen255"), obj);
		}
	}
}
pub type EnabledMetrics = Vec<EnabledMetric>;
/// Parse EnabledMetrics from XML
struct EnabledMetricsParser;
impl EnabledMetricsParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<EnabledMetrics, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "EnabledMetric" {
			obj.push(try!(EnabledMetricParser::parse_xml("EnabledMetric", stack)));
		}
		Ok(obj)
	}
}
/// Write EnabledMetrics contents to a SignedRequest
struct EnabledMetricsWriter;
impl EnabledMetricsWriter {
	fn write_params(params: &mut Params, name: &str, obj: &EnabledMetrics) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			EnabledMetricWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
pub type AutoScalingInstances = Vec<AutoScalingInstanceDetails>;
/// Parse AutoScalingInstances from XML
struct AutoScalingInstancesParser;
impl AutoScalingInstancesParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<AutoScalingInstances, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "AutoScalingInstanceDetails" {
			obj.push(try!(AutoScalingInstanceDetailsParser::parse_xml("AutoScalingInstanceDetails", stack)));
		}
		Ok(obj)
	}
}
/// Write AutoScalingInstances contents to a SignedRequest
struct AutoScalingInstancesWriter;
impl AutoScalingInstancesWriter {
	fn write_params(params: &mut Params, name: &str, obj: &AutoScalingInstances) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			AutoScalingInstanceDetailsWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
#[derive(Debug, Default)]
pub struct DescribeAccountLimitsAnswer {
	/// The maximum number of launch configurations allowed for your AWS account. The
	/// default limit is 100 per region.
	pub max_number_of_launch_configurations: MaxNumberOfLaunchConfigurations,
	/// The maximum number of groups allowed for your AWS account. The default limit
	/// is 20 per region.
	pub max_number_of_auto_scaling_groups: MaxNumberOfAutoScalingGroups,
}

/// Parse DescribeAccountLimitsAnswer from XML
struct DescribeAccountLimitsAnswerParser;
impl DescribeAccountLimitsAnswerParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<DescribeAccountLimitsAnswer, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = DescribeAccountLimitsAnswer::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "MaxNumberOfLaunchConfigurations" {
				obj.max_number_of_launch_configurations = try!(MaxNumberOfLaunchConfigurationsParser::parse_xml("MaxNumberOfLaunchConfigurations", stack));
				continue;
			}
			if current_name == "MaxNumberOfAutoScalingGroups" {
				obj.max_number_of_auto_scaling_groups = try!(MaxNumberOfAutoScalingGroupsParser::parse_xml("MaxNumberOfAutoScalingGroups", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write DescribeAccountLimitsAnswer contents to a SignedRequest
struct DescribeAccountLimitsAnswerWriter;
impl DescribeAccountLimitsAnswerWriter {
	fn write_params(params: &mut Params, name: &str, obj: &DescribeAccountLimitsAnswer) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		MaxNumberOfLaunchConfigurationsWriter::write_params(params, &(prefix.to_string() + "MaxNumberOfLaunchConfigurations"), &obj.max_number_of_launch_configurations);
		MaxNumberOfAutoScalingGroupsWriter::write_params(params, &(prefix.to_string() + "MaxNumberOfAutoScalingGroups"), &obj.max_number_of_auto_scaling_groups);
	}
}
pub type AutoScalingGroupNames = Vec<ResourceName>;
/// Parse AutoScalingGroupNames from XML
struct AutoScalingGroupNamesParser;
impl AutoScalingGroupNamesParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<AutoScalingGroupNames, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "ResourceName" {
			obj.push(try!(ResourceNameParser::parse_xml("ResourceName", stack)));
		}
		Ok(obj)
	}
}
/// Write AutoScalingGroupNames contents to a SignedRequest
struct AutoScalingGroupNamesWriter;
impl AutoScalingGroupNamesWriter {
	fn write_params(params: &mut Params, name: &str, obj: &AutoScalingGroupNames) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			ResourceNameWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
/// Describes a tag for an Auto Scaling group.
#[derive(Debug, Default)]
pub struct TagDescription {
	/// The type of resource. The only supported value is `auto-scaling-group`.
	pub resource_type: XmlString,
	/// The name of the group.
	pub resource_id: XmlString,
	/// Determines whether the tag is added to new instances as they are launched in
	/// the group.
	pub propagate_at_launch: PropagateAtLaunch,
	/// The tag value.
	pub value: TagValue,
	/// The tag key.
	pub key: TagKey,
}

/// Parse TagDescription from XML
struct TagDescriptionParser;
impl TagDescriptionParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<TagDescription, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = TagDescription::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "ResourceType" {
				obj.resource_type = try!(XmlStringParser::parse_xml("ResourceType", stack));
				continue;
			}
			if current_name == "ResourceId" {
				obj.resource_id = try!(XmlStringParser::parse_xml("ResourceId", stack));
				continue;
			}
			if current_name == "PropagateAtLaunch" {
				obj.propagate_at_launch = try!(PropagateAtLaunchParser::parse_xml("PropagateAtLaunch", stack));
				continue;
			}
			if current_name == "Value" {
				obj.value = try!(TagValueParser::parse_xml("Value", stack));
				continue;
			}
			if current_name == "Key" {
				obj.key = try!(TagKeyParser::parse_xml("Key", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write TagDescription contents to a SignedRequest
struct TagDescriptionWriter;
impl TagDescriptionWriter {
	fn write_params(params: &mut Params, name: &str, obj: &TagDescription) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		XmlStringWriter::write_params(params, &(prefix.to_string() + "ResourceType"), &obj.resource_type);
		XmlStringWriter::write_params(params, &(prefix.to_string() + "ResourceId"), &obj.resource_id);
		PropagateAtLaunchWriter::write_params(params, &(prefix.to_string() + "PropagateAtLaunch"), &obj.propagate_at_launch);
		TagValueWriter::write_params(params, &(prefix.to_string() + "Value"), &obj.value);
		TagKeyWriter::write_params(params, &(prefix.to_string() + "Key"), &obj.key);
	}
}
/// Describes a tag for an Auto Scaling group.
#[derive(Debug, Default)]
pub struct Tag {
	/// The type of resource. The only supported value is `auto-scaling-group`.
	pub resource_type: Option<XmlString>,
	/// The name of the group.
	pub resource_id: Option<XmlString>,
	/// Determines whether the tag is added to new instances as they are launched in
	/// the group.
	pub propagate_at_launch: Option<PropagateAtLaunch>,
	/// The tag value.
	pub value: Option<TagValue>,
	/// The tag key.
	pub key: TagKey,
}

/// Parse Tag from XML
struct TagParser;
impl TagParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<Tag, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = Tag::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "ResourceType" {
				obj.resource_type = Some(try!(XmlStringParser::parse_xml("ResourceType", stack)));
				continue;
			}
			if current_name == "ResourceId" {
				obj.resource_id = Some(try!(XmlStringParser::parse_xml("ResourceId", stack)));
				continue;
			}
			if current_name == "PropagateAtLaunch" {
				obj.propagate_at_launch = Some(try!(PropagateAtLaunchParser::parse_xml("PropagateAtLaunch", stack)));
				continue;
			}
			if current_name == "Value" {
				obj.value = Some(try!(TagValueParser::parse_xml("Value", stack)));
				continue;
			}
			if current_name == "Key" {
				obj.key = try!(TagKeyParser::parse_xml("Key", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write Tag contents to a SignedRequest
struct TagWriter;
impl TagWriter {
	fn write_params(params: &mut Params, name: &str, obj: &Tag) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		if let Some(ref obj) = obj.resource_type {
			XmlStringWriter::write_params(params, &(prefix.to_string() + "ResourceType"), obj);
		}
		if let Some(ref obj) = obj.resource_id {
			XmlStringWriter::write_params(params, &(prefix.to_string() + "ResourceId"), obj);
		}
		if let Some(ref obj) = obj.propagate_at_launch {
			PropagateAtLaunchWriter::write_params(params, &(prefix.to_string() + "PropagateAtLaunch"), obj);
		}
		if let Some(ref obj) = obj.value {
			TagValueWriter::write_params(params, &(prefix.to_string() + "Value"), obj);
		}
		TagKeyWriter::write_params(params, &(prefix.to_string() + "Key"), &obj.key);
	}
}
#[derive(Debug, Default)]
pub struct CreateOrUpdateTagsType {
	/// One or more tags.
	pub tags: Tags,
}

/// Parse CreateOrUpdateTagsType from XML
struct CreateOrUpdateTagsTypeParser;
impl CreateOrUpdateTagsTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<CreateOrUpdateTagsType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = CreateOrUpdateTagsType::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "Tag" {
				obj.tags = try!(TagsParser::parse_xml("Tag", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write CreateOrUpdateTagsType contents to a SignedRequest
struct CreateOrUpdateTagsTypeWriter;
impl CreateOrUpdateTagsTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &CreateOrUpdateTagsType) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		TagsWriter::write_params(params, &(prefix.to_string() + "Tag"), &obj.tags);
	}
}
/// Describes an Amazon EBS volume.
#[derive(Debug, Default)]
pub struct Ebs {
	/// Indicates whether to delete the volume on instance termination.
	/// Default: `true`
	pub delete_on_termination: BlockDeviceEbsDeleteOnTermination,
	/// The ID of the snapshot.
	pub snapshot_id: XmlStringMaxLen255,
	/// The volume size, in gigabytes.
	/// Valid values: If the volume type is `io1`, the minimum size of the volume is
	/// 10 GiB. If you specify `SnapshotId` and `VolumeSize`, `VolumeSize` must be
	/// equal to or larger than the size of the snapshot.
	/// Default: If you create a volume from a snapshot and you don't specify a volume
	/// size, the default is the size of the snapshot.
	/// Required: Required when the volume type is `io1`.
	pub volume_size: BlockDeviceEbsVolumeSize,
	/// The volume type.
	/// Valid values: `standard | io1 | gp2`
	/// Default: `standard`
	pub volume_type: BlockDeviceEbsVolumeType,
	/// For Provisioned IOPS (SSD) volumes only. The number of I/O operations per
	/// second (IOPS) to provision for the volume.
	/// Valid values: Range is 100 to 4000.
	/// Default: None
	pub iops: BlockDeviceEbsIops,
}

/// Parse Ebs from XML
struct EbsParser;
impl EbsParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<Ebs, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = Ebs::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "DeleteOnTermination" {
				obj.delete_on_termination = try!(BlockDeviceEbsDeleteOnTerminationParser::parse_xml("DeleteOnTermination", stack));
				continue;
			}
			if current_name == "SnapshotId" {
				obj.snapshot_id = try!(XmlStringMaxLen255Parser::parse_xml("SnapshotId", stack));
				continue;
			}
			if current_name == "VolumeSize" {
				obj.volume_size = try!(BlockDeviceEbsVolumeSizeParser::parse_xml("VolumeSize", stack));
				continue;
			}
			if current_name == "VolumeType" {
				obj.volume_type = try!(BlockDeviceEbsVolumeTypeParser::parse_xml("VolumeType", stack));
				continue;
			}
			if current_name == "Iops" {
				obj.iops = try!(BlockDeviceEbsIopsParser::parse_xml("Iops", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write Ebs contents to a SignedRequest
struct EbsWriter;
impl EbsWriter {
	fn write_params(params: &mut Params, name: &str, obj: &Ebs) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		BlockDeviceEbsDeleteOnTerminationWriter::write_params(params, &(prefix.to_string() + "DeleteOnTermination"), &obj.delete_on_termination);
		XmlStringMaxLen255Writer::write_params(params, &(prefix.to_string() + "SnapshotId"), &obj.snapshot_id);
		BlockDeviceEbsVolumeSizeWriter::write_params(params, &(prefix.to_string() + "VolumeSize"), &obj.volume_size);
		BlockDeviceEbsVolumeTypeWriter::write_params(params, &(prefix.to_string() + "VolumeType"), &obj.volume_type);
		BlockDeviceEbsIopsWriter::write_params(params, &(prefix.to_string() + "Iops"), &obj.iops);
	}
}
#[derive(Debug, Default)]
pub struct ExitStandbyQuery {
	/// The name of the Auto Scaling group.
	pub auto_scaling_group_name: ResourceName,
	/// One or more instance IDs. You must specify at least one instance ID.
	pub instance_ids: Option<InstanceIds>,
}

/// Parse ExitStandbyQuery from XML
struct ExitStandbyQueryParser;
impl ExitStandbyQueryParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ExitStandbyQuery, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = ExitStandbyQuery::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "AutoScalingGroupName" {
				obj.auto_scaling_group_name = try!(ResourceNameParser::parse_xml("AutoScalingGroupName", stack));
				continue;
			}
			if current_name == "XmlStringMaxLen16" {
				obj.instance_ids = Some(try!(InstanceIdsParser::parse_xml("XmlStringMaxLen16", stack)));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ExitStandbyQuery contents to a SignedRequest
struct ExitStandbyQueryWriter;
impl ExitStandbyQueryWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ExitStandbyQuery) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		ResourceNameWriter::write_params(params, &(prefix.to_string() + "AutoScalingGroupName"), &obj.auto_scaling_group_name);
		if let Some(ref obj) = obj.instance_ids {
			InstanceIdsWriter::write_params(params, &(prefix.to_string() + "XmlStringMaxLen16"), obj);
		}
	}
}
#[derive(Debug, Default)]
pub struct DescribeMetricCollectionTypesAnswer {
	/// One or more metrics.
	pub metrics: MetricCollectionTypes,
	/// The granularities for the metrics.
	pub granularities: MetricGranularityTypes,
}

/// Parse DescribeMetricCollectionTypesAnswer from XML
struct DescribeMetricCollectionTypesAnswerParser;
impl DescribeMetricCollectionTypesAnswerParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<DescribeMetricCollectionTypesAnswer, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = DescribeMetricCollectionTypesAnswer::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "MetricCollectionType" {
				obj.metrics = try!(MetricCollectionTypesParser::parse_xml("MetricCollectionType", stack));
				continue;
			}
			if current_name == "MetricGranularityType" {
				obj.granularities = try!(MetricGranularityTypesParser::parse_xml("MetricGranularityType", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write DescribeMetricCollectionTypesAnswer contents to a SignedRequest
struct DescribeMetricCollectionTypesAnswerWriter;
impl DescribeMetricCollectionTypesAnswerWriter {
	fn write_params(params: &mut Params, name: &str, obj: &DescribeMetricCollectionTypesAnswer) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		MetricCollectionTypesWriter::write_params(params, &(prefix.to_string() + "MetricCollectionType"), &obj.metrics);
		MetricGranularityTypesWriter::write_params(params, &(prefix.to_string() + "MetricGranularityType"), &obj.granularities);
	}
}
#[derive(Debug, Default)]
pub struct DeleteTagsType {
	/// Each tag should be defined by its resource type, resource ID, key, value, and
	/// a propagate flag. Valid values are: Resource type = _auto-scaling-group_,
	/// Resource ID = _AutoScalingGroupName_, key=_value_, value=_value_,
	/// propagate=_true_ or _false_.
	pub tags: Tags,
}

/// Parse DeleteTagsType from XML
struct DeleteTagsTypeParser;
impl DeleteTagsTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<DeleteTagsType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = DeleteTagsType::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "Tag" {
				obj.tags = try!(TagsParser::parse_xml("Tag", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write DeleteTagsType contents to a SignedRequest
struct DeleteTagsTypeWriter;
impl DeleteTagsTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &DeleteTagsType) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		TagsWriter::write_params(params, &(prefix.to_string() + "Tag"), &obj.tags);
	}
}
/// You already have an Auto Scaling group or launch configuration with this name.
#[derive(Debug, Default)]
pub struct AlreadyExistsFault {
	pub message: XmlStringMaxLen255,
}

/// Parse AlreadyExistsFault from XML
struct AlreadyExistsFaultParser;
impl AlreadyExistsFaultParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<AlreadyExistsFault, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = AlreadyExistsFault::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "message" {
				obj.message = try!(XmlStringMaxLen255Parser::parse_xml("message", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write AlreadyExistsFault contents to a SignedRequest
struct AlreadyExistsFaultWriter;
impl AlreadyExistsFaultWriter {
	fn write_params(params: &mut Params, name: &str, obj: &AlreadyExistsFault) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		XmlStringMaxLen255Writer::write_params(params, &(prefix.to_string() + "message"), &obj.message);
	}
}
pub type XmlStringMaxLen1600 = String;
/// Parse XmlStringMaxLen1600 from XML
struct XmlStringMaxLen1600Parser;
impl XmlStringMaxLen1600Parser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<XmlStringMaxLen1600, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write XmlStringMaxLen1600 contents to a SignedRequest
struct XmlStringMaxLen1600Writer;
impl XmlStringMaxLen1600Writer {
	fn write_params(params: &mut Params, name: &str, obj: &XmlStringMaxLen1600) {
		params.put(name, obj);
	}
}
pub type XmlStringMaxLen32 = String;
/// Parse XmlStringMaxLen32 from XML
struct XmlStringMaxLen32Parser;
impl XmlStringMaxLen32Parser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<XmlStringMaxLen32, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write XmlStringMaxLen32 contents to a SignedRequest
struct XmlStringMaxLen32Writer;
impl XmlStringMaxLen32Writer {
	fn write_params(params: &mut Params, name: &str, obj: &XmlStringMaxLen32) {
		params.put(name, obj);
	}
}
pub type ScalingActivityStatusCode = String;
/// Parse ScalingActivityStatusCode from XML
struct ScalingActivityStatusCodeParser;
impl ScalingActivityStatusCodeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ScalingActivityStatusCode, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ScalingActivityStatusCode contents to a SignedRequest
struct ScalingActivityStatusCodeWriter;
impl ScalingActivityStatusCodeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ScalingActivityStatusCode) {
		params.put(name, obj);
	}
}
/// Describes an Auto Scaling group.
#[derive(Debug, Default)]
pub struct AutoScalingGroup {
	/// The current state of the group when DeleteAutoScalingGroup is in progress.
	pub status: Option<XmlStringMaxLen255>,
	/// The Amazon Resource Name (ARN) of the group.
	pub auto_scaling_group_arn: Option<ResourceName>,
	/// The amount of time that Auto Scaling waits before checking an instance's
	/// health status. The grace period begins when an instance comes into service.
	pub health_check_grace_period: Option<HealthCheckGracePeriod>,
	/// The suspended processes associated with the group.
	pub suspended_processes: Option<SuspendedProcesses>,
	/// The desired size of the group.
	pub desired_capacity: AutoScalingGroupDesiredCapacity,
	/// The tags for the group.
	pub tags: Option<TagDescriptionList>,
	/// The metrics enabled for the group.
	pub enabled_metrics: Option<EnabledMetrics>,
	/// One or more load balancers associated with the group.
	pub load_balancer_names: Option<LoadBalancerNames>,
	/// The name of the group.
	pub auto_scaling_group_name: XmlStringMaxLen255,
	/// The number of seconds after a scaling activity completes before any further
	/// scaling activities can start.
	pub default_cooldown: Cooldown,
	/// The minimum size of the group.
	pub min_size: AutoScalingGroupMinSize,
	/// The EC2 instances associated with the group.
	pub instances: Option<Instances>,
	/// The maximum size of the group.
	pub max_size: AutoScalingGroupMaxSize,
	/// One or more subnet IDs, if applicable, separated by commas.
	/// If you specify `VPCZoneIdentifier` and `AvailabilityZones`, ensure that the
	/// Availability Zones of the subnets match the values for `AvailabilityZones`.
	pub vpc_zone_identifier: Option<XmlStringMaxLen255>,
	/// The termination policies for the group.
	pub termination_policies: Option<TerminationPolicies>,
	/// The name of the associated launch configuration.
	pub launch_configuration_name: XmlStringMaxLen255,
	/// The date and time the group was created.
	pub created_time: TimestampType,
	/// The name of the placement group into which you'll launch your instances, if
	/// any. For more information, see [Placement
	/// Groups](http://docs.aws.amazon.com/AWSEC2/latest/UserGuide/placement-
	/// groups.html).
	pub placement_group: Option<XmlStringMaxLen255>,
	/// One or more Availability Zones for the group.
	pub availability_zones: AvailabilityZones,
	/// The service of interest for the health status check, which can be either `EC2`
	/// for Amazon EC2 or `ELB` for Elastic Load Balancing.
	pub health_check_type: XmlStringMaxLen32,
}

/// Parse AutoScalingGroup from XML
struct AutoScalingGroupParser;
impl AutoScalingGroupParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<AutoScalingGroup, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = AutoScalingGroup::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "Status" {
				obj.status = Some(try!(XmlStringMaxLen255Parser::parse_xml("Status", stack)));
				continue;
			}
			if current_name == "AutoScalingGroupARN" {
				obj.auto_scaling_group_arn = Some(try!(ResourceNameParser::parse_xml("AutoScalingGroupARN", stack)));
				continue;
			}
			if current_name == "HealthCheckGracePeriod" {
				obj.health_check_grace_period = Some(try!(HealthCheckGracePeriodParser::parse_xml("HealthCheckGracePeriod", stack)));
				continue;
			}
			if current_name == "SuspendedProcess" {
				obj.suspended_processes = Some(try!(SuspendedProcessesParser::parse_xml("SuspendedProcess", stack)));
				continue;
			}
			if current_name == "DesiredCapacity" {
				obj.desired_capacity = try!(AutoScalingGroupDesiredCapacityParser::parse_xml("DesiredCapacity", stack));
				continue;
			}
			if current_name == "TagDescription" {
				obj.tags = Some(try!(TagDescriptionListParser::parse_xml("TagDescription", stack)));
				continue;
			}
			if current_name == "EnabledMetric" {
				obj.enabled_metrics = Some(try!(EnabledMetricsParser::parse_xml("EnabledMetric", stack)));
				continue;
			}
			if current_name == "XmlStringMaxLen255" {
				obj.load_balancer_names = Some(try!(LoadBalancerNamesParser::parse_xml("XmlStringMaxLen255", stack)));
				continue;
			}
			if current_name == "AutoScalingGroupName" {
				obj.auto_scaling_group_name = try!(XmlStringMaxLen255Parser::parse_xml("AutoScalingGroupName", stack));
				continue;
			}
			if current_name == "DefaultCooldown" {
				obj.default_cooldown = try!(CooldownParser::parse_xml("DefaultCooldown", stack));
				continue;
			}
			if current_name == "MinSize" {
				obj.min_size = try!(AutoScalingGroupMinSizeParser::parse_xml("MinSize", stack));
				continue;
			}
			if current_name == "Instance" {
				obj.instances = Some(try!(InstancesParser::parse_xml("Instance", stack)));
				continue;
			}
			if current_name == "MaxSize" {
				obj.max_size = try!(AutoScalingGroupMaxSizeParser::parse_xml("MaxSize", stack));
				continue;
			}
			if current_name == "VPCZoneIdentifier" {
				obj.vpc_zone_identifier = Some(try!(XmlStringMaxLen255Parser::parse_xml("VPCZoneIdentifier", stack)));
				continue;
			}
			if current_name == "XmlStringMaxLen1600" {
				obj.termination_policies = Some(try!(TerminationPoliciesParser::parse_xml("XmlStringMaxLen1600", stack)));
				continue;
			}
			if current_name == "LaunchConfigurationName" {
				obj.launch_configuration_name = try!(XmlStringMaxLen255Parser::parse_xml("LaunchConfigurationName", stack));
				continue;
			}
			if current_name == "CreatedTime" {
				obj.created_time = try!(TimestampTypeParser::parse_xml("CreatedTime", stack));
				continue;
			}
			if current_name == "PlacementGroup" {
				obj.placement_group = Some(try!(XmlStringMaxLen255Parser::parse_xml("PlacementGroup", stack)));
				continue;
			}
			if current_name == "XmlStringMaxLen255" {
				obj.availability_zones = try!(AvailabilityZonesParser::parse_xml("XmlStringMaxLen255", stack));
				continue;
			}
			if current_name == "HealthCheckType" {
				obj.health_check_type = try!(XmlStringMaxLen32Parser::parse_xml("HealthCheckType", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write AutoScalingGroup contents to a SignedRequest
struct AutoScalingGroupWriter;
impl AutoScalingGroupWriter {
	fn write_params(params: &mut Params, name: &str, obj: &AutoScalingGroup) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		if let Some(ref obj) = obj.status {
			XmlStringMaxLen255Writer::write_params(params, &(prefix.to_string() + "Status"), obj);
		}
		if let Some(ref obj) = obj.auto_scaling_group_arn {
			ResourceNameWriter::write_params(params, &(prefix.to_string() + "AutoScalingGroupARN"), obj);
		}
		if let Some(ref obj) = obj.health_check_grace_period {
			HealthCheckGracePeriodWriter::write_params(params, &(prefix.to_string() + "HealthCheckGracePeriod"), obj);
		}
		if let Some(ref obj) = obj.suspended_processes {
			SuspendedProcessesWriter::write_params(params, &(prefix.to_string() + "SuspendedProcess"), obj);
		}
		AutoScalingGroupDesiredCapacityWriter::write_params(params, &(prefix.to_string() + "DesiredCapacity"), &obj.desired_capacity);
		if let Some(ref obj) = obj.tags {
			TagDescriptionListWriter::write_params(params, &(prefix.to_string() + "TagDescription"), obj);
		}
		if let Some(ref obj) = obj.enabled_metrics {
			EnabledMetricsWriter::write_params(params, &(prefix.to_string() + "EnabledMetric"), obj);
		}
		if let Some(ref obj) = obj.load_balancer_names {
			LoadBalancerNamesWriter::write_params(params, &(prefix.to_string() + "XmlStringMaxLen255"), obj);
		}
		XmlStringMaxLen255Writer::write_params(params, &(prefix.to_string() + "AutoScalingGroupName"), &obj.auto_scaling_group_name);
		CooldownWriter::write_params(params, &(prefix.to_string() + "DefaultCooldown"), &obj.default_cooldown);
		AutoScalingGroupMinSizeWriter::write_params(params, &(prefix.to_string() + "MinSize"), &obj.min_size);
		if let Some(ref obj) = obj.instances {
			InstancesWriter::write_params(params, &(prefix.to_string() + "Instance"), obj);
		}
		AutoScalingGroupMaxSizeWriter::write_params(params, &(prefix.to_string() + "MaxSize"), &obj.max_size);
		if let Some(ref obj) = obj.vpc_zone_identifier {
			XmlStringMaxLen255Writer::write_params(params, &(prefix.to_string() + "VPCZoneIdentifier"), obj);
		}
		if let Some(ref obj) = obj.termination_policies {
			TerminationPoliciesWriter::write_params(params, &(prefix.to_string() + "XmlStringMaxLen1600"), obj);
		}
		XmlStringMaxLen255Writer::write_params(params, &(prefix.to_string() + "LaunchConfigurationName"), &obj.launch_configuration_name);
		TimestampTypeWriter::write_params(params, &(prefix.to_string() + "CreatedTime"), &obj.created_time);
		if let Some(ref obj) = obj.placement_group {
			XmlStringMaxLen255Writer::write_params(params, &(prefix.to_string() + "PlacementGroup"), obj);
		}
		AvailabilityZonesWriter::write_params(params, &(prefix.to_string() + "XmlStringMaxLen255"), &obj.availability_zones);
		XmlStringMaxLen32Writer::write_params(params, &(prefix.to_string() + "HealthCheckType"), &obj.health_check_type);
	}
}
pub type LoadBalancerNames = Vec<XmlStringMaxLen255>;
/// Parse LoadBalancerNames from XML
struct LoadBalancerNamesParser;
impl LoadBalancerNamesParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<LoadBalancerNames, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "XmlStringMaxLen255" {
			obj.push(try!(XmlStringMaxLen255Parser::parse_xml("XmlStringMaxLen255", stack)));
		}
		Ok(obj)
	}
}
/// Write LoadBalancerNames contents to a SignedRequest
struct LoadBalancerNamesWriter;
impl LoadBalancerNamesWriter {
	fn write_params(params: &mut Params, name: &str, obj: &LoadBalancerNames) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			XmlStringMaxLen255Writer::write_params(params, key, &element);
			index += 1;
		}
	}
}
#[derive(Debug, Default)]
pub struct DetachInstancesAnswer {
	/// The activities related to detaching the instances from the Auto Scaling group.
	pub activities: Activities,
}

/// Parse DetachInstancesAnswer from XML
struct DetachInstancesAnswerParser;
impl DetachInstancesAnswerParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<DetachInstancesAnswer, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = DetachInstancesAnswer::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "Activity" {
				obj.activities = try!(ActivitiesParser::parse_xml("Activity", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write DetachInstancesAnswer contents to a SignedRequest
struct DetachInstancesAnswerWriter;
impl DetachInstancesAnswerWriter {
	fn write_params(params: &mut Params, name: &str, obj: &DetachInstancesAnswer) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		ActivitiesWriter::write_params(params, &(prefix.to_string() + "Activity"), &obj.activities);
	}
}
pub type GlobalTimeout = i32;
/// Parse GlobalTimeout from XML
struct GlobalTimeoutParser;
impl GlobalTimeoutParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<GlobalTimeout, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = i32::from_str(try!(characters(stack)).as_ref()).unwrap();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write GlobalTimeout contents to a SignedRequest
struct GlobalTimeoutWriter;
impl GlobalTimeoutWriter {
	fn write_params(params: &mut Params, name: &str, obj: &GlobalTimeout) {
		params.put(name, &obj.to_string());
	}
}
pub type XmlString = String;
/// Parse XmlString from XML
struct XmlStringParser;
impl XmlStringParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<XmlString, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write XmlString contents to a SignedRequest
struct XmlStringWriter;
impl XmlStringWriter {
	fn write_params(params: &mut Params, name: &str, obj: &XmlString) {
		params.put(name, obj);
	}
}
pub type ProcessNames = Vec<XmlStringMaxLen255>;
/// Parse ProcessNames from XML
struct ProcessNamesParser;
impl ProcessNamesParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ProcessNames, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "XmlStringMaxLen255" {
			obj.push(try!(XmlStringMaxLen255Parser::parse_xml("XmlStringMaxLen255", stack)));
		}
		Ok(obj)
	}
}
/// Write ProcessNames contents to a SignedRequest
struct ProcessNamesWriter;
impl ProcessNamesWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ProcessNames) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			XmlStringMaxLen255Writer::write_params(params, key, &element);
			index += 1;
		}
	}
}
/// Describes a scaling policy.
#[derive(Debug, Default)]
pub struct ScalingPolicy {
	/// The name of the scaling policy.
	pub policy_name: XmlStringMaxLen255,
	/// The estimated time, in seconds, until a newly launched instance can contribute
	/// to the CloudWatch metrics.
	pub estimated_instance_warmup: EstimatedInstanceWarmup,
	/// Available for backward compatibility. Use `MinAdjustmentMagnitude` instead.
	pub min_adjustment_step: MinAdjustmentStep,
	/// The minimum number of instances to scale. If the value of `AdjustmentType` is
	/// `PercentChangeInCapacity`, the scaling policy changes the `DesiredCapacity` of
	/// the Auto Scaling group by at least this many instances. Otherwise, the error
	/// is `ValidationError`.
	pub min_adjustment_magnitude: MinAdjustmentMagnitude,
	/// The aggregation type for the CloudWatch metrics. Valid values are `Minimum`,
	/// `Maximum`, and `Average`.
	pub metric_aggregation_type: XmlStringMaxLen32,
	/// The name of the Auto Scaling group associated with this scaling policy.
	pub auto_scaling_group_name: XmlStringMaxLen255,
	/// The Amazon Resource Name (ARN) of the policy.
	pub policy_arn: ResourceName,
	/// The amount of time, in seconds, after a scaling activity completes before any
	/// further trigger-related scaling activities can start.
	pub cooldown: Cooldown,
	/// The policy type. Valid values are `SimpleScaling` and `StepScaling`.
	pub policy_type: XmlStringMaxLen64,
	/// A set of adjustments that enable you to scale based on the size of the alarm
	/// breach.
	pub step_adjustments: StepAdjustments,
	/// The adjustment type, which specifies how `ScalingAdjustment` is interpreted.
	/// Valid values are `ChangeInCapacity`, `ExactCapacity`, and
	/// `PercentChangeInCapacity`.
	pub adjustment_type: XmlStringMaxLen255,
	/// The CloudWatch alarms related to the policy.
	pub alarms: Alarms,
	/// The amount by which to scale, based on the specified adjustment type. A
	/// positive value adds to the current capacity while a negative number removes
	/// from the current capacity.
	pub scaling_adjustment: PolicyIncrement,
}

/// Parse ScalingPolicy from XML
struct ScalingPolicyParser;
impl ScalingPolicyParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ScalingPolicy, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = ScalingPolicy::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "PolicyName" {
				obj.policy_name = try!(XmlStringMaxLen255Parser::parse_xml("PolicyName", stack));
				continue;
			}
			if current_name == "EstimatedInstanceWarmup" {
				obj.estimated_instance_warmup = try!(EstimatedInstanceWarmupParser::parse_xml("EstimatedInstanceWarmup", stack));
				continue;
			}
			if current_name == "MinAdjustmentStep" {
				obj.min_adjustment_step = try!(MinAdjustmentStepParser::parse_xml("MinAdjustmentStep", stack));
				continue;
			}
			if current_name == "MinAdjustmentMagnitude" {
				obj.min_adjustment_magnitude = try!(MinAdjustmentMagnitudeParser::parse_xml("MinAdjustmentMagnitude", stack));
				continue;
			}
			if current_name == "MetricAggregationType" {
				obj.metric_aggregation_type = try!(XmlStringMaxLen32Parser::parse_xml("MetricAggregationType", stack));
				continue;
			}
			if current_name == "AutoScalingGroupName" {
				obj.auto_scaling_group_name = try!(XmlStringMaxLen255Parser::parse_xml("AutoScalingGroupName", stack));
				continue;
			}
			if current_name == "PolicyARN" {
				obj.policy_arn = try!(ResourceNameParser::parse_xml("PolicyARN", stack));
				continue;
			}
			if current_name == "Cooldown" {
				obj.cooldown = try!(CooldownParser::parse_xml("Cooldown", stack));
				continue;
			}
			if current_name == "PolicyType" {
				obj.policy_type = try!(XmlStringMaxLen64Parser::parse_xml("PolicyType", stack));
				continue;
			}
			if current_name == "StepAdjustment" {
				obj.step_adjustments = try!(StepAdjustmentsParser::parse_xml("StepAdjustment", stack));
				continue;
			}
			if current_name == "AdjustmentType" {
				obj.adjustment_type = try!(XmlStringMaxLen255Parser::parse_xml("AdjustmentType", stack));
				continue;
			}
			if current_name == "Alarm" {
				obj.alarms = try!(AlarmsParser::parse_xml("Alarm", stack));
				continue;
			}
			if current_name == "ScalingAdjustment" {
				obj.scaling_adjustment = try!(PolicyIncrementParser::parse_xml("ScalingAdjustment", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ScalingPolicy contents to a SignedRequest
struct ScalingPolicyWriter;
impl ScalingPolicyWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ScalingPolicy) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		XmlStringMaxLen255Writer::write_params(params, &(prefix.to_string() + "PolicyName"), &obj.policy_name);
		EstimatedInstanceWarmupWriter::write_params(params, &(prefix.to_string() + "EstimatedInstanceWarmup"), &obj.estimated_instance_warmup);
		MinAdjustmentStepWriter::write_params(params, &(prefix.to_string() + "MinAdjustmentStep"), &obj.min_adjustment_step);
		MinAdjustmentMagnitudeWriter::write_params(params, &(prefix.to_string() + "MinAdjustmentMagnitude"), &obj.min_adjustment_magnitude);
		XmlStringMaxLen32Writer::write_params(params, &(prefix.to_string() + "MetricAggregationType"), &obj.metric_aggregation_type);
		XmlStringMaxLen255Writer::write_params(params, &(prefix.to_string() + "AutoScalingGroupName"), &obj.auto_scaling_group_name);
		ResourceNameWriter::write_params(params, &(prefix.to_string() + "PolicyARN"), &obj.policy_arn);
		CooldownWriter::write_params(params, &(prefix.to_string() + "Cooldown"), &obj.cooldown);
		XmlStringMaxLen64Writer::write_params(params, &(prefix.to_string() + "PolicyType"), &obj.policy_type);
		StepAdjustmentsWriter::write_params(params, &(prefix.to_string() + "StepAdjustment"), &obj.step_adjustments);
		XmlStringMaxLen255Writer::write_params(params, &(prefix.to_string() + "AdjustmentType"), &obj.adjustment_type);
		AlarmsWriter::write_params(params, &(prefix.to_string() + "Alarm"), &obj.alarms);
		PolicyIncrementWriter::write_params(params, &(prefix.to_string() + "ScalingAdjustment"), &obj.scaling_adjustment);
	}
}
pub type SuspendedProcesses = Vec<SuspendedProcess>;
/// Parse SuspendedProcesses from XML
struct SuspendedProcessesParser;
impl SuspendedProcessesParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<SuspendedProcesses, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "SuspendedProcess" {
			obj.push(try!(SuspendedProcessParser::parse_xml("SuspendedProcess", stack)));
		}
		Ok(obj)
	}
}
/// Write SuspendedProcesses contents to a SignedRequest
struct SuspendedProcessesWriter;
impl SuspendedProcessesWriter {
	fn write_params(params: &mut Params, name: &str, obj: &SuspendedProcesses) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			SuspendedProcessWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
pub type HonorCooldown = bool;
/// Parse HonorCooldown from XML
struct HonorCooldownParser;
impl HonorCooldownParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<HonorCooldown, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = bool::from_str(try!(characters(stack)).as_ref()).unwrap();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write HonorCooldown contents to a SignedRequest
struct HonorCooldownWriter;
impl HonorCooldownWriter {
	fn write_params(params: &mut Params, name: &str, obj: &HonorCooldown) {
		params.put(name, &obj.to_string());
	}
}
#[derive(Debug, Default)]
pub struct RecordLifecycleActionHeartbeatAnswer;

/// Parse RecordLifecycleActionHeartbeatAnswer from XML
struct RecordLifecycleActionHeartbeatAnswerParser;
impl RecordLifecycleActionHeartbeatAnswerParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<RecordLifecycleActionHeartbeatAnswer, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = RecordLifecycleActionHeartbeatAnswer::default();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write RecordLifecycleActionHeartbeatAnswer contents to a SignedRequest
struct RecordLifecycleActionHeartbeatAnswerWriter;
impl RecordLifecycleActionHeartbeatAnswerWriter {
	fn write_params(params: &mut Params, name: &str, obj: &RecordLifecycleActionHeartbeatAnswer) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
	}
}
#[derive(Debug, Default)]
pub struct DeleteScheduledActionType {
	/// The name of the Auto Scaling group.
	pub auto_scaling_group_name: Option<ResourceName>,
	/// The name of the action to delete.
	pub scheduled_action_name: ResourceName,
}

/// Parse DeleteScheduledActionType from XML
struct DeleteScheduledActionTypeParser;
impl DeleteScheduledActionTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<DeleteScheduledActionType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = DeleteScheduledActionType::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "AutoScalingGroupName" {
				obj.auto_scaling_group_name = Some(try!(ResourceNameParser::parse_xml("AutoScalingGroupName", stack)));
				continue;
			}
			if current_name == "ScheduledActionName" {
				obj.scheduled_action_name = try!(ResourceNameParser::parse_xml("ScheduledActionName", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write DeleteScheduledActionType contents to a SignedRequest
struct DeleteScheduledActionTypeWriter;
impl DeleteScheduledActionTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &DeleteScheduledActionType) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		if let Some(ref obj) = obj.auto_scaling_group_name {
			ResourceNameWriter::write_params(params, &(prefix.to_string() + "AutoScalingGroupName"), obj);
		}
		ResourceNameWriter::write_params(params, &(prefix.to_string() + "ScheduledActionName"), &obj.scheduled_action_name);
	}
}
pub type LifecycleActionResult = String;
/// Parse LifecycleActionResult from XML
struct LifecycleActionResultParser;
impl LifecycleActionResultParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<LifecycleActionResult, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write LifecycleActionResult contents to a SignedRequest
struct LifecycleActionResultWriter;
impl LifecycleActionResultWriter {
	fn write_params(params: &mut Params, name: &str, obj: &LifecycleActionResult) {
		params.put(name, obj);
	}
}
pub type AsciiStringMaxLen255 = String;
/// Parse AsciiStringMaxLen255 from XML
struct AsciiStringMaxLen255Parser;
impl AsciiStringMaxLen255Parser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<AsciiStringMaxLen255, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write AsciiStringMaxLen255 contents to a SignedRequest
struct AsciiStringMaxLen255Writer;
impl AsciiStringMaxLen255Writer {
	fn write_params(params: &mut Params, name: &str, obj: &AsciiStringMaxLen255) {
		params.put(name, obj);
	}
}
#[derive(Debug, Default)]
pub struct DescribeNotificationConfigurationsType {
	/// The maximum number of items to return with this call.
	pub max_records: MaxRecords,
	/// The token for the next set of items to return. (You received this token from a
	/// previous call.)
	pub next_token: XmlString,
	/// The name of the group.
	pub auto_scaling_group_names: AutoScalingGroupNames,
}

/// Parse DescribeNotificationConfigurationsType from XML
struct DescribeNotificationConfigurationsTypeParser;
impl DescribeNotificationConfigurationsTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<DescribeNotificationConfigurationsType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = DescribeNotificationConfigurationsType::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "MaxRecords" {
				obj.max_records = try!(MaxRecordsParser::parse_xml("MaxRecords", stack));
				continue;
			}
			if current_name == "NextToken" {
				obj.next_token = try!(XmlStringParser::parse_xml("NextToken", stack));
				continue;
			}
			if current_name == "ResourceName" {
				obj.auto_scaling_group_names = try!(AutoScalingGroupNamesParser::parse_xml("ResourceName", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write DescribeNotificationConfigurationsType contents to a SignedRequest
struct DescribeNotificationConfigurationsTypeWriter;
impl DescribeNotificationConfigurationsTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &DescribeNotificationConfigurationsType) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		MaxRecordsWriter::write_params(params, &(prefix.to_string() + "MaxRecords"), &obj.max_records);
		XmlStringWriter::write_params(params, &(prefix.to_string() + "NextToken"), &obj.next_token);
		AutoScalingGroupNamesWriter::write_params(params, &(prefix.to_string() + "ResourceName"), &obj.auto_scaling_group_names);
	}
}
/// Describes a granularity of a metric.
#[derive(Debug, Default)]
pub struct MetricGranularityType {
	/// The granularity. The only valid value is `1Minute`.
	pub granularity: XmlStringMaxLen255,
}

/// Parse MetricGranularityType from XML
struct MetricGranularityTypeParser;
impl MetricGranularityTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<MetricGranularityType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = MetricGranularityType::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "Granularity" {
				obj.granularity = try!(XmlStringMaxLen255Parser::parse_xml("Granularity", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write MetricGranularityType contents to a SignedRequest
struct MetricGranularityTypeWriter;
impl MetricGranularityTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &MetricGranularityType) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		XmlStringMaxLen255Writer::write_params(params, &(prefix.to_string() + "Granularity"), &obj.granularity);
	}
}
#[derive(Debug, Default)]
pub struct DescribeScheduledActionsType {
	/// The latest scheduled start time to return. If scheduled action names are
	/// provided, this parameter is ignored.
	pub end_time: TimestampType,
	/// The name of the group.
	pub auto_scaling_group_name: ResourceName,
	/// The maximum number of items to return with this call.
	pub max_records: MaxRecords,
	/// Describes one or more scheduled actions. If you omit this list, the call
	/// describes all scheduled actions. If you specify an unknown scheduled action it
	/// is ignored with no error.
	/// You can describe up to a maximum of 50 instances with a single call. If there
	/// are more items to return, the call returns a token. To get the next set of
	/// items, repeat the call with the returned token in the `NextToken` parameter.
	pub scheduled_action_names: ScheduledActionNames,
	/// The earliest scheduled start time to return. If scheduled action names are
	/// provided, this parameter is ignored.
	pub start_time: TimestampType,
	/// The token for the next set of items to return. (You received this token from a
	/// previous call.)
	pub next_token: XmlString,
}

/// Parse DescribeScheduledActionsType from XML
struct DescribeScheduledActionsTypeParser;
impl DescribeScheduledActionsTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<DescribeScheduledActionsType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = DescribeScheduledActionsType::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "EndTime" {
				obj.end_time = try!(TimestampTypeParser::parse_xml("EndTime", stack));
				continue;
			}
			if current_name == "AutoScalingGroupName" {
				obj.auto_scaling_group_name = try!(ResourceNameParser::parse_xml("AutoScalingGroupName", stack));
				continue;
			}
			if current_name == "MaxRecords" {
				obj.max_records = try!(MaxRecordsParser::parse_xml("MaxRecords", stack));
				continue;
			}
			if current_name == "ResourceName" {
				obj.scheduled_action_names = try!(ScheduledActionNamesParser::parse_xml("ResourceName", stack));
				continue;
			}
			if current_name == "StartTime" {
				obj.start_time = try!(TimestampTypeParser::parse_xml("StartTime", stack));
				continue;
			}
			if current_name == "NextToken" {
				obj.next_token = try!(XmlStringParser::parse_xml("NextToken", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write DescribeScheduledActionsType contents to a SignedRequest
struct DescribeScheduledActionsTypeWriter;
impl DescribeScheduledActionsTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &DescribeScheduledActionsType) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		TimestampTypeWriter::write_params(params, &(prefix.to_string() + "EndTime"), &obj.end_time);
		ResourceNameWriter::write_params(params, &(prefix.to_string() + "AutoScalingGroupName"), &obj.auto_scaling_group_name);
		MaxRecordsWriter::write_params(params, &(prefix.to_string() + "MaxRecords"), &obj.max_records);
		ScheduledActionNamesWriter::write_params(params, &(prefix.to_string() + "ResourceName"), &obj.scheduled_action_names);
		TimestampTypeWriter::write_params(params, &(prefix.to_string() + "StartTime"), &obj.start_time);
		XmlStringWriter::write_params(params, &(prefix.to_string() + "NextToken"), &obj.next_token);
	}
}
/// You already have a pending update to an Auto Scaling resource (for example, a
/// group, instance, or load balancer).
#[derive(Debug, Default)]
pub struct ResourceContentionFault {
	pub message: XmlStringMaxLen255,
}

/// Parse ResourceContentionFault from XML
struct ResourceContentionFaultParser;
impl ResourceContentionFaultParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ResourceContentionFault, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = ResourceContentionFault::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "message" {
				obj.message = try!(XmlStringMaxLen255Parser::parse_xml("message", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ResourceContentionFault contents to a SignedRequest
struct ResourceContentionFaultWriter;
impl ResourceContentionFaultWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ResourceContentionFault) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		XmlStringMaxLen255Writer::write_params(params, &(prefix.to_string() + "message"), &obj.message);
	}
}
/// Describes a scheduled update to an Auto Scaling group.
#[derive(Debug, Default)]
pub struct ScheduledUpdateGroupAction {
	/// The minimum size of the group.
	pub min_size: AutoScalingGroupMinSize,
	/// The number of instances you prefer to maintain in the group.
	pub desired_capacity: AutoScalingGroupDesiredCapacity,
	/// The name of the group.
	pub auto_scaling_group_name: XmlStringMaxLen255,
	/// The maximum size of the group.
	pub max_size: AutoScalingGroupMaxSize,
	/// The recurring schedule for the action.
	pub recurrence: XmlStringMaxLen255,
	/// The Amazon Resource Name (ARN) of the scheduled action.
	pub scheduled_action_arn: ResourceName,
	/// The name of the scheduled action.
	pub scheduled_action_name: XmlStringMaxLen255,
	/// The date and time that the action is scheduled to begin. This date and time
	/// can be up to one month in the future.
	/// When `StartTime` and `EndTime` are specified with `Recurrence`, they form the
	/// boundaries of when the recurring action will start and stop.
	pub start_time: TimestampType,
	/// This parameter is deprecated; use `StartTime` instead.
	pub time: TimestampType,
	/// The date and time that the action is scheduled to end. This date and time can
	/// be up to one month in the future.
	pub end_time: TimestampType,
}

/// Parse ScheduledUpdateGroupAction from XML
struct ScheduledUpdateGroupActionParser;
impl ScheduledUpdateGroupActionParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ScheduledUpdateGroupAction, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = ScheduledUpdateGroupAction::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "MinSize" {
				obj.min_size = try!(AutoScalingGroupMinSizeParser::parse_xml("MinSize", stack));
				continue;
			}
			if current_name == "DesiredCapacity" {
				obj.desired_capacity = try!(AutoScalingGroupDesiredCapacityParser::parse_xml("DesiredCapacity", stack));
				continue;
			}
			if current_name == "AutoScalingGroupName" {
				obj.auto_scaling_group_name = try!(XmlStringMaxLen255Parser::parse_xml("AutoScalingGroupName", stack));
				continue;
			}
			if current_name == "MaxSize" {
				obj.max_size = try!(AutoScalingGroupMaxSizeParser::parse_xml("MaxSize", stack));
				continue;
			}
			if current_name == "Recurrence" {
				obj.recurrence = try!(XmlStringMaxLen255Parser::parse_xml("Recurrence", stack));
				continue;
			}
			if current_name == "ScheduledActionARN" {
				obj.scheduled_action_arn = try!(ResourceNameParser::parse_xml("ScheduledActionARN", stack));
				continue;
			}
			if current_name == "ScheduledActionName" {
				obj.scheduled_action_name = try!(XmlStringMaxLen255Parser::parse_xml("ScheduledActionName", stack));
				continue;
			}
			if current_name == "StartTime" {
				obj.start_time = try!(TimestampTypeParser::parse_xml("StartTime", stack));
				continue;
			}
			if current_name == "Time" {
				obj.time = try!(TimestampTypeParser::parse_xml("Time", stack));
				continue;
			}
			if current_name == "EndTime" {
				obj.end_time = try!(TimestampTypeParser::parse_xml("EndTime", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ScheduledUpdateGroupAction contents to a SignedRequest
struct ScheduledUpdateGroupActionWriter;
impl ScheduledUpdateGroupActionWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ScheduledUpdateGroupAction) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		AutoScalingGroupMinSizeWriter::write_params(params, &(prefix.to_string() + "MinSize"), &obj.min_size);
		AutoScalingGroupDesiredCapacityWriter::write_params(params, &(prefix.to_string() + "DesiredCapacity"), &obj.desired_capacity);
		XmlStringMaxLen255Writer::write_params(params, &(prefix.to_string() + "AutoScalingGroupName"), &obj.auto_scaling_group_name);
		AutoScalingGroupMaxSizeWriter::write_params(params, &(prefix.to_string() + "MaxSize"), &obj.max_size);
		XmlStringMaxLen255Writer::write_params(params, &(prefix.to_string() + "Recurrence"), &obj.recurrence);
		ResourceNameWriter::write_params(params, &(prefix.to_string() + "ScheduledActionARN"), &obj.scheduled_action_arn);
		XmlStringMaxLen255Writer::write_params(params, &(prefix.to_string() + "ScheduledActionName"), &obj.scheduled_action_name);
		TimestampTypeWriter::write_params(params, &(prefix.to_string() + "StartTime"), &obj.start_time);
		TimestampTypeWriter::write_params(params, &(prefix.to_string() + "Time"), &obj.time);
		TimestampTypeWriter::write_params(params, &(prefix.to_string() + "EndTime"), &obj.end_time);
	}
}
/// Describes an enabled metric.
#[derive(Debug, Default)]
pub struct EnabledMetric {
	/// The name of the metric.
	///   * `GroupMinSize`
	///   * `GroupMaxSize`
	///   * `GroupDesiredCapacity`
	///   * `GroupInServiceInstances`
	///   * `GroupPendingInstances`
	///   * `GroupStandbyInstances`
	///   * `GroupTerminatingInstances`
	///   * `GroupTotalInstances`
	pub metric: XmlStringMaxLen255,
	/// The granularity of the metric. The only valid value is `1Minute`.
	pub granularity: XmlStringMaxLen255,
}

/// Parse EnabledMetric from XML
struct EnabledMetricParser;
impl EnabledMetricParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<EnabledMetric, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = EnabledMetric::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "Metric" {
				obj.metric = try!(XmlStringMaxLen255Parser::parse_xml("Metric", stack));
				continue;
			}
			if current_name == "Granularity" {
				obj.granularity = try!(XmlStringMaxLen255Parser::parse_xml("Granularity", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write EnabledMetric contents to a SignedRequest
struct EnabledMetricWriter;
impl EnabledMetricWriter {
	fn write_params(params: &mut Params, name: &str, obj: &EnabledMetric) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		XmlStringMaxLen255Writer::write_params(params, &(prefix.to_string() + "Metric"), &obj.metric);
		XmlStringMaxLen255Writer::write_params(params, &(prefix.to_string() + "Granularity"), &obj.granularity);
	}
}
/// 
#[derive(Debug, Default)]
pub struct DeletePolicyType {
	/// The name or Amazon Resource Name (ARN) of the policy.
	pub policy_name: ResourceName,
	/// The name of the Auto Scaling group.
	pub auto_scaling_group_name: Option<ResourceName>,
}

/// Parse DeletePolicyType from XML
struct DeletePolicyTypeParser;
impl DeletePolicyTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<DeletePolicyType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = DeletePolicyType::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "PolicyName" {
				obj.policy_name = try!(ResourceNameParser::parse_xml("PolicyName", stack));
				continue;
			}
			if current_name == "AutoScalingGroupName" {
				obj.auto_scaling_group_name = Some(try!(ResourceNameParser::parse_xml("AutoScalingGroupName", stack)));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write DeletePolicyType contents to a SignedRequest
struct DeletePolicyTypeWriter;
impl DeletePolicyTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &DeletePolicyType) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		ResourceNameWriter::write_params(params, &(prefix.to_string() + "PolicyName"), &obj.policy_name);
		if let Some(ref obj) = obj.auto_scaling_group_name {
			ResourceNameWriter::write_params(params, &(prefix.to_string() + "AutoScalingGroupName"), obj);
		}
	}
}
#[derive(Debug, Default)]
pub struct DeleteLifecycleHookType {
	/// The name of the lifecycle hook.
	pub lifecycle_hook_name: AsciiStringMaxLen255,
	/// The name of the Auto Scaling group for the lifecycle hook.
	pub auto_scaling_group_name: ResourceName,
}

/// Parse DeleteLifecycleHookType from XML
struct DeleteLifecycleHookTypeParser;
impl DeleteLifecycleHookTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<DeleteLifecycleHookType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = DeleteLifecycleHookType::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "LifecycleHookName" {
				obj.lifecycle_hook_name = try!(AsciiStringMaxLen255Parser::parse_xml("LifecycleHookName", stack));
				continue;
			}
			if current_name == "AutoScalingGroupName" {
				obj.auto_scaling_group_name = try!(ResourceNameParser::parse_xml("AutoScalingGroupName", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write DeleteLifecycleHookType contents to a SignedRequest
struct DeleteLifecycleHookTypeWriter;
impl DeleteLifecycleHookTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &DeleteLifecycleHookType) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		AsciiStringMaxLen255Writer::write_params(params, &(prefix.to_string() + "LifecycleHookName"), &obj.lifecycle_hook_name);
		ResourceNameWriter::write_params(params, &(prefix.to_string() + "AutoScalingGroupName"), &obj.auto_scaling_group_name);
	}
}
pub type TagValue = String;
/// Parse TagValue from XML
struct TagValueParser;
impl TagValueParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<TagValue, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write TagValue contents to a SignedRequest
struct TagValueWriter;
impl TagValueWriter {
	fn write_params(params: &mut Params, name: &str, obj: &TagValue) {
		params.put(name, obj);
	}
}
/// Describes a process type.
/// For more information, see [Auto Scaling Processes](http://docs.aws.amazon.com/
/// AutoScaling/latest/DeveloperGuide/US_SuspendResume.html#process-types) in the
/// _Auto Scaling Developer Guide_.
#[derive(Debug, Default)]
pub struct ProcessType {
	/// The name of the process.
	///   * `Launch`
	///   * `Terminate`
	///   * `AddToLoadBalancer`
	///   * `AlarmNotification`
	///   * `AZRebalance`
	///   * `HealthCheck`
	///   * `ReplaceUnhealthy`
	///   * `ScheduledActions`
	pub process_name: XmlStringMaxLen255,
}

/// Parse ProcessType from XML
struct ProcessTypeParser;
impl ProcessTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ProcessType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = ProcessType::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "ProcessName" {
				obj.process_name = try!(XmlStringMaxLen255Parser::parse_xml("ProcessName", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ProcessType contents to a SignedRequest
struct ProcessTypeWriter;
impl ProcessTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ProcessType) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		XmlStringMaxLen255Writer::write_params(params, &(prefix.to_string() + "ProcessName"), &obj.process_name);
	}
}
#[derive(Debug, Default)]
pub struct PutNotificationConfigurationType {
	/// The name of the Auto Scaling group.
	pub auto_scaling_group_name: ResourceName,
	/// The type of event that will cause the notification to be sent. For details
	/// about notification types supported by Auto Scaling, see
	/// DescribeAutoScalingNotificationTypes.
	pub notification_types: AutoScalingNotificationTypes,
	/// The Amazon Resource Name (ARN) of the Amazon Simple Notification Service (SNS)
	/// topic.
	pub topic_arn: ResourceName,
}

/// Parse PutNotificationConfigurationType from XML
struct PutNotificationConfigurationTypeParser;
impl PutNotificationConfigurationTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<PutNotificationConfigurationType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = PutNotificationConfigurationType::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "AutoScalingGroupName" {
				obj.auto_scaling_group_name = try!(ResourceNameParser::parse_xml("AutoScalingGroupName", stack));
				continue;
			}
			if current_name == "XmlStringMaxLen255" {
				obj.notification_types = try!(AutoScalingNotificationTypesParser::parse_xml("XmlStringMaxLen255", stack));
				continue;
			}
			if current_name == "TopicARN" {
				obj.topic_arn = try!(ResourceNameParser::parse_xml("TopicARN", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write PutNotificationConfigurationType contents to a SignedRequest
struct PutNotificationConfigurationTypeWriter;
impl PutNotificationConfigurationTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &PutNotificationConfigurationType) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		ResourceNameWriter::write_params(params, &(prefix.to_string() + "AutoScalingGroupName"), &obj.auto_scaling_group_name);
		AutoScalingNotificationTypesWriter::write_params(params, &(prefix.to_string() + "XmlStringMaxLen255"), &obj.notification_types);
		ResourceNameWriter::write_params(params, &(prefix.to_string() + "TopicARN"), &obj.topic_arn);
	}
}
pub type StepAdjustments = Vec<StepAdjustment>;
/// Parse StepAdjustments from XML
struct StepAdjustmentsParser;
impl StepAdjustmentsParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<StepAdjustments, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "StepAdjustment" {
			obj.push(try!(StepAdjustmentParser::parse_xml("StepAdjustment", stack)));
		}
		Ok(obj)
	}
}
/// Write StepAdjustments contents to a SignedRequest
struct StepAdjustmentsWriter;
impl StepAdjustmentsWriter {
	fn write_params(params: &mut Params, name: &str, obj: &StepAdjustments) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			StepAdjustmentWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
pub type EstimatedInstanceWarmup = i32;
/// Parse EstimatedInstanceWarmup from XML
struct EstimatedInstanceWarmupParser;
impl EstimatedInstanceWarmupParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<EstimatedInstanceWarmup, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = i32::from_str(try!(characters(stack)).as_ref()).unwrap();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write EstimatedInstanceWarmup contents to a SignedRequest
struct EstimatedInstanceWarmupWriter;
impl EstimatedInstanceWarmupWriter {
	fn write_params(params: &mut Params, name: &str, obj: &EstimatedInstanceWarmup) {
		params.put(name, &obj.to_string());
	}
}
#[derive(Debug, Default)]
pub struct EnterStandbyAnswer {
	/// The activities related to moving instances into `Standby` mode.
	pub activities: Activities,
}

/// Parse EnterStandbyAnswer from XML
struct EnterStandbyAnswerParser;
impl EnterStandbyAnswerParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<EnterStandbyAnswer, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = EnterStandbyAnswer::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "Activity" {
				obj.activities = try!(ActivitiesParser::parse_xml("Activity", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write EnterStandbyAnswer contents to a SignedRequest
struct EnterStandbyAnswerWriter;
impl EnterStandbyAnswerWriter {
	fn write_params(params: &mut Params, name: &str, obj: &EnterStandbyAnswer) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		ActivitiesWriter::write_params(params, &(prefix.to_string() + "Activity"), &obj.activities);
	}
}
pub type MetricScale = f32;
/// Parse MetricScale from XML
struct MetricScaleParser;
impl MetricScaleParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<MetricScale, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = f32::from_str(try!(characters(stack)).as_ref()).unwrap();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write MetricScale contents to a SignedRequest
struct MetricScaleWriter;
impl MetricScaleWriter {
	fn write_params(params: &mut Params, name: &str, obj: &MetricScale) {
		params.put(name, &obj.to_string());
	}
}
pub type LaunchConfigurations = Vec<LaunchConfiguration>;
/// Parse LaunchConfigurations from XML
struct LaunchConfigurationsParser;
impl LaunchConfigurationsParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<LaunchConfigurations, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "LaunchConfiguration" {
			obj.push(try!(LaunchConfigurationParser::parse_xml("LaunchConfiguration", stack)));
		}
		Ok(obj)
	}
}
/// Write LaunchConfigurations contents to a SignedRequest
struct LaunchConfigurationsWriter;
impl LaunchConfigurationsWriter {
	fn write_params(params: &mut Params, name: &str, obj: &LaunchConfigurations) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			LaunchConfigurationWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
#[derive(Debug, Default)]
pub struct ExitStandbyAnswer {
	/// The activities related to moving instances out of `Standby` mode.
	pub activities: Activities,
}

/// Parse ExitStandbyAnswer from XML
struct ExitStandbyAnswerParser;
impl ExitStandbyAnswerParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ExitStandbyAnswer, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = ExitStandbyAnswer::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "Activity" {
				obj.activities = try!(ActivitiesParser::parse_xml("Activity", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ExitStandbyAnswer contents to a SignedRequest
struct ExitStandbyAnswerWriter;
impl ExitStandbyAnswerWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ExitStandbyAnswer) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		ActivitiesWriter::write_params(params, &(prefix.to_string() + "Activity"), &obj.activities);
	}
}
pub type MaxNumberOfAutoScalingGroups = i32;
/// Parse MaxNumberOfAutoScalingGroups from XML
struct MaxNumberOfAutoScalingGroupsParser;
impl MaxNumberOfAutoScalingGroupsParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<MaxNumberOfAutoScalingGroups, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = i32::from_str(try!(characters(stack)).as_ref()).unwrap();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write MaxNumberOfAutoScalingGroups contents to a SignedRequest
struct MaxNumberOfAutoScalingGroupsWriter;
impl MaxNumberOfAutoScalingGroupsWriter {
	fn write_params(params: &mut Params, name: &str, obj: &MaxNumberOfAutoScalingGroups) {
		params.put(name, &obj.to_string());
	}
}
#[derive(Debug, Default)]
pub struct DescribeAdjustmentTypesAnswer {
	/// The policy adjustment types.
	pub adjustment_types: AdjustmentTypes,
}

/// Parse DescribeAdjustmentTypesAnswer from XML
struct DescribeAdjustmentTypesAnswerParser;
impl DescribeAdjustmentTypesAnswerParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<DescribeAdjustmentTypesAnswer, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = DescribeAdjustmentTypesAnswer::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "AdjustmentType" {
				obj.adjustment_types = try!(AdjustmentTypesParser::parse_xml("AdjustmentType", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write DescribeAdjustmentTypesAnswer contents to a SignedRequest
struct DescribeAdjustmentTypesAnswerWriter;
impl DescribeAdjustmentTypesAnswerWriter {
	fn write_params(params: &mut Params, name: &str, obj: &DescribeAdjustmentTypesAnswer) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		AdjustmentTypesWriter::write_params(params, &(prefix.to_string() + "AdjustmentType"), &obj.adjustment_types);
	}
}
pub type TimestampType = String;
/// Parse TimestampType from XML
struct TimestampTypeParser;
impl TimestampTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<TimestampType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write TimestampType contents to a SignedRequest
struct TimestampTypeWriter;
impl TimestampTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &TimestampType) {
		params.put(name, obj);
	}
}
/// Describes a policy adjustment type.
/// For more information, see [Dynamic
/// Scaling](http://docs.aws.amazon.com/AutoScaling/latest/DeveloperGuide/as-
/// scale-based-on-demand.html) in the _Auto Scaling Developer Guide_.
#[derive(Debug, Default)]
pub struct AdjustmentType {
	/// The policy adjustment type. The valid values are `ChangeInCapacity`,
	/// `ExactCapacity`, and `PercentChangeInCapacity`.
	pub adjustment_type: XmlStringMaxLen255,
}

/// Parse AdjustmentType from XML
struct AdjustmentTypeParser;
impl AdjustmentTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<AdjustmentType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = AdjustmentType::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "AdjustmentType" {
				obj.adjustment_type = try!(XmlStringMaxLen255Parser::parse_xml("AdjustmentType", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write AdjustmentType contents to a SignedRequest
struct AdjustmentTypeWriter;
impl AdjustmentTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &AdjustmentType) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		XmlStringMaxLen255Writer::write_params(params, &(prefix.to_string() + "AdjustmentType"), &obj.adjustment_type);
	}
}
pub type Values = Vec<XmlString>;
/// Parse Values from XML
struct ValuesParser;
impl ValuesParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<Values, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "XmlString" {
			obj.push(try!(XmlStringParser::parse_xml("XmlString", stack)));
		}
		Ok(obj)
	}
}
/// Write Values contents to a SignedRequest
struct ValuesWriter;
impl ValuesWriter {
	fn write_params(params: &mut Params, name: &str, obj: &Values) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			XmlStringWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
#[derive(Debug, Default)]
pub struct DisableMetricsCollectionQuery {
	/// One or more metrics. If you omit this parameter, all metrics are disabled.
	///   * `GroupMinSize`
	///   * `GroupMaxSize`
	///   * `GroupDesiredCapacity`
	///   * `GroupInServiceInstances`
	///   * `GroupPendingInstances`
	///   * `GroupStandbyInstances`
	///   * `GroupTerminatingInstances`
	///   * `GroupTotalInstances`
	pub metrics: Option<Metrics>,
	/// The name or Amazon Resource Name (ARN) of the group.
	pub auto_scaling_group_name: ResourceName,
}

/// Parse DisableMetricsCollectionQuery from XML
struct DisableMetricsCollectionQueryParser;
impl DisableMetricsCollectionQueryParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<DisableMetricsCollectionQuery, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = DisableMetricsCollectionQuery::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "XmlStringMaxLen255" {
				obj.metrics = Some(try!(MetricsParser::parse_xml("XmlStringMaxLen255", stack)));
				continue;
			}
			if current_name == "AutoScalingGroupName" {
				obj.auto_scaling_group_name = try!(ResourceNameParser::parse_xml("AutoScalingGroupName", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write DisableMetricsCollectionQuery contents to a SignedRequest
struct DisableMetricsCollectionQueryWriter;
impl DisableMetricsCollectionQueryWriter {
	fn write_params(params: &mut Params, name: &str, obj: &DisableMetricsCollectionQuery) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		if let Some(ref obj) = obj.metrics {
			MetricsWriter::write_params(params, &(prefix.to_string() + "XmlStringMaxLen255"), obj);
		}
		ResourceNameWriter::write_params(params, &(prefix.to_string() + "AutoScalingGroupName"), &obj.auto_scaling_group_name);
	}
}
pub type LifecycleTransition = String;
/// Parse LifecycleTransition from XML
struct LifecycleTransitionParser;
impl LifecycleTransitionParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<LifecycleTransition, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write LifecycleTransition contents to a SignedRequest
struct LifecycleTransitionWriter;
impl LifecycleTransitionWriter {
	fn write_params(params: &mut Params, name: &str, obj: &LifecycleTransition) {
		params.put(name, obj);
	}
}
/// Describes a notification.
#[derive(Debug, Default)]
pub struct NotificationConfiguration {
	/// The name of the group.
	pub auto_scaling_group_name: ResourceName,
	/// The types of events for an action to start.
	///   * `autoscaling:EC2_INSTANCE_LAUNCH`
	///   * `autoscaling:EC2_INSTANCE_LAUNCH_ERROR`
	///   * `autoscaling:EC2_INSTANCE_TERMINATE`
	///   * `autoscaling:EC2_INSTANCE_TERMINATE_ERROR`
	///   * `autoscaling:TEST_NOTIFICATION`
	pub notification_type: XmlStringMaxLen255,
	/// The Amazon Resource Name (ARN) of the Amazon Simple Notification Service (SNS)
	/// topic.
	pub topic_arn: ResourceName,
}

/// Parse NotificationConfiguration from XML
struct NotificationConfigurationParser;
impl NotificationConfigurationParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<NotificationConfiguration, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = NotificationConfiguration::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "AutoScalingGroupName" {
				obj.auto_scaling_group_name = try!(ResourceNameParser::parse_xml("AutoScalingGroupName", stack));
				continue;
			}
			if current_name == "NotificationType" {
				obj.notification_type = try!(XmlStringMaxLen255Parser::parse_xml("NotificationType", stack));
				continue;
			}
			if current_name == "TopicARN" {
				obj.topic_arn = try!(ResourceNameParser::parse_xml("TopicARN", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write NotificationConfiguration contents to a SignedRequest
struct NotificationConfigurationWriter;
impl NotificationConfigurationWriter {
	fn write_params(params: &mut Params, name: &str, obj: &NotificationConfiguration) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		ResourceNameWriter::write_params(params, &(prefix.to_string() + "AutoScalingGroupName"), &obj.auto_scaling_group_name);
		XmlStringMaxLen255Writer::write_params(params, &(prefix.to_string() + "NotificationType"), &obj.notification_type);
		ResourceNameWriter::write_params(params, &(prefix.to_string() + "TopicARN"), &obj.topic_arn);
	}
}
pub type LaunchConfigurationNames = Vec<ResourceName>;
/// Parse LaunchConfigurationNames from XML
struct LaunchConfigurationNamesParser;
impl LaunchConfigurationNamesParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<LaunchConfigurationNames, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "ResourceName" {
			obj.push(try!(ResourceNameParser::parse_xml("ResourceName", stack)));
		}
		Ok(obj)
	}
}
/// Write LaunchConfigurationNames contents to a SignedRequest
struct LaunchConfigurationNamesWriter;
impl LaunchConfigurationNamesWriter {
	fn write_params(params: &mut Params, name: &str, obj: &LaunchConfigurationNames) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			ResourceNameWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
pub type MinAdjustmentStep = i32;
/// Parse MinAdjustmentStep from XML
struct MinAdjustmentStepParser;
impl MinAdjustmentStepParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<MinAdjustmentStep, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = i32::from_str(try!(characters(stack)).as_ref()).unwrap();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write MinAdjustmentStep contents to a SignedRequest
struct MinAdjustmentStepWriter;
impl MinAdjustmentStepWriter {
	fn write_params(params: &mut Params, name: &str, obj: &MinAdjustmentStep) {
		params.put(name, &obj.to_string());
	}
}
#[derive(Debug, Default)]
pub struct LaunchConfigurationNamesType {
	/// The maximum number of items to return with this call. The default is 100.
	pub max_records: MaxRecords,
	/// The launch configuration names.
	pub launch_configuration_names: LaunchConfigurationNames,
	/// The token for the next set of items to return. (You received this token from a
	/// previous call.)
	pub next_token: XmlString,
}

/// Parse LaunchConfigurationNamesType from XML
struct LaunchConfigurationNamesTypeParser;
impl LaunchConfigurationNamesTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<LaunchConfigurationNamesType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = LaunchConfigurationNamesType::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "MaxRecords" {
				obj.max_records = try!(MaxRecordsParser::parse_xml("MaxRecords", stack));
				continue;
			}
			if current_name == "ResourceName" {
				obj.launch_configuration_names = try!(LaunchConfigurationNamesParser::parse_xml("ResourceName", stack));
				continue;
			}
			if current_name == "NextToken" {
				obj.next_token = try!(XmlStringParser::parse_xml("NextToken", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write LaunchConfigurationNamesType contents to a SignedRequest
struct LaunchConfigurationNamesTypeWriter;
impl LaunchConfigurationNamesTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &LaunchConfigurationNamesType) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		MaxRecordsWriter::write_params(params, &(prefix.to_string() + "MaxRecords"), &obj.max_records);
		LaunchConfigurationNamesWriter::write_params(params, &(prefix.to_string() + "ResourceName"), &obj.launch_configuration_names);
		XmlStringWriter::write_params(params, &(prefix.to_string() + "NextToken"), &obj.next_token);
	}
}
#[derive(Debug, Default)]
pub struct PutScalingPolicyType {
	/// The name of the policy.
	pub policy_name: XmlStringMaxLen255,
	/// The estimated time, in seconds, until a newly launched instance can contribute
	/// to the CloudWatch metrics. The default is to use the value specified for the
	/// default cooldown period for the group.
	/// This parameter is not supported if the policy type is `SimpleScaling`.
	pub estimated_instance_warmup: Option<EstimatedInstanceWarmup>,
	/// Available for backward compatibility. Use `MinAdjustmentMagnitude` instead.
	pub min_adjustment_step: Option<MinAdjustmentStep>,
	/// The minimum number of instances to scale. If the value of `AdjustmentType` is
	/// `PercentChangeInCapacity`, the scaling policy changes the `DesiredCapacity` of
	/// the Auto Scaling group by at least this many instances. Otherwise, the error
	/// is `ValidationError`.
	pub min_adjustment_magnitude: Option<MinAdjustmentMagnitude>,
	/// The aggregation type for the CloudWatch metrics. Valid values are `Minimum`,
	/// `Maximum`, and `Average`. If the aggregation type is null, the value is
	/// treated as `Average`.
	/// This parameter is not supported if the policy type is `SimpleScaling`.
	pub metric_aggregation_type: Option<XmlStringMaxLen32>,
	/// The name or ARN of the group.
	pub auto_scaling_group_name: ResourceName,
	/// The amount of time, in seconds, after a scaling activity completes and before
	/// the next scaling activity can start. If this parameter is not specified, the
	/// default cooldown period for the group applies.
	/// This parameter is not supported unless the policy type is `SimpleScaling`.
	/// For more information, see [Understanding Auto Scaling Cooldowns](http://docs.a
	/// ws.amazon.com/AutoScaling/latest/DeveloperGuide/Cooldown.html) in the _Auto
	/// Scaling Developer Guide_.
	pub cooldown: Option<Cooldown>,
	/// The policy type. Valid values are `SimpleScaling` and `StepScaling`. If the
	/// policy type is null, the value is treated as `SimpleScaling`.
	pub policy_type: Option<XmlStringMaxLen64>,
	/// A set of adjustments that enable you to scale based on the size of the alarm
	/// breach.
	/// This parameter is required if the policy type is `StepScaling` and not
	/// supported otherwise.
	pub step_adjustments: Option<StepAdjustments>,
	/// The adjustment type. Valid values are `ChangeInCapacity`, `ExactCapacity`, and
	/// `PercentChangeInCapacity`.
	/// For more information, see [Dynamic
	/// Scaling](http://docs.aws.amazon.com/AutoScaling/latest/DeveloperGuide/as-
	/// scale-based-on-demand.html) in the _Auto Scaling Developer Guide_.
	pub adjustment_type: XmlStringMaxLen255,
	/// The amount by which to scale, based on the specified adjustment type. A
	/// positive value adds to the current capacity while a negative number removes
	/// from the current capacity.
	/// This parameter is required if the policy type is `SimpleScaling` and not
	/// supported otherwise.
	pub scaling_adjustment: Option<PolicyIncrement>,
}

/// Parse PutScalingPolicyType from XML
struct PutScalingPolicyTypeParser;
impl PutScalingPolicyTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<PutScalingPolicyType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = PutScalingPolicyType::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "PolicyName" {
				obj.policy_name = try!(XmlStringMaxLen255Parser::parse_xml("PolicyName", stack));
				continue;
			}
			if current_name == "EstimatedInstanceWarmup" {
				obj.estimated_instance_warmup = Some(try!(EstimatedInstanceWarmupParser::parse_xml("EstimatedInstanceWarmup", stack)));
				continue;
			}
			if current_name == "MinAdjustmentStep" {
				obj.min_adjustment_step = Some(try!(MinAdjustmentStepParser::parse_xml("MinAdjustmentStep", stack)));
				continue;
			}
			if current_name == "MinAdjustmentMagnitude" {
				obj.min_adjustment_magnitude = Some(try!(MinAdjustmentMagnitudeParser::parse_xml("MinAdjustmentMagnitude", stack)));
				continue;
			}
			if current_name == "MetricAggregationType" {
				obj.metric_aggregation_type = Some(try!(XmlStringMaxLen32Parser::parse_xml("MetricAggregationType", stack)));
				continue;
			}
			if current_name == "AutoScalingGroupName" {
				obj.auto_scaling_group_name = try!(ResourceNameParser::parse_xml("AutoScalingGroupName", stack));
				continue;
			}
			if current_name == "Cooldown" {
				obj.cooldown = Some(try!(CooldownParser::parse_xml("Cooldown", stack)));
				continue;
			}
			if current_name == "PolicyType" {
				obj.policy_type = Some(try!(XmlStringMaxLen64Parser::parse_xml("PolicyType", stack)));
				continue;
			}
			if current_name == "StepAdjustment" {
				obj.step_adjustments = Some(try!(StepAdjustmentsParser::parse_xml("StepAdjustment", stack)));
				continue;
			}
			if current_name == "AdjustmentType" {
				obj.adjustment_type = try!(XmlStringMaxLen255Parser::parse_xml("AdjustmentType", stack));
				continue;
			}
			if current_name == "ScalingAdjustment" {
				obj.scaling_adjustment = Some(try!(PolicyIncrementParser::parse_xml("ScalingAdjustment", stack)));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write PutScalingPolicyType contents to a SignedRequest
struct PutScalingPolicyTypeWriter;
impl PutScalingPolicyTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &PutScalingPolicyType) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		XmlStringMaxLen255Writer::write_params(params, &(prefix.to_string() + "PolicyName"), &obj.policy_name);
		if let Some(ref obj) = obj.estimated_instance_warmup {
			EstimatedInstanceWarmupWriter::write_params(params, &(prefix.to_string() + "EstimatedInstanceWarmup"), obj);
		}
		if let Some(ref obj) = obj.min_adjustment_step {
			MinAdjustmentStepWriter::write_params(params, &(prefix.to_string() + "MinAdjustmentStep"), obj);
		}
		if let Some(ref obj) = obj.min_adjustment_magnitude {
			MinAdjustmentMagnitudeWriter::write_params(params, &(prefix.to_string() + "MinAdjustmentMagnitude"), obj);
		}
		if let Some(ref obj) = obj.metric_aggregation_type {
			XmlStringMaxLen32Writer::write_params(params, &(prefix.to_string() + "MetricAggregationType"), obj);
		}
		ResourceNameWriter::write_params(params, &(prefix.to_string() + "AutoScalingGroupName"), &obj.auto_scaling_group_name);
		if let Some(ref obj) = obj.cooldown {
			CooldownWriter::write_params(params, &(prefix.to_string() + "Cooldown"), obj);
		}
		if let Some(ref obj) = obj.policy_type {
			XmlStringMaxLen64Writer::write_params(params, &(prefix.to_string() + "PolicyType"), obj);
		}
		if let Some(ref obj) = obj.step_adjustments {
			StepAdjustmentsWriter::write_params(params, &(prefix.to_string() + "StepAdjustment"), obj);
		}
		XmlStringMaxLen255Writer::write_params(params, &(prefix.to_string() + "AdjustmentType"), &obj.adjustment_type);
		if let Some(ref obj) = obj.scaling_adjustment {
			PolicyIncrementWriter::write_params(params, &(prefix.to_string() + "ScalingAdjustment"), obj);
		}
	}
}
/// Describes whether instance monitoring is enabled.
#[derive(Debug, Default)]
pub struct InstanceMonitoring {
	/// If `True`, instance monitoring is enabled.
	pub enabled: MonitoringEnabled,
}

/// Parse InstanceMonitoring from XML
struct InstanceMonitoringParser;
impl InstanceMonitoringParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<InstanceMonitoring, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = InstanceMonitoring::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "Enabled" {
				obj.enabled = try!(MonitoringEnabledParser::parse_xml("Enabled", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write InstanceMonitoring contents to a SignedRequest
struct InstanceMonitoringWriter;
impl InstanceMonitoringWriter {
	fn write_params(params: &mut Params, name: &str, obj: &InstanceMonitoring) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		MonitoringEnabledWriter::write_params(params, &(prefix.to_string() + "Enabled"), &obj.enabled);
	}
}
pub type ClassicLinkVPCSecurityGroups = Vec<XmlStringMaxLen255>;
/// Parse ClassicLinkVPCSecurityGroups from XML
struct ClassicLinkVPCSecurityGroupsParser;
impl ClassicLinkVPCSecurityGroupsParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ClassicLinkVPCSecurityGroups, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "XmlStringMaxLen255" {
			obj.push(try!(XmlStringMaxLen255Parser::parse_xml("XmlStringMaxLen255", stack)));
		}
		Ok(obj)
	}
}
/// Write ClassicLinkVPCSecurityGroups contents to a SignedRequest
struct ClassicLinkVPCSecurityGroupsWriter;
impl ClassicLinkVPCSecurityGroupsWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ClassicLinkVPCSecurityGroups) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			XmlStringMaxLen255Writer::write_params(params, key, &element);
			index += 1;
		}
	}
}
pub type TagDescriptionList = Vec<TagDescription>;
/// Parse TagDescriptionList from XML
struct TagDescriptionListParser;
impl TagDescriptionListParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<TagDescriptionList, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "TagDescription" {
			obj.push(try!(TagDescriptionParser::parse_xml("TagDescription", stack)));
		}
		Ok(obj)
	}
}
/// Write TagDescriptionList contents to a SignedRequest
struct TagDescriptionListWriter;
impl TagDescriptionListWriter {
	fn write_params(params: &mut Params, name: &str, obj: &TagDescriptionList) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			TagDescriptionWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
/// Describes an EC2 instance.
#[derive(Debug, Default)]
pub struct Instance {
	/// The ID of the instance.
	pub instance_id: XmlStringMaxLen16,
	/// The Availability Zone in which the instance is running.
	pub availability_zone: XmlStringMaxLen255,
	/// The health status of the instance.
	pub health_status: XmlStringMaxLen32,
	/// A description of the current lifecycle state. Note that the `Quarantined`
	/// state is not used.
	pub lifecycle_state: LifecycleState,
	/// The launch configuration associated with the instance.
	pub launch_configuration_name: XmlStringMaxLen255,
}

/// Parse Instance from XML
struct InstanceParser;
impl InstanceParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<Instance, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = Instance::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "InstanceId" {
				obj.instance_id = try!(XmlStringMaxLen16Parser::parse_xml("InstanceId", stack));
				continue;
			}
			if current_name == "AvailabilityZone" {
				obj.availability_zone = try!(XmlStringMaxLen255Parser::parse_xml("AvailabilityZone", stack));
				continue;
			}
			if current_name == "HealthStatus" {
				obj.health_status = try!(XmlStringMaxLen32Parser::parse_xml("HealthStatus", stack));
				continue;
			}
			if current_name == "LifecycleState" {
				obj.lifecycle_state = try!(LifecycleStateParser::parse_xml("LifecycleState", stack));
				continue;
			}
			if current_name == "LaunchConfigurationName" {
				obj.launch_configuration_name = try!(XmlStringMaxLen255Parser::parse_xml("LaunchConfigurationName", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write Instance contents to a SignedRequest
struct InstanceWriter;
impl InstanceWriter {
	fn write_params(params: &mut Params, name: &str, obj: &Instance) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		XmlStringMaxLen16Writer::write_params(params, &(prefix.to_string() + "InstanceId"), &obj.instance_id);
		XmlStringMaxLen255Writer::write_params(params, &(prefix.to_string() + "AvailabilityZone"), &obj.availability_zone);
		XmlStringMaxLen32Writer::write_params(params, &(prefix.to_string() + "HealthStatus"), &obj.health_status);
		LifecycleStateWriter::write_params(params, &(prefix.to_string() + "LifecycleState"), &obj.lifecycle_state);
		XmlStringMaxLen255Writer::write_params(params, &(prefix.to_string() + "LaunchConfigurationName"), &obj.launch_configuration_name);
	}
}
pub type EbsOptimized = bool;
/// Parse EbsOptimized from XML
struct EbsOptimizedParser;
impl EbsOptimizedParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<EbsOptimized, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = bool::from_str(try!(characters(stack)).as_ref()).unwrap();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write EbsOptimized contents to a SignedRequest
struct EbsOptimizedWriter;
impl EbsOptimizedWriter {
	fn write_params(params: &mut Params, name: &str, obj: &EbsOptimized) {
		params.put(name, &obj.to_string());
	}
}
pub type PolicyNames = Vec<ResourceName>;
/// Parse PolicyNames from XML
struct PolicyNamesParser;
impl PolicyNamesParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<PolicyNames, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "ResourceName" {
			obj.push(try!(ResourceNameParser::parse_xml("ResourceName", stack)));
		}
		Ok(obj)
	}
}
/// Write PolicyNames contents to a SignedRequest
struct PolicyNamesWriter;
impl PolicyNamesWriter {
	fn write_params(params: &mut Params, name: &str, obj: &PolicyNames) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			ResourceNameWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
#[derive(Debug, Default)]
pub struct DetachLoadBalancersType {
	/// The name of the group.
	pub auto_scaling_group_name: ResourceName,
	/// One or more load balancer names.
	pub load_balancer_names: LoadBalancerNames,
}

/// Parse DetachLoadBalancersType from XML
struct DetachLoadBalancersTypeParser;
impl DetachLoadBalancersTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<DetachLoadBalancersType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = DetachLoadBalancersType::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "AutoScalingGroupName" {
				obj.auto_scaling_group_name = try!(ResourceNameParser::parse_xml("AutoScalingGroupName", stack));
				continue;
			}
			if current_name == "XmlStringMaxLen255" {
				obj.load_balancer_names = try!(LoadBalancerNamesParser::parse_xml("XmlStringMaxLen255", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write DetachLoadBalancersType contents to a SignedRequest
struct DetachLoadBalancersTypeWriter;
impl DetachLoadBalancersTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &DetachLoadBalancersType) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		ResourceNameWriter::write_params(params, &(prefix.to_string() + "AutoScalingGroupName"), &obj.auto_scaling_group_name);
		LoadBalancerNamesWriter::write_params(params, &(prefix.to_string() + "XmlStringMaxLen255"), &obj.load_balancer_names);
	}
}
pub type LoadBalancerStates = Vec<LoadBalancerState>;
/// Parse LoadBalancerStates from XML
struct LoadBalancerStatesParser;
impl LoadBalancerStatesParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<LoadBalancerStates, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "LoadBalancerState" {
			obj.push(try!(LoadBalancerStateParser::parse_xml("LoadBalancerState", stack)));
		}
		Ok(obj)
	}
}
/// Write LoadBalancerStates contents to a SignedRequest
struct LoadBalancerStatesWriter;
impl LoadBalancerStatesWriter {
	fn write_params(params: &mut Params, name: &str, obj: &LoadBalancerStates) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			LoadBalancerStateWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
pub type InstanceIds = Vec<XmlStringMaxLen16>;
/// Parse InstanceIds from XML
struct InstanceIdsParser;
impl InstanceIdsParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<InstanceIds, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "XmlStringMaxLen16" {
			obj.push(try!(XmlStringMaxLen16Parser::parse_xml("XmlStringMaxLen16", stack)));
		}
		Ok(obj)
	}
}
/// Write InstanceIds contents to a SignedRequest
struct InstanceIdsWriter;
impl InstanceIdsWriter {
	fn write_params(params: &mut Params, name: &str, obj: &InstanceIds) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			XmlStringMaxLen16Writer::write_params(params, key, &element);
			index += 1;
		}
	}
}
#[derive(Debug, Default)]
pub struct TagsType {
	/// The token to use when requesting the next set of items. If there are no
	/// additional items to return, the string is empty.
	pub next_token: XmlString,
	/// The tags.
	pub tags: TagDescriptionList,
}

/// Parse TagsType from XML
struct TagsTypeParser;
impl TagsTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<TagsType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = TagsType::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "NextToken" {
				obj.next_token = try!(XmlStringParser::parse_xml("NextToken", stack));
				continue;
			}
			if current_name == "TagDescription" {
				obj.tags = try!(TagDescriptionListParser::parse_xml("TagDescription", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write TagsType contents to a SignedRequest
struct TagsTypeWriter;
impl TagsTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &TagsType) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		XmlStringWriter::write_params(params, &(prefix.to_string() + "NextToken"), &obj.next_token);
		TagDescriptionListWriter::write_params(params, &(prefix.to_string() + "TagDescription"), &obj.tags);
	}
}
pub type Tags = Vec<Tag>;
/// Parse Tags from XML
struct TagsParser;
impl TagsParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<Tags, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "Tag" {
			obj.push(try!(TagParser::parse_xml("Tag", stack)));
		}
		Ok(obj)
	}
}
/// Write Tags contents to a SignedRequest
struct TagsWriter;
impl TagsWriter {
	fn write_params(params: &mut Params, name: &str, obj: &Tags) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			TagWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
#[derive(Debug, Default)]
pub struct LaunchConfigurationsType {
	/// The token to use when requesting the next set of items. If there are no
	/// additional items to return, the string is empty.
	pub next_token: Option<XmlString>,
	/// The launch configurations.
	pub launch_configurations: LaunchConfigurations,
}

/// Parse LaunchConfigurationsType from XML
struct LaunchConfigurationsTypeParser;
impl LaunchConfigurationsTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<LaunchConfigurationsType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = LaunchConfigurationsType::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "NextToken" {
				obj.next_token = Some(try!(XmlStringParser::parse_xml("NextToken", stack)));
				continue;
			}
			if current_name == "LaunchConfiguration" {
				obj.launch_configurations = try!(LaunchConfigurationsParser::parse_xml("LaunchConfiguration", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write LaunchConfigurationsType contents to a SignedRequest
struct LaunchConfigurationsTypeWriter;
impl LaunchConfigurationsTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &LaunchConfigurationsType) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		if let Some(ref obj) = obj.next_token {
			XmlStringWriter::write_params(params, &(prefix.to_string() + "NextToken"), obj);
		}
		LaunchConfigurationsWriter::write_params(params, &(prefix.to_string() + "LaunchConfiguration"), &obj.launch_configurations);
	}
}
/// You have already reached a limit for your Auto Scaling resources (for example,
/// groups, launch configurations, or lifecycle hooks). For more information, see
/// DescribeAccountLimits.
#[derive(Debug, Default)]
pub struct LimitExceededFault {
	pub message: XmlStringMaxLen255,
}

/// Parse LimitExceededFault from XML
struct LimitExceededFaultParser;
impl LimitExceededFaultParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<LimitExceededFault, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = LimitExceededFault::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "message" {
				obj.message = try!(XmlStringMaxLen255Parser::parse_xml("message", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write LimitExceededFault contents to a SignedRequest
struct LimitExceededFaultWriter;
impl LimitExceededFaultWriter {
	fn write_params(params: &mut Params, name: &str, obj: &LimitExceededFault) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		XmlStringMaxLen255Writer::write_params(params, &(prefix.to_string() + "message"), &obj.message);
	}
}
pub type LifecycleActionToken = String;
/// Parse LifecycleActionToken from XML
struct LifecycleActionTokenParser;
impl LifecycleActionTokenParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<LifecycleActionToken, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write LifecycleActionToken contents to a SignedRequest
struct LifecycleActionTokenWriter;
impl LifecycleActionTokenWriter {
	fn write_params(params: &mut Params, name: &str, obj: &LifecycleActionToken) {
		params.put(name, obj);
	}
}
#[derive(Debug, Default)]
pub struct CreateLaunchConfigurationType {
	/// The user data to make available to the launched EC2 instances. For more
	/// information, see [Instance Metadata and User
	/// Data](http://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-instance-
	/// metadata.html) in the _Amazon Elastic Compute Cloud User Guide_.
	/// At this time, launch configurations don't support compressed (zipped) user
	/// data files.
	pub user_data: Option<XmlStringUserData>,
	/// The name or the Amazon Resource Name (ARN) of the instance profile associated
	/// with the IAM role for the instance.
	/// EC2 instances launched with an IAM role will automatically have AWS security
	/// credentials available. You can use IAM roles with Auto Scaling to
	/// automatically enable applications running on your EC2 instances to securely
	/// access other AWS resources. For more information, see [Launch Auto Scaling
	/// Instances with an IAM
	/// Role](http://docs.aws.amazon.com/AutoScaling/latest/DeveloperGuide/us-iam-
	/// role.html) in the _Auto Scaling Developer Guide_.
	pub iam_instance_profile: Option<XmlStringMaxLen1600>,
	/// The ID of a ClassicLink-enabled VPC to link your EC2-Classic instances to.
	/// This parameter is supported only if you are launching EC2-Classic instances.
	/// For more information, see
	/// [ClassicLink](http://docs.aws.amazon.com/AWSEC2/latest/UserGuide/vpc-
	/// classiclink.html) in the _Amazon Elastic Compute Cloud User Guide_.
	pub classic_link_vpc_id: Option<XmlStringMaxLen255>,
	/// The ID of the EC2 instance to use to create the launch configuration.
	/// The new launch configuration derives attributes from the instance, with the
	/// exception of the block device mapping.
	/// To create a launch configuration with a block device mapping or override any
	/// other instance attributes, specify them as part of the same request.
	/// For more information, see [Create a Launch Configuration Using an EC2
	/// Instance](http://docs.aws.amazon.com/AutoScaling/latest/DeveloperGuide/create-
	/// lc-with-instanceID.html) in the _Auto Scaling Developer Guide_.
	pub instance_id: Option<XmlStringMaxLen16>,
	/// The tenancy of the instance. An instance with a tenancy of `dedicated` runs on
	/// single-tenant hardware and can only be launched into a VPC.
	/// You must set the value of this parameter to `dedicated` if want to launch
	/// Dedicated Instances into a shared tenancy VPC (VPC with instance placement
	/// tenancy attribute set to `default`).
	/// If you specify a value for this parameter, be sure to specify at least one
	/// subnet using the _VPCZoneIdentifier_ parameter when you create your group.
	/// For more information, see [Auto Scaling and Amazon Virtual Private Cloud](http
	/// ://docs.aws.amazon.com/AutoScaling/latest/DeveloperGuide/autoscalingsubnets.ht
	/// ml) in the _Auto Scaling Developer Guide_.
	/// Valid values: `default` | `dedicated`
	pub placement_tenancy: Option<XmlStringMaxLen64>,
	/// Used for groups that launch instances into a virtual private cloud (VPC).
	/// Specifies whether to assign a public IP address to each instance. For more
	/// information, see [Auto Scaling and Amazon Virtual Private Cloud](http://docs.a
	/// ws.amazon.com/AutoScaling/latest/DeveloperGuide/autoscalingsubnets.html) in
	/// the _Auto Scaling Developer Guide_.
	/// If you specify a value for this parameter, be sure to specify at least one
	/// subnet using the _VPCZoneIdentifier_ parameter when you create your group.
	/// Default: If the instance is launched into a default subnet, the default is
	/// `true`. If the instance is launched into a nondefault subnet, the default is
	/// `false`. For more information, see [Supported
	/// Platforms](http://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-supported-
	/// platforms.html) in the _Amazon Elastic Compute Cloud User Guide_.
	pub associate_public_ip_address: Option<AssociatePublicIpAddress>,
	/// Enables detailed monitoring if it is disabled. Detailed monitoring is enabled
	/// by default.
	/// When detailed monitoring is enabled, Amazon CloudWatch generates metrics every
	/// minute and your account is charged a fee. When you disable detailed
	/// monitoring, by specifying `False`, CloudWatch generates metrics every 5
	/// minutes. For more information, see [Monitor Your Auto Scaling
	/// Instances](http://docs.aws.amazon.com/AutoScaling/latest/DeveloperGuide/as-
	/// instance-monitoring.html) in the _Auto Scaling Developer Guide_.
	pub instance_monitoring: Option<InstanceMonitoring>,
	/// The IDs of one or more security groups for the VPC specified in
	/// `ClassicLinkVPCId`. This parameter is required if `ClassicLinkVPCId` is
	/// specified, and is not supported otherwise. For more information, see
	/// [ClassicLink](http://docs.aws.amazon.com/AWSEC2/latest/UserGuide/vpc-
	/// classiclink.html) in the _Amazon Elastic Compute Cloud User Guide_.
	pub classic_link_vpc_security_groups: Option<ClassicLinkVPCSecurityGroups>,
	/// One or more mappings that specify how block devices are exposed to the
	/// instance. For more information, see [Block Device
	/// Mapping](http://docs.aws.amazon.com/AWSEC2/latest/UserGuide/block-device-
	/// mapping-concepts.html) in the _Amazon Elastic Compute Cloud User Guide_.
	pub block_device_mappings: Option<BlockDeviceMappings>,
	/// The name of the key pair. For more information, see [Amazon EC2 Key
	/// Pairs](http://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-key-pairs.html)
	/// in the _Amazon Elastic Compute Cloud User Guide_.
	pub key_name: Option<XmlStringMaxLen255>,
	/// One or more security groups with which to associate the instances.
	/// If your instances are launched in EC2-Classic, you can either specify security
	/// group names or the security group IDs. For more information about security
	/// groups for EC2-Classic, see [Amazon EC2 Security
	/// Groups](http://docs.aws.amazon.com/AWSEC2/latest/UserGuide/using-network-
	/// security.html) in the _Amazon Elastic Compute Cloud User Guide_.
	/// If your instances are launched into a VPC, specify security group IDs. For
	/// more information, see [Security Groups for Your VPC](http://docs.aws.amazon.co
	/// m/AmazonVPC/latest/UserGuide/VPC_SecurityGroups.html) in the _Amazon Virtual
	/// Private Cloud User Guide_.
	pub security_groups: Option<SecurityGroups>,
	/// Indicates whether the instance is optimized for Amazon EBS I/O. By default,
	/// the instance is not optimized for EBS I/O. The optimization provides dedicated
	/// throughput to Amazon EBS and an optimized configuration stack to provide
	/// optimal I/O performance. This optimization is not available with all instance
	/// types. Additional usage charges apply. For more information, see [Amazon EBS-
	/// Optimized Instances](http://docs.aws.amazon.com/AWSEC2/latest/UserGuide/EBSOpt
	/// imized.html) in the _Amazon Elastic Compute Cloud User Guide_.
	pub ebs_optimized: Option<EbsOptimized>,
	/// The name of the launch configuration. This name must be unique within the
	/// scope of your AWS account.
	pub launch_configuration_name: XmlStringMaxLen255,
	/// The ID of the kernel associated with the AMI.
	pub kernel_id: Option<XmlStringMaxLen255>,
	/// The ID of the RAM disk associated with the AMI.
	pub ramdisk_id: Option<XmlStringMaxLen255>,
	/// The ID of the Amazon Machine Image (AMI) to use to launch your EC2 instances.
	/// For more information, see [Finding an
	/// AMI](http://docs.aws.amazon.com/AWSEC2/latest/UserGuide/finding-an-ami.html)
	/// in the _Amazon Elastic Compute Cloud User Guide_.
	pub image_id: Option<XmlStringMaxLen255>,
	/// The instance type of the EC2 instance. For information about available
	/// instance types, see [ Available Instance
	/// Types](http://docs.aws.amazon.com/AWSEC2/latest/UserGuide/instance-
	/// types.html#AvailableInstanceTypes) in the _Amazon Elastic Cloud Compute User
	/// Guide._
	pub instance_type: Option<XmlStringMaxLen255>,
	/// The maximum hourly price to be paid for any Spot Instance launched to fulfill
	/// the request. Spot Instances are launched when the price you specify exceeds
	/// the current Spot market price. For more information, see [Launch Spot
	/// Instances in Your Auto Scaling
	/// Group](http://docs.aws.amazon.com/AutoScaling/latest/DeveloperGuide/US-
	/// SpotInstances.html) in the _Auto Scaling Developer Guide_.
	pub spot_price: Option<SpotPrice>,
}

/// Parse CreateLaunchConfigurationType from XML
struct CreateLaunchConfigurationTypeParser;
impl CreateLaunchConfigurationTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<CreateLaunchConfigurationType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = CreateLaunchConfigurationType::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "UserData" {
				obj.user_data = Some(try!(XmlStringUserDataParser::parse_xml("UserData", stack)));
				continue;
			}
			if current_name == "IamInstanceProfile" {
				obj.iam_instance_profile = Some(try!(XmlStringMaxLen1600Parser::parse_xml("IamInstanceProfile", stack)));
				continue;
			}
			if current_name == "ClassicLinkVPCId" {
				obj.classic_link_vpc_id = Some(try!(XmlStringMaxLen255Parser::parse_xml("ClassicLinkVPCId", stack)));
				continue;
			}
			if current_name == "InstanceId" {
				obj.instance_id = Some(try!(XmlStringMaxLen16Parser::parse_xml("InstanceId", stack)));
				continue;
			}
			if current_name == "PlacementTenancy" {
				obj.placement_tenancy = Some(try!(XmlStringMaxLen64Parser::parse_xml("PlacementTenancy", stack)));
				continue;
			}
			if current_name == "AssociatePublicIpAddress" {
				obj.associate_public_ip_address = Some(try!(AssociatePublicIpAddressParser::parse_xml("AssociatePublicIpAddress", stack)));
				continue;
			}
			if current_name == "InstanceMonitoring" {
				obj.instance_monitoring = Some(try!(InstanceMonitoringParser::parse_xml("InstanceMonitoring", stack)));
				continue;
			}
			if current_name == "XmlStringMaxLen255" {
				obj.classic_link_vpc_security_groups = Some(try!(ClassicLinkVPCSecurityGroupsParser::parse_xml("XmlStringMaxLen255", stack)));
				continue;
			}
			if current_name == "BlockDeviceMapping" {
				obj.block_device_mappings = Some(try!(BlockDeviceMappingsParser::parse_xml("BlockDeviceMapping", stack)));
				continue;
			}
			if current_name == "KeyName" {
				obj.key_name = Some(try!(XmlStringMaxLen255Parser::parse_xml("KeyName", stack)));
				continue;
			}
			if current_name == "XmlString" {
				obj.security_groups = Some(try!(SecurityGroupsParser::parse_xml("XmlString", stack)));
				continue;
			}
			if current_name == "EbsOptimized" {
				obj.ebs_optimized = Some(try!(EbsOptimizedParser::parse_xml("EbsOptimized", stack)));
				continue;
			}
			if current_name == "LaunchConfigurationName" {
				obj.launch_configuration_name = try!(XmlStringMaxLen255Parser::parse_xml("LaunchConfigurationName", stack));
				continue;
			}
			if current_name == "KernelId" {
				obj.kernel_id = Some(try!(XmlStringMaxLen255Parser::parse_xml("KernelId", stack)));
				continue;
			}
			if current_name == "RamdiskId" {
				obj.ramdisk_id = Some(try!(XmlStringMaxLen255Parser::parse_xml("RamdiskId", stack)));
				continue;
			}
			if current_name == "ImageId" {
				obj.image_id = Some(try!(XmlStringMaxLen255Parser::parse_xml("ImageId", stack)));
				continue;
			}
			if current_name == "InstanceType" {
				obj.instance_type = Some(try!(XmlStringMaxLen255Parser::parse_xml("InstanceType", stack)));
				continue;
			}
			if current_name == "SpotPrice" {
				obj.spot_price = Some(try!(SpotPriceParser::parse_xml("SpotPrice", stack)));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write CreateLaunchConfigurationType contents to a SignedRequest
struct CreateLaunchConfigurationTypeWriter;
impl CreateLaunchConfigurationTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &CreateLaunchConfigurationType) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		if let Some(ref obj) = obj.user_data {
			XmlStringUserDataWriter::write_params(params, &(prefix.to_string() + "UserData"), obj);
		}
		if let Some(ref obj) = obj.iam_instance_profile {
			XmlStringMaxLen1600Writer::write_params(params, &(prefix.to_string() + "IamInstanceProfile"), obj);
		}
		if let Some(ref obj) = obj.classic_link_vpc_id {
			XmlStringMaxLen255Writer::write_params(params, &(prefix.to_string() + "ClassicLinkVPCId"), obj);
		}
		if let Some(ref obj) = obj.instance_id {
			XmlStringMaxLen16Writer::write_params(params, &(prefix.to_string() + "InstanceId"), obj);
		}
		if let Some(ref obj) = obj.placement_tenancy {
			XmlStringMaxLen64Writer::write_params(params, &(prefix.to_string() + "PlacementTenancy"), obj);
		}
		if let Some(ref obj) = obj.associate_public_ip_address {
			AssociatePublicIpAddressWriter::write_params(params, &(prefix.to_string() + "AssociatePublicIpAddress"), obj);
		}
		if let Some(ref obj) = obj.instance_monitoring {
			InstanceMonitoringWriter::write_params(params, &(prefix.to_string() + "InstanceMonitoring"), obj);
		}
		if let Some(ref obj) = obj.classic_link_vpc_security_groups {
			ClassicLinkVPCSecurityGroupsWriter::write_params(params, &(prefix.to_string() + "XmlStringMaxLen255"), obj);
		}
		if let Some(ref obj) = obj.block_device_mappings {
			BlockDeviceMappingsWriter::write_params(params, &(prefix.to_string() + "BlockDeviceMapping"), obj);
		}
		if let Some(ref obj) = obj.key_name {
			XmlStringMaxLen255Writer::write_params(params, &(prefix.to_string() + "KeyName"), obj);
		}
		if let Some(ref obj) = obj.security_groups {
			SecurityGroupsWriter::write_params(params, &(prefix.to_string() + "XmlString"), obj);
		}
		if let Some(ref obj) = obj.ebs_optimized {
			EbsOptimizedWriter::write_params(params, &(prefix.to_string() + "EbsOptimized"), obj);
		}
		XmlStringMaxLen255Writer::write_params(params, &(prefix.to_string() + "LaunchConfigurationName"), &obj.launch_configuration_name);
		if let Some(ref obj) = obj.kernel_id {
			XmlStringMaxLen255Writer::write_params(params, &(prefix.to_string() + "KernelId"), obj);
		}
		if let Some(ref obj) = obj.ramdisk_id {
			XmlStringMaxLen255Writer::write_params(params, &(prefix.to_string() + "RamdiskId"), obj);
		}
		if let Some(ref obj) = obj.image_id {
			XmlStringMaxLen255Writer::write_params(params, &(prefix.to_string() + "ImageId"), obj);
		}
		if let Some(ref obj) = obj.instance_type {
			XmlStringMaxLen255Writer::write_params(params, &(prefix.to_string() + "InstanceType"), obj);
		}
		if let Some(ref obj) = obj.spot_price {
			SpotPriceWriter::write_params(params, &(prefix.to_string() + "SpotPrice"), obj);
		}
	}
}
pub type HeartbeatTimeout = i32;
/// Parse HeartbeatTimeout from XML
struct HeartbeatTimeoutParser;
impl HeartbeatTimeoutParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<HeartbeatTimeout, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = i32::from_str(try!(characters(stack)).as_ref()).unwrap();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write HeartbeatTimeout contents to a SignedRequest
struct HeartbeatTimeoutWriter;
impl HeartbeatTimeoutWriter {
	fn write_params(params: &mut Params, name: &str, obj: &HeartbeatTimeout) {
		params.put(name, &obj.to_string());
	}
}
#[derive(Debug, Default)]
pub struct PolicyARNType {
	/// The Amazon Resource Name (ARN) of the policy.
	pub policy_arn: ResourceName,
}

/// Parse PolicyARNType from XML
struct PolicyARNTypeParser;
impl PolicyARNTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<PolicyARNType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = PolicyARNType::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "PolicyARN" {
				obj.policy_arn = try!(ResourceNameParser::parse_xml("PolicyARN", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write PolicyARNType contents to a SignedRequest
struct PolicyARNTypeWriter;
impl PolicyARNTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &PolicyARNType) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		ResourceNameWriter::write_params(params, &(prefix.to_string() + "PolicyARN"), &obj.policy_arn);
	}
}
#[derive(Debug, Default)]
pub struct AutoScalingInstancesType {
	/// The token to use when requesting the next set of items. If there are no
	/// additional items to return, the string is empty.
	pub next_token: XmlString,
	/// The instances.
	pub auto_scaling_instances: AutoScalingInstances,
}

/// Parse AutoScalingInstancesType from XML
struct AutoScalingInstancesTypeParser;
impl AutoScalingInstancesTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<AutoScalingInstancesType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = AutoScalingInstancesType::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "NextToken" {
				obj.next_token = try!(XmlStringParser::parse_xml("NextToken", stack));
				continue;
			}
			if current_name == "AutoScalingInstanceDetails" {
				obj.auto_scaling_instances = try!(AutoScalingInstancesParser::parse_xml("AutoScalingInstanceDetails", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write AutoScalingInstancesType contents to a SignedRequest
struct AutoScalingInstancesTypeWriter;
impl AutoScalingInstancesTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &AutoScalingInstancesType) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		XmlStringWriter::write_params(params, &(prefix.to_string() + "NextToken"), &obj.next_token);
		AutoScalingInstancesWriter::write_params(params, &(prefix.to_string() + "AutoScalingInstanceDetails"), &obj.auto_scaling_instances);
	}
}
pub type AutoScalingGroupMaxSize = i32;
/// Parse AutoScalingGroupMaxSize from XML
struct AutoScalingGroupMaxSizeParser;
impl AutoScalingGroupMaxSizeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<AutoScalingGroupMaxSize, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = i32::from_str(try!(characters(stack)).as_ref()).unwrap();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write AutoScalingGroupMaxSize contents to a SignedRequest
struct AutoScalingGroupMaxSizeWriter;
impl AutoScalingGroupMaxSizeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &AutoScalingGroupMaxSize) {
		params.put(name, &obj.to_string());
	}
}
#[derive(Debug, Default)]
pub struct SetDesiredCapacityType {
	/// The name of the Auto Scaling group.
	pub auto_scaling_group_name: ResourceName,
	/// The number of EC2 instances that should be running in the Auto Scaling group.
	pub desired_capacity: AutoScalingGroupDesiredCapacity,
	/// By default, `SetDesiredCapacity` overrides any cooldown period associated with
	/// the Auto Scaling group. Specify `True` to make Auto Scaling to wait for the
	/// cool-down period associated with the Auto Scaling group to complete before
	/// initiating a scaling activity to set your Auto Scaling group to its new
	/// capacity.
	pub honor_cooldown: Option<HonorCooldown>,
}

/// Parse SetDesiredCapacityType from XML
struct SetDesiredCapacityTypeParser;
impl SetDesiredCapacityTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<SetDesiredCapacityType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = SetDesiredCapacityType::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "AutoScalingGroupName" {
				obj.auto_scaling_group_name = try!(ResourceNameParser::parse_xml("AutoScalingGroupName", stack));
				continue;
			}
			if current_name == "DesiredCapacity" {
				obj.desired_capacity = try!(AutoScalingGroupDesiredCapacityParser::parse_xml("DesiredCapacity", stack));
				continue;
			}
			if current_name == "HonorCooldown" {
				obj.honor_cooldown = Some(try!(HonorCooldownParser::parse_xml("HonorCooldown", stack)));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write SetDesiredCapacityType contents to a SignedRequest
struct SetDesiredCapacityTypeWriter;
impl SetDesiredCapacityTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &SetDesiredCapacityType) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		ResourceNameWriter::write_params(params, &(prefix.to_string() + "AutoScalingGroupName"), &obj.auto_scaling_group_name);
		AutoScalingGroupDesiredCapacityWriter::write_params(params, &(prefix.to_string() + "DesiredCapacity"), &obj.desired_capacity);
		if let Some(ref obj) = obj.honor_cooldown {
			HonorCooldownWriter::write_params(params, &(prefix.to_string() + "HonorCooldown"), obj);
		}
	}
}
#[derive(Debug, Default)]
pub struct AutoScalingGroupsType {
	/// The groups.
	pub auto_scaling_groups: AutoScalingGroups,
	/// The token to use when requesting the next set of items. If there are no
	/// additional items to return, the string is empty.
	pub next_token: Option<XmlString>,
}

/// Parse AutoScalingGroupsType from XML
struct AutoScalingGroupsTypeParser;
impl AutoScalingGroupsTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<AutoScalingGroupsType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = AutoScalingGroupsType::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "AutoScalingGroup" {
				obj.auto_scaling_groups = try!(AutoScalingGroupsParser::parse_xml("AutoScalingGroup", stack));
				continue;
			}
			if current_name == "NextToken" {
				obj.next_token = Some(try!(XmlStringParser::parse_xml("NextToken", stack)));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write AutoScalingGroupsType contents to a SignedRequest
struct AutoScalingGroupsTypeWriter;
impl AutoScalingGroupsTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &AutoScalingGroupsType) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		AutoScalingGroupsWriter::write_params(params, &(prefix.to_string() + "AutoScalingGroup"), &obj.auto_scaling_groups);
		if let Some(ref obj) = obj.next_token {
			XmlStringWriter::write_params(params, &(prefix.to_string() + "NextToken"), obj);
		}
	}
}
pub type Processes = Vec<ProcessType>;
/// Parse Processes from XML
struct ProcessesParser;
impl ProcessesParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<Processes, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "ProcessType" {
			obj.push(try!(ProcessTypeParser::parse_xml("ProcessType", stack)));
		}
		Ok(obj)
	}
}
/// Write Processes contents to a SignedRequest
struct ProcessesWriter;
impl ProcessesWriter {
	fn write_params(params: &mut Params, name: &str, obj: &Processes) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			ProcessTypeWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
/// Describes a block device mapping.
#[derive(Debug, Default)]
pub struct BlockDeviceMapping {
	/// The device name exposed to the EC2 instance (for example, `/dev/sdh` or
	/// `xvdh`).
	pub device_name: XmlStringMaxLen255,
	/// The name of the virtual device, `ephemeral0` to `ephemeral3`.
	pub virtual_name: Option<XmlStringMaxLen255>,
	/// Suppresses a device mapping.
	/// If this parameter is true for the root device, the instance might fail the EC2
	/// health check. Auto Scaling launches a replacement instance if the instance
	/// fails the health check.
	pub no_device: Option<NoDevice>,
	/// The information about the Amazon EBS volume.
	pub ebs: Option<Ebs>,
}

/// Parse BlockDeviceMapping from XML
struct BlockDeviceMappingParser;
impl BlockDeviceMappingParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<BlockDeviceMapping, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = BlockDeviceMapping::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "DeviceName" {
				obj.device_name = try!(XmlStringMaxLen255Parser::parse_xml("DeviceName", stack));
				continue;
			}
			if current_name == "VirtualName" {
				obj.virtual_name = Some(try!(XmlStringMaxLen255Parser::parse_xml("VirtualName", stack)));
				continue;
			}
			if current_name == "NoDevice" {
				obj.no_device = Some(try!(NoDeviceParser::parse_xml("NoDevice", stack)));
				continue;
			}
			if current_name == "Ebs" {
				obj.ebs = Some(try!(EbsParser::parse_xml("Ebs", stack)));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write BlockDeviceMapping contents to a SignedRequest
struct BlockDeviceMappingWriter;
impl BlockDeviceMappingWriter {
	fn write_params(params: &mut Params, name: &str, obj: &BlockDeviceMapping) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		XmlStringMaxLen255Writer::write_params(params, &(prefix.to_string() + "DeviceName"), &obj.device_name);
		if let Some(ref obj) = obj.virtual_name {
			XmlStringMaxLen255Writer::write_params(params, &(prefix.to_string() + "VirtualName"), obj);
		}
		if let Some(ref obj) = obj.no_device {
			NoDeviceWriter::write_params(params, &(prefix.to_string() + "NoDevice"), obj);
		}
		if let Some(ref obj) = obj.ebs {
			EbsWriter::write_params(params, &(prefix.to_string() + "Ebs"), obj);
		}
	}
}
pub type XmlStringUserData = String;
/// Parse XmlStringUserData from XML
struct XmlStringUserDataParser;
impl XmlStringUserDataParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<XmlStringUserData, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write XmlStringUserData contents to a SignedRequest
struct XmlStringUserDataWriter;
impl XmlStringUserDataWriter {
	fn write_params(params: &mut Params, name: &str, obj: &XmlStringUserData) {
		params.put(name, obj);
	}
}
pub type MetricGranularityTypes = Vec<MetricGranularityType>;
/// Parse MetricGranularityTypes from XML
struct MetricGranularityTypesParser;
impl MetricGranularityTypesParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<MetricGranularityTypes, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "MetricGranularityType" {
			obj.push(try!(MetricGranularityTypeParser::parse_xml("MetricGranularityType", stack)));
		}
		Ok(obj)
	}
}
/// Write MetricGranularityTypes contents to a SignedRequest
struct MetricGranularityTypesWriter;
impl MetricGranularityTypesWriter {
	fn write_params(params: &mut Params, name: &str, obj: &MetricGranularityTypes) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			MetricGranularityTypeWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
pub type LifecycleHooks = Vec<LifecycleHook>;
/// Parse LifecycleHooks from XML
struct LifecycleHooksParser;
impl LifecycleHooksParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<LifecycleHooks, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "LifecycleHook" {
			obj.push(try!(LifecycleHookParser::parse_xml("LifecycleHook", stack)));
		}
		Ok(obj)
	}
}
/// Write LifecycleHooks contents to a SignedRequest
struct LifecycleHooksWriter;
impl LifecycleHooksWriter {
	fn write_params(params: &mut Params, name: &str, obj: &LifecycleHooks) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			LifecycleHookWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
/// Describes a filter.
#[derive(Debug, Default)]
pub struct Filter {
	/// The value of the filter.
	pub values: Values,
	/// The name of the filter. The valid values are: `"auto-scaling-group"`, `"key"`,
	/// `"value"`, and `"propagate-at-launch"`.
	pub name: XmlString,
}

/// Parse Filter from XML
struct FilterParser;
impl FilterParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<Filter, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = Filter::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "XmlString" {
				obj.values = try!(ValuesParser::parse_xml("XmlString", stack));
				continue;
			}
			if current_name == "Name" {
				obj.name = try!(XmlStringParser::parse_xml("Name", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write Filter contents to a SignedRequest
struct FilterWriter;
impl FilterWriter {
	fn write_params(params: &mut Params, name: &str, obj: &Filter) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		ValuesWriter::write_params(params, &(prefix.to_string() + "XmlString"), &obj.values);
		XmlStringWriter::write_params(params, &(prefix.to_string() + "Name"), &obj.name);
	}
}
#[derive(Debug, Default)]
pub struct PutScheduledUpdateGroupActionType {
	/// The minimum size for the Auto Scaling group.
	pub min_size: Option<AutoScalingGroupMinSize>,
	/// The number of EC2 instances that should be running in the group.
	pub desired_capacity: Option<AutoScalingGroupDesiredCapacity>,
	/// The name or Amazon Resource Name (ARN) of the Auto Scaling group.
	pub auto_scaling_group_name: ResourceName,
	/// The time when recurring future actions will start. Start time is specified by
	/// the user following the Unix cron syntax format. For more information, see
	/// [Cron](http://en.wikipedia.org/wiki/Cron) in Wikipedia.
	/// When `StartTime` and `EndTime` are specified with `Recurrence`, they form the
	/// boundaries of when the recurring action will start and stop.
	pub recurrence: Option<XmlStringMaxLen255>,
	/// The maximum size for the Auto Scaling group.
	pub max_size: Option<AutoScalingGroupMaxSize>,
	/// The name of this scaling action.
	pub scheduled_action_name: XmlStringMaxLen255,
	/// The time for this action to start, in "YYYY-MM-DDThh:mm:ssZ" format in UTC/GMT
	/// only (for example, `2014-06-01T00:00:00Z`).
	/// If you try to schedule your action in the past, Auto Scaling returns an error
	/// message.
	/// When `StartTime` and `EndTime` are specified with `Recurrence`, they form the
	/// boundaries of when the recurring action starts and stops.
	pub start_time: Option<TimestampType>,
	/// This parameter is deprecated; use `StartTime` instead.
	/// The time for this action to start. If both `Time` and `StartTime` are
	/// specified, their values must be identical.
	pub time: Option<TimestampType>,
	/// The time for this action to end.
	pub end_time: Option<TimestampType>,
}

/// Parse PutScheduledUpdateGroupActionType from XML
struct PutScheduledUpdateGroupActionTypeParser;
impl PutScheduledUpdateGroupActionTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<PutScheduledUpdateGroupActionType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = PutScheduledUpdateGroupActionType::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "MinSize" {
				obj.min_size = Some(try!(AutoScalingGroupMinSizeParser::parse_xml("MinSize", stack)));
				continue;
			}
			if current_name == "DesiredCapacity" {
				obj.desired_capacity = Some(try!(AutoScalingGroupDesiredCapacityParser::parse_xml("DesiredCapacity", stack)));
				continue;
			}
			if current_name == "AutoScalingGroupName" {
				obj.auto_scaling_group_name = try!(ResourceNameParser::parse_xml("AutoScalingGroupName", stack));
				continue;
			}
			if current_name == "Recurrence" {
				obj.recurrence = Some(try!(XmlStringMaxLen255Parser::parse_xml("Recurrence", stack)));
				continue;
			}
			if current_name == "MaxSize" {
				obj.max_size = Some(try!(AutoScalingGroupMaxSizeParser::parse_xml("MaxSize", stack)));
				continue;
			}
			if current_name == "ScheduledActionName" {
				obj.scheduled_action_name = try!(XmlStringMaxLen255Parser::parse_xml("ScheduledActionName", stack));
				continue;
			}
			if current_name == "StartTime" {
				obj.start_time = Some(try!(TimestampTypeParser::parse_xml("StartTime", stack)));
				continue;
			}
			if current_name == "Time" {
				obj.time = Some(try!(TimestampTypeParser::parse_xml("Time", stack)));
				continue;
			}
			if current_name == "EndTime" {
				obj.end_time = Some(try!(TimestampTypeParser::parse_xml("EndTime", stack)));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write PutScheduledUpdateGroupActionType contents to a SignedRequest
struct PutScheduledUpdateGroupActionTypeWriter;
impl PutScheduledUpdateGroupActionTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &PutScheduledUpdateGroupActionType) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		if let Some(ref obj) = obj.min_size {
			AutoScalingGroupMinSizeWriter::write_params(params, &(prefix.to_string() + "MinSize"), obj);
		}
		if let Some(ref obj) = obj.desired_capacity {
			AutoScalingGroupDesiredCapacityWriter::write_params(params, &(prefix.to_string() + "DesiredCapacity"), obj);
		}
		ResourceNameWriter::write_params(params, &(prefix.to_string() + "AutoScalingGroupName"), &obj.auto_scaling_group_name);
		if let Some(ref obj) = obj.recurrence {
			XmlStringMaxLen255Writer::write_params(params, &(prefix.to_string() + "Recurrence"), obj);
		}
		if let Some(ref obj) = obj.max_size {
			AutoScalingGroupMaxSizeWriter::write_params(params, &(prefix.to_string() + "MaxSize"), obj);
		}
		XmlStringMaxLen255Writer::write_params(params, &(prefix.to_string() + "ScheduledActionName"), &obj.scheduled_action_name);
		if let Some(ref obj) = obj.start_time {
			TimestampTypeWriter::write_params(params, &(prefix.to_string() + "StartTime"), obj);
		}
		if let Some(ref obj) = obj.time {
			TimestampTypeWriter::write_params(params, &(prefix.to_string() + "Time"), obj);
		}
		if let Some(ref obj) = obj.end_time {
			TimestampTypeWriter::write_params(params, &(prefix.to_string() + "EndTime"), obj);
		}
	}
}
pub type AdjustmentTypes = Vec<AdjustmentType>;
/// Parse AdjustmentTypes from XML
struct AdjustmentTypesParser;
impl AdjustmentTypesParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<AdjustmentTypes, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "AdjustmentType" {
			obj.push(try!(AdjustmentTypeParser::parse_xml("AdjustmentType", stack)));
		}
		Ok(obj)
	}
}
/// Write AdjustmentTypes contents to a SignedRequest
struct AdjustmentTypesWriter;
impl AdjustmentTypesWriter {
	fn write_params(params: &mut Params, name: &str, obj: &AdjustmentTypes) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			AdjustmentTypeWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
pub type Alarms = Vec<Alarm>;
/// Parse Alarms from XML
struct AlarmsParser;
impl AlarmsParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<Alarms, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "Alarm" {
			obj.push(try!(AlarmParser::parse_xml("Alarm", stack)));
		}
		Ok(obj)
	}
}
/// Write Alarms contents to a SignedRequest
struct AlarmsWriter;
impl AlarmsWriter {
	fn write_params(params: &mut Params, name: &str, obj: &Alarms) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			AlarmWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
#[derive(Debug, Default)]
pub struct DescribeTagsType {
	/// The maximum number of items to return with this call.
	pub max_records: MaxRecords,
	/// The token for the next set of items to return. (You received this token from a
	/// previous call.)
	pub next_token: XmlString,
	/// A filter used to scope the tags to return.
	pub filters: Filters,
}

/// Parse DescribeTagsType from XML
struct DescribeTagsTypeParser;
impl DescribeTagsTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<DescribeTagsType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = DescribeTagsType::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "MaxRecords" {
				obj.max_records = try!(MaxRecordsParser::parse_xml("MaxRecords", stack));
				continue;
			}
			if current_name == "NextToken" {
				obj.next_token = try!(XmlStringParser::parse_xml("NextToken", stack));
				continue;
			}
			if current_name == "Filter" {
				obj.filters = try!(FiltersParser::parse_xml("Filter", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write DescribeTagsType contents to a SignedRequest
struct DescribeTagsTypeWriter;
impl DescribeTagsTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &DescribeTagsType) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		MaxRecordsWriter::write_params(params, &(prefix.to_string() + "MaxRecords"), &obj.max_records);
		XmlStringWriter::write_params(params, &(prefix.to_string() + "NextToken"), &obj.next_token);
		FiltersWriter::write_params(params, &(prefix.to_string() + "Filter"), &obj.filters);
	}
}
#[derive(Debug, Default)]
pub struct EnterStandbyQuery {
	/// Specifies whether the instances moved to `Standby` mode count as part of the
	/// Auto Scaling group's desired capacity. If set, the desired capacity for the
	/// Auto Scaling group decrements by the number of instances moved to `Standby`
	/// mode.
	pub should_decrement_desired_capacity: ShouldDecrementDesiredCapacity,
	/// The name of the Auto Scaling group.
	pub auto_scaling_group_name: ResourceName,
	/// One or more instances to move into `Standby` mode. You must specify at least
	/// one instance ID.
	pub instance_ids: Option<InstanceIds>,
}

/// Parse EnterStandbyQuery from XML
struct EnterStandbyQueryParser;
impl EnterStandbyQueryParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<EnterStandbyQuery, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = EnterStandbyQuery::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "ShouldDecrementDesiredCapacity" {
				obj.should_decrement_desired_capacity = try!(ShouldDecrementDesiredCapacityParser::parse_xml("ShouldDecrementDesiredCapacity", stack));
				continue;
			}
			if current_name == "AutoScalingGroupName" {
				obj.auto_scaling_group_name = try!(ResourceNameParser::parse_xml("AutoScalingGroupName", stack));
				continue;
			}
			if current_name == "XmlStringMaxLen16" {
				obj.instance_ids = Some(try!(InstanceIdsParser::parse_xml("XmlStringMaxLen16", stack)));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write EnterStandbyQuery contents to a SignedRequest
struct EnterStandbyQueryWriter;
impl EnterStandbyQueryWriter {
	fn write_params(params: &mut Params, name: &str, obj: &EnterStandbyQuery) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		ShouldDecrementDesiredCapacityWriter::write_params(params, &(prefix.to_string() + "ShouldDecrementDesiredCapacity"), &obj.should_decrement_desired_capacity);
		ResourceNameWriter::write_params(params, &(prefix.to_string() + "AutoScalingGroupName"), &obj.auto_scaling_group_name);
		if let Some(ref obj) = obj.instance_ids {
			InstanceIdsWriter::write_params(params, &(prefix.to_string() + "XmlStringMaxLen16"), obj);
		}
	}
}
/// The `NextToken` value is not valid.
#[derive(Debug, Default)]
pub struct InvalidNextToken {
	pub message: XmlStringMaxLen255,
}

/// Parse InvalidNextToken from XML
struct InvalidNextTokenParser;
impl InvalidNextTokenParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<InvalidNextToken, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = InvalidNextToken::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "message" {
				obj.message = try!(XmlStringMaxLen255Parser::parse_xml("message", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write InvalidNextToken contents to a SignedRequest
struct InvalidNextTokenWriter;
impl InvalidNextTokenWriter {
	fn write_params(params: &mut Params, name: &str, obj: &InvalidNextToken) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		XmlStringMaxLen255Writer::write_params(params, &(prefix.to_string() + "message"), &obj.message);
	}
}
#[derive(Debug, Default)]
pub struct DescribeLoadBalancersRequest {
	/// The maximum number of items to return with this call.
	pub max_records: Option<MaxRecords>,
	/// The token for the next set of items to return. (You received this token from a
	/// previous call.)
	pub next_token: Option<XmlString>,
	/// The name of the group.
	pub auto_scaling_group_name: ResourceName,
}

/// Parse DescribeLoadBalancersRequest from XML
struct DescribeLoadBalancersRequestParser;
impl DescribeLoadBalancersRequestParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<DescribeLoadBalancersRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = DescribeLoadBalancersRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "MaxRecords" {
				obj.max_records = Some(try!(MaxRecordsParser::parse_xml("MaxRecords", stack)));
				continue;
			}
			if current_name == "NextToken" {
				obj.next_token = Some(try!(XmlStringParser::parse_xml("NextToken", stack)));
				continue;
			}
			if current_name == "AutoScalingGroupName" {
				obj.auto_scaling_group_name = try!(ResourceNameParser::parse_xml("AutoScalingGroupName", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write DescribeLoadBalancersRequest contents to a SignedRequest
struct DescribeLoadBalancersRequestWriter;
impl DescribeLoadBalancersRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &DescribeLoadBalancersRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		if let Some(ref obj) = obj.max_records {
			MaxRecordsWriter::write_params(params, &(prefix.to_string() + "MaxRecords"), obj);
		}
		if let Some(ref obj) = obj.next_token {
			XmlStringWriter::write_params(params, &(prefix.to_string() + "NextToken"), obj);
		}
		ResourceNameWriter::write_params(params, &(prefix.to_string() + "AutoScalingGroupName"), &obj.auto_scaling_group_name);
	}
}
pub type BlockDeviceEbsDeleteOnTermination = bool;
/// Parse BlockDeviceEbsDeleteOnTermination from XML
struct BlockDeviceEbsDeleteOnTerminationParser;
impl BlockDeviceEbsDeleteOnTerminationParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<BlockDeviceEbsDeleteOnTermination, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = bool::from_str(try!(characters(stack)).as_ref()).unwrap();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write BlockDeviceEbsDeleteOnTermination contents to a SignedRequest
struct BlockDeviceEbsDeleteOnTerminationWriter;
impl BlockDeviceEbsDeleteOnTerminationWriter {
	fn write_params(params: &mut Params, name: &str, obj: &BlockDeviceEbsDeleteOnTermination) {
		params.put(name, &obj.to_string());
	}
}
pub type BlockDeviceEbsIops = i32;
/// Parse BlockDeviceEbsIops from XML
struct BlockDeviceEbsIopsParser;
impl BlockDeviceEbsIopsParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<BlockDeviceEbsIops, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = i32::from_str(try!(characters(stack)).as_ref()).unwrap();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write BlockDeviceEbsIops contents to a SignedRequest
struct BlockDeviceEbsIopsWriter;
impl BlockDeviceEbsIopsWriter {
	fn write_params(params: &mut Params, name: &str, obj: &BlockDeviceEbsIops) {
		params.put(name, &obj.to_string());
	}
}
pub type BlockDeviceEbsVolumeType = String;
/// Parse BlockDeviceEbsVolumeType from XML
struct BlockDeviceEbsVolumeTypeParser;
impl BlockDeviceEbsVolumeTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<BlockDeviceEbsVolumeType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write BlockDeviceEbsVolumeType contents to a SignedRequest
struct BlockDeviceEbsVolumeTypeWriter;
impl BlockDeviceEbsVolumeTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &BlockDeviceEbsVolumeType) {
		params.put(name, obj);
	}
}
#[derive(Debug, Default)]
pub struct RecordLifecycleActionHeartbeatType {
	/// The name of the lifecycle hook.
	pub lifecycle_hook_name: AsciiStringMaxLen255,
	/// The name of the Auto Scaling group for the hook.
	pub auto_scaling_group_name: ResourceName,
	/// A token that uniquely identifies a specific lifecycle action associated with
	/// an instance. Auto Scaling sends this token to the notification target you
	/// specified when you created the lifecycle hook.
	pub lifecycle_action_token: LifecycleActionToken,
}

/// Parse RecordLifecycleActionHeartbeatType from XML
struct RecordLifecycleActionHeartbeatTypeParser;
impl RecordLifecycleActionHeartbeatTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<RecordLifecycleActionHeartbeatType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = RecordLifecycleActionHeartbeatType::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "LifecycleHookName" {
				obj.lifecycle_hook_name = try!(AsciiStringMaxLen255Parser::parse_xml("LifecycleHookName", stack));
				continue;
			}
			if current_name == "AutoScalingGroupName" {
				obj.auto_scaling_group_name = try!(ResourceNameParser::parse_xml("AutoScalingGroupName", stack));
				continue;
			}
			if current_name == "LifecycleActionToken" {
				obj.lifecycle_action_token = try!(LifecycleActionTokenParser::parse_xml("LifecycleActionToken", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write RecordLifecycleActionHeartbeatType contents to a SignedRequest
struct RecordLifecycleActionHeartbeatTypeWriter;
impl RecordLifecycleActionHeartbeatTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &RecordLifecycleActionHeartbeatType) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		AsciiStringMaxLen255Writer::write_params(params, &(prefix.to_string() + "LifecycleHookName"), &obj.lifecycle_hook_name);
		ResourceNameWriter::write_params(params, &(prefix.to_string() + "AutoScalingGroupName"), &obj.auto_scaling_group_name);
		LifecycleActionTokenWriter::write_params(params, &(prefix.to_string() + "LifecycleActionToken"), &obj.lifecycle_action_token);
	}
}
pub type XmlStringMaxLen255 = String;
/// Parse XmlStringMaxLen255 from XML
struct XmlStringMaxLen255Parser;
impl XmlStringMaxLen255Parser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<XmlStringMaxLen255, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write XmlStringMaxLen255 contents to a SignedRequest
struct XmlStringMaxLen255Writer;
impl XmlStringMaxLen255Writer {
	fn write_params(params: &mut Params, name: &str, obj: &XmlStringMaxLen255) {
		params.put(name, obj);
	}
}
pub type ActivityIds = Vec<XmlString>;
/// Parse ActivityIds from XML
struct ActivityIdsParser;
impl ActivityIdsParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ActivityIds, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "XmlString" {
			obj.push(try!(XmlStringParser::parse_xml("XmlString", stack)));
		}
		Ok(obj)
	}
}
/// Write ActivityIds contents to a SignedRequest
struct ActivityIdsWriter;
impl ActivityIdsWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ActivityIds) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			XmlStringWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
#[derive(Debug, Default)]
pub struct DescribeNotificationConfigurationsAnswer {
	/// The token to use when requesting the next set of items. If there are no
	/// additional items to return, the string is empty.
	pub next_token: Option<XmlString>,
	/// The notification configurations.
	pub notification_configurations: NotificationConfigurations,
}

/// Parse DescribeNotificationConfigurationsAnswer from XML
struct DescribeNotificationConfigurationsAnswerParser;
impl DescribeNotificationConfigurationsAnswerParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<DescribeNotificationConfigurationsAnswer, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = DescribeNotificationConfigurationsAnswer::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "NextToken" {
				obj.next_token = Some(try!(XmlStringParser::parse_xml("NextToken", stack)));
				continue;
			}
			if current_name == "NotificationConfiguration" {
				obj.notification_configurations = try!(NotificationConfigurationsParser::parse_xml("NotificationConfiguration", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write DescribeNotificationConfigurationsAnswer contents to a SignedRequest
struct DescribeNotificationConfigurationsAnswerWriter;
impl DescribeNotificationConfigurationsAnswerWriter {
	fn write_params(params: &mut Params, name: &str, obj: &DescribeNotificationConfigurationsAnswer) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		if let Some(ref obj) = obj.next_token {
			XmlStringWriter::write_params(params, &(prefix.to_string() + "NextToken"), obj);
		}
		NotificationConfigurationsWriter::write_params(params, &(prefix.to_string() + "NotificationConfiguration"), &obj.notification_configurations);
	}
}
pub type Cooldown = i32;
/// Parse Cooldown from XML
struct CooldownParser;
impl CooldownParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<Cooldown, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = i32::from_str(try!(characters(stack)).as_ref()).unwrap();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write Cooldown contents to a SignedRequest
struct CooldownWriter;
impl CooldownWriter {
	fn write_params(params: &mut Params, name: &str, obj: &Cooldown) {
		params.put(name, &obj.to_string());
	}
}
pub type MetricCollectionTypes = Vec<MetricCollectionType>;
/// Parse MetricCollectionTypes from XML
struct MetricCollectionTypesParser;
impl MetricCollectionTypesParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<MetricCollectionTypes, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "MetricCollectionType" {
			obj.push(try!(MetricCollectionTypeParser::parse_xml("MetricCollectionType", stack)));
		}
		Ok(obj)
	}
}
/// Write MetricCollectionTypes contents to a SignedRequest
struct MetricCollectionTypesWriter;
impl MetricCollectionTypesWriter {
	fn write_params(params: &mut Params, name: &str, obj: &MetricCollectionTypes) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			MetricCollectionTypeWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
#[derive(Debug, Default)]
pub struct CreateAutoScalingGroupType {
	/// The amount of time, in seconds, after an EC2 instance comes into service that
	/// Auto Scaling starts checking its health. During this time, any health check
	/// failures for the instance are ignored.
	/// This parameter is required if you are adding an `ELB` health check.
	/// Frequently, new instances need to warm up, briefly, before they can pass a
	/// health check. To provide ample warm-up time, set the health check grace period
	/// of the group to match the expected startup period of your application.
	/// For more information, see [Add an Elastic Load Balancing Health Check to Your
	/// Auto Scaling
	/// Group](http://docs.aws.amazon.com/AutoScaling/latest/DeveloperGuide/as-add-
	/// elb-healthcheck.html) in the _Auto Scaling Developer Guide_.
	pub health_check_grace_period: Option<HealthCheckGracePeriod>,
	/// The name of the placement group into which you'll launch your instances, if
	/// any. For more information, see [Placement
	/// Groups](http://docs.aws.amazon.com/AWSEC2/latest/UserGuide/placement-
	/// groups.html) in the _Amazon Elastic Compute Cloud User Guide_.
	pub placement_group: Option<XmlStringMaxLen255>,
	/// The number of EC2 instances that should be running in the group. This number
	/// must be greater than or equal to the minimum size of the group and less than
	/// or equal to the maximum size of the group.
	pub desired_capacity: Option<AutoScalingGroupDesiredCapacity>,
	/// The tag to be created or updated. Each tag should be defined by its resource
	/// type, resource ID, key, value, and a propagate flag. Valid values:
	/// key=_value_, value=_value_, propagate=_true_ or _false_. Value and propagate
	/// are optional parameters.
	/// For more information, see [Tagging Auto Scaling Groups and Instances](http://d
	/// ocs.aws.amazon.com/AutoScaling/latest/DeveloperGuide/ASTagging.html) in the
	/// _Auto Scaling Developer Guide_.
	pub tags: Option<Tags>,
	/// The ID of the EC2 instance used to create a launch configuration for the
	/// group. Alternatively, use the `LaunchConfigurationName` parameter to specify a
	/// launch configuration instead of an EC2 instance.
	/// When you specify an ID of an instance, Auto Scaling creates a new launch
	/// configuration and associates it with the group. This launch configuration
	/// derives its attributes from the specified instance, with the exception of the
	/// block device mapping.
	/// For more information, see [Create an Auto Scaling Group from an EC2
	/// Instance](http://docs.aws.amazon.com/AutoScaling/latest/DeveloperGuide/create-
	/// asg-from-instance.html) in the _Auto Scaling Developer Guide_.
	pub instance_id: Option<XmlStringMaxLen16>,
	/// One or more load balancers.
	/// For more information, see [Load Balance Your Auto Scaling Group](http://docs.a
	/// ws.amazon.com/AutoScaling/latest/DeveloperGuide/US_SetUpASLBApp.html) in the
	/// _Auto Scaling Developer Guide_.
	pub load_balancer_names: Option<LoadBalancerNames>,
	/// The name of the group. This name must be unique within the scope of your AWS
	/// account.
	pub auto_scaling_group_name: XmlStringMaxLen255,
	/// The amount of time, in seconds, after a scaling activity completes before
	/// another scaling activity can start.
	/// If this parameter is not specified, the default value is 300. For more
	/// information, see [Understanding Auto Scaling Cooldowns](http://docs.aws.amazon
	/// .com/AutoScaling/latest/DeveloperGuide/Cooldown.html) in the _Auto Scaling
	/// Developer Guide_.
	pub default_cooldown: Option<Cooldown>,
	/// The minimum size of the group.
	pub min_size: AutoScalingGroupMinSize,
	/// The maximum size of the group.
	pub max_size: AutoScalingGroupMaxSize,
	/// A comma-separated list of subnet identifiers for your virtual private cloud
	/// (VPC).
	/// If you specify subnets and Availability Zones with this call, ensure that the
	/// subnets' Availability Zones match the Availability Zones specified.
	/// For more information, see [Auto Scaling and Amazon Virtual Private Cloud](http
	/// ://docs.aws.amazon.com/AutoScaling/latest/DeveloperGuide/autoscalingsubnets.ht
	/// ml) in the _Auto Scaling Developer Guide_.
	pub vpc_zone_identifier: Option<XmlStringMaxLen255>,
	/// One or more termination policies used to select the instance to terminate.
	/// These policies are executed in the order that they are listed.
	/// For more information, see [Choosing a Termination Policy for Your Auto Scaling
	/// Group](http://docs.aws.amazon.com/AutoScaling/latest/DeveloperGuide/us-
	/// termination-policy.html) in the _Auto Scaling Developer Guide_.
	pub termination_policies: Option<TerminationPolicies>,
	/// The name of the launch configuration. Alternatively, use the `InstanceId`
	/// parameter to specify an EC2 instance instead of a launch configuration.
	pub launch_configuration_name: Option<ResourceName>,
	/// One or more Availability Zones for the group. This parameter is optional if
	/// you specify subnets using the `VPCZoneIdentifier` parameter.
	pub availability_zones: Option<AvailabilityZones>,
	/// The service to use for the health checks. The valid values are `EC2` and
	/// `ELB`.
	/// By default, health checks use Amazon EC2 instance status checks to determine
	/// the health of an instance. For more information, see [Health Checks](http://do
	/// cs.aws.amazon.com/AutoScaling/latest/DeveloperGuide/healthcheck.html).
	pub health_check_type: Option<XmlStringMaxLen32>,
}

/// Parse CreateAutoScalingGroupType from XML
struct CreateAutoScalingGroupTypeParser;
impl CreateAutoScalingGroupTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<CreateAutoScalingGroupType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = CreateAutoScalingGroupType::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "HealthCheckGracePeriod" {
				obj.health_check_grace_period = Some(try!(HealthCheckGracePeriodParser::parse_xml("HealthCheckGracePeriod", stack)));
				continue;
			}
			if current_name == "PlacementGroup" {
				obj.placement_group = Some(try!(XmlStringMaxLen255Parser::parse_xml("PlacementGroup", stack)));
				continue;
			}
			if current_name == "DesiredCapacity" {
				obj.desired_capacity = Some(try!(AutoScalingGroupDesiredCapacityParser::parse_xml("DesiredCapacity", stack)));
				continue;
			}
			if current_name == "Tag" {
				obj.tags = Some(try!(TagsParser::parse_xml("Tag", stack)));
				continue;
			}
			if current_name == "InstanceId" {
				obj.instance_id = Some(try!(XmlStringMaxLen16Parser::parse_xml("InstanceId", stack)));
				continue;
			}
			if current_name == "XmlStringMaxLen255" {
				obj.load_balancer_names = Some(try!(LoadBalancerNamesParser::parse_xml("XmlStringMaxLen255", stack)));
				continue;
			}
			if current_name == "AutoScalingGroupName" {
				obj.auto_scaling_group_name = try!(XmlStringMaxLen255Parser::parse_xml("AutoScalingGroupName", stack));
				continue;
			}
			if current_name == "DefaultCooldown" {
				obj.default_cooldown = Some(try!(CooldownParser::parse_xml("DefaultCooldown", stack)));
				continue;
			}
			if current_name == "MinSize" {
				obj.min_size = try!(AutoScalingGroupMinSizeParser::parse_xml("MinSize", stack));
				continue;
			}
			if current_name == "MaxSize" {
				obj.max_size = try!(AutoScalingGroupMaxSizeParser::parse_xml("MaxSize", stack));
				continue;
			}
			if current_name == "VPCZoneIdentifier" {
				obj.vpc_zone_identifier = Some(try!(XmlStringMaxLen255Parser::parse_xml("VPCZoneIdentifier", stack)));
				continue;
			}
			if current_name == "XmlStringMaxLen1600" {
				obj.termination_policies = Some(try!(TerminationPoliciesParser::parse_xml("XmlStringMaxLen1600", stack)));
				continue;
			}
			if current_name == "LaunchConfigurationName" {
				obj.launch_configuration_name = Some(try!(ResourceNameParser::parse_xml("LaunchConfigurationName", stack)));
				continue;
			}
			if current_name == "XmlStringMaxLen255" {
				obj.availability_zones = Some(try!(AvailabilityZonesParser::parse_xml("XmlStringMaxLen255", stack)));
				continue;
			}
			if current_name == "HealthCheckType" {
				obj.health_check_type = Some(try!(XmlStringMaxLen32Parser::parse_xml("HealthCheckType", stack)));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write CreateAutoScalingGroupType contents to a SignedRequest
struct CreateAutoScalingGroupTypeWriter;
impl CreateAutoScalingGroupTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &CreateAutoScalingGroupType) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		if let Some(ref obj) = obj.health_check_grace_period {
			HealthCheckGracePeriodWriter::write_params(params, &(prefix.to_string() + "HealthCheckGracePeriod"), obj);
		}
		if let Some(ref obj) = obj.placement_group {
			XmlStringMaxLen255Writer::write_params(params, &(prefix.to_string() + "PlacementGroup"), obj);
		}
		if let Some(ref obj) = obj.desired_capacity {
			AutoScalingGroupDesiredCapacityWriter::write_params(params, &(prefix.to_string() + "DesiredCapacity"), obj);
		}
		if let Some(ref obj) = obj.tags {
			TagsWriter::write_params(params, &(prefix.to_string() + "Tag"), obj);
		}
		if let Some(ref obj) = obj.instance_id {
			XmlStringMaxLen16Writer::write_params(params, &(prefix.to_string() + "InstanceId"), obj);
		}
		if let Some(ref obj) = obj.load_balancer_names {
			LoadBalancerNamesWriter::write_params(params, &(prefix.to_string() + "XmlStringMaxLen255"), obj);
		}
		XmlStringMaxLen255Writer::write_params(params, &(prefix.to_string() + "AutoScalingGroupName"), &obj.auto_scaling_group_name);
		if let Some(ref obj) = obj.default_cooldown {
			CooldownWriter::write_params(params, &(prefix.to_string() + "DefaultCooldown"), obj);
		}
		AutoScalingGroupMinSizeWriter::write_params(params, &(prefix.to_string() + "MinSize"), &obj.min_size);
		AutoScalingGroupMaxSizeWriter::write_params(params, &(prefix.to_string() + "MaxSize"), &obj.max_size);
		if let Some(ref obj) = obj.vpc_zone_identifier {
			XmlStringMaxLen255Writer::write_params(params, &(prefix.to_string() + "VPCZoneIdentifier"), obj);
		}
		if let Some(ref obj) = obj.termination_policies {
			TerminationPoliciesWriter::write_params(params, &(prefix.to_string() + "XmlStringMaxLen1600"), obj);
		}
		if let Some(ref obj) = obj.launch_configuration_name {
			ResourceNameWriter::write_params(params, &(prefix.to_string() + "LaunchConfigurationName"), obj);
		}
		if let Some(ref obj) = obj.availability_zones {
			AvailabilityZonesWriter::write_params(params, &(prefix.to_string() + "XmlStringMaxLen255"), obj);
		}
		if let Some(ref obj) = obj.health_check_type {
			XmlStringMaxLen32Writer::write_params(params, &(prefix.to_string() + "HealthCheckType"), obj);
		}
	}
}
#[derive(Debug, Default)]
pub struct CompleteLifecycleActionType {
	/// The action for the group to take. This parameter can be either `CONTINUE` or
	/// `ABANDON`.
	pub lifecycle_action_result: LifecycleActionResult,
	/// The name of the lifecycle hook.
	pub lifecycle_hook_name: AsciiStringMaxLen255,
	/// The name of the group for the lifecycle hook.
	pub auto_scaling_group_name: ResourceName,
	/// A universally unique identifier (UUID) that identifies a specific lifecycle
	/// action associated with an instance. Auto Scaling sends this token to the
	/// notification target you specified when you created the lifecycle hook.
	pub lifecycle_action_token: LifecycleActionToken,
}

/// Parse CompleteLifecycleActionType from XML
struct CompleteLifecycleActionTypeParser;
impl CompleteLifecycleActionTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<CompleteLifecycleActionType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = CompleteLifecycleActionType::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "LifecycleActionResult" {
				obj.lifecycle_action_result = try!(LifecycleActionResultParser::parse_xml("LifecycleActionResult", stack));
				continue;
			}
			if current_name == "LifecycleHookName" {
				obj.lifecycle_hook_name = try!(AsciiStringMaxLen255Parser::parse_xml("LifecycleHookName", stack));
				continue;
			}
			if current_name == "AutoScalingGroupName" {
				obj.auto_scaling_group_name = try!(ResourceNameParser::parse_xml("AutoScalingGroupName", stack));
				continue;
			}
			if current_name == "LifecycleActionToken" {
				obj.lifecycle_action_token = try!(LifecycleActionTokenParser::parse_xml("LifecycleActionToken", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write CompleteLifecycleActionType contents to a SignedRequest
struct CompleteLifecycleActionTypeWriter;
impl CompleteLifecycleActionTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &CompleteLifecycleActionType) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		LifecycleActionResultWriter::write_params(params, &(prefix.to_string() + "LifecycleActionResult"), &obj.lifecycle_action_result);
		AsciiStringMaxLen255Writer::write_params(params, &(prefix.to_string() + "LifecycleHookName"), &obj.lifecycle_hook_name);
		ResourceNameWriter::write_params(params, &(prefix.to_string() + "AutoScalingGroupName"), &obj.auto_scaling_group_name);
		LifecycleActionTokenWriter::write_params(params, &(prefix.to_string() + "LifecycleActionToken"), &obj.lifecycle_action_token);
	}
}
#[derive(Debug, Default)]
pub struct DescribePoliciesType {
	/// One or more policy names or policy ARNs to be described. If you omit this
	/// list, all policy names are described. If an group name is provided, the
	/// results are limited to that group. This list is limited to 50 items. If you
	/// specify an unknown policy name, it is ignored with no error.
	pub policy_names: PolicyNames,
	/// One or more policy types. Valid values are `SimpleScaling` and `StepScaling`.
	pub policy_types: PolicyTypes,
	/// The token for the next set of items to return. (You received this token from a
	/// previous call.)
	pub next_token: XmlString,
	/// The name of the group.
	pub auto_scaling_group_name: ResourceName,
	/// The maximum number of items to be returned with each call.
	pub max_records: MaxRecords,
}

/// Parse DescribePoliciesType from XML
struct DescribePoliciesTypeParser;
impl DescribePoliciesTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<DescribePoliciesType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = DescribePoliciesType::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "ResourceName" {
				obj.policy_names = try!(PolicyNamesParser::parse_xml("ResourceName", stack));
				continue;
			}
			if current_name == "XmlStringMaxLen64" {
				obj.policy_types = try!(PolicyTypesParser::parse_xml("XmlStringMaxLen64", stack));
				continue;
			}
			if current_name == "NextToken" {
				obj.next_token = try!(XmlStringParser::parse_xml("NextToken", stack));
				continue;
			}
			if current_name == "AutoScalingGroupName" {
				obj.auto_scaling_group_name = try!(ResourceNameParser::parse_xml("AutoScalingGroupName", stack));
				continue;
			}
			if current_name == "MaxRecords" {
				obj.max_records = try!(MaxRecordsParser::parse_xml("MaxRecords", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write DescribePoliciesType contents to a SignedRequest
struct DescribePoliciesTypeWriter;
impl DescribePoliciesTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &DescribePoliciesType) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		PolicyNamesWriter::write_params(params, &(prefix.to_string() + "ResourceName"), &obj.policy_names);
		PolicyTypesWriter::write_params(params, &(prefix.to_string() + "XmlStringMaxLen64"), &obj.policy_types);
		XmlStringWriter::write_params(params, &(prefix.to_string() + "NextToken"), &obj.next_token);
		ResourceNameWriter::write_params(params, &(prefix.to_string() + "AutoScalingGroupName"), &obj.auto_scaling_group_name);
		MaxRecordsWriter::write_params(params, &(prefix.to_string() + "MaxRecords"), &obj.max_records);
	}
}
#[derive(Debug, Default)]
pub struct DescribeLifecycleHooksAnswer {
	/// The lifecycle hooks for the specified group.
	pub lifecycle_hooks: LifecycleHooks,
}

/// Parse DescribeLifecycleHooksAnswer from XML
struct DescribeLifecycleHooksAnswerParser;
impl DescribeLifecycleHooksAnswerParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<DescribeLifecycleHooksAnswer, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = DescribeLifecycleHooksAnswer::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "LifecycleHook" {
				obj.lifecycle_hooks = try!(LifecycleHooksParser::parse_xml("LifecycleHook", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write DescribeLifecycleHooksAnswer contents to a SignedRequest
struct DescribeLifecycleHooksAnswerWriter;
impl DescribeLifecycleHooksAnswerWriter {
	fn write_params(params: &mut Params, name: &str, obj: &DescribeLifecycleHooksAnswer) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		LifecycleHooksWriter::write_params(params, &(prefix.to_string() + "LifecycleHook"), &obj.lifecycle_hooks);
	}
}
pub type PolicyTypes = Vec<XmlStringMaxLen64>;
/// Parse PolicyTypes from XML
struct PolicyTypesParser;
impl PolicyTypesParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<PolicyTypes, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "XmlStringMaxLen64" {
			obj.push(try!(XmlStringMaxLen64Parser::parse_xml("XmlStringMaxLen64", stack)));
		}
		Ok(obj)
	}
}
/// Write PolicyTypes contents to a SignedRequest
struct PolicyTypesWriter;
impl PolicyTypesWriter {
	fn write_params(params: &mut Params, name: &str, obj: &PolicyTypes) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			XmlStringMaxLen64Writer::write_params(params, key, &element);
			index += 1;
		}
	}
}
#[derive(Debug, Default)]
pub struct DetachLoadBalancersResultType;

/// Parse DetachLoadBalancersResultType from XML
struct DetachLoadBalancersResultTypeParser;
impl DetachLoadBalancersResultTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<DetachLoadBalancersResultType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = DetachLoadBalancersResultType::default();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write DetachLoadBalancersResultType contents to a SignedRequest
struct DetachLoadBalancersResultTypeWriter;
impl DetachLoadBalancersResultTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &DetachLoadBalancersResultType) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
	}
}
/// Describes an EC2 instance associated with an Auto Scaling group.
#[derive(Debug, Default)]
pub struct AutoScalingInstanceDetails {
	/// The Availability Zone for the instance.
	pub availability_zone: XmlStringMaxLen255,
	/// The ID of the instance.
	pub instance_id: XmlStringMaxLen16,
	/// The name of the Auto Scaling group associated with the instance.
	pub auto_scaling_group_name: XmlStringMaxLen255,
	/// The health status of this instance. "Healthy" means that the instance is
	/// healthy and should remain in service. "Unhealthy" means that the instance is
	/// unhealthy and Auto Scaling should terminate and replace it.
	pub health_status: XmlStringMaxLen32,
	/// The lifecycle state for the instance. For more information, see [Auto Scaling
	/// Instance States](http://docs.aws.amazon.com/AutoScaling/latest/DeveloperGuide/
	/// AutoScalingGroupLifecycle.html#AutoScalingStates) in the _Auto Scaling
	/// Developer Guide_.
	pub lifecycle_state: XmlStringMaxLen32,
	/// The launch configuration associated with the instance.
	pub launch_configuration_name: XmlStringMaxLen255,
}

/// Parse AutoScalingInstanceDetails from XML
struct AutoScalingInstanceDetailsParser;
impl AutoScalingInstanceDetailsParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<AutoScalingInstanceDetails, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = AutoScalingInstanceDetails::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "AvailabilityZone" {
				obj.availability_zone = try!(XmlStringMaxLen255Parser::parse_xml("AvailabilityZone", stack));
				continue;
			}
			if current_name == "InstanceId" {
				obj.instance_id = try!(XmlStringMaxLen16Parser::parse_xml("InstanceId", stack));
				continue;
			}
			if current_name == "AutoScalingGroupName" {
				obj.auto_scaling_group_name = try!(XmlStringMaxLen255Parser::parse_xml("AutoScalingGroupName", stack));
				continue;
			}
			if current_name == "HealthStatus" {
				obj.health_status = try!(XmlStringMaxLen32Parser::parse_xml("HealthStatus", stack));
				continue;
			}
			if current_name == "LifecycleState" {
				obj.lifecycle_state = try!(XmlStringMaxLen32Parser::parse_xml("LifecycleState", stack));
				continue;
			}
			if current_name == "LaunchConfigurationName" {
				obj.launch_configuration_name = try!(XmlStringMaxLen255Parser::parse_xml("LaunchConfigurationName", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write AutoScalingInstanceDetails contents to a SignedRequest
struct AutoScalingInstanceDetailsWriter;
impl AutoScalingInstanceDetailsWriter {
	fn write_params(params: &mut Params, name: &str, obj: &AutoScalingInstanceDetails) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		XmlStringMaxLen255Writer::write_params(params, &(prefix.to_string() + "AvailabilityZone"), &obj.availability_zone);
		XmlStringMaxLen16Writer::write_params(params, &(prefix.to_string() + "InstanceId"), &obj.instance_id);
		XmlStringMaxLen255Writer::write_params(params, &(prefix.to_string() + "AutoScalingGroupName"), &obj.auto_scaling_group_name);
		XmlStringMaxLen32Writer::write_params(params, &(prefix.to_string() + "HealthStatus"), &obj.health_status);
		XmlStringMaxLen32Writer::write_params(params, &(prefix.to_string() + "LifecycleState"), &obj.lifecycle_state);
		XmlStringMaxLen255Writer::write_params(params, &(prefix.to_string() + "LaunchConfigurationName"), &obj.launch_configuration_name);
	}
}
pub type PolicyIncrement = i32;
/// Parse PolicyIncrement from XML
struct PolicyIncrementParser;
impl PolicyIncrementParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<PolicyIncrement, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = i32::from_str(try!(characters(stack)).as_ref()).unwrap();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write PolicyIncrement contents to a SignedRequest
struct PolicyIncrementWriter;
impl PolicyIncrementWriter {
	fn write_params(params: &mut Params, name: &str, obj: &PolicyIncrement) {
		params.put(name, &obj.to_string());
	}
}
#[derive(Debug, Default)]
pub struct DeleteAutoScalingGroupType {
	/// Specifies that the group will be deleted along with all instances associated
	/// with the group, without waiting for all instances to be terminated. This
	/// parameter also deletes any lifecycle actions associated with the group.
	pub force_delete: Option<ForceDelete>,
	/// The name of the group to delete.
	pub auto_scaling_group_name: ResourceName,
}

/// Parse DeleteAutoScalingGroupType from XML
struct DeleteAutoScalingGroupTypeParser;
impl DeleteAutoScalingGroupTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<DeleteAutoScalingGroupType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = DeleteAutoScalingGroupType::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "ForceDelete" {
				obj.force_delete = Some(try!(ForceDeleteParser::parse_xml("ForceDelete", stack)));
				continue;
			}
			if current_name == "AutoScalingGroupName" {
				obj.auto_scaling_group_name = try!(ResourceNameParser::parse_xml("AutoScalingGroupName", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write DeleteAutoScalingGroupType contents to a SignedRequest
struct DeleteAutoScalingGroupTypeWriter;
impl DeleteAutoScalingGroupTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &DeleteAutoScalingGroupType) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		if let Some(ref obj) = obj.force_delete {
			ForceDeleteWriter::write_params(params, &(prefix.to_string() + "ForceDelete"), obj);
		}
		ResourceNameWriter::write_params(params, &(prefix.to_string() + "AutoScalingGroupName"), &obj.auto_scaling_group_name);
	}
}
pub type MonitoringEnabled = bool;
/// Parse MonitoringEnabled from XML
struct MonitoringEnabledParser;
impl MonitoringEnabledParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<MonitoringEnabled, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = bool::from_str(try!(characters(stack)).as_ref()).unwrap();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write MonitoringEnabled contents to a SignedRequest
struct MonitoringEnabledWriter;
impl MonitoringEnabledWriter {
	fn write_params(params: &mut Params, name: &str, obj: &MonitoringEnabled) {
		params.put(name, &obj.to_string());
	}
}
#[derive(Debug, Default)]
pub struct DescribeLifecycleHooksType {
	/// The names of one or more lifecycle hooks.
	pub lifecycle_hook_names: Option<LifecycleHookNames>,
	/// The name of the group.
	pub auto_scaling_group_name: ResourceName,
}

/// Parse DescribeLifecycleHooksType from XML
struct DescribeLifecycleHooksTypeParser;
impl DescribeLifecycleHooksTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<DescribeLifecycleHooksType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = DescribeLifecycleHooksType::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "AsciiStringMaxLen255" {
				obj.lifecycle_hook_names = Some(try!(LifecycleHookNamesParser::parse_xml("AsciiStringMaxLen255", stack)));
				continue;
			}
			if current_name == "AutoScalingGroupName" {
				obj.auto_scaling_group_name = try!(ResourceNameParser::parse_xml("AutoScalingGroupName", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write DescribeLifecycleHooksType contents to a SignedRequest
struct DescribeLifecycleHooksTypeWriter;
impl DescribeLifecycleHooksTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &DescribeLifecycleHooksType) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		if let Some(ref obj) = obj.lifecycle_hook_names {
			LifecycleHookNamesWriter::write_params(params, &(prefix.to_string() + "AsciiStringMaxLen255"), obj);
		}
		ResourceNameWriter::write_params(params, &(prefix.to_string() + "AutoScalingGroupName"), &obj.auto_scaling_group_name);
	}
}
#[derive(Debug, Default)]
pub struct DescribeAutoScalingNotificationTypesAnswer {
	/// One or more of the following notification types:
	///   * `autoscaling:EC2_INSTANCE_LAUNCH`
	///   * `autoscaling:EC2_INSTANCE_LAUNCH_ERROR`
	///   * `autoscaling:EC2_INSTANCE_TERMINATE`
	///   * `autoscaling:EC2_INSTANCE_TERMINATE_ERROR`
	///   * `autoscaling:TEST_NOTIFICATION`
	pub auto_scaling_notification_types: AutoScalingNotificationTypes,
}

/// Parse DescribeAutoScalingNotificationTypesAnswer from XML
struct DescribeAutoScalingNotificationTypesAnswerParser;
impl DescribeAutoScalingNotificationTypesAnswerParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<DescribeAutoScalingNotificationTypesAnswer, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = DescribeAutoScalingNotificationTypesAnswer::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "XmlStringMaxLen255" {
				obj.auto_scaling_notification_types = try!(AutoScalingNotificationTypesParser::parse_xml("XmlStringMaxLen255", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write DescribeAutoScalingNotificationTypesAnswer contents to a SignedRequest
struct DescribeAutoScalingNotificationTypesAnswerWriter;
impl DescribeAutoScalingNotificationTypesAnswerWriter {
	fn write_params(params: &mut Params, name: &str, obj: &DescribeAutoScalingNotificationTypesAnswer) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		AutoScalingNotificationTypesWriter::write_params(params, &(prefix.to_string() + "XmlStringMaxLen255"), &obj.auto_scaling_notification_types);
	}
}
pub type Activities = Vec<Activity>;
/// Parse Activities from XML
struct ActivitiesParser;
impl ActivitiesParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<Activities, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "Activity" {
			obj.push(try!(ActivityParser::parse_xml("Activity", stack)));
		}
		Ok(obj)
	}
}
/// Write Activities contents to a SignedRequest
struct ActivitiesWriter;
impl ActivitiesWriter {
	fn write_params(params: &mut Params, name: &str, obj: &Activities) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			ActivityWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
#[derive(Debug, Default)]
pub struct CompleteLifecycleActionAnswer;

/// Parse CompleteLifecycleActionAnswer from XML
struct CompleteLifecycleActionAnswerParser;
impl CompleteLifecycleActionAnswerParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<CompleteLifecycleActionAnswer, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = CompleteLifecycleActionAnswer::default();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write CompleteLifecycleActionAnswer contents to a SignedRequest
struct CompleteLifecycleActionAnswerWriter;
impl CompleteLifecycleActionAnswerWriter {
	fn write_params(params: &mut Params, name: &str, obj: &CompleteLifecycleActionAnswer) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
	}
}
/// Describes a lifecycle hook, which tells Auto Scaling that you want to perform
/// an action when an instance launches or terminates. When you have a lifecycle
/// hook in place, the Auto Scaling group will either:
///   * Pause the instance after it launches, but before it is put into service
///   * Pause the instance as it terminates, but before it is fully terminated
/// For more information, see [Auto Scaling Pending State](http://docs.aws.amazon.
/// com/AutoScaling/latest/DeveloperGuide/AutoScalingPendingState.html) and [Auto
/// Scaling Terminating State](http://docs.aws.amazon.com/AutoScaling/latest/Devel
/// operGuide/AutoScalingTerminatingState.html) in the _Auto Scaling Developer
/// Guide_.
#[derive(Debug, Default)]
pub struct LifecycleHook {
	/// The maximum length of time an instance can remain in a `Pending:Wait` or
	/// `Terminating:Wait` state. Currently, the maximum is set to 48 hours.
	pub global_timeout: GlobalTimeout,
	/// The amount of time that can elapse before the lifecycle hook times out. When
	/// the lifecycle hook times out, Auto Scaling performs the action defined in the
	/// `DefaultResult` parameter. You can prevent the lifecycle hook from timing out
	/// by calling RecordLifecycleActionHeartbeat.
	pub heartbeat_timeout: HeartbeatTimeout,
	/// The ARN of the IAM role that allows the Auto Scaling group to publish to the
	/// specified notification target.
	pub role_arn: ResourceName,
	/// The name of the Auto Scaling group for the lifecycle hook.
	pub auto_scaling_group_name: ResourceName,
	/// The name of the lifecycle hook.
	pub lifecycle_hook_name: AsciiStringMaxLen255,
	/// Additional information that you want to include any time Auto Scaling sends a
	/// message to the notification target.
	pub notification_metadata: XmlStringMaxLen1023,
	/// Defines the action the Auto Scaling group should take when the lifecycle hook
	/// timeout elapses or if an unexpected failure occurs. The valid values are
	/// `CONTINUE` and `ABANDON`. The default value is `CONTINUE`.
	pub default_result: LifecycleActionResult,
	/// The ARN of the notification target that Auto Scaling uses to notify you when
	/// an instance is in the transition state for the lifecycle hook. This ARN target
	/// can be either an SQS queue or an SNS topic. The notification message sent to
	/// the target includes the following:
	///   * Lifecycle action token
	///   * User account ID
	///   * Name of the Auto Scaling group
	///   * Lifecycle hook name
	///   * EC2 instance ID
	///   * Lifecycle transition
	///   * Notification metadata
	pub notification_target_arn: ResourceName,
	/// The state of the EC2 instance to which you want to attach the lifecycle hook.
	/// For a list of lifecycle hook types, see DescribeLifecycleHookTypes.
	pub lifecycle_transition: LifecycleTransition,
}

/// Parse LifecycleHook from XML
struct LifecycleHookParser;
impl LifecycleHookParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<LifecycleHook, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = LifecycleHook::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "GlobalTimeout" {
				obj.global_timeout = try!(GlobalTimeoutParser::parse_xml("GlobalTimeout", stack));
				continue;
			}
			if current_name == "HeartbeatTimeout" {
				obj.heartbeat_timeout = try!(HeartbeatTimeoutParser::parse_xml("HeartbeatTimeout", stack));
				continue;
			}
			if current_name == "RoleARN" {
				obj.role_arn = try!(ResourceNameParser::parse_xml("RoleARN", stack));
				continue;
			}
			if current_name == "AutoScalingGroupName" {
				obj.auto_scaling_group_name = try!(ResourceNameParser::parse_xml("AutoScalingGroupName", stack));
				continue;
			}
			if current_name == "LifecycleHookName" {
				obj.lifecycle_hook_name = try!(AsciiStringMaxLen255Parser::parse_xml("LifecycleHookName", stack));
				continue;
			}
			if current_name == "NotificationMetadata" {
				obj.notification_metadata = try!(XmlStringMaxLen1023Parser::parse_xml("NotificationMetadata", stack));
				continue;
			}
			if current_name == "DefaultResult" {
				obj.default_result = try!(LifecycleActionResultParser::parse_xml("DefaultResult", stack));
				continue;
			}
			if current_name == "NotificationTargetARN" {
				obj.notification_target_arn = try!(ResourceNameParser::parse_xml("NotificationTargetARN", stack));
				continue;
			}
			if current_name == "LifecycleTransition" {
				obj.lifecycle_transition = try!(LifecycleTransitionParser::parse_xml("LifecycleTransition", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write LifecycleHook contents to a SignedRequest
struct LifecycleHookWriter;
impl LifecycleHookWriter {
	fn write_params(params: &mut Params, name: &str, obj: &LifecycleHook) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		GlobalTimeoutWriter::write_params(params, &(prefix.to_string() + "GlobalTimeout"), &obj.global_timeout);
		HeartbeatTimeoutWriter::write_params(params, &(prefix.to_string() + "HeartbeatTimeout"), &obj.heartbeat_timeout);
		ResourceNameWriter::write_params(params, &(prefix.to_string() + "RoleARN"), &obj.role_arn);
		ResourceNameWriter::write_params(params, &(prefix.to_string() + "AutoScalingGroupName"), &obj.auto_scaling_group_name);
		AsciiStringMaxLen255Writer::write_params(params, &(prefix.to_string() + "LifecycleHookName"), &obj.lifecycle_hook_name);
		XmlStringMaxLen1023Writer::write_params(params, &(prefix.to_string() + "NotificationMetadata"), &obj.notification_metadata);
		LifecycleActionResultWriter::write_params(params, &(prefix.to_string() + "DefaultResult"), &obj.default_result);
		ResourceNameWriter::write_params(params, &(prefix.to_string() + "NotificationTargetARN"), &obj.notification_target_arn);
		LifecycleTransitionWriter::write_params(params, &(prefix.to_string() + "LifecycleTransition"), &obj.lifecycle_transition);
	}
}
#[derive(Debug, Default)]
pub struct DescribeLifecycleHookTypesAnswer {
	/// One or more of the following notification types:
	///   * `autoscaling:EC2_INSTANCE_LAUNCHING`
	///   * `autoscaling:EC2_INSTANCE_TERMINATING`
	pub lifecycle_hook_types: AutoScalingNotificationTypes,
}

/// Parse DescribeLifecycleHookTypesAnswer from XML
struct DescribeLifecycleHookTypesAnswerParser;
impl DescribeLifecycleHookTypesAnswerParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<DescribeLifecycleHookTypesAnswer, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = DescribeLifecycleHookTypesAnswer::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "XmlStringMaxLen255" {
				obj.lifecycle_hook_types = try!(AutoScalingNotificationTypesParser::parse_xml("XmlStringMaxLen255", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write DescribeLifecycleHookTypesAnswer contents to a SignedRequest
struct DescribeLifecycleHookTypesAnswerWriter;
impl DescribeLifecycleHookTypesAnswerWriter {
	fn write_params(params: &mut Params, name: &str, obj: &DescribeLifecycleHookTypesAnswer) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		AutoScalingNotificationTypesWriter::write_params(params, &(prefix.to_string() + "XmlStringMaxLen255"), &obj.lifecycle_hook_types);
	}
}
#[derive(Debug, Default)]
pub struct DescribeTerminationPolicyTypesAnswer {
	/// The termination policies supported by Auto Scaling (`OldestInstance`,
	/// `OldestLaunchConfiguration`, `NewestInstance`, `ClosestToNextInstanceHour`,
	/// and `Default`).
	pub termination_policy_types: TerminationPolicies,
}

/// Parse DescribeTerminationPolicyTypesAnswer from XML
struct DescribeTerminationPolicyTypesAnswerParser;
impl DescribeTerminationPolicyTypesAnswerParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<DescribeTerminationPolicyTypesAnswer, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = DescribeTerminationPolicyTypesAnswer::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "XmlStringMaxLen1600" {
				obj.termination_policy_types = try!(TerminationPoliciesParser::parse_xml("XmlStringMaxLen1600", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write DescribeTerminationPolicyTypesAnswer contents to a SignedRequest
struct DescribeTerminationPolicyTypesAnswerWriter;
impl DescribeTerminationPolicyTypesAnswerWriter {
	fn write_params(params: &mut Params, name: &str, obj: &DescribeTerminationPolicyTypesAnswer) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		TerminationPoliciesWriter::write_params(params, &(prefix.to_string() + "XmlStringMaxLen1600"), &obj.termination_policy_types);
	}
}
#[derive(Debug, Default)]
pub struct DeleteNotificationConfigurationType {
	/// The name of the Auto Scaling group.
	pub auto_scaling_group_name: ResourceName,
	/// The Amazon Resource Name (ARN) of the Amazon Simple Notification Service (SNS)
	/// topic.
	pub topic_arn: ResourceName,
}

/// Parse DeleteNotificationConfigurationType from XML
struct DeleteNotificationConfigurationTypeParser;
impl DeleteNotificationConfigurationTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<DeleteNotificationConfigurationType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = DeleteNotificationConfigurationType::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "AutoScalingGroupName" {
				obj.auto_scaling_group_name = try!(ResourceNameParser::parse_xml("AutoScalingGroupName", stack));
				continue;
			}
			if current_name == "TopicARN" {
				obj.topic_arn = try!(ResourceNameParser::parse_xml("TopicARN", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write DeleteNotificationConfigurationType contents to a SignedRequest
struct DeleteNotificationConfigurationTypeWriter;
impl DeleteNotificationConfigurationTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &DeleteNotificationConfigurationType) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		ResourceNameWriter::write_params(params, &(prefix.to_string() + "AutoScalingGroupName"), &obj.auto_scaling_group_name);
		ResourceNameWriter::write_params(params, &(prefix.to_string() + "TopicARN"), &obj.topic_arn);
	}
}
pub type Metrics = Vec<XmlStringMaxLen255>;
/// Parse Metrics from XML
struct MetricsParser;
impl MetricsParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<Metrics, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "XmlStringMaxLen255" {
			obj.push(try!(XmlStringMaxLen255Parser::parse_xml("XmlStringMaxLen255", stack)));
		}
		Ok(obj)
	}
}
/// Write Metrics contents to a SignedRequest
struct MetricsWriter;
impl MetricsWriter {
	fn write_params(params: &mut Params, name: &str, obj: &Metrics) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			XmlStringMaxLen255Writer::write_params(params, key, &element);
			index += 1;
		}
	}
}
pub type MaxRecords = i32;
/// Parse MaxRecords from XML
struct MaxRecordsParser;
impl MaxRecordsParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<MaxRecords, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = i32::from_str(try!(characters(stack)).as_ref()).unwrap();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write MaxRecords contents to a SignedRequest
struct MaxRecordsWriter;
impl MaxRecordsWriter {
	fn write_params(params: &mut Params, name: &str, obj: &MaxRecords) {
		params.put(name, &obj.to_string());
	}
}
#[derive(Debug, Default)]
pub struct EnableMetricsCollectionQuery {
	/// One or more metrics. If you omit this parameter, all metrics are enabled.
	///   * `GroupMinSize`
	///   * `GroupMaxSize`
	///   * `GroupDesiredCapacity`
	///   * `GroupInServiceInstances`
	///   * `GroupPendingInstances`
	///   * `GroupStandbyInstances`
	///   * `GroupTerminatingInstances`
	///   * `GroupTotalInstances`
	/// Note that the `GroupStandbyInstances` metric is not enabled by default. You
	/// must explicitly request this metric.
	pub metrics: Option<Metrics>,
	/// The name or ARN of the Auto Scaling group.
	pub auto_scaling_group_name: ResourceName,
	/// The granularity to associate with the metrics to collect. The only valid value
	/// is `1Minute`.
	pub granularity: XmlStringMaxLen255,
}

/// Parse EnableMetricsCollectionQuery from XML
struct EnableMetricsCollectionQueryParser;
impl EnableMetricsCollectionQueryParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<EnableMetricsCollectionQuery, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = EnableMetricsCollectionQuery::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "XmlStringMaxLen255" {
				obj.metrics = Some(try!(MetricsParser::parse_xml("XmlStringMaxLen255", stack)));
				continue;
			}
			if current_name == "AutoScalingGroupName" {
				obj.auto_scaling_group_name = try!(ResourceNameParser::parse_xml("AutoScalingGroupName", stack));
				continue;
			}
			if current_name == "Granularity" {
				obj.granularity = try!(XmlStringMaxLen255Parser::parse_xml("Granularity", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write EnableMetricsCollectionQuery contents to a SignedRequest
struct EnableMetricsCollectionQueryWriter;
impl EnableMetricsCollectionQueryWriter {
	fn write_params(params: &mut Params, name: &str, obj: &EnableMetricsCollectionQuery) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		if let Some(ref obj) = obj.metrics {
			MetricsWriter::write_params(params, &(prefix.to_string() + "XmlStringMaxLen255"), obj);
		}
		ResourceNameWriter::write_params(params, &(prefix.to_string() + "AutoScalingGroupName"), &obj.auto_scaling_group_name);
		XmlStringMaxLen255Writer::write_params(params, &(prefix.to_string() + "Granularity"), &obj.granularity);
	}
}
pub type TerminationPolicies = Vec<XmlStringMaxLen1600>;
/// Parse TerminationPolicies from XML
struct TerminationPoliciesParser;
impl TerminationPoliciesParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<TerminationPolicies, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "XmlStringMaxLen1600" {
			obj.push(try!(XmlStringMaxLen1600Parser::parse_xml("XmlStringMaxLen1600", stack)));
		}
		Ok(obj)
	}
}
/// Write TerminationPolicies contents to a SignedRequest
struct TerminationPoliciesWriter;
impl TerminationPoliciesWriter {
	fn write_params(params: &mut Params, name: &str, obj: &TerminationPolicies) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			XmlStringMaxLen1600Writer::write_params(params, key, &element);
			index += 1;
		}
	}
}
/// Describes the state of a load balancer.
#[derive(Debug, Default)]
pub struct LoadBalancerState {
	/// The state of the load balancer.
	///   * `Adding` \- The instances in the group are being registered with the load balancer.
	///   * `Added` \- All instances in the group are registered with the load balancer.
	///   * `InService` \- At least one instance in the group passed an ELB health check.
	///   * `Removing` \- The instances are being deregistered from the load balancer. If connection draining is enabled, Elastic Load Balancing waits for in-flight requests to complete before deregistering the instances.
	pub state: XmlStringMaxLen255,
	/// The name of the load balancer.
	pub load_balancer_name: XmlStringMaxLen255,
}

/// Parse LoadBalancerState from XML
struct LoadBalancerStateParser;
impl LoadBalancerStateParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<LoadBalancerState, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = LoadBalancerState::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "State" {
				obj.state = try!(XmlStringMaxLen255Parser::parse_xml("State", stack));
				continue;
			}
			if current_name == "LoadBalancerName" {
				obj.load_balancer_name = try!(XmlStringMaxLen255Parser::parse_xml("LoadBalancerName", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write LoadBalancerState contents to a SignedRequest
struct LoadBalancerStateWriter;
impl LoadBalancerStateWriter {
	fn write_params(params: &mut Params, name: &str, obj: &LoadBalancerState) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		XmlStringMaxLen255Writer::write_params(params, &(prefix.to_string() + "State"), &obj.state);
		XmlStringMaxLen255Writer::write_params(params, &(prefix.to_string() + "LoadBalancerName"), &obj.load_balancer_name);
	}
}
pub type ResourceName = String;
/// Parse ResourceName from XML
struct ResourceNameParser;
impl ResourceNameParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ResourceName, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ResourceName contents to a SignedRequest
struct ResourceNameWriter;
impl ResourceNameWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ResourceName) {
		params.put(name, obj);
	}
}
pub type SpotPrice = String;
/// Parse SpotPrice from XML
struct SpotPriceParser;
impl SpotPriceParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<SpotPrice, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write SpotPrice contents to a SignedRequest
struct SpotPriceWriter;
impl SpotPriceWriter {
	fn write_params(params: &mut Params, name: &str, obj: &SpotPrice) {
		params.put(name, obj);
	}
}
pub type ScheduledUpdateGroupActions = Vec<ScheduledUpdateGroupAction>;
/// Parse ScheduledUpdateGroupActions from XML
struct ScheduledUpdateGroupActionsParser;
impl ScheduledUpdateGroupActionsParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ScheduledUpdateGroupActions, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "ScheduledUpdateGroupAction" {
			obj.push(try!(ScheduledUpdateGroupActionParser::parse_xml("ScheduledUpdateGroupAction", stack)));
		}
		Ok(obj)
	}
}
/// Write ScheduledUpdateGroupActions contents to a SignedRequest
struct ScheduledUpdateGroupActionsWriter;
impl ScheduledUpdateGroupActionsWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ScheduledUpdateGroupActions) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			ScheduledUpdateGroupActionWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
/// Describes an Auto Scaling process that has been suspended. For more
/// information, see ProcessType.
#[derive(Debug, Default)]
pub struct SuspendedProcess {
	/// The name of the suspended process.
	pub process_name: XmlStringMaxLen255,
	/// The reason that the process was suspended.
	pub suspension_reason: XmlStringMaxLen255,
}

/// Parse SuspendedProcess from XML
struct SuspendedProcessParser;
impl SuspendedProcessParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<SuspendedProcess, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = SuspendedProcess::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "ProcessName" {
				obj.process_name = try!(XmlStringMaxLen255Parser::parse_xml("ProcessName", stack));
				continue;
			}
			if current_name == "SuspensionReason" {
				obj.suspension_reason = try!(XmlStringMaxLen255Parser::parse_xml("SuspensionReason", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write SuspendedProcess contents to a SignedRequest
struct SuspendedProcessWriter;
impl SuspendedProcessWriter {
	fn write_params(params: &mut Params, name: &str, obj: &SuspendedProcess) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		XmlStringMaxLen255Writer::write_params(params, &(prefix.to_string() + "ProcessName"), &obj.process_name);
		XmlStringMaxLen255Writer::write_params(params, &(prefix.to_string() + "SuspensionReason"), &obj.suspension_reason);
	}
}
pub type AutoScalingGroups = Vec<AutoScalingGroup>;
/// Parse AutoScalingGroups from XML
struct AutoScalingGroupsParser;
impl AutoScalingGroupsParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<AutoScalingGroups, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "AutoScalingGroup" {
			obj.push(try!(AutoScalingGroupParser::parse_xml("AutoScalingGroup", stack)));
		}
		Ok(obj)
	}
}
/// Write AutoScalingGroups contents to a SignedRequest
struct AutoScalingGroupsWriter;
impl AutoScalingGroupsWriter {
	fn write_params(params: &mut Params, name: &str, obj: &AutoScalingGroups) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			AutoScalingGroupWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
/// Describes an alarm.
#[derive(Debug, Default)]
pub struct Alarm {
	/// The name of the alarm.
	pub alarm_name: XmlStringMaxLen255,
	/// The Amazon Resource Name (ARN) of the alarm.
	pub alarm_arn: ResourceName,
}

/// Parse Alarm from XML
struct AlarmParser;
impl AlarmParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<Alarm, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = Alarm::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "AlarmName" {
				obj.alarm_name = try!(XmlStringMaxLen255Parser::parse_xml("AlarmName", stack));
				continue;
			}
			if current_name == "AlarmARN" {
				obj.alarm_arn = try!(ResourceNameParser::parse_xml("AlarmARN", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write Alarm contents to a SignedRequest
struct AlarmWriter;
impl AlarmWriter {
	fn write_params(params: &mut Params, name: &str, obj: &Alarm) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		XmlStringMaxLen255Writer::write_params(params, &(prefix.to_string() + "AlarmName"), &obj.alarm_name);
		ResourceNameWriter::write_params(params, &(prefix.to_string() + "AlarmARN"), &obj.alarm_arn);
	}
}
#[derive(Debug, Default)]
pub struct AttachLoadBalancersType {
	/// The name of the group.
	pub auto_scaling_group_name: ResourceName,
	/// One or more load balancer names.
	pub load_balancer_names: LoadBalancerNames,
}

/// Parse AttachLoadBalancersType from XML
struct AttachLoadBalancersTypeParser;
impl AttachLoadBalancersTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<AttachLoadBalancersType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = AttachLoadBalancersType::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "AutoScalingGroupName" {
				obj.auto_scaling_group_name = try!(ResourceNameParser::parse_xml("AutoScalingGroupName", stack));
				continue;
			}
			if current_name == "XmlStringMaxLen255" {
				obj.load_balancer_names = try!(LoadBalancerNamesParser::parse_xml("XmlStringMaxLen255", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write AttachLoadBalancersType contents to a SignedRequest
struct AttachLoadBalancersTypeWriter;
impl AttachLoadBalancersTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &AttachLoadBalancersType) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		ResourceNameWriter::write_params(params, &(prefix.to_string() + "AutoScalingGroupName"), &obj.auto_scaling_group_name);
		LoadBalancerNamesWriter::write_params(params, &(prefix.to_string() + "XmlStringMaxLen255"), &obj.load_balancer_names);
	}
}
#[derive(Debug, Default)]
pub struct ActivitiesType {
	/// The scaling activities.
	pub activities: Activities,
	/// The token to use when requesting the next set of items. If there are no
	/// additional items to return, the string is empty.
	pub next_token: Option<XmlString>,
}

/// Parse ActivitiesType from XML
struct ActivitiesTypeParser;
impl ActivitiesTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ActivitiesType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = ActivitiesType::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "Activity" {
				obj.activities = try!(ActivitiesParser::parse_xml("Activity", stack));
				continue;
			}
			if current_name == "NextToken" {
				obj.next_token = Some(try!(XmlStringParser::parse_xml("NextToken", stack)));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ActivitiesType contents to a SignedRequest
struct ActivitiesTypeWriter;
impl ActivitiesTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ActivitiesType) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		ActivitiesWriter::write_params(params, &(prefix.to_string() + "Activity"), &obj.activities);
		if let Some(ref obj) = obj.next_token {
			XmlStringWriter::write_params(params, &(prefix.to_string() + "NextToken"), obj);
		}
	}
}
pub type XmlStringMaxLen1023 = String;
/// Parse XmlStringMaxLen1023 from XML
struct XmlStringMaxLen1023Parser;
impl XmlStringMaxLen1023Parser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<XmlStringMaxLen1023, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write XmlStringMaxLen1023 contents to a SignedRequest
struct XmlStringMaxLen1023Writer;
impl XmlStringMaxLen1023Writer {
	fn write_params(params: &mut Params, name: &str, obj: &XmlStringMaxLen1023) {
		params.put(name, obj);
	}
}
pub type NoDevice = bool;
/// Parse NoDevice from XML
struct NoDeviceParser;
impl NoDeviceParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<NoDevice, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = bool::from_str(try!(characters(stack)).as_ref()).unwrap();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write NoDevice contents to a SignedRequest
struct NoDeviceWriter;
impl NoDeviceWriter {
	fn write_params(params: &mut Params, name: &str, obj: &NoDevice) {
		params.put(name, &obj.to_string());
	}
}
#[derive(Debug, Default)]
pub struct PutLifecycleHookType {
	/// Defines the amount of time, in seconds, that can elapse before the lifecycle
	/// hook times out. When the lifecycle hook times out, Auto Scaling performs the
	/// action defined in the `DefaultResult` parameter. You can prevent the lifecycle
	/// hook from timing out by calling RecordLifecycleActionHeartbeat. The default
	/// value for this parameter is 3600 seconds (1 hour).
	pub heartbeat_timeout: Option<HeartbeatTimeout>,
	/// The ARN of the IAM role that allows the Auto Scaling group to publish to the
	/// specified notification target.
	/// This parameter is required for new lifecycle hooks, but optional when updating
	/// existing hooks.
	pub role_arn: Option<ResourceName>,
	/// The name of the Auto Scaling group to which you want to assign the lifecycle
	/// hook.
	pub auto_scaling_group_name: ResourceName,
	/// The name of the lifecycle hook.
	pub lifecycle_hook_name: AsciiStringMaxLen255,
	/// Contains additional information that you want to include any time Auto Scaling
	/// sends a message to the notification target.
	pub notification_metadata: Option<XmlStringMaxLen1023>,
	/// Defines the action the Auto Scaling group should take when the lifecycle hook
	/// timeout elapses or if an unexpected failure occurs. The value for this
	/// parameter can be either `CONTINUE` or `ABANDON`. The default value for this
	/// parameter is `ABANDON`.
	pub default_result: Option<LifecycleActionResult>,
	/// The ARN of the notification target that Auto Scaling will use to notify you
	/// when an instance is in the transition state for the lifecycle hook. This ARN
	/// target can be either an SQS queue or an SNS topic.
	/// This parameter is required for new lifecycle hooks, but optional when updating
	/// existing hooks.
	/// The notification message sent to the target will include:
	///   * **LifecycleActionToken**. The Lifecycle action token.
	///   * **AccountId**. The user account ID.
	///   * **AutoScalingGroupName**. The name of the Auto Scaling group.
	///   * **LifecycleHookName**. The lifecycle hook name.
	///   * **EC2InstanceId**. The EC2 instance ID.
	///   * **LifecycleTransition**. The lifecycle transition.
	///   * **NotificationMetadata**. The notification metadata.
	/// This operation uses the JSON format when sending notifications to an Amazon
	/// SQS queue, and an email key/value pair format when sending notifications to an
	/// Amazon SNS topic.
	/// When you call this operation, a test message is sent to the notification
	/// target. This test message contains an additional key/value pair:
	/// `Event:autoscaling:TEST_NOTIFICATION`.
	pub notification_target_arn: Option<ResourceName>,
	/// The instance state to which you want to attach the lifecycle hook. For a list
	/// of lifecycle hook types, see DescribeLifecycleHookTypes.
	/// This parameter is required for new lifecycle hooks, but optional when updating
	/// existing hooks.
	pub lifecycle_transition: Option<LifecycleTransition>,
}

/// Parse PutLifecycleHookType from XML
struct PutLifecycleHookTypeParser;
impl PutLifecycleHookTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<PutLifecycleHookType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = PutLifecycleHookType::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "HeartbeatTimeout" {
				obj.heartbeat_timeout = Some(try!(HeartbeatTimeoutParser::parse_xml("HeartbeatTimeout", stack)));
				continue;
			}
			if current_name == "RoleARN" {
				obj.role_arn = Some(try!(ResourceNameParser::parse_xml("RoleARN", stack)));
				continue;
			}
			if current_name == "AutoScalingGroupName" {
				obj.auto_scaling_group_name = try!(ResourceNameParser::parse_xml("AutoScalingGroupName", stack));
				continue;
			}
			if current_name == "LifecycleHookName" {
				obj.lifecycle_hook_name = try!(AsciiStringMaxLen255Parser::parse_xml("LifecycleHookName", stack));
				continue;
			}
			if current_name == "NotificationMetadata" {
				obj.notification_metadata = Some(try!(XmlStringMaxLen1023Parser::parse_xml("NotificationMetadata", stack)));
				continue;
			}
			if current_name == "DefaultResult" {
				obj.default_result = Some(try!(LifecycleActionResultParser::parse_xml("DefaultResult", stack)));
				continue;
			}
			if current_name == "NotificationTargetARN" {
				obj.notification_target_arn = Some(try!(ResourceNameParser::parse_xml("NotificationTargetARN", stack)));
				continue;
			}
			if current_name == "LifecycleTransition" {
				obj.lifecycle_transition = Some(try!(LifecycleTransitionParser::parse_xml("LifecycleTransition", stack)));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write PutLifecycleHookType contents to a SignedRequest
struct PutLifecycleHookTypeWriter;
impl PutLifecycleHookTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &PutLifecycleHookType) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		if let Some(ref obj) = obj.heartbeat_timeout {
			HeartbeatTimeoutWriter::write_params(params, &(prefix.to_string() + "HeartbeatTimeout"), obj);
		}
		if let Some(ref obj) = obj.role_arn {
			ResourceNameWriter::write_params(params, &(prefix.to_string() + "RoleARN"), obj);
		}
		ResourceNameWriter::write_params(params, &(prefix.to_string() + "AutoScalingGroupName"), &obj.auto_scaling_group_name);
		AsciiStringMaxLen255Writer::write_params(params, &(prefix.to_string() + "LifecycleHookName"), &obj.lifecycle_hook_name);
		if let Some(ref obj) = obj.notification_metadata {
			XmlStringMaxLen1023Writer::write_params(params, &(prefix.to_string() + "NotificationMetadata"), obj);
		}
		if let Some(ref obj) = obj.default_result {
			LifecycleActionResultWriter::write_params(params, &(prefix.to_string() + "DefaultResult"), obj);
		}
		if let Some(ref obj) = obj.notification_target_arn {
			ResourceNameWriter::write_params(params, &(prefix.to_string() + "NotificationTargetARN"), obj);
		}
		if let Some(ref obj) = obj.lifecycle_transition {
			LifecycleTransitionWriter::write_params(params, &(prefix.to_string() + "LifecycleTransition"), obj);
		}
	}
}
#[derive(Debug, Default)]
pub struct PutLifecycleHookAnswer;

/// Parse PutLifecycleHookAnswer from XML
struct PutLifecycleHookAnswerParser;
impl PutLifecycleHookAnswerParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<PutLifecycleHookAnswer, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = PutLifecycleHookAnswer::default();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write PutLifecycleHookAnswer contents to a SignedRequest
struct PutLifecycleHookAnswerWriter;
impl PutLifecycleHookAnswerWriter {
	fn write_params(params: &mut Params, name: &str, obj: &PutLifecycleHookAnswer) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
	}
}
pub type ScheduledActionNames = Vec<ResourceName>;
/// Parse ScheduledActionNames from XML
struct ScheduledActionNamesParser;
impl ScheduledActionNamesParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ScheduledActionNames, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "ResourceName" {
			obj.push(try!(ResourceNameParser::parse_xml("ResourceName", stack)));
		}
		Ok(obj)
	}
}
/// Write ScheduledActionNames contents to a SignedRequest
struct ScheduledActionNamesWriter;
impl ScheduledActionNamesWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ScheduledActionNames) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			ResourceNameWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
/// Describes scaling activity, which is a long-running process that represents a
/// change to your Auto Scaling group, such as changing its size or replacing an
/// instance.
#[derive(Debug, Default)]
pub struct Activity {
	/// A friendly, more verbose description of the activity.
	pub description: Option<XmlString>,
	/// The name of the Auto Scaling group.
	pub auto_scaling_group_name: XmlStringMaxLen255,
	/// The ID of the activity.
	pub activity_id: XmlString,
	/// The details about the activity.
	pub details: Option<XmlString>,
	/// The start time of the activity.
	pub start_time: TimestampType,
	/// A value between 0 and 100 that indicates the progress of the activity.
	pub progress: Option<Progress>,
	/// The end time of the activity.
	pub end_time: Option<TimestampType>,
	/// The reason the activity began.
	pub cause: XmlStringMaxLen1023,
	/// A friendly, more verbose description of the activity status.
	pub status_message: Option<XmlStringMaxLen255>,
	/// The current status of the activity.
	pub status_code: ScalingActivityStatusCode,
}

/// Parse Activity from XML
struct ActivityParser;
impl ActivityParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<Activity, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = Activity::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "Description" {
				obj.description = Some(try!(XmlStringParser::parse_xml("Description", stack)));
				continue;
			}
			if current_name == "AutoScalingGroupName" {
				obj.auto_scaling_group_name = try!(XmlStringMaxLen255Parser::parse_xml("AutoScalingGroupName", stack));
				continue;
			}
			if current_name == "ActivityId" {
				obj.activity_id = try!(XmlStringParser::parse_xml("ActivityId", stack));
				continue;
			}
			if current_name == "Details" {
				obj.details = Some(try!(XmlStringParser::parse_xml("Details", stack)));
				continue;
			}
			if current_name == "StartTime" {
				obj.start_time = try!(TimestampTypeParser::parse_xml("StartTime", stack));
				continue;
			}
			if current_name == "Progress" {
				obj.progress = Some(try!(ProgressParser::parse_xml("Progress", stack)));
				continue;
			}
			if current_name == "EndTime" {
				obj.end_time = Some(try!(TimestampTypeParser::parse_xml("EndTime", stack)));
				continue;
			}
			if current_name == "Cause" {
				obj.cause = try!(XmlStringMaxLen1023Parser::parse_xml("Cause", stack));
				continue;
			}
			if current_name == "StatusMessage" {
				obj.status_message = Some(try!(XmlStringMaxLen255Parser::parse_xml("StatusMessage", stack)));
				continue;
			}
			if current_name == "StatusCode" {
				obj.status_code = try!(ScalingActivityStatusCodeParser::parse_xml("StatusCode", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write Activity contents to a SignedRequest
struct ActivityWriter;
impl ActivityWriter {
	fn write_params(params: &mut Params, name: &str, obj: &Activity) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		if let Some(ref obj) = obj.description {
			XmlStringWriter::write_params(params, &(prefix.to_string() + "Description"), obj);
		}
		XmlStringMaxLen255Writer::write_params(params, &(prefix.to_string() + "AutoScalingGroupName"), &obj.auto_scaling_group_name);
		XmlStringWriter::write_params(params, &(prefix.to_string() + "ActivityId"), &obj.activity_id);
		if let Some(ref obj) = obj.details {
			XmlStringWriter::write_params(params, &(prefix.to_string() + "Details"), obj);
		}
		TimestampTypeWriter::write_params(params, &(prefix.to_string() + "StartTime"), &obj.start_time);
		if let Some(ref obj) = obj.progress {
			ProgressWriter::write_params(params, &(prefix.to_string() + "Progress"), obj);
		}
		if let Some(ref obj) = obj.end_time {
			TimestampTypeWriter::write_params(params, &(prefix.to_string() + "EndTime"), obj);
		}
		XmlStringMaxLen1023Writer::write_params(params, &(prefix.to_string() + "Cause"), &obj.cause);
		if let Some(ref obj) = obj.status_message {
			XmlStringMaxLen255Writer::write_params(params, &(prefix.to_string() + "StatusMessage"), obj);
		}
		ScalingActivityStatusCodeWriter::write_params(params, &(prefix.to_string() + "StatusCode"), &obj.status_code);
	}
}
pub type XmlStringMaxLen64 = String;
/// Parse XmlStringMaxLen64 from XML
struct XmlStringMaxLen64Parser;
impl XmlStringMaxLen64Parser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<XmlStringMaxLen64, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write XmlStringMaxLen64 contents to a SignedRequest
struct XmlStringMaxLen64Writer;
impl XmlStringMaxLen64Writer {
	fn write_params(params: &mut Params, name: &str, obj: &XmlStringMaxLen64) {
		params.put(name, obj);
	}
}
/// Describes a metric.
#[derive(Debug, Default)]
pub struct MetricCollectionType {
	/// The metric.
	///   * `GroupMinSize`
	///   * `GroupMaxSize`
	///   * `GroupDesiredCapacity`
	///   * `GroupInServiceInstances`
	///   * `GroupPendingInstances`
	///   * `GroupStandbyInstances`
	///   * `GroupTerminatingInstances`
	///   * `GroupTotalInstances`
	pub metric: XmlStringMaxLen255,
}

/// Parse MetricCollectionType from XML
struct MetricCollectionTypeParser;
impl MetricCollectionTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<MetricCollectionType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = MetricCollectionType::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "Metric" {
				obj.metric = try!(XmlStringMaxLen255Parser::parse_xml("Metric", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write MetricCollectionType contents to a SignedRequest
struct MetricCollectionTypeWriter;
impl MetricCollectionTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &MetricCollectionType) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		XmlStringMaxLen255Writer::write_params(params, &(prefix.to_string() + "Metric"), &obj.metric);
	}
}
#[derive(Debug, Default)]
pub struct DescribeAutoScalingInstancesType {
	/// The maximum number of items to return with this call.
	pub max_records: MaxRecords,
	/// The token for the next set of items to return. (You received this token from a
	/// previous call.)
	pub next_token: XmlString,
	/// One or more Auto Scaling instances to describe, up to 50 instances. If you
	/// omit this parameter, all Auto Scaling instances are described. If you specify
	/// an ID that does not exist, it is ignored with no error.
	pub instance_ids: InstanceIds,
}

/// Parse DescribeAutoScalingInstancesType from XML
struct DescribeAutoScalingInstancesTypeParser;
impl DescribeAutoScalingInstancesTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<DescribeAutoScalingInstancesType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = DescribeAutoScalingInstancesType::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "MaxRecords" {
				obj.max_records = try!(MaxRecordsParser::parse_xml("MaxRecords", stack));
				continue;
			}
			if current_name == "NextToken" {
				obj.next_token = try!(XmlStringParser::parse_xml("NextToken", stack));
				continue;
			}
			if current_name == "XmlStringMaxLen16" {
				obj.instance_ids = try!(InstanceIdsParser::parse_xml("XmlStringMaxLen16", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write DescribeAutoScalingInstancesType contents to a SignedRequest
struct DescribeAutoScalingInstancesTypeWriter;
impl DescribeAutoScalingInstancesTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &DescribeAutoScalingInstancesType) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		MaxRecordsWriter::write_params(params, &(prefix.to_string() + "MaxRecords"), &obj.max_records);
		XmlStringWriter::write_params(params, &(prefix.to_string() + "NextToken"), &obj.next_token);
		InstanceIdsWriter::write_params(params, &(prefix.to_string() + "XmlStringMaxLen16"), &obj.instance_ids);
	}
}
#[derive(Debug, Default)]
pub struct SetInstanceHealthQuery {
	/// The ID of the EC2 instance.
	pub instance_id: XmlStringMaxLen16,
	/// If the Auto Scaling group of the specified instance has a
	/// `HealthCheckGracePeriod` specified for the group, by default, this call will
	/// respect the grace period. Set this to `False`, if you do not want the call to
	/// respect the grace period associated with the group.
	/// For more information, see the `HealthCheckGracePeriod` parameter description
	/// for CreateAutoScalingGroup.
	pub should_respect_grace_period: Option<ShouldRespectGracePeriod>,
	/// The health status of the instance. Set to `Healthy` if you want the instance
	/// to remain in service. Set to `Unhealthy` if you want the instance to be out of
	/// service. Auto Scaling will terminate and replace the unhealthy instance.
	pub health_status: XmlStringMaxLen32,
}

/// Parse SetInstanceHealthQuery from XML
struct SetInstanceHealthQueryParser;
impl SetInstanceHealthQueryParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<SetInstanceHealthQuery, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = SetInstanceHealthQuery::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "InstanceId" {
				obj.instance_id = try!(XmlStringMaxLen16Parser::parse_xml("InstanceId", stack));
				continue;
			}
			if current_name == "ShouldRespectGracePeriod" {
				obj.should_respect_grace_period = Some(try!(ShouldRespectGracePeriodParser::parse_xml("ShouldRespectGracePeriod", stack)));
				continue;
			}
			if current_name == "HealthStatus" {
				obj.health_status = try!(XmlStringMaxLen32Parser::parse_xml("HealthStatus", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write SetInstanceHealthQuery contents to a SignedRequest
struct SetInstanceHealthQueryWriter;
impl SetInstanceHealthQueryWriter {
	fn write_params(params: &mut Params, name: &str, obj: &SetInstanceHealthQuery) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		XmlStringMaxLen16Writer::write_params(params, &(prefix.to_string() + "InstanceId"), &obj.instance_id);
		if let Some(ref obj) = obj.should_respect_grace_period {
			ShouldRespectGracePeriodWriter::write_params(params, &(prefix.to_string() + "ShouldRespectGracePeriod"), obj);
		}
		XmlStringMaxLen32Writer::write_params(params, &(prefix.to_string() + "HealthStatus"), &obj.health_status);
	}
}
#[derive(Debug, Default)]
pub struct DescribeLoadBalancersResponse {
	/// The load balancers.
	pub load_balancers: LoadBalancerStates,
	/// The token to use when requesting the next set of items. If there are no
	/// additional items to return, the string is empty.
	pub next_token: XmlString,
}

/// Parse DescribeLoadBalancersResponse from XML
struct DescribeLoadBalancersResponseParser;
impl DescribeLoadBalancersResponseParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<DescribeLoadBalancersResponse, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = DescribeLoadBalancersResponse::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "LoadBalancerState" {
				obj.load_balancers = try!(LoadBalancerStatesParser::parse_xml("LoadBalancerState", stack));
				continue;
			}
			if current_name == "NextToken" {
				obj.next_token = try!(XmlStringParser::parse_xml("NextToken", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write DescribeLoadBalancersResponse contents to a SignedRequest
struct DescribeLoadBalancersResponseWriter;
impl DescribeLoadBalancersResponseWriter {
	fn write_params(params: &mut Params, name: &str, obj: &DescribeLoadBalancersResponse) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		LoadBalancerStatesWriter::write_params(params, &(prefix.to_string() + "LoadBalancerState"), &obj.load_balancers);
		XmlStringWriter::write_params(params, &(prefix.to_string() + "NextToken"), &obj.next_token);
	}
}
pub type ShouldDecrementDesiredCapacity = bool;
/// Parse ShouldDecrementDesiredCapacity from XML
struct ShouldDecrementDesiredCapacityParser;
impl ShouldDecrementDesiredCapacityParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ShouldDecrementDesiredCapacity, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = bool::from_str(try!(characters(stack)).as_ref()).unwrap();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ShouldDecrementDesiredCapacity contents to a SignedRequest
struct ShouldDecrementDesiredCapacityWriter;
impl ShouldDecrementDesiredCapacityWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ShouldDecrementDesiredCapacity) {
		params.put(name, &obj.to_string());
	}
}
#[derive(Debug, Default)]
pub struct UpdateAutoScalingGroupType {
	/// The amount of time, in seconds, that Auto Scaling waits before checking the
	/// health status of an instance. The grace period begins when the instance passes
	/// the system status and instance status checks from Amazon EC2. For more
	/// information, see []().
	pub health_check_grace_period: Option<HealthCheckGracePeriod>,
	/// The name of the placement group into which you'll launch your instances, if
	/// any. For more information, see [Placement
	/// Groups](http://docs.aws.amazon.com/AWSEC2/latest/UserGuide/placement-
	/// groups.html).
	pub placement_group: Option<XmlStringMaxLen255>,
	/// The number of EC2 instances that should be running in the Auto Scaling group.
	/// This number must be greater than or equal to the minimum size of the group and
	/// less than or equal to the maximum size of the group.
	pub desired_capacity: Option<AutoScalingGroupDesiredCapacity>,
	/// A standalone termination policy or a list of termination policies used to
	/// select the instance to terminate. The policies are executed in the order that
	/// they are listed.
	/// For more information, see [Choosing a Termination Policy for Your Auto Scaling
	/// Group](http://docs.aws.amazon.com/AutoScaling/latest/DeveloperGuide/us-
	/// termination-policy.html) in the _Auto Scaling Developer Guide_.
	pub termination_policies: Option<TerminationPolicies>,
	/// The name of the Auto Scaling group.
	pub auto_scaling_group_name: ResourceName,
	/// The amount of time, in seconds, after a scaling activity completes before
	/// another scaling activity can start. For more information, see [Understanding
	/// Auto Scaling Cooldowns](http://docs.aws.amazon.com/AutoScaling/latest/Develope
	/// rGuide/Cooldown.html).
	pub default_cooldown: Option<Cooldown>,
	/// The minimum size of the Auto Scaling group.
	pub min_size: Option<AutoScalingGroupMinSize>,
	/// The maximum size of the Auto Scaling group.
	pub max_size: Option<AutoScalingGroupMaxSize>,
	/// The ID of the subnet, if you are launching into a VPC. You can specify several
	/// subnets in a comma-separated list.
	/// When you specify `VPCZoneIdentifier` with `AvailabilityZones`, ensure that the
	/// subnets' Availability Zones match the values you specify for
	/// `AvailabilityZones`.
	/// For more information, see [Auto Scaling and Amazon Virtual Private Cloud](http
	/// ://docs.aws.amazon.com/AutoScaling/latest/DeveloperGuide/autoscalingsubnets.ht
	/// ml) in the _Auto Scaling Developer Guide_.
	pub vpc_zone_identifier: Option<XmlStringMaxLen255>,
	/// The name of the launch configuration.
	pub launch_configuration_name: Option<ResourceName>,
	/// One or more Availability Zones for the group.
	pub availability_zones: Option<AvailabilityZones>,
	/// The type of health check for the instances in the Auto Scaling group. The
	/// health check type can either be `EC2` for Amazon EC2 or `ELB` for Elastic Load
	/// Balancing.
	pub health_check_type: Option<XmlStringMaxLen32>,
}

/// Parse UpdateAutoScalingGroupType from XML
struct UpdateAutoScalingGroupTypeParser;
impl UpdateAutoScalingGroupTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<UpdateAutoScalingGroupType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = UpdateAutoScalingGroupType::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "HealthCheckGracePeriod" {
				obj.health_check_grace_period = Some(try!(HealthCheckGracePeriodParser::parse_xml("HealthCheckGracePeriod", stack)));
				continue;
			}
			if current_name == "PlacementGroup" {
				obj.placement_group = Some(try!(XmlStringMaxLen255Parser::parse_xml("PlacementGroup", stack)));
				continue;
			}
			if current_name == "DesiredCapacity" {
				obj.desired_capacity = Some(try!(AutoScalingGroupDesiredCapacityParser::parse_xml("DesiredCapacity", stack)));
				continue;
			}
			if current_name == "XmlStringMaxLen1600" {
				obj.termination_policies = Some(try!(TerminationPoliciesParser::parse_xml("XmlStringMaxLen1600", stack)));
				continue;
			}
			if current_name == "AutoScalingGroupName" {
				obj.auto_scaling_group_name = try!(ResourceNameParser::parse_xml("AutoScalingGroupName", stack));
				continue;
			}
			if current_name == "DefaultCooldown" {
				obj.default_cooldown = Some(try!(CooldownParser::parse_xml("DefaultCooldown", stack)));
				continue;
			}
			if current_name == "MinSize" {
				obj.min_size = Some(try!(AutoScalingGroupMinSizeParser::parse_xml("MinSize", stack)));
				continue;
			}
			if current_name == "MaxSize" {
				obj.max_size = Some(try!(AutoScalingGroupMaxSizeParser::parse_xml("MaxSize", stack)));
				continue;
			}
			if current_name == "VPCZoneIdentifier" {
				obj.vpc_zone_identifier = Some(try!(XmlStringMaxLen255Parser::parse_xml("VPCZoneIdentifier", stack)));
				continue;
			}
			if current_name == "LaunchConfigurationName" {
				obj.launch_configuration_name = Some(try!(ResourceNameParser::parse_xml("LaunchConfigurationName", stack)));
				continue;
			}
			if current_name == "XmlStringMaxLen255" {
				obj.availability_zones = Some(try!(AvailabilityZonesParser::parse_xml("XmlStringMaxLen255", stack)));
				continue;
			}
			if current_name == "HealthCheckType" {
				obj.health_check_type = Some(try!(XmlStringMaxLen32Parser::parse_xml("HealthCheckType", stack)));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write UpdateAutoScalingGroupType contents to a SignedRequest
struct UpdateAutoScalingGroupTypeWriter;
impl UpdateAutoScalingGroupTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &UpdateAutoScalingGroupType) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		if let Some(ref obj) = obj.health_check_grace_period {
			HealthCheckGracePeriodWriter::write_params(params, &(prefix.to_string() + "HealthCheckGracePeriod"), obj);
		}
		if let Some(ref obj) = obj.placement_group {
			XmlStringMaxLen255Writer::write_params(params, &(prefix.to_string() + "PlacementGroup"), obj);
		}
		if let Some(ref obj) = obj.desired_capacity {
			AutoScalingGroupDesiredCapacityWriter::write_params(params, &(prefix.to_string() + "DesiredCapacity"), obj);
		}
		if let Some(ref obj) = obj.termination_policies {
			TerminationPoliciesWriter::write_params(params, &(prefix.to_string() + "XmlStringMaxLen1600"), obj);
		}
		ResourceNameWriter::write_params(params, &(prefix.to_string() + "AutoScalingGroupName"), &obj.auto_scaling_group_name);
		if let Some(ref obj) = obj.default_cooldown {
			CooldownWriter::write_params(params, &(prefix.to_string() + "DefaultCooldown"), obj);
		}
		if let Some(ref obj) = obj.min_size {
			AutoScalingGroupMinSizeWriter::write_params(params, &(prefix.to_string() + "MinSize"), obj);
		}
		if let Some(ref obj) = obj.max_size {
			AutoScalingGroupMaxSizeWriter::write_params(params, &(prefix.to_string() + "MaxSize"), obj);
		}
		if let Some(ref obj) = obj.vpc_zone_identifier {
			XmlStringMaxLen255Writer::write_params(params, &(prefix.to_string() + "VPCZoneIdentifier"), obj);
		}
		if let Some(ref obj) = obj.launch_configuration_name {
			ResourceNameWriter::write_params(params, &(prefix.to_string() + "LaunchConfigurationName"), obj);
		}
		if let Some(ref obj) = obj.availability_zones {
			AvailabilityZonesWriter::write_params(params, &(prefix.to_string() + "XmlStringMaxLen255"), obj);
		}
		if let Some(ref obj) = obj.health_check_type {
			XmlStringMaxLen32Writer::write_params(params, &(prefix.to_string() + "HealthCheckType"), obj);
		}
	}
}
#[derive(Debug, Default)]
pub struct AutoScalingGroupNamesType {
	/// The maximum number of items to return with this call.
	pub max_records: MaxRecords,
	/// The token for the next set of items to return. (You received this token from a
	/// previous call.)
	pub next_token: XmlString,
	/// The group names.
	pub auto_scaling_group_names: AutoScalingGroupNames,
}

/// Parse AutoScalingGroupNamesType from XML
struct AutoScalingGroupNamesTypeParser;
impl AutoScalingGroupNamesTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<AutoScalingGroupNamesType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = AutoScalingGroupNamesType::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "MaxRecords" {
				obj.max_records = try!(MaxRecordsParser::parse_xml("MaxRecords", stack));
				continue;
			}
			if current_name == "NextToken" {
				obj.next_token = try!(XmlStringParser::parse_xml("NextToken", stack));
				continue;
			}
			if current_name == "ResourceName" {
				obj.auto_scaling_group_names = try!(AutoScalingGroupNamesParser::parse_xml("ResourceName", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write AutoScalingGroupNamesType contents to a SignedRequest
struct AutoScalingGroupNamesTypeWriter;
impl AutoScalingGroupNamesTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &AutoScalingGroupNamesType) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		MaxRecordsWriter::write_params(params, &(prefix.to_string() + "MaxRecords"), &obj.max_records);
		XmlStringWriter::write_params(params, &(prefix.to_string() + "NextToken"), &obj.next_token);
		AutoScalingGroupNamesWriter::write_params(params, &(prefix.to_string() + "ResourceName"), &obj.auto_scaling_group_names);
	}
}
pub type Instances = Vec<Instance>;
/// Parse Instances from XML
struct InstancesParser;
impl InstancesParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<Instances, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "Instance" {
			obj.push(try!(InstanceParser::parse_xml("Instance", stack)));
		}
		Ok(obj)
	}
}
/// Write Instances contents to a SignedRequest
struct InstancesWriter;
impl InstancesWriter {
	fn write_params(params: &mut Params, name: &str, obj: &Instances) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			InstanceWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
pub type AutoScalingGroupMinSize = i32;
/// Parse AutoScalingGroupMinSize from XML
struct AutoScalingGroupMinSizeParser;
impl AutoScalingGroupMinSizeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<AutoScalingGroupMinSize, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = i32::from_str(try!(characters(stack)).as_ref()).unwrap();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write AutoScalingGroupMinSize contents to a SignedRequest
struct AutoScalingGroupMinSizeWriter;
impl AutoScalingGroupMinSizeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &AutoScalingGroupMinSize) {
		params.put(name, &obj.to_string());
	}
}
pub type LifecycleState = String;
/// Parse LifecycleState from XML
struct LifecycleStateParser;
impl LifecycleStateParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<LifecycleState, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write LifecycleState contents to a SignedRequest
struct LifecycleStateWriter;
impl LifecycleStateWriter {
	fn write_params(params: &mut Params, name: &str, obj: &LifecycleState) {
		params.put(name, obj);
	}
}
pub type Filters = Vec<Filter>;
/// Parse Filters from XML
struct FiltersParser;
impl FiltersParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<Filters, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "Filter" {
			obj.push(try!(FilterParser::parse_xml("Filter", stack)));
		}
		Ok(obj)
	}
}
/// Write Filters contents to a SignedRequest
struct FiltersWriter;
impl FiltersWriter {
	fn write_params(params: &mut Params, name: &str, obj: &Filters) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			FilterWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
pub type AvailabilityZones = Vec<XmlStringMaxLen255>;
/// Parse AvailabilityZones from XML
struct AvailabilityZonesParser;
impl AvailabilityZonesParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<AvailabilityZones, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "XmlStringMaxLen255" {
			obj.push(try!(XmlStringMaxLen255Parser::parse_xml("XmlStringMaxLen255", stack)));
		}
		Ok(obj)
	}
}
/// Write AvailabilityZones contents to a SignedRequest
struct AvailabilityZonesWriter;
impl AvailabilityZonesWriter {
	fn write_params(params: &mut Params, name: &str, obj: &AvailabilityZones) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			XmlStringMaxLen255Writer::write_params(params, key, &element);
			index += 1;
		}
	}
}
pub struct AutoscalingClient<'a> {
	creds: &'a AWSCredentials,
	region: &'a str
}

impl<'a> AutoscalingClient<'a> { 
	pub fn new(creds: &'a AWSCredentials, region: &'a str) -> AutoscalingClient<'a> {
		AutoscalingClient { creds: creds, region: region }
	}
	/// Deletes the specified tags.
	pub fn delete_tags(&self, input: &DeleteTagsType) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "DeleteTags");
		DeleteTagsTypeWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(())
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Describes one or more scaling activities for the specified Auto Scaling group.
	/// If you omit the `ActivityIds`, the call returns all activities from the past
	/// six weeks. Activities are sorted by the start time. Activities still in
	/// progress appear first on the list.
	pub fn describe_scaling_activities(&self, input: &DescribeScalingActivitiesType) -> Result<ActivitiesType, AWSError> {
		let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "DescribeScalingActivities");
		DescribeScalingActivitiesTypeWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(ActivitiesTypeParser::parse_xml("ActivitiesType", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Creates an Auto Scaling group with the specified name and attributes.
	/// If you exceed your maximum limit of Auto Scaling groups, which by default is
	/// 20 per region, the call fails. For information about viewing and updating this
	/// limit, see DescribeAccountLimits.
	/// For more information, see [Auto Scaling Groups](http://docs.aws.amazon.com/Aut
	/// oScaling/latest/DeveloperGuide/AutoScalingGroup.html) in the _Auto Scaling
	/// Developer Guide_.
	pub fn create_auto_scaling_group(&self, input: &CreateAutoScalingGroupType) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "CreateAutoScalingGroup");
		CreateAutoScalingGroupTypeWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(())
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Executes the specified policy.
	pub fn execute_policy(&self, input: &ExecutePolicyType) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "ExecutePolicy");
		ExecutePolicyTypeWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(())
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Describes the notification actions associated with the specified Auto Scaling
	/// group.
	pub fn describe_notification_configurations(&self, input: &DescribeNotificationConfigurationsType) -> Result<DescribeNotificationConfigurationsAnswer, AWSError> {
		let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "DescribeNotificationConfigurations");
		DescribeNotificationConfigurationsTypeWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(DescribeNotificationConfigurationsAnswerParser::parse_xml("DescribeNotificationConfigurationsAnswer", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Deletes the specified Auto Scaling group.
	/// The group must have no instances and no scaling activities in progress.
	/// To remove all instances before calling `DeleteAutoScalingGroup`, call
	/// UpdateAutoScalingGroup to set the minimum and maximum size of the Auto Scaling
	/// group to zero.
	pub fn delete_auto_scaling_group(&self, input: &DeleteAutoScalingGroupType) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "DeleteAutoScalingGroup");
		DeleteAutoScalingGroupTypeWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(())
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Deletes the specified Auto Scaling policy.
	pub fn delete_policy(&self, input: &DeletePolicyType) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "DeletePolicy");
		DeletePolicyTypeWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(())
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Moves the specified instances out of `Standby` mode.
	/// For more information, see [Auto Scaling InService State](http://docs.aws.amazo
	/// n.com/AutoScaling/latest/DeveloperGuide/AutoScalingInServiceState.html) in the
	/// _Auto Scaling Developer Guide_.
	pub fn exit_standby(&self, input: &ExitStandbyQuery) -> Result<ExitStandbyAnswer, AWSError> {
		let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "ExitStandby");
		ExitStandbyQueryWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(ExitStandbyAnswerParser::parse_xml("ExitStandbyAnswer", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Sets the health status of the specified instance.
	/// For more information, see [Health Checks](http://docs.aws.amazon.com/AutoScali
	/// ng/latest/DeveloperGuide/healthcheck.html) in the _Auto Scaling Developer
	/// Guide_.
	pub fn set_instance_health(&self, input: &SetInstanceHealthQuery) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "SetInstanceHealth");
		SetInstanceHealthQueryWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(())
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Creates or updates tags for the specified Auto Scaling group.
	/// A tag is defined by its resource ID, resource type, key, value, and propagate
	/// flag. The value and the propagate flag are optional parameters. The only
	/// supported resource type is `auto-scaling-group`, and the resource ID must be
	/// the name of the group. The `PropagateAtLaunch` flag determines whether the tag
	/// is added to instances launched in the group. Valid values are `true` or
	/// `false`.
	/// When you specify a tag with a key that already exists, the operation
	/// overwrites the previous tag definition, and you do not get an error message.
	/// For more information, see [Tagging Auto Scaling Groups and Instances](http://d
	/// ocs.aws.amazon.com/AutoScaling/latest/DeveloperGuide/ASTagging.html) in the
	/// _Auto Scaling Developer Guide_.
	pub fn create_or_update_tags(&self, input: &CreateOrUpdateTagsType) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "CreateOrUpdateTags");
		CreateOrUpdateTagsTypeWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(())
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Creates or updates a lifecycle hook for the specified Auto Scaling Group.
	/// A lifecycle hook tells Auto Scaling that you want to perform an action on an
	/// instance that is not actively in service; for example, either when the
	/// instance launches or before the instance terminates.
	/// This operation is a part of the basic sequence for adding a lifecycle hook to
	/// an Auto Scaling group:
	///   1. Create a notification target. A target can be either an Amazon SQS queue or an Amazon SNS topic.
	///   2. Create an IAM role. This role allows Auto Scaling to publish lifecycle notifications to the designated SQS queue or SNS topic.
	///   3. **Create the lifecycle hook. You can create a hook that acts when instances launch or when instances terminate.**
	///   4. If necessary, record the lifecycle action heartbeat to keep the instance in a pending state.
	///   5. Complete the lifecycle action.
	/// For more information, see [Auto Scaling Pending State](http://docs.aws.amazon.
	/// com/AutoScaling/latest/DeveloperGuide/AutoScalingPendingState.html) and [Auto
	/// Scaling Terminating State](http://docs.aws.amazon.com/AutoScaling/latest/Devel
	/// operGuide/AutoScalingTerminatingState.html) in the _Auto Scaling Developer
	/// Guide_.
	/// If you exceed your maximum limit of lifecycle hooks, which by default is 50
	/// per region, the call fails. For information about updating this limit, see
	/// [AWS Service
	/// Limits](http://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html)
	/// in the _Amazon Web Services General Reference_.
	pub fn put_lifecycle_hook(&self, input: &PutLifecycleHookType) -> Result<PutLifecycleHookAnswer, AWSError> {
		let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "PutLifecycleHook");
		PutLifecycleHookTypeWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(PutLifecycleHookAnswerParser::parse_xml("PutLifecycleHookAnswer", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Removes one or more load balancers from the specified Auto Scaling group.
	/// When you detach a load balancer, it enters the `Removing` state while
	/// deregistering the instances in the group. When all instances are deregistered,
	/// then you can no longer describe the load balancer using DescribeLoadBalancers.
	/// Note that the instances remain running.
	pub fn detach_load_balancers(&self, input: &DetachLoadBalancersType) -> Result<DetachLoadBalancersResultType, AWSError> {
		let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "DetachLoadBalancers");
		DetachLoadBalancersTypeWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(DetachLoadBalancersResultTypeParser::parse_xml("DetachLoadBalancersResultType", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Attaches one or more EC2 instances to the specified Auto Scaling group.
	/// For more information, see [Attach EC2 Instances to Your Auto Scaling
	/// Group](http://docs.aws.amazon.com/AutoScaling/latest/DeveloperGuide/attach-
	/// instance-asg.html) in the _Auto Scaling Developer Guide_.
	pub fn attach_instances(&self, input: &AttachInstancesQuery) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "AttachInstances");
		AttachInstancesQueryWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(())
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Creates or updates a scheduled scaling action for an Auto Scaling group. When
	/// updating a scheduled scaling action, if you leave a parameter unspecified, the
	/// corresponding value remains unchanged in the affected Auto Scaling group.
	/// For more information, see [Scheduled Scaling](http://docs.aws.amazon.com/AutoS
	/// caling/latest/DeveloperGuide/schedule_time.html) in the _Auto Scaling
	/// Developer Guide_.
	pub fn put_scheduled_update_group_action(&self, input: &PutScheduledUpdateGroupActionType) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "PutScheduledUpdateGroupAction");
		PutScheduledUpdateGroupActionTypeWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(())
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Creates or updates a policy for an Auto Scaling group. To update an existing
	/// policy, use the existing policy name and set the parameters you want to
	/// change. Any existing parameter not changed in an update to an existing policy
	/// is not changed in this update request.
	/// If you exceed your maximum limit of step adjustments, which by default is 20
	/// per region, the call fails. For information about updating this limit, see
	/// [AWS Service
	/// Limits](http://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html)
	/// in the _Amazon Web Services General Reference_.
	pub fn put_scaling_policy(&self, input: &PutScalingPolicyType) -> Result<PolicyARNType, AWSError> {
		let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "PutScalingPolicy");
		PutScalingPolicyTypeWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(PolicyARNTypeParser::parse_xml("PolicyARNType", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Deletes the specified scheduled action.
	pub fn delete_scheduled_action(&self, input: &DeleteScheduledActionType) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "DeleteScheduledAction");
		DeleteScheduledActionTypeWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(())
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Attaches one or more load balancers to the specified Auto Scaling group.
	/// To describe the load balancers for an Auto Scaling group, use
	/// DescribeLoadBalancers. To detach the load balancer from the Auto Scaling
	/// group, use DetachLoadBalancers.
	/// For more information, see [Attach a Load Balancer to Your Auto Scaling
	/// Group](http://docs.aws.amazon.com/AutoScaling/latest/DeveloperGuide/attach-
	/// load-balancer-asg.html) in the _Auto Scaling Developer Guide_.
	pub fn attach_load_balancers(&self, input: &AttachLoadBalancersType) -> Result<AttachLoadBalancersResultType, AWSError> {
		let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "AttachLoadBalancers");
		AttachLoadBalancersTypeWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(AttachLoadBalancersResultTypeParser::parse_xml("AttachLoadBalancersResultType", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Removes one or more instances from the specified Auto Scaling group. After the
	/// instances are detached, you can manage them independently from the rest of the
	/// Auto Scaling group.
	/// For more information, see [Detach EC2 Instances from Your Auto Scaling
	/// Group](http://docs.aws.amazon.com/AutoScaling/latest/DeveloperGuide/detach-
	/// instance-asg.html) in the _Auto Scaling Developer Guide_.
	pub fn detach_instances(&self, input: &DetachInstancesQuery) -> Result<DetachInstancesAnswer, AWSError> {
		let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "DetachInstances");
		DetachInstancesQueryWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(DetachInstancesAnswerParser::parse_xml("DetachInstancesAnswer", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Terminates the specified instance and optionally adjusts the desired group
	/// size.
	/// This call simply makes a termination request. The instances is not terminated
	/// immediately.
	pub fn terminate_instance_in_auto_scaling_group(&self, input: &TerminateInstanceInAutoScalingGroupType) -> Result<ActivityType, AWSError> {
		let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "TerminateInstanceInAutoScalingGroup");
		TerminateInstanceInAutoScalingGroupTypeWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(ActivityTypeParser::parse_xml("ActivityType", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Describes the load balancers for the specified Auto Scaling group.
	pub fn describe_load_balancers(&self, input: &DescribeLoadBalancersRequest) -> Result<DescribeLoadBalancersResponse, AWSError> {
		let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "DescribeLoadBalancers");
		DescribeLoadBalancersRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(DescribeLoadBalancersResponseParser::parse_xml("DescribeLoadBalancersResponse", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Completes the lifecycle action for the associated token initiated under the
	/// given lifecycle hook with the specified result.
	/// This operation is a part of the basic sequence for adding a lifecycle hook to
	/// an Auto Scaling group:
	///   1. Create a notification target. A target can be either an Amazon SQS queue or an Amazon SNS topic.
	///   2. Create an IAM role. This role allows Auto Scaling to publish lifecycle notifications to the designated SQS queue or SNS topic.
	///   3. Create the lifecycle hook. You can create a hook that acts when instances launch or when instances terminate.
	///   4. If necessary, record the lifecycle action heartbeat to keep the instance in a pending state.
	///   5. **Complete the lifecycle action**.
	/// For more information, see [Auto Scaling Pending State](http://docs.aws.amazon.
	/// com/AutoScaling/latest/DeveloperGuide/AutoScalingPendingState.html) and [Auto
	/// Scaling Terminating State](http://docs.aws.amazon.com/AutoScaling/latest/Devel
	/// operGuide/AutoScalingTerminatingState.html) in the _Auto Scaling Developer
	/// Guide_.
	pub fn complete_lifecycle_action(&self, input: &CompleteLifecycleActionType) -> Result<CompleteLifecycleActionAnswer, AWSError> {
		let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "CompleteLifecycleAction");
		CompleteLifecycleActionTypeWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(CompleteLifecycleActionAnswerParser::parse_xml("CompleteLifecycleActionAnswer", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Updates the configuration for the specified Auto Scaling group.
	/// To update an Auto Scaling group with a launch configuration with
	/// `InstanceMonitoring` set to `False`, you must first disable the collection of
	/// group metrics. Otherwise, you will get an error. If you have previously
	/// enabled the collection of group metrics, you can disable it using
	/// DisableMetricsCollection.
	/// The new settings are registered upon the completion of this call. Any launch
	/// configuration settings take effect on any triggers after this call returns.
	/// Scaling activities that are currently in progress aren't affected.
	/// Note the following:
	///   * If you specify a new value for `MinSize` without specifying a value for `DesiredCapacity`, and the new `MinSize` is larger than the current size of the group, we implicitly call SetDesiredCapacity to set the size of the group to the new value of `MinSize`.
	///   * If you specify a new value for `MaxSize` without specifying a value for `DesiredCapacity`, and the new `MaxSize` is smaller than the current size of the group, we implicitly call SetDesiredCapacity to set the size of the group to the new value of `MaxSize`.
	///   * All other optional parameters are left unchanged if not specified.
	pub fn update_auto_scaling_group(&self, input: &UpdateAutoScalingGroupType) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "UpdateAutoScalingGroup");
		UpdateAutoScalingGroupTypeWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(())
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Records a heartbeat for the lifecycle action associated with a specific token.
	/// This extends the timeout by the length of time defined by the
	/// `HeartbeatTimeout` parameter of PutLifecycleHook.
	/// This operation is a part of the basic sequence for adding a lifecycle hook to
	/// an Auto Scaling group:
	///   1. Create a notification target. A target can be either an Amazon SQS queue or an Amazon SNS topic.
	///   2. Create an IAM role. This role allows Auto Scaling to publish lifecycle notifications to the designated SQS queue or SNS topic.
	///   3. Create the lifecycle hook. You can create a hook that acts when instances launch or when instances terminate.
	///   4. **If necessary, record the lifecycle action heartbeat to keep the instance in a pending state.**
	///   5. Complete the lifecycle action.
	/// For more information, see [Auto Scaling Pending State](http://docs.aws.amazon.
	/// com/AutoScaling/latest/DeveloperGuide/AutoScalingPendingState.html) and [Auto
	/// Scaling Terminating State](http://docs.aws.amazon.com/AutoScaling/latest/Devel
	/// operGuide/AutoScalingTerminatingState.html) in the _Auto Scaling Developer
	/// Guide_.
	pub fn record_lifecycle_action_heartbeat(&self, input: &RecordLifecycleActionHeartbeatType) -> Result<RecordLifecycleActionHeartbeatAnswer, AWSError> {
		let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "RecordLifecycleActionHeartbeat");
		RecordLifecycleActionHeartbeatTypeWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(RecordLifecycleActionHeartbeatAnswerParser::parse_xml("RecordLifecycleActionHeartbeatAnswer", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Configures an Auto Scaling group to send notifications when specified events
	/// take place. Subscribers to this topic can have messages for events delivered
	/// to an endpoint such as a web server or email address.
	/// For more information see [Getting Notifications When Your Auto Scaling Group C
	/// hanges](http://docs.aws.amazon.com/AutoScaling/latest/DeveloperGuide/ASGetting
	/// Notifications.html) in the _Auto Scaling Developer Guide_.
	/// This configuration overwrites an existing configuration.
	pub fn put_notification_configuration(&self, input: &PutNotificationConfigurationType) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "PutNotificationConfiguration");
		PutNotificationConfigurationTypeWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(())
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Resumes the specified suspended Auto Scaling processes for the specified Auto
	/// Scaling group. To resume specific processes, use the `ScalingProcesses`
	/// parameter. To resume all processes, omit the `ScalingProcesses` parameter. For
	/// more information, see [Suspend and Resume Auto Scaling Processes](http://docs.
	/// aws.amazon.com/AutoScaling/latest/DeveloperGuide/US_SuspendResume.html) in the
	/// _Auto Scaling Developer Guide_.
	pub fn resume_processes(&self, input: &ScalingProcessQuery) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "ResumeProcesses");
		ScalingProcessQueryWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(())
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Creates a launch configuration.
	/// If you exceed your maximum limit of launch configurations, which by default is
	/// 100 per region, the call fails. For information about viewing and updating
	/// this limit, see DescribeAccountLimits.
	/// For more information, see [Launch Configurations](http://docs.aws.amazon.com/A
	/// utoScaling/latest/DeveloperGuide/LaunchConfiguration.html) in the _Auto
	/// Scaling Developer Guide_.
	pub fn create_launch_configuration(&self, input: &CreateLaunchConfigurationType) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "CreateLaunchConfiguration");
		CreateLaunchConfigurationTypeWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(())
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Describes one or more Auto Scaling instances. If a list is not provided, the
	/// call describes all instances.
	pub fn describe_auto_scaling_instances(&self, input: &DescribeAutoScalingInstancesType) -> Result<AutoScalingInstancesType, AWSError> {
		let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "DescribeAutoScalingInstances");
		DescribeAutoScalingInstancesTypeWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(AutoScalingInstancesTypeParser::parse_xml("AutoScalingInstancesType", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Describes the specified tags.
	/// You can use filters to limit the results. For example, you can query for the
	/// tags for a specific Auto Scaling group. You can specify multiple values for a
	/// filter. A tag must match at least one of the specified values for it to be
	/// included in the results.
	/// You can also specify multiple filters. The result includes information for a
	/// particular tag only if it matches all the filters. If there's no match, no
	/// special message is returned.
	pub fn describe_tags(&self, input: &DescribeTagsType) -> Result<TagsType, AWSError> {
		let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "DescribeTags");
		DescribeTagsTypeWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(TagsTypeParser::parse_xml("TagsType", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Disables monitoring of the specified metrics for the specified Auto Scaling
	/// group.
	pub fn disable_metrics_collection(&self, input: &DisableMetricsCollectionQuery) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "DisableMetricsCollection");
		DisableMetricsCollectionQueryWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(())
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Sets the size of the specified Auto Scaling group.
	/// For more information about desired capacity, see [What Is Auto Scaling?](http:
	/// //docs.aws.amazon.com/AutoScaling/latest/DeveloperGuide/WhatIsAutoScaling.html
	/// ) in the _Auto Scaling Developer Guide_.
	pub fn set_desired_capacity(&self, input: &SetDesiredCapacityType) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "SetDesiredCapacity");
		SetDesiredCapacityTypeWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(())
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Describes one or more launch configurations. If you omit the list of names,
	/// then the call describes all launch configurations.
	pub fn describe_launch_configurations(&self, input: &LaunchConfigurationNamesType) -> Result<LaunchConfigurationsType, AWSError> {
		let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "DescribeLaunchConfigurations");
		LaunchConfigurationNamesTypeWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(LaunchConfigurationsTypeParser::parse_xml("LaunchConfigurationsType", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Moves the specified instances into `Standby` mode.
	/// For more information, see [Auto Scaling InService State](http://docs.aws.amazo
	/// n.com/AutoScaling/latest/DeveloperGuide/AutoScalingInServiceState.html) in the
	/// _Auto Scaling Developer Guide_.
	pub fn enter_standby(&self, input: &EnterStandbyQuery) -> Result<EnterStandbyAnswer, AWSError> {
		let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "EnterStandby");
		EnterStandbyQueryWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(EnterStandbyAnswerParser::parse_xml("EnterStandbyAnswer", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Deletes the specified launch configuration.
	/// The launch configuration must not be attached to an Auto Scaling group. When
	/// this call completes, the launch configuration is no longer available for use.
	pub fn delete_launch_configuration(&self, input: &LaunchConfigurationNameType) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "DeleteLaunchConfiguration");
		LaunchConfigurationNameTypeWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(())
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Deletes the specified lifecycle hook.
	/// If there are any outstanding lifecycle actions, they are completed first
	/// (`ABANDON` for launching instances, `CONTINUE` for terminating instances).
	pub fn delete_lifecycle_hook(&self, input: &DeleteLifecycleHookType) -> Result<DeleteLifecycleHookAnswer, AWSError> {
		let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "DeleteLifecycleHook");
		DeleteLifecycleHookTypeWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(DeleteLifecycleHookAnswerParser::parse_xml("DeleteLifecycleHookAnswer", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Describes the policies for the specified Auto Scaling group.
	pub fn describe_policies(&self, input: &DescribePoliciesType) -> Result<PoliciesType, AWSError> {
		let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "DescribePolicies");
		DescribePoliciesTypeWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(PoliciesTypeParser::parse_xml("PoliciesType", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Describes the lifecycle hooks for the specified Auto Scaling group.
	pub fn describe_lifecycle_hooks(&self, input: &DescribeLifecycleHooksType) -> Result<DescribeLifecycleHooksAnswer, AWSError> {
		let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "DescribeLifecycleHooks");
		DescribeLifecycleHooksTypeWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(DescribeLifecycleHooksAnswerParser::parse_xml("DescribeLifecycleHooksAnswer", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Describes the actions scheduled for your Auto Scaling group that haven't run.
	/// To describe the actions that have already run, use DescribeScalingActivities.
	pub fn describe_scheduled_actions(&self, input: &DescribeScheduledActionsType) -> Result<ScheduledActionsType, AWSError> {
		let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "DescribeScheduledActions");
		DescribeScheduledActionsTypeWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(ScheduledActionsTypeParser::parse_xml("ScheduledActionsType", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Describes one or more Auto Scaling groups. If a list of names is not provided,
	/// the call describes all Auto Scaling groups.
	pub fn describe_auto_scaling_groups(&self, input: &AutoScalingGroupNamesType) -> Result<AutoScalingGroupsType, AWSError> {
		let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "DescribeAutoScalingGroups");
		AutoScalingGroupNamesTypeWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(AutoScalingGroupsTypeParser::parse_xml("AutoScalingGroupsType", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Enables monitoring of the specified metrics for the specified Auto Scaling
	/// group.
	/// You can only enable metrics collection if `InstanceMonitoring` in the launch
	/// configuration for the group is set to `True`.
	pub fn enable_metrics_collection(&self, input: &EnableMetricsCollectionQuery) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "EnableMetricsCollection");
		EnableMetricsCollectionQueryWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(())
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Deletes the specified notification.
	pub fn delete_notification_configuration(&self, input: &DeleteNotificationConfigurationType) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "DeleteNotificationConfiguration");
		DeleteNotificationConfigurationTypeWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(())
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Suspends the specified Auto Scaling processes for the specified Auto Scaling
	/// group. To suspend specific processes, use the `ScalingProcesses` parameter. To
	/// suspend all processes, omit the `ScalingProcesses` parameter.
	/// Note that if you suspend either the `Launch` or `Terminate` process types, it
	/// can prevent other process types from functioning properly.
	/// To resume processes that have been suspended, use ResumeProcesses.
	/// For more information, see [Suspend and Resume Auto Scaling Processes](http://d
	/// ocs.aws.amazon.com/AutoScaling/latest/DeveloperGuide/US_SuspendResume.html) in
	/// the _Auto Scaling Developer Guide_.
	pub fn suspend_processes(&self, input: &ScalingProcessQuery) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "autoscaling", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "SuspendProcesses");
		ScalingProcessQueryWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(())
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
}
