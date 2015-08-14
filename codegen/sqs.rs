use std::collections::HashMap;
use std::str;
/// Error code 400. Unsupported operation.
#[derive(Debug, Default)]
pub struct UnsupportedOperation;

/// Parse UnsupportedOperation from XML
struct UnsupportedOperationParser;
impl UnsupportedOperationParser {
	fn parse_xml<'a, T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<UnsupportedOperation, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = UnsupportedOperation::default();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write UnsupportedOperation contents to a SignedRequest
struct UnsupportedOperationWriter;
impl UnsupportedOperationWriter {
	fn write_params(params: &mut Params, name: &str, obj: &UnsupportedOperation) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
	}
}
pub type Binary = Vec<u8>;
/// Parse Binary from XML
struct BinaryParser;
impl BinaryParser {
	fn parse_xml<'a, T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Binary, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack)).into_bytes();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write Binary contents to a SignedRequest
struct BinaryWriter;
impl BinaryWriter {
	fn write_params(params: &mut Params, name: &str, obj: &Binary) {
		params.put(name, str::from_utf8(&obj).unwrap());
	}
}
/// Indicates that the specified queue previously received a `PurgeQueue` request
/// within the last 60 seconds, the time it can take to delete the messages in the
/// queue.
#[derive(Debug, Default)]
pub struct PurgeQueueInProgress;

/// Parse PurgeQueueInProgress from XML
struct PurgeQueueInProgressParser;
impl PurgeQueueInProgressParser {
	fn parse_xml<'a, T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<PurgeQueueInProgress, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = PurgeQueueInProgress::default();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write PurgeQueueInProgress contents to a SignedRequest
struct PurgeQueueInProgressWriter;
impl PurgeQueueInProgressWriter {
	fn write_params(params: &mut Params, name: &str, obj: &PurgeQueueInProgress) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
	}
}
/// The user-specified message attribute value. For string data types, the value
/// attribute has the same restrictions on the content as the message body. For
/// more information, see [SendMessage](http://docs.aws.amazon.com/AWSSimpleQueueS
/// ervice/latest/APIReference/API_SendMessage.html).
/// Name, type, and value must not be empty or null. In addition, the message body
/// should not be empty or null. All parts of the message attribute, including
/// name, type, and value, are included in the message size restriction, which is
/// currently 256 KB (262,144 bytes).
#[derive(Debug, Default)]
pub struct MessageAttributeValue {
	/// Not implemented. Reserved for future use.
	pub binary_list_values: Option<BinaryList>,
	/// Strings are Unicode with UTF8 binary encoding. For a list of code values, see
	/// <http://en.wikipedia.org/wiki/ASCII#ASCII_printable_characters>.
	pub string_value: Option<String>,
	/// Amazon SQS supports the following logical data types: String, Number, and
	/// Binary. In addition, you can append your own custom labels. For more
	/// information, see [Message Attribute Data Types](http://docs.aws.amazon.com/AWS
	/// SimpleQueueService/latest/SQSDeveloperGuide/SQSMessageAttributes.html#SQSMessa
	/// geAttributes.DataTypes).
	pub data_type: String,
	/// Binary type attributes can store any binary data, for example, compressed
	/// data, encrypted data, or images.
	pub binary_value: Option<Binary>,
	/// Not implemented. Reserved for future use.
	pub string_list_values: Option<StringList>,
}

/// Parse MessageAttributeValue from XML
struct MessageAttributeValueParser;
impl MessageAttributeValueParser {
	fn parse_xml<'a, T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<MessageAttributeValue, XmlParseError> {
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
/// Write MessageAttributeValue contents to a SignedRequest
struct MessageAttributeValueWriter;
impl MessageAttributeValueWriter {
	fn write_params(params: &mut Params, name: &str, obj: &MessageAttributeValue) {
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
/// Parse MessageAttributeNameList from XML
struct MessageAttributeNameListParser;
impl MessageAttributeNameListParser {
	fn parse_xml<'a, T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<MessageAttributeNameList, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "MessageAttributeName" {
			obj.push(try!(MessageAttributeNameParser::parse_xml("MessageAttributeName", stack)));
		}
		Ok(obj)
	}
}
/// Write MessageAttributeNameList contents to a SignedRequest
struct MessageAttributeNameListWriter;
impl MessageAttributeNameListWriter {
	fn write_params(params: &mut Params, name: &str, obj: &MessageAttributeNameList) {
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
	/// The URL of the Amazon SQS queue to take action on.
	pub queue_url: String,
	/// A map of attributes to set.
	/// The following lists the names, descriptions, and values of the special request
	/// parameters the `SetQueueAttributes` action uses:
	///   * `DelaySeconds` \- The time in seconds that the delivery of all messages in the queue will be delayed. An integer from 0 to 900 (15 minutes). The default for this attribute is 0 (zero).
	///   * `MaximumMessageSize` \- The limit of how many bytes a message can contain before Amazon SQS rejects it. An integer from 1024 bytes (1 KiB) up to 262144 bytes (256 KiB). The default for this attribute is 262144 (256 KiB).
	///   * `MessageRetentionPeriod` \- The number of seconds Amazon SQS retains a message. Integer representing seconds, from 60 (1 minute) to 1209600 (14 days). The default for this attribute is 345600 (4 days).
	///   * `Policy` \- The queue's policy. A valid AWS policy. For more information about policy structure, see [Overview of AWS IAM Policies](http://docs.aws.amazon.com/IAM/latest/UserGuide/PoliciesOverview.html) in the _Amazon IAM User Guide_.
	///   * `ReceiveMessageWaitTimeSeconds` \- The time for which a ReceiveMessage call will wait for a message to arrive. An integer from 0 to 20 (seconds). The default for this attribute is 0. 
	///   * `VisibilityTimeout` \- The visibility timeout for the queue. An integer from 0 to 43200 (12 hours). The default for this attribute is 30. For more information about visibility timeout, see Visibility Timeout in the _Amazon SQS Developer Guide_.
	///   * `RedrivePolicy` \- The parameters for dead letter queue functionality of the source queue. For more information about RedrivePolicy and dead letter queues, see Using Amazon SQS Dead Letter Queues in the _Amazon SQS Developer Guide_.
	pub attributes: AttributeMap,
}

/// Parse SetQueueAttributesRequest from XML
struct SetQueueAttributesRequestParser;
impl SetQueueAttributesRequestParser {
	fn parse_xml<'a, T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<SetQueueAttributesRequest, XmlParseError> {
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
/// Write SetQueueAttributesRequest contents to a SignedRequest
struct SetQueueAttributesRequestWriter;
impl SetQueueAttributesRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &SetQueueAttributesRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		StringWriter::write_params(params, &(prefix.to_string() + "QueueUrl"), &obj.queue_url);
		AttributeMapWriter::write_params(params, &(prefix.to_string() + "Attribute"), &obj.attributes);
	}
}
#[derive(Debug, Default)]
pub struct GetQueueAttributesRequest {
	/// The URL of the Amazon SQS queue to take action on.
	pub queue_url: String,
	/// A list of attributes to retrieve information for.
	pub attribute_names: Option<AttributeNameList>,
}

/// Parse GetQueueAttributesRequest from XML
struct GetQueueAttributesRequestParser;
impl GetQueueAttributesRequestParser {
	fn parse_xml<'a, T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<GetQueueAttributesRequest, XmlParseError> {
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
/// Write GetQueueAttributesRequest contents to a SignedRequest
struct GetQueueAttributesRequestWriter;
impl GetQueueAttributesRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &GetQueueAttributesRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		StringWriter::write_params(params, &(prefix.to_string() + "QueueUrl"), &obj.queue_url);
		if let Some(ref obj) = obj.attribute_names {
			AttributeNameListWriter::write_params(params, &(prefix.to_string() + "AttributeName"), obj);
		}
	}
}
/// For each message in the batch, the response contains a
/// SendMessageBatchResultEntry tag if the message succeeds or a
/// BatchResultErrorEntry tag if the message fails.
#[derive(Debug, Default)]
pub struct SendMessageBatchResult {
	/// A list of SendMessageBatchResultEntry items.
	pub successful: SendMessageBatchResultEntryList,
	/// A list of BatchResultErrorEntry items with the error detail about each message
	/// that could not be enqueued.
	pub failed: BatchResultErrorEntryList,
}

/// Parse SendMessageBatchResult from XML
struct SendMessageBatchResultParser;
impl SendMessageBatchResultParser {
	fn parse_xml<'a, T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<SendMessageBatchResult, XmlParseError> {
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
/// Write SendMessageBatchResult contents to a SignedRequest
struct SendMessageBatchResultWriter;
impl SendMessageBatchResultWriter {
	fn write_params(params: &mut Params, name: &str, obj: &SendMessageBatchResult) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		SendMessageBatchResultEntryListWriter::write_params(params, &(prefix.to_string() + "SendMessageBatchResultEntry"), &obj.successful);
		BatchResultErrorEntryListWriter::write_params(params, &(prefix.to_string() + "BatchResultErrorEntry"), &obj.failed);
	}
}
#[derive(Debug, Default)]
pub struct CreateQueueRequest {
	/// A map of attributes with their corresponding values.
	/// The following lists the names, descriptions, and values of the special request
	/// parameters the `CreateQueue` action uses:
	///   * `DelaySeconds` \- The time in seconds that the delivery of all messages in the queue will be delayed. An integer from 0 to 900 (15 minutes). The default for this attribute is 0 (zero).
	///   * `MaximumMessageSize` \- The limit of how many bytes a message can contain before Amazon SQS rejects it. An integer from 1024 bytes (1 KiB) up to 262144 bytes (256 KiB). The default for this attribute is 262144 (256 KiB).
	///   * `MessageRetentionPeriod` \- The number of seconds Amazon SQS retains a message. Integer representing seconds, from 60 (1 minute) to 1209600 (14 days). The default for this attribute is 345600 (4 days).
	///   * `Policy` \- The queue's policy. A valid AWS policy. For more information about policy structure, see [Overview of AWS IAM Policies](http://docs.aws.amazon.com/IAM/latest/UserGuide/PoliciesOverview.html) in the _Amazon IAM User Guide_.
	///   * `ReceiveMessageWaitTimeSeconds` \- The time for which a ReceiveMessage call will wait for a message to arrive. An integer from 0 to 20 (seconds). The default for this attribute is 0. 
	///   * `VisibilityTimeout` \- The visibility timeout for the queue. An integer from 0 to 43200 (12 hours). The default for this attribute is 30. For more information about visibility timeout, see [Visibility Timeout](http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/AboutVT.html) in the _Amazon SQS Developer Guide_.
	pub attributes: Option<AttributeMap>,
	/// The name for the queue to be created.
	pub queue_name: String,
}

/// Parse CreateQueueRequest from XML
struct CreateQueueRequestParser;
impl CreateQueueRequestParser {
	fn parse_xml<'a, T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<CreateQueueRequest, XmlParseError> {
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
/// Write CreateQueueRequest contents to a SignedRequest
struct CreateQueueRequestWriter;
impl CreateQueueRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &CreateQueueRequest) {
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
	/// The URL of the Amazon SQS queue to take action on.
	pub queue_url: String,
	/// The identification of the permission to remove. This is the label added with
	/// the AddPermission action.
	pub label: String,
}

/// Parse RemovePermissionRequest from XML
struct RemovePermissionRequestParser;
impl RemovePermissionRequestParser {
	fn parse_xml<'a, T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<RemovePermissionRequest, XmlParseError> {
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
/// Write RemovePermissionRequest contents to a SignedRequest
struct RemovePermissionRequestWriter;
impl RemovePermissionRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &RemovePermissionRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		StringWriter::write_params(params, &(prefix.to_string() + "QueueUrl"), &obj.queue_url);
		StringWriter::write_params(params, &(prefix.to_string() + "Label"), &obj.label);
	}
}
/// Encloses a receipt handle and an identifier for it.
#[derive(Debug, Default)]
pub struct DeleteMessageBatchRequestEntry {
	/// A receipt handle.
	pub receipt_handle: String,
	/// An identifier for this particular receipt handle. This is used to communicate
	/// the result. Note that the `Id`s of a batch request need to be unique within
	/// the request.
	pub id: String,
}

