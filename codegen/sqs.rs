use std::collections::HashMap;
use std::str;

#[derive(Debug, Default)]
pub struct UnsupportedOperation;

pub struct UnsupportedOperationParser;
impl UnsupportedOperationParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<UnsupportedOperation, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = UnsupportedOperation::default();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub struct UnsupportedOperationWriter;
impl UnsupportedOperationWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &UnsupportedOperation) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
	}
}
pub type Binary = Vec<u8>;
pub struct BinaryParser;
impl BinaryParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<Binary, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack)).into_bytes();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub struct BinaryWriter;
impl BinaryWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &Binary) {
		params.put(name, str::from_utf8(&obj).unwrap());
	}
}

#[derive(Debug, Default)]
pub struct PurgeQueueInProgress;

pub struct PurgeQueueInProgressParser;
impl PurgeQueueInProgressParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<PurgeQueueInProgress, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = PurgeQueueInProgress::default();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub struct PurgeQueueInProgressWriter;
impl PurgeQueueInProgressWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &PurgeQueueInProgress) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
	}
}

#[derive(Debug, Default)]
pub struct MessageAttributeValue {
	pub binary_list_values: Option<BinaryList>,
	pub string_value: Option<String>,
	pub data_type: String,
	pub binary_value: Option<Binary>,
	pub string_list_values: Option<StringList>,
}

pub struct MessageAttributeValueParser;
impl MessageAttributeValueParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<MessageAttributeValue, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = MessageAttributeValue::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "BinaryListValue" {
				obj.binary_list_values = Some(try!(BinaryListParser::parse_xml("BinaryListValue", stack)));
				continue;
			}
			if current_name == "StringValue" {
				obj.string_value = Some(try!(StringParser::parse_xml("StringValue", stack)));
				continue;
			}
			if current_name == "DataType" {
				obj.data_type = try!(StringParser::parse_xml("DataType", stack));
				continue;
			}
			if current_name == "BinaryValue" {
				obj.binary_value = Some(try!(BinaryParser::parse_xml("BinaryValue", stack)));
				continue;
			}
			if current_name == "StringListValue" {
				obj.string_list_values = Some(try!(StringListParser::parse_xml("StringListValue", stack)));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub struct MessageAttributeValueWriter;
impl MessageAttributeValueWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &MessageAttributeValue) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		if let Some(ref obj) = obj.binary_list_values {
			BinaryListWriter::write_params(params, &(prefix.to_string() + "BinaryListValue"), obj);
		}
		if let Some(ref obj) = obj.string_value {
			StringWriter::write_params(params, &(prefix.to_string() + "StringValue"), obj);
		}
		StringWriter::write_params(params, &(prefix.to_string() + "DataType"), &obj.data_type);
		if let Some(ref obj) = obj.binary_value {
			BinaryWriter::write_params(params, &(prefix.to_string() + "BinaryValue"), obj);
		}
		if let Some(ref obj) = obj.string_list_values {
			StringListWriter::write_params(params, &(prefix.to_string() + "StringListValue"), obj);
		}
	}
}
pub type MessageAttributeNameList = Vec<MessageAttributeName>;
pub struct MessageAttributeNameListParser;
impl MessageAttributeNameListParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<MessageAttributeNameList, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "MessageAttributeName" {
			obj.push(try!(MessageAttributeNameParser::parse_xml("MessageAttributeName", stack)));
		}
		Ok(obj)
	}
}
pub struct MessageAttributeNameListWriter;
impl MessageAttributeNameListWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &MessageAttributeNameList) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			MessageAttributeNameWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}

#[derive(Debug, Default)]
pub struct SetQueueAttributesRequest {
	pub queue_url: String,
	pub attributes: AttributeMap,
}

pub struct SetQueueAttributesRequestParser;
impl SetQueueAttributesRequestParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<SetQueueAttributesRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = SetQueueAttributesRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "QueueUrl" {
				obj.queue_url = try!(StringParser::parse_xml("QueueUrl", stack));
				continue;
			}
			if current_name == "Attribute" {
				obj.attributes = try!(AttributeMapParser::parse_xml("Attribute", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub struct SetQueueAttributesRequestWriter;
impl SetQueueAttributesRequestWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &SetQueueAttributesRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		StringWriter::write_params(params, &(prefix.to_string() + "QueueUrl"), &obj.queue_url);
		AttributeMapWriter::write_params(params, &(prefix.to_string() + "Attribute"), &obj.attributes);
	}
}

#[derive(Debug, Default)]
pub struct GetQueueAttributesRequest {
	pub queue_url: String,
	pub attribute_names: Option<AttributeNameList>,
}

pub struct GetQueueAttributesRequestParser;
impl GetQueueAttributesRequestParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<GetQueueAttributesRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = GetQueueAttributesRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "QueueUrl" {
				obj.queue_url = try!(StringParser::parse_xml("QueueUrl", stack));
				continue;
			}
			if current_name == "AttributeName" {
				obj.attribute_names = Some(try!(AttributeNameListParser::parse_xml("AttributeName", stack)));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub struct GetQueueAttributesRequestWriter;
impl GetQueueAttributesRequestWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &GetQueueAttributesRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		StringWriter::write_params(params, &(prefix.to_string() + "QueueUrl"), &obj.queue_url);
		if let Some(ref obj) = obj.attribute_names {
			AttributeNameListWriter::write_params(params, &(prefix.to_string() + "AttributeName"), obj);
		}
	}
}

#[derive(Debug, Default)]
pub struct SendMessageBatchResult {
	pub successful: SendMessageBatchResultEntryList,
	pub failed: BatchResultErrorEntryList,
}

pub struct SendMessageBatchResultParser;
impl SendMessageBatchResultParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<SendMessageBatchResult, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = SendMessageBatchResult::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "SendMessageBatchResultEntry" {
				obj.successful = try!(SendMessageBatchResultEntryListParser::parse_xml("SendMessageBatchResultEntry", stack));
				continue;
			}
			if current_name == "BatchResultErrorEntry" {
				obj.failed = try!(BatchResultErrorEntryListParser::parse_xml("BatchResultErrorEntry", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub struct SendMessageBatchResultWriter;
impl SendMessageBatchResultWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &SendMessageBatchResult) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		SendMessageBatchResultEntryListWriter::write_params(params, &(prefix.to_string() + "SendMessageBatchResultEntry"), &obj.successful);
		BatchResultErrorEntryListWriter::write_params(params, &(prefix.to_string() + "BatchResultErrorEntry"), &obj.failed);
	}
}

#[derive(Debug, Default)]
pub struct CreateQueueRequest {
	pub attributes: Option<AttributeMap>,
	pub queue_name: String,
}

