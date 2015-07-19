use std::collections::HashMap;
use std::str;
pub type StackId = String;
/// Parse StackId from XML
struct StackIdParser;
impl StackIdParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<StackId, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write StackId contents to a SignedRequest
struct StackIdWriter;
impl StackIdWriter {
	fn write_params(params: &mut Params, name: &str, obj: &StackId) {
		params.put(name, obj);
	}
}
pub type TemplateDescription = String;
/// Parse TemplateDescription from XML
struct TemplateDescriptionParser;
impl TemplateDescriptionParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<TemplateDescription, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write TemplateDescription contents to a SignedRequest
struct TemplateDescriptionWriter;
impl TemplateDescriptionWriter {
	fn write_params(params: &mut Params, name: &str, obj: &TemplateDescription) {
		params.put(name, obj);
	}
}
pub type ResourceType = String;
/// Parse ResourceType from XML
struct ResourceTypeParser;
impl ResourceTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ResourceType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ResourceType contents to a SignedRequest
struct ResourceTypeWriter;
impl ResourceTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ResourceType) {
		params.put(name, obj);
	}
}
pub type CreationTime = String;
/// Parse CreationTime from XML
struct CreationTimeParser;
impl CreationTimeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<CreationTime, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write CreationTime contents to a SignedRequest
struct CreationTimeWriter;
impl CreationTimeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &CreationTime) {
		params.put(name, obj);
	}
}
pub type UsePreviousValue = bool;
/// Parse UsePreviousValue from XML
struct UsePreviousValueParser;
impl UsePreviousValueParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<UsePreviousValue, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = bool::from_str(try!(characters(stack)).as_ref()).unwrap();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write UsePreviousValue contents to a SignedRequest
struct UsePreviousValueWriter;
impl UsePreviousValueWriter {
	fn write_params(params: &mut Params, name: &str, obj: &UsePreviousValue) {
		params.put(name, &obj.to_string());
	}
}
pub type NotificationARNs = Vec<NotificationARN>;
/// Parse NotificationARNs from XML
struct NotificationARNsParser;
impl NotificationARNsParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<NotificationARNs, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "NotificationARN" {
			obj.push(try!(NotificationARNParser::parse_xml("NotificationARN", stack)));
		}
		Ok(obj)
	}
}
/// Write NotificationARNs contents to a SignedRequest
struct NotificationARNsWriter;
impl NotificationARNsWriter {
	fn write_params(params: &mut Params, name: &str, obj: &NotificationARNs) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			NotificationARNWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
/// The StackResource data type.
#[derive(Debug, Default)]
pub struct StackResource {
	/// Unique identifier of the stack.
	pub stack_id: Option<StackId>,
	/// Current status of the resource.
	pub resource_status: ResourceStatus,
	/// User defined description associated with the resource.
	pub description: Option<Description>,
	/// Type of resource. (For more information, go to [ AWS Resource Types
	/// Reference](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-
	/// template-resource-type-ref.html) in the AWS CloudFormation User Guide.)
	pub resource_type: ResourceType,
	/// Time the status was updated.
	pub timestamp: Timestamp,
	/// Success/failure message associated with the resource.
	pub resource_status_reason: Option<ResourceStatusReason>,
	/// The name associated with the stack.
	pub stack_name: Option<StackName>,
	/// The name or unique identifier that corresponds to a physical instance ID of a
	/// resource supported by AWS CloudFormation.
	pub physical_resource_id: Option<PhysicalResourceId>,
	/// The logical name of the resource specified in the template.
	pub logical_resource_id: LogicalResourceId,
}

/// Parse StackResource from XML
struct StackResourceParser;
impl StackResourceParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<StackResource, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = StackResource::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "StackId" {
				obj.stack_id = Some(try!(StackIdParser::parse_xml("StackId", stack)));
				continue;
			}
			if current_name == "ResourceStatus" {
				obj.resource_status = try!(ResourceStatusParser::parse_xml("ResourceStatus", stack));
				continue;
			}
			if current_name == "Description" {
				obj.description = Some(try!(DescriptionParser::parse_xml("Description", stack)));
				continue;
			}
			if current_name == "ResourceType" {
				obj.resource_type = try!(ResourceTypeParser::parse_xml("ResourceType", stack));
				continue;
			}
			if current_name == "Timestamp" {
				obj.timestamp = try!(TimestampParser::parse_xml("Timestamp", stack));
				continue;
			}
			if current_name == "ResourceStatusReason" {
				obj.resource_status_reason = Some(try!(ResourceStatusReasonParser::parse_xml("ResourceStatusReason", stack)));
				continue;
			}
			if current_name == "StackName" {
				obj.stack_name = Some(try!(StackNameParser::parse_xml("StackName", stack)));
				continue;
			}
			if current_name == "PhysicalResourceId" {
				obj.physical_resource_id = Some(try!(PhysicalResourceIdParser::parse_xml("PhysicalResourceId", stack)));
				continue;
			}
			if current_name == "LogicalResourceId" {
				obj.logical_resource_id = try!(LogicalResourceIdParser::parse_xml("LogicalResourceId", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write StackResource contents to a SignedRequest
struct StackResourceWriter;
impl StackResourceWriter {
	fn write_params(params: &mut Params, name: &str, obj: &StackResource) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		if let Some(ref obj) = obj.stack_id {
			StackIdWriter::write_params(params, &(prefix.to_string() + "StackId"), obj);
		}
		ResourceStatusWriter::write_params(params, &(prefix.to_string() + "ResourceStatus"), &obj.resource_status);
		if let Some(ref obj) = obj.description {
			DescriptionWriter::write_params(params, &(prefix.to_string() + "Description"), obj);
		}
		ResourceTypeWriter::write_params(params, &(prefix.to_string() + "ResourceType"), &obj.resource_type);
		TimestampWriter::write_params(params, &(prefix.to_string() + "Timestamp"), &obj.timestamp);
		if let Some(ref obj) = obj.resource_status_reason {
			ResourceStatusReasonWriter::write_params(params, &(prefix.to_string() + "ResourceStatusReason"), obj);
		}
		if let Some(ref obj) = obj.stack_name {
			StackNameWriter::write_params(params, &(prefix.to_string() + "StackName"), obj);
		}
		if let Some(ref obj) = obj.physical_resource_id {
			PhysicalResourceIdWriter::write_params(params, &(prefix.to_string() + "PhysicalResourceId"), obj);
		}
		LogicalResourceIdWriter::write_params(params, &(prefix.to_string() + "LogicalResourceId"), &obj.logical_resource_id);
	}
}
/// The input for the ListStackResource action.
#[derive(Debug, Default)]
pub struct ListStackResourcesInput {
	/// The name or the unique stack ID that is associated with the stack, which are
	/// not always interchangeable:
	///   * Running stacks: You can specify either the stack's name or its unique stack ID.
	///   * Deleted stacks: You must specify the unique stack ID.
	/// Default: There is no default value.
	pub stack_name: StackName,
	/// String that identifies the start of the next list of stack resource summaries,
	/// if there is one.
	/// Default: There is no default value.
	pub next_token: Option<NextToken>,
}

/// Parse ListStackResourcesInput from XML
struct ListStackResourcesInputParser;
impl ListStackResourcesInputParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ListStackResourcesInput, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = ListStackResourcesInput::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "StackName" {
				obj.stack_name = try!(StackNameParser::parse_xml("StackName", stack));
				continue;
			}
			if current_name == "NextToken" {
				obj.next_token = Some(try!(NextTokenParser::parse_xml("NextToken", stack)));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ListStackResourcesInput contents to a SignedRequest
struct ListStackResourcesInputWriter;
impl ListStackResourcesInputWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ListStackResourcesInput) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		StackNameWriter::write_params(params, &(prefix.to_string() + "StackName"), &obj.stack_name);
		if let Some(ref obj) = obj.next_token {
			NextTokenWriter::write_params(params, &(prefix.to_string() + "NextToken"), obj);
		}
	}
}
pub type Metadata = String;
/// Parse Metadata from XML
struct MetadataParser;
impl MetadataParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<Metadata, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write Metadata contents to a SignedRequest
struct MetadataWriter;
impl MetadataWriter {
	fn write_params(params: &mut Params, name: &str, obj: &Metadata) {
		params.put(name, obj);
	}
}
/// A set of criteria that AWS CloudFormation uses to validate parameter values.
/// Although other constraints might be defined in the stack template, AWS
/// CloudFormation returns only the `AllowedValues` property.
#[derive(Debug, Default)]
pub struct ParameterConstraints {
	/// A list of values that are permitted for a parameter.
	pub allowed_values: AllowedValues,
}

/// Parse ParameterConstraints from XML
struct ParameterConstraintsParser;
impl ParameterConstraintsParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ParameterConstraints, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = ParameterConstraints::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "AllowedValue" {
				obj.allowed_values = try!(AllowedValuesParser::parse_xml("AllowedValue", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ParameterConstraints contents to a SignedRequest
struct ParameterConstraintsWriter;
impl ParameterConstraintsWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ParameterConstraints) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		AllowedValuesWriter::write_params(params, &(prefix.to_string() + "AllowedValue"), &obj.allowed_values);
	}
}
pub type CapabilitiesReason = String;
/// Parse CapabilitiesReason from XML
struct CapabilitiesReasonParser;
impl CapabilitiesReasonParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<CapabilitiesReason, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write CapabilitiesReason contents to a SignedRequest
struct CapabilitiesReasonWriter;
impl CapabilitiesReasonWriter {
	fn write_params(params: &mut Params, name: &str, obj: &CapabilitiesReason) {
		params.put(name, obj);
	}
}
/// The input for the GetTemplateSummary action.
#[derive(Debug, Default)]
pub struct GetTemplateSummaryInput {
	/// The name or the stack ID that is associated with the stack, which are not
	/// always interchangeable. For running stacks, you can specify either the stack's
	/// name or its unique stack ID. For deleted stack, you must specify the unique
	/// stack ID.
	/// Conditional: You must specify only one of the following parameters:
	/// `StackName`, `TemplateBody`, or `TemplateURL`.
	pub stack_name: StackNameOrId,
	/// Location of file containing the template body. The URL must point to a
	/// template (max size: 460,800 bytes) located in an Amazon S3 bucket. For more
	/// information about templates, see [Template
	/// Anatomy](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide
	/// /template-anatomy.html) in the AWS CloudFormation User Guide.
	/// Conditional: You must specify only one of the following parameters:
	/// `StackName`, `TemplateBody`, or `TemplateURL`.
	pub template_url: TemplateURL,
	/// Structure containing the template body with a minimum length of 1 byte and a
	/// maximum length of 51,200 bytes. For more information about templates, see
	/// [Template
	/// Anatomy](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide
	/// /template-anatomy.html) in the AWS CloudFormation User Guide.
	/// Conditional: You must specify only one of the following parameters:
	/// `StackName`, `TemplateBody`, or `TemplateURL`.
	pub template_body: TemplateBody,
}

/// Parse GetTemplateSummaryInput from XML
struct GetTemplateSummaryInputParser;
impl GetTemplateSummaryInputParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<GetTemplateSummaryInput, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = GetTemplateSummaryInput::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "StackName" {
				obj.stack_name = try!(StackNameOrIdParser::parse_xml("StackName", stack));
				continue;
			}
			if current_name == "TemplateURL" {
				obj.template_url = try!(TemplateURLParser::parse_xml("TemplateURL", stack));
				continue;
			}
			if current_name == "TemplateBody" {
				obj.template_body = try!(TemplateBodyParser::parse_xml("TemplateBody", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write GetTemplateSummaryInput contents to a SignedRequest
struct GetTemplateSummaryInputWriter;
impl GetTemplateSummaryInputWriter {
	fn write_params(params: &mut Params, name: &str, obj: &GetTemplateSummaryInput) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		StackNameOrIdWriter::write_params(params, &(prefix.to_string() + "StackName"), &obj.stack_name);
		TemplateURLWriter::write_params(params, &(prefix.to_string() + "TemplateURL"), &obj.template_url);
		TemplateBodyWriter::write_params(params, &(prefix.to_string() + "TemplateBody"), &obj.template_body);
	}
}
pub type TimeoutMinutes = i32;
/// Parse TimeoutMinutes from XML
struct TimeoutMinutesParser;
impl TimeoutMinutesParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<TimeoutMinutes, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = i32::from_str(try!(characters(stack)).as_ref()).unwrap();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write TimeoutMinutes contents to a SignedRequest
struct TimeoutMinutesWriter;
impl TimeoutMinutesWriter {
	fn write_params(params: &mut Params, name: &str, obj: &TimeoutMinutes) {
		params.put(name, &obj.to_string());
	}
}
pub type StackEvents = Vec<StackEvent>;
/// Parse StackEvents from XML
struct StackEventsParser;
impl StackEventsParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<StackEvents, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "StackEvent" {
			obj.push(try!(StackEventParser::parse_xml("StackEvent", stack)));
		}
		Ok(obj)
	}
}
/// Write StackEvents contents to a SignedRequest
struct StackEventsWriter;
impl StackEventsWriter {
	fn write_params(params: &mut Params, name: &str, obj: &StackEvents) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			StackEventWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
pub type TemplateURL = String;
/// Parse TemplateURL from XML
struct TemplateURLParser;
impl TemplateURLParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<TemplateURL, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write TemplateURL contents to a SignedRequest
struct TemplateURLWriter;
impl TemplateURLWriter {
	fn write_params(params: &mut Params, name: &str, obj: &TemplateURL) {
		params.put(name, obj);
	}
}
pub type StackStatusFilter = Vec<StackStatus>;
/// Parse StackStatusFilter from XML
struct StackStatusFilterParser;
impl StackStatusFilterParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<StackStatusFilter, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "StackStatus" {
			obj.push(try!(StackStatusParser::parse_xml("StackStatus", stack)));
		}
		Ok(obj)
	}
}
/// Write StackStatusFilter contents to a SignedRequest
struct StackStatusFilterWriter;
impl StackStatusFilterWriter {
	fn write_params(params: &mut Params, name: &str, obj: &StackStatusFilter) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			StackStatusWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
pub type StackSummaries = Vec<StackSummary>;
/// Parse StackSummaries from XML
struct StackSummariesParser;
impl StackSummariesParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<StackSummaries, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "StackSummary" {
			obj.push(try!(StackSummaryParser::parse_xml("StackSummary", stack)));
		}
		Ok(obj)
	}
}
/// Write StackSummaries contents to a SignedRequest
struct StackSummariesWriter;
impl StackSummariesWriter {
	fn write_params(params: &mut Params, name: &str, obj: &StackSummaries) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			StackSummaryWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
/// The StackSummary Data Type
#[derive(Debug, Default)]
pub struct StackSummary {
	/// Unique stack identifier.
	pub stack_id: Option<StackId>,
	/// The time the stack was deleted.
	pub deletion_time: Option<DeletionTime>,
	/// The template description of the template used to create the stack.
	pub template_description: Option<TemplateDescription>,
	/// Success/Failure message associated with the stack status.
	pub stack_status_reason: Option<StackStatusReason>,
	/// The time the stack was created.
	pub creation_time: CreationTime,
	/// The name associated with the stack.
	pub stack_name: StackName,
	/// The current status of the stack.
	pub stack_status: StackStatus,
	/// The time the stack was last updated. This field will only be returned if the
	/// stack has been updated at least once.
	pub last_updated_time: Option<LastUpdatedTime>,
}

/// Parse StackSummary from XML
struct StackSummaryParser;
impl StackSummaryParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<StackSummary, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = StackSummary::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "StackId" {
				obj.stack_id = Some(try!(StackIdParser::parse_xml("StackId", stack)));
				continue;
			}
			if current_name == "DeletionTime" {
				obj.deletion_time = Some(try!(DeletionTimeParser::parse_xml("DeletionTime", stack)));
				continue;
			}
			if current_name == "TemplateDescription" {
				obj.template_description = Some(try!(TemplateDescriptionParser::parse_xml("TemplateDescription", stack)));
				continue;
			}
			if current_name == "StackStatusReason" {
				obj.stack_status_reason = Some(try!(StackStatusReasonParser::parse_xml("StackStatusReason", stack)));
				continue;
			}
			if current_name == "CreationTime" {
				obj.creation_time = try!(CreationTimeParser::parse_xml("CreationTime", stack));
				continue;
			}
			if current_name == "StackName" {
				obj.stack_name = try!(StackNameParser::parse_xml("StackName", stack));
				continue;
			}
			if current_name == "StackStatus" {
				obj.stack_status = try!(StackStatusParser::parse_xml("StackStatus", stack));
				continue;
			}
			if current_name == "LastUpdatedTime" {
				obj.last_updated_time = Some(try!(LastUpdatedTimeParser::parse_xml("LastUpdatedTime", stack)));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write StackSummary contents to a SignedRequest
struct StackSummaryWriter;
impl StackSummaryWriter {
	fn write_params(params: &mut Params, name: &str, obj: &StackSummary) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		if let Some(ref obj) = obj.stack_id {
			StackIdWriter::write_params(params, &(prefix.to_string() + "StackId"), obj);
		}
		if let Some(ref obj) = obj.deletion_time {
			DeletionTimeWriter::write_params(params, &(prefix.to_string() + "DeletionTime"), obj);
		}
		if let Some(ref obj) = obj.template_description {
			TemplateDescriptionWriter::write_params(params, &(prefix.to_string() + "TemplateDescription"), obj);
		}
		if let Some(ref obj) = obj.stack_status_reason {
			StackStatusReasonWriter::write_params(params, &(prefix.to_string() + "StackStatusReason"), obj);
		}
		CreationTimeWriter::write_params(params, &(prefix.to_string() + "CreationTime"), &obj.creation_time);
		StackNameWriter::write_params(params, &(prefix.to_string() + "StackName"), &obj.stack_name);
		StackStatusWriter::write_params(params, &(prefix.to_string() + "StackStatus"), &obj.stack_status);
		if let Some(ref obj) = obj.last_updated_time {
			LastUpdatedTimeWriter::write_params(params, &(prefix.to_string() + "LastUpdatedTime"), obj);
		}
	}
}
pub type StackStatus = String;
/// Parse StackStatus from XML
struct StackStatusParser;
impl StackStatusParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<StackStatus, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write StackStatus contents to a SignedRequest
struct StackStatusWriter;
impl StackStatusWriter {
	fn write_params(params: &mut Params, name: &str, obj: &StackStatus) {
		params.put(name, obj);
	}
}
/// Contains detailed information about the specified stack resource.
#[derive(Debug, Default)]
pub struct StackResourceDetail {
	/// Unique identifier of the stack.
	pub stack_id: Option<StackId>,
	/// Current status of the resource.
	pub resource_status: ResourceStatus,
	/// User defined description associated with the resource.
	pub description: Option<Description>,
	/// Type of resource. ((For more information, go to [ AWS Resource Types
	/// Reference](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-
	/// template-resource-type-ref.html) in the AWS CloudFormation User Guide.)
	pub resource_type: ResourceType,
	/// Success/failure message associated with the resource.
	pub resource_status_reason: Option<ResourceStatusReason>,
	/// Time the status was updated.
	pub last_updated_timestamp: Timestamp,
	/// The name associated with the stack.
	pub stack_name: Option<StackName>,
	/// The name or unique identifier that corresponds to a physical instance ID of a
	/// resource supported by AWS CloudFormation.
	pub physical_resource_id: Option<PhysicalResourceId>,
	/// The JSON format content of the `Metadata` attribute declared for the resource.
	/// For more information, see [Metadata
	/// Attribute](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-
	/// attribute-metadata.html) in the AWS CloudFormation User Guide.
	pub metadata: Option<Metadata>,
	/// The logical name of the resource specified in the template.
	pub logical_resource_id: LogicalResourceId,
}

/// Parse StackResourceDetail from XML
struct StackResourceDetailParser;
impl StackResourceDetailParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<StackResourceDetail, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = StackResourceDetail::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "StackId" {
				obj.stack_id = Some(try!(StackIdParser::parse_xml("StackId", stack)));
				continue;
			}
			if current_name == "ResourceStatus" {
				obj.resource_status = try!(ResourceStatusParser::parse_xml("ResourceStatus", stack));
				continue;
			}
			if current_name == "Description" {
				obj.description = Some(try!(DescriptionParser::parse_xml("Description", stack)));
				continue;
			}
			if current_name == "ResourceType" {
				obj.resource_type = try!(ResourceTypeParser::parse_xml("ResourceType", stack));
				continue;
			}
			if current_name == "ResourceStatusReason" {
				obj.resource_status_reason = Some(try!(ResourceStatusReasonParser::parse_xml("ResourceStatusReason", stack)));
				continue;
			}
			if current_name == "LastUpdatedTimestamp" {
				obj.last_updated_timestamp = try!(TimestampParser::parse_xml("LastUpdatedTimestamp", stack));
				continue;
			}
			if current_name == "StackName" {
				obj.stack_name = Some(try!(StackNameParser::parse_xml("StackName", stack)));
				continue;
			}
			if current_name == "PhysicalResourceId" {
				obj.physical_resource_id = Some(try!(PhysicalResourceIdParser::parse_xml("PhysicalResourceId", stack)));
				continue;
			}
			if current_name == "Metadata" {
				obj.metadata = Some(try!(MetadataParser::parse_xml("Metadata", stack)));
				continue;
			}
			if current_name == "LogicalResourceId" {
				obj.logical_resource_id = try!(LogicalResourceIdParser::parse_xml("LogicalResourceId", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write StackResourceDetail contents to a SignedRequest
struct StackResourceDetailWriter;
impl StackResourceDetailWriter {
	fn write_params(params: &mut Params, name: &str, obj: &StackResourceDetail) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		if let Some(ref obj) = obj.stack_id {
			StackIdWriter::write_params(params, &(prefix.to_string() + "StackId"), obj);
		}
		ResourceStatusWriter::write_params(params, &(prefix.to_string() + "ResourceStatus"), &obj.resource_status);
		if let Some(ref obj) = obj.description {
			DescriptionWriter::write_params(params, &(prefix.to_string() + "Description"), obj);
		}
		ResourceTypeWriter::write_params(params, &(prefix.to_string() + "ResourceType"), &obj.resource_type);
		if let Some(ref obj) = obj.resource_status_reason {
			ResourceStatusReasonWriter::write_params(params, &(prefix.to_string() + "ResourceStatusReason"), obj);
		}
		TimestampWriter::write_params(params, &(prefix.to_string() + "LastUpdatedTimestamp"), &obj.last_updated_timestamp);
		if let Some(ref obj) = obj.stack_name {
			StackNameWriter::write_params(params, &(prefix.to_string() + "StackName"), obj);
		}
		if let Some(ref obj) = obj.physical_resource_id {
			PhysicalResourceIdWriter::write_params(params, &(prefix.to_string() + "PhysicalResourceId"), obj);
		}
		if let Some(ref obj) = obj.metadata {
			MetadataWriter::write_params(params, &(prefix.to_string() + "Metadata"), obj);
		}
		LogicalResourceIdWriter::write_params(params, &(prefix.to_string() + "LogicalResourceId"), &obj.logical_resource_id);
	}
}
/// The input for DeleteStack action.
#[derive(Debug, Default)]
pub struct DeleteStackInput {
	/// The name or the unique stack ID that is associated with the stack.
	pub stack_name: StackName,
}

/// Parse DeleteStackInput from XML
struct DeleteStackInputParser;
impl DeleteStackInputParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<DeleteStackInput, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = DeleteStackInput::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "StackName" {
				obj.stack_name = try!(StackNameParser::parse_xml("StackName", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write DeleteStackInput contents to a SignedRequest
struct DeleteStackInputWriter;
impl DeleteStackInputWriter {
	fn write_params(params: &mut Params, name: &str, obj: &DeleteStackInput) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		StackNameWriter::write_params(params, &(prefix.to_string() + "StackName"), &obj.stack_name);
	}
}
/// The Tag type is used by `CreateStack` in the `Tags` parameter. It allows you
/// to specify a key/value pair that can be used to store information related to
/// cost allocation for an AWS CloudFormation stack.
#[derive(Debug, Default)]
pub struct Tag {
	/// _Required_. A string containing the value for this tag. You can specify a
	/// maximum of 256 characters for a tag value.
	pub value: TagValue,
	/// _Required_. A string used to identify this tag. You can specify a maximum of
	/// 128 characters for a tag key. Tags owned by Amazon Web Services (AWS) have the
	/// reserved prefix: `aws:`.
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
/// Write Tag contents to a SignedRequest
struct TagWriter;
impl TagWriter {
	fn write_params(params: &mut Params, name: &str, obj: &Tag) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		TagValueWriter::write_params(params, &(prefix.to_string() + "Value"), &obj.value);
		TagKeyWriter::write_params(params, &(prefix.to_string() + "Key"), &obj.key);
	}
}
/// The output for a DescribeStackEvents action.
#[derive(Debug, Default)]
pub struct DescribeStackEventsOutput {
	/// A list of `StackEvents` structures.
	pub stack_events: StackEvents,
	/// String that identifies the start of the next list of events, if there is one.
	pub next_token: NextToken,
}

/// Parse DescribeStackEventsOutput from XML
struct DescribeStackEventsOutputParser;
impl DescribeStackEventsOutputParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<DescribeStackEventsOutput, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = DescribeStackEventsOutput::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "StackEvent" {
				obj.stack_events = try!(StackEventsParser::parse_xml("StackEvent", stack));
				continue;
			}
			if current_name == "NextToken" {
				obj.next_token = try!(NextTokenParser::parse_xml("NextToken", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write DescribeStackEventsOutput contents to a SignedRequest
struct DescribeStackEventsOutputWriter;
impl DescribeStackEventsOutputWriter {
	fn write_params(params: &mut Params, name: &str, obj: &DescribeStackEventsOutput) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		StackEventsWriter::write_params(params, &(prefix.to_string() + "StackEvent"), &obj.stack_events);
		NextTokenWriter::write_params(params, &(prefix.to_string() + "NextToken"), &obj.next_token);
	}
}
/// Quota for the resource has already been reached.
#[derive(Debug, Default)]
pub struct LimitExceededException;

/// Parse LimitExceededException from XML
struct LimitExceededExceptionParser;
impl LimitExceededExceptionParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<LimitExceededException, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = LimitExceededException::default();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write LimitExceededException contents to a SignedRequest
struct LimitExceededExceptionWriter;
impl LimitExceededExceptionWriter {
	fn write_params(params: &mut Params, name: &str, obj: &LimitExceededException) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
	}
}
pub type Parameters = Vec<Parameter>;
/// Parse Parameters from XML
struct ParametersParser;
impl ParametersParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<Parameters, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "Parameter" {
			obj.push(try!(ParameterParser::parse_xml("Parameter", stack)));
		}
		Ok(obj)
	}
}
/// Write Parameters contents to a SignedRequest
struct ParametersWriter;
impl ParametersWriter {
	fn write_params(params: &mut Params, name: &str, obj: &Parameters) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			ParameterWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
/// The TemplateParameter data type.
#[derive(Debug, Default)]
pub struct TemplateParameter {
	/// The default value associated with the parameter.
	pub default_value: ParameterValue,
	/// Flag indicating whether the parameter should be displayed as plain text in
	/// logs and UIs.
	pub no_echo: NoEcho,
	/// User defined description associated with the parameter.
	pub description: Description,
	/// The name associated with the parameter.
	pub parameter_key: ParameterKey,
}

/// Parse TemplateParameter from XML
struct TemplateParameterParser;
impl TemplateParameterParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<TemplateParameter, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = TemplateParameter::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "DefaultValue" {
				obj.default_value = try!(ParameterValueParser::parse_xml("DefaultValue", stack));
				continue;
			}
			if current_name == "NoEcho" {
				obj.no_echo = try!(NoEchoParser::parse_xml("NoEcho", stack));
				continue;
			}
			if current_name == "Description" {
				obj.description = try!(DescriptionParser::parse_xml("Description", stack));
				continue;
			}
			if current_name == "ParameterKey" {
				obj.parameter_key = try!(ParameterKeyParser::parse_xml("ParameterKey", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write TemplateParameter contents to a SignedRequest
struct TemplateParameterWriter;
impl TemplateParameterWriter {
	fn write_params(params: &mut Params, name: &str, obj: &TemplateParameter) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		ParameterValueWriter::write_params(params, &(prefix.to_string() + "DefaultValue"), &obj.default_value);
		NoEchoWriter::write_params(params, &(prefix.to_string() + "NoEcho"), &obj.no_echo);
		DescriptionWriter::write_params(params, &(prefix.to_string() + "Description"), &obj.description);
		ParameterKeyWriter::write_params(params, &(prefix.to_string() + "ParameterKey"), &obj.parameter_key);
	}
}
/// The output for ListStacks action.
#[derive(Debug, Default)]
pub struct ListStacksOutput {
	/// A list of `StackSummary` structures containing information about the specified
	/// stacks.
	pub stack_summaries: StackSummaries,
	/// String that identifies the start of the next list of stacks, if there is one.
	pub next_token: NextToken,
}

/// Parse ListStacksOutput from XML
struct ListStacksOutputParser;
impl ListStacksOutputParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ListStacksOutput, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = ListStacksOutput::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "StackSummary" {
				obj.stack_summaries = try!(StackSummariesParser::parse_xml("StackSummary", stack));
				continue;
			}
			if current_name == "NextToken" {
				obj.next_token = try!(NextTokenParser::parse_xml("NextToken", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ListStacksOutput contents to a SignedRequest
struct ListStacksOutputWriter;
impl ListStacksOutputWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ListStacksOutput) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		StackSummariesWriter::write_params(params, &(prefix.to_string() + "StackSummary"), &obj.stack_summaries);
		NextTokenWriter::write_params(params, &(prefix.to_string() + "NextToken"), &obj.next_token);
	}
}
/// The input for UpdateStack action.
#[derive(Debug, Default)]
pub struct UpdateStackInput {
	/// Location of a file containing the temporary overriding stack policy. The URL
	/// must point to a policy (max size: 16KB) located in an S3 bucket in the same
	/// region as the stack. You can specify either the `StackPolicyDuringUpdateBody`
	/// or the `StackPolicyDuringUpdateURL` parameter, but not both.
	/// If you want to update protected resources, specify a temporary overriding
	/// stack policy during this update. If you do not specify a stack policy, the
	/// current policy that is associated with the stack will be used.
	pub stack_policy_during_update_url: Option<StackPolicyDuringUpdateURL>,
	/// A list of `Parameter` structures that specify input parameters for the stack.
	/// For more information, see the [Parameter](http://docs.aws.amazon.com/AWSCloudF
	/// ormation/latest/APIReference/API_Parameter.html) data type.
	pub parameters: Option<Parameters>,
	/// Structure containing the template body with a minimum length of 1 byte and a
	/// maximum length of 51,200 bytes. (For more information, go to [Template
	/// Anatomy](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide
	/// /template-anatomy.html) in the AWS CloudFormation User Guide.)
	/// Conditional: You must specify either the `TemplateBody` or the `TemplateURL`
	/// parameter, but not both.
	pub template_body: Option<TemplateBody>,
	/// Location of a file containing the updated stack policy. The URL must point to
	/// a policy (max size: 16KB) located in an S3 bucket in the same region as the
	/// stack. You can specify either the `StackPolicyBody` or the `StackPolicyURL`
	/// parameter, but not both.
	/// You might update the stack policy, for example, in order to protect a new
	/// resource that you created during a stack update. If you do not specify a stack
	/// policy, the current policy that is associated with the stack is unchanged.
	pub stack_policy_url: Option<StackPolicyURL>,
	/// Reuse the existing template that is associated with the stack that you are
	/// updating.
	pub use_previous_template: Option<UsePreviousTemplate>,
	/// Structure containing a new stack policy body. You can specify either the
	/// `StackPolicyBody` or the `StackPolicyURL` parameter, but not both.
	/// You might update the stack policy, for example, in order to protect a new
	/// resource that you created during a stack update. If you do not specify a stack
	/// policy, the current policy that is associated with the stack is unchanged.
	pub stack_policy_body: Option<StackPolicyBody>,
	/// A list of capabilities that you must specify before AWS CloudFormation can
	/// create or update certain stacks. Some stack templates might include resources
	/// that can affect permissions in your AWS account. For those stacks, you must
	/// explicitly acknowledge their capabilities by specifying this parameter.
	/// Currently, the only valid value is `CAPABILITY_IAM`, which is required for the
	/// following resources: [ AWS::IAM::AccessKey](http://docs.aws.amazon.com/AWSClou
	/// dFormation/latest/UserGuide/aws-properties-iam-accesskey.html), [
	/// AWS::IAM::Group](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide
	/// /aws-properties-iam-group.html), [ AWS::IAM::InstanceProfile](http://docs.aws.
	/// amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-
	/// instanceprofile.html), [ AWS::IAM::Policy](http://docs.aws.amazon.com/AWSCloud
	/// Formation/latest/UserGuide/aws-properties-iam-policy.html), [
	/// AWS::IAM::Role](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide
	/// /aws-resource-iam-role.html), [
	/// AWS::IAM::User](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide
	/// /aws-properties-iam-user.html), and [ AWS::IAM::UserToGroupAddition](http://do
	/// cs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-
	/// addusertogroup.html). If your stack template contains these resources, we
	/// recommend that you review any permissions associated with them. If you don't
	/// specify this parameter, this action returns an InsufficientCapabilities error.
	pub capabilities: Option<Capabilities>,
	/// The name or unique stack ID of the stack to update.
	pub stack_name: StackName,
	/// Structure containing the temporary overriding stack policy body. You can
	/// specify either the `StackPolicyDuringUpdateBody` or the
	/// `StackPolicyDuringUpdateURL` parameter, but not both.
	/// If you want to update protected resources, specify a temporary overriding
	/// stack policy during this update. If you do not specify a stack policy, the
	/// current policy that is associated with the stack will be used.
	pub stack_policy_during_update_body: Option<StackPolicyDuringUpdateBody>,
	/// Update the ARNs for the Amazon SNS topics that are associated with the stack.
	pub notification_ar_ns: Option<NotificationARNs>,
	/// Location of file containing the template body. The URL must point to a
	/// template located in an S3 bucket in the same region as the stack. For more
	/// information, go to [Template
	/// Anatomy](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide
	/// /template-anatomy.html) in the AWS CloudFormation User Guide.
	/// Conditional: You must specify either the `TemplateBody` or the `TemplateURL`
	/// parameter, but not both.
	pub template_url: Option<TemplateURL>,
}

/// Parse UpdateStackInput from XML
struct UpdateStackInputParser;
impl UpdateStackInputParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<UpdateStackInput, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = UpdateStackInput::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "StackPolicyDuringUpdateURL" {
				obj.stack_policy_during_update_url = Some(try!(StackPolicyDuringUpdateURLParser::parse_xml("StackPolicyDuringUpdateURL", stack)));
				continue;
			}
			if current_name == "Parameter" {
				obj.parameters = Some(try!(ParametersParser::parse_xml("Parameter", stack)));
				continue;
			}
			if current_name == "TemplateBody" {
				obj.template_body = Some(try!(TemplateBodyParser::parse_xml("TemplateBody", stack)));
				continue;
			}
			if current_name == "StackPolicyURL" {
				obj.stack_policy_url = Some(try!(StackPolicyURLParser::parse_xml("StackPolicyURL", stack)));
				continue;
			}
			if current_name == "UsePreviousTemplate" {
				obj.use_previous_template = Some(try!(UsePreviousTemplateParser::parse_xml("UsePreviousTemplate", stack)));
				continue;
			}
			if current_name == "StackPolicyBody" {
				obj.stack_policy_body = Some(try!(StackPolicyBodyParser::parse_xml("StackPolicyBody", stack)));
				continue;
			}
			if current_name == "Capability" {
				obj.capabilities = Some(try!(CapabilitiesParser::parse_xml("Capability", stack)));
				continue;
			}
			if current_name == "StackName" {
				obj.stack_name = try!(StackNameParser::parse_xml("StackName", stack));
				continue;
			}
			if current_name == "StackPolicyDuringUpdateBody" {
				obj.stack_policy_during_update_body = Some(try!(StackPolicyDuringUpdateBodyParser::parse_xml("StackPolicyDuringUpdateBody", stack)));
				continue;
			}
			if current_name == "NotificationARN" {
				obj.notification_ar_ns = Some(try!(NotificationARNsParser::parse_xml("NotificationARN", stack)));
				continue;
			}
			if current_name == "TemplateURL" {
				obj.template_url = Some(try!(TemplateURLParser::parse_xml("TemplateURL", stack)));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write UpdateStackInput contents to a SignedRequest
struct UpdateStackInputWriter;
impl UpdateStackInputWriter {
	fn write_params(params: &mut Params, name: &str, obj: &UpdateStackInput) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		if let Some(ref obj) = obj.stack_policy_during_update_url {
			StackPolicyDuringUpdateURLWriter::write_params(params, &(prefix.to_string() + "StackPolicyDuringUpdateURL"), obj);
		}
		if let Some(ref obj) = obj.parameters {
			ParametersWriter::write_params(params, &(prefix.to_string() + "Parameter"), obj);
		}
		if let Some(ref obj) = obj.template_body {
			TemplateBodyWriter::write_params(params, &(prefix.to_string() + "TemplateBody"), obj);
		}
		if let Some(ref obj) = obj.stack_policy_url {
			StackPolicyURLWriter::write_params(params, &(prefix.to_string() + "StackPolicyURL"), obj);
		}
		if let Some(ref obj) = obj.use_previous_template {
			UsePreviousTemplateWriter::write_params(params, &(prefix.to_string() + "UsePreviousTemplate"), obj);
		}
		if let Some(ref obj) = obj.stack_policy_body {
			StackPolicyBodyWriter::write_params(params, &(prefix.to_string() + "StackPolicyBody"), obj);
		}
		if let Some(ref obj) = obj.capabilities {
			CapabilitiesWriter::write_params(params, &(prefix.to_string() + "Capability"), obj);
		}
		StackNameWriter::write_params(params, &(prefix.to_string() + "StackName"), &obj.stack_name);
		if let Some(ref obj) = obj.stack_policy_during_update_body {
			StackPolicyDuringUpdateBodyWriter::write_params(params, &(prefix.to_string() + "StackPolicyDuringUpdateBody"), obj);
		}
		if let Some(ref obj) = obj.notification_ar_ns {
			NotificationARNsWriter::write_params(params, &(prefix.to_string() + "NotificationARN"), obj);
		}
		if let Some(ref obj) = obj.template_url {
			TemplateURLWriter::write_params(params, &(prefix.to_string() + "TemplateURL"), obj);
		}
	}
}
pub type ResourceSignalUniqueId = String;
/// Parse ResourceSignalUniqueId from XML
struct ResourceSignalUniqueIdParser;
impl ResourceSignalUniqueIdParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ResourceSignalUniqueId, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ResourceSignalUniqueId contents to a SignedRequest
struct ResourceSignalUniqueIdWriter;
impl ResourceSignalUniqueIdWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ResourceSignalUniqueId) {
		params.put(name, obj);
	}
}
pub type NextToken = String;
/// Parse NextToken from XML
struct NextTokenParser;
impl NextTokenParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<NextToken, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write NextToken contents to a SignedRequest
struct NextTokenWriter;
impl NextTokenWriter {
	fn write_params(params: &mut Params, name: &str, obj: &NextToken) {
		params.put(name, obj);
	}
}
/// The Parameter data type.
#[derive(Debug, Default)]
pub struct Parameter {
	/// The value associated with the parameter.
	pub parameter_value: ParameterValue,
	/// During a stack update, use the existing parameter value that the stack is
	/// using for a given parameter key. If you specify `true`, do not specify a
	/// parameter value.
	pub use_previous_value: UsePreviousValue,
	/// The key associated with the parameter. If you don't specify a key and value
	/// for a particular parameter, AWS CloudFormation uses the default value that is
	/// specified in your template.
	pub parameter_key: ParameterKey,
}

/// Parse Parameter from XML
struct ParameterParser;
impl ParameterParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<Parameter, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = Parameter::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "ParameterValue" {
				obj.parameter_value = try!(ParameterValueParser::parse_xml("ParameterValue", stack));
				continue;
			}
			if current_name == "UsePreviousValue" {
				obj.use_previous_value = try!(UsePreviousValueParser::parse_xml("UsePreviousValue", stack));
				continue;
			}
			if current_name == "ParameterKey" {
				obj.parameter_key = try!(ParameterKeyParser::parse_xml("ParameterKey", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write Parameter contents to a SignedRequest
struct ParameterWriter;
impl ParameterWriter {
	fn write_params(params: &mut Params, name: &str, obj: &Parameter) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		ParameterValueWriter::write_params(params, &(prefix.to_string() + "ParameterValue"), &obj.parameter_value);
		UsePreviousValueWriter::write_params(params, &(prefix.to_string() + "UsePreviousValue"), &obj.use_previous_value);
		ParameterKeyWriter::write_params(params, &(prefix.to_string() + "ParameterKey"), &obj.parameter_key);
	}
}
pub type DeletionTime = String;
/// Parse DeletionTime from XML
struct DeletionTimeParser;
impl DeletionTimeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<DeletionTime, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write DeletionTime contents to a SignedRequest
struct DeletionTimeWriter;
impl DeletionTimeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &DeletionTime) {
		params.put(name, obj);
	}
}
pub type ResourceStatus = String;
/// Parse ResourceStatus from XML
struct ResourceStatusParser;
impl ResourceStatusParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ResourceStatus, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ResourceStatus contents to a SignedRequest
struct ResourceStatusWriter;
impl ResourceStatusWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ResourceStatus) {
		params.put(name, obj);
	}
}
/// The output for a DescribeStackResource action.
#[derive(Debug, Default)]
pub struct DescribeStackResourceOutput {
	/// A `StackResourceDetail` structure containing the description of the specified
	/// resource in the specified stack.
	pub stack_resource_detail: StackResourceDetail,
}

/// Parse DescribeStackResourceOutput from XML
struct DescribeStackResourceOutputParser;
impl DescribeStackResourceOutputParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<DescribeStackResourceOutput, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = DescribeStackResourceOutput::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "StackResourceDetail" {
				obj.stack_resource_detail = try!(StackResourceDetailParser::parse_xml("StackResourceDetail", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write DescribeStackResourceOutput contents to a SignedRequest
struct DescribeStackResourceOutputWriter;
impl DescribeStackResourceOutputWriter {
	fn write_params(params: &mut Params, name: &str, obj: &DescribeStackResourceOutput) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		StackResourceDetailWriter::write_params(params, &(prefix.to_string() + "StackResourceDetail"), &obj.stack_resource_detail);
	}
}
pub type NoEcho = bool;
/// Parse NoEcho from XML
struct NoEchoParser;
impl NoEchoParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<NoEcho, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = bool::from_str(try!(characters(stack)).as_ref()).unwrap();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write NoEcho contents to a SignedRequest
struct NoEchoWriter;
impl NoEchoWriter {
	fn write_params(params: &mut Params, name: &str, obj: &NoEcho) {
		params.put(name, &obj.to_string());
	}
}
pub type Timestamp = String;
/// Parse Timestamp from XML
struct TimestampParser;
impl TimestampParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<Timestamp, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write Timestamp contents to a SignedRequest
struct TimestampWriter;
impl TimestampWriter {
	fn write_params(params: &mut Params, name: &str, obj: &Timestamp) {
		params.put(name, obj);
	}
}
/// The input for a GetTemplate action.
#[derive(Debug, Default)]
pub struct GetTemplateInput {
	/// The name or the unique stack ID that is associated with the stack, which are
	/// not always interchangeable:
	///   * Running stacks: You can specify either the stack's name or its unique stack ID.
	///   * Deleted stacks: You must specify the unique stack ID.
	/// Default: There is no default value.
	pub stack_name: StackName,
}

/// Parse GetTemplateInput from XML
struct GetTemplateInputParser;
impl GetTemplateInputParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<GetTemplateInput, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = GetTemplateInput::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "StackName" {
				obj.stack_name = try!(StackNameParser::parse_xml("StackName", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write GetTemplateInput contents to a SignedRequest
struct GetTemplateInputWriter;
impl GetTemplateInputWriter {
	fn write_params(params: &mut Params, name: &str, obj: &GetTemplateInput) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		StackNameWriter::write_params(params, &(prefix.to_string() + "StackName"), &obj.stack_name);
	}
}
/// The output for a ListStackResources action.
#[derive(Debug, Default)]
pub struct ListStackResourcesOutput {
	/// String that identifies the start of the next list of stack resources, if there
	/// is one.
	pub next_token: NextToken,
	/// A list of `StackResourceSummary` structures.
	pub stack_resource_summaries: StackResourceSummaries,
}

/// Parse ListStackResourcesOutput from XML
struct ListStackResourcesOutputParser;
impl ListStackResourcesOutputParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ListStackResourcesOutput, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = ListStackResourcesOutput::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "NextToken" {
				obj.next_token = try!(NextTokenParser::parse_xml("NextToken", stack));
				continue;
			}
			if current_name == "StackResourceSummary" {
				obj.stack_resource_summaries = try!(StackResourceSummariesParser::parse_xml("StackResourceSummary", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ListStackResourcesOutput contents to a SignedRequest
struct ListStackResourcesOutputWriter;
impl ListStackResourcesOutputWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ListStackResourcesOutput) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		NextTokenWriter::write_params(params, &(prefix.to_string() + "NextToken"), &obj.next_token);
		StackResourceSummariesWriter::write_params(params, &(prefix.to_string() + "StackResourceSummary"), &obj.stack_resource_summaries);
	}
}
/// The input for DescribeStackResources action.
#[derive(Debug, Default)]
pub struct DescribeStackResourcesInput {
	/// The name or the unique stack ID that is associated with the stack, which are
	/// not always interchangeable:
	///   * Running stacks: You can specify either the stack's name or its unique stack ID.
	///   * Deleted stacks: You must specify the unique stack ID.
	/// Default: There is no default value.
	/// Required: Conditional. If you do not specify `StackName`, you must specify
	/// `PhysicalResourceId`.
	pub stack_name: StackName,
	/// The name or unique identifier that corresponds to a physical instance ID of a
	/// resource supported by AWS CloudFormation.
	/// For example, for an Amazon Elastic Compute Cloud (EC2) instance,
	/// `PhysicalResourceId` corresponds to the `InstanceId`. You can pass the EC2
	/// `InstanceId` to `DescribeStackResources` to find which stack the instance
	/// belongs to and what other resources are part of the stack.
	/// Required: Conditional. If you do not specify `PhysicalResourceId`, you must
	/// specify `StackName`.
	/// Default: There is no default value.
	pub physical_resource_id: PhysicalResourceId,
	/// The logical name of the resource as specified in the template.
	/// Default: There is no default value.
	pub logical_resource_id: LogicalResourceId,
}

/// Parse DescribeStackResourcesInput from XML
struct DescribeStackResourcesInputParser;
impl DescribeStackResourcesInputParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<DescribeStackResourcesInput, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = DescribeStackResourcesInput::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "StackName" {
				obj.stack_name = try!(StackNameParser::parse_xml("StackName", stack));
				continue;
			}
			if current_name == "PhysicalResourceId" {
				obj.physical_resource_id = try!(PhysicalResourceIdParser::parse_xml("PhysicalResourceId", stack));
				continue;
			}
			if current_name == "LogicalResourceId" {
				obj.logical_resource_id = try!(LogicalResourceIdParser::parse_xml("LogicalResourceId", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write DescribeStackResourcesInput contents to a SignedRequest
struct DescribeStackResourcesInputWriter;
impl DescribeStackResourcesInputWriter {
	fn write_params(params: &mut Params, name: &str, obj: &DescribeStackResourcesInput) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		StackNameWriter::write_params(params, &(prefix.to_string() + "StackName"), &obj.stack_name);
		PhysicalResourceIdWriter::write_params(params, &(prefix.to_string() + "PhysicalResourceId"), &obj.physical_resource_id);
		LogicalResourceIdWriter::write_params(params, &(prefix.to_string() + "LogicalResourceId"), &obj.logical_resource_id);
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
pub type AllowedValue = String;
/// Parse AllowedValue from XML
struct AllowedValueParser;
impl AllowedValueParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<AllowedValue, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write AllowedValue contents to a SignedRequest
struct AllowedValueWriter;
impl AllowedValueWriter {
	fn write_params(params: &mut Params, name: &str, obj: &AllowedValue) {
		params.put(name, obj);
	}
}
/// The StackEvent data type.
#[derive(Debug, Default)]
pub struct StackEvent {
	/// The unique ID name of the instance of the stack.
	pub stack_id: StackId,
	/// The unique ID of this event.
	pub event_id: EventId,
	/// Current status of the resource.
	pub resource_status: Option<ResourceStatus>,
	/// Type of resource. (For more information, go to [ AWS Resource Types
	/// Reference](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-
	/// template-resource-type-ref.html) in the AWS CloudFormation User Guide.)
	pub resource_type: Option<ResourceType>,
	/// Time the status was updated.
	pub timestamp: Timestamp,
	/// Success/failure message associated with the resource.
	pub resource_status_reason: Option<ResourceStatusReason>,
	/// The name associated with a stack.
	pub stack_name: StackName,
	/// BLOB of the properties used to create the resource.
	pub resource_properties: Option<ResourceProperties>,
	/// The name or unique identifier associated with the physical instance of the
	/// resource.
	pub physical_resource_id: Option<PhysicalResourceId>,
	/// The logical name of the resource specified in the template.
	pub logical_resource_id: Option<LogicalResourceId>,
}

/// Parse StackEvent from XML
struct StackEventParser;
impl StackEventParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<StackEvent, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = StackEvent::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "StackId" {
				obj.stack_id = try!(StackIdParser::parse_xml("StackId", stack));
				continue;
			}
			if current_name == "EventId" {
				obj.event_id = try!(EventIdParser::parse_xml("EventId", stack));
				continue;
			}
			if current_name == "ResourceStatus" {
				obj.resource_status = Some(try!(ResourceStatusParser::parse_xml("ResourceStatus", stack)));
				continue;
			}
			if current_name == "ResourceType" {
				obj.resource_type = Some(try!(ResourceTypeParser::parse_xml("ResourceType", stack)));
				continue;
			}
			if current_name == "Timestamp" {
				obj.timestamp = try!(TimestampParser::parse_xml("Timestamp", stack));
				continue;
			}
			if current_name == "ResourceStatusReason" {
				obj.resource_status_reason = Some(try!(ResourceStatusReasonParser::parse_xml("ResourceStatusReason", stack)));
				continue;
			}
			if current_name == "StackName" {
				obj.stack_name = try!(StackNameParser::parse_xml("StackName", stack));
				continue;
			}
			if current_name == "ResourceProperties" {
				obj.resource_properties = Some(try!(ResourcePropertiesParser::parse_xml("ResourceProperties", stack)));
				continue;
			}
			if current_name == "PhysicalResourceId" {
				obj.physical_resource_id = Some(try!(PhysicalResourceIdParser::parse_xml("PhysicalResourceId", stack)));
				continue;
			}
			if current_name == "LogicalResourceId" {
				obj.logical_resource_id = Some(try!(LogicalResourceIdParser::parse_xml("LogicalResourceId", stack)));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write StackEvent contents to a SignedRequest
struct StackEventWriter;
impl StackEventWriter {
	fn write_params(params: &mut Params, name: &str, obj: &StackEvent) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		StackIdWriter::write_params(params, &(prefix.to_string() + "StackId"), &obj.stack_id);
		EventIdWriter::write_params(params, &(prefix.to_string() + "EventId"), &obj.event_id);
		if let Some(ref obj) = obj.resource_status {
			ResourceStatusWriter::write_params(params, &(prefix.to_string() + "ResourceStatus"), obj);
		}
		if let Some(ref obj) = obj.resource_type {
			ResourceTypeWriter::write_params(params, &(prefix.to_string() + "ResourceType"), obj);
		}
		TimestampWriter::write_params(params, &(prefix.to_string() + "Timestamp"), &obj.timestamp);
		if let Some(ref obj) = obj.resource_status_reason {
			ResourceStatusReasonWriter::write_params(params, &(prefix.to_string() + "ResourceStatusReason"), obj);
		}
		StackNameWriter::write_params(params, &(prefix.to_string() + "StackName"), &obj.stack_name);
		if let Some(ref obj) = obj.resource_properties {
			ResourcePropertiesWriter::write_params(params, &(prefix.to_string() + "ResourceProperties"), obj);
		}
		if let Some(ref obj) = obj.physical_resource_id {
			PhysicalResourceIdWriter::write_params(params, &(prefix.to_string() + "PhysicalResourceId"), obj);
		}
		if let Some(ref obj) = obj.logical_resource_id {
			LogicalResourceIdWriter::write_params(params, &(prefix.to_string() + "LogicalResourceId"), obj);
		}
	}
}
pub type ParameterKey = String;
/// Parse ParameterKey from XML
struct ParameterKeyParser;
impl ParameterKeyParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ParameterKey, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ParameterKey contents to a SignedRequest
struct ParameterKeyWriter;
impl ParameterKeyWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ParameterKey) {
		params.put(name, obj);
	}
}
/// The ParameterDeclaration data type.
#[derive(Debug, Default)]
pub struct ParameterDeclaration {
	/// The type of parameter.
	pub parameter_type: ParameterType,
	/// The criteria that AWS CloudFormation uses to validate parameter values.
	pub parameter_constraints: ParameterConstraints,
	/// The description that is associate with the parameter.
	pub description: Description,
	/// The default value of the parameter.
	pub default_value: ParameterValue,
	/// Flag that indicates whether the parameter value is shown as plain text in logs
	/// and in the AWS Management Console.
	pub no_echo: NoEcho,
	/// The name that is associated with the parameter.
	pub parameter_key: ParameterKey,
}

/// Parse ParameterDeclaration from XML
struct ParameterDeclarationParser;
impl ParameterDeclarationParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ParameterDeclaration, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = ParameterDeclaration::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "ParameterType" {
				obj.parameter_type = try!(ParameterTypeParser::parse_xml("ParameterType", stack));
				continue;
			}
			if current_name == "ParameterConstraints" {
				obj.parameter_constraints = try!(ParameterConstraintsParser::parse_xml("ParameterConstraints", stack));
				continue;
			}
			if current_name == "Description" {
				obj.description = try!(DescriptionParser::parse_xml("Description", stack));
				continue;
			}
			if current_name == "DefaultValue" {
				obj.default_value = try!(ParameterValueParser::parse_xml("DefaultValue", stack));
				continue;
			}
			if current_name == "NoEcho" {
				obj.no_echo = try!(NoEchoParser::parse_xml("NoEcho", stack));
				continue;
			}
			if current_name == "ParameterKey" {
				obj.parameter_key = try!(ParameterKeyParser::parse_xml("ParameterKey", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ParameterDeclaration contents to a SignedRequest
struct ParameterDeclarationWriter;
impl ParameterDeclarationWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ParameterDeclaration) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		ParameterTypeWriter::write_params(params, &(prefix.to_string() + "ParameterType"), &obj.parameter_type);
		ParameterConstraintsWriter::write_params(params, &(prefix.to_string() + "ParameterConstraints"), &obj.parameter_constraints);
		DescriptionWriter::write_params(params, &(prefix.to_string() + "Description"), &obj.description);
		ParameterValueWriter::write_params(params, &(prefix.to_string() + "DefaultValue"), &obj.default_value);
		NoEchoWriter::write_params(params, &(prefix.to_string() + "NoEcho"), &obj.no_echo);
		ParameterKeyWriter::write_params(params, &(prefix.to_string() + "ParameterKey"), &obj.parameter_key);
	}
}
pub type StackNameOrId = String;
/// Parse StackNameOrId from XML
struct StackNameOrIdParser;
impl StackNameOrIdParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<StackNameOrId, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write StackNameOrId contents to a SignedRequest
struct StackNameOrIdWriter;
impl StackNameOrIdWriter {
	fn write_params(params: &mut Params, name: &str, obj: &StackNameOrId) {
		params.put(name, obj);
	}
}
pub type StackPolicyDuringUpdateBody = String;
/// Parse StackPolicyDuringUpdateBody from XML
struct StackPolicyDuringUpdateBodyParser;
impl StackPolicyDuringUpdateBodyParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<StackPolicyDuringUpdateBody, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write StackPolicyDuringUpdateBody contents to a SignedRequest
struct StackPolicyDuringUpdateBodyWriter;
impl StackPolicyDuringUpdateBodyWriter {
	fn write_params(params: &mut Params, name: &str, obj: &StackPolicyDuringUpdateBody) {
		params.put(name, obj);
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
pub type StackResourceSummaries = Vec<StackResourceSummary>;
/// Parse StackResourceSummaries from XML
struct StackResourceSummariesParser;
impl StackResourceSummariesParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<StackResourceSummaries, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "StackResourceSummary" {
			obj.push(try!(StackResourceSummaryParser::parse_xml("StackResourceSummary", stack)));
		}
		Ok(obj)
	}
}
/// Write StackResourceSummaries contents to a SignedRequest
struct StackResourceSummariesWriter;
impl StackResourceSummariesWriter {
	fn write_params(params: &mut Params, name: &str, obj: &StackResourceSummaries) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			StackResourceSummaryWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
/// The output for the GetStackPolicy action.
#[derive(Debug, Default)]
pub struct GetStackPolicyOutput {
	/// Structure containing the stack policy body. (For more information, go to [
	/// Prevent Updates to Stack
	/// Resources](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide
	/// /protect-stack-resources.html) in the AWS CloudFormation User Guide.)
	pub stack_policy_body: StackPolicyBody,
}

/// Parse GetStackPolicyOutput from XML
struct GetStackPolicyOutputParser;
impl GetStackPolicyOutputParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<GetStackPolicyOutput, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = GetStackPolicyOutput::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "StackPolicyBody" {
				obj.stack_policy_body = try!(StackPolicyBodyParser::parse_xml("StackPolicyBody", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write GetStackPolicyOutput contents to a SignedRequest
struct GetStackPolicyOutputWriter;
impl GetStackPolicyOutputWriter {
	fn write_params(params: &mut Params, name: &str, obj: &GetStackPolicyOutput) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		StackPolicyBodyWriter::write_params(params, &(prefix.to_string() + "StackPolicyBody"), &obj.stack_policy_body);
	}
}
/// The input for DescribeStacks action.
#[derive(Debug, Default)]
pub struct DescribeStacksInput {
	/// The name or the unique stack ID that is associated with the stack, which are
	/// not always interchangeable:
	///   * Running stacks: You can specify either the stack's name or its unique stack ID.
	///   * Deleted stacks: You must specify the unique stack ID.
	/// Default: There is no default value.
	pub stack_name: StackName,
	/// String that identifies the start of the next list of stacks, if there is one.
	pub next_token: NextToken,
}

/// Parse DescribeStacksInput from XML
struct DescribeStacksInputParser;
impl DescribeStacksInputParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<DescribeStacksInput, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = DescribeStacksInput::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "StackName" {
				obj.stack_name = try!(StackNameParser::parse_xml("StackName", stack));
				continue;
			}
			if current_name == "NextToken" {
				obj.next_token = try!(NextTokenParser::parse_xml("NextToken", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write DescribeStacksInput contents to a SignedRequest
struct DescribeStacksInputWriter;
impl DescribeStacksInputWriter {
	fn write_params(params: &mut Params, name: &str, obj: &DescribeStacksInput) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		StackNameWriter::write_params(params, &(prefix.to_string() + "StackName"), &obj.stack_name);
		NextTokenWriter::write_params(params, &(prefix.to_string() + "NextToken"), &obj.next_token);
	}
}
pub type OnFailure = String;
/// Parse OnFailure from XML
struct OnFailureParser;
impl OnFailureParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<OnFailure, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write OnFailure contents to a SignedRequest
struct OnFailureWriter;
impl OnFailureWriter {
	fn write_params(params: &mut Params, name: &str, obj: &OnFailure) {
		params.put(name, obj);
	}
}
/// Resource with the name requested already exists.
#[derive(Debug, Default)]
pub struct AlreadyExistsException;

/// Parse AlreadyExistsException from XML
struct AlreadyExistsExceptionParser;
impl AlreadyExistsExceptionParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<AlreadyExistsException, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = AlreadyExistsException::default();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write AlreadyExistsException contents to a SignedRequest
struct AlreadyExistsExceptionWriter;
impl AlreadyExistsExceptionWriter {
	fn write_params(params: &mut Params, name: &str, obj: &AlreadyExistsException) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
	}
}
pub type StackPolicyBody = String;
/// Parse StackPolicyBody from XML
struct StackPolicyBodyParser;
impl StackPolicyBodyParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<StackPolicyBody, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write StackPolicyBody contents to a SignedRequest
struct StackPolicyBodyWriter;
impl StackPolicyBodyWriter {
	fn write_params(params: &mut Params, name: &str, obj: &StackPolicyBody) {
		params.put(name, obj);
	}
}
/// The input for ValidateTemplate action.
#[derive(Debug, Default)]
pub struct ValidateTemplateInput {
	/// Location of file containing the template body. The URL must point to a
	/// template (max size: 460,800 bytes) located in an S3 bucket in the same region
	/// as the stack. For more information, go to [Template
	/// Anatomy](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide
	/// /template-anatomy.html) in the AWS CloudFormation User Guide.
	/// Conditional: You must pass `TemplateURL` or `TemplateBody`. If both are
	/// passed, only `TemplateBody` is used.
	pub template_url: TemplateURL,
	/// Structure containing the template body with a minimum length of 1 byte and a
	/// maximum length of 51,200 bytes. For more information, go to [Template
	/// Anatomy](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide
	/// /template-anatomy.html) in the AWS CloudFormation User Guide.
	/// Conditional: You must pass `TemplateURL` or `TemplateBody`. If both are
	/// passed, only `TemplateBody` is used.
	pub template_body: TemplateBody,
}

/// Parse ValidateTemplateInput from XML
struct ValidateTemplateInputParser;
impl ValidateTemplateInputParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ValidateTemplateInput, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = ValidateTemplateInput::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "TemplateURL" {
				obj.template_url = try!(TemplateURLParser::parse_xml("TemplateURL", stack));
				continue;
			}
			if current_name == "TemplateBody" {
				obj.template_body = try!(TemplateBodyParser::parse_xml("TemplateBody", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ValidateTemplateInput contents to a SignedRequest
struct ValidateTemplateInputWriter;
impl ValidateTemplateInputWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ValidateTemplateInput) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		TemplateURLWriter::write_params(params, &(prefix.to_string() + "TemplateURL"), &obj.template_url);
		TemplateBodyWriter::write_params(params, &(prefix.to_string() + "TemplateBody"), &obj.template_body);
	}
}
/// The input for the GetStackPolicy action.
#[derive(Debug, Default)]
pub struct GetStackPolicyInput {
	/// The name or unique stack ID that is associated with the stack whose policy you
	/// want to get.
	pub stack_name: StackName,
}

/// Parse GetStackPolicyInput from XML
struct GetStackPolicyInputParser;
impl GetStackPolicyInputParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<GetStackPolicyInput, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = GetStackPolicyInput::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "StackName" {
				obj.stack_name = try!(StackNameParser::parse_xml("StackName", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write GetStackPolicyInput contents to a SignedRequest
struct GetStackPolicyInputWriter;
impl GetStackPolicyInputWriter {
	fn write_params(params: &mut Params, name: &str, obj: &GetStackPolicyInput) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		StackNameWriter::write_params(params, &(prefix.to_string() + "StackName"), &obj.stack_name);
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
pub type Outputs = Vec<Output>;
/// Parse Outputs from XML
struct OutputsParser;
impl OutputsParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<Outputs, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "Output" {
			obj.push(try!(OutputParser::parse_xml("Output", stack)));
		}
		Ok(obj)
	}
}
/// Write Outputs contents to a SignedRequest
struct OutputsWriter;
impl OutputsWriter {
	fn write_params(params: &mut Params, name: &str, obj: &Outputs) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			OutputWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
pub type AllowedValues = Vec<AllowedValue>;
/// Parse AllowedValues from XML
struct AllowedValuesParser;
impl AllowedValuesParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<AllowedValues, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "AllowedValue" {
			obj.push(try!(AllowedValueParser::parse_xml("AllowedValue", stack)));
		}
		Ok(obj)
	}
}
/// Write AllowedValues contents to a SignedRequest
struct AllowedValuesWriter;
impl AllowedValuesWriter {
	fn write_params(params: &mut Params, name: &str, obj: &AllowedValues) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			AllowedValueWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
/// The input for CreateStack action.
#[derive(Debug, Default)]
pub struct CreateStackInput {
	/// A set of user-defined `Tags` to associate with this stack, represented by
	/// key/value pairs. Tags defined for the stack are propagated to EC2 resources
	/// that are created as part of the stack. A maximum number of 10 tags can be
	/// specified.
	pub tags: Option<Tags>,
	/// Structure containing the template body with a minimum length of 1 byte and a
	/// maximum length of 51,200 bytes. For more information, go to [Template
	/// Anatomy](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide
	/// /template-anatomy.html) in the AWS CloudFormation User Guide.
	/// Conditional: You must specify either the `TemplateBody` or the `TemplateURL`
	/// parameter, but not both.
	pub template_body: Option<TemplateBody>,
	/// A list of `Parameter` structures that specify input parameters for the stack.
	pub parameters: Option<Parameters>,
	/// Location of a file containing the stack policy. The URL must point to a policy
	/// (max size: 16KB) located in an S3 bucket in the same region as the stack. You
	/// can specify either the `StackPolicyBody` or the `StackPolicyURL` parameter,
	/// but not both.
	pub stack_policy_url: Option<StackPolicyURL>,
	/// The amount of time that can pass before the stack status becomes
	/// CREATE_FAILED; if `DisableRollback` is not set or is set to `false`, the stack
	/// will be rolled back.
	pub timeout_in_minutes: Option<TimeoutMinutes>,
	/// Structure containing the stack policy body. For more information, go to [
	/// Prevent Updates to Stack
	/// Resources](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide
	/// /protect-stack-resources.html) in the AWS CloudFormation User Guide. You can
	/// specify either the `StackPolicyBody` or the `StackPolicyURL` parameter, but
	/// not both.
	pub stack_policy_body: Option<StackPolicyBody>,
	/// A list of capabilities that you must specify before AWS CloudFormation can
	/// create or update certain stacks. Some stack templates might include resources
	/// that can affect permissions in your AWS account. For those stacks, you must
	/// explicitly acknowledge their capabilities by specifying this parameter.
	/// Currently, the only valid value is `CAPABILITY_IAM`, which is required for the
	/// following resources: [ AWS::IAM::AccessKey](http://docs.aws.amazon.com/AWSClou
	/// dFormation/latest/UserGuide/aws-properties-iam-accesskey.html), [
	/// AWS::IAM::Group](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide
	/// /aws-properties-iam-group.html), [ AWS::IAM::InstanceProfile](http://docs.aws.
	/// amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-
	/// instanceprofile.html), [ AWS::IAM::Policy](http://docs.aws.amazon.com/AWSCloud
	/// Formation/latest/UserGuide/aws-properties-iam-policy.html), [
	/// AWS::IAM::Role](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide
	/// /aws-resource-iam-role.html), [
	/// AWS::IAM::User](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide
	/// /aws-properties-iam-user.html), and [ AWS::IAM::UserToGroupAddition](http://do
	/// cs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-
	/// addusertogroup.html). If your stack template contains these resources, we
	/// recommend that you review any permissions associated with them. If you don't
	/// specify this parameter, this action returns an `InsufficientCapabilities`
	/// error.
	pub capabilities: Option<Capabilities>,
	/// The name that is associated with the stack. The name must be unique in the
	/// region in which you are creating the stack.
	/// A stack name can contain only alphanumeric characters (case sensitive) and
	/// hyphens. It must start with an alphabetic character and cannot be longer than
	/// 255 characters.
	pub stack_name: StackName,
	/// The Simple Notification Service (SNS) topic ARNs to publish stack related
	/// events. You can find your SNS topic ARNs using the [SNS
	/// console](http://console.aws.amazon.com/sns) or your Command Line Interface
	/// (CLI).
	pub notification_ar_ns: Option<NotificationARNs>,
	/// Determines what action will be taken if stack creation fails. This must be one
	/// of: DO_NOTHING, ROLLBACK, or DELETE. You can specify either `OnFailure` or
	/// `DisableRollback`, but not both.
	/// Default: `ROLLBACK`
	pub on_failure: Option<OnFailure>,
	/// Set to `true` to disable rollback of the stack if stack creation failed. You
	/// can specify either `DisableRollback` or `OnFailure`, but not both.
	/// Default: `false`
	pub disable_rollback: Option<DisableRollback>,
	/// Location of file containing the template body. The URL must point to a
	/// template (max size: 460,800 bytes) located in an S3 bucket in the same region
	/// as the stack. For more information, go to the [Template
	/// Anatomy](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide
	/// /template-anatomy.html) in the AWS CloudFormation User Guide.
	/// Conditional: You must specify either the `TemplateBody` or the `TemplateURL`
	/// parameter, but not both.
	pub template_url: Option<TemplateURL>,
}

/// Parse CreateStackInput from XML
struct CreateStackInputParser;
impl CreateStackInputParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<CreateStackInput, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = CreateStackInput::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "Tag" {
				obj.tags = Some(try!(TagsParser::parse_xml("Tag", stack)));
				continue;
			}
			if current_name == "TemplateBody" {
				obj.template_body = Some(try!(TemplateBodyParser::parse_xml("TemplateBody", stack)));
				continue;
			}
			if current_name == "Parameter" {
				obj.parameters = Some(try!(ParametersParser::parse_xml("Parameter", stack)));
				continue;
			}
			if current_name == "StackPolicyURL" {
				obj.stack_policy_url = Some(try!(StackPolicyURLParser::parse_xml("StackPolicyURL", stack)));
				continue;
			}
			if current_name == "TimeoutInMinutes" {
				obj.timeout_in_minutes = Some(try!(TimeoutMinutesParser::parse_xml("TimeoutInMinutes", stack)));
				continue;
			}
			if current_name == "StackPolicyBody" {
				obj.stack_policy_body = Some(try!(StackPolicyBodyParser::parse_xml("StackPolicyBody", stack)));
				continue;
			}
			if current_name == "Capability" {
				obj.capabilities = Some(try!(CapabilitiesParser::parse_xml("Capability", stack)));
				continue;
			}
			if current_name == "StackName" {
				obj.stack_name = try!(StackNameParser::parse_xml("StackName", stack));
				continue;
			}
			if current_name == "NotificationARN" {
				obj.notification_ar_ns = Some(try!(NotificationARNsParser::parse_xml("NotificationARN", stack)));
				continue;
			}
			if current_name == "OnFailure" {
				obj.on_failure = Some(try!(OnFailureParser::parse_xml("OnFailure", stack)));
				continue;
			}
			if current_name == "DisableRollback" {
				obj.disable_rollback = Some(try!(DisableRollbackParser::parse_xml("DisableRollback", stack)));
				continue;
			}
			if current_name == "TemplateURL" {
				obj.template_url = Some(try!(TemplateURLParser::parse_xml("TemplateURL", stack)));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write CreateStackInput contents to a SignedRequest
struct CreateStackInputWriter;
impl CreateStackInputWriter {
	fn write_params(params: &mut Params, name: &str, obj: &CreateStackInput) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		if let Some(ref obj) = obj.tags {
			TagsWriter::write_params(params, &(prefix.to_string() + "Tag"), obj);
		}
		if let Some(ref obj) = obj.template_body {
			TemplateBodyWriter::write_params(params, &(prefix.to_string() + "TemplateBody"), obj);
		}
		if let Some(ref obj) = obj.parameters {
			ParametersWriter::write_params(params, &(prefix.to_string() + "Parameter"), obj);
		}
		if let Some(ref obj) = obj.stack_policy_url {
			StackPolicyURLWriter::write_params(params, &(prefix.to_string() + "StackPolicyURL"), obj);
		}
		if let Some(ref obj) = obj.timeout_in_minutes {
			TimeoutMinutesWriter::write_params(params, &(prefix.to_string() + "TimeoutInMinutes"), obj);
		}
		if let Some(ref obj) = obj.stack_policy_body {
			StackPolicyBodyWriter::write_params(params, &(prefix.to_string() + "StackPolicyBody"), obj);
		}
		if let Some(ref obj) = obj.capabilities {
			CapabilitiesWriter::write_params(params, &(prefix.to_string() + "Capability"), obj);
		}
		StackNameWriter::write_params(params, &(prefix.to_string() + "StackName"), &obj.stack_name);
		if let Some(ref obj) = obj.notification_ar_ns {
			NotificationARNsWriter::write_params(params, &(prefix.to_string() + "NotificationARN"), obj);
		}
		if let Some(ref obj) = obj.on_failure {
			OnFailureWriter::write_params(params, &(prefix.to_string() + "OnFailure"), obj);
		}
		if let Some(ref obj) = obj.disable_rollback {
			DisableRollbackWriter::write_params(params, &(prefix.to_string() + "DisableRollback"), obj);
		}
		if let Some(ref obj) = obj.template_url {
			TemplateURLWriter::write_params(params, &(prefix.to_string() + "TemplateURL"), obj);
		}
	}
}
pub type PhysicalResourceId = String;
/// Parse PhysicalResourceId from XML
struct PhysicalResourceIdParser;
impl PhysicalResourceIdParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<PhysicalResourceId, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write PhysicalResourceId contents to a SignedRequest
struct PhysicalResourceIdWriter;
impl PhysicalResourceIdWriter {
	fn write_params(params: &mut Params, name: &str, obj: &PhysicalResourceId) {
		params.put(name, obj);
	}
}
pub type OutputValue = String;
/// Parse OutputValue from XML
struct OutputValueParser;
impl OutputValueParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<OutputValue, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write OutputValue contents to a SignedRequest
struct OutputValueWriter;
impl OutputValueWriter {
	fn write_params(params: &mut Params, name: &str, obj: &OutputValue) {
		params.put(name, obj);
	}
}
pub type StackResources = Vec<StackResource>;
/// Parse StackResources from XML
struct StackResourcesParser;
impl StackResourcesParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<StackResources, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "StackResource" {
			obj.push(try!(StackResourceParser::parse_xml("StackResource", stack)));
		}
		Ok(obj)
	}
}
/// Write StackResources contents to a SignedRequest
struct StackResourcesWriter;
impl StackResourcesWriter {
	fn write_params(params: &mut Params, name: &str, obj: &StackResources) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			StackResourceWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
/// The output for a UpdateStack action.
#[derive(Debug, Default)]
pub struct UpdateStackOutput {
	/// Unique identifier of the stack.
	pub stack_id: StackId,
}

/// Parse UpdateStackOutput from XML
struct UpdateStackOutputParser;
impl UpdateStackOutputParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<UpdateStackOutput, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = UpdateStackOutput::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "StackId" {
				obj.stack_id = try!(StackIdParser::parse_xml("StackId", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write UpdateStackOutput contents to a SignedRequest
struct UpdateStackOutputWriter;
impl UpdateStackOutputWriter {
	fn write_params(params: &mut Params, name: &str, obj: &UpdateStackOutput) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		StackIdWriter::write_params(params, &(prefix.to_string() + "StackId"), &obj.stack_id);
	}
}
pub type LastUpdatedTime = String;
/// Parse LastUpdatedTime from XML
struct LastUpdatedTimeParser;
impl LastUpdatedTimeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<LastUpdatedTime, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write LastUpdatedTime contents to a SignedRequest
struct LastUpdatedTimeWriter;
impl LastUpdatedTimeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &LastUpdatedTime) {
		params.put(name, obj);
	}
}
pub type StackStatusReason = String;
/// Parse StackStatusReason from XML
struct StackStatusReasonParser;
impl StackStatusReasonParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<StackStatusReason, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write StackStatusReason contents to a SignedRequest
struct StackStatusReasonWriter;
impl StackStatusReasonWriter {
	fn write_params(params: &mut Params, name: &str, obj: &StackStatusReason) {
		params.put(name, obj);
	}
}
/// The template contains resources with capabilities that were not specified in
/// the Capabilities parameter.
#[derive(Debug, Default)]
pub struct InsufficientCapabilitiesException;

/// Parse InsufficientCapabilitiesException from XML
struct InsufficientCapabilitiesExceptionParser;
impl InsufficientCapabilitiesExceptionParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<InsufficientCapabilitiesException, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = InsufficientCapabilitiesException::default();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write InsufficientCapabilitiesException contents to a SignedRequest
struct InsufficientCapabilitiesExceptionWriter;
impl InsufficientCapabilitiesExceptionWriter {
	fn write_params(params: &mut Params, name: &str, obj: &InsufficientCapabilitiesException) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
	}
}
pub type DisableRollback = bool;
/// Parse DisableRollback from XML
struct DisableRollbackParser;
impl DisableRollbackParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<DisableRollback, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = bool::from_str(try!(characters(stack)).as_ref()).unwrap();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write DisableRollback contents to a SignedRequest
struct DisableRollbackWriter;
impl DisableRollbackWriter {
	fn write_params(params: &mut Params, name: &str, obj: &DisableRollback) {
		params.put(name, &obj.to_string());
	}
}
/// Contains high-level information about the specified stack resource.
#[derive(Debug, Default)]
pub struct StackResourceSummary {
	/// Current status of the resource.
	pub resource_status: ResourceStatus,
	/// Type of resource. (For more information, go to [ AWS Resource Types
	/// Reference](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-
	/// template-resource-type-ref.html) in the AWS CloudFormation User Guide.)
	pub resource_type: ResourceType,
	/// Success/failure message associated with the resource.
	pub resource_status_reason: Option<ResourceStatusReason>,
	/// Time the status was updated.
	pub last_updated_timestamp: Timestamp,
	/// The name or unique identifier that corresponds to a physical instance ID of
	/// the resource.
	pub physical_resource_id: Option<PhysicalResourceId>,
	/// The logical name of the resource specified in the template.
	pub logical_resource_id: LogicalResourceId,
}

/// Parse StackResourceSummary from XML
struct StackResourceSummaryParser;
impl StackResourceSummaryParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<StackResourceSummary, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = StackResourceSummary::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "ResourceStatus" {
				obj.resource_status = try!(ResourceStatusParser::parse_xml("ResourceStatus", stack));
				continue;
			}
			if current_name == "ResourceType" {
				obj.resource_type = try!(ResourceTypeParser::parse_xml("ResourceType", stack));
				continue;
			}
			if current_name == "ResourceStatusReason" {
				obj.resource_status_reason = Some(try!(ResourceStatusReasonParser::parse_xml("ResourceStatusReason", stack)));
				continue;
			}
			if current_name == "LastUpdatedTimestamp" {
				obj.last_updated_timestamp = try!(TimestampParser::parse_xml("LastUpdatedTimestamp", stack));
				continue;
			}
			if current_name == "PhysicalResourceId" {
				obj.physical_resource_id = Some(try!(PhysicalResourceIdParser::parse_xml("PhysicalResourceId", stack)));
				continue;
			}
			if current_name == "LogicalResourceId" {
				obj.logical_resource_id = try!(LogicalResourceIdParser::parse_xml("LogicalResourceId", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write StackResourceSummary contents to a SignedRequest
struct StackResourceSummaryWriter;
impl StackResourceSummaryWriter {
	fn write_params(params: &mut Params, name: &str, obj: &StackResourceSummary) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		ResourceStatusWriter::write_params(params, &(prefix.to_string() + "ResourceStatus"), &obj.resource_status);
		ResourceTypeWriter::write_params(params, &(prefix.to_string() + "ResourceType"), &obj.resource_type);
		if let Some(ref obj) = obj.resource_status_reason {
			ResourceStatusReasonWriter::write_params(params, &(prefix.to_string() + "ResourceStatusReason"), obj);
		}
		TimestampWriter::write_params(params, &(prefix.to_string() + "LastUpdatedTimestamp"), &obj.last_updated_timestamp);
		if let Some(ref obj) = obj.physical_resource_id {
			PhysicalResourceIdWriter::write_params(params, &(prefix.to_string() + "PhysicalResourceId"), obj);
		}
		LogicalResourceIdWriter::write_params(params, &(prefix.to_string() + "LogicalResourceId"), &obj.logical_resource_id);
	}
}
/// The output for a CreateStack action.
#[derive(Debug, Default)]
pub struct CreateStackOutput {
	/// Unique identifier of the stack.
	pub stack_id: StackId,
}

/// Parse CreateStackOutput from XML
struct CreateStackOutputParser;
impl CreateStackOutputParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<CreateStackOutput, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = CreateStackOutput::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "StackId" {
				obj.stack_id = try!(StackIdParser::parse_xml("StackId", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write CreateStackOutput contents to a SignedRequest
struct CreateStackOutputWriter;
impl CreateStackOutputWriter {
	fn write_params(params: &mut Params, name: &str, obj: &CreateStackOutput) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		StackIdWriter::write_params(params, &(prefix.to_string() + "StackId"), &obj.stack_id);
	}
}
pub type Stacks = Vec<Stack>;
/// Parse Stacks from XML
struct StacksParser;
impl StacksParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<Stacks, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "Stack" {
			obj.push(try!(StackParser::parse_xml("Stack", stack)));
		}
		Ok(obj)
	}
}
/// Write Stacks contents to a SignedRequest
struct StacksWriter;
impl StacksWriter {
	fn write_params(params: &mut Params, name: &str, obj: &Stacks) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			StackWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
pub type ParameterType = String;
/// Parse ParameterType from XML
struct ParameterTypeParser;
impl ParameterTypeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ParameterType, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ParameterType contents to a SignedRequest
struct ParameterTypeWriter;
impl ParameterTypeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ParameterType) {
		params.put(name, obj);
	}
}
/// The input for the SetStackPolicy action.
#[derive(Debug, Default)]
pub struct SetStackPolicyInput {
	/// The name or unique stack ID that you want to associate a policy with.
	pub stack_name: StackName,
	/// Structure containing the stack policy body. For more information, go to [
	/// Prevent Updates to Stack
	/// Resources](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide
	/// /protect-stack-resources.html) in the AWS CloudFormation User Guide. You can
	/// specify either the `StackPolicyBody` or the `StackPolicyURL` parameter, but
	/// not both.
	pub stack_policy_body: Option<StackPolicyBody>,
	/// Location of a file containing the stack policy. The URL must point to a policy
	/// (max size: 16KB) located in an S3 bucket in the same region as the stack. You
	/// can specify either the `StackPolicyBody` or the `StackPolicyURL` parameter,
	/// but not both.
	pub stack_policy_url: Option<StackPolicyURL>,
}

/// Parse SetStackPolicyInput from XML
struct SetStackPolicyInputParser;
impl SetStackPolicyInputParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<SetStackPolicyInput, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = SetStackPolicyInput::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "StackName" {
				obj.stack_name = try!(StackNameParser::parse_xml("StackName", stack));
				continue;
			}
			if current_name == "StackPolicyBody" {
				obj.stack_policy_body = Some(try!(StackPolicyBodyParser::parse_xml("StackPolicyBody", stack)));
				continue;
			}
			if current_name == "StackPolicyURL" {
				obj.stack_policy_url = Some(try!(StackPolicyURLParser::parse_xml("StackPolicyURL", stack)));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write SetStackPolicyInput contents to a SignedRequest
struct SetStackPolicyInputWriter;
impl SetStackPolicyInputWriter {
	fn write_params(params: &mut Params, name: &str, obj: &SetStackPolicyInput) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		StackNameWriter::write_params(params, &(prefix.to_string() + "StackName"), &obj.stack_name);
		if let Some(ref obj) = obj.stack_policy_body {
			StackPolicyBodyWriter::write_params(params, &(prefix.to_string() + "StackPolicyBody"), obj);
		}
		if let Some(ref obj) = obj.stack_policy_url {
			StackPolicyURLWriter::write_params(params, &(prefix.to_string() + "StackPolicyURL"), obj);
		}
	}
}
/// The input for DescribeStackEvents action.
#[derive(Debug, Default)]
pub struct DescribeStackEventsInput {
	/// The name or the unique stack ID that is associated with the stack, which are
	/// not always interchangeable:
	///   * Running stacks: You can specify either the stack's name or its unique stack ID.
	///   * Deleted stacks: You must specify the unique stack ID.
	/// Default: There is no default value.
	pub stack_name: StackName,
	/// String that identifies the start of the next list of events, if there is one.
	/// Default: There is no default value.
	pub next_token: NextToken,
}

/// Parse DescribeStackEventsInput from XML
struct DescribeStackEventsInputParser;
impl DescribeStackEventsInputParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<DescribeStackEventsInput, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = DescribeStackEventsInput::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "StackName" {
				obj.stack_name = try!(StackNameParser::parse_xml("StackName", stack));
				continue;
			}
			if current_name == "NextToken" {
				obj.next_token = try!(NextTokenParser::parse_xml("NextToken", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write DescribeStackEventsInput contents to a SignedRequest
struct DescribeStackEventsInputWriter;
impl DescribeStackEventsInputWriter {
	fn write_params(params: &mut Params, name: &str, obj: &DescribeStackEventsInput) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		StackNameWriter::write_params(params, &(prefix.to_string() + "StackName"), &obj.stack_name);
		NextTokenWriter::write_params(params, &(prefix.to_string() + "NextToken"), &obj.next_token);
	}
}
/// The output for a DescribeStackResources action.
#[derive(Debug, Default)]
pub struct DescribeStackResourcesOutput {
	/// A list of `StackResource` structures.
	pub stack_resources: StackResources,
}

/// Parse DescribeStackResourcesOutput from XML
struct DescribeStackResourcesOutputParser;
impl DescribeStackResourcesOutputParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<DescribeStackResourcesOutput, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = DescribeStackResourcesOutput::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "StackResource" {
				obj.stack_resources = try!(StackResourcesParser::parse_xml("StackResource", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write DescribeStackResourcesOutput contents to a SignedRequest
struct DescribeStackResourcesOutputWriter;
impl DescribeStackResourcesOutputWriter {
	fn write_params(params: &mut Params, name: &str, obj: &DescribeStackResourcesOutput) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		StackResourcesWriter::write_params(params, &(prefix.to_string() + "StackResource"), &obj.stack_resources);
	}
}
pub type Version = String;
/// Parse Version from XML
struct VersionParser;
impl VersionParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<Version, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write Version contents to a SignedRequest
struct VersionWriter;
impl VersionWriter {
	fn write_params(params: &mut Params, name: &str, obj: &Version) {
		params.put(name, obj);
	}
}
/// The input for DescribeStackResource action.
#[derive(Debug, Default)]
pub struct DescribeStackResourceInput {
	/// The name or the unique stack ID that is associated with the stack, which are
	/// not always interchangeable:
	///   * Running stacks: You can specify either the stack's name or its unique stack ID.
	///   * Deleted stacks: You must specify the unique stack ID.
	/// Default: There is no default value.
	pub stack_name: StackName,
	/// The logical name of the resource as specified in the template.
	/// Default: There is no default value.
	pub logical_resource_id: LogicalResourceId,
}

/// Parse DescribeStackResourceInput from XML
struct DescribeStackResourceInputParser;
impl DescribeStackResourceInputParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<DescribeStackResourceInput, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = DescribeStackResourceInput::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "StackName" {
				obj.stack_name = try!(StackNameParser::parse_xml("StackName", stack));
				continue;
			}
			if current_name == "LogicalResourceId" {
				obj.logical_resource_id = try!(LogicalResourceIdParser::parse_xml("LogicalResourceId", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write DescribeStackResourceInput contents to a SignedRequest
struct DescribeStackResourceInputWriter;
impl DescribeStackResourceInputWriter {
	fn write_params(params: &mut Params, name: &str, obj: &DescribeStackResourceInput) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		StackNameWriter::write_params(params, &(prefix.to_string() + "StackName"), &obj.stack_name);
		LogicalResourceIdWriter::write_params(params, &(prefix.to_string() + "LogicalResourceId"), &obj.logical_resource_id);
	}
}
/// The input for CancelUpdateStack action.
#[derive(Debug, Default)]
pub struct CancelUpdateStackInput {
	/// The name or the unique stack ID that is associated with the stack.
	pub stack_name: StackName,
}

/// Parse CancelUpdateStackInput from XML
struct CancelUpdateStackInputParser;
impl CancelUpdateStackInputParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<CancelUpdateStackInput, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = CancelUpdateStackInput::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "StackName" {
				obj.stack_name = try!(StackNameParser::parse_xml("StackName", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write CancelUpdateStackInput contents to a SignedRequest
struct CancelUpdateStackInputWriter;
impl CancelUpdateStackInputWriter {
	fn write_params(params: &mut Params, name: &str, obj: &CancelUpdateStackInput) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		StackNameWriter::write_params(params, &(prefix.to_string() + "StackName"), &obj.stack_name);
	}
}
/// The output for ValidateTemplate action.
#[derive(Debug, Default)]
pub struct ValidateTemplateOutput {
	/// The list of resources that generated the values in the `Capabilities` response
	/// element.
	pub capabilities_reason: CapabilitiesReason,
	/// The description found within the template.
	pub description: Description,
	/// A list of `TemplateParameter` structures.
	pub parameters: TemplateParameters,
	/// The capabilities found within the template. Currently, AWS CloudFormation
	/// supports only the CAPABILITY_IAM capability. If your template contains IAM
	/// resources, you must specify the CAPABILITY_IAM value for this parameter when
	/// you use the CreateStack or UpdateStack actions with your template; otherwise,
	/// those actions return an InsufficientCapabilities error.
	pub capabilities: Capabilities,
}

/// Parse ValidateTemplateOutput from XML
struct ValidateTemplateOutputParser;
impl ValidateTemplateOutputParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ValidateTemplateOutput, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = ValidateTemplateOutput::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "CapabilitiesReason" {
				obj.capabilities_reason = try!(CapabilitiesReasonParser::parse_xml("CapabilitiesReason", stack));
				continue;
			}
			if current_name == "Description" {
				obj.description = try!(DescriptionParser::parse_xml("Description", stack));
				continue;
			}
			if current_name == "TemplateParameter" {
				obj.parameters = try!(TemplateParametersParser::parse_xml("TemplateParameter", stack));
				continue;
			}
			if current_name == "Capability" {
				obj.capabilities = try!(CapabilitiesParser::parse_xml("Capability", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ValidateTemplateOutput contents to a SignedRequest
struct ValidateTemplateOutputWriter;
impl ValidateTemplateOutputWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ValidateTemplateOutput) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		CapabilitiesReasonWriter::write_params(params, &(prefix.to_string() + "CapabilitiesReason"), &obj.capabilities_reason);
		DescriptionWriter::write_params(params, &(prefix.to_string() + "Description"), &obj.description);
		TemplateParametersWriter::write_params(params, &(prefix.to_string() + "TemplateParameter"), &obj.parameters);
		CapabilitiesWriter::write_params(params, &(prefix.to_string() + "Capability"), &obj.capabilities);
	}
}
pub type StackPolicyURL = String;
/// Parse StackPolicyURL from XML
struct StackPolicyURLParser;
impl StackPolicyURLParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<StackPolicyURL, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write StackPolicyURL contents to a SignedRequest
struct StackPolicyURLWriter;
impl StackPolicyURLWriter {
	fn write_params(params: &mut Params, name: &str, obj: &StackPolicyURL) {
		params.put(name, obj);
	}
}
#[derive(Debug, Default)]
pub struct EstimateTemplateCostInput {
	/// Location of file containing the template body. The URL must point to a
	/// template located in an S3 bucket in the same region as the stack. For more
	/// information, go to [Template
	/// Anatomy](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide
	/// /template-anatomy.html) in the AWS CloudFormation User Guide.
	/// Conditional: You must pass `TemplateURL` or `TemplateBody`. If both are
	/// passed, only `TemplateBody` is used.
	pub template_url: TemplateURL,
	/// A list of `Parameter` structures that specify input parameters.
	pub parameters: Parameters,
	/// Structure containing the template body with a minimum length of 1 byte and a
	/// maximum length of 51,200 bytes. (For more information, go to [Template
	/// Anatomy](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide
	/// /template-anatomy.html) in the AWS CloudFormation User Guide.)
	/// Conditional: You must pass `TemplateBody` or `TemplateURL`. If both are
	/// passed, only `TemplateBody` is used.
	pub template_body: TemplateBody,
}

/// Parse EstimateTemplateCostInput from XML
struct EstimateTemplateCostInputParser;
impl EstimateTemplateCostInputParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<EstimateTemplateCostInput, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = EstimateTemplateCostInput::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "TemplateURL" {
				obj.template_url = try!(TemplateURLParser::parse_xml("TemplateURL", stack));
				continue;
			}
			if current_name == "Parameter" {
				obj.parameters = try!(ParametersParser::parse_xml("Parameter", stack));
				continue;
			}
			if current_name == "TemplateBody" {
				obj.template_body = try!(TemplateBodyParser::parse_xml("TemplateBody", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write EstimateTemplateCostInput contents to a SignedRequest
struct EstimateTemplateCostInputWriter;
impl EstimateTemplateCostInputWriter {
	fn write_params(params: &mut Params, name: &str, obj: &EstimateTemplateCostInput) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		TemplateURLWriter::write_params(params, &(prefix.to_string() + "TemplateURL"), &obj.template_url);
		ParametersWriter::write_params(params, &(prefix.to_string() + "Parameter"), &obj.parameters);
		TemplateBodyWriter::write_params(params, &(prefix.to_string() + "TemplateBody"), &obj.template_body);
	}
}
/// The output for the GetTemplateSummary action.
#[derive(Debug, Default)]
pub struct GetTemplateSummaryOutput {
	/// The list of resources that generated the values in the `Capabilities` response
	/// element.
	pub capabilities_reason: CapabilitiesReason,
	/// The value that is defined in the `Description` property of the template.
	pub description: Description,
	/// A list of parameter declarations that describe various properties for each
	/// parameter.
	pub parameters: ParameterDeclarations,
	/// The capabilities found within the template. Currently, AWS CloudFormation
	/// supports only the CAPABILITY_IAM capability. If your template contains IAM
	/// resources, you must specify the CAPABILITY_IAM value for this parameter when
	/// you use the CreateStack or UpdateStack actions with your template; otherwise,
	/// those actions return an InsufficientCapabilities error.
	pub capabilities: Capabilities,
	/// The AWS template format version, which identifies the capabilities of the
	/// template.
	pub version: Version,
	/// The value that is defined for the `Metadata` property of the template.
	pub metadata: Metadata,
}

/// Parse GetTemplateSummaryOutput from XML
struct GetTemplateSummaryOutputParser;
impl GetTemplateSummaryOutputParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<GetTemplateSummaryOutput, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = GetTemplateSummaryOutput::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "CapabilitiesReason" {
				obj.capabilities_reason = try!(CapabilitiesReasonParser::parse_xml("CapabilitiesReason", stack));
				continue;
			}
			if current_name == "Description" {
				obj.description = try!(DescriptionParser::parse_xml("Description", stack));
				continue;
			}
			if current_name == "ParameterDeclaration" {
				obj.parameters = try!(ParameterDeclarationsParser::parse_xml("ParameterDeclaration", stack));
				continue;
			}
			if current_name == "Capability" {
				obj.capabilities = try!(CapabilitiesParser::parse_xml("Capability", stack));
				continue;
			}
			if current_name == "Version" {
				obj.version = try!(VersionParser::parse_xml("Version", stack));
				continue;
			}
			if current_name == "Metadata" {
				obj.metadata = try!(MetadataParser::parse_xml("Metadata", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write GetTemplateSummaryOutput contents to a SignedRequest
struct GetTemplateSummaryOutputWriter;
impl GetTemplateSummaryOutputWriter {
	fn write_params(params: &mut Params, name: &str, obj: &GetTemplateSummaryOutput) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		CapabilitiesReasonWriter::write_params(params, &(prefix.to_string() + "CapabilitiesReason"), &obj.capabilities_reason);
		DescriptionWriter::write_params(params, &(prefix.to_string() + "Description"), &obj.description);
		ParameterDeclarationsWriter::write_params(params, &(prefix.to_string() + "ParameterDeclaration"), &obj.parameters);
		CapabilitiesWriter::write_params(params, &(prefix.to_string() + "Capability"), &obj.capabilities);
		VersionWriter::write_params(params, &(prefix.to_string() + "Version"), &obj.version);
		MetadataWriter::write_params(params, &(prefix.to_string() + "Metadata"), &obj.metadata);
	}
}
pub type UsePreviousTemplate = bool;
/// Parse UsePreviousTemplate from XML
struct UsePreviousTemplateParser;
impl UsePreviousTemplateParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<UsePreviousTemplate, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = bool::from_str(try!(characters(stack)).as_ref()).unwrap();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write UsePreviousTemplate contents to a SignedRequest
struct UsePreviousTemplateWriter;
impl UsePreviousTemplateWriter {
	fn write_params(params: &mut Params, name: &str, obj: &UsePreviousTemplate) {
		params.put(name, &obj.to_string());
	}
}
pub type ResourceSignalStatus = String;
/// Parse ResourceSignalStatus from XML
struct ResourceSignalStatusParser;
impl ResourceSignalStatusParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ResourceSignalStatus, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ResourceSignalStatus contents to a SignedRequest
struct ResourceSignalStatusWriter;
impl ResourceSignalStatusWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ResourceSignalStatus) {
		params.put(name, obj);
	}
}
pub type Capabilities = Vec<Capability>;
/// Parse Capabilities from XML
struct CapabilitiesParser;
impl CapabilitiesParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<Capabilities, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "Capability" {
			obj.push(try!(CapabilityParser::parse_xml("Capability", stack)));
		}
		Ok(obj)
	}
}
/// Write Capabilities contents to a SignedRequest
struct CapabilitiesWriter;
impl CapabilitiesWriter {
	fn write_params(params: &mut Params, name: &str, obj: &Capabilities) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			CapabilityWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
pub type StackName = String;
/// Parse StackName from XML
struct StackNameParser;
impl StackNameParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<StackName, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write StackName contents to a SignedRequest
struct StackNameWriter;
impl StackNameWriter {
	fn write_params(params: &mut Params, name: &str, obj: &StackName) {
		params.put(name, obj);
	}
}
/// The input for ListStacks action.
#[derive(Debug, Default)]
pub struct ListStacksInput {
	/// String that identifies the start of the next list of stacks, if there is one.
	/// Default: There is no default value.
	pub next_token: NextToken,
	/// Stack status to use as a filter. Specify one or more stack status codes to
	/// list only stacks with the specified status codes. For a complete list of stack
	/// status codes, see the `StackStatus` parameter of the Stack data type.
	pub stack_status_filter: StackStatusFilter,
}

/// Parse ListStacksInput from XML
struct ListStacksInputParser;
impl ListStacksInputParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ListStacksInput, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = ListStacksInput::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "NextToken" {
				obj.next_token = try!(NextTokenParser::parse_xml("NextToken", stack));
				continue;
			}
			if current_name == "StackStatus" {
				obj.stack_status_filter = try!(StackStatusFilterParser::parse_xml("StackStatus", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ListStacksInput contents to a SignedRequest
struct ListStacksInputWriter;
impl ListStacksInputWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ListStacksInput) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		NextTokenWriter::write_params(params, &(prefix.to_string() + "NextToken"), &obj.next_token);
		StackStatusFilterWriter::write_params(params, &(prefix.to_string() + "StackStatus"), &obj.stack_status_filter);
	}
}
/// The output for a DescribeStacks action.
#[derive(Debug, Default)]
pub struct DescribeStacksOutput {
	/// String that identifies the start of the next list of stacks, if there is one.
	pub next_token: NextToken,
	/// A list of stack structures.
	pub stacks: Stacks,
}

/// Parse DescribeStacksOutput from XML
struct DescribeStacksOutputParser;
impl DescribeStacksOutputParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<DescribeStacksOutput, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = DescribeStacksOutput::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "NextToken" {
				obj.next_token = try!(NextTokenParser::parse_xml("NextToken", stack));
				continue;
			}
			if current_name == "Stack" {
				obj.stacks = try!(StacksParser::parse_xml("Stack", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write DescribeStacksOutput contents to a SignedRequest
struct DescribeStacksOutputWriter;
impl DescribeStacksOutputWriter {
	fn write_params(params: &mut Params, name: &str, obj: &DescribeStacksOutput) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		NextTokenWriter::write_params(params, &(prefix.to_string() + "NextToken"), &obj.next_token);
		StacksWriter::write_params(params, &(prefix.to_string() + "Stack"), &obj.stacks);
	}
}
pub type Description = String;
/// Parse Description from XML
struct DescriptionParser;
impl DescriptionParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<Description, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write Description contents to a SignedRequest
struct DescriptionWriter;
impl DescriptionWriter {
	fn write_params(params: &mut Params, name: &str, obj: &Description) {
		params.put(name, obj);
	}
}
pub type NotificationARN = String;
/// Parse NotificationARN from XML
struct NotificationARNParser;
impl NotificationARNParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<NotificationARN, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write NotificationARN contents to a SignedRequest
struct NotificationARNWriter;
impl NotificationARNWriter {
	fn write_params(params: &mut Params, name: &str, obj: &NotificationARN) {
		params.put(name, obj);
	}
}
pub type ResourceStatusReason = String;
/// Parse ResourceStatusReason from XML
struct ResourceStatusReasonParser;
impl ResourceStatusReasonParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ResourceStatusReason, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ResourceStatusReason contents to a SignedRequest
struct ResourceStatusReasonWriter;
impl ResourceStatusReasonWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ResourceStatusReason) {
		params.put(name, obj);
	}
}
/// The input for the SignalResource action.
#[derive(Debug, Default)]
pub struct SignalResourceInput {
	/// The stack name or unique stack ID that includes the resource that you want to
	/// signal.
	pub stack_name: StackNameOrId,
	/// The status of the signal, which is either success or failure. A failure signal
	/// causes AWS CloudFormation to immediately fail the stack creation or update.
	pub status: ResourceSignalStatus,
	/// A unique ID of the signal. When you signal Amazon EC2 instances or Auto
	/// Scaling groups, specify the instance ID that you are signaling as the unique
	/// ID. If you send multiple signals to a single resource (such as signaling a
	/// wait condition), each signal requires a different unique ID.
	pub unique_id: ResourceSignalUniqueId,
	/// The logical ID of the resource that you want to signal. The logical ID is the
	/// name of the resource that given in the template.
	pub logical_resource_id: LogicalResourceId,
}

/// Parse SignalResourceInput from XML
struct SignalResourceInputParser;
impl SignalResourceInputParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<SignalResourceInput, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = SignalResourceInput::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "StackName" {
				obj.stack_name = try!(StackNameOrIdParser::parse_xml("StackName", stack));
				continue;
			}
			if current_name == "Status" {
				obj.status = try!(ResourceSignalStatusParser::parse_xml("Status", stack));
				continue;
			}
			if current_name == "UniqueId" {
				obj.unique_id = try!(ResourceSignalUniqueIdParser::parse_xml("UniqueId", stack));
				continue;
			}
			if current_name == "LogicalResourceId" {
				obj.logical_resource_id = try!(LogicalResourceIdParser::parse_xml("LogicalResourceId", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write SignalResourceInput contents to a SignedRequest
struct SignalResourceInputWriter;
impl SignalResourceInputWriter {
	fn write_params(params: &mut Params, name: &str, obj: &SignalResourceInput) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		StackNameOrIdWriter::write_params(params, &(prefix.to_string() + "StackName"), &obj.stack_name);
		ResourceSignalStatusWriter::write_params(params, &(prefix.to_string() + "Status"), &obj.status);
		ResourceSignalUniqueIdWriter::write_params(params, &(prefix.to_string() + "UniqueId"), &obj.unique_id);
		LogicalResourceIdWriter::write_params(params, &(prefix.to_string() + "LogicalResourceId"), &obj.logical_resource_id);
	}
}
/// The Stack data type.
#[derive(Debug, Default)]
pub struct Stack {
	/// Unique identifier of the stack.
	pub stack_id: Option<StackId>,
	/// The amount of time within which stack creation should complete.
	pub timeout_in_minutes: Option<TimeoutMinutes>,
	/// User defined description associated with the stack.
	pub description: Option<Description>,
	/// A list of `Parameter` structures.
	pub parameters: Option<Parameters>,
	/// A list of `Tag`s that specify cost allocation information for the stack.
	pub tags: Option<Tags>,
	/// A list of output structures.
	pub outputs: Option<Outputs>,
	/// Success/failure message associated with the stack status.
	pub stack_status_reason: Option<StackStatusReason>,
	/// Time at which the stack was created.
	pub creation_time: CreationTime,
	/// The capabilities allowed in the stack.
	pub capabilities: Option<Capabilities>,
	/// The name associated with the stack.
	pub stack_name: StackName,
	/// SNS topic ARNs to which stack related events are published.
	pub notification_ar_ns: Option<NotificationARNs>,
	/// Current status of the stack.
	pub stack_status: StackStatus,
	/// Boolean to enable or disable rollback on stack creation failures:
	///   * `true`: disable rollback
	///   * `false`: enable rollback
	pub disable_rollback: Option<DisableRollback>,
	/// The time the stack was last updated. This field will only be returned if the
	/// stack has been updated at least once.
	pub last_updated_time: Option<LastUpdatedTime>,
}

/// Parse Stack from XML
struct StackParser;
impl StackParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<Stack, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = Stack::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "StackId" {
				obj.stack_id = Some(try!(StackIdParser::parse_xml("StackId", stack)));
				continue;
			}
			if current_name == "TimeoutInMinutes" {
				obj.timeout_in_minutes = Some(try!(TimeoutMinutesParser::parse_xml("TimeoutInMinutes", stack)));
				continue;
			}
			if current_name == "Description" {
				obj.description = Some(try!(DescriptionParser::parse_xml("Description", stack)));
				continue;
			}
			if current_name == "Parameter" {
				obj.parameters = Some(try!(ParametersParser::parse_xml("Parameter", stack)));
				continue;
			}
			if current_name == "Tag" {
				obj.tags = Some(try!(TagsParser::parse_xml("Tag", stack)));
				continue;
			}
			if current_name == "Output" {
				obj.outputs = Some(try!(OutputsParser::parse_xml("Output", stack)));
				continue;
			}
			if current_name == "StackStatusReason" {
				obj.stack_status_reason = Some(try!(StackStatusReasonParser::parse_xml("StackStatusReason", stack)));
				continue;
			}
			if current_name == "CreationTime" {
				obj.creation_time = try!(CreationTimeParser::parse_xml("CreationTime", stack));
				continue;
			}
			if current_name == "Capability" {
				obj.capabilities = Some(try!(CapabilitiesParser::parse_xml("Capability", stack)));
				continue;
			}
			if current_name == "StackName" {
				obj.stack_name = try!(StackNameParser::parse_xml("StackName", stack));
				continue;
			}
			if current_name == "NotificationARN" {
				obj.notification_ar_ns = Some(try!(NotificationARNsParser::parse_xml("NotificationARN", stack)));
				continue;
			}
			if current_name == "StackStatus" {
				obj.stack_status = try!(StackStatusParser::parse_xml("StackStatus", stack));
				continue;
			}
			if current_name == "DisableRollback" {
				obj.disable_rollback = Some(try!(DisableRollbackParser::parse_xml("DisableRollback", stack)));
				continue;
			}
			if current_name == "LastUpdatedTime" {
				obj.last_updated_time = Some(try!(LastUpdatedTimeParser::parse_xml("LastUpdatedTime", stack)));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write Stack contents to a SignedRequest
struct StackWriter;
impl StackWriter {
	fn write_params(params: &mut Params, name: &str, obj: &Stack) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		if let Some(ref obj) = obj.stack_id {
			StackIdWriter::write_params(params, &(prefix.to_string() + "StackId"), obj);
		}
		if let Some(ref obj) = obj.timeout_in_minutes {
			TimeoutMinutesWriter::write_params(params, &(prefix.to_string() + "TimeoutInMinutes"), obj);
		}
		if let Some(ref obj) = obj.description {
			DescriptionWriter::write_params(params, &(prefix.to_string() + "Description"), obj);
		}
		if let Some(ref obj) = obj.parameters {
			ParametersWriter::write_params(params, &(prefix.to_string() + "Parameter"), obj);
		}
		if let Some(ref obj) = obj.tags {
			TagsWriter::write_params(params, &(prefix.to_string() + "Tag"), obj);
		}
		if let Some(ref obj) = obj.outputs {
			OutputsWriter::write_params(params, &(prefix.to_string() + "Output"), obj);
		}
		if let Some(ref obj) = obj.stack_status_reason {
			StackStatusReasonWriter::write_params(params, &(prefix.to_string() + "StackStatusReason"), obj);
		}
		CreationTimeWriter::write_params(params, &(prefix.to_string() + "CreationTime"), &obj.creation_time);
		if let Some(ref obj) = obj.capabilities {
			CapabilitiesWriter::write_params(params, &(prefix.to_string() + "Capability"), obj);
		}
		StackNameWriter::write_params(params, &(prefix.to_string() + "StackName"), &obj.stack_name);
		if let Some(ref obj) = obj.notification_ar_ns {
			NotificationARNsWriter::write_params(params, &(prefix.to_string() + "NotificationARN"), obj);
		}
		StackStatusWriter::write_params(params, &(prefix.to_string() + "StackStatus"), &obj.stack_status);
		if let Some(ref obj) = obj.disable_rollback {
			DisableRollbackWriter::write_params(params, &(prefix.to_string() + "DisableRollback"), obj);
		}
		if let Some(ref obj) = obj.last_updated_time {
			LastUpdatedTimeWriter::write_params(params, &(prefix.to_string() + "LastUpdatedTime"), obj);
		}
	}
}
pub type EventId = String;
/// Parse EventId from XML
struct EventIdParser;
impl EventIdParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<EventId, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write EventId contents to a SignedRequest
struct EventIdWriter;
impl EventIdWriter {
	fn write_params(params: &mut Params, name: &str, obj: &EventId) {
		params.put(name, obj);
	}
}
pub type StackPolicyDuringUpdateURL = String;
/// Parse StackPolicyDuringUpdateURL from XML
struct StackPolicyDuringUpdateURLParser;
impl StackPolicyDuringUpdateURLParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<StackPolicyDuringUpdateURL, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write StackPolicyDuringUpdateURL contents to a SignedRequest
struct StackPolicyDuringUpdateURLWriter;
impl StackPolicyDuringUpdateURLWriter {
	fn write_params(params: &mut Params, name: &str, obj: &StackPolicyDuringUpdateURL) {
		params.put(name, obj);
	}
}
pub type OutputKey = String;
/// Parse OutputKey from XML
struct OutputKeyParser;
impl OutputKeyParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<OutputKey, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write OutputKey contents to a SignedRequest
struct OutputKeyWriter;
impl OutputKeyWriter {
	fn write_params(params: &mut Params, name: &str, obj: &OutputKey) {
		params.put(name, obj);
	}
}
pub type TemplateBody = String;
/// Parse TemplateBody from XML
struct TemplateBodyParser;
impl TemplateBodyParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<TemplateBody, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write TemplateBody contents to a SignedRequest
struct TemplateBodyWriter;
impl TemplateBodyWriter {
	fn write_params(params: &mut Params, name: &str, obj: &TemplateBody) {
		params.put(name, obj);
	}
}
pub type Url = String;
/// Parse Url from XML
struct UrlParser;
impl UrlParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<Url, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write Url contents to a SignedRequest
struct UrlWriter;
impl UrlWriter {
	fn write_params(params: &mut Params, name: &str, obj: &Url) {
		params.put(name, obj);
	}
}
pub type Capability = String;
/// Parse Capability from XML
struct CapabilityParser;
impl CapabilityParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<Capability, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write Capability contents to a SignedRequest
struct CapabilityWriter;
impl CapabilityWriter {
	fn write_params(params: &mut Params, name: &str, obj: &Capability) {
		params.put(name, obj);
	}
}
pub type ParameterValue = String;
/// Parse ParameterValue from XML
struct ParameterValueParser;
impl ParameterValueParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ParameterValue, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ParameterValue contents to a SignedRequest
struct ParameterValueWriter;
impl ParameterValueWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ParameterValue) {
		params.put(name, obj);
	}
}
/// The Output data type.
#[derive(Debug, Default)]
pub struct Output {
	/// User defined description associated with the output.
	pub description: Description,
	/// The key associated with the output.
	pub output_key: OutputKey,
	/// The value associated with the output.
	pub output_value: OutputValue,
}

/// Parse Output from XML
struct OutputParser;
impl OutputParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<Output, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = Output::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "Description" {
				obj.description = try!(DescriptionParser::parse_xml("Description", stack));
				continue;
			}
			if current_name == "OutputKey" {
				obj.output_key = try!(OutputKeyParser::parse_xml("OutputKey", stack));
				continue;
			}
			if current_name == "OutputValue" {
				obj.output_value = try!(OutputValueParser::parse_xml("OutputValue", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write Output contents to a SignedRequest
struct OutputWriter;
impl OutputWriter {
	fn write_params(params: &mut Params, name: &str, obj: &Output) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		DescriptionWriter::write_params(params, &(prefix.to_string() + "Description"), &obj.description);
		OutputKeyWriter::write_params(params, &(prefix.to_string() + "OutputKey"), &obj.output_key);
		OutputValueWriter::write_params(params, &(prefix.to_string() + "OutputValue"), &obj.output_value);
	}
}
/// The output for a EstimateTemplateCost action.
#[derive(Debug, Default)]
pub struct EstimateTemplateCostOutput {
	/// An AWS Simple Monthly Calculator URL with a query string that describes the
	/// resources required to run the template.
	pub url: Url,
}

/// Parse EstimateTemplateCostOutput from XML
struct EstimateTemplateCostOutputParser;
impl EstimateTemplateCostOutputParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<EstimateTemplateCostOutput, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = EstimateTemplateCostOutput::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "Url" {
				obj.url = try!(UrlParser::parse_xml("Url", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write EstimateTemplateCostOutput contents to a SignedRequest
struct EstimateTemplateCostOutputWriter;
impl EstimateTemplateCostOutputWriter {
	fn write_params(params: &mut Params, name: &str, obj: &EstimateTemplateCostOutput) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		UrlWriter::write_params(params, &(prefix.to_string() + "Url"), &obj.url);
	}
}
pub type LogicalResourceId = String;
/// Parse LogicalResourceId from XML
struct LogicalResourceIdParser;
impl LogicalResourceIdParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<LogicalResourceId, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write LogicalResourceId contents to a SignedRequest
struct LogicalResourceIdWriter;
impl LogicalResourceIdWriter {
	fn write_params(params: &mut Params, name: &str, obj: &LogicalResourceId) {
		params.put(name, obj);
	}
}
/// The output for GetTemplate action.
#[derive(Debug, Default)]
pub struct GetTemplateOutput {
	/// Structure containing the template body. (For more information, go to [Template
	/// Anatomy](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide
	/// /template-anatomy.html) in the AWS CloudFormation User Guide.)
	pub template_body: TemplateBody,
}

/// Parse GetTemplateOutput from XML
struct GetTemplateOutputParser;
impl GetTemplateOutputParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<GetTemplateOutput, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = GetTemplateOutput::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "TemplateBody" {
				obj.template_body = try!(TemplateBodyParser::parse_xml("TemplateBody", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write GetTemplateOutput contents to a SignedRequest
struct GetTemplateOutputWriter;
impl GetTemplateOutputWriter {
	fn write_params(params: &mut Params, name: &str, obj: &GetTemplateOutput) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		TemplateBodyWriter::write_params(params, &(prefix.to_string() + "TemplateBody"), &obj.template_body);
	}
}
pub type ResourceProperties = String;
/// Parse ResourceProperties from XML
struct ResourcePropertiesParser;
impl ResourcePropertiesParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ResourceProperties, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ResourceProperties contents to a SignedRequest
struct ResourcePropertiesWriter;
impl ResourcePropertiesWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ResourceProperties) {
		params.put(name, obj);
	}
}
pub type TemplateParameters = Vec<TemplateParameter>;
/// Parse TemplateParameters from XML
struct TemplateParametersParser;
impl TemplateParametersParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<TemplateParameters, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "TemplateParameter" {
			obj.push(try!(TemplateParameterParser::parse_xml("TemplateParameter", stack)));
		}
		Ok(obj)
	}
}
/// Write TemplateParameters contents to a SignedRequest
struct TemplateParametersWriter;
impl TemplateParametersWriter {
	fn write_params(params: &mut Params, name: &str, obj: &TemplateParameters) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			TemplateParameterWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
pub type ParameterDeclarations = Vec<ParameterDeclaration>;
/// Parse ParameterDeclarations from XML
struct ParameterDeclarationsParser;
impl ParameterDeclarationsParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ParameterDeclarations, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "ParameterDeclaration" {
			obj.push(try!(ParameterDeclarationParser::parse_xml("ParameterDeclaration", stack)));
		}
		Ok(obj)
	}
}
/// Write ParameterDeclarations contents to a SignedRequest
struct ParameterDeclarationsWriter;
impl ParameterDeclarationsWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ParameterDeclarations) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			ParameterDeclarationWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
pub struct CloudFormationClient<'a> {
	creds: &'a AWSCredentials,
	region: &'a str
}

impl<'a> CloudFormationClient<'a> { 
	pub fn new(creds: &'a AWSCredentials, region: &'a str) -> CloudFormationClient<'a> {
		CloudFormationClient { creds: creds, region: region }
	}
	/// Returns a description of the specified resource in the specified stack.
	/// For deleted stacks, DescribeStackResource returns resource information for up
	/// to 90 days after the stack has been deleted.
	pub fn describe_stack_resource(&self, input: &DescribeStackResourceInput) -> Result<DescribeStackResourceOutput, AWSError> {
		let mut request = SignedRequest::new("POST", "cloudformation", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "DescribeStackResource");
		DescribeStackResourceInputWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(DescribeStackResourceOutputParser::parse_xml("DescribeStackResourceOutput", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Returns all stack related events for a specified stack. For more information
	/// about a stack's event history, go to
	/// [Stacks](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide
	/// /concept-stack.html) in the AWS CloudFormation User Guide.
	/// You can list events for stacks that have failed to create or have been deleted
	/// by specifying the unique stack identifier (stack ID).
	pub fn describe_stack_events(&self, input: &DescribeStackEventsInput) -> Result<DescribeStackEventsOutput, AWSError> {
		let mut request = SignedRequest::new("POST", "cloudformation", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "DescribeStackEvents");
		DescribeStackEventsInputWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(DescribeStackEventsOutputParser::parse_xml("DescribeStackEventsOutput", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Updates a stack as specified in the template. After the call completes
	/// successfully, the stack update starts. You can check the status of the stack
	/// via the DescribeStacks action.
	/// To get a copy of the template for an existing stack, you can use the
	/// GetTemplate action.
	/// Tags that were associated with this stack during creation time will still be
	/// associated with the stack after an `UpdateStack` operation.
	/// For more information about creating an update template, updating a stack, and
	/// monitoring the progress of the update, see [Updating a
	/// Stack](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-
	/// cfn-updating-stacks.html).
	pub fn update_stack(&self, input: &UpdateStackInput) -> Result<UpdateStackOutput, AWSError> {
		let mut request = SignedRequest::new("POST", "cloudformation", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "UpdateStack");
		UpdateStackInputWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(UpdateStackOutputParser::parse_xml("UpdateStackOutput", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Returns information about a new or existing template. The `GetTemplateSummary`
	/// action is useful for viewing parameter information, such as default parameter
	/// values and parameter types, before you create or update a stack.
	/// You can use the `GetTemplateSummary` action when you submit a template, or you
	/// can get template information for a running or deleted stack.
	/// For deleted stacks, `GetTemplateSummary` returns the template information for
	/// up to 90 days after the stack has been deleted. If the template does not
	/// exist, a `ValidationError` is returned.
	pub fn get_template_summary(&self, input: &GetTemplateSummaryInput) -> Result<GetTemplateSummaryOutput, AWSError> {
		let mut request = SignedRequest::new("POST", "cloudformation", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "GetTemplateSummary");
		GetTemplateSummaryInputWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(GetTemplateSummaryOutputParser::parse_xml("GetTemplateSummaryOutput", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Returns descriptions of all resources of the specified stack.
	/// For deleted stacks, ListStackResources returns resource information for up to
	/// 90 days after the stack has been deleted.
	pub fn list_stack_resources(&self, input: &ListStackResourcesInput) -> Result<ListStackResourcesOutput, AWSError> {
		let mut request = SignedRequest::new("POST", "cloudformation", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "ListStackResources");
		ListStackResourcesInputWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(ListStackResourcesOutputParser::parse_xml("ListStackResourcesOutput", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Cancels an update on the specified stack. If the call completes successfully,
	/// the stack will roll back the update and revert to the previous stack
	/// configuration.
	/// Only stacks that are in the UPDATE_IN_PROGRESS state can be canceled.
	pub fn cancel_update_stack(&self, input: &CancelUpdateStackInput) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "cloudformation", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "CancelUpdateStack");
		CancelUpdateStackInputWriter::write_params(&mut params, "", &input);
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
	/// Returns the template body for a specified stack. You can get the template for
	/// running or deleted stacks.
	/// For deleted stacks, GetTemplate returns the template for up to 90 days after
	/// the stack has been deleted.
	/// If the template does not exist, a `ValidationError` is returned.
	pub fn get_template(&self, input: &GetTemplateInput) -> Result<GetTemplateOutput, AWSError> {
		let mut request = SignedRequest::new("POST", "cloudformation", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "GetTemplate");
		GetTemplateInputWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(GetTemplateOutputParser::parse_xml("GetTemplateOutput", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Validates a specified template.
	pub fn validate_template(&self, input: &ValidateTemplateInput) -> Result<ValidateTemplateOutput, AWSError> {
		let mut request = SignedRequest::new("POST", "cloudformation", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "ValidateTemplate");
		ValidateTemplateInputWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(ValidateTemplateOutputParser::parse_xml("ValidateTemplateOutput", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Deletes a specified stack. Once the call completes successfully, stack
	/// deletion starts. Deleted stacks do not show up in the DescribeStacks API if
	/// the deletion has been completed successfully.
	pub fn delete_stack(&self, input: &DeleteStackInput) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "cloudformation", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "DeleteStack");
		DeleteStackInputWriter::write_params(&mut params, "", &input);
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
	/// Sends a signal to the specified resource with a success or failure status. You
	/// can use the SignalResource API in conjunction with a creation policy or update
	/// policy. AWS CloudFormation doesn't proceed with a stack creation or update
	/// until resources receive the required number of signals or the timeout period
	/// is exceeded. The SignalResource API is useful in cases where you want to send
	/// signals from anywhere other than an Amazon EC2 instance.
	pub fn signal_resource(&self, input: &SignalResourceInput) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "cloudformation", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "SignalResource");
		SignalResourceInputWriter::write_params(&mut params, "", &input);
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
	/// Returns the summary information for stacks whose status matches the specified
	/// StackStatusFilter. Summary information for stacks that have been deleted is
	/// kept for 90 days after the stack is deleted. If no StackStatusFilter is
	/// specified, summary information for all stacks is returned (including existing
	/// stacks and stacks that have been deleted).
	pub fn list_stacks(&self, input: &ListStacksInput) -> Result<ListStacksOutput, AWSError> {
		let mut request = SignedRequest::new("POST", "cloudformation", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "ListStacks");
		ListStacksInputWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(ListStacksOutputParser::parse_xml("ListStacksOutput", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Sets a stack policy for a specified stack.
	pub fn set_stack_policy(&self, input: &SetStackPolicyInput) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "cloudformation", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "SetStackPolicy");
		SetStackPolicyInputWriter::write_params(&mut params, "", &input);
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
	/// Returns the stack policy for a specified stack. If a stack doesn't have a
	/// policy, a null value is returned.
	pub fn get_stack_policy(&self, input: &GetStackPolicyInput) -> Result<GetStackPolicyOutput, AWSError> {
		let mut request = SignedRequest::new("POST", "cloudformation", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "GetStackPolicy");
		GetStackPolicyInputWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(GetStackPolicyOutputParser::parse_xml("GetStackPolicyOutput", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Returns AWS resource descriptions for running and deleted stacks. If
	/// `StackName` is specified, all the associated resources that are part of the
	/// stack are returned. If `PhysicalResourceId` is specified, the associated
	/// resources of the stack that the resource belongs to are returned.
	/// Only the first 100 resources will be returned. If your stack has more
	/// resources than this, you should use `ListStackResources` instead.
	/// For deleted stacks, `DescribeStackResources` returns resource information for
	/// up to 90 days after the stack has been deleted.
	/// You must specify either `StackName` or `PhysicalResourceId`, but not both. In
	/// addition, you can specify `LogicalResourceId` to filter the returned result.
	/// For more information about resources, the `LogicalResourceId` and
	/// `PhysicalResourceId`, go to the [AWS CloudFormation User
	/// Guide](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide).
	/// A `ValidationError` is returned if you specify both `StackName` and
	/// `PhysicalResourceId` in the same request.
	pub fn describe_stack_resources(&self, input: &DescribeStackResourcesInput) -> Result<DescribeStackResourcesOutput, AWSError> {
		let mut request = SignedRequest::new("POST", "cloudformation", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "DescribeStackResources");
		DescribeStackResourcesInputWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(DescribeStackResourcesOutputParser::parse_xml("DescribeStackResourcesOutput", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Returns the estimated monthly cost of a template. The return value is an AWS
	/// Simple Monthly Calculator URL with a query string that describes the resources
	/// required to run the template.
	pub fn estimate_template_cost(&self, input: &EstimateTemplateCostInput) -> Result<EstimateTemplateCostOutput, AWSError> {
		let mut request = SignedRequest::new("POST", "cloudformation", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "EstimateTemplateCost");
		EstimateTemplateCostInputWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(EstimateTemplateCostOutputParser::parse_xml("EstimateTemplateCostOutput", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Returns the description for the specified stack; if no stack name was
	/// specified, then it returns the description for all the stacks created.
	pub fn describe_stacks(&self, input: &DescribeStacksInput) -> Result<DescribeStacksOutput, AWSError> {
		let mut request = SignedRequest::new("POST", "cloudformation", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "DescribeStacks");
		DescribeStacksInputWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(DescribeStacksOutputParser::parse_xml("DescribeStacksOutput", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Creates a stack as specified in the template. After the call completes
	/// successfully, the stack creation starts. You can check the status of the stack
	/// via the DescribeStacks API.
	pub fn create_stack(&self, input: &CreateStackInput) -> Result<CreateStackOutput, AWSError> {
		let mut request = SignedRequest::new("POST", "cloudformation", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "CreateStack");
		CreateStackInputWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(CreateStackOutputParser::parse_xml("CreateStackOutput", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
}