/// Parse DeleteMessageBatchRequestEntry from XML
struct DeleteMessageBatchRequestEntryParser;
impl DeleteMessageBatchRequestEntryParser {
	fn parse_xml<'a, T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<DeleteMessageBatchRequestEntry, XmlParseError> {
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
/// Write DeleteMessageBatchRequestEntry contents to a SignedRequest
struct DeleteMessageBatchRequestEntryWriter;
impl DeleteMessageBatchRequestEntryWriter {
	fn write_params(params: &mut Params, name: &str, obj: &DeleteMessageBatchRequestEntry) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		StringWriter::write_params(params, &(prefix.to_string() + "ReceiptHandle"), &obj.receipt_handle);
		StringWriter::write_params(params, &(prefix.to_string() + "Id"), &obj.id);
	}
}
#[derive(Debug, Default)]
pub struct SendMessageBatchRequest {
	/// The URL of the Amazon SQS queue to take action on.
	pub queue_url: String,
	/// A list of SendMessageBatchRequestEntry items.
	pub entries: SendMessageBatchRequestEntryList,
}

/// Parse SendMessageBatchRequest from XML
struct SendMessageBatchRequestParser;
impl SendMessageBatchRequestParser {
	fn parse_xml<'a, T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<SendMessageBatchRequest, XmlParseError> {
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
/// Write SendMessageBatchRequest contents to a SignedRequest
struct SendMessageBatchRequestWriter;
impl SendMessageBatchRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &SendMessageBatchRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		StringWriter::write_params(params, &(prefix.to_string() + "QueueUrl"), &obj.queue_url);
		SendMessageBatchRequestEntryListWriter::write_params(params, &(prefix.to_string() + "SendMessageBatchRequestEntry"), &obj.entries);
	}
}
/// For each message in the batch, the response contains a
/// ChangeMessageVisibilityBatchResultEntry tag if the message succeeds or a
/// BatchResultErrorEntry tag if the message fails.
#[derive(Debug, Default)]
pub struct ChangeMessageVisibilityBatchResult {
	/// A list of ChangeMessageVisibilityBatchResultEntry items.
	pub successful: ChangeMessageVisibilityBatchResultEntryList,
	/// A list of BatchResultErrorEntry items.
	pub failed: BatchResultErrorEntryList,
}

/// Parse ChangeMessageVisibilityBatchResult from XML
struct ChangeMessageVisibilityBatchResultParser;
impl ChangeMessageVisibilityBatchResultParser {
	fn parse_xml<'a, T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ChangeMessageVisibilityBatchResult, XmlParseError> {
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
/// Write ChangeMessageVisibilityBatchResult contents to a SignedRequest
struct ChangeMessageVisibilityBatchResultWriter;
impl ChangeMessageVisibilityBatchResultWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ChangeMessageVisibilityBatchResult) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		ChangeMessageVisibilityBatchResultEntryListWriter::write_params(params, &(prefix.to_string() + "ChangeMessageVisibilityBatchResultEntry"), &obj.successful);
		BatchResultErrorEntryListWriter::write_params(params, &(prefix.to_string() + "BatchResultErrorEntry"), &obj.failed);
	}
}
/// Returns the QueueUrl element of the created queue.
#[derive(Debug, Default)]
pub struct CreateQueueResult {
	/// The URL for the created Amazon SQS queue.
	pub queue_url: String,
}

/// Parse CreateQueueResult from XML
struct CreateQueueResultParser;
impl CreateQueueResultParser {
	fn parse_xml<'a, T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<CreateQueueResult, XmlParseError> {
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
/// Write CreateQueueResult contents to a SignedRequest
struct CreateQueueResultWriter;
impl CreateQueueResultWriter {
	fn write_params(params: &mut Params, name: &str, obj: &CreateQueueResult) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		StringWriter::write_params(params, &(prefix.to_string() + "QueueUrl"), &obj.queue_url);
	}
}
#[derive(Debug, Default)]
pub struct PurgeQueueRequest {
	/// The queue URL of the queue to delete the messages from when using the
	/// `PurgeQueue` API.
	pub queue_url: String,
}

/// Parse PurgeQueueRequest from XML
struct PurgeQueueRequestParser;
impl PurgeQueueRequestParser {
	fn parse_xml<'a, T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<PurgeQueueRequest, XmlParseError> {
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
/// Write PurgeQueueRequest contents to a SignedRequest
struct PurgeQueueRequestWriter;
impl PurgeQueueRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &PurgeQueueRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		StringWriter::write_params(params, &(prefix.to_string() + "QueueUrl"), &obj.queue_url);
	}
}
/// The receipt handle provided is not valid.
#[derive(Debug, Default)]
pub struct ReceiptHandleIsInvalid;

/// Parse ReceiptHandleIsInvalid from XML
struct ReceiptHandleIsInvalidParser;
impl ReceiptHandleIsInvalidParser {
	fn parse_xml<'a, T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ReceiptHandleIsInvalid, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = ReceiptHandleIsInvalid::default();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ReceiptHandleIsInvalid contents to a SignedRequest
struct ReceiptHandleIsInvalidWriter;
impl ReceiptHandleIsInvalidWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ReceiptHandleIsInvalid) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
	}
}
/// The attribute referred to does not exist.
#[derive(Debug, Default)]
pub struct InvalidAttributeName;

/// Parse InvalidAttributeName from XML
struct InvalidAttributeNameParser;
impl InvalidAttributeNameParser {
	fn parse_xml<'a, T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<InvalidAttributeName, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = InvalidAttributeName::default();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write InvalidAttributeName contents to a SignedRequest
struct InvalidAttributeNameWriter;
impl InvalidAttributeNameWriter {
	fn write_params(params: &mut Params, name: &str, obj: &InvalidAttributeName) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
	}
}
#[derive(Debug, Default)]
pub struct ChangeMessageVisibilityBatchRequest {
	/// The URL of the Amazon SQS queue to take action on.
	pub queue_url: String,
	/// A list of receipt handles of the messages for which the visibility timeout
	/// must be changed.
	pub entries: ChangeMessageVisibilityBatchRequestEntryList,
}