pub struct CreateQueueRequestParser;
impl CreateQueueRequestParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<CreateQueueRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = CreateQueueRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "Attribute" {
				obj.attributes = Some(try!(AttributeMapParser::parse_xml("Attribute", stack)));
				continue;
			}
			if current_name == "QueueName" {
				obj.queue_name = try!(StringParser::parse_xml("QueueName", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub struct CreateQueueRequestWriter;
impl CreateQueueRequestWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &CreateQueueRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		if let Some(ref obj) = obj.attributes {
			AttributeMapWriter::write_params(params, &(prefix.to_string() + "Attribute"), obj);
		}
		StringWriter::write_params(params, &(prefix.to_string() + "QueueName"), &obj.queue_name);
	}
}

#[derive(Debug, Default)]
pub struct RemovePermissionRequest {
	pub queue_url: String,
	pub label: String,
}

pub struct RemovePermissionRequestParser;
impl RemovePermissionRequestParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<RemovePermissionRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = RemovePermissionRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "QueueUrl" {
				obj.queue_url = try!(StringParser::parse_xml("QueueUrl", stack));
				continue;
			}
			if current_name == "Label" {
				obj.label = try!(StringParser::parse_xml("Label", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub struct RemovePermissionRequestWriter;
impl RemovePermissionRequestWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &RemovePermissionRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		StringWriter::write_params(params, &(prefix.to_string() + "QueueUrl"), &obj.queue_url);
		StringWriter::write_params(params, &(prefix.to_string() + "Label"), &obj.label);
	}
}

#[derive(Debug, Default)]
pub struct DeleteMessageBatchRequestEntry {
	pub receipt_handle: String,
	pub id: String,
}

pub struct DeleteMessageBatchRequestEntryParser;
impl DeleteMessageBatchRequestEntryParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<DeleteMessageBatchRequestEntry, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = DeleteMessageBatchRequestEntry::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "ReceiptHandle" {
				obj.receipt_handle = try!(StringParser::parse_xml("ReceiptHandle", stack));
				continue;
			}
			if current_name == "Id" {
				obj.id = try!(StringParser::parse_xml("Id", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub struct DeleteMessageBatchRequestEntryWriter;
impl DeleteMessageBatchRequestEntryWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &DeleteMessageBatchRequestEntry) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		StringWriter::write_params(params, &(prefix.to_string() + "ReceiptHandle"), &obj.receipt_handle);
		StringWriter::write_params(params, &(prefix.to_string() + "Id"), &obj.id);
	}
}

#[derive(Debug, Default)]
pub struct SendMessageBatchRequest {
	pub queue_url: String,
	pub entries: SendMessageBatchRequestEntryList,
}

pub struct SendMessageBatchRequestParser;
impl SendMessageBatchRequestParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<SendMessageBatchRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = SendMessageBatchRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "QueueUrl" {
				obj.queue_url = try!(StringParser::parse_xml("QueueUrl", stack));
				continue;
			}
			if current_name == "SendMessageBatchRequestEntry" {
				obj.entries = try!(SendMessageBatchRequestEntryListParser::parse_xml("SendMessageBatchRequestEntry", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub struct SendMessageBatchRequestWriter;
impl SendMessageBatchRequestWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &SendMessageBatchRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		StringWriter::write_params(params, &(prefix.to_string() + "QueueUrl"), &obj.queue_url);
		SendMessageBatchRequestEntryListWriter::write_params(params, &(prefix.to_string() + "SendMessageBatchRequestEntry"), &obj.entries);
	}
}

#[derive(Debug, Default)]
pub struct ChangeMessageVisibilityBatchResult {
	pub successful: ChangeMessageVisibilityBatchResultEntryList,
	pub failed: BatchResultErrorEntryList,
}

pub struct ChangeMessageVisibilityBatchResultParser;
impl ChangeMessageVisibilityBatchResultParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ChangeMessageVisibilityBatchResult, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = ChangeMessageVisibilityBatchResult::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "ChangeMessageVisibilityBatchResultEntry" {
				obj.successful = try!(ChangeMessageVisibilityBatchResultEntryListParser::parse_xml("ChangeMessageVisibilityBatchResultEntry", stack));
				continue;
			}
			if current_name == "BatchResultErrorEntry" {
				obj.failed = try!(BatchResultErrorEntryListParser::parse_xml("BatchResultErrorEntry", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub struct ChangeMessageVisibilityBatchResultWriter;
impl ChangeMessageVisibilityBatchResultWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &ChangeMessageVisibilityBatchResult) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		ChangeMessageVisibilityBatchResultEntryListWriter::write_params(params, &(prefix.to_string() + "ChangeMessageVisibilityBatchResultEntry"), &obj.successful);
		BatchResultErrorEntryListWriter::write_params(params, &(prefix.to_string() + "BatchResultErrorEntry"), &obj.failed);
	}
}

#[derive(Debug, Default)]
pub struct CreateQueueResult {
	pub queue_url: String,
}

pub struct CreateQueueResultParser;
impl CreateQueueResultParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<CreateQueueResult, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = CreateQueueResult::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "QueueUrl" {
				obj.queue_url = try!(StringParser::parse_xml("QueueUrl", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub struct CreateQueueResultWriter;
impl CreateQueueResultWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &CreateQueueResult) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		StringWriter::write_params(params, &(prefix.to_string() + "QueueUrl"), &obj.queue_url);
	}
}

#[derive(Debug, Default)]
pub struct PurgeQueueRequest {
	pub queue_url: String,
}

pub struct PurgeQueueRequestParser;
impl PurgeQueueRequestParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<PurgeQueueRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = PurgeQueueRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "QueueUrl" {
				obj.queue_url = try!(StringParser::parse_xml("QueueUrl", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub struct PurgeQueueRequestWriter;
impl PurgeQueueRequestWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &PurgeQueueRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		StringWriter::write_params(params, &(prefix.to_string() + "QueueUrl"), &obj.queue_url);
	}
}

#[derive(Debug, Default)]
pub struct ReceiptHandleIsInvalid;

pub struct ReceiptHandleIsInvalidParser;
impl ReceiptHandleIsInvalidParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ReceiptHandleIsInvalid, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = ReceiptHandleIsInvalid::default();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub struct ReceiptHandleIsInvalidWriter;
impl ReceiptHandleIsInvalidWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &ReceiptHandleIsInvalid) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
	}
}

#[derive(Debug, Default)]
pub struct InvalidAttributeName;

pub struct InvalidAttributeNameParser;
impl InvalidAttributeNameParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<InvalidAttributeName, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = InvalidAttributeName::default();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub struct InvalidAttributeNameWriter;
impl InvalidAttributeNameWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &InvalidAttributeName) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
	}
}

#[derive(Debug, Default)]
pub struct ChangeMessageVisibilityBatchRequest {
	pub queue_url: String,
	pub entries: ChangeMessageVisibilityBatchRequestEntryList,
}

pub struct ChangeMessageVisibilityBatchRequestParser;
impl ChangeMessageVisibilityBatchRequestParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ChangeMessageVisibilityBatchRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = ChangeMessageVisibilityBatchRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "QueueUrl" {
				obj.queue_url = try!(StringParser::parse_xml("QueueUrl", stack));
				continue;
			}
			if current_name == "ChangeMessageVisibilityBatchRequestEntry" {
				obj.entries = try!(ChangeMessageVisibilityBatchRequestEntryListParser::parse_xml("ChangeMessageVisibilityBatchRequestEntry", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub struct ChangeMessageVisibilityBatchRequestWriter;
impl ChangeMessageVisibilityBatchRequestWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &ChangeMessageVisibilityBatchRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		StringWriter::write_params(params, &(prefix.to_string() + "QueueUrl"), &obj.queue_url);
		ChangeMessageVisibilityBatchRequestEntryListWriter::write_params(params, &(prefix.to_string() + "ChangeMessageVisibilityBatchRequestEntry"), &obj.entries);
	}
}
pub type MessageAttributeMap = HashMap<String,MessageAttributeValue>;
pub struct MessageAttributeMapParser;
impl MessageAttributeMapParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<MessageAttributeMap, XmlParseError> {
		let mut obj = HashMap::new();
		while try!(peek_at_name(stack)) == tag_name {
			try!(start_element(tag_name, stack));
			let key = try!(StringParser::parse_xml("Name", stack));
			let value = try!(MessageAttributeValueParser::parse_xml("Value", stack));
			obj.insert(key, value);
			try!(end_element(tag_name, stack));
		}
		Ok(obj)
	}
}
pub struct MessageAttributeMapWriter;
impl MessageAttributeMapWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &MessageAttributeMap) {
		let mut index = 1;
		for (key,value) in obj {
			let prefix = &format!("{}.{}", name, index);
			StringWriter::write_params(params, &format!("{}.{}", prefix, "Name"), &key);
			MessageAttributeValueWriter::write_params(params, &format!("{}.{}", prefix, "Value"), &value);
			index += 1;
		}
	}
}

#[derive(Debug, Default)]
pub struct ReceiveMessageRequest {
	pub queue_url: String,
	pub max_number_of_messages: Option<Integer>,
	pub wait_time_seconds: Option<Integer>,
	pub message_attribute_names: Option<MessageAttributeNameList>,
	pub visibility_timeout: Option<Integer>,
	pub attribute_names: Option<AttributeNameList>,
}