/// Parse ChangeMessageVisibilityBatchRequest from XML
struct ChangeMessageVisibilityBatchRequestParser;
impl ChangeMessageVisibilityBatchRequestParser {
	fn parse_xml<'a, T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ChangeMessageVisibilityBatchRequest, XmlParseError> {
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
/// Write ChangeMessageVisibilityBatchRequest contents to a SignedRequest
struct ChangeMessageVisibilityBatchRequestWriter;
impl ChangeMessageVisibilityBatchRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ChangeMessageVisibilityBatchRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		StringWriter::write_params(params, &(prefix.to_string() + "QueueUrl"), &obj.queue_url);
		ChangeMessageVisibilityBatchRequestEntryListWriter::write_params(params, &(prefix.to_string() + "ChangeMessageVisibilityBatchRequestEntry"), &obj.entries);
	}
}
pub type MessageAttributeMap = HashMap<String,MessageAttributeValue>;
/// Parse MessageAttributeMap from XML
struct MessageAttributeMapParser;
impl MessageAttributeMapParser {
	fn parse_xml<'a, T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<MessageAttributeMap, XmlParseError> {
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
/// Write MessageAttributeMap contents to a SignedRequest
struct MessageAttributeMapWriter;
impl MessageAttributeMapWriter {
	fn write_params(params: &mut Params, name: &str, obj: &MessageAttributeMap) {
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
	/// The URL of the Amazon SQS queue to take action on.
	pub queue_url: String,
	/// The maximum number of messages to return. Amazon SQS never returns more
	/// messages than this value but may return fewer. Values can be from 1 to 10.
	/// Default is 1.
	/// All of the messages are not necessarily returned.
	pub max_number_of_messages: Option<Integer>,
	/// The duration (in seconds) for which the call will wait for a message to arrive
	/// in the queue before returning. If a message is available, the call will return
	/// sooner than WaitTimeSeconds.
	pub wait_time_seconds: Option<Integer>,
	/// The name of the message attribute, where _N_ is the index. The message
	/// attribute name can contain the following characters: A-Z, a-z, 0-9, underscore
	/// (_), hyphen (-), and period (.). The name must not start or end with a period,
	/// and it should not have successive periods. The name is case sensitive and must
	/// be unique among all attribute names for the message. The name can be up to 256
	/// characters long. The name cannot start with "AWS." or "Amazon." (or any
	/// variations in casing), because these prefixes are reserved for use by Amazon
	/// Web Services.
	/// When using `ReceiveMessage`, you can send a list of attribute names to
	/// receive, or you can return all of the attributes by specifying "All" or ".*"
	/// in your request. You can also use "foo.*" to return all message attributes
	/// starting with the "foo" prefix.
	pub message_attribute_names: Option<MessageAttributeNameList>,
	/// The duration (in seconds) that the received messages are hidden from
	/// subsequent retrieve requests after being retrieved by a `ReceiveMessage`
	/// request.
	pub visibility_timeout: Option<Integer>,
	/// A list of attributes that need to be returned along with each message.
	/// The following lists the names and descriptions of the attributes that can be
	/// returned:
	///   * `All` \- returns all values.
	///   * `ApproximateFirstReceiveTimestamp` \- returns the time when the message was first received from the queue (epoch time in milliseconds).
	///   * `ApproximateReceiveCount` \- returns the number of times a message has been received from the queue but not deleted.
	///   * `SenderId` \- returns the AWS account number (or the IP address, if anonymous access is allowed) of the sender.
	///   * `SentTimestamp` \- returns the time when the message was sent to the queue (epoch time in milliseconds).
	pub attribute_names: Option<AttributeNameList>,
}

/// Parse ReceiveMessageRequest from XML
struct ReceiveMessageRequestParser;
impl ReceiveMessageRequestParser {
	fn parse_xml<'a, T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ReceiveMessageRequest, XmlParseError> {
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
/// Write ReceiveMessageRequest contents to a SignedRequest
struct ReceiveMessageRequestWriter;
impl ReceiveMessageRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ReceiveMessageRequest) {
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
/// A list of received messages.
#[derive(Debug, Default)]
pub struct ReceiveMessageResult {
	/// A list of messages.
	pub messages: MessageList,
}

/// Parse ReceiveMessageResult from XML
struct ReceiveMessageResultParser;
impl ReceiveMessageResultParser {
	fn parse_xml<'a, T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ReceiveMessageResult, XmlParseError> {
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
/// Write ReceiveMessageResult contents to a SignedRequest
struct ReceiveMessageResultWriter;
impl ReceiveMessageResultWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ReceiveMessageResult) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		MessageListWriter::write_params(params, &(prefix.to_string() + "Message"), &obj.messages);
	}
}
/// Parse String from XML
struct StringParser;
impl StringParser {
	fn parse_xml<'a, T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write String contents to a SignedRequest
struct StringWriter;
impl StringWriter {
	fn write_params(params: &mut Params, name: &str, obj: &String) {
		params.put(name, obj);
	}
}
pub type ChangeMessageVisibilityBatchRequestEntryList = Vec<ChangeMessageVisibilityBatchRequestEntry>;
/// Parse ChangeMessageVisibilityBatchRequestEntryList from XML
struct ChangeMessageVisibilityBatchRequestEntryListParser;
impl ChangeMessageVisibilityBatchRequestEntryListParser {
	fn parse_xml<'a, T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ChangeMessageVisibilityBatchRequestEntryList, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "ChangeMessageVisibilityBatchRequestEntry" {
			obj.push(try!(ChangeMessageVisibilityBatchRequestEntryParser::parse_xml("ChangeMessageVisibilityBatchRequestEntry", stack)));
		}
		Ok(obj)
	}
}
/// Write ChangeMessageVisibilityBatchRequestEntryList contents to a SignedRequest
struct ChangeMessageVisibilityBatchRequestEntryListWriter;
impl ChangeMessageVisibilityBatchRequestEntryListWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ChangeMessageVisibilityBatchRequestEntryList) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			ChangeMessageVisibilityBatchRequestEntryWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
pub type SendMessageBatchResultEntryList = Vec<SendMessageBatchResultEntry>;
/// Parse SendMessageBatchResultEntryList from XML
struct SendMessageBatchResultEntryListParser;
impl SendMessageBatchResultEntryListParser {
	fn parse_xml<'a, T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<SendMessageBatchResultEntryList, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "SendMessageBatchResultEntry" {
			obj.push(try!(SendMessageBatchResultEntryParser::parse_xml("SendMessageBatchResultEntry", stack)));
		}
		Ok(obj)
	}
}
/// Write SendMessageBatchResultEntryList contents to a SignedRequest
struct SendMessageBatchResultEntryListWriter;
impl SendMessageBatchResultEntryListWriter {
	fn write_params(params: &mut Params, name: &str, obj: &SendMessageBatchResultEntryList) {
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
	/// The URL of the Amazon SQS queue to take action on.
	pub queue_url: String,
	/// The number of seconds (0 to 900 - 15 minutes) to delay a specific message.
	/// Messages with a positive `DelaySeconds` value become available for processing
	/// after the delay time is finished. If you don't specify a value, the default
	/// value for the queue applies.
	pub delay_seconds: Option<Integer>,
	/// The message to send. String maximum 256 KB in size. For a list of allowed
	/// characters, see the preceding important note.
	pub message_body: String,
	/// Each message attribute consists of a Name, Type, and Value. For more
	/// information, see [Message Attribute Items](http://docs.aws.amazon.com/AWSSimpl
	/// eQueueService/latest/SQSDeveloperGuide/SQSMessageAttributes.html#SQSMessageAtt
	/// ributesNTV).
	pub message_attributes: Option<MessageAttributeMap>,
}

/// Parse SendMessageRequest from XML
struct SendMessageRequestParser;
impl SendMessageRequestParser {
	fn parse_xml<'a, T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<SendMessageRequest, XmlParseError> {
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
/// Write SendMessageRequest contents to a SignedRequest
struct SendMessageRequestWriter;
impl SendMessageRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &SendMessageRequest) {
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
/// Encloses a message ID for successfully enqueued message of a SendMessageBatch.
#[derive(Debug, Default)]
pub struct SendMessageBatchResultEntry {
	/// An MD5 digest of the non-URL-encoded message body string. This can be used to
	/// verify that Amazon SQS received the message correctly. Amazon SQS first URL
	/// decodes the message before creating the MD5 digest. For information about MD5,
	/// go to <http://www.faqs.org/rfcs/rfc1321.html>.
	pub md5_of_message_body: String,
	/// An MD5 digest of the non-URL-encoded message attribute string. This can be
	/// used to verify that Amazon SQS received the message batch correctly. Amazon
	/// SQS first URL decodes the message before creating the MD5 digest. For
	/// information about MD5, go to <http://www.faqs.org/rfcs/rfc1321.html>.
	pub md5_of_message_attributes: Option<String>,
	/// An identifier for the message in this batch.
	pub id: String,
	/// An identifier for the message.
	pub message_id: String,
}

/// Parse SendMessageBatchResultEntry from XML
struct SendMessageBatchResultEntryParser;
impl SendMessageBatchResultEntryParser {
	fn parse_xml<'a, T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<SendMessageBatchResultEntry, XmlParseError> {
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
/// Write SendMessageBatchResultEntry contents to a SignedRequest
struct SendMessageBatchResultEntryWriter;
impl SendMessageBatchResultEntryWriter {
	fn write_params(params: &mut Params, name: &str, obj: &SendMessageBatchResultEntry) {
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
/// Parse DeleteMessageBatchResultEntryList from XML
struct DeleteMessageBatchResultEntryListParser;
impl DeleteMessageBatchResultEntryListParser {
	fn parse_xml<'a, T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<DeleteMessageBatchResultEntryList, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "DeleteMessageBatchResultEntry" {
			obj.push(try!(DeleteMessageBatchResultEntryParser::parse_xml("DeleteMessageBatchResultEntry", stack)));
		}
		Ok(obj)
	}
}
/// Write DeleteMessageBatchResultEntryList contents to a SignedRequest
struct DeleteMessageBatchResultEntryListWriter;
impl DeleteMessageBatchResultEntryListWriter {
	fn write_params(params: &mut Params, name: &str, obj: &DeleteMessageBatchResultEntryList) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			DeleteMessageBatchResultEntryWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
pub type StringList = Vec<String>;
/// Parse StringList from XML
struct StringListParser;
impl StringListParser {
	fn parse_xml<'a, T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<StringList, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "StringListValue" {
			obj.push(try!(StringParser::parse_xml("StringListValue", stack)));
		}
		Ok(obj)
	}
}
/// Write StringList contents to a SignedRequest
struct StringListWriter;
impl StringListWriter {
	fn write_params(params: &mut Params, name: &str, obj: &StringList) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			StringWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
pub type BatchResultErrorEntryList = Vec<BatchResultErrorEntry>;
/// Parse BatchResultErrorEntryList from XML
struct BatchResultErrorEntryListParser;
impl BatchResultErrorEntryListParser {
	fn parse_xml<'a, T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<BatchResultErrorEntryList, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "BatchResultErrorEntry" {
			obj.push(try!(BatchResultErrorEntryParser::parse_xml("BatchResultErrorEntry", stack)));
		}
		Ok(obj)
	}
}
/// Write BatchResultErrorEntryList contents to a SignedRequest
struct BatchResultErrorEntryListWriter;
impl BatchResultErrorEntryListWriter {
	fn write_params(params: &mut Params, name: &str, obj: &BatchResultErrorEntryList) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			BatchResultErrorEntryWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
/// The `Id` of a batch entry in a batch request does not abide by the
/// specification.
#[derive(Debug, Default)]
pub struct InvalidBatchEntryId;

/// Parse InvalidBatchEntryId from XML
struct InvalidBatchEntryIdParser;
impl InvalidBatchEntryIdParser {
	fn parse_xml<'a, T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<InvalidBatchEntryId, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = InvalidBatchEntryId::default();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write InvalidBatchEntryId contents to a SignedRequest
struct InvalidBatchEntryIdWriter;
impl InvalidBatchEntryIdWriter {
	fn write_params(params: &mut Params, name: &str, obj: &InvalidBatchEntryId) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
	}
}
#[derive(Debug, Default)]
pub struct ChangeMessageVisibilityRequest {
	/// The URL of the Amazon SQS queue to take action on.
	pub queue_url: String,
	/// The receipt handle associated with the message whose visibility timeout should
	/// be changed. This parameter is returned by the ReceiveMessage action.
	pub receipt_handle: String,
	/// The new value (in seconds - from 0 to 43200 - maximum 12 hours) for the
	/// message's visibility timeout.
	pub visibility_timeout: Integer,
}

/// Parse ChangeMessageVisibilityRequest from XML
struct ChangeMessageVisibilityRequestParser;
impl ChangeMessageVisibilityRequestParser {
	fn parse_xml<'a, T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ChangeMessageVisibilityRequest, XmlParseError> {
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
/// Write ChangeMessageVisibilityRequest contents to a SignedRequest
struct ChangeMessageVisibilityRequestWriter;
impl ChangeMessageVisibilityRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ChangeMessageVisibilityRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		StringWriter::write_params(params, &(prefix.to_string() + "QueueUrl"), &obj.queue_url);
		StringWriter::write_params(params, &(prefix.to_string() + "ReceiptHandle"), &obj.receipt_handle);
		IntegerWriter::write_params(params, &(prefix.to_string() + "VisibilityTimeout"), &obj.visibility_timeout);
	}
}
pub type ChangeMessageVisibilityBatchResultEntryList = Vec<ChangeMessageVisibilityBatchResultEntry>;
/// Parse ChangeMessageVisibilityBatchResultEntryList from XML
struct ChangeMessageVisibilityBatchResultEntryListParser;
impl ChangeMessageVisibilityBatchResultEntryListParser {
	fn parse_xml<'a, T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ChangeMessageVisibilityBatchResultEntryList, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "ChangeMessageVisibilityBatchResultEntry" {
			obj.push(try!(ChangeMessageVisibilityBatchResultEntryParser::parse_xml("ChangeMessageVisibilityBatchResultEntry", stack)));
		}
		Ok(obj)
	}
}
/// Write ChangeMessageVisibilityBatchResultEntryList contents to a SignedRequest
struct ChangeMessageVisibilityBatchResultEntryListWriter;
impl ChangeMessageVisibilityBatchResultEntryListWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ChangeMessageVisibilityBatchResultEntryList) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			ChangeMessageVisibilityBatchResultEntryWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
/// This is used in the responses of batch API to give a detailed description of
/// the result of an action on each entry in the request.
#[derive(Debug, Default)]
pub struct BatchResultErrorEntry {
	/// A message explaining why the action failed on this entry.
	pub message: Option<String>,
	/// Whether the error happened due to the sender's fault.
	pub sender_fault: Boolean,
	/// An error code representing why the action failed on this entry.
	pub code: String,
	/// The id of an entry in a batch request.
	pub id: String,
}

/// Parse BatchResultErrorEntry from XML
struct BatchResultErrorEntryParser;
impl BatchResultErrorEntryParser {
	fn parse_xml<'a, T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<BatchResultErrorEntry, XmlParseError> {
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
/// Write BatchResultErrorEntry contents to a SignedRequest
struct BatchResultErrorEntryWriter;
impl BatchResultErrorEntryWriter {
	fn write_params(params: &mut Params, name: &str, obj: &BatchResultErrorEntry) {
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
/// The MD5OfMessageBody and MessageId elements.
#[derive(Debug, Default)]
pub struct SendMessageResult {
	/// An MD5 digest of the non-URL-encoded message body string. This can be used to
	/// verify that Amazon SQS received the message correctly. Amazon SQS first URL
	/// decodes the message before creating the MD5 digest. For information about MD5,
	/// go to <http://www.faqs.org/rfcs/rfc1321.html>.
	pub md5_of_message_body: String,
	/// An MD5 digest of the non-URL-encoded message attribute string. This can be
	/// used to verify that Amazon SQS received the message correctly. Amazon SQS
	/// first URL decodes the message before creating the MD5 digest. For information
	/// about MD5, go to <http://www.faqs.org/rfcs/rfc1321.html>.
	pub md5_of_message_attributes: String,
	/// An element containing the message ID of the message sent to the queue. For
	/// more information, see [Queue and Message Identifiers](http://docs.aws.amazon.c
	/// om/AWSSimpleQueueService/latest/SQSDeveloperGuide/ImportantIdentifiers.html)
	/// in the _Amazon SQS Developer Guide_.
	pub message_id: String,
}

/// Parse SendMessageResult from XML
struct SendMessageResultParser;
impl SendMessageResultParser {
	fn parse_xml<'a, T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<SendMessageResult, XmlParseError> {
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
/// Write SendMessageResult contents to a SignedRequest
struct SendMessageResultWriter;
impl SendMessageResultWriter {
	fn write_params(params: &mut Params, name: &str, obj: &SendMessageResult) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		StringWriter::write_params(params, &(prefix.to_string() + "MD5OfMessageBody"), &obj.md5_of_message_body);
		StringWriter::write_params(params, &(prefix.to_string() + "MD5OfMessageAttributes"), &obj.md5_of_message_attributes);
		StringWriter::write_params(params, &(prefix.to_string() + "MessageId"), &obj.message_id);
	}
}
#[derive(Debug, Default)]
pub struct DeleteMessageBatchRequest {
	/// The URL of the Amazon SQS queue to take action on.
	pub queue_url: String,
	/// A list of receipt handles for the messages to be deleted.
	pub entries: DeleteMessageBatchRequestEntryList,
}

/// Parse DeleteMessageBatchRequest from XML
struct DeleteMessageBatchRequestParser;
impl DeleteMessageBatchRequestParser {
	fn parse_xml<'a, T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<DeleteMessageBatchRequest, XmlParseError> {
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
/// Write DeleteMessageBatchRequest contents to a SignedRequest
struct DeleteMessageBatchRequestWriter;
impl DeleteMessageBatchRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &DeleteMessageBatchRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		StringWriter::write_params(params, &(prefix.to_string() + "QueueUrl"), &obj.queue_url);
		DeleteMessageBatchRequestEntryListWriter::write_params(params, &(prefix.to_string() + "DeleteMessageBatchRequestEntry"), &obj.entries);
	}
}
pub type Integer = i32;
/// Parse Integer from XML
struct IntegerParser;
impl IntegerParser {
	fn parse_xml<'a, T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Integer, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = i32::from_str(try!(characters(stack)).as_ref()).unwrap();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write Integer contents to a SignedRequest
struct IntegerWriter;
impl IntegerWriter {
	fn write_params(params: &mut Params, name: &str, obj: &Integer) {
		params.put(name, &obj.to_string());
	}
}
/// The queue referred to does not exist.
#[derive(Debug, Default)]
pub struct QueueDoesNotExist;

/// Parse QueueDoesNotExist from XML
struct QueueDoesNotExistParser;
impl QueueDoesNotExistParser {
	fn parse_xml<'a, T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<QueueDoesNotExist, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = QueueDoesNotExist::default();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write QueueDoesNotExist contents to a SignedRequest
struct QueueDoesNotExistWriter;
impl QueueDoesNotExistWriter {
	fn write_params(params: &mut Params, name: &str, obj: &QueueDoesNotExist) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
	}
}
/// The message contains characters outside the allowed set.
#[derive(Debug, Default)]
pub struct InvalidMessageContents;

/// Parse InvalidMessageContents from XML
struct InvalidMessageContentsParser;
impl InvalidMessageContentsParser {
	fn parse_xml<'a, T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<InvalidMessageContents, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = InvalidMessageContents::default();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write InvalidMessageContents contents to a SignedRequest
struct InvalidMessageContentsWriter;
impl InvalidMessageContentsWriter {
	fn write_params(params: &mut Params, name: &str, obj: &InvalidMessageContents) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
	}
}
/// The message referred to is not in flight.
#[derive(Debug, Default)]
pub struct MessageNotInflight;

/// Parse MessageNotInflight from XML
struct MessageNotInflightParser;
impl MessageNotInflightParser {
	fn parse_xml<'a, T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<MessageNotInflight, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = MessageNotInflight::default();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write MessageNotInflight contents to a SignedRequest
struct MessageNotInflightWriter;
impl MessageNotInflightWriter {
	fn write_params(params: &mut Params, name: &str, obj: &MessageNotInflight) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
	}
}
pub type MessageAttributeName = String;
/// Parse MessageAttributeName from XML
struct MessageAttributeNameParser;
impl MessageAttributeNameParser {
	fn parse_xml<'a, T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<MessageAttributeName, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write MessageAttributeName contents to a SignedRequest
struct MessageAttributeNameWriter;
impl MessageAttributeNameWriter {
	fn write_params(params: &mut Params, name: &str, obj: &MessageAttributeName) {
		params.put(name, obj);
	}
}
/// A list of returned queue attributes.
#[derive(Debug, Default)]
pub struct GetQueueAttributesResult {
	/// A map of attributes to the respective values.
	pub attributes: AttributeMap,
}

/// Parse GetQueueAttributesResult from XML
struct GetQueueAttributesResultParser;
impl GetQueueAttributesResultParser {
	fn parse_xml<'a, T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<GetQueueAttributesResult, XmlParseError> {
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
/// Write GetQueueAttributesResult contents to a SignedRequest
struct GetQueueAttributesResultWriter;
impl GetQueueAttributesResultWriter {
	fn write_params(params: &mut Params, name: &str, obj: &GetQueueAttributesResult) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		AttributeMapWriter::write_params(params, &(prefix.to_string() + "Attribute"), &obj.attributes);
	}
}
pub type BinaryList = Vec<Binary>;
/// Parse BinaryList from XML
struct BinaryListParser;
impl BinaryListParser {
	fn parse_xml<'a, T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<BinaryList, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "BinaryListValue" {
			obj.push(try!(BinaryParser::parse_xml("BinaryListValue", stack)));
		}
		Ok(obj)
	}
}
/// Write BinaryList contents to a SignedRequest
struct BinaryListWriter;
impl BinaryListWriter {
	fn write_params(params: &mut Params, name: &str, obj: &BinaryList) {
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
	/// The queue URL of a dead letter queue.
	pub queue_url: String,
}

/// Parse ListDeadLetterSourceQueuesRequest from XML
struct ListDeadLetterSourceQueuesRequestParser;
impl ListDeadLetterSourceQueuesRequestParser {
	fn parse_xml<'a, T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ListDeadLetterSourceQueuesRequest, XmlParseError> {
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
/// Write ListDeadLetterSourceQueuesRequest contents to a SignedRequest
struct ListDeadLetterSourceQueuesRequestWriter;
impl ListDeadLetterSourceQueuesRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ListDeadLetterSourceQueuesRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		StringWriter::write_params(params, &(prefix.to_string() + "QueueUrl"), &obj.queue_url);
	}
}
pub type MessageList = Vec<Message>;
/// Parse MessageList from XML
struct MessageListParser;
impl MessageListParser {
	fn parse_xml<'a, T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<MessageList, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "Message" {
			obj.push(try!(MessageParser::parse_xml("Message", stack)));
		}
		Ok(obj)
	}
}
/// Write MessageList contents to a SignedRequest
struct MessageListWriter;
impl MessageListWriter {
	fn write_params(params: &mut Params, name: &str, obj: &MessageList) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			MessageWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
pub type SendMessageBatchRequestEntryList = Vec<SendMessageBatchRequestEntry>;
/// Parse SendMessageBatchRequestEntryList from XML
struct SendMessageBatchRequestEntryListParser;
impl SendMessageBatchRequestEntryListParser {
	fn parse_xml<'a, T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<SendMessageBatchRequestEntryList, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "SendMessageBatchRequestEntry" {
			obj.push(try!(SendMessageBatchRequestEntryParser::parse_xml("SendMessageBatchRequestEntry", stack)));
		}
		Ok(obj)
	}
}
/// Write SendMessageBatchRequestEntryList contents to a SignedRequest
struct SendMessageBatchRequestEntryListWriter;
impl SendMessageBatchRequestEntryListWriter {
	fn write_params(params: &mut Params, name: &str, obj: &SendMessageBatchRequestEntryList) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			SendMessageBatchRequestEntryWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
/// An Amazon SQS message.
#[derive(Debug, Default)]
pub struct Message {
	/// The message's contents (not URL-encoded).
	pub body: String,
	/// An identifier associated with the act of receiving the message. A new receipt
	/// handle is returned every time you receive a message. When deleting a message,
	/// you provide the last received receipt handle to delete the message.
	pub receipt_handle: String,
	/// An MD5 digest of the non-URL-encoded message body string.
	pub md5_of_body: String,
	/// An MD5 digest of the non-URL-encoded message attribute string. This can be
	/// used to verify that Amazon SQS received the message correctly. Amazon SQS
	/// first URL decodes the message before creating the MD5 digest. For information
	/// about MD5, go to <http://www.faqs.org/rfcs/rfc1321.html>.
	pub md5_of_message_attributes: String,
	/// A unique identifier for the message. Message IDs are considered unique across
	/// all AWS accounts for an extended period of time.
	pub message_id: String,
	/// `SenderId`, `SentTimestamp`, `ApproximateReceiveCount`, and/or
	/// `ApproximateFirstReceiveTimestamp`. `SentTimestamp` and
	/// `ApproximateFirstReceiveTimestamp` are each returned as an integer
	/// representing the [epoch time](http://en.wikipedia.org/wiki/Unix_time) in
	/// milliseconds.
	pub attributes: AttributeMap,
	/// Each message attribute consists of a Name, Type, and Value. For more
	/// information, see [Message Attribute Items](http://docs.aws.amazon.com/AWSSimpl
	/// eQueueService/latest/SQSDeveloperGuide/SQSMessageAttributes.html#SQSMessageAtt
	/// ributesNTV).
	pub message_attributes: MessageAttributeMap,
}

/// Parse Message from XML
struct MessageParser;
impl MessageParser {
	fn parse_xml<'a, T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Message, XmlParseError> {
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
/// Write Message contents to a SignedRequest
struct MessageWriter;
impl MessageWriter {
	fn write_params(params: &mut Params, name: &str, obj: &Message) {
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
/// The action that you requested would violate a limit. For example,
/// ReceiveMessage returns this error if the maximum number of messages inflight
/// has already been reached. AddPermission returns this error if the maximum
/// number of permissions for the queue has already been reached.
#[derive(Debug, Default)]
pub struct OverLimit;

/// Parse OverLimit from XML
struct OverLimitParser;
impl OverLimitParser {
	fn parse_xml<'a, T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<OverLimit, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = OverLimit::default();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write OverLimit contents to a SignedRequest
struct OverLimitWriter;
impl OverLimitWriter {
	fn write_params(params: &mut Params, name: &str, obj: &OverLimit) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
	}
}
#[derive(Debug, Default)]
pub struct GetQueueUrlRequest {
	/// The name of the queue whose URL must be fetched. Maximum 80 characters;
	/// alphanumeric characters, hyphens (-), and underscores (_) are allowed.
	pub queue_name: String,
	/// The AWS account ID of the account that created the queue.
	pub queue_owner_aws_account_id: Option<String>,
}

/// Parse GetQueueUrlRequest from XML
struct GetQueueUrlRequestParser;
impl GetQueueUrlRequestParser {
	fn parse_xml<'a, T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<GetQueueUrlRequest, XmlParseError> {
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
/// Write GetQueueUrlRequest contents to a SignedRequest
struct GetQueueUrlRequestWriter;
impl GetQueueUrlRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &GetQueueUrlRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		StringWriter::write_params(params, &(prefix.to_string() + "QueueName"), &obj.queue_name);
		if let Some(ref obj) = obj.queue_owner_aws_account_id {
			StringWriter::write_params(params, &(prefix.to_string() + "QueueOwnerAWSAccountId"), obj);
		}
	}
}
pub type AttributeMap = HashMap<QueueAttributeName,String>;
/// Parse AttributeMap from XML
struct AttributeMapParser;
impl AttributeMapParser {
	fn parse_xml<'a, T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<AttributeMap, XmlParseError> {
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
/// Write AttributeMap contents to a SignedRequest
struct AttributeMapWriter;
impl AttributeMapWriter {
	fn write_params(params: &mut Params, name: &str, obj: &AttributeMap) {
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
	/// The URL of the Amazon SQS queue to take action on.
	pub queue_url: String,
}

/// Parse DeleteQueueRequest from XML
struct DeleteQueueRequestParser;
impl DeleteQueueRequestParser {
	fn parse_xml<'a, T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<DeleteQueueRequest, XmlParseError> {
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
/// Write DeleteQueueRequest contents to a SignedRequest
struct DeleteQueueRequestWriter;
impl DeleteQueueRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &DeleteQueueRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		StringWriter::write_params(params, &(prefix.to_string() + "QueueUrl"), &obj.queue_url);
	}
}
pub type ActionNameList = Vec<String>;
/// Parse ActionNameList from XML
struct ActionNameListParser;
impl ActionNameListParser {
	fn parse_xml<'a, T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ActionNameList, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "ActionName" {
			obj.push(try!(StringParser::parse_xml("ActionName", stack)));
		}
		Ok(obj)
	}
}
/// Write ActionNameList contents to a SignedRequest
struct ActionNameListWriter;
impl ActionNameListWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ActionNameList) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			StringWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
/// The length of all the messages put together is more than the limit.
#[derive(Debug, Default)]
pub struct BatchRequestTooLong;

/// Parse BatchRequestTooLong from XML
struct BatchRequestTooLongParser;
impl BatchRequestTooLongParser {
	fn parse_xml<'a, T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<BatchRequestTooLong, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = BatchRequestTooLong::default();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write BatchRequestTooLong contents to a SignedRequest
struct BatchRequestTooLongWriter;
impl BatchRequestTooLongWriter {
	fn write_params(params: &mut Params, name: &str, obj: &BatchRequestTooLong) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
	}
}
/// For more information, see [Responses](http://docs.aws.amazon.com/AWSSimpleQueu
/// eService/latest/SQSDeveloperGuide/UnderstandingResponses.html) in the _Amazon
/// SQS Developer Guide_.
#[derive(Debug, Default)]
pub struct GetQueueUrlResult {
	/// The URL for the queue.
	pub queue_url: String,
}

/// Parse GetQueueUrlResult from XML
struct GetQueueUrlResultParser;
impl GetQueueUrlResultParser {
	fn parse_xml<'a, T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<GetQueueUrlResult, XmlParseError> {
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
/// Write GetQueueUrlResult contents to a SignedRequest
struct GetQueueUrlResultWriter;
impl GetQueueUrlResultWriter {
	fn write_params(params: &mut Params, name: &str, obj: &GetQueueUrlResult) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		StringWriter::write_params(params, &(prefix.to_string() + "QueueUrl"), &obj.queue_url);
	}
}
#[derive(Debug, Default)]
pub struct ListQueuesRequest {
	/// A string to use for filtering the list results. Only those queues whose name
	/// begins with the specified string are returned.
	pub queue_name_prefix: String,
}

/// Parse ListQueuesRequest from XML
struct ListQueuesRequestParser;
impl ListQueuesRequestParser {
	fn parse_xml<'a, T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ListQueuesRequest, XmlParseError> {
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
/// Write ListQueuesRequest contents to a SignedRequest
struct ListQueuesRequestWriter;
impl ListQueuesRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ListQueuesRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		StringWriter::write_params(params, &(prefix.to_string() + "QueueNamePrefix"), &obj.queue_name_prefix);
	}
}
pub type AWSAccountIdList = Vec<String>;
/// Parse AWSAccountIdList from XML
struct AWSAccountIdListParser;
impl AWSAccountIdListParser {
	fn parse_xml<'a, T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<AWSAccountIdList, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "AWSAccountId" {
			obj.push(try!(StringParser::parse_xml("AWSAccountId", stack)));
		}
		Ok(obj)
	}
}
/// Write AWSAccountIdList contents to a SignedRequest
struct AWSAccountIdListWriter;
impl AWSAccountIdListWriter {
	fn write_params(params: &mut Params, name: &str, obj: &AWSAccountIdList) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			StringWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
/// The receipt handle is not valid for the current version.
#[derive(Debug, Default)]
pub struct InvalidIdFormat;

/// Parse InvalidIdFormat from XML
struct InvalidIdFormatParser;
impl InvalidIdFormatParser {
	fn parse_xml<'a, T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<InvalidIdFormat, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = InvalidIdFormat::default();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write InvalidIdFormat contents to a SignedRequest
struct InvalidIdFormatWriter;
impl InvalidIdFormatWriter {
	fn write_params(params: &mut Params, name: &str, obj: &InvalidIdFormat) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
	}
}
#[derive(Debug, Default)]
pub struct AddPermissionRequest {
	/// The URL of the Amazon SQS queue to take action on.
	pub queue_url: String,
	/// The AWS account number of the
	/// [principal](http://docs.aws.amazon.com/general/latest/gr/glos-chap.html#P) who
	/// will be given permission. The principal must have an AWS account, but does not
	/// need to be signed up for Amazon SQS. For information about locating the AWS
	/// account identification, see [Your AWS Identifiers](http://docs.aws.amazon.com/
	/// AWSSimpleQueueService/latest/SQSDeveloperGuide/AWSCredentials.html) in the
	/// _Amazon SQS Developer Guide_.
	pub aws_account_ids: AWSAccountIdList,
	/// The action the client wants to allow for the specified principal. The
	/// following are valid values: `* | SendMessage | ReceiveMessage | DeleteMessage
	/// | ChangeMessageVisibility | GetQueueAttributes | GetQueueUrl`. For more
	/// information about these actions, see [Understanding Permissions](http://docs.a
	/// ws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/acp-
	/// overview.html#PermissionTypes) in the _Amazon SQS Developer Guide_.
	/// Specifying `SendMessage`, `DeleteMessage`, or `ChangeMessageVisibility` for
	/// the `ActionName.n` also grants permissions for the corresponding batch
	/// versions of those actions: `SendMessageBatch`, `DeleteMessageBatch`, and
	/// `ChangeMessageVisibilityBatch`.
	pub actions: ActionNameList,
	/// The unique identification of the permission you're setting (e.g.,
	/// `AliceSendMessage`). Constraints: Maximum 80 characters; alphanumeric
	/// characters, hyphens (-), and underscores (_) are allowed.
	pub label: String,
}

/// Parse AddPermissionRequest from XML
struct AddPermissionRequestParser;
impl AddPermissionRequestParser {
	fn parse_xml<'a, T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<AddPermissionRequest, XmlParseError> {
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
/// Write AddPermissionRequest contents to a SignedRequest
struct AddPermissionRequestWriter;
impl AddPermissionRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &AddPermissionRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		StringWriter::write_params(params, &(prefix.to_string() + "QueueUrl"), &obj.queue_url);
		AWSAccountIdListWriter::write_params(params, &(prefix.to_string() + "AWSAccountId"), &obj.aws_account_ids);
		ActionNameListWriter::write_params(params, &(prefix.to_string() + "ActionName"), &obj.actions);
		StringWriter::write_params(params, &(prefix.to_string() + "Label"), &obj.label);
	}
}
pub type QueueUrlList = Vec<String>;
/// Parse QueueUrlList from XML
struct QueueUrlListParser;
impl QueueUrlListParser {
	fn parse_xml<'a, T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<QueueUrlList, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "QueueUrl" {
			obj.push(try!(StringParser::parse_xml("QueueUrl", stack)));
		}
		Ok(obj)
	}
}
/// Write QueueUrlList contents to a SignedRequest
struct QueueUrlListWriter;
impl QueueUrlListWriter {
	fn write_params(params: &mut Params, name: &str, obj: &QueueUrlList) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			StringWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
/// A list of your dead letter source queues.
#[derive(Debug, Default)]
pub struct ListDeadLetterSourceQueuesResult {
	/// A list of source queue URLs that have the RedrivePolicy queue attribute
	/// configured with a dead letter queue.
	pub queue_urls: QueueUrlList,
}

/// Parse ListDeadLetterSourceQueuesResult from XML
struct ListDeadLetterSourceQueuesResultParser;
impl ListDeadLetterSourceQueuesResultParser {
	fn parse_xml<'a, T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ListDeadLetterSourceQueuesResult, XmlParseError> {
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
/// Write ListDeadLetterSourceQueuesResult contents to a SignedRequest
struct ListDeadLetterSourceQueuesResultWriter;
impl ListDeadLetterSourceQueuesResultWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ListDeadLetterSourceQueuesResult) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		QueueUrlListWriter::write_params(params, &(prefix.to_string() + "QueueUrl"), &obj.queue_urls);
	}
}
pub type QueueAttributeName = String;
/// Parse QueueAttributeName from XML
struct QueueAttributeNameParser;
impl QueueAttributeNameParser {
	fn parse_xml<'a, T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<QueueAttributeName, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write QueueAttributeName contents to a SignedRequest
struct QueueAttributeNameWriter;
impl QueueAttributeNameWriter {
	fn write_params(params: &mut Params, name: &str, obj: &QueueAttributeName) {
		params.put(name, obj);
	}
}
/// A queue already exists with this name. Amazon SQS returns this error only if
/// the request includes attributes whose values differ from those of the existing
/// queue.
#[derive(Debug, Default)]
pub struct QueueNameExists;

/// Parse QueueNameExists from XML
struct QueueNameExistsParser;
impl QueueNameExistsParser {
	fn parse_xml<'a, T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<QueueNameExists, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = QueueNameExists::default();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write QueueNameExists contents to a SignedRequest
struct QueueNameExistsWriter;
impl QueueNameExistsWriter {
	fn write_params(params: &mut Params, name: &str, obj: &QueueNameExists) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
	}
}
/// Two or more batch entries have the same `Id` in the request.
#[derive(Debug, Default)]
pub struct BatchEntryIdsNotDistinct;

/// Parse BatchEntryIdsNotDistinct from XML
struct BatchEntryIdsNotDistinctParser;
impl BatchEntryIdsNotDistinctParser {
	fn parse_xml<'a, T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<BatchEntryIdsNotDistinct, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = BatchEntryIdsNotDistinct::default();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write BatchEntryIdsNotDistinct contents to a SignedRequest
struct BatchEntryIdsNotDistinctWriter;
impl BatchEntryIdsNotDistinctWriter {
	fn write_params(params: &mut Params, name: &str, obj: &BatchEntryIdsNotDistinct) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
	}
}
/// Encloses the id an entry in DeleteMessageBatch.
#[derive(Debug, Default)]
pub struct DeleteMessageBatchResultEntry {
	/// Represents a successfully deleted message.
	pub id: String,
}

/// Parse DeleteMessageBatchResultEntry from XML
struct DeleteMessageBatchResultEntryParser;
impl DeleteMessageBatchResultEntryParser {
	fn parse_xml<'a, T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<DeleteMessageBatchResultEntry, XmlParseError> {
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
/// Write DeleteMessageBatchResultEntry contents to a SignedRequest
struct DeleteMessageBatchResultEntryWriter;
impl DeleteMessageBatchResultEntryWriter {
	fn write_params(params: &mut Params, name: &str, obj: &DeleteMessageBatchResultEntry) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		StringWriter::write_params(params, &(prefix.to_string() + "Id"), &obj.id);
	}
}
pub type DeleteMessageBatchRequestEntryList = Vec<DeleteMessageBatchRequestEntry>;
/// Parse DeleteMessageBatchRequestEntryList from XML
struct DeleteMessageBatchRequestEntryListParser;
impl DeleteMessageBatchRequestEntryListParser {
	fn parse_xml<'a, T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<DeleteMessageBatchRequestEntryList, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "DeleteMessageBatchRequestEntry" {
			obj.push(try!(DeleteMessageBatchRequestEntryParser::parse_xml("DeleteMessageBatchRequestEntry", stack)));
		}
		Ok(obj)
	}
}
/// Write DeleteMessageBatchRequestEntryList contents to a SignedRequest
struct DeleteMessageBatchRequestEntryListWriter;
impl DeleteMessageBatchRequestEntryListWriter {
	fn write_params(params: &mut Params, name: &str, obj: &DeleteMessageBatchRequestEntryList) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			DeleteMessageBatchRequestEntryWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
/// Batch request does not contain an entry.
#[derive(Debug, Default)]
pub struct EmptyBatchRequest;

/// Parse EmptyBatchRequest from XML
struct EmptyBatchRequestParser;
impl EmptyBatchRequestParser {
	fn parse_xml<'a, T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<EmptyBatchRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = EmptyBatchRequest::default();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write EmptyBatchRequest contents to a SignedRequest
struct EmptyBatchRequestWriter;
impl EmptyBatchRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &EmptyBatchRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
	}
}
/// A list of your queues.
#[derive(Debug, Default)]
pub struct ListQueuesResult {
	/// A list of queue URLs, up to 1000 entries.
	pub queue_urls: QueueUrlList,
}