pub struct ReceiveMessageRequestParser;
impl ReceiveMessageRequestParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ReceiveMessageRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = ReceiveMessageRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "QueueUrl" {
				obj.queue_url = try!(StringParser::parse_xml("QueueUrl", stack));
				continue;
			}
			if current_name == "MaxNumberOfMessages" {
				obj.max_number_of_messages = Some(try!(IntegerParser::parse_xml("MaxNumberOfMessages", stack)));
				continue;
			}
			if current_name == "WaitTimeSeconds" {
				obj.wait_time_seconds = Some(try!(IntegerParser::parse_xml("WaitTimeSeconds", stack)));
				continue;
			}
			if current_name == "MessageAttributeName" {
				obj.message_attribute_names = Some(try!(MessageAttributeNameListParser::parse_xml("MessageAttributeName", stack)));
				continue;
			}
			if current_name == "VisibilityTimeout" {
				obj.visibility_timeout = Some(try!(IntegerParser::parse_xml("VisibilityTimeout", stack)));
				continue;
			}
			if current_name == "AttributeName" {
				obj.attribute_names = Some(try!(AttributeNameListParser::parse_xml("AttributeName", stack)));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub struct ReceiveMessageRequestWriter;
impl ReceiveMessageRequestWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &ReceiveMessageRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		StringWriter::write_params(params, &(prefix.to_string() + "QueueUrl"), &obj.queue_url);
		if let Some(ref obj) = obj.max_number_of_messages {
			IntegerWriter::write_params(params, &(prefix.to_string() + "MaxNumberOfMessages"), obj);
		}
		if let Some(ref obj) = obj.wait_time_seconds {
			IntegerWriter::write_params(params, &(prefix.to_string() + "WaitTimeSeconds"), obj);
		}
		if let Some(ref obj) = obj.message_attribute_names {
			MessageAttributeNameListWriter::write_params(params, &(prefix.to_string() + "MessageAttributeName"), obj);
		}
		if let Some(ref obj) = obj.visibility_timeout {
			IntegerWriter::write_params(params, &(prefix.to_string() + "VisibilityTimeout"), obj);
		}
		if let Some(ref obj) = obj.attribute_names {
			AttributeNameListWriter::write_params(params, &(prefix.to_string() + "AttributeName"), obj);
		}
	}
}

#[derive(Debug, Default)]
pub struct ReceiveMessageResult {
	pub messages: MessageList,
}

pub struct ReceiveMessageResultParser;
impl ReceiveMessageResultParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ReceiveMessageResult, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = ReceiveMessageResult::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "Message" {
				obj.messages = try!(MessageListParser::parse_xml("Message", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub struct ReceiveMessageResultWriter;
impl ReceiveMessageResultWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &ReceiveMessageResult) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		MessageListWriter::write_params(params, &(prefix.to_string() + "Message"), &obj.messages);
	}
}
pub struct StringParser;
impl StringParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<String, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub struct StringWriter;
impl StringWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &String) {
		params.put(name, obj);
	}
}
pub type ChangeMessageVisibilityBatchRequestEntryList = Vec<ChangeMessageVisibilityBatchRequestEntry>;
pub struct ChangeMessageVisibilityBatchRequestEntryListParser;
impl ChangeMessageVisibilityBatchRequestEntryListParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ChangeMessageVisibilityBatchRequestEntryList, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "ChangeMessageVisibilityBatchRequestEntry" {
			obj.push(try!(ChangeMessageVisibilityBatchRequestEntryParser::parse_xml("ChangeMessageVisibilityBatchRequestEntry", stack)));
		}
		Ok(obj)
	}
}
pub struct ChangeMessageVisibilityBatchRequestEntryListWriter;
impl ChangeMessageVisibilityBatchRequestEntryListWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &ChangeMessageVisibilityBatchRequestEntryList) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			ChangeMessageVisibilityBatchRequestEntryWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
pub type SendMessageBatchResultEntryList = Vec<SendMessageBatchResultEntry>;
pub struct SendMessageBatchResultEntryListParser;
impl SendMessageBatchResultEntryListParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<SendMessageBatchResultEntryList, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "SendMessageBatchResultEntry" {
			obj.push(try!(SendMessageBatchResultEntryParser::parse_xml("SendMessageBatchResultEntry", stack)));
		}
		Ok(obj)
	}
}
pub struct SendMessageBatchResultEntryListWriter;
impl SendMessageBatchResultEntryListWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &SendMessageBatchResultEntryList) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			SendMessageBatchResultEntryWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}

#[derive(Debug, Default)]
pub struct SendMessageRequest {
	pub queue_url: String,
	pub delay_seconds: Option<Integer>,
	pub message_body: String,
	pub message_attributes: Option<MessageAttributeMap>,
}

pub struct SendMessageRequestParser;
impl SendMessageRequestParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<SendMessageRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = SendMessageRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "QueueUrl" {
				obj.queue_url = try!(StringParser::parse_xml("QueueUrl", stack));
				continue;
			}
			if current_name == "DelaySeconds" {
				obj.delay_seconds = Some(try!(IntegerParser::parse_xml("DelaySeconds", stack)));
				continue;
			}
			if current_name == "MessageBody" {
				obj.message_body = try!(StringParser::parse_xml("MessageBody", stack));
				continue;
			}
			if current_name == "MessageAttribute" {
				obj.message_attributes = Some(try!(MessageAttributeMapParser::parse_xml("MessageAttribute", stack)));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub struct SendMessageRequestWriter;
impl SendMessageRequestWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &SendMessageRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		StringWriter::write_params(params, &(prefix.to_string() + "QueueUrl"), &obj.queue_url);
		if let Some(ref obj) = obj.delay_seconds {
			IntegerWriter::write_params(params, &(prefix.to_string() + "DelaySeconds"), obj);
		}
		StringWriter::write_params(params, &(prefix.to_string() + "MessageBody"), &obj.message_body);
		if let Some(ref obj) = obj.message_attributes {
			MessageAttributeMapWriter::write_params(params, &(prefix.to_string() + "MessageAttribute"), obj);
		}
	}
}

#[derive(Debug, Default)]
pub struct SendMessageBatchResultEntry {
	pub md5_of_message_body: String,
	pub md5_of_message_attributes: Option<String>,
	pub id: String,
	pub message_id: String,
}

pub struct SendMessageBatchResultEntryParser;
impl SendMessageBatchResultEntryParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<SendMessageBatchResultEntry, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = SendMessageBatchResultEntry::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "MD5OfMessageBody" {
				obj.md5_of_message_body = try!(StringParser::parse_xml("MD5OfMessageBody", stack));
				continue;
			}
			if current_name == "MD5OfMessageAttributes" {
				obj.md5_of_message_attributes = Some(try!(StringParser::parse_xml("MD5OfMessageAttributes", stack)));
				continue;
			}
			if current_name == "Id" {
				obj.id = try!(StringParser::parse_xml("Id", stack));
				continue;
			}
			if current_name == "MessageId" {
				obj.message_id = try!(StringParser::parse_xml("MessageId", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub struct SendMessageBatchResultEntryWriter;
impl SendMessageBatchResultEntryWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &SendMessageBatchResultEntry) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		StringWriter::write_params(params, &(prefix.to_string() + "MD5OfMessageBody"), &obj.md5_of_message_body);
		if let Some(ref obj) = obj.md5_of_message_attributes {
			StringWriter::write_params(params, &(prefix.to_string() + "MD5OfMessageAttributes"), obj);
		}
		StringWriter::write_params(params, &(prefix.to_string() + "Id"), &obj.id);
		StringWriter::write_params(params, &(prefix.to_string() + "MessageId"), &obj.message_id);
	}
}
pub type DeleteMessageBatchResultEntryList = Vec<DeleteMessageBatchResultEntry>;
pub struct DeleteMessageBatchResultEntryListParser;
impl DeleteMessageBatchResultEntryListParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<DeleteMessageBatchResultEntryList, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "DeleteMessageBatchResultEntry" {
			obj.push(try!(DeleteMessageBatchResultEntryParser::parse_xml("DeleteMessageBatchResultEntry", stack)));
		}
		Ok(obj)
	}
}
pub struct DeleteMessageBatchResultEntryListWriter;
impl DeleteMessageBatchResultEntryListWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &DeleteMessageBatchResultEntryList) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			DeleteMessageBatchResultEntryWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
pub type StringList = Vec<String>;
pub struct StringListParser;
impl StringListParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<StringList, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "StringListValue" {
			obj.push(try!(StringParser::parse_xml("StringListValue", stack)));
		}
		Ok(obj)
	}
}
pub struct StringListWriter;
impl StringListWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &StringList) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			StringWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
pub type BatchResultErrorEntryList = Vec<BatchResultErrorEntry>;
pub struct BatchResultErrorEntryListParser;
impl BatchResultErrorEntryListParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<BatchResultErrorEntryList, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "BatchResultErrorEntry" {
			obj.push(try!(BatchResultErrorEntryParser::parse_xml("BatchResultErrorEntry", stack)));
		}
		Ok(obj)
	}
}
pub struct BatchResultErrorEntryListWriter;
impl BatchResultErrorEntryListWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &BatchResultErrorEntryList) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			BatchResultErrorEntryWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}