/// Parse ListQueuesResult from XML
struct ListQueuesResultParser;
impl ListQueuesResultParser {
	fn parse_xml<'a, T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ListQueuesResult, XmlParseError> {
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
/// Write ListQueuesResult contents to a SignedRequest
struct ListQueuesResultWriter;
impl ListQueuesResultWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ListQueuesResult) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		QueueUrlListWriter::write_params(params, &(prefix.to_string() + "QueueUrl"), &obj.queue_urls);
	}
}
/// Encloses a receipt handle and an entry id for each message in
/// ChangeMessageVisibilityBatch.
/// All of the following parameters are list parameters that must be prefixed with
/// `ChangeMessageVisibilityBatchRequestEntry.n`, where `n` is an integer value
/// starting with 1. For example, a parameter list for this action might look like
/// this:
/// `&ChangeMessageVisibilityBatchRequestEntry.1.Id=change_visibility_msg_2`
/// `&ChangeMessageVisibilityBatchRequestEntry.1.ReceiptHandle=Your_Receipt_Handle
/// `
/// `&ChangeMessageVisibilityBatchRequestEntry.1.VisibilityTimeout=45`
#[derive(Debug, Default)]
pub struct ChangeMessageVisibilityBatchRequestEntry {
	/// A receipt handle.
	pub receipt_handle: String,
	/// The new value (in seconds) for the message's visibility timeout.
	pub visibility_timeout: Option<Integer>,
	/// An identifier for this particular receipt handle. This is used to communicate
	/// the result. Note that the `Id`s of a batch request need to be unique within
	/// the request.
	pub id: String,
}