#[derive(Debug, Default)]
pub struct InvalidBatchEntryId;

pub struct InvalidBatchEntryIdParser;
impl InvalidBatchEntryIdParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<InvalidBatchEntryId, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = InvalidBatchEntryId::default();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub struct InvalidBatchEntryIdWriter;
impl InvalidBatchEntryIdWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &InvalidBatchEntryId) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
	}
}

#[derive(Debug, Default)]
pub struct ChangeMessageVisibilityRequest {
	pub queue_url: String,
	pub receipt_handle: String,
	pub visibility_timeout: Integer,
}

pub struct ChangeMessageVisibilityRequestParser;
impl ChangeMessageVisibilityRequestParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ChangeMessageVisibilityRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = ChangeMessageVisibilityRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "QueueUrl" {
				obj.queue_url = try!(StringParser::parse_xml("QueueUrl", stack));
				continue;
			}
			if current_name == "ReceiptHandle" {
				obj.receipt_handle = try!(StringParser::parse_xml("ReceiptHandle", stack));
				continue;
			}
			if current_name == "VisibilityTimeout" {
				obj.visibility_timeout = try!(IntegerParser::parse_xml("VisibilityTimeout", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub struct ChangeMessageVisibilityRequestWriter;
impl ChangeMessageVisibilityRequestWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &ChangeMessageVisibilityRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		StringWriter::write_params(params, &(prefix.to_string() + "QueueUrl"), &obj.queue_url);
		StringWriter::write_params(params, &(prefix.to_string() + "ReceiptHandle"), &obj.receipt_handle);
		IntegerWriter::write_params(params, &(prefix.to_string() + "VisibilityTimeout"), &obj.visibility_timeout);
	}
}
pub type ChangeMessageVisibilityBatchResultEntryList = Vec<ChangeMessageVisibilityBatchResultEntry>;
pub struct ChangeMessageVisibilityBatchResultEntryListParser;
impl ChangeMessageVisibilityBatchResultEntryListParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ChangeMessageVisibilityBatchResultEntryList, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "ChangeMessageVisibilityBatchResultEntry" {
			obj.push(try!(ChangeMessageVisibilityBatchResultEntryParser::parse_xml("ChangeMessageVisibilityBatchResultEntry", stack)));
		}
		Ok(obj)
	}
}
pub struct ChangeMessageVisibilityBatchResultEntryListWriter;
impl ChangeMessageVisibilityBatchResultEntryListWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &ChangeMessageVisibilityBatchResultEntryList) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			ChangeMessageVisibilityBatchResultEntryWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}

#[derive(Debug, Default)]
pub struct BatchResultErrorEntry {
	pub message: Option<String>,
	pub sender_fault: Boolean,
	pub code: String,
	pub id: String,
}

pub struct BatchResultErrorEntryParser;
impl BatchResultErrorEntryParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<BatchResultErrorEntry, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = BatchResultErrorEntry::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "Message" {
				obj.message = Some(try!(StringParser::parse_xml("Message", stack)));
				continue;
			}
			if current_name == "SenderFault" {
				obj.sender_fault = try!(BooleanParser::parse_xml("SenderFault", stack));
				continue;
			}
			if current_name == "Code" {
				obj.code = try!(StringParser::parse_xml("Code", stack));
				continue;
			}
			if current_name == "Id" {
				obj.id = try!(StringParser::parse_xml("Id", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub struct BatchResultErrorEntryWriter;
impl BatchResultErrorEntryWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &BatchResultErrorEntry) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		if let Some(ref obj) = obj.message {
			StringWriter::write_params(params, &(prefix.to_string() + "Message"), obj);
		}
		BooleanWriter::write_params(params, &(prefix.to_string() + "SenderFault"), &obj.sender_fault);
		StringWriter::write_params(params, &(prefix.to_string() + "Code"), &obj.code);
		StringWriter::write_params(params, &(prefix.to_string() + "Id"), &obj.id);
	}
}

#[derive(Debug, Default)]
pub struct SendMessageResult {
	pub md5_of_message_body: String,
	pub md5_of_message_attributes: String,
	pub message_id: String,
}

pub struct SendMessageResultParser;
impl SendMessageResultParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<SendMessageResult, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = SendMessageResult::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "MD5OfMessageBody" {
				obj.md5_of_message_body = try!(StringParser::parse_xml("MD5OfMessageBody", stack));
				continue;
			}
			if current_name == "MD5OfMessageAttributes" {
				obj.md5_of_message_attributes = try!(StringParser::parse_xml("MD5OfMessageAttributes", stack));
				continue;
			}
			if current_name == "MessageId" {
				obj.message_id = try!(StringParser::parse_xml("MessageId", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub struct SendMessageResultWriter;
impl SendMessageResultWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &SendMessageResult) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		StringWriter::write_params(params, &(prefix.to_string() + "MD5OfMessageBody"), &obj.md5_of_message_body);
		StringWriter::write_params(params, &(prefix.to_string() + "MD5OfMessageAttributes"), &obj.md5_of_message_attributes);
		StringWriter::write_params(params, &(prefix.to_string() + "MessageId"), &obj.message_id);
	}
}

#[derive(Debug, Default)]
pub struct DeleteMessageBatchRequest {
	pub queue_url: String,
	pub entries: DeleteMessageBatchRequestEntryList,
}

pub struct DeleteMessageBatchRequestParser;
impl DeleteMessageBatchRequestParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<DeleteMessageBatchRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = DeleteMessageBatchRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "QueueUrl" {
				obj.queue_url = try!(StringParser::parse_xml("QueueUrl", stack));
				continue;
			}
			if current_name == "DeleteMessageBatchRequestEntry" {
				obj.entries = try!(DeleteMessageBatchRequestEntryListParser::parse_xml("DeleteMessageBatchRequestEntry", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub struct DeleteMessageBatchRequestWriter;
impl DeleteMessageBatchRequestWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &DeleteMessageBatchRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		StringWriter::write_params(params, &(prefix.to_string() + "QueueUrl"), &obj.queue_url);
		DeleteMessageBatchRequestEntryListWriter::write_params(params, &(prefix.to_string() + "DeleteMessageBatchRequestEntry"), &obj.entries);
	}
}
pub type Integer = i32;
pub struct IntegerParser;
impl IntegerParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<Integer, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = i32::from_str(try!(characters(stack)).as_ref()).unwrap();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub struct IntegerWriter;
impl IntegerWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &Integer) {
		params.put(name, &obj.to_string());
	}
}

#[derive(Debug, Default)]
pub struct QueueDoesNotExist;

pub struct QueueDoesNotExistParser;
impl QueueDoesNotExistParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<QueueDoesNotExist, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = QueueDoesNotExist::default();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub struct QueueDoesNotExistWriter;
impl QueueDoesNotExistWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &QueueDoesNotExist) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
	}
}

#[derive(Debug, Default)]
pub struct InvalidMessageContents;