/// Parse ChangeMessageVisibilityBatchRequestEntry from XML
struct ChangeMessageVisibilityBatchRequestEntryParser;
impl ChangeMessageVisibilityBatchRequestEntryParser {
	fn parse_xml<'a, T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ChangeMessageVisibilityBatchRequestEntry, XmlParseError> {
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
/// Write ChangeMessageVisibilityBatchRequestEntry contents to a SignedRequest
struct ChangeMessageVisibilityBatchRequestEntryWriter;
impl ChangeMessageVisibilityBatchRequestEntryWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ChangeMessageVisibilityBatchRequestEntry) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		StringWriter::write_params(params, &(prefix.to_string() + "ReceiptHandle"), &obj.receipt_handle);
		if let Some(ref obj) = obj.visibility_timeout {
			IntegerWriter::write_params(params, &(prefix.to_string() + "VisibilityTimeout"), obj);
		}
		StringWriter::write_params(params, &(prefix.to_string() + "Id"), &obj.id);
	}
}
/// Batch request contains more number of entries than permissible.
#[derive(Debug, Default)]
pub struct TooManyEntriesInBatchRequest;

/// Parse TooManyEntriesInBatchRequest from XML
struct TooManyEntriesInBatchRequestParser;
impl TooManyEntriesInBatchRequestParser {
	fn parse_xml<'a, T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<TooManyEntriesInBatchRequest, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = TooManyEntriesInBatchRequest::default();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write TooManyEntriesInBatchRequest contents to a SignedRequest
struct TooManyEntriesInBatchRequestWriter;
impl TooManyEntriesInBatchRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &TooManyEntriesInBatchRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
	}
}
/// You must wait 60 seconds after deleting a queue before you can create another
/// with the same name.
#[derive(Debug, Default)]
pub struct QueueDeletedRecently;