pub struct InvalidMessageContentsParser;
impl InvalidMessageContentsParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<InvalidMessageContents, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = InvalidMessageContents::default();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub struct InvalidMessageContentsWriter;
impl InvalidMessageContentsWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &InvalidMessageContents) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
	}
}

#[derive(Debug, Default)]
pub struct MessageNotInflight;

pub struct MessageNotInflightParser;
impl MessageNotInflightParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<MessageNotInflight, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = MessageNotInflight::default();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub struct MessageNotInflightWriter;
impl MessageNotInflightWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &MessageNotInflight) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
	}
}
pub type MessageAttributeName = String;
pub struct MessageAttributeNameParser;
impl MessageAttributeNameParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<MessageAttributeName, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub struct MessageAttributeNameWriter;
impl MessageAttributeNameWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &MessageAttributeName) {
		params.put(name, obj);
	}
}

#[derive(Debug, Default)]
pub struct GetQueueAttributesResult {
	pub attributes: AttributeMap,
}

pub struct GetQueueAttributesResultParser;
impl GetQueueAttributesResultParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<GetQueueAttributesResult, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = GetQueueAttributesResult::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "Attribute" {
				obj.attributes = try!(AttributeMapParser::parse_xml("Attribute", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub struct GetQueueAttributesResultWriter;
impl GetQueueAttributesResultWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &GetQueueAttributesResult) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		AttributeMapWriter::write_params(params, &(prefix.to_string() + "Attribute"), &obj.attributes);
	}
}
pub type BinaryList = Vec<Binary>;
pub struct BinaryListParser;
impl BinaryListParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<BinaryList, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "BinaryListValue" {
			obj.push(try!(BinaryParser::parse_xml("BinaryListValue", stack)));
		}
		Ok(obj)
	}
}
pub struct BinaryListWriter;
impl BinaryListWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &BinaryList) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			BinaryWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}

#[derive(Debug, Default)]
pub struct ListDeadLetterSourceQueuesRequest {
	pub queue_url: String,
}

pub struct ListDeadLetterSourceQueuesRequestParser;
impl ListDeadLetterSourceQueuesRequestParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ListDeadLetterSourceQueuesRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = ListDeadLetterSourceQueuesRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "QueueUrl" {
				obj.queue_url = try!(StringParser::parse_xml("QueueUrl", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub struct ListDeadLetterSourceQueuesRequestWriter;
impl ListDeadLetterSourceQueuesRequestWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &ListDeadLetterSourceQueuesRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		StringWriter::write_params(params, &(prefix.to_string() + "QueueUrl"), &obj.queue_url);
	}
}
pub type MessageList = Vec<Message>;
pub struct MessageListParser;
impl MessageListParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<MessageList, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "Message" {
			obj.push(try!(MessageParser::parse_xml("Message", stack)));
		}
		Ok(obj)
	}
}
pub struct MessageListWriter;
impl MessageListWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &MessageList) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			MessageWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
pub type SendMessageBatchRequestEntryList = Vec<SendMessageBatchRequestEntry>;
pub struct SendMessageBatchRequestEntryListParser;
impl SendMessageBatchRequestEntryListParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<SendMessageBatchRequestEntryList, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "SendMessageBatchRequestEntry" {
			obj.push(try!(SendMessageBatchRequestEntryParser::parse_xml("SendMessageBatchRequestEntry", stack)));
		}
		Ok(obj)
	}
}
pub struct SendMessageBatchRequestEntryListWriter;
impl SendMessageBatchRequestEntryListWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &SendMessageBatchRequestEntryList) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			SendMessageBatchRequestEntryWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}

#[derive(Debug, Default)]
pub struct Message {
	pub body: String,
	pub receipt_handle: String,
	pub md5_of_body: String,
	pub md5_of_message_attributes: String,
	pub message_id: String,
	pub attributes: AttributeMap,
	pub message_attributes: MessageAttributeMap,
}

pub struct MessageParser;
impl MessageParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<Message, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = Message::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "Body" {
				obj.body = try!(StringParser::parse_xml("Body", stack));
				continue;
			}
			if current_name == "ReceiptHandle" {
				obj.receipt_handle = try!(StringParser::parse_xml("ReceiptHandle", stack));
				continue;
			}
			if current_name == "MD5OfBody" {
				obj.md5_of_body = try!(StringParser::parse_xml("MD5OfBody", stack));
				continue;
			}
			if current_name == "MD5OfMessageAttributes" {
				obj.md5_of_message_attributes = try!(StringParser::parse_xml("MD5OfMessageAttributes", stack));
				continue;
			}
			if current_name == "MessageId" {
				obj.message_id = try!(StringParser::parse_xml("MessageId", stack));
				continue;
			}
			if current_name == "Attribute" {
				obj.attributes = try!(AttributeMapParser::parse_xml("Attribute", stack));
				continue;
			}
			if current_name == "MessageAttribute" {
				obj.message_attributes = try!(MessageAttributeMapParser::parse_xml("MessageAttribute", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub struct MessageWriter;
impl MessageWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &Message) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		StringWriter::write_params(params, &(prefix.to_string() + "Body"), &obj.body);
		StringWriter::write_params(params, &(prefix.to_string() + "ReceiptHandle"), &obj.receipt_handle);
		StringWriter::write_params(params, &(prefix.to_string() + "MD5OfBody"), &obj.md5_of_body);
		StringWriter::write_params(params, &(prefix.to_string() + "MD5OfMessageAttributes"), &obj.md5_of_message_attributes);
		StringWriter::write_params(params, &(prefix.to_string() + "MessageId"), &obj.message_id);
		AttributeMapWriter::write_params(params, &(prefix.to_string() + "Attribute"), &obj.attributes);
		MessageAttributeMapWriter::write_params(params, &(prefix.to_string() + "MessageAttribute"), &obj.message_attributes);
	}
}

#[derive(Debug, Default)]
pub struct OverLimit;

pub struct OverLimitParser;
impl OverLimitParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<OverLimit, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = OverLimit::default();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub struct OverLimitWriter;
impl OverLimitWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &OverLimit) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
	}
}

#[derive(Debug, Default)]
pub struct GetQueueUrlRequest {
	pub queue_name: String,
	pub queue_owner_aws_account_id: Option<String>,
}

pub struct GetQueueUrlRequestParser;
impl GetQueueUrlRequestParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<GetQueueUrlRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = GetQueueUrlRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "QueueName" {
				obj.queue_name = try!(StringParser::parse_xml("QueueName", stack));
				continue;
			}
			if current_name == "QueueOwnerAWSAccountId" {
				obj.queue_owner_aws_account_id = Some(try!(StringParser::parse_xml("QueueOwnerAWSAccountId", stack)));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub struct GetQueueUrlRequestWriter;
impl GetQueueUrlRequestWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &GetQueueUrlRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		StringWriter::write_params(params, &(prefix.to_string() + "QueueName"), &obj.queue_name);
		if let Some(ref obj) = obj.queue_owner_aws_account_id {
			StringWriter::write_params(params, &(prefix.to_string() + "QueueOwnerAWSAccountId"), obj);
		}
	}
}
pub type AttributeMap = HashMap<QueueAttributeName,String>;
pub struct AttributeMapParser;
impl AttributeMapParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<AttributeMap, XmlParseError> {
		let mut obj = HashMap::new();
		while try!(peek_at_name(stack)) == tag_name {
			try!(start_element(tag_name, stack));
			let key = try!(QueueAttributeNameParser::parse_xml("Name", stack));
			let value = try!(StringParser::parse_xml("Value", stack));
			obj.insert(key, value);
			try!(end_element(tag_name, stack));
		}
		Ok(obj)
	}
}
pub struct AttributeMapWriter;
impl AttributeMapWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &AttributeMap) {
		let mut index = 1;
		for (key,value) in obj {
			let prefix = &format!("{}.{}", name, index);
			QueueAttributeNameWriter::write_params(params, &format!("{}.{}", prefix, "Name"), &key);
			StringWriter::write_params(params, &format!("{}.{}", prefix, "Value"), &value);
			index += 1;
		}
	}
}

#[derive(Debug, Default)]
pub struct DeleteQueueRequest {
	pub queue_url: String,
}

pub struct DeleteQueueRequestParser;
impl DeleteQueueRequestParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<DeleteQueueRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = DeleteQueueRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "QueueUrl" {
				obj.queue_url = try!(StringParser::parse_xml("QueueUrl", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub struct DeleteQueueRequestWriter;
impl DeleteQueueRequestWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &DeleteQueueRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		StringWriter::write_params(params, &(prefix.to_string() + "QueueUrl"), &obj.queue_url);
	}
}
pub type ActionNameList = Vec<String>;
pub struct ActionNameListParser;
impl ActionNameListParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ActionNameList, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "ActionName" {
			obj.push(try!(StringParser::parse_xml("ActionName", stack)));
		}
		Ok(obj)
	}
}
pub struct ActionNameListWriter;
impl ActionNameListWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &ActionNameList) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			StringWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}

#[derive(Debug, Default)]
pub struct BatchRequestTooLong;

pub struct BatchRequestTooLongParser;
impl BatchRequestTooLongParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<BatchRequestTooLong, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = BatchRequestTooLong::default();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub struct BatchRequestTooLongWriter;
impl BatchRequestTooLongWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &BatchRequestTooLong) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
	}
}

#[derive(Debug, Default)]
pub struct GetQueueUrlResult {
	pub queue_url: String,
}

pub struct GetQueueUrlResultParser;
impl GetQueueUrlResultParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<GetQueueUrlResult, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = GetQueueUrlResult::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "QueueUrl" {
				obj.queue_url = try!(StringParser::parse_xml("QueueUrl", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub struct GetQueueUrlResultWriter;
impl GetQueueUrlResultWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &GetQueueUrlResult) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		StringWriter::write_params(params, &(prefix.to_string() + "QueueUrl"), &obj.queue_url);
	}
}

#[derive(Debug, Default)]
pub struct ListQueuesRequest {
	pub queue_name_prefix: String,
}

pub struct ListQueuesRequestParser;
impl ListQueuesRequestParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ListQueuesRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = ListQueuesRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "QueueNamePrefix" {
				obj.queue_name_prefix = try!(StringParser::parse_xml("QueueNamePrefix", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub struct ListQueuesRequestWriter;
impl ListQueuesRequestWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &ListQueuesRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		StringWriter::write_params(params, &(prefix.to_string() + "QueueNamePrefix"), &obj.queue_name_prefix);
	}
}
pub type AWSAccountIdList = Vec<String>;
pub struct AWSAccountIdListParser;
impl AWSAccountIdListParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<AWSAccountIdList, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "AWSAccountId" {
			obj.push(try!(StringParser::parse_xml("AWSAccountId", stack)));
		}
		Ok(obj)
	}
}
pub struct AWSAccountIdListWriter;
impl AWSAccountIdListWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &AWSAccountIdList) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			StringWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}

#[derive(Debug, Default)]
pub struct InvalidIdFormat;

pub struct InvalidIdFormatParser;
impl InvalidIdFormatParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<InvalidIdFormat, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = InvalidIdFormat::default();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub struct InvalidIdFormatWriter;
impl InvalidIdFormatWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &InvalidIdFormat) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
	}
}

#[derive(Debug, Default)]
pub struct AddPermissionRequest {
	pub queue_url: String,
	pub aws_account_ids: AWSAccountIdList,
	pub actions: ActionNameList,
	pub label: String,
}

pub struct AddPermissionRequestParser;
impl AddPermissionRequestParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<AddPermissionRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = AddPermissionRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "QueueUrl" {
				obj.queue_url = try!(StringParser::parse_xml("QueueUrl", stack));
				continue;
			}
			if current_name == "AWSAccountId" {
				obj.aws_account_ids = try!(AWSAccountIdListParser::parse_xml("AWSAccountId", stack));
				continue;
			}
			if current_name == "ActionName" {
				obj.actions = try!(ActionNameListParser::parse_xml("ActionName", stack));
				continue;
			}
			if current_name == "Label" {
				obj.label = try!(StringParser::parse_xml("Label", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub struct AddPermissionRequestWriter;
impl AddPermissionRequestWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &AddPermissionRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		StringWriter::write_params(params, &(prefix.to_string() + "QueueUrl"), &obj.queue_url);
		AWSAccountIdListWriter::write_params(params, &(prefix.to_string() + "AWSAccountId"), &obj.aws_account_ids);
		ActionNameListWriter::write_params(params, &(prefix.to_string() + "ActionName"), &obj.actions);
		StringWriter::write_params(params, &(prefix.to_string() + "Label"), &obj.label);
	}
}
pub type QueueUrlList = Vec<String>;
pub struct QueueUrlListParser;
impl QueueUrlListParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<QueueUrlList, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "QueueUrl" {
			obj.push(try!(StringParser::parse_xml("QueueUrl", stack)));
		}
		Ok(obj)
	}
}
pub struct QueueUrlListWriter;
impl QueueUrlListWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &QueueUrlList) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			StringWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}

#[derive(Debug, Default)]
pub struct ListDeadLetterSourceQueuesResult {
	pub queue_urls: QueueUrlList,
}

pub struct ListDeadLetterSourceQueuesResultParser;
impl ListDeadLetterSourceQueuesResultParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ListDeadLetterSourceQueuesResult, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = ListDeadLetterSourceQueuesResult::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "QueueUrl" {
				obj.queue_urls = try!(QueueUrlListParser::parse_xml("QueueUrl", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub struct ListDeadLetterSourceQueuesResultWriter;
impl ListDeadLetterSourceQueuesResultWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &ListDeadLetterSourceQueuesResult) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		QueueUrlListWriter::write_params(params, &(prefix.to_string() + "QueueUrl"), &obj.queue_urls);
	}
}
pub type QueueAttributeName = String;
pub struct QueueAttributeNameParser;
impl QueueAttributeNameParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<QueueAttributeName, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub struct QueueAttributeNameWriter;
impl QueueAttributeNameWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &QueueAttributeName) {
		params.put(name, obj);
	}
}

#[derive(Debug, Default)]
pub struct QueueNameExists;

pub struct QueueNameExistsParser;
impl QueueNameExistsParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<QueueNameExists, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = QueueNameExists::default();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub struct QueueNameExistsWriter;
impl QueueNameExistsWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &QueueNameExists) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
	}
}

#[derive(Debug, Default)]
pub struct BatchEntryIdsNotDistinct;

pub struct BatchEntryIdsNotDistinctParser;
impl BatchEntryIdsNotDistinctParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<BatchEntryIdsNotDistinct, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = BatchEntryIdsNotDistinct::default();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub struct BatchEntryIdsNotDistinctWriter;
impl BatchEntryIdsNotDistinctWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &BatchEntryIdsNotDistinct) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
	}
}

#[derive(Debug, Default)]
pub struct DeleteMessageBatchResultEntry {
	pub id: String,
}

pub struct DeleteMessageBatchResultEntryParser;
impl DeleteMessageBatchResultEntryParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<DeleteMessageBatchResultEntry, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = DeleteMessageBatchResultEntry::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "Id" {
				obj.id = try!(StringParser::parse_xml("Id", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub struct DeleteMessageBatchResultEntryWriter;
impl DeleteMessageBatchResultEntryWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &DeleteMessageBatchResultEntry) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		StringWriter::write_params(params, &(prefix.to_string() + "Id"), &obj.id);
	}
}
pub type DeleteMessageBatchRequestEntryList = Vec<DeleteMessageBatchRequestEntry>;
pub struct DeleteMessageBatchRequestEntryListParser;
impl DeleteMessageBatchRequestEntryListParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<DeleteMessageBatchRequestEntryList, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "DeleteMessageBatchRequestEntry" {
			obj.push(try!(DeleteMessageBatchRequestEntryParser::parse_xml("DeleteMessageBatchRequestEntry", stack)));
		}
		Ok(obj)
	}
}
pub struct DeleteMessageBatchRequestEntryListWriter;
impl DeleteMessageBatchRequestEntryListWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &DeleteMessageBatchRequestEntryList) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			DeleteMessageBatchRequestEntryWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}