/// Parse QueueDeletedRecently from XML
struct QueueDeletedRecentlyParser;
impl QueueDeletedRecentlyParser {
	fn parse_xml<'a, T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<QueueDeletedRecently, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = QueueDeletedRecently::default();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write QueueDeletedRecently contents to a SignedRequest
struct QueueDeletedRecentlyWriter;
impl QueueDeletedRecentlyWriter {
	fn write_params(params: &mut Params, name: &str, obj: &QueueDeletedRecently) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
	}
}
pub type Boolean = bool;
/// Parse Boolean from XML
struct BooleanParser;
impl BooleanParser {
	fn parse_xml<'a, T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Boolean, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = bool::from_str(try!(characters(stack)).as_ref()).unwrap();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write Boolean contents to a SignedRequest
struct BooleanWriter;
impl BooleanWriter {
	fn write_params(params: &mut Params, name: &str, obj: &Boolean) {
		params.put(name, &obj.to_string());
	}
}
/// For each message in the batch, the response contains a
/// DeleteMessageBatchResultEntry tag if the message is deleted or a
/// BatchResultErrorEntry tag if the message cannot be deleted.
#[derive(Debug, Default)]
pub struct DeleteMessageBatchResult {
	/// A list of DeleteMessageBatchResultEntry items.
	pub successful: DeleteMessageBatchResultEntryList,
	/// A list of BatchResultErrorEntry items.
	pub failed: BatchResultErrorEntryList,
}

/// Parse DeleteMessageBatchResult from XML
struct DeleteMessageBatchResultParser;
impl DeleteMessageBatchResultParser {
	fn parse_xml<'a, T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<DeleteMessageBatchResult, XmlParseError> {
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
/// Write DeleteMessageBatchResult contents to a SignedRequest
struct DeleteMessageBatchResultWriter;
impl DeleteMessageBatchResultWriter {
	fn write_params(params: &mut Params, name: &str, obj: &DeleteMessageBatchResult) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		DeleteMessageBatchResultEntryListWriter::write_params(params, &(prefix.to_string() + "DeleteMessageBatchResultEntry"), &obj.successful);
		BatchResultErrorEntryListWriter::write_params(params, &(prefix.to_string() + "BatchResultErrorEntry"), &obj.failed);
	}
}
/// Encloses the id of an entry in ChangeMessageVisibilityBatch.
#[derive(Debug, Default)]
pub struct ChangeMessageVisibilityBatchResultEntry {
	/// Represents a message whose visibility timeout has been changed successfully.
	pub id: String,
}

/// Parse ChangeMessageVisibilityBatchResultEntry from XML
struct ChangeMessageVisibilityBatchResultEntryParser;
impl ChangeMessageVisibilityBatchResultEntryParser {
	fn parse_xml<'a, T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ChangeMessageVisibilityBatchResultEntry, XmlParseError> {
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
/// Write ChangeMessageVisibilityBatchResultEntry contents to a SignedRequest
struct ChangeMessageVisibilityBatchResultEntryWriter;
impl ChangeMessageVisibilityBatchResultEntryWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ChangeMessageVisibilityBatchResultEntry) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		StringWriter::write_params(params, &(prefix.to_string() + "Id"), &obj.id);
	}
}
pub type AttributeNameList = Vec<QueueAttributeName>;
/// Parse AttributeNameList from XML
struct AttributeNameListParser;
impl AttributeNameListParser {
	fn parse_xml<'a, T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<AttributeNameList, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "AttributeName" {
			obj.push(try!(QueueAttributeNameParser::parse_xml("AttributeName", stack)));
		}
		Ok(obj)
	}
}
/// Write AttributeNameList contents to a SignedRequest
struct AttributeNameListWriter;
impl AttributeNameListWriter {
	fn write_params(params: &mut Params, name: &str, obj: &AttributeNameList) {
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
	/// The URL of the Amazon SQS queue to take action on.
	pub queue_url: String,
	/// The receipt handle associated with the message to delete.
	pub receipt_handle: String,
}

/// Parse DeleteMessageRequest from XML
struct DeleteMessageRequestParser;
impl DeleteMessageRequestParser {
	fn parse_xml<'a, T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<DeleteMessageRequest, XmlParseError> {
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
/// Write DeleteMessageRequest contents to a SignedRequest
struct DeleteMessageRequestWriter;
impl DeleteMessageRequestWriter {
	fn write_params(params: &mut Params, name: &str, obj: &DeleteMessageRequest) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		StringWriter::write_params(params, &(prefix.to_string() + "QueueUrl"), &obj.queue_url);
		StringWriter::write_params(params, &(prefix.to_string() + "ReceiptHandle"), &obj.receipt_handle);
	}
}
/// Contains the details of a single Amazon SQS message along with a `Id`.
#[derive(Debug, Default)]
pub struct SendMessageBatchRequestEntry {
	/// The number of seconds for which the message has to be delayed.
	pub delay_seconds: Option<Integer>,
	/// An identifier for the message in this batch. This is used to communicate the
	/// result. Note that the `Id`s of a batch request need to be unique within the
	/// request.
	pub id: String,
	/// Body of the message.
	pub message_body: String,
	/// Each message attribute consists of a Name, Type, and Value. For more
	/// information, see [Message Attribute Items](http://docs.aws.amazon.com/AWSSimpl
	/// eQueueService/latest/SQSDeveloperGuide/SQSMessageAttributes.html#SQSMessageAtt
	/// ributesNTV).
	pub message_attributes: Option<MessageAttributeMap>,
}

/// Parse SendMessageBatchRequestEntry from XML
struct SendMessageBatchRequestEntryParser;
impl SendMessageBatchRequestEntryParser {
	fn parse_xml<'a, T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<SendMessageBatchRequestEntry, XmlParseError> {
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
/// Write SendMessageBatchRequestEntry contents to a SignedRequest
struct SendMessageBatchRequestEntryWriter;
impl SendMessageBatchRequestEntryWriter {
	fn write_params(params: &mut Params, name: &str, obj: &SendMessageBatchRequestEntry) {
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
pub struct SQSClient<'a> {
	creds: &'a AWSCredentials,
	region: &'a str
}

impl<'a> SQSClient<'a> { 
	pub fn new(creds: &'a AWSCredentials, region: &'a str) -> SQSClient<'a> {
		SQSClient { creds: creds, region: region }
	}
	/// Creates a new queue, or returns the URL of an existing one. When you request
	/// `CreateQueue`, you provide a name for the queue. To successfully create a new
	/// queue, you must provide a name that is unique within the scope of your own
	/// queues.
	/// If you delete a queue, you must wait at least 60 seconds before creating a
	/// queue with the same name.
	/// You may pass one or more attributes in the request. If you do not provide a
	/// value for any attribute, the queue will have the default value for that
	/// attribute. Permitted attributes are the same that can be set using
	/// SetQueueAttributes.
	/// Use GetQueueUrl to get a queue's URL. GetQueueUrl requires only the
	/// `QueueName` parameter.
	/// If you provide the name of an existing queue, along with the exact names and
	/// values of all the queue's attributes, `CreateQueue` returns the queue URL for
	/// the existing queue. If the queue name, attribute names, or attribute values do
	/// not match an existing queue, `CreateQueue` returns an error.
	/// Some API actions take lists of parameters. These lists are specified using the
	/// `param.n` notation. Values of `n` are integers starting from 1. For example, a
	/// parameter list with two elements looks like this:
	/// `&Attribute.1=this`
	/// `&Attribute.2=that`
	pub fn create_queue(&self, input: &CreateQueueRequest) -> Result<CreateQueueResult, AWSError> {
		let mut request = SignedRequest::new("POST", "sqs", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "CreateQueue");
		CreateQueueRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let result = request.sign_and_execute(&self.creds);
		let status = result.status.to_u16();
		let mut reader = EventReader::new(result);
		let mut stack = XmlResponseFromAws::new(reader.events().peekable());
		stack.next(); // xml start tag
		stack.next();
		match status {
			200 => { 
				Ok(try!(CreateQueueResultParser::parse_xml("CreateQueueResult", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Gets attributes for the specified queue. The following attributes are
	/// supported:
	///   * `All` \- returns all values.
	///   * `ApproximateNumberOfMessages` \- returns the approximate number of visible messages in a queue. For more information, see [Resources Required to Process Messages](http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/ApproximateNumber.html) in the _Amazon SQS Developer Guide_.
	///   * `ApproximateNumberOfMessagesNotVisible` \- returns the approximate number of messages that are not timed-out and not deleted. For more information, see [Resources Required to Process Messages](http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/ApproximateNumber.html) in the _Amazon SQS Developer Guide_.
	///   * `VisibilityTimeout` \- returns the visibility timeout for the queue. For more information about visibility timeout, see [Visibility Timeout](http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/AboutVT.html) in the _Amazon SQS Developer Guide_.
	///   * `CreatedTimestamp` \- returns the time when the queue was created (epoch time in seconds).
	///   * `LastModifiedTimestamp` \- returns the time when the queue was last changed (epoch time in seconds).
	///   * `Policy` \- returns the queue's policy.
	///   * `MaximumMessageSize` \- returns the limit of how many bytes a message can contain before Amazon SQS rejects it.
	///   * `MessageRetentionPeriod` \- returns the number of seconds Amazon SQS retains a message.
	///   * `QueueArn` \- returns the queue's Amazon resource name (ARN).
	///   * `ApproximateNumberOfMessagesDelayed` \- returns the approximate number of messages that are pending to be added to the queue.
	///   * `DelaySeconds` \- returns the default delay on the queue in seconds.
	///   * `ReceiveMessageWaitTimeSeconds` \- returns the time for which a ReceiveMessage call will wait for a message to arrive.
	///   * `RedrivePolicy` \- returns the parameters for dead letter queue functionality of the source queue. For more information about RedrivePolicy and dead letter queues, see [Using Amazon SQS Dead Letter Queues](http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/SQSDeadLetterQueue.html) in the _Amazon SQS Developer Guide_.
	/// Going forward, new attributes might be added. If you are writing code that
	/// calls this action, we recommend that you structure your code so that it can
	/// handle new attributes gracefully. Some API actions take lists of parameters.
	/// These lists are specified using the `param.n` notation. Values of `n` are
	/// integers starting from 1. For example, a parameter list with two elements
	/// looks like this:
	/// `&Attribute.1=this`
	/// `&Attribute.2=that`
	pub fn get_queue_attributes(&self, input: &GetQueueAttributesRequest) -> Result<GetQueueAttributesResult, AWSError> {
		let mut request = SignedRequest::new("POST", "sqs", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "GetQueueAttributes");
		GetQueueAttributesRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let result = request.sign_and_execute(&self.creds);
		let status = result.status.to_u16();
		let mut reader = EventReader::new(result);
		let mut stack = XmlResponseFromAws::new(reader.events().peekable());
		stack.next(); // xml start tag
		stack.next();
		match status {
			200 => { 
				Ok(try!(GetQueueAttributesResultParser::parse_xml("GetQueueAttributesResult", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Sets the value of one or more queue attributes. When you change a queue's
	/// attributes, the change can take up to 60 seconds for most of the attributes to
	/// propagate throughout the SQS system. Changes made to the
	/// `MessageRetentionPeriod` attribute can take up to 15 minutes.
	/// Going forward, new attributes might be added. If you are writing code that
	/// calls this action, we recommend that you structure your code so that it can
	/// handle new attributes gracefully.
	pub fn set_queue_attributes(&self, input: &SetQueueAttributesRequest) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "sqs", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "SetQueueAttributes");
		SetQueueAttributesRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let result = request.sign_and_execute(&self.creds);
		let status = result.status.to_u16();
		let mut reader = EventReader::new(result);
		let mut stack = XmlResponseFromAws::new(reader.events().peekable());
		stack.next(); // xml start tag
		stack.next();
		match status {
			200 => { 
				Ok(())
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Returns the URL of an existing queue. This action provides a simple way to
	/// retrieve the URL of an Amazon SQS queue.
	/// To access a queue that belongs to another AWS account, use the
	/// `QueueOwnerAWSAccountId` parameter to specify the account ID of the queue's
	/// owner. The queue's owner must grant you permission to access the queue. For
	/// more information about shared queue access, see AddPermission or go to [Shared
	/// Queues](http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGu
	/// ide/acp-overview.html) in the _Amazon SQS Developer Guide_.
	pub fn get_queue_url(&self, input: &GetQueueUrlRequest) -> Result<GetQueueUrlResult, AWSError> {
		let mut request = SignedRequest::new("POST", "sqs", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "GetQueueUrl");
		GetQueueUrlRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let result = request.sign_and_execute(&self.creds);
		let status = result.status.to_u16();
		let mut reader = EventReader::new(result);
		let mut stack = XmlResponseFromAws::new(reader.events().peekable());
		stack.next(); // xml start tag
		stack.next();
		match status {
			200 => { 
				Ok(try!(GetQueueUrlResultParser::parse_xml("GetQueueUrlResult", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Deletes up to ten messages from the specified queue. This is a batch version
	/// of DeleteMessage. The result of the delete action on each message is reported
	/// individually in the response.
	/// Because the batch request can result in a combination of successful and
	/// unsuccessful actions, you should check for batch errors even when the call
	/// returns an HTTP status code of 200.
	/// Some API actions take lists of parameters. These lists are specified using the
	/// `param.n` notation. Values of `n` are integers starting from 1. For example, a
	/// parameter list with two elements looks like this:
	/// `&Attribute.1=this`
	/// `&Attribute.2=that`
	pub fn delete_message_batch(&self, input: &DeleteMessageBatchRequest) -> Result<DeleteMessageBatchResult, AWSError> {
		let mut request = SignedRequest::new("POST", "sqs", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "DeleteMessageBatch");
		DeleteMessageBatchRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let result = request.sign_and_execute(&self.creds);
		let status = result.status.to_u16();
		let mut reader = EventReader::new(result);
		let mut stack = XmlResponseFromAws::new(reader.events().peekable());
		stack.next(); // xml start tag
		stack.next();
		match status {
			200 => { 
				Ok(try!(DeleteMessageBatchResultParser::parse_xml("DeleteMessageBatchResult", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Delivers up to ten messages to the specified queue. This is a batch version of
	/// SendMessage. The result of the send action on each message is reported
	/// individually in the response. The maximum allowed individual message size is
	/// 256 KB (262,144 bytes).
	/// The maximum total payload size (i.e., the sum of all a batch's individual
	/// message lengths) is also 256 KB (262,144 bytes).
	/// If the `DelaySeconds` parameter is not specified for an entry, the default for
	/// the queue is used.
	/// The following list shows the characters (in Unicode) that are allowed in your
	/// message, according to the W3C XML specification. For more information, go to
	/// <http://www.faqs.org/rfcs/rfc1321.html>. If you send any characters that are
	/// not included in the list, your request will be rejected.
	/// #x9 | #xA | #xD | [#x20 to #xD7FF] | [#xE000 to #xFFFD] | [#x10000 to
	/// #x10FFFF]
	/// Because the batch request can result in a combination of successful and
	/// unsuccessful actions, you should check for batch errors even when the call
	/// returns an HTTP status code of 200.  Some API actions take lists of
	/// parameters. These lists are specified using the `param.n` notation. Values of
	/// `n` are integers starting from 1. For example, a parameter list with two
	/// elements looks like this:
	/// `&Attribute.1=this`
	/// `&Attribute.2=that`
	pub fn send_message_batch(&self, input: &SendMessageBatchRequest) -> Result<SendMessageBatchResult, AWSError> {
		let mut request = SignedRequest::new("POST", "sqs", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "SendMessageBatch");
		SendMessageBatchRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let result = request.sign_and_execute(&self.creds);
		let status = result.status.to_u16();
		let mut reader = EventReader::new(result);
		let mut stack = XmlResponseFromAws::new(reader.events().peekable());
		stack.next(); // xml start tag
		stack.next();
		match status {
			200 => { 
				Ok(try!(SendMessageBatchResultParser::parse_xml("SendMessageBatchResult", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Returns a list of your queues that have the RedrivePolicy queue attribute
	/// configured with a dead letter queue.
	/// For more information about using dead letter queues, see [Using Amazon SQS
	/// Dead Letter Queues](http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQ
	/// SDeveloperGuide/SQSDeadLetterQueue.html).
	pub fn list_dead_letter_source_queues(&self, input: &ListDeadLetterSourceQueuesRequest) -> Result<ListDeadLetterSourceQueuesResult, AWSError> {
		let mut request = SignedRequest::new("POST", "sqs", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "ListDeadLetterSourceQueues");
		ListDeadLetterSourceQueuesRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let result = request.sign_and_execute(&self.creds);
		let status = result.status.to_u16();
		let mut reader = EventReader::new(result);
		let mut stack = XmlResponseFromAws::new(reader.events().peekable());
		stack.next(); // xml start tag
		stack.next();
		match status {
			200 => { 
				Ok(try!(ListDeadLetterSourceQueuesResultParser::parse_xml("ListDeadLetterSourceQueuesResult", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Changes the visibility timeout of a specified message in a queue to a new
	/// value. The maximum allowed timeout value you can set the value to is 12 hours.
	/// This means you can't extend the timeout of a message in an existing queue to
	/// more than a total visibility timeout of 12 hours. (For more information
	/// visibility timeout, see [Visibility Timeout](http://docs.aws.amazon.com/AWSSim
	/// pleQueueService/latest/SQSDeveloperGuide/AboutVT.html) in the _Amazon SQS
	/// Developer Guide_.)
	/// For example, let's say you have a message and its default message visibility
	/// timeout is 30 minutes. You could call `ChangeMessageVisiblity` with a value of
	/// two hours and the effective timeout would be two hours and 30 minutes. When
	/// that time comes near you could again extend the time out by calling
	/// ChangeMessageVisiblity, but this time the maximum allowed timeout would be 9
	/// hours and 30 minutes.
	/// There is a 120,000 limit for the number of inflight messages per queue.
	/// Messages are inflight after they have been received from the queue by a
	/// consuming component, but have not yet been deleted from the queue. If you
	/// reach the 120,000 limit, you will receive an OverLimit error message from
	/// Amazon SQS. To help avoid reaching the limit, you should delete the messages
	/// from the queue after they have been processed. You can also increase the
	/// number of queues you use to process the messages.
	/// If you attempt to set the `VisibilityTimeout` to an amount more than the
	/// maximum time left, Amazon SQS returns an error. It will not automatically
	/// recalculate and increase the timeout to the maximum time remaining. Unlike
	/// with a queue, when you change the visibility timeout for a specific message,
	/// that timeout value is applied immediately but is not saved in memory for that
	/// message. If you don't delete a message after it is received, the visibility
	/// timeout for the message the next time it is received reverts to the original
	/// timeout value, not the value you set with the `ChangeMessageVisibility`
	/// action.
	pub fn change_message_visibility(&self, input: &ChangeMessageVisibilityRequest) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "sqs", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "ChangeMessageVisibility");
		ChangeMessageVisibilityRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let result = request.sign_and_execute(&self.creds);
		let status = result.status.to_u16();
		let mut reader = EventReader::new(result);
		let mut stack = XmlResponseFromAws::new(reader.events().peekable());
		stack.next(); // xml start tag
		stack.next();
		match status {
			200 => { 
				Ok(())
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Adds a permission to a queue for a specific
	/// [principal](http://docs.aws.amazon.com/general/latest/gr/glos-chap.html#P).
	/// This allows for sharing access to the queue.
	/// When you create a queue, you have full control access rights for the queue.
	/// Only you (as owner of the queue) can grant or deny permissions to the queue.
	/// For more information about these permissions, see [Shared Queues](http://docs.
	/// aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/acp-
	/// overview.html) in the _Amazon SQS Developer Guide_.
	/// `AddPermission` writes an Amazon SQS-generated policy. If you want to write
	/// your own policy, use SetQueueAttributes to upload your policy. For more
	/// information about writing your own policy, see [Using The Access Policy Langua
	/// ge](http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/
	/// AccessPolicyLanguage.html) in the _Amazon SQS Developer Guide_.
	/// Some API actions take lists of parameters. These lists are specified using the
	/// `param.n` notation. Values of `n` are integers starting from 1. For example, a
	/// parameter list with two elements looks like this:
	/// `&Attribute.1=this`
	/// `&Attribute.2=that`
	pub fn add_permission(&self, input: &AddPermissionRequest) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "sqs", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "AddPermission");
		AddPermissionRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let result = request.sign_and_execute(&self.creds);
		let status = result.status.to_u16();
		let mut reader = EventReader::new(result);
		let mut stack = XmlResponseFromAws::new(reader.events().peekable());
		stack.next(); // xml start tag
		stack.next();
		match status {
			200 => { 
				Ok(())
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Changes the visibility timeout of multiple messages. This is a batch version
	/// of ChangeMessageVisibility. The result of the action on each message is
	/// reported individually in the response. You can send up to 10
	/// ChangeMessageVisibility requests with each `ChangeMessageVisibilityBatch`
	/// action.
	/// Because the batch request can result in a combination of successful and
	/// unsuccessful actions, you should check for batch errors even when the call
	/// returns an HTTP status code of 200. Some API actions take lists of parameters.
	/// These lists are specified using the `param.n` notation. Values of `n` are
	/// integers starting from 1. For example, a parameter list with two elements
	/// looks like this:
	/// `&Attribute.1=this`
	/// `&Attribute.2=that`
	pub fn change_message_visibility_batch(&self, input: &ChangeMessageVisibilityBatchRequest) -> Result<ChangeMessageVisibilityBatchResult, AWSError> {
		let mut request = SignedRequest::new("POST", "sqs", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "ChangeMessageVisibilityBatch");
		ChangeMessageVisibilityBatchRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let result = request.sign_and_execute(&self.creds);
		let status = result.status.to_u16();
		let mut reader = EventReader::new(result);
		let mut stack = XmlResponseFromAws::new(reader.events().peekable());
		stack.next(); // xml start tag
		stack.next();
		match status {
			200 => { 
				Ok(try!(ChangeMessageVisibilityBatchResultParser::parse_xml("ChangeMessageVisibilityBatchResult", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Delivers a message to the specified queue. With Amazon SQS, you now have the
	/// ability to send large payload messages that are up to 256KB (262,144 bytes) in
	/// size. To send large payloads, you must use an AWS SDK that supports SigV4
	/// signing. To verify whether SigV4 is supported for an AWS SDK, check the SDK
	/// release notes.
	/// The following list shows the characters (in Unicode) allowed in your message,
	/// according to the W3C XML specification. For more information, go to
	/// <http://www.w3.org/TR/REC-xml/#charsets> If you send any characters not
	/// included in the list, your request will be rejected.
	/// #x9 | #xA | #xD | [#x20 to #xD7FF] | [#xE000 to #xFFFD] | [#x10000 to
	/// #x10FFFF]
	pub fn send_message(&self, input: &SendMessageRequest) -> Result<SendMessageResult, AWSError> {
		let mut request = SignedRequest::new("POST", "sqs", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "SendMessage");
		SendMessageRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let result = request.sign_and_execute(&self.creds);
		let status = result.status.to_u16();
		let mut reader = EventReader::new(result);
		let mut stack = XmlResponseFromAws::new(reader.events().peekable());
		stack.next(); // xml start tag
		stack.next();
		match status {
			200 => { 
				Ok(try!(SendMessageResultParser::parse_xml("SendMessageResult", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Deletes the queue specified by the **queue URL**, regardless of whether the
	/// queue is empty. If the specified queue does not exist, Amazon SQS returns a
	/// successful response.
	/// Use `DeleteQueue` with care; once you delete your queue, any messages in the
	/// queue are no longer available.
	/// When you delete a queue, the deletion process takes up to 60 seconds. Requests
	/// you send involving that queue during the 60 seconds might succeed. For
	/// example, a SendMessage request might succeed, but after the 60 seconds, the
	/// queue and that message you sent no longer exist. Also, when you delete a
	/// queue, you must wait at least 60 seconds before creating a queue with the same
	/// name.
	/// We reserve the right to delete queues that have had no activity for more than
	/// 30 days. For more information, see [How Amazon SQS Queues Work](http://docs.aw
	/// s.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/SQSConcepts.html)
	/// in the _Amazon SQS Developer Guide_.
	pub fn delete_queue(&self, input: &DeleteQueueRequest) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "sqs", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "DeleteQueue");
		DeleteQueueRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let result = request.sign_and_execute(&self.creds);
		let status = result.status.to_u16();
		let mut reader = EventReader::new(result);
		let mut stack = XmlResponseFromAws::new(reader.events().peekable());
		stack.next(); // xml start tag
		stack.next();
		match status {
			200 => { 
				Ok(())
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Deletes the messages in a queue specified by the **queue URL**.
	/// When you use the `PurgeQueue` API, the deleted messages in the queue cannot be
	/// retrieved.
	/// When you purge a queue, the message deletion process takes up to 60 seconds.
	/// All messages sent to the queue before calling `PurgeQueue` will be deleted;
	/// messages sent to the queue while it is being purged may be deleted. While the
	/// queue is being purged, messages sent to the queue before `PurgeQueue` was
	/// called may be received, but will be deleted within the next minute.
	pub fn purge_queue(&self, input: &PurgeQueueRequest) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "sqs", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "PurgeQueue");
		PurgeQueueRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let result = request.sign_and_execute(&self.creds);
		let status = result.status.to_u16();
		let mut reader = EventReader::new(result);
		let mut stack = XmlResponseFromAws::new(reader.events().peekable());
		stack.next(); // xml start tag
		stack.next();
		match status {
			200 => { 
				Ok(())
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Retrieves one or more messages, with a maximum limit of 10 messages, from the
	/// specified queue. Long poll support is enabled by using the `WaitTimeSeconds`
	/// parameter. For more information, see [Amazon SQS Long Poll](http://docs.aws.am
	/// azon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-long-polling.html)
	/// in the _Amazon SQS Developer Guide_.
	/// Short poll is the default behavior where a weighted random set of machines is
	/// sampled on a `ReceiveMessage` call. This means only the messages on the
	/// sampled machines are returned. If the number of messages in the queue is small
	/// (less than 1000), it is likely you will get fewer messages than you requested
	/// per `ReceiveMessage` call. If the number of messages in the queue is extremely
	/// small, you might not receive any messages in a particular `ReceiveMessage`
	/// response; in which case you should repeat the request.
	/// For each message returned, the response includes the following:
	///   * Message body 
	///   * MD5 digest of the message body. For information about MD5, go to <http://www.faqs.org/rfcs/rfc1321.html>. 
	///   * Message ID you received when you sent the message to the queue. 
	///   * Receipt handle. 
	///   * Message attributes. 
	///   * MD5 digest of the message attributes. 
	/// The receipt handle is the identifier you must provide when deleting the
	/// message. For more information, see [Queue and Message Identifiers](http://docs
	/// .aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/ImportantIdenti
	/// fiers.html) in the _Amazon SQS Developer Guide_.
	/// You can provide the `VisibilityTimeout` parameter in your request, which will
	/// be applied to the messages that Amazon SQS returns in the response. If you do
	/// not include the parameter, the overall visibility timeout for the queue is
	/// used for the returned messages. For more information, see [Visibility Timeout]
	/// (http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/Abo
	/// utVT.html) in the _Amazon SQS Developer Guide_.
	/// Going forward, new attributes might be added. If you are writing code that
	/// calls this action, we recommend that you structure your code so that it can
	/// handle new attributes gracefully.
	pub fn receive_message(&self, input: &ReceiveMessageRequest) -> Result<ReceiveMessageResult, AWSError> {
		let mut request = SignedRequest::new("POST", "sqs", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "ReceiveMessage");
		ReceiveMessageRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let result = request.sign_and_execute(&self.creds);
		let status = result.status.to_u16();
		let mut reader = EventReader::new(result);
		let mut stack = XmlResponseFromAws::new(reader.events().peekable());
		stack.next(); // xml start tag
		stack.next();
		match status {
			200 => { 
				Ok(try!(ReceiveMessageResultParser::parse_xml("ReceiveMessageResult", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Deletes the specified message from the specified queue. You specify the
	/// message by using the message's `receipt handle` and not the `message ID` you
	/// received when you sent the message. Even if the message is locked by another
	/// reader due to the visibility timeout setting, it is still deleted from the
	/// queue. If you leave a message in the queue for longer than the queue's
	/// configured retention period, Amazon SQS automatically deletes it.
	/// The receipt handle is associated with a specific instance of receiving the
	/// message. If you receive a message more than once, the receipt handle you get
	/// each time you receive the message is different. When you request
	/// `DeleteMessage`, if you don't provide the most recently received receipt
	/// handle for the message, the request will still succeed, but the message might
	/// not be deleted.
	/// It is possible you will receive a message even after you have deleted it. This
	/// might happen on rare occasions if one of the servers storing a copy of the
	/// message is unavailable when you request to delete the message. The copy
	/// remains on the server and might be returned to you again on a subsequent
	/// receive request. You should create your system to be idempotent so that
	/// receiving a particular message more than once is not a problem.
	pub fn delete_message(&self, input: &DeleteMessageRequest) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "sqs", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "DeleteMessage");
		DeleteMessageRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let result = request.sign_and_execute(&self.creds);
		let status = result.status.to_u16();
		let mut reader = EventReader::new(result);
		let mut stack = XmlResponseFromAws::new(reader.events().peekable());
		stack.next(); // xml start tag
		stack.next();
		match status {
			200 => { 
				Ok(())
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Returns a list of your queues. The maximum number of queues that can be
	/// returned is 1000. If you specify a value for the optional `QueueNamePrefix`
	/// parameter, only queues with a name beginning with the specified value are
	/// returned.
	pub fn list_queues(&self, input: &ListQueuesRequest) -> Result<ListQueuesResult, AWSError> {
		let mut request = SignedRequest::new("POST", "sqs", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "ListQueues");
		ListQueuesRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let result = request.sign_and_execute(&self.creds);
		let status = result.status.to_u16();
		let mut reader = EventReader::new(result);
		let mut stack = XmlResponseFromAws::new(reader.events().peekable());
		stack.next(); // xml start tag
		stack.next();
		match status {
			200 => { 
				Ok(try!(ListQueuesResultParser::parse_xml("ListQueuesResult", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Revokes any permissions in the queue policy that matches the specified `Label`
	/// parameter. Only the owner of the queue can remove permissions.
	pub fn remove_permission(&self, input: &RemovePermissionRequest) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "sqs", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "RemovePermission");
		RemovePermissionRequestWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let result = request.sign_and_execute(&self.creds);
		let status = result.status.to_u16();
		let mut reader = EventReader::new(result);
		let mut stack = XmlResponseFromAws::new(reader.events().peekable());
		stack.next(); // xml start tag
		stack.next();
		match status {
			200 => { 
				Ok(())
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
}