#[derive(Debug, Default)]
pub struct EmptyBatchRequest;

pub struct EmptyBatchRequestParser;
impl EmptyBatchRequestParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<EmptyBatchRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = EmptyBatchRequest::default();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub struct EmptyBatchRequestWriter;
impl EmptyBatchRequestWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &EmptyBatchRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
	}
}

#[derive(Debug, Default)]
pub struct ListQueuesResult {
	pub queue_urls: QueueUrlList,
}

pub struct ListQueuesResultParser;
impl ListQueuesResultParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ListQueuesResult, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = ListQueuesResult::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "QueueUrl" {
				obj.queue_urls = try!(QueueUrlListParser::parse_xml("QueueUrl", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub struct ListQueuesResultWriter;
impl ListQueuesResultWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &ListQueuesResult) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		QueueUrlListWriter::write_params(params, &(prefix.to_string() + "QueueUrl"), &obj.queue_urls);
	}
}

#[derive(Debug, Default)]
pub struct ChangeMessageVisibilityBatchRequestEntry {
	pub receipt_handle: String,
	pub visibility_timeout: Option<Integer>,
	pub id: String,
}

pub struct ChangeMessageVisibilityBatchRequestEntryParser;
impl ChangeMessageVisibilityBatchRequestEntryParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ChangeMessageVisibilityBatchRequestEntry, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = ChangeMessageVisibilityBatchRequestEntry::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "ReceiptHandle" {
				obj.receipt_handle = try!(StringParser::parse_xml("ReceiptHandle", stack));
				continue;
			}
			if current_name == "VisibilityTimeout" {
				obj.visibility_timeout = Some(try!(IntegerParser::parse_xml("VisibilityTimeout", stack)));
				continue;
			}
			if current_name == "Id" {
				obj.id = try!(StringParser::parse_xml("Id", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub struct ChangeMessageVisibilityBatchRequestEntryWriter;
impl ChangeMessageVisibilityBatchRequestEntryWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &ChangeMessageVisibilityBatchRequestEntry) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		StringWriter::write_params(params, &(prefix.to_string() + "ReceiptHandle"), &obj.receipt_handle);
		if let Some(ref obj) = obj.visibility_timeout {
			IntegerWriter::write_params(params, &(prefix.to_string() + "VisibilityTimeout"), obj);
		}
		StringWriter::write_params(params, &(prefix.to_string() + "Id"), &obj.id);
	}
}

#[derive(Debug, Default)]
pub struct TooManyEntriesInBatchRequest;

pub struct TooManyEntriesInBatchRequestParser;
impl TooManyEntriesInBatchRequestParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<TooManyEntriesInBatchRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = TooManyEntriesInBatchRequest::default();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub struct TooManyEntriesInBatchRequestWriter;
impl TooManyEntriesInBatchRequestWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &TooManyEntriesInBatchRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
	}
}

#[derive(Debug, Default)]
pub struct QueueDeletedRecently;

pub struct QueueDeletedRecentlyParser;
impl QueueDeletedRecentlyParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<QueueDeletedRecently, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = QueueDeletedRecently::default();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub struct QueueDeletedRecentlyWriter;
impl QueueDeletedRecentlyWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &QueueDeletedRecently) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
	}
}
pub type Boolean = bool;
pub struct BooleanParser;
impl BooleanParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<Boolean, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = bool::from_str(try!(characters(stack)).as_ref()).unwrap();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub struct BooleanWriter;
impl BooleanWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &Boolean) {
		params.put(name, &obj.to_string());
	}
}

#[derive(Debug, Default)]
pub struct DeleteMessageBatchResult {
	pub successful: DeleteMessageBatchResultEntryList,
	pub failed: BatchResultErrorEntryList,
}

pub struct DeleteMessageBatchResultParser;
impl DeleteMessageBatchResultParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<DeleteMessageBatchResult, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = DeleteMessageBatchResult::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "DeleteMessageBatchResultEntry" {
				obj.successful = try!(DeleteMessageBatchResultEntryListParser::parse_xml("DeleteMessageBatchResultEntry", stack));
				continue;
			}
			if current_name == "BatchResultErrorEntry" {
				obj.failed = try!(BatchResultErrorEntryListParser::parse_xml("BatchResultErrorEntry", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub struct DeleteMessageBatchResultWriter;
impl DeleteMessageBatchResultWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &DeleteMessageBatchResult) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		DeleteMessageBatchResultEntryListWriter::write_params(params, &(prefix.to_string() + "DeleteMessageBatchResultEntry"), &obj.successful);
		BatchResultErrorEntryListWriter::write_params(params, &(prefix.to_string() + "BatchResultErrorEntry"), &obj.failed);
	}
}

#[derive(Debug, Default)]
pub struct ChangeMessageVisibilityBatchResultEntry {
	pub id: String,
}

pub struct ChangeMessageVisibilityBatchResultEntryParser;
impl ChangeMessageVisibilityBatchResultEntryParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ChangeMessageVisibilityBatchResultEntry, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = ChangeMessageVisibilityBatchResultEntry::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "Id" {
				obj.id = try!(StringParser::parse_xml("Id", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub struct ChangeMessageVisibilityBatchResultEntryWriter;
impl ChangeMessageVisibilityBatchResultEntryWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &ChangeMessageVisibilityBatchResultEntry) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		StringWriter::write_params(params, &(prefix.to_string() + "Id"), &obj.id);
	}
}
pub type AttributeNameList = Vec<QueueAttributeName>;
pub struct AttributeNameListParser;
impl AttributeNameListParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<AttributeNameList, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "AttributeName" {
			obj.push(try!(QueueAttributeNameParser::parse_xml("AttributeName", stack)));
		}
		Ok(obj)
	}
}
pub struct AttributeNameListWriter;
impl AttributeNameListWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &AttributeNameList) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			QueueAttributeNameWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}

#[derive(Debug, Default)]
pub struct DeleteMessageRequest {
	pub queue_url: String,
	pub receipt_handle: String,
}

pub struct DeleteMessageRequestParser;
impl DeleteMessageRequestParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<DeleteMessageRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = DeleteMessageRequest::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "QueueUrl" {
				obj.queue_url = try!(StringParser::parse_xml("QueueUrl", stack));
				continue;
			}
			if current_name == "ReceiptHandle" {
				obj.receipt_handle = try!(StringParser::parse_xml("ReceiptHandle", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub struct DeleteMessageRequestWriter;
impl DeleteMessageRequestWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &DeleteMessageRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		StringWriter::write_params(params, &(prefix.to_string() + "QueueUrl"), &obj.queue_url);
		StringWriter::write_params(params, &(prefix.to_string() + "ReceiptHandle"), &obj.receipt_handle);
	}
}

#[derive(Debug, Default)]
pub struct SendMessageBatchRequestEntry {
	pub delay_seconds: Option<Integer>,
	pub id: String,
	pub message_body: String,
	pub message_attributes: Option<MessageAttributeMap>,
}

pub struct SendMessageBatchRequestEntryParser;
impl SendMessageBatchRequestEntryParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<SendMessageBatchRequestEntry, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = SendMessageBatchRequestEntry::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "DelaySeconds" {
				obj.delay_seconds = Some(try!(IntegerParser::parse_xml("DelaySeconds", stack)));
				continue;
			}
			if current_name == "Id" {
				obj.id = try!(StringParser::parse_xml("Id", stack));
				continue;
			}
			if current_name == "MessageBody" {
				obj.message_body = try!(StringParser::parse_xml("MessageBody", stack));
				continue;
			}
			if current_name == "MessageAttribute" {
				obj.message_attributes = Some(try!(MessageAttributeMapParser::parse_xml("MessageAttribute", stack)));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub struct SendMessageBatchRequestEntryWriter;
impl SendMessageBatchRequestEntryWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &SendMessageBatchRequestEntry) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		if let Some(ref obj) = obj.delay_seconds {
			IntegerWriter::write_params(params, &(prefix.to_string() + "DelaySeconds"), obj);
		}
		StringWriter::write_params(params, &(prefix.to_string() + "Id"), &obj.id);
		StringWriter::write_params(params, &(prefix.to_string() + "MessageBody"), &obj.message_body);
		if let Some(ref obj) = obj.message_attributes {
			MessageAttributeMapWriter::write_params(params, &(prefix.to_string() + "MessageAttribute"), obj);
		}
	}
}

impl AWSRequest<CreateQueueResult> for CreateQueueRequest {
	fn execute(&self, creds: &AWSCredentials, region: &str) -> Result<CreateQueueResult, AWSError> {
		let mut request = SignedRequest::new("POST", "sqs", region, "/");
		let mut params = Params::new();
		params.put("Action", "CreateQueue");
		CreateQueueRequestWriter::write_params(&mut params, "", self);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(CreateQueueResultParser::parse_xml("CreateQueueResult", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
}

impl AWSRequest<GetQueueAttributesResult> for GetQueueAttributesRequest {
	fn execute(&self, creds: &AWSCredentials, region: &str) -> Result<GetQueueAttributesResult, AWSError> {
		let mut request = SignedRequest::new("POST", "sqs", region, "/");
		let mut params = Params::new();
		params.put("Action", "GetQueueAttributes");
		GetQueueAttributesRequestWriter::write_params(&mut params, "", self);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(GetQueueAttributesResultParser::parse_xml("GetQueueAttributesResult", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
}

impl AWSRequest<()> for SetQueueAttributesRequest {
	fn execute(&self, creds: &AWSCredentials, region: &str) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "sqs", region, "/");
		let mut params = Params::new();
		params.put("Action", "SetQueueAttributes");
		SetQueueAttributesRequestWriter::write_params(&mut params, "", self);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(creds));
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

impl AWSRequest<GetQueueUrlResult> for GetQueueUrlRequest {
	fn execute(&self, creds: &AWSCredentials, region: &str) -> Result<GetQueueUrlResult, AWSError> {
		let mut request = SignedRequest::new("POST", "sqs", region, "/");
		let mut params = Params::new();
		params.put("Action", "GetQueueUrl");
		GetQueueUrlRequestWriter::write_params(&mut params, "", self);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(GetQueueUrlResultParser::parse_xml("GetQueueUrlResult", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
}

impl AWSRequest<DeleteMessageBatchResult> for DeleteMessageBatchRequest {
	fn execute(&self, creds: &AWSCredentials, region: &str) -> Result<DeleteMessageBatchResult, AWSError> {
		let mut request = SignedRequest::new("POST", "sqs", region, "/");
		let mut params = Params::new();
		params.put("Action", "DeleteMessageBatch");
		DeleteMessageBatchRequestWriter::write_params(&mut params, "", self);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(DeleteMessageBatchResultParser::parse_xml("DeleteMessageBatchResult", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
}

impl AWSRequest<SendMessageBatchResult> for SendMessageBatchRequest {
	fn execute(&self, creds: &AWSCredentials, region: &str) -> Result<SendMessageBatchResult, AWSError> {
		let mut request = SignedRequest::new("POST", "sqs", region, "/");
		let mut params = Params::new();
		params.put("Action", "SendMessageBatch");
		SendMessageBatchRequestWriter::write_params(&mut params, "", self);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(SendMessageBatchResultParser::parse_xml("SendMessageBatchResult", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
}

impl AWSRequest<ListDeadLetterSourceQueuesResult> for ListDeadLetterSourceQueuesRequest {
	fn execute(&self, creds: &AWSCredentials, region: &str) -> Result<ListDeadLetterSourceQueuesResult, AWSError> {
		let mut request = SignedRequest::new("POST", "sqs", region, "/");
		let mut params = Params::new();
		params.put("Action", "ListDeadLetterSourceQueues");
		ListDeadLetterSourceQueuesRequestWriter::write_params(&mut params, "", self);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(ListDeadLetterSourceQueuesResultParser::parse_xml("ListDeadLetterSourceQueuesResult", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
}

impl AWSRequest<()> for ChangeMessageVisibilityRequest {
	fn execute(&self, creds: &AWSCredentials, region: &str) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "sqs", region, "/");
		let mut params = Params::new();
		params.put("Action", "ChangeMessageVisibility");
		ChangeMessageVisibilityRequestWriter::write_params(&mut params, "", self);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(creds));
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

impl AWSRequest<()> for AddPermissionRequest {
	fn execute(&self, creds: &AWSCredentials, region: &str) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "sqs", region, "/");
		let mut params = Params::new();
		params.put("Action", "AddPermission");
		AddPermissionRequestWriter::write_params(&mut params, "", self);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(creds));
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

impl AWSRequest<ChangeMessageVisibilityBatchResult> for ChangeMessageVisibilityBatchRequest {
	fn execute(&self, creds: &AWSCredentials, region: &str) -> Result<ChangeMessageVisibilityBatchResult, AWSError> {
		let mut request = SignedRequest::new("POST", "sqs", region, "/");
		let mut params = Params::new();
		params.put("Action", "ChangeMessageVisibilityBatch");
		ChangeMessageVisibilityBatchRequestWriter::write_params(&mut params, "", self);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(ChangeMessageVisibilityBatchResultParser::parse_xml("ChangeMessageVisibilityBatchResult", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
}

impl AWSRequest<SendMessageResult> for SendMessageRequest {
	fn execute(&self, creds: &AWSCredentials, region: &str) -> Result<SendMessageResult, AWSError> {
		let mut request = SignedRequest::new("POST", "sqs", region, "/");
		let mut params = Params::new();
		params.put("Action", "SendMessage");
		SendMessageRequestWriter::write_params(&mut params, "", self);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(SendMessageResultParser::parse_xml("SendMessageResult", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
}

impl AWSRequest<()> for DeleteQueueRequest {
	fn execute(&self, creds: &AWSCredentials, region: &str) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "sqs", region, "/");
		let mut params = Params::new();
		params.put("Action", "DeleteQueue");
		DeleteQueueRequestWriter::write_params(&mut params, "", self);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(creds));
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

impl AWSRequest<()> for PurgeQueueRequest {
	fn execute(&self, creds: &AWSCredentials, region: &str) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "sqs", region, "/");
		let mut params = Params::new();
		params.put("Action", "PurgeQueue");
		PurgeQueueRequestWriter::write_params(&mut params, "", self);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(creds));
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

impl AWSRequest<ReceiveMessageResult> for ReceiveMessageRequest {
	fn execute(&self, creds: &AWSCredentials, region: &str) -> Result<ReceiveMessageResult, AWSError> {
		let mut request = SignedRequest::new("POST", "sqs", region, "/");
		let mut params = Params::new();
		params.put("Action", "ReceiveMessage");
		ReceiveMessageRequestWriter::write_params(&mut params, "", self);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(ReceiveMessageResultParser::parse_xml("ReceiveMessageResult", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
}

impl AWSRequest<()> for DeleteMessageRequest {
	fn execute(&self, creds: &AWSCredentials, region: &str) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "sqs", region, "/");
		let mut params = Params::new();
		params.put("Action", "DeleteMessage");
		DeleteMessageRequestWriter::write_params(&mut params, "", self);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(creds));
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

impl AWSRequest<ListQueuesResult> for ListQueuesRequest {
	fn execute(&self, creds: &AWSCredentials, region: &str) -> Result<ListQueuesResult, AWSError> {
		let mut request = SignedRequest::new("POST", "sqs", region, "/");
		let mut params = Params::new();
		params.put("Action", "ListQueues");
		ListQueuesRequestWriter::write_params(&mut params, "", self);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(ListQueuesResultParser::parse_xml("ListQueuesResult", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
}

impl AWSRequest<()> for RemovePermissionRequest {
	fn execute(&self, creds: &AWSCredentials, region: &str) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "sqs", region, "/");
		let mut params = Params::new();
		params.put("Action", "RemovePermission");
		RemovePermissionRequestWriter::write_params(&mut params, "", self);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(creds));
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
